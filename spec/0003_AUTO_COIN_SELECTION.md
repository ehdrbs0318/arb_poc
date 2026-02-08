# 0003_AUTO_COIN_SELECTION

## 사용자의 요청

감시 대상 코인을 설정 파일 수동 지정에서 **거래량과 변동성 기반 자동 선택**으로 전환:

> "현재는 감시하는 코인을 설정 파일을 통해서 지정하는데, 이러면 변동성과 볼륨이 큰 코인에서 더 많은 기회가 있는데 이 기회를 놓칠 수 있어. 거래소 코인 볼륨과 변동성에 의한 자동 선택으로 바꾸고 싶어"

### 배경

현재 `strategy.toml`에서 `coins = ["BTC"]`처럼 수동으로 감시 대상을 지정한다. 이 방식은:

- **고정적**: 시장 상황이 변해도 같은 코인만 감시
- **기회 손실**: 특정 시점에 변동성/볼륨이 급증하는 코인의 차익 기회를 놓침
- **수동 관리 필요**: 사용자가 시장을 모니터링하여 직접 코인 목록을 갱신해야 함

### 확정 요구사항

| 항목 | 설정 |
|------|------|
| 적용 범위 | **라이브(monitor) 전용**, 백테스트는 기존 방식 유지 |
| 후보 풀 | Upbit KRW 마켓 ∩ Bybit USDT Linear 마켓 공통 상장 코인 |
| 제외 규칙 | 스테이블코인 자동 제외 + 설정 가능한 블랙리스트 |
| 볼륨 기준 | **1시간 거래량** (1h 캔들 API), 최소 임계값 이상 필터 |
| 변동성 기준 | **24h (high - low) / low × 100%** (Ticker API) |
| 선택 방식 | 볼륨 임계값 필터 → 변동성 내림차순 정렬 → 상위 N개 |
| 선택 개수 | 기본 5개 (설정 가능) |
| 갱신 주기 | 기본 10분 (설정 가능) |
| 탈락 코인 처리 | 포지션 있으면 청산까지 감시 유지 (최대 24시간 TTL), 없으면 데이터 삭제 |
| 포지션 TTL | 탈락 후 24시간 내 미청산 시 강제 청산 (zombie position 방지) |
| WebSocket | 끊김 없이 동적 구독 변경 (subscribe/unsubscribe) |
| 설정 호환 | `auto_select = true/false`로 기존 수동 방식과 선택 가능 |
| 볼륨 최소 임계값 기본값 | 1,000,000 USDT |

---

## 설계

### 전체 흐름

```
┌─────────────────────────────────────────────────────────────┐
│                    CoinSelector (신규)                       │
│                                                             │
│  1. Upbit 전종목 Ticker + Bybit 전종목 Ticker → 교집합       │
│  2. 스테이블코인/블랙리스트 제외                               │
│  2-b. 24h 거래대금으로 1차 필터 (하위 50% 사전 제거)            │
│  3. 통과 후보만 1h 캔들 → 1시간 볼륨 계산                      │
│  4. 볼륨 임계값 필터                                         │
│  5. 변동성 = (high_24h - low_24h) / low_24h × 100           │
│  6. 변동성 내림차순 정렬 → 상위 N개 선택                       │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│                   ZScoreMonitor (수정)                       │
│                                                             │
│  시작 시:                                                    │
│    auto_select=true → CoinSelector.select() → 코인 목록      │
│    auto_select=false → config.coins 사용 (기존 동작)          │
│                                                             │
│  10분마다 (auto_select=true):                                │
│    CoinSelector.select() → 새 목록                           │
│    diff(현재, 새) →                                          │
│      추가 코인: warmup → WebSocket subscribe_markets 추가     │
│      탈락 코인(포지션 없음): 데이터 삭제, unsubscribe_markets  │
│      탈락 코인(포지션 있음): 감시 유지 (24h TTL 후 강제 청산)   │
│                                                             │
│  이벤트 루프: 기존과 동일 (분봉 기반 시그널 평가)               │
└─────────────────────────────────────────────────────────────┘
```

### 코인 선택 알고리즘

