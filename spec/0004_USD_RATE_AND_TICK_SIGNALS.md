# 0004_USD_RATE_AND_TICK_SIGNALS

## 사용자의 요청

두 가지 핵심 변경:

1. **USD 기준 환율 전환**: Upbit KRW 가격을 USDT로 환산하던 방식에서, Yahoo Finance의 실제 USD/KRW 환율로 USD 환산하는 방식으로 변경
2. **틱 기반 시그널 도출**: 분봉으로 rolling statistics(mean/stddev)를 갱신하되, 각 거래소의 개별 거래 메시지(틱)마다 즉시 z-score를 계산하여 진입/청산 판단

### 배경

#### 환율 문제

현재 시스템은 Upbit KRW-USDT 마켓 가격으로 환율을 계산한다:

```
upbit_usd_equivalent = Upbit_BTC_KRW / Upbit_USDT_KRW
spread = (Bybit_USDT - upbit_usd_equivalent) / upbit_usd_equivalent × 100
```

**문제**: Upbit USDT/KRW 가격에는 **USDT 김프**(한국 프리미엄)가 포함되어 있다. BTC 김프가 +1.5%이고 USDT 김프가 +3%인 경우:

| 항목 | 값 |
|------|-----|
| BTC 글로벌 가격 | $97,000 |
| USD/KRW 기준 환율 | 1,450 |
| Upbit BTC KRW (김프 1.5%) | 142,785,750 KRW |
| Upbit USDT/KRW (김프 3%) | 1,493.5 KRW |
| Upbit BTC USDT 환산 | 95,601 USDT (왜곡) |
| **결과 스프레드** | **+1.46% (역김프로 오판)** |

실제로는 BTC 1.5% 김프가 있지만, USDT 김프가 더 커서 역김프로 나타난다. **절대적 김프를 정확히 측정하려면 USDT 김프가 없는 기준 환율이 필요하다.**

#### 시그널 지연 문제

현재 시스템은 분봉 마감 시점에만 시그널을 평가한다. 차익거래는 밀리초 단위의 기회이므로, 최대 60초의 지연은 기회 손실로 이어진다.

### 확정 요구사항

| 항목 | 설정 |
|------|------|
| 환율 소스 | Yahoo Finance USD/KRW (`USDKRW=X`) |
| 환율 캐싱 | TTL 10분 |
| 환율 워밍업 | Yahoo Finance 일봉 환율 조회 (워밍업 기간에 해당하는 날짜) |
| 환율 모듈 | 신규 crate `arb-forex` |
| 시그널 타이밍 | 각 거래 메시지(틱)마다 즉시 z-score 계산 → 시그널 판단 |
| 통계 갱신 | 분봉 경계에서 rolling mean/stddev 갱신 (기존과 동일) |
| 진입/청산 | 모두 틱 기반 (즉시 실행) |
| 수수료 검증 | 유지 (expected_profit > roundtrip_fee) |
| KRW-USDT | **완전 제거** (스트림 구독, 캔들 조회, forward-fill 모두) |
| 백테스트 | **완전 제거** (simulator.rs, sweep.rs, console.rs, csv.rs, 관련 examples) |
| 슬리피지 모델 | **완전 제거** (시뮬레이션 모델 삭제) |
| 호가 기반 평단가 | 진입/청산 시 호가(order book) 볼륨을 고려한 예상 평단가로 PnL 계산 |

---

## 설계

### 전체 아키텍처 변경