```
fn select(upbit: &U, bybit: &B, config: &AutoSelectConfig) -> Vec<String>:
    // 1단계: 양쪽 거래소 전종목 Ticker 조회 (각 1 API 호출)
    upbit_tickers = upbit.get_all_tickers()       // 24h high/low/거래대금 포함
    bybit_tickers = bybit.get_all_tickers()       // 24h high/low/turnover24h 포함

    // 2단계: 교집합 (base coin 기준)
    common_coins = intersect(upbit_coins, bybit_coins)

    // 3단계: 제외
    common_coins -= STABLECOINS  // ["USDT", "USDC", "DAI", "TUSD", "BUSD", "FDUSD", "PYUSD", ...]
    common_coins -= config.blacklist

    // 3-b단계: 24h 거래대금 1차 필터 (API 호출 절감)
    //   Ticker의 acc_trade_price_24h / turnover24h로 하위 50% 사전 제거
    pre_filtered = common_coins.sort_by(24h_volume).top_half()

    // 4단계: 1시간 볼륨 조회 (통과 후보만, 코인당 1h 캔들 1개씩, 양쪽)
    for coin in pre_filtered:
        upbit_1h = upbit.get_candles("KRW-{coin}", Minute60, 1)
        bybit_1h = bybit.get_candles("{coin}USDT", Minute60, 1)
        // 양쪽 중 낮은 쪽의 USDT 환산 볼륨 사용
        volume_usdt = min(upbit_1h_volume_usdt, bybit_1h_volume_usdt)

    // 5단계: 볼륨 임계값 필터
    candidates = coins where volume_usdt >= config.min_volume_usdt

    // 6단계: 변동성 계산 및 정렬
    for coin in candidates:
        volatility = (high_24h - low_24h) / low_24h × 100  // Ticker에서 가져온 값
    sort by volatility DESC

    // 7단계: 상위 N개 반환 (후보 < max_coins이면 전부 반환)
    return candidates[..min(config.max_coins, candidates.len())]
```

### MarketData trait 확장

현재 `get_ticker()`는 마켓 코드를 인자로 받아야 하므로, 전종목 조회가 불가능하다. 교집합 추출을 위해 전종목 Ticker 조회 메서드를 추가한다:

```rust
pub trait MarketData: Send + Sync {
    // ... 기존 메서드 ...

    /// 전종목 Ticker를 조회합니다.
    ///
    /// Upbit: GET /v1/market/all → 마켓 목록 → GET /v1/ticker (전체)
    /// Bybit: GET /v5/market/tickers?category=linear (전종목 1회)
    fn get_all_tickers(&self) -> impl Future<Output = ExchangeResult<Vec<Ticker>>> + Send;
}
```

### MarketStream trait 확장

현재 `subscribe()`는 기존 구독을 종료하고 새로운 구독으로 **교체**하는 방식이다. 동적 추가/제거를 위해 trait에 메서드를 추가한다:

```rust
#[async_trait]
pub trait MarketStream: Send + Sync {
    // 기존
    fn stream_name(&self) -> &str;
    async fn subscribe(&self, markets: &[&str]) -> ExchangeResult<Receiver<MarketEvent>>;
    async fn unsubscribe(&self) -> ExchangeResult<()>;

    // 신규: 기존 연결을 유지하면서 마켓 추가/제거 (default impl 제공)
    async fn subscribe_markets(&self, _markets: &[&str]) -> ExchangeResult<()> {
        Err(ExchangeError::Unsupported("subscribe_markets not implemented".into()))
    }
    async fn unsubscribe_markets(&self, _markets: &[&str]) -> ExchangeResult<()> {
        Err(ExchangeError::Unsupported("unsubscribe_markets not implemented".into()))
    }
}
```

> **하위 호환**: default impl을 제공하여 기존 구현체(factory.rs 어댑터 포함)의 컴파일 에러를 방지한다.

### WebSocket command 채널 패턴

현재 WebSocket 루프(`upbit_ws_loop`, `bybit_ws_loop`)는 연결 시 구독 메시지를 1회 전송하고, 이후 write half에 접근할 방법이 없다. 동적 구독을 위해 **command 채널**을 추가한다:

```rust
/// WebSocket 루프에 전달하는 명령.
enum StreamCommand {
    /// 마켓 추가 (Upbit: 전체 재구독, Bybit: 개별 subscribe)
    Subscribe(Vec<String>),
    /// 마켓 제거 (Upbit: 전체 재구독, Bybit: 개별 unsubscribe)
    Unsubscribe(Vec<String>),
}
```