```
변경 전:
┌─────────────────────────────────────────────────────────┐
│ WebSocket Ticks → MinuteCandleBuilder → 분봉 Close      │
│   → SpreadCalc(3-way: Upbit KRW, USDT/KRW, Bybit USDT) │
│   → Z-Score → Signal (분봉 마감 시점에만)                 │
└─────────────────────────────────────────────────────────┘

변경 후:
┌─────────────────────────────────────────────────────────┐
│ [분봉 경로] MinuteCandleBuilder → 분봉 Close             │
│   → SpreadCalc(2-way: Upbit KRW/USD_KRW, Bybit USDT)   │
│   → Rolling mean/stddev 갱신 (시그널 평가 안 함)          │
│                                                         │
│ [틱 경로] WebSocket Tick 수신 즉시                        │
│   → 현재 스프레드 계산 (last_trade + USD/KRW 캐시)        │
│   → Z-Score = (현재 spread - mean) / stddev              │
│   → Signal 즉시 평가 (진입/청산)                          │
│                                                         │
│ [환율 경로] Yahoo Finance USD/KRW (10분 TTL 캐시)         │
│   → 틱 스프레드 계산에 사용                               │
└─────────────────────────────────────────────────────────┘
```

### 1. arb-forex crate

신규 workspace crate로 Yahoo Finance USD/KRW 환율 조회 및 캐싱을 담당한다.

```rust
/// USD/KRW 환율 캐시.
///
/// Yahoo Finance API에서 현재 USD/KRW 환율을 조회하고
/// TTL 기반으로 캐싱합니다.
///
/// 설계 원칙:
/// - 틱 경로(hot path)에서는 캐시 값만 동기적으로 반환 (blocking I/O 없음)
/// - 환율 갱신은 별도 `tokio::spawn` task에서 비동기로 수행
/// - `AtomicU64` 또는 `arc_swap::ArcSwap`로 read contention 제거
pub struct ForexCache {
    client: reqwest::Client,
    /// 캐시된 환율 (lock-free read를 위해 atomic 사용).
    cached_rate: AtomicU64,       // f64를 u64 bits로 저장
    cached_at: AtomicI64,         // 캐시 시각 (unix millis)
    /// 캐시 TTL.
    ttl: Duration,
}

impl ForexCache {
    /// 새 ForexCache를 생성합니다.
    pub fn new(ttl: Duration) -> Self;

    /// 캐시된 USD/KRW 환율을 즉시 반환합니다 (blocking 없음).
    /// 캐시가 비어있으면 None 반환.
    /// 틱 경로(hot path)에서 사용합니다.
    pub fn get_cached_rate(&self) -> Option<f64>;

    /// USD/KRW 환율을 Yahoo Finance에서 조회하고 캐시를 갱신합니다.
    /// TTL 만료 시에만 실제 HTTP 요청을 발행합니다.
    /// 별도 갱신 task 또는 초기화 시 호출합니다.
    pub async fn refresh_if_expired(&self) -> Result<f64, ForexError>;

    /// 특정 기간의 일봉 USD/KRW 환율을 조회합니다 (워밍업용).
    /// 각 날짜의 close 환율을 반환합니다.
    pub async fn get_daily_rates(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<(DateTime<Utc>, f64)>, ForexError>;
}
```

**Yahoo Finance API:**

```
# 현재 환율
GET https://query1.finance.yahoo.com/v8/finance/chart/USDKRW=X?interval=1m&range=1d

# 일봉 환율 (워밍업용)
GET https://query1.finance.yahoo.com/v8/finance/chart/USDKRW=X?interval=1d&period1={unix}&period2={unix}
```

### 2. SpreadCalculator 변경

3-way 입력(Upbit KRW, USDT/KRW, Bybit USDT)에서 **2-way 입력**(Upbit KRW, Bybit USDT)으로 변경. 환율은 외부에서 주입.

```rust
// 변경 전
pub fn update(
    &mut self, coin: &str, timestamp: DateTime<Utc>,
    upbit_coin: Option<Decimal>,
    usdt_krw: Option<Decimal>,   // ← 제거
    bybit: Option<Decimal>,
) -> Result<(), StrategyError>

// 변경 후
pub fn update(
    &mut self, coin: &str, timestamp: DateTime<Utc>,
    upbit_coin: Option<Decimal>,
    usd_krw: f64,                // ← Yahoo Finance 환율 (f64, 캐시 값)
    bybit: Option<Decimal>,
) -> Result<(), StrategyError>
```

스프레드 공식 변경:

```
// 변경 전: Upbit KRW / Upbit USDT_KRW (김프 포함)
upbit_usdt = upbit_krw / usdt_krw_close

// 변경 후: Upbit KRW / Yahoo USD_KRW (김프 미포함)
upbit_usd = upbit_krw / usd_krw
```

**USDT/KRW forward-fill 상태 제거**, `usdt_krw_window` 제거. 환율은 캐시에서 직접 가져오므로 forward-fill 불필요.

### 3. 틱 기반 시그널

#### 새로운 이벤트 루프

```rust
// ★ 환율 갱신을 별도 task로 분리 (틱 루프 blocking 방지)
let forex = Arc::clone(&self.forex_cache);
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(600)); // 10분
    loop {
        interval.tick().await;
        if let Err(e) = forex.refresh_if_expired().await {
            warn!(error = %e, "USD/KRW 환율 갱신 실패, 캐시 값 유지");
        }
    }
});

loop {
    select! {
        // WebSocket 이벤트
        event = ws_rx.recv() => {
            match event {
                UpbitTrade(market, price) => {
                    candle_builder.on_upbit_trade(market, price);
                    // ★ 틱 시그널 즉시 평가 (에러 시 warn 후 계속)
                    if let Err(e) = self.check_tick_signal(coin) {
                        warn!(coin = coin, error = %e, "틱 시그널 평가 실패");
                    }
                }
                BybitBestQuote(symbol, bid) => {
                    candle_builder.on_bybit_best_quote(symbol, bid);
                    if let Err(e) = self.check_tick_signal(coin) {
                        warn!(coin = coin, error = %e, "틱 시그널 평가 실패");
                    }
                }
            }
        }
        // 분봉 타이머
        _ = candle_timer.tick() => {
            // 분봉 완결 → SpreadCalculator 갱신 (incremental stats 업데이트)
            // 시그널 평가는 하지 않음 (틱에서 처리)
        }
        // 코인 재선택 타이머 (기존)
        _ = reselect_timer.tick() => { ... }
    }
}
```

#### 틱 시그널 평가 함수

```rust
/// 틱 수신 시 즉시 시그널을 평가합니다.
///
/// 현재 last_trade 가격들로 스프레드를 계산하고,
/// 분봉 기반 rolling mean/stddev를 사용하여 z-score를 구합니다.
fn check_tick_signal(&mut self, coin: &str) -> Result<(), StrategyError> {
    // 1. 양쪽 last_trade가 모두 있어야 함
    let upbit_krw = self.candle_builder.upbit_last_trade.get(coin)?;
    let bybit_usd = self.candle_builder.bybit_last_bid.get(coin)?;
    let usd_krw = self.forex_cache.get_cached_rate()?;  // 캐시에서 즉시

    // 2. 현재 스프레드 계산
    let upbit_usd = upbit_krw.to_f64() / usd_krw;
    let bybit_f64 = bybit_usd.to_f64();
    let current_spread = (bybit_f64 - upbit_usd) / upbit_usd * 100.0;

    // 3. 분봉 기반 mean/stddev 조회 (incremental statistics, O(1))
    let (mean, stddev) = self.spread_calc.cached_stats(coin)?;
    if mean.is_none() { return Ok(()); }  // 윈도우 미충족
    let (mean, stddev) = (mean.unwrap(), stddev.unwrap());

    // 4. Z-Score 계산
    let z = statistics::z_score(current_spread, mean, stddev, config.min_stddev)?;

    // 5. 시그널 평가 (기존 signal.rs 로직 재사용)
    //    진입: z >= entry_z AND expected_profit > fee
    //    청산: z <= exit_z AND 포지션 보유
    ...
}
```

**핵심 차이점:**
- 분봉 경로: `SpreadCalculator.update()` → rolling window push → **incremental mean/stddev 갱신** (O(1))
- 틱 경로: last_trade 가격 → 즉석 스프레드 계산 → 캐시된 mean/stddev로 z-score → 즉시 시그널
- 에러 처리: `check_tick_signal()` 내부 에러는 `warn!` 로그 후 `continue` (전체 루프 중단 방지)