`StreamState`에 `command_tx: mpsc::Sender<StreamCommand>`를 추가하고, WebSocket 루프의 `select!`에서 command를 수신하여 write half로 메시지를 전송한다.

- **Upbit**: `Subscribe`/`Unsubscribe` 수신 시 내부 마켓 목록 갱신 → 전체 목록으로 새 구독 메시지 전송
- **Bybit**: `Subscribe` 수신 시 `{"op": "subscribe", "args": [...]}`, `Unsubscribe` 수신 시 `{"op": "unsubscribe", "args": [...]}` 전송

### SpreadCalculator 확장

코인 동적 추가/제거를 위한 메서드:

```rust
impl SpreadCalculator {
    // 기존
    fn new(coins: &[String], window_size: usize) -> Self;

    // 신규
    fn add_coin(&mut self, coin: &str);        // 새 코인의 윈도우 초기화
    fn remove_coin(&mut self, coin: &str);     // 코인 데이터 삭제
    fn active_coins(&self) -> Vec<&str>;       // 현재 감시 중인 코인 목록
}
```

### 설정

`strategy.toml` 확장:

```toml
[zscore]
# 기존 수동 방식 (auto_select = false일 때 사용)
coins = ["BTC"]

# 자동 선택 설정
auto_select = false           # true: 자동 선택, false: 수동 (기본 false)
max_coins = 5                 # 최대 감시 코인 수 (기본 5)
reselect_interval_min = 10    # 재선택 주기 (분, 기본 10)
min_volume_1h_usdt = 1000000  # 1시간 최소 거래량 (USDT, 기본 1,000,000)
blacklist = []                # 제외 코인 목록 (예: ["SHIB", "DOGE"])
position_ttl_hours = 24       # 탈락 코인 포지션 최대 유지 시간 (기본 24)
```

### 갱신 시 전환 로직

```
현재 감시: [BTC, ETH, SOL, XRP, DOGE]
새 선택:   [BTC, ETH, SOL, AVAX, LINK]
포지션:    XRP에 오픈 포지션 있음

→ 추가: [AVAX, LINK]
    1. warmup(AVAX), warmup(LINK)  — 캔들 로드 (~2초)
    2. subscribe_markets(["KRW-AVAX", "AVAXUSDT", "KRW-LINK", "LINKUSDT"])
    3. SpreadCalculator.add_coin("AVAX"), add_coin("LINK")

→ 유지: [BTC, ETH, SOL] — 변경 없음

→ 유지 (포지션): [XRP] — 선택 목록에서 탈락했지만 포지션 있으므로 감시 계속
    탈락 시각 기록 → 24시간 TTL 경과 시 강제 청산
    (청산 시그널로 포지션 닫히거나, TTL 만료 시 제거)

→ 제거: [DOGE] — 포지션 없음
    1. unsubscribe_markets(["KRW-DOGE", "DOGEUSDТ"])
    2. SpreadCalculator.remove_coin("DOGE")
```

### API 호출 분석

24h 거래대금 1차 필터를 적용하여 API 호출을 최적화한다.

#### 초기 선택 (시작 시 1회)

| 단계 | API | 호출 수 | 소요 시간 |
|------|-----|---------|----------|
| 전종목 Ticker (양쪽) | `get_all_tickers()` × 2 | 2회 | ~1초 |
| 24h 1차 필터 | 로컬 계산 | 0회 | 즉시 |
| 1h 캔들 (1차 통과 ~50개) | 코인당 2회 (Upbit + Bybit) | ~100회 | ~10초 |
| 워밍업 (5개) | 코인당 ~10회 | ~50회 | ~5초 |
| **합계** | | ~152회 | **~16초** |

#### 10분마다 재선택

| 단계 | API | 호출 수 | 소요 시간 |
|------|-----|---------|----------|
| 전종목 Ticker + 1h 캔들 | 위와 동일 | ~102회 | ~11초 |
| 신규 코인 워밍업 (0~2개) | 코인당 ~10회 | 0~20회 | 0~2초 |
| **합계** | | ~102~122회 | **~11~13초** |

10분 = 600초 중 ~13초 사용. 충분한 여유.

#### Rate Limit 준수