**성능 최적화 — Incremental Statistics:**

매 틱마다 `mean()`/`stddev()`를 O(window_size) 재계산하면 성능 병목이 된다. `SpreadCalculator`에 incremental statistics를 도입한다:

```rust
/// 분봉 push 시 O(1)로 갱신되는 통계 캐시.
struct IncrementalStats {
    running_sum: f64,
    running_sum_sq: f64,
    count: usize,
}

impl IncrementalStats {
    /// 새 값 push 시 갱신 (ring buffer pop된 값 반영).
    fn push(&mut self, new_val: f64, popped_val: Option<f64>) { ... }
    fn mean(&self) -> f64 { self.running_sum / self.count as f64 }
    fn stddev(&self) -> f64 { ... }
}
```

분봉 경계에서 `SpreadCalculator.update()` → window push 시 incremental stats 갱신.
틱 경로에서 `cached_stats(coin)` → O(1)로 mean/stddev 반환.

### 4. 워밍업 변경

```
변경 전:
  1. Upbit BTC/KRW 분봉 조회
  2. Upbit USDT/KRW 분봉 조회  ← 제거
  3. Bybit BTC/USDT 분봉 조회
  4. 3-way 정렬 → SpreadCalculator

변경 후:
  1. Yahoo Finance USD/KRW 일봉 조회 (워밍업 기간 커버)
  2. Upbit BTC/KRW 분봉 조회
  3. Bybit BTC/USDT 분봉 조회
  4. 각 분봉 timestamp에 해당하는 일봉 환율 매핑
  5. 2-way 정렬 → SpreadCalculator
```

일봉 환율은 분봉 대비 해상도가 낮지만, USD/KRW 환율은 하루 내 변동이 크지 않으므로 (일반적으로 < 0.5%) 워밍업 윈도우(~1시간) 구축에 충분하다.

### 5. VirtualPosition / ClosedPosition / PnL 변경

```rust
pub struct VirtualPosition {
    // ...
    pub entry_usd_krw: f64,        // 변경: entry_usdt_krw: Decimal → entry_usd_krw: f64
    // ...
}

pub struct ClosedPosition {
    // ...
    pub entry_usd_krw: f64,        // 변경: entry_usdt_krw: Decimal → entry_usd_krw: f64
    pub exit_usd_krw: f64,         // 변경: exit_usdt_krw: Decimal → exit_usd_krw: f64
    // ...
}

/// close_position() 시그니처도 변경
pub fn close_position(
    // ...
    exit_usd_krw: f64,             // 변경: exit_usdt_krw: Decimal → exit_usd_krw: f64
    // ...
) -> ClosedPosition
```

PnL 계산도 USD/KRW 기반으로 변경.

### 6. 호가 기반 평단가 계산

슬리피지 시뮬레이션 모델(square-root impact)은 제거하되, 실시간에서 진입/청산 시 **호가(order book)의 볼륨을 고려하여 예상 평단가를 계산**하고 이를 PnL에 반영한다.

```rust
/// 호가 볼륨을 고려한 예상 평단가를 계산합니다.
///
/// 주문 크기(size_usdt)만큼 호가를 소화할 때의 가중 평균 가격을 반환합니다.
/// 예: 매수 시 ask 호가를 낮은 가격부터 소화, 매도 시 bid 호가를 높은 가격부터 소화.
///
/// # 인자
/// - `orderbook_levels`: 호가 단계별 (가격, 수량) 배열
/// - `size_usdt`: 체결할 주문 크기 (USDT)
///
/// # 반환
/// - 가중 평균 체결 예상가
pub fn estimate_avg_price(
    orderbook_levels: &[(f64, f64)],  // (price, qty_in_usdt)
    size_usdt: f64,
) -> Option<f64>
```

**적용 시점:**
- 진입 시: Upbit ask 호가로 매수 평단가, Bybit bid 호가로 short 평단가 계산 → `VirtualPosition.upbit_entry_price`, `bybit_entry_price`에 반영
- 청산 시: Upbit bid 호가로 매도 평단가, Bybit ask 호가로 close 평단가 계산 → PnL 계산에 반영

**데이터 소스:**
- 현재 Bybit에서 `BestQuote` (최우선 호가 1단계)만 수신 중
- 호가 깊이(depth) 데이터가 필요하므로 Bybit의 orderbook WebSocket 구독 또는 REST API 조회 추가
- Upbit은 REST API로 호가 조회 (`/v1/orderbook`)

**주의:** 이 계산은 실제 주문 체결이 아닌 **시뮬레이션의 예상 평단가**이다. 실제 라이브 거래에서는 실제 체결가를 사용한다.

### 7. signal.rs 변경

`evaluate_signal()` 함수를 두 가지 용도로 분리:

```rust
/// [분봉용] SpreadCalculator 기반 시그널 평가 (워밍업/통계 갱신용)
/// → 제거 (분봉에서 시그널 평가하지 않음)

/// [틱용] 즉석 스프레드 기반 시그널 평가
pub fn evaluate_tick_signal(
    coin: &str,
    current_spread: f64,      // 틱에서 계산한 현재 스프레드
    mean: f64,                // 분봉 기반 rolling mean
    stddev: f64,              // 분봉 기반 rolling stddev
    position_mgr: &PositionManager,
    config: &ZScoreConfig,
) -> Result<Option<Signal>, StrategyError>
```

기존 `evaluate_signal()`은 내부적으로 `SpreadCalculator`에서 mean/stddev와 current_spread를 모두 가져왔다. 틱 기반에서는 current_spread가 외부(틱)에서 오고, mean/stddev는 SpreadCalculator에서 오므로 인터페이스를 분리한다.

---

## 제거 대상

### 파일 삭제

| 파일 | 이유 |
|------|------|
| `crates/arb-strategy/src/zscore/simulator.rs` | 백테스트 제거 |
| `crates/arb-strategy/src/zscore/sweep.rs` | 파라미터 스윕 제거 |
| `crates/arb-strategy/src/zscore/slippage.rs` | 슬리피지 모델 제거 |
| `crates/arb-strategy/src/output/console.rs` | 백테스트 콘솔 출력 |
| `crates/arb-strategy/src/output/csv.rs` | 백테스트 CSV 출력 |
| `crates/arb-strategy/src/output/mod.rs` | output 모듈 전체 |
| `examples/zscore_backtest.rs` | 백테스트 예제 |
| `examples/zscore_parameter_sweep.rs` | 스윕 예제 |

### 코드 제거 (파일 내부)

| 파일 | 제거 내용 |
|------|----------|
| `crates/arb-strategy/src/zscore/mod.rs` | `pub mod simulator`, `pub mod sweep`, `pub mod slippage` |
| `crates/arb-strategy/src/lib.rs` 또는 `mod.rs` | `pub mod output` |
| `crates/arb-strategy/src/zscore/config.rs` | 아래 필드 목록 참조 |
| `crates/arb-strategy/src/zscore/spread.rs` | `usdt_krw_ff`, `usdt_krw_window`, USDT/KRW forward-fill 로직 |
| `crates/arb-strategy/src/zscore/monitor.rs` | USDT/KRW 스트림 구독, `usdt_krw_last_trade`, USDT 캔들 워밍업, `use crate::output::csv`, `use crate::zscore::simulator::{TimeseriesRecord, fetch_all_candles}`, `timeseries_records` 관련 코드 |
| `crates/arb-strategy/src/zscore/coin_selector.rs` | Upbit KRW-USDT 티커 기반 환율 환산 → `ForexCache` 사용으로 전환 |
| `crates/arb-strategy/Cargo.toml` | `csv` 의존성 제거 (output 삭제 시) |