| 거래소 | 제한 | 최대 사용량 (100ms 딜레이 적용) | 여유율 |
|--------|------|-------------------------------|--------|
| Upbit Quotation | ~10 req/s | ~50회 × 100ms = 5초, 1 req/s | 충분 |
| Bybit Market | ~10 req/s | ~50회 × 100ms = 5초, 1 req/s | 충분 |

#### 후보 부족 시 동작

볼륨 임계값을 통과하는 코인이 `max_coins`보다 적을 경우, 통과한 코인 전부를 사용한다 (0개 포함). 새벽 시간대(KST 03:00~06:00) 등 저유동성 구간에서 감시 코인이 0개가 될 수 있으며, 이는 의도된 동작이다.

---

## 구현 플랜

### Phase 1: 설정 확장 (`config.rs`)

`ZScoreConfig`에 자동 선택 관련 필드 추가.

```rust
// ZScoreConfig 추가 필드
pub auto_select: bool,                   // 기본 false
pub max_coins: usize,                    // 기본 5
pub reselect_interval_min: u64,          // 기본 10
pub min_volume_1h_usdt: Decimal,         // 기본 1_000_000
pub blacklist: Vec<String>,              // 기본 빈 벡터
pub position_ttl_hours: u64,             // 기본 24
```

`RawZScoreConfig`에도 대응 필드 추가하고 TOML 파싱 구현.

### Phase 2: CoinSelector 모듈 (`coin_selector.rs`)

새 모듈을 `crates/arb-strategy/src/zscore/coin_selector.rs`에 생성.

```rust
/// 코인 후보 정보.
#[derive(Debug, Clone)]
pub struct CoinCandidate {
    pub coin: String,
    pub volume_1h_usdt: f64,
    pub volatility_24h_pct: f64,
}

/// 자동 코인 선택기.
pub struct CoinSelector<'a, U: MarketData, B: MarketData> {
    upbit: &'a U,
    bybit: &'a B,
}

impl<U: MarketData, B: MarketData> CoinSelector<'_, U, B> {
    /// 볼륨/변동성 기반으로 상위 코인을 선택합니다.
    pub async fn select(
        &self,
        max_coins: usize,
        min_volume_1h_usdt: Decimal,
        blacklist: &[String],
    ) -> Result<Vec<CoinCandidate>, StrategyError>;
}
```

주요 로직:
1. 양쪽 Ticker API → 교집합 추출
2. 스테이블코인 + 블랙리스트 제외
3. **1차 필터 (최적화)**: Ticker의 24h 거래대금으로 하위 50% 사전 제거
4. 통과 후보만 1h 캔들 조회 → 1시간 볼륨 계산
5. 볼륨 임계값 필터 → 변동성 정렬 → 상위 N개 반환

### Phase 3: MarketData/MarketStream trait 확장

**MarketData** (`traits.rs`):
- `get_all_tickers()` 메서드 추가 (전종목 Ticker 조회)

**MarketStream** (`stream.rs`):
- `subscribe_markets()`/`unsubscribe_markets()` 메서드 추가 (default impl 제공)
- `StreamCommand` enum 정의

### Phase 4: Upbit/Bybit 구현 확장 (command 채널 리팩터링)

WebSocket 루프에 command 채널을 추가하여 동적 구독을 지원한다. 이는 `StreamState`에 `command_tx`를 추가하고, WebSocket 루프의 `select!`에 command 수신을 추가하는 리팩터링이 필요하다.

**Upbit** (`arb-exchanges/src/upbit/stream.rs`):
- `StreamState`에 `command_tx: mpsc::Sender<StreamCommand>` 추가
- `upbit_ws_loop`의 `select!`에 command 수신 분기 추가
- `subscribe_markets()`: command_tx로 `Subscribe` 전송 → 루프에서 전체 마켓 재구독 메시지 전송
- `unsubscribe_markets()`: command_tx로 `Unsubscribe` 전송 → 루프에서 마켓 제거 후 재구독
- `get_all_tickers()`: `GET /v1/market/all` → KRW 마켓 필터 → `GET /v1/ticker` 구현

**Bybit** (`arb-exchanges/src/bybit/stream.rs`):
- `StreamState`에 `command_tx: mpsc::Sender<StreamCommand>` 추가
- `bybit_ws_loop`의 `select!`에 command 수신 분기 추가
- `subscribe_markets()`: command_tx로 `Subscribe` 전송 → 루프에서 `{"op":"subscribe"}` 전송
- `unsubscribe_markets()`: command_tx로 `Unsubscribe` 전송 → 루프에서 `{"op":"unsubscribe"}` 전송
- `get_all_tickers()`: `GET /v5/market/tickers?category=linear` (전종목 1회) 구현

**Factory** (`arb-exchanges/src/factory.rs`):
- 어댑터에 신규 메서드 위임 구현 추가

### Phase 5: SpreadCalculator 확장 (`spread.rs`)

```rust
impl SpreadCalculator {
    /// 새 코인을 추가하고 빈 윈도우를 초기화합니다.
    pub fn add_coin(&mut self, coin: &str);

    /// 코인을 제거하고 관련 데이터를 삭제합니다.
    pub fn remove_coin(&mut self, coin: &str);

    /// 현재 감시 중인 코인 목록을 반환합니다.
    pub fn active_coins(&self) -> Vec<&str>;
}
```

### Phase 6: ZScoreMonitor 수정 (`monitor.rs`)

1. **시작 시**: `auto_select = true`이면 `CoinSelector.select()` 호출하여 초기 코인 목록 결정
2. **이벤트 루프**: 10분 타이머 추가, 만료 시 재선택 수행
3. **전환 로직**: diff 계산 → 추가/제거/유지 분류 → warmup + subscribe/unsubscribe + SpreadCalculator 갱신
4. **TTL 관리**: 탈락 코인의 탈락 시각 기록, 24시간 초과 시 강제 청산
5. **비동기 워밍업**: `tokio::spawn`으로 별도 task에서 워밍업 실행 (이벤트 루프 블로킹 방지)
6. **에러 복구**: 재선택 실패 시 이전 목록 유지 (warn 로그)

```rust
// 탈락 코인 TTL 추적
let mut dropped_at: HashMap<String, DateTime<Utc>> = HashMap::new();

// 이벤트 루프에 추가
_ = reselect_timer.tick(), if config.auto_select => {
    let new_coins = match selector.select(...).await {
        Ok(coins) => coins,
        Err(e) => {
            warn!(error = %e, "코인 재선택 실패, 이전 목록 유지");
            continue;
        }
    };

    let (to_add, to_remove, to_keep_with_position) = diff_coins(
        &current_coins,
        &new_coins,
        &position_mgr,
    );

    // 추가 코인: tokio::spawn으로 워밍업 (블로킹 방지)
    let warmup_handle = tokio::spawn(async move {
        for coin in &to_add {
            warmup_single(coin, &upbit, &bybit, ...).await?;
        }
        Ok(to_add)
    });
    // pending_warmups에 저장, 완료 시 select!에서 subscribe + add_coin

    // 제거 코인 (포지션 없음): 즉시 정리
    for coin in &to_remove {
        spread_calc.remove_coin(coin);
        // unsubscribe_markets 호출
        dropped_at.remove(coin);
    }

    // 탈락+포지션 코인: 탈락 시각 기록
    for coin in &to_keep_with_position {
        dropped_at.entry(coin.clone()).or_insert(Utc::now());
    }

    // TTL 만료 체크: 24시간 초과 → 강제 청산
    for (coin, dropped_time) in &dropped_at {
        if Utc::now() - *dropped_time > Duration::hours(config.position_ttl_hours) {
            warn!(coin = coin.as_str(), "TTL 만료: 강제 청산");
            // 시장가 강제 청산 처리
        }
    }

    current_coins = updated_list;
}
```

### Phase 7: 테스트