#### config.rs 제거 대상 필드

`ZScoreConfig`:
- `backtest_period_minutes`
- `volume_filter_enabled`
- `max_participation_rate`
- `slippage_base_bps`
- `slippage_impact_coeff`
- `output_dir`

`RawZScoreConfig`:
- 대응하는 위 필드들
- `RawSweepConfig` 관련: `into_sweep_config()` 메서드, `sweep` 필드

관련 테스트:
- `test_from_toml_str_with_sweep()`, `test_from_toml_str_with_sweep_defaults()`, `test_from_toml_str_without_sweep()`

#### monitor.rs에서 이동이 필요한 코드

`simulator.rs` 삭제 전, 다음 항목을 별도 모듈로 이동:
- `fetch_all_candles()` → `common/candle_fetcher.rs` (워밍업에서 계속 사용)
- `TimeseriesRecord` → 삭제 (시계열 기록 기능은 로그로 대체)

#### MinuteCloses 타입 변경

```rust
// 변경 전: 3-tuple (Upbit closes, USDT/KRW, Bybit closes)
type MinuteCloses = (
    HashMap<String, Option<Decimal>>,
    Option<Decimal>,                    // ← 제거
    HashMap<String, Option<Decimal>>,
);

// 변경 후: 2-tuple
type MinuteCloses = (
    HashMap<String, Option<Decimal>>,
    HashMap<String, Option<Decimal>>,
);
```

---

## 구현 플랜

### Phase 1: arb-forex crate 생성

1. `crates/arb-forex/` 디렉토리 및 `Cargo.toml` 생성
2. `ForexCache` 구현: `get_usd_krw()`, `get_daily_rates()`
3. Yahoo Finance API 파싱 (`v8/finance/chart/USDKRW=X`)
4. TTL 기반 캐싱 (`RwLock<Option<CachedRate>>`)
5. `ForexError` 에러 타입
6. 테스트: 캐시 TTL 만료, 파싱, 에러 처리
7. workspace `Cargo.toml`에 멤버 추가

### Phase 2a: 독립 모듈 제거

`monitor.rs`에 의존하지 않는 코드부터 제거한다.

1. 파일 삭제: `sweep.rs`, `slippage.rs`, `output/` 디렉토리
2. 예제 삭제: `zscore_backtest.rs`, `zscore_parameter_sweep.rs`
3. `mod.rs` 정리: `pub mod sweep`, `pub mod slippage` 제거
4. `lib.rs` 정리: `pub mod output` 제거
5. `config.rs` 정리: 백테스트/슬리피지 전용 설정 필드 제거, `RawSweepConfig` 관련 코드 제거
6. `Cargo.toml` 정리: `csv` 의존성 제거
7. `cargo build` 통과 확인

### Phase 2b: simulator.rs 제거 + 의존 코드 이동

`simulator.rs`는 `monitor.rs`에서 참조하므로 먼저 의존 코드를 이동한다.

1. `fetch_all_candles()` → `common/candle_fetcher.rs`로 이동 (워밍업에서 계속 사용)
2. `monitor.rs`의 import 변경: `crate::zscore::simulator::fetch_all_candles` → `crate::common::candle_fetcher::fetch_all_candles`
3. `monitor.rs`에서 `TimeseriesRecord` 관련 코드 제거 (시계열 기록은 로그로 대체)
4. `monitor.rs`에서 `use crate::output::csv` 제거
5. 파일 삭제: `simulator.rs`
6. `mod.rs` 정리: `pub mod simulator` 제거
7. `cargo build` 통과 확인

### Phase 3: SpreadCalculator 2-way 전환

1. `spread.rs`: `update()` 시그니처 변경 (`usdt_krw: Option<Decimal>` → `usd_krw: f64`)
2. USDT/KRW forward-fill 상태 제거
3. `usdt_krw_window` 제거
4. 스프레드 공식 변경: `upbit_krw / usd_krw`
5. 기존 테스트 수정 (환율 파라미터 변경)