| 테스트 | 모듈 | 시나리오 |
|--------|------|---------|
| `test_stablecoin_excluded` | coin_selector | USDT, USDC, DAI, FDUSD 등 자동 제외 확인 |
| `test_blacklist_excluded` | coin_selector | 블랙리스트 코인 제외 확인 |
| `test_volume_filter` | coin_selector | 임계값 미달 코인 제외 확인 |
| `test_volatility_sort` | coin_selector | 변동성 내림차순 정렬 확인 |
| `test_max_coins_limit` | coin_selector | 상위 N개만 반환 확인 |
| `test_select_empty_intersection` | coin_selector | 교집합 빈 경우 빈 Vec 반환 |
| `test_select_fewer_than_max` | coin_selector | 후보 < max_coins일 때 전부 반환 |
| `test_spread_calc_add_remove` | spread | 동적 코인 추가/제거 동작 확인 |
| `test_add_existing_coin_idempotent` | spread | 이미 존재하는 코인 add 시 데이터 유지 |
| `test_remove_nonexistent_coin` | spread | 없는 코인 remove 시 패닉 없음 |
| `test_diff_coins_with_position` | monitor | 포지션 있는 탈락 코인 유지 확인 |
| `test_diff_coins_without_position` | monitor | 포지션 없는 탈락 코인 제거 확인 |
| `test_reselect_no_change` | monitor | 재선택 결과 동일 시 불필요한 변경 안 함 |
| `test_ttl_force_close` | monitor | TTL 만료 시 강제 청산 확인 |
| `test_reselect_failure_keeps_previous` | monitor | 재선택 실패 시 이전 목록 유지 |
| `test_auto_select_false_uses_config` | config | auto_select=false 시 기존 동작 확인 |
| `test_config_defaults` | config | 기본값 검증 (TTL 포함) |

---

## 파일 변경 목록

| 파일 | 변경 유형 | 설명 |
|------|-----------|------|
| `crates/arb-strategy/src/zscore/config.rs` | **수정** | 자동 선택 설정 필드 + TTL 추가 |
| `crates/arb-strategy/src/zscore/coin_selector.rs` | **신규** | CoinSelector 모듈 |
| `crates/arb-strategy/src/zscore/mod.rs` | **수정** | coin_selector 모듈 등록 |
| `crates/arb-strategy/src/zscore/spread.rs` | **수정** | add_coin/remove_coin/active_coins 메서드 추가 |
| `crates/arb-strategy/src/zscore/monitor.rs` | **수정** | 자동 선택 통합, 재선택 타이머, TTL, 비동기 워밍업, 에러 복구 |
| `crates/arb-exchange/src/traits.rs` | **수정** | MarketData에 `get_all_tickers()` 추가 |
| `crates/arb-exchange/src/stream.rs` | **수정** | MarketStream에 subscribe/unsubscribe_markets + StreamCommand 추가 |
| `crates/arb-exchanges/src/upbit/stream.rs` | **수정** | command 채널 리팩터링 + subscribe/unsubscribe_markets 구현 |
| `crates/arb-exchanges/src/bybit/stream.rs` | **수정** | command 채널 리팩터링 + subscribe/unsubscribe_markets 구현 |
| `crates/arb-exchanges/src/upbit/client.rs` | **수정** | `get_all_tickers()` 구현 (`/v1/market/all` + `/v1/ticker`) |
| `crates/arb-exchanges/src/bybit/client.rs` | **수정** | `get_all_tickers()` 구현 (`/v5/market/tickers?category=linear`) |
| `crates/arb-exchanges/src/factory.rs` | **수정** | 어댑터에 신규 메서드 위임 구현 |
| `strategy.toml` | **수정** | 자동 선택 설정 예시 추가 |

### 변경하지 않는 파일

| 파일 | 이유 |
|------|------|
| `signal.rs` | 코인 선택과 무관, 단일 코인 시그널 평가 로직 |
| `simulator.rs` | 백테스트 전용, 자동 선택 미적용 |
| `slippage.rs` | 슬리피지 모델 자체는 변경 없음 |
| `pnl.rs` | PnL 계산 로직 무관 |

---

## 알려진 한계

- **후보 스캔 비용**: ~100개 후보의 1h 캔들을 매 10분마다 조회하므로 API 호출이 많다. 1차 필터(24h 볼륨)로 후보를 줄여 최적화 가능하나, rate limit 내이므로 v1에서는 허용.
- **워밍업 지연**: 새 코인 추가 시 ~1초의 워밍업이 필요하며, 이 동안 해당 코인의 시그널 평가가 지연된다.
- **Upbit 재구독**: Upbit WebSocket은 개별 종목 추가/제거를 지원하지 않아, 목록 변경 시 전체 종목 재구독 메시지를 보내야 한다. 수십ms의 미미한 공백이 발생할 수 있다.
- **백테스트 미적용**: 자동 선택은 라이브 전용이다. 백테스트에서 자동 선택을 시뮬레이션하려면 과거 Ticker/볼륨 데이터가 필요하며, 이는 향후 과제.
- **변동성 급변**: 10분 주기로 재선택하므로, 그 사이에 변동성이 급변하는 코인은 반영이 늦을 수 있다.

---

## 체크리스트

### Phase 1: 설정 확장
- [ ] `ZScoreConfig`에 `auto_select`, `max_coins`, `reselect_interval_min`, `min_volume_1h_usdt`, `blacklist`, `position_ttl_hours` 필드 추가
- [ ] `RawZScoreConfig` TOML 파싱 추가
- [ ] 기본값 검증 (auto_select=false, max_coins=5, reselect=10, min_volume=1M, blacklist=[], ttl=24)
- [ ] `auto_select=false`일 때 기존 `coins` 필드 사용 확인

### Phase 2: CoinSelector 모듈
- [ ] `CoinCandidate` 구조체
- [ ] `CoinSelector::select()` 구현
- [ ] 스테이블코인 목록 정의 및 자동 제외 (USDT, USDC, DAI, TUSD, BUSD, FDUSD, PYUSD 등)
- [ ] 블랙리스트 제외
- [ ] 24h 거래대금 1차 필터 (하위 50% 사전 제거)
- [ ] 1h 캔들 볼륨 계산 (USDT 환산, 양쪽 min)
- [ ] 볼륨 임계값 필터
- [ ] 변동성 계산 및 정렬
- [ ] 상위 N개 선택 (후보 < max_coins이면 전부)
- [ ] 개별 코인 1h 캔들 조회 실패 시 해당 코인만 제외 (partial result)

### Phase 3: MarketData/MarketStream trait 확장
- [ ] `MarketData`에 `get_all_tickers()` 메서드 추가
- [ ] `MarketStream`에 `subscribe_markets()` 메서드 추가 (default impl)
- [ ] `MarketStream`에 `unsubscribe_markets()` 메서드 추가 (default impl)
- [ ] `StreamCommand` enum 정의

### Phase 4: WebSocket command 채널 리팩터링
- [ ] Upbit `StreamState`에 `command_tx` 추가
- [ ] Upbit `upbit_ws_loop` select!에 command 수신 분기 추가
- [ ] Upbit `subscribe_markets()` / `unsubscribe_markets()` 구현 (전체 재구독)
- [ ] Bybit `StreamState`에 `command_tx` 추가
- [ ] Bybit `bybit_ws_loop` select!에 command 수신 분기 추가
- [ ] Bybit `subscribe_markets()` / `unsubscribe_markets()` 구현 (개별 op)
- [ ] Upbit `get_all_tickers()` 구현 (`/v1/market/all` + `/v1/ticker`)
- [ ] Bybit `get_all_tickers()` 구현 (`/v5/market/tickers?category=linear`)
- [ ] Factory 어댑터에 신규 메서드 위임 구현

### Phase 5: SpreadCalculator 확장
- [ ] `add_coin()` 메서드 (idempotent: 이미 존재하면 무시)
- [ ] `remove_coin()` 메서드 (존재하지 않으면 무시)
- [ ] `active_coins()` 메서드

### Phase 6: ZScoreMonitor 수정
- [ ] 시작 시 자동 선택 분기
- [ ] 재선택 타이머 추가 (10분)
- [ ] diff 계산 로직 (추가/제거/유지 + 포지션 체크)
- [ ] 추가 코인 비동기 워밍업 (`tokio::spawn`, 이벤트 루프 블로킹 방지)
- [ ] 제거 코인 cleanup + unsubscribe
- [ ] 탈락+포지션 코인: 탈락 시각 기록 (`dropped_at` HashMap)
- [ ] TTL 만료 체크 (24h 초과 → 강제 청산)
- [ ] 재선택 실패 시 이전 목록 유지 fallback (warn 로그)

### Phase 7: 테스트
- [ ] coin_selector 단위 테스트 (7개)
- [ ] spread 동적 추가/제거 테스트 (3개)
- [ ] monitor diff/TTL/에러 복구 테스트 (5개)
- [ ] config 기본값/파싱 테스트 (2개)

### Phase 8: 검증
- [ ] `cargo test -p arb-strategy` 전체 통과
- [ ] `cargo clippy -p arb-strategy` 경고 0
- [ ] `cargo test -p arb-exchange` 전체 통과
- [ ] `cargo test -p arb-exchanges` 전체 통과
- [ ] `auto_select=false` 기존 동작 확인
- [ ] `auto_select=true` 실시간 테스트 (코인 선택 → 감시 → 재선택 확인)