### Phase 4: monitor.rs 환율 전환 + KRW-USDT 제거

1. `ForexCache` 의존성 추가 (Arc로 공유, 환율 갱신 task 분리)
2. 워밍업: Yahoo Finance 일봉 조회 (코인과 무관하므로 **1회만 호출**) → Upbit USDT 캔들 조회 제거
3. `MinuteCandleBuilder`: `usdt_krw_last_trade` 필드 제거, `MinuteCloses` 3-tuple → 2-tuple 변경
4. `on_upbit_trade`: KRW-USDT 분기 제거
5. WebSocket 구독: KRW-USDT 마켓 구독 제거
6. 분봉 완결: USDT/KRW close 제거, `SpreadCalculator.update()` 호출 시 `forex_cache.get_cached_rate()` 사용
7. `coin_selector.rs`: Upbit KRW 볼륨→USDT 환산에서 `KRW-USDT` 티커 대신 `ForexCache` 사용으로 전환
8. `usdt_krw_window()` 호출부 정리 (TTL 만료, liquidation 등에서 ForexCache 사용)

### Phase 5: 틱 기반 시그널 구현

1. `spread.rs`: `IncrementalStats` 구조체 추가, `cached_stats(coin)` 메서드 추가
2. `signal.rs`: `evaluate_tick_signal()` 함수 추가 (z_score, current_spread를 외부에서 받는 구조)
3. `signal.rs`: 기존 `evaluate_signal()` 제거, 기존 테스트 6개를 새 인터페이스에 맞게 재작성
4. `monitor.rs`: `check_tick_signal()` 메서드 추가 (에러 시 warn + continue)
5. 이벤트 루프 수정:
   - Upbit Trade 수신 → `check_tick_signal(coin)` (에러 시 warn)
   - Bybit BestQuote 수신 → `check_tick_signal(coin)` (에러 시 warn)
6. 분봉 타이머: 시그널 평가 로직 제거 (incremental stats 갱신만 유지)
7. 포지션 진입/청산: 틱 시점에서 즉시 실행
8. 환율 갱신: 별도 `tokio::spawn` task로 10분 간격 갱신
9. `estimate_avg_price()` 함수 구현 (호가 볼륨 기반 예상 평단가)
10. 진입/청산 시 호가 조회 → 평단가 계산 → VirtualPosition/PnL에 반영
11. 필요 시 Bybit orderbook 구독 또는 REST 호가 조회 추가

### Phase 6: VirtualPosition / ClosedPosition / PnL 정리

1. `VirtualPosition.entry_usdt_krw` → `entry_usd_krw: f64`
2. `ClosedPosition.entry_usdt_krw` → `entry_usd_krw: f64`, `exit_usdt_krw` → `exit_usd_krw: f64`
3. `close_position()` 시그니처: `exit_usdt_krw: Decimal` → `exit_usd_krw: f64`
4. PnL 계산: USD/KRW 기반으로 변경
5. `monitor.rs`의 VirtualPosition 생성부, close_position 호출부 수정
6. 기존 테스트 수정 (signal.rs 테스트의 VirtualPosition 생성 포함)

### Phase 7: 테스트 및 검증

1. `cargo test -p arb-strategy` — 모든 테스트 통과
2. `cargo test -p arb-forex` — forex 테스트 통과
3. `cargo clippy --workspace` — 경고 0
4. `cargo fmt --check` — 포맷 확인
5. `zscore_monitor` 예제 실행 → 환율/틱 시그널 동작 확인

---

## 파일 변경 요약

### 신규 생성

| 파일 | 설명 |
|------|------|
| `crates/arb-forex/Cargo.toml` | forex crate 설정 |
| `crates/arb-forex/src/lib.rs` | ForexCache, ForexError, Yahoo Finance API |
| `crates/arb-strategy/src/common/candle_fetcher.rs` | `fetch_all_candles()` (simulator.rs에서 이동) |

### 수정

| 파일 | 변경 내용 |
|------|----------|
| `Cargo.toml` (workspace) | `arb-forex` 멤버 추가 |
| `crates/arb-strategy/Cargo.toml` | `arb-forex` 의존성 추가, 불필요 의존성 제거 |
| `crates/arb-strategy/src/zscore/mod.rs` | simulator/sweep/slippage 모듈 제거 |
| `crates/arb-strategy/src/lib.rs` | output 모듈 제거 |
| `crates/arb-strategy/src/zscore/spread.rs` | 2-way 입력, USDT/KRW 제거 |
| `crates/arb-strategy/src/zscore/signal.rs` | `evaluate_tick_signal()` 추가/리팩터 |
| `crates/arb-strategy/src/zscore/orderbook.rs` (신규) | `estimate_avg_price()` 호가 기반 평단가 계산 |
| `crates/arb-strategy/src/zscore/monitor.rs` | 환율 전환, KRW-USDT 제거, 틱 시그널 추가 |
| `crates/arb-strategy/src/zscore/config.rs` | 백테스트/슬리피지 설정 제거, forex TTL 설정 추가 |
| `crates/arb-strategy/src/zscore/position.rs` | `entry_usdt_krw` → `entry_usd_krw` |
| `crates/arb-strategy/src/zscore/pnl.rs` | USD/KRW 기반 PnL |
| `crates/arb-strategy/src/zscore/coin_selector.rs` | KRW-USDT 티커 기반 환율 환산 → ForexCache 사용으로 전환 |
| `crates/arb-strategy/src/common/mod.rs` | `pub mod candle_fetcher` 추가 |
| `src/lib.rs` | `arb-forex` re-export 추가 |
| `examples/zscore_monitor.rs` | ForexCache 초기화 추가, 환율 전환 |

### 삭제

| 파일 | 이유 |
|------|------|
| `crates/arb-strategy/src/zscore/simulator.rs` | 백테스트 제거 |
| `crates/arb-strategy/src/zscore/sweep.rs` | 파라미터 스윕 제거 |
| `crates/arb-strategy/src/zscore/slippage.rs` | 슬리피지 모델 제거 |
| `crates/arb-strategy/src/output/` (전체) | 백테스트 출력 |
| `examples/zscore_backtest.rs` | 백테스트 예제 |
| `examples/zscore_parameter_sweep.rs` | 스윕 예제 |

---

## 알려진 한계

- **USDT ≈ USD 가정**: Bybit 가격은 USDT 표시이며, 1 USDT = 1 USD로 가정한다. 평상시 편차는 ±0.05% 수준으로 무시 가능하나, **tail event**(UST/LUNA 사태, SVB/USDC depeg 등) 시 USDT가 $0.97~$1.03까지 변동한 사례가 있다. 이때 스프레드 오차가 수수료 마진(0.21%)의 20%+ 수준에 달할 수 있다. 향후 USDT/USD 모니터링 및 circuit breaker 도입을 고려한다.
- **워밍업 환율 해상도**: 워밍업 시 일봉 환율을 사용하므로, 같은 날의 모든 분봉에 동일한 환율이 적용된다. USD/KRW 일중 변동은 일반적으로 0.3% 미만이므로 워밍업 윈도우(~1시간) 구축에 충분하다.
- **Yahoo Finance API 안정성**: Yahoo Finance는 공식 API가 아니며 변경될 수 있다. 조회 실패 시 마지막 캐시 값을 유지하고, 장기 장애 시 에러 로그를 출력한다.
- **백테스트 부재**: 이 스펙 이후 백테스트 기능이 없다. 전략 파라미터 검증은 실시간 시뮬레이션으로 수행한다. 추후 필요 시 백테스트를 재구현한다.
- **틱 시그널 노이즈**: 개별 틱은 분봉 close보다 노이즈가 크다. 빈번한 거짓 시그널 방지를 위해 수수료 수익성 검증(expected_profit > fee)이 필수적이며, 이는 유지된다.
