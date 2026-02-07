# 0000_Z_SCORE_STRATEGY_SIMULATE

## 사용자의 요청

Upbit(KRW/코인 현물)와 Bybit(USDT/코인 선물 short 포지션)을 통한 **Z-Score 기반 차익거래 전략의 수익성 시뮬레이션**.

### 전략 개요

1. Upbit에서 `{coin}/KRW` 가격과 `USDT/KRW` 가격(Upbit `KRW-USDT` 마켓)을 통해 `{coin}/USDT` 합성 가격을 산출
2. Bybit에서 `{coin}USDT` 무기한 선물(linear) 계약의 가격으로 short 포지션 가격을 산출
   - **백테스트**: 캔들 close 가격 사용
   - **실시간 시그널 감지**: Upbit last trade 가격 / Bybit bid1(best bid) 가격
   - **실시간 PnL 기록**: 매수/매도 방향 반영 (Upbit 매수=ask1, 매도=bid1 / Bybit short=bid1, 청산=ask1)
3. **스프레드(%)** = (Bybit short 가격 - Upbit USDT 환산 가격) / Upbit USDT 환산 가격 × 100
4. 1분 캔들 1440개(1일치)의 롤링 평균/표준편차로 **상대 스프레드(%)에 대한 Z-Score** 계산
5. Z-Score가 진입 임계값 이상 AND 수수료 차감 후 수익이면 AND 가용 자본 충분하면 → Upbit 현물 매수 + Bybit 선물 short 진입 (1배 레버리지)
6. Z-Score가 청산 임계값 이하로 **평균(mean)에 수렴**하면 → 양쪽 포지션 청산
7. 거래소별 PnL 합산으로 스프레드 수렴분만큼 수익 기대

> **참고**: 이 전략은 단방향만 가능합니다. Upbit에서 현물 공매도가 불가능하므로, Bybit이 상대적으로 비쌀 때(양의 스프레드)만 진입할 수 있습니다.

### 확정된 세부 요구사항

| 항목 | 설정 |
|------|------|
| 거래소 | Upbit (현물 KRW) + Bybit (USDT perpetual linear) |
| Bybit 마진 모드 | **Isolated Margin** (포지션 리스크 격리, API로 사전 설정 필수) |
| Bybit 포지션 모드 | **One-Way Mode** (short만 사용하므로 Hedge Mode 불필요) |
| USDT/KRW 소스 | Upbit `KRW-USDT` 마켓 |
| 캔들 기준 | 1분봉, 1440개(1일치), deque 기반 롤링 윈도우 |
| 스프레드 | **상대 스프레드(%)** 사용 — 다중 코인 간 비교 가능 |
| 수수료 | Upbit/Bybit 기본 taker 수수료 (진입 + 청산 양측, **진입 시 notional 기준 근사치**) |
| Z-Score 임계값 | 진입/청산 모두 **파라미터로 설정 가능** |
| 수렴 목표 | **평균(mean) 수렴** — exit_z ≤ 임계값일 때 청산 |
| 손절 | **없음** (수렴할 때까지 보유, 단 Bybit 강제 청산은 반영) |
| Liquidation 체크 | **포함** — Bybit Isolated Margin 기준 강제 청산 가격 계산, 초과 시 강제 청산 처리 |
| 레버리지 | **1배** (레버리지 없음) |
| 펀딩비 | **미반영** (시뮬레이션 범위에서 제외) |
| 대상 코인 | **다중 코인 설정 가능** |
| 포지션 제한 | **코인당 최대 1개** |
| 포지션 매칭 | **USDT notional matching** (양쪽 동일 USDT 금액, 잔여 delta 허용) |
| 포지션 사이징 | **총 자본 대비 비율** (양쪽 거래소 합산 자본 고려) |
| 자본 배분 제한 | **진입 시 가용 자본 확인** (사용 중 자본 + 신규 소요 자본 ≤ total_capital) |
| 백테스트 기간 | **파라미터로 설정 가능** (기본값 7일 = 워밍업 1일 + 테스트 6일) |
| 실시간 워밍업 | **REST API로 초기 1440개 캔들 로드** 후 WebSocket 전환 |
| 실시간 가격 기준 | **매수/매도 방향 반영** (시그널: last trade/bid1, PnL: ask1/bid1 방향별) |
| timestamp 정렬 | **Forward-fill** (누락 캔들은 직전 close 대체, 연속 5분 이상 누락 시 경고) |
| 결과 출력 | **콘솔 로그 + CSV 파일** (출력 디렉토리 설정 가능, 기본 `./output/`) |
| 범위 | 히스토리컬 백테스트 + 실시간 시뮬레이션 모두 구현 |

### 알려진 한계

- **USDT/KRW 환율 리스크**: Upbit 현물은 KRW로 거래하지만 PnL을 USDT로 환산한다. 포지션 보유 중 USDT/KRW 환율이 변동하면, 스프레드 수렴과 무관하게 환산 PnL이 왜곡될 수 있다. 별도 헤지는 하지 않으며, 이 한계를 인지한 상태로 시뮬레이션한다.
- **잔여 delta**: USDT notional matching 방식은 양쪽 leg의 코인 수량이 미세하게 다르다. 스프레드가 0.1~0.5% 수준이면 잔여 delta도 해당 비율 이하로 실무적으로 무시 가능. 예시: `size_usdt=1000, spread=0.3%`에서 `delta ≈ 0.003 코인, PnL 영향 ≈ 0.015%` (무시 가능).
- **스프레드 발산 리스크**: 손절이 없으므로 스프레드가 mean으로 회귀하지 않고 구조적으로 이동하는 경우(regime change — 규제 변경, 입출금 제한, 유동성 고갈 등), 포지션이 무기한 보유되며 손실이 누적된다. Bybit 1배 레버리지라 해도, 코인 가격이 급등하면 short 포지션의 unrealized loss가 마진 잔고를 초과하여 강제 청산(liquidation)될 수 있다. 시뮬레이션에서 liquidation 체크를 포함하여 이 시나리오를 반영한다.
- **수수료 근사치**: 수수료는 진입 시 notional(`size_usdt`)을 기준으로 양쪽 모두 계산한다. 실제로 청산 시 가격이 변동했으므로 청산 시 notional은 다르지만, 스프레드 0.1~0.5% 수준에서 오차는 `0.05% × 0.5% ≈ 0.00025%`로 극히 미미하여 시뮬레이션 단계에서 허용한다. **실전 전환 시에는 정확한 계산으로 교체 필수.**
- **Upbit 수수료 부과 구조**: Upbit 현물 매수 시 수수료는 수령 코인 수량에서 차감되어, 실제 보유 코인이 계획 수량보다 소폭 적어진다. 시뮬레이션에서는 notional 기준 근사치를 사용하며, 실전 전환 시 `actual_qty`와 `order_qty`의 구분이 필요하다.
- **백테스트/실시간 가격 기준 차이**: 백테스트는 캔들 close(= last trade) 가격을 사용하지만, 실시간에서는 방향별 호가(ask1/bid1)를 사용한다. 이로 인해 실시간 성과가 백테스트 대비 하향 편향될 수 있다.
- **Rolling mean 이동**: 포지션 보유 중 새로운 스프레드 데이터가 윈도우에 추가되면서 rolling mean이 변동한다. 진입 시 기대한 수렴 목표(mean)가 보유 기간 동안 이동하여, 실제 수익이 `expected_profit_pct`와 괴리될 수 있다. 이는 모든 rolling window 기반 mean-reversion 전략의 공통적 한계이다.
- **구조적 양의 프리미엄(structural positive basis)**: 단방향 제약(Upbit 공매도 불가)으로 인해 양의 스프레드(Bybit > Upbit)만 포착할 수 있다. 구조적으로 Bybit 프리미엄이 존재하는 환경에서는 mean이 양(+)으로 편향되어, 시뮬레이션 수익이 과대 추정될 수 있음을 인지해야 한다.

### 타입 설계 원칙

| 도메인 | 타입 | 사유 |
|--------|------|------|
| 가격, 수수료, PnL, 포지션 크기 | `Decimal` | 금융 정밀도 보장 |
| Z-Score, 평균, 표준편차, spread_pct(통계 엔진 내부) | `f64` | 통계 연산 성능 |
| Signal enum의 spread_pct 필드 | `f64` | 통계 엔진에서 그대로 전달 |

**변환 경계**: `CandleWindow`에 데이터를 넣을 때 `Decimal → f64`, PnL 계산에 사용할 때 `f64 → Decimal`. 변환 함수는 `common/convert.rs` 모듈에 집중 정의.

**변환 유틸리티 인터페이스:**

```rust
/// Decimal을 f64로 변환. None이면 StrategyError::Statistics 반환.
pub fn decimal_to_f64(d: Decimal) -> Result<f64, StrategyError>;

/// f64를 Decimal로 변환. NaN/Infinity이면 StrategyError::Statistics 반환.
pub fn f64_to_decimal(f: f64) -> Result<Decimal, StrategyError>;
```

---

## 구현 플랜

### Phase 0: 기존 SDK 확장 + 신규 크레이트 생성

#### 0-0. `ExchangeName` canonical 통일

현재 각 크레이트에서 분산 정의되어 있는 `ExchangeName`을 `arb-exchange`의 것으로 canonical 통일한다. 모든 SDK 구현체와 전략 코드에서 `arb_exchange::ExchangeName`을 사용하도록 변경.

#### 0-1. `MarketData` trait 확장 (`arb-exchange`)

기존 `get_candles`는 최신 N개만 조회 가능하여 **페이지네이션 불가**. 타임스탬프 기반 조회 메서드를 추가한다.

```rust
// 기존 (변경 없음)
fn get_candles(&self, market: &str, interval: CandleInterval, count: u32)
    -> impl Future<Output = ExchangeResult<Vec<Candle>>> + Send;

// 신규 추가
// before: exclusive (해당 시점 미포함, 직전까지 반환)
// 반환: timestamp 오름차순 정렬 보장 (구현체에서 정렬하여 반환)
fn get_candles_before(&self, market: &str, interval: CandleInterval, count: u32, before: DateTime<Utc>)
    -> impl Future<Output = ExchangeResult<Vec<Candle>>> + Send;

// 신규 추가: 마켓 코드 생성
// 각 거래소에 맞는 마켓 코드를 생성한다. (e.g., Upbit: "KRW-BTC", Bybit: "BTCUSDT")
fn market_code(base: &str, quote: &str) -> String;
```

> **`before` 파라미터 규약**: exclusive (해당 timestamp 미포함). Upbit API의 `to`는 inclusive이므로 구현 시 `before - 1sec`로 변환. Bybit API의 `end`도 inclusive이므로 `before_ms - 1`로 변환. 이 변환은 각 SDK 구현체 내부에서 처리하여 trait 소비자에게 일관된 exclusive 동작을 보장한다.

**Breaking change 대응** (모든 구현체 수정):
- **Upbit**: `/v1/candles/minutes/1?to={before - 1sec}` + 결과 오름차순 정렬
- **Bybit**: `/v5/market/kline?end={before_ms - 1}` + 결과 오름차순 정렬 (Bybit API는 역순 반환이므로 `.reverse()` 필요)
- **Bithumb**: stub 구현 (`Err(ExchangeError::InternalError("get_candles_before not implemented for Bithumb"))` 반환)
- **`ExchangeAdapter` trait**: `get_candles_before` 메서드 추가 (`arb-exchange`에서 `pub use chrono;` re-export 필요)
  > **참고**: 장기적으로 `src/exchange/`는 `arb-exchange`의 facade 역할만 수행하며, 실질적인 trait 정의와 타입은 `arb-exchange` 크레이트에 집중한다.
- **`impl_exchange_adapter!` 매크로 삭제**: 매크로를 삭제하고 수동 구현으로 단일화. `get_candles_before` 포함 모든 메서드를 직접 구현

> **기존 `get_candles` 반환 순서 통일 (Phase 0 범위)**: Bybit의 `get_candles`도 역순(최신순)으로 반환하는데, 현재 코드에서 정렬하지 않고 있다. **Phase 0에서 모든 거래소 SDK의 `get_candles` 결과를 오름차순(timestamp ascending) 정렬 보장하도록 변경한다.** `get_candles_before`와 동일한 정렬 규약을 적용하여 일관성을 확보.

#### 0-2. `MarketStream` trait 신규 정의 (`arb-exchange`)

WebSocket 실시간 스트림을 위한 trait을 `arb-exchange` 추상화 계층에 추가한다.

**의존성 추가 필요:**
- `arb-exchange`의 `Cargo.toml`에 `tokio = { workspace = true }` 추가 (`[dependencies]`)
- `arb-exchanges`의 `Cargo.toml`에 `tokio = { workspace = true }` 추가 (`[dependencies]`)
- `arb-exchanges`의 `Cargo.toml`에 `tokio-tungstenite`, `futures-util` 추가 (`[dependencies]`)

```rust
/// 실시간 시세 데이터 이벤트
pub enum MarketEvent {
    /// 체결 이벤트 (개별 체결 데이터)
    Trade { market: String, price: Decimal, volume: Decimal, timestamp: DateTime<Utc> },
    /// 호가 업데이트 (best bid/ask)
    BestQuote { market: String, bid: Decimal, ask: Decimal, timestamp: DateTime<Utc> },
}

/// WebSocket 재연결 정책 설정
pub struct StreamConfig {
    /// 초기 백오프 딜레이 (기본값: 1초)
    pub initial_backoff: Duration,
    /// 최대 백오프 딜레이 (기본값: 30초)
    pub max_backoff: Duration,
    /// 최대 재시도 횟수 (기본값: 10, 0이면 무제한)
    pub max_retries: u32,
    /// REST fallback 폴링 간격 (기본값: 5초)
    pub rest_fallback_interval: Duration,
    /// 채널 버퍼 크기 (기본값: 10000)
    pub channel_buffer_size: usize,
}

/// 실시간 마켓 데이터 스트림 trait
#[async_trait]
pub trait MarketStream: Send + Sync {
    /// 거래소 이름 (MarketData::name()과 별도 — 동일 구조체에 두 trait 구현 시 충돌 방지)
    fn stream_name(&self) -> &str;

    /// 지정한 마켓들에 대한 실시간 스트림을 시작한다.
    /// 반환되는 Receiver에서 MarketEvent를 수신한다.
    /// NOTE: 내부적으로 Arc<Mutex<InnerState>>를 사용하여 &self로 상태 관리.
    /// bounded channel을 사용하여 backpressure를 적용한다.
    /// 이미 구독 중인 상태에서 재호출하면 기존 구독을 종료하고 새로운 구독으로 대체한다.
    async fn subscribe(&self, markets: &[&str]) -> ExchangeResult<tokio::sync::mpsc::Receiver<MarketEvent>>;

    /// 모든 구독을 종료한다.
    async fn unsubscribe(&self) -> ExchangeResult<()>;
}
```

**Backpressure 정책**: bounded channel이 가득 차면 **오래된 이벤트를 드롭**하고 최신 이벤트를 유지한다. 구현 시 **송신자 측**에서 `try_send` 실패 시 `receiver` 한 개를 드롭하고 재시도하는 패턴으로 처리한다. 금융 데이터 스트림에서는 **최신 가격이 가장 중요**하므로, 송신자 task가 블록되어 WebSocket 수신이 지연되는 것을 방지한다.

```rust
// Backpressure 구현 패턴
if sender.try_send(event).is_err() {
    let _ = receiver.try_recv(); // 가장 오래된 이벤트 드롭
    sender.try_send(event).ok(); // 재시도
}
```

각 거래소 SDK에서 구현:
- **Upbit**: `wss://api.upbit.com/websocket/v1` → **`trade` 타입 구독** (개별 체결 데이터) → `MarketEvent::Trade`
  - `ticker`가 아닌 `trade` 타입을 사용: 매 체결마다 정확한 price/volume을 수신
  - `trade` 응답의 `trade_price` → `price`, `trade_volume` → `volume`
- **Bybit**: `wss://stream.bybit.com/v5/public/linear` → `orderbook.1` 구독 → `MarketEvent::BestQuote`
  - **Bybit 클라이언트는 linear 전용 인스턴스를 별도 생성** (`BybitClient::with_category("linear")`)
- 자동 재연결 (exponential backoff, `StreamConfig` 파라미터 기반)
- 연속 `max_retries`회 실패 시 REST API fallback으로 전환
- REST fallback 중에도 `rest_fallback_interval` 간격으로 폴링하여 캔들 집계 유지
- 재연결 성공 시 자동으로 WebSocket 모드로 복귀

#### 0-3. `StrategyError` 에러 타입 정의

```rust
/// 통계 연산 에러 (구조화)
#[derive(Error, Debug)]
pub enum StatisticsError {
    #[error("Standard deviation is zero, cannot compute z-score")]
    ZeroDivision,
    #[error("NaN detected in calculation: {0}")]
    NanDetected(String),
    #[error("Insufficient data: need {required}, have {actual}")]
    InsufficientData { required: usize, actual: usize },
    #[error("Value below minimum threshold: {value} < {threshold}")]
    BelowThreshold { value: f64, threshold: f64 },
}

/// 포지션 에러 (구조화)
#[derive(Error, Debug)]
pub enum PositionError {
    #[error("Position already exists for {coin}")]
    AlreadyExists { coin: String },
    #[error("Position not found for {coin}")]
    NotFound { coin: String },
    #[error("Insufficient capital: need {required}, available {available}")]
    InsufficientCapital { required: Decimal, available: Decimal },
    #[error("Position liquidated for {coin} at price {price}")]
    Liquidated { coin: String, price: Decimal },
}

#[derive(Error, Debug)]
pub enum StrategyError {
    #[error("Exchange error: {0}")]
    Exchange(#[from] ExchangeError),
    #[error("Statistics error: {0}")]
    Statistics(#[from] StatisticsError),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Data alignment error: {0}")]
    DataAlignment(String),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Position error: {0}")]
    Position(#[from] PositionError),
}
```

#### 0-4. `arb-strategy` 크레이트 생성

> **아키텍처 원칙**: `arb-strategy`는 `arb-exchange`(추상화 trait)에만 의존한다.
> 구체적인 SDK(`arb-exchanges`)는 `examples/` 또는 `main.rs`에서 DI(Dependency Injection)로 주입한다.

```
crates/arb-strategy/
├── Cargo.toml            # 의존: arb-exchange, arb-config, csv, chrono, rust_decimal, tokio, thiserror, tracing, serde
└── src/
    ├── lib.rs
    ├── error.rs              # StrategyError, StatisticsError, PositionError
    ├── zscore/
    │   ├── mod.rs
    │   ├── config.rs
    │   ├── spread.rs
    │   ├── signal.rs
    │   ├── position.rs
    │   ├── pnl.rs
    │   ├── simulator.rs
    │   └── monitor.rs
    ├── common/
    │   ├── mod.rs
    │   ├── candle_window.rs
    │   ├── statistics.rs
    │   ├── convert.rs        # Decimal ↔ f64 변환 유틸리티
    │   └── fee.rs
    └── output/
        ├── mod.rs
        ├── console.rs
        └── csv.rs
```

**DI 패턴 — 두 개의 generic type parameter:**

```rust
// arb-strategy 내부: trait bound만 사용, 두 거래소가 다른 타입
pub struct BacktestSimulator<U: MarketData, B: MarketData> {
    upbit: U,
    bybit: B,
    config: ZScoreConfig,
}

// 실시간 모니터: MarketData + MarketStream 복합 trait bound
// 참고: 복합 trait bound가 복잡할 경우 factory function으로 감싸는 패턴 고려
//       e.g., fn create_monitor(upbit: impl MarketData + MarketStream, ...) -> ZScoreMonitor<...>
pub struct ZScoreMonitor<U: MarketData + MarketStream, B: MarketData + MarketStream> {
    upbit: U,
    bybit: B,
    config: ZScoreConfig,
}

// examples/zscore_backtest.rs: 구체 타입 DI 주입
let upbit = UpbitClient::with_credentials(&config.upbit.api_key, &config.upbit.secret_key)?;
let bybit = BybitClient::with_credentials(&config.bybit.api_key, &config.bybit.secret_key)?
    .with_category("linear");
let simulator = BacktestSimulator::new(upbit, bybit, zscore_config);
```

### Phase 1: 핵심 데이터 구조 및 통계 엔진

#### 1-1. 전략 설정 (`zscore/config.rs`)

```rust
#[derive(Clone, Debug)]
pub struct ZScoreConfig {
    /// 대상 코인 목록 (e.g., ["BTC", "ETH", "XRP"])
    pub coins: Vec<String>,
    /// 캔들 윈도우 크기 (기본값: 1440 = 1일치 1분봉)
    /// 가이드라인: 최적 윈도우 = 추정 half-life의 3~5배
    /// (예: half-life=300분이면 window_size=900~1500)
    pub window_size: usize,
    /// 캔들 간격 (기본값: 1분)
    pub candle_interval: CandleInterval,
    /// Z-Score 진입 임계값 (파라미터, 기본값: 2.0)
    pub entry_z_threshold: f64,
    /// Z-Score 청산 임계값 (파라미터, 기본값: 0.5)
    pub exit_z_threshold: f64,
    /// 총 자본금 (USDT 기준, 양 거래소 합산)
    pub total_capital_usdt: Decimal,
    /// 포지션당 자본금 비율 (e.g., 0.1 = 총 자본의 10%)
    /// NOTE: 실제 필요 자본 = total_capital × position_ratio × 2
    ///       (Upbit 현물 매수분 + Bybit short 마진분)
    pub position_ratio: Decimal,
    /// Upbit taker 수수료율 (기본값: 0.0005 = 0.05%)
    pub upbit_taker_fee: Decimal,
    /// Bybit linear taker 수수료율 (기본값: 0.00055 = 0.055%)
    pub bybit_taker_fee: Decimal,
    /// Bybit 레버리지 (기본값: 1)
    pub leverage: u32,
    /// Bybit maintenance margin rate (기본값: 0.005 = 0.5%)
    pub bybit_mmr: Decimal,
    /// 백테스트 기간 (분 단위, 워밍업 제외, 기본값: 8640 = 6일)
    pub backtest_period_minutes: usize,
    /// Z-Score 계산 시 최소 stddev 임계값 (기본값: 0.01)
    pub min_stddev_threshold: f64,
    /// 결과 CSV 출력 디렉토리 (기본값: "./output/")
    pub output_dir: PathBuf,
    /// 최대 동시 포지션 수 (None이면 coins.len()까지 허용)
    pub max_concurrent_positions: Option<usize>,
}

impl ZScoreConfig {
    /// 설정값 유효성 검증
    pub fn validate(&self) -> Result<(), StrategyError> {
        // entry_z_threshold > exit_z_threshold 확인
        // window_size > 0 확인
        // position_ratio > 0, <= 0.5 확인 (ratio × 2 > 100% 방지)
        // coins 비어있지 않은지 확인
        // entry_z_threshold > 0, exit_z_threshold >= 0 확인
        // min_stddev_threshold > 0 확인
        // 다중 코인 경고: position_ratio × coins.len() × 2 > 1.0 이면
        //   warn!("position_ratio({}) × 코인 수({}) × 2 = {}. 모든 코인에 동시 진입하면 자본 초과.",
        //          ratio, coins.len(), ratio * coins.len() * 2)
        // 상관관계 경고: 암호화폐 간 상관계수가 0.7~0.9로 높으므로
        //   다중 코인 동시 진입 시 집중 리스크(concentration risk) 경고
        //   warn!("암호화폐 간 상관계수가 높습니다(0.7~0.9). \
        //          다중 코인 동시 진입 시 실질적 분산 효과가 제한적이며, \
        //          집중 리스크에 노출됩니다. max_concurrent_positions 설정을 권장합니다.")
    }
}
```

**포지션 사이징 상세:**

```
single_leg_size = total_capital_usdt × position_ratio
                = 10,000 × 0.1 = 1,000 USDT

USDT notional matching:
  Upbit 측: 1,000 USDT 상당의 코인 현물 매수 (qty = 1000 / upbit_usdt_price)
  Bybit 측: 1,000 USDT short (qty = 1000 / bybit_price)
  → 양쪽 qty가 미세하게 다름 (잔여 delta 허용)

실제 필요 자본: 2,000 USDT (= single_leg_size × 2)
즉, position_ratio = 0.1이면 총 자본의 20%를 실제 사용
```

#### 1-2. 캔들 윈도우 (`common/candle_window.rs`)

- `VecDeque<f64>` 기반, 최대 `window_size`개 유지
- 새 데이터 push 시 오래된 데이터 자동 pop
- **전체 재계산 방식**: 매 업데이트 시 윈도우 전체를 순회하여 mean/stddev 계산 (O(1440), 수 마이크로초 이내)
- Welford 증분 계산의 슬라이딩 윈도우 역방향 제거 시 부동소수점 누적 오차 문제를 회피
- `is_ready()`: 윈도우가 가득 찼는지 (1440개) 확인

#### 1-3. 통계 유틸리티 (`common/statistics.rs`)

- `mean(data: &VecDeque<f64>) -> f64`: 전체 순회 평균
- `stddev(data: &VecDeque<f64>, mean: f64) -> f64`: 전체 순회 **모집단 표준편차 (N으로 나눔)**
  - N=1440에서 N과 N-1의 차이는 0.07%로 무시 가능하며, 일관성을 위해 모집단 표준편차 사용
- `z_score(current: f64, mean: f64, stddev: f64, min_stddev: f64) -> Result<f64, StatisticsError>`:
  - `stddev < min_stddev`이면 `Err(StatisticsError::BelowThreshold)` 반환
  - stddev가 극도로 작으면 Z-Score가 과도하게 증폭되어 의미 없는 진입 신호 방지
- 모든 연산은 `f64` 도메인에서 수행

#### 1-4. 수수료 계산 (`common/fee.rs`)

- 라운드트립 수수료(%) 계산 (진입 + 청산, 양 거래소)
- 손익분기 스프레드(%) 계산

```
총 라운드트립 수수료 = (upbit_fee × 2) + (bybit_fee × 2)
                    = (0.05% × 2) + (0.055% × 2)
                    = 0.21%

∴ 스프레드 변동이 0.21% 이상이어야 수익 발생
```

> **NOTE**: 이 수수료 계산은 진입 시 notional 기준 근사치이다. (알려진 한계 참조)

### Phase 2: 스프레드 계산 엔진

#### 2-1. 스프레드 계산 (`zscore/spread.rs`)

```rust
pub struct SpreadCalculator {
    /// 코인별 Upbit 캔들 윈도우 (coin/KRW close 가격)
    upbit_coin_windows: HashMap<String, CandleWindow>,
    /// USDT/KRW 캔들 윈도우 (Upbit KRW-USDT close 가격)
    usdt_krw_window: CandleWindow,
    /// 코인별 Bybit 캔들 윈도우 (coinUSDT linear close 가격)
    bybit_windows: HashMap<String, CandleWindow>,
    /// 코인별 스프레드(%) 윈도우 — Z-Score 계산의 입력
    spread_pct_windows: HashMap<String, CandleWindow>,
}

impl SpreadCalculator {
    /// 캔들 데이터를 업데이트하고 spread_pct를 재계산한다.
    /// 각 입력은 Option — None이면 forward-fill (직전 값 유지).
    /// Decimal → f64 변환 실패 시 StrategyError 반환.
    pub fn update(
        &mut self,
        timestamp: DateTime<Utc>,
        upbit_coin: Option<Decimal>,
        usdt_krw: Option<Decimal>,
        bybit: Option<Decimal>,
    ) -> Result<(), StrategyError> { ... }
}
```

**3-way 입력 동기화 규약:**

`SpreadCalculator::update()` 메서드는 3개 입력(upbit_coin, usdt_krw, bybit)이 **동일 timestamp에 대해 모두 준비되었을 때만** spread_pct를 계산하고 윈도우에 push한다. 하나라도 누락된 timestamp에서는 **forward-fill**을 적용한다:

```
// 매 분(timestamp t)에 대해:
for each coin:
    upbit_coin_close = candles[upbit_coin][t] ?? candles[upbit_coin][t-1]  // forward-fill
    usdt_krw_close   = candles[usdt_krw][t]   ?? candles[usdt_krw][t-1]    // forward-fill
    bybit_close      = candles[bybit][t]       ?? candles[bybit][t-1]      // forward-fill

    // 연속 누락 감지 (5분 이상이면 경고)
    if consecutive_missing_count >= 5:
        warn!("연속 {}분 캔들 누락: {} at {}", count, source, t)

    // Decimal → f64 변환 (실패 시 StrategyError 전파)
    upbit_usdt_f64 = decimal_to_f64(upbit_coin_close / usdt_krw_close)?
    bybit_f64 = decimal_to_f64(bybit_close)?

    spread_pct = (bybit_f64 - upbit_usdt_f64) / upbit_usdt_f64 × 100.0
    spread_pct_windows[coin].push(spread_pct)
```

**스프레드 계산 로직:**

```
upbit_usdt_price = upbit_coin_krw_close / usdt_krw_close
bybit_short_price = bybit_linear_close  (백테스트) 또는 bid1 (실시간 시그널)

// Z-Score 입력으로 사용되는 상대 스프레드 (%), f64로 계산
spread_pct = (bybit_short_price - upbit_usdt_price) / upbit_usdt_price × 100.0
```

> **주의**: USDT/KRW 환율(`usdt_krw_close`)과 코인/KRW 가격(`upbit_coin_krw_close`)의
> 캔들 close 시점이 정확히 동기화되지 않을 수 있음. 같은 분 캔들이라도 유동성 차이로
> 마지막 체결 시간이 수초 차이날 수 있으나, 1분봉 기준 시뮬레이션에서는 허용 가능한 오차로 간주.

**동시성 패턴 (실시간 모드):** 모든 `MarketEvent`를 단일 tokio task에서 순차 처리하여 동기화 복잡도를 최소화한다. Upbit/Bybit WebSocket 이벤트를 `tokio::select!`로 다중화하여 하나의 이벤트 루프에서 처리.

### Phase 3: 시그널 및 포지션 관리

#### 3-1. 시그널 생성 (`zscore/signal.rs`)

```rust
pub enum Signal {
    /// 진입: Upbit 현물 매수 + Bybit 선물 short
    Enter {
        coin: String,
        z_score: f64,
        spread_pct: f64,
        expected_profit_pct: f64,
    },
    /// 청산: 양쪽 포지션 종료
    Exit {
        coin: String,
        z_score: f64,
        spread_pct: f64,
    },
}
```

> **시그널 반환 타입**: `Option<Signal>` — 진입/청산 조건을 모두 충족하지 못하면 `None` 반환.

**`expected_profit_pct` 산출 공식:**

이 전략은 **스프레드가 평균(mean)으로 수렴**한다고 가정한다. 따라서:

```
// 현재 스프레드와 평균의 차이가 수렴 시 기대되는 수익
expected_spread_change_pct = current_spread_pct - mean_spread_pct

// 수수료 차감
roundtrip_fee_pct = (upbit_taker_fee + bybit_taker_fee) × 2 × 100
                  = (0.0005 + 0.00055) × 2 × 100 = 0.21%

// 기대 수익 (수수료 차감 후, 단일 leg notional 기준 수익률)
// NOTE: 실제 투입 자본 대비 수익률은 이의 약 1/2 (양 leg 합산 자본 대비)
expected_profit_pct = expected_spread_change_pct - roundtrip_fee_pct
```

**실시간 모드 가격 기준 (PnL 기록용):**

시그널 감지는 last trade/bid1 기반 스프레드로 수행하지만, **진입/청산 기록 가격**은 매수/매도 방향을 반영한다:

| 동작 | Upbit (현물) | Bybit (선물) |
|------|-------------|-------------|
| 진입 | ask1 (매수) | bid1 (short 매도) |
| 청산 | bid1 (매도) | ask1 (short 청산 매수) |

> 백테스트에서는 양쪽 모두 캔들 close를 사용하므로 방향 구분이 없다.
> Bybit의 `BestQuote` 이벤트에서 bid/ask 양쪽을 이미 수신하고 있어 추가 구독 불필요.
> Upbit의 `trade` 이벤트는 last trade만 제공하므로, ask1/bid1을 얻으려면 **`orderbook` 타입도 추가 구독**하거나, last trade를 근사치로 사용한다. **시뮬레이션에서는 last trade를 진입/청산가로 사용하되, 이것이 실전 대비 유리한 편향**임을 "알려진 한계"에 이미 기록하였다.

**진입 조건:**
1. `z_score >= entry_z_threshold` (파라미터)
2. `expected_profit_pct > 0` (mean 수렴 가정 시 수수료 차감 후 수익)
3. 해당 코인에 기존 포지션 없음 (코인당 최대 1개)
4. **가용 자본 충분**: `현재 사용 중 자본 + (size_usdt × 2) <= total_capital_usdt`

**청산 조건:**
1. `z_score <= exit_z_threshold` (파라미터, 기본값 0.5 = mean 근처)
2. **또는** Bybit short 포지션이 **liquidation price 도달** (강제 청산)

**손절: 없음** — 스프레드가 수렴할 때까지 보유 (단, Bybit 강제 청산은 반영)

#### 3-2. 가상 포지션 관리 (`zscore/position.rs`)

```rust
pub struct VirtualPosition {
    pub coin: String,
    pub entry_time: DateTime<Utc>,
    /// Upbit 현물 진입가 (USDT 환산)
    pub upbit_entry_price: Decimal,
    /// Bybit short 진입가 (USDT)
    pub bybit_entry_price: Decimal,
    /// Bybit liquidation price (Isolated Margin 기준)
    pub bybit_liquidation_price: Decimal,
    /// 진입 시 USDT/KRW 환율 (사후 분석용)
    pub entry_usdt_krw: Decimal,
    /// 진입 시 스프레드 (%)
    pub entry_spread_pct: f64,
    /// 진입 시 Z-Score
    pub entry_z_score: f64,
    /// 포지션 크기 (USDT, 단일 leg 기준)
    pub size_usdt: Decimal,
}

pub struct PositionManager {
    /// 활성 포지션 (코인별 최대 1개)
    pub open_positions: HashMap<String, VirtualPosition>,
    /// 청산된 포지션 이력
    pub closed_positions: Vec<ClosedPosition>,
}

impl PositionManager {
    /// 현재 사용 중인 자본 합계 (양 leg 합산)
    pub fn used_capital(&self) -> Decimal {
        self.open_positions.values()
            .map(|p| p.size_usdt * Decimal::from(2))
            .sum()
    }

    /// 가용 자본 확인
    pub fn available_capital(&self, total_capital: Decimal) -> Decimal {
        total_capital - self.used_capital()
    }

    /// Bybit liquidation 체크: 현재 가격이 liquidation price 이상이면 강제 청산
    pub fn check_liquidation(&self, coin: &str, current_bybit_price: Decimal) -> bool {
        self.open_positions.get(coin)
            .map(|p| current_bybit_price >= p.bybit_liquidation_price)
            .unwrap_or(false)
    }
}
```

**Bybit Liquidation Price 계산 (short, Isolated Margin):**

```
liq_price = entry_price × (1 + 1/leverage - MMR - bybit_taker_fee)

예시: entry_price = 100,000, leverage = 1, MMR = 0.5%, bybit_taker_fee = 0.055%
liq_price = 100,000 × (1 + 1 - 0.005 - 0.00055) = 100,000 × 1.99445 = 199,445

즉, 코인 가격이 진입가의 약 2배가 되면 강제 청산
(수수료를 반영하면 liquidation price가 소폭 낮아짐)
```

#### 3-3. PnL 계산 (`zscore/pnl.rs`)

```rust
pub struct ClosedPosition {
    pub coin: String,
    pub entry_time: DateTime<Utc>,
    pub exit_time: DateTime<Utc>,
    pub holding_minutes: u64,
    /// 포지션 크기 (USDT, 단일 leg 기준)
    pub size_usdt: Decimal,
    /// Upbit 측 PnL (현물 매수→매도 손익)
    pub upbit_pnl: Decimal,
    /// Bybit 측 PnL (선물 short→청산 손익)
    pub bybit_pnl: Decimal,
    /// Upbit 측 수수료
    pub upbit_fees: Decimal,
    /// Bybit 측 수수료
    pub bybit_fees: Decimal,
    /// 총 수수료 (양 거래소 합산) = upbit_fees + bybit_fees
    pub total_fees: Decimal,
    /// 순 PnL = upbit_pnl + bybit_pnl - total_fees
    pub net_pnl: Decimal,
    pub entry_z_score: f64,
    pub exit_z_score: f64,
    pub entry_spread_pct: f64,
    pub exit_spread_pct: f64,
    /// 진입 시 USDT/KRW 환율
    pub entry_usdt_krw: Decimal,
    /// 청산 시 USDT/KRW 환율
    pub exit_usdt_krw: Decimal,
    /// 강제 청산 여부
    pub is_liquidated: bool,
}
```

**PnL 계산 상세:**

```
// USDT notional matching: 양쪽 동일 USDT 금액으로 진입
upbit_qty = size_usdt / entry_upbit_usdt_price
bybit_qty = size_usdt / entry_bybit_price

// Upbit 현물 PnL
upbit_pnl = (exit_upbit_usdt_price - entry_upbit_usdt_price) × upbit_qty

// Bybit short PnL (강제 청산 시 exit_price = liquidation_price)
// Liquidation 발생 시: Bybit short 강제 청산과 동시에 Upbit 현물도 시장가 매도하여 양쪽 동시 청산 처리
bybit_pnl = (entry_bybit_price - exit_bybit_price) × bybit_qty

// 수수료 (시장가 기준, 진입+청산 양쪽)
// NOTE: 진입 시 notional 기준 근사치. 실전에서는 실제 체결 notional로 계산 필수.
upbit_fees = size_usdt × upbit_taker_fee × 2
bybit_fees = size_usdt × bybit_taker_fee × 2
total_fees = upbit_fees + bybit_fees

// 순수익 (펀딩비 미반영)
net_pnl = upbit_pnl + bybit_pnl - total_fees
```

### Phase 4: 히스토리컬 백테스트

#### 4-1. 데이터 수집

양 거래소 REST API로 1분 캔들을 **페이지네이션(`get_candles_before`)** 하여 수집한다.

- **총 필요 데이터**: 워밍업(1440분) + 백테스트 기간(파라미터, 기본 6일=8640분) = 기본 10,080분
- **API 페이지네이션**: Upbit(200개/요청), Bybit(1000개/요청)
- **API rate limit 준수**: `arb-strategy`의 데이터 수집 함수에서 `tokio::time::sleep`으로 구현
  - Upbit: 요청 간 100ms 딜레이 (초당 10회 제한)
  - Bybit: 요청 간 10ms 딜레이 (초당 120회 제한)
- **데이터 소스**:
  - Upbit: `get_candles_before("KRW-{COIN}", Minute1, 200, before)` 반복
  - Upbit: `get_candles_before("KRW-USDT", Minute1, 200, before)` 반복
  - Bybit: `get_candles_before("{COIN}USDT", Minute1, 1000, before)` 반복

**페이지네이션 merge 알고리즘:**

> `get_candles_before`는 `before` exclusive이고, 결과를 timestamp 오름차순으로 반환한다.

```
fn fetch_all_candles(client, market, interval, total_count, end_time, page_size, delay):
    collected = []
    cursor = end_time
    while collected.len() < total_count:
        sleep(delay)  // rate limit 준수
        batch = client.get_candles_before(market, interval, page_size, cursor)
        if batch.is_empty(): break
        // batch는 이미 오름차순 (trait 규약)

        // 중복 제거 (경계 캔들)
        // 권장: 중복 판정 시 분 단위 truncation 후 비교 (초 단위 차이로 인한 오판 방지)
        if !collected.is_empty():
            batch.retain(|c| c.timestamp < collected.first().timestamp)

        if batch.is_empty(): break  // 더 이상 새 데이터 없음

        // 가장 오래된 캔들의 시간으로 커서 이동 (exclusive이므로 그대로 사용 가능)
        cursor = batch.first().timestamp
        collected = [batch, collected].concat()  // 앞에 prepend
        // 권장: 역순으로 수집(Vec::push)한 후 최종 reverse()로 O(n) 달성

    // 필요 개수 초과분 제거 (앞쪽에서 자름)
    if collected.len() > total_count:
        collected = collected[collected.len() - total_count..]
    collected
```

#### 4-2. 백테스트 시뮬레이터 (`zscore/simulator.rs`)

1. **데이터 수집**: 페이지네이션으로 전체 기간 캔들 수집
2. **시간 정렬**: 3개 시계열(Upbit coin, Upbit USDT, Bybit)을 timestamp 기준으로 align
   - **forward-fill**: 누락 캔들은 직전 close로 대체하여 **push** (윈도우 크기 일관성 유지)
   - 연속 5분 이상 누락 시 `warn!` 로그
   - 모든 시계열이 가장 이른 공통 시작 시점부터 정렬
3. **워밍업**: 첫 `window_size`(1440)개 캔들로 롤링 통계 구성 (이 기간에는 시그널 발생 안 함)
4. **순차 시뮬레이션**: 워밍업 이후 1분씩 진행하며:
   - spread_pct/Z-Score 계산 → 시그널 생성 → 포지션 관리
   - **매 분마다 Bybit liquidation 체크** (현재 Bybit close >= liquidation_price이면 강제 청산)
5. **Regime change 감지**: 시뮬레이션 중 다음 조건에서 경고 로그 출력
   - Rolling mean이 이전 mean 대비 2σ 이상 이동 시: `warn!("Regime change 감지: rolling mean이 2σ 이상 이동")`
   - 포지션 보유 시간이 `window_size × 2`분 초과 시: `warn!("포지션 보유 시간이 window_size의 2배 초과: {}분", holding_minutes)`
6. **결과 집계**: 콘솔 출력 + CSV 파일 내보내기

```rust
pub struct BacktestResult {
    pub config: ZScoreConfig,
    pub test_period_start: DateTime<Utc>,
    pub test_period_end: DateTime<Utc>,
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub liquidated_trades: usize,
    pub win_rate: f64,
    /// 총 gross PnL (수수료 차감 전) = sum(upbit_pnl + bybit_pnl)
    pub total_pnl: Decimal,
    /// 총 수수료
    pub total_fees: Decimal,
    /// 순 PnL (수수료 차감 후) = total_pnl - total_fees
    pub net_pnl: Decimal,
    /// 최대 낙폭 (USDT 절대값, equity curve peak-to-trough)
    pub max_drawdown: Decimal,
    pub avg_holding_minutes: f64,
    pub trades: Vec<ClosedPosition>,
    /// 미청산 포지션 (백테스트 종료 시점에 아직 열려있는 포지션)
    pub open_positions: Vec<VirtualPosition>,
    /// 미청산 포지션의 unrealized PnL (종료 시점 가격 기준 mark-to-market)
    pub unrealized_pnl: Decimal,
    /// 일별 PnL 집계 (시간대별 성과 분석용)
    pub daily_pnl: Vec<(NaiveDate, Decimal)>,
    /// 정상성 메트릭 (예약 필드, Phase 4에서 구현)
    /// ADF p-value, Hurst exponent, OU half-life 추정치 포함
    pub stationarity_metrics: Option<StationarityMetrics>,
    /// 실측 half-life (분 단위, 스프레드 시계열 기반 추정)
    pub estimated_half_life: Option<f64>,
}

/// 스프레드 시계열의 정상성 검증 메트릭 (예약)
pub struct StationarityMetrics {
    /// ADF(Augmented Dickey-Fuller) 검정 p-value (< 0.05이면 정상성)
    pub adf_p_value: Option<f64>,
    /// Hurst exponent (< 0.5이면 mean-reverting)
    pub hurst_exponent: Option<f64>,
    /// OU(Ornstein-Uhlenbeck) 프로세스 half-life 추정치 (분 단위)
    pub ou_half_life: Option<f64>,
}
```

**Max Drawdown 계산:**

```
// 거래 건별 누적 PnL 기반 equity curve
equity[0] = 0
equity[i] = equity[i-1] + trades[i].net_pnl  // realized PnL만 사용

// peak-to-trough
peak = max(equity[0..=i])
drawdown[i] = peak - equity[i]
max_drawdown = max(drawdown[0..=n])  // USDT 절대값
```

**통계적 유의성 경고:**

```rust
if result.total_trades < 30 {
    warn!(
        "거래 횟수 {}회: 통계적 유의성이 부족합니다. \
         최소 30회 이상의 거래가 필요합니다.",
        result.total_trades
    );
}
```

### Phase 5a: 실시간 시뮬레이션 — 기본 WebSocket 연결 + 캔들 구성 + 시그널

#### 5a-1. WebSocket 스트림 구현

`MarketStream` trait 구현을 `arb-exchanges` 크레이트의 각 거래소 모듈에 추가한다.

**Upbit MarketStream**:
- `wss://api.upbit.com/websocket/v1`
- 구독: **`trade` 타입**으로 `KRW-{COIN}`, `KRW-USDT` 구독 (개별 체결 데이터)
- `trade` 응답: `trade_price` → `MarketEvent::Trade.price`, `trade_volume` → `volume`
- bounded channel + backpressure (오래된 이벤트 드롭)

**Bybit MarketStream**:
- `wss://stream.bybit.com/v5/public/linear`
- 구독: `orderbook.1.{COIN}USDT` (best bid/ask 호가)
- 수신 데이터 → `MarketEvent::BestQuote` 변환
- bounded channel + backpressure (오래된 이벤트 드롭)

#### 5a-2. 실시간 모니터링 (`zscore/monitor.rs`)

1. **초기화**: REST API(`get_candles_before`)로 1440개 1분 캔들 사전 로드 (워밍업)
2. **WebSocket 연결**: `MarketStream::subscribe()` 로 양 거래소 동시 구독
3. **캔들 구성**: `MarketEvent`를 1분 캔들로 집계
   - **분 경계 판정**: UTC 기준, 이벤트 timestamp를 `floor` truncation하여 분 단위 grouping (`timestamp.trunc_subsecs(0)` 후 분 단위)
   - **Upbit 캔들 close**: 해당 분의 마지막 `Trade` 이벤트의 `price` (last trade)
   - **Bybit 캔들 close**: 해당 분의 마지막 `BestQuote` 이벤트의 `bid` (best bid)
   - **빈 분 처리**: 1분 동안 이벤트가 없으면 직전 분의 close를 값으로 하는 **새 데이터 포인트를 push** (백테스트 forward-fill과 동일하게 윈도우 크기 일관성 유지)
4. **이벤트 루프**: **단일 tokio task**에서 `tokio::select!`로 양쪽 WebSocket 이벤트를 순차 처리 (동기화 복잡도 최소화)
   - **빈 분 감지 타이머**: `tokio::time::interval(Duration::from_secs(60))` 타이머를 `select!`의 **3번째 arm**으로 추가하여, 60초 동안 이벤트가 없는 분을 감지하고 forward-fill을 트리거한다.
   ```rust
   loop {
       tokio::select! {
           Some(upbit_event) = upbit_rx.recv() => { /* Upbit 이벤트 처리 */ }
           Some(bybit_event) = bybit_rx.recv() => { /* Bybit 이벤트 처리 */ }
           _ = minute_timer.tick() => { /* 빈 분 감지 및 forward-fill */ }
       }
   }
   ```
5. **deque 업데이트**: 새 캔들 완성 시 롤링 윈도우에 push
6. **시그널 감지**: 매 캔들 완성 시 Z-Score 재계산 → 시그널 확인
7. **Liquidation 체크**: 매 캔들 완성 시 Bybit 가격이 liquidation price 이상인지 확인
8. **알림**: 시그널 발생 시 콘솔 로그 출력
9. **CSV 기록**: 거래 내역 실시간 CSV 기록

### Phase 5b: 실시간 시뮬레이션 — 재연결 정책 + REST fallback + task 종료

#### 5b-1. 재연결 정책

- **Upbit/Bybit 공통**: `StreamConfig` 기반 exponential backoff 재연결
- 연속 `max_retries`회 실패 시 REST API fallback으로 전환
- REST fallback 중에도 `rest_fallback_interval` 간격으로 폴링하여 캔들 집계 유지
- 재연결 성공 시 자동으로 WebSocket 모드로 복귀

#### 5b-2. `CancellationToken` 기반 task 종료

- `tokio_util::sync::CancellationToken`을 사용하여 graceful shutdown 구현
- 메인 이벤트 루프, WebSocket 송수신 task, REST fallback task 모두 `CancellationToken`을 공유
- `Ctrl+C` 시그널 또는 외부 종료 요청 시 token을 cancel하여 모든 task가 정리(cleanup) 후 종료

> **Leg risk 대응**: 시뮬레이션 범위 제외. 실전 전환 시 별도 스펙으로 상세화한다.
> (양쪽 거래소 동시 주문 시 한쪽 실패에 대한 타임아웃/재시도/반대쪽 청산 로직은 실전 스펙에서 다룸)

### Phase 6: 결과 출력

#### 6-1. 콘솔 출력 (`output/console.rs`)

- `tracing` 기반 구조화된 로그
- 시그널 발생 시 진입/청산 정보 출력 (liquidation 포함)
- 백테스트 완료 시 요약 통계 출력 (unrealized PnL, 일별 PnL, max_drawdown 포함)
- 통계적 유의성 경고 (거래 횟수 < 30)

#### 6-2. CSV 출력 (`output/csv.rs`)

CSV 파일은 `ZScoreConfig.output_dir` (기본 `./output/`)에 저장. 디렉토리 미존재 시 자동 생성.
파일명의 `{timestamp}` 형식: `YYYYMMDD_HHmmss` (UTC 기준).
실시간 모드에서 장기 실행 시 **일별 파일 분할**을 고려 (향후 개선사항).

**거래 내역 CSV** (`trades_{timestamp}.csv`):
```csv
coin,entry_time,exit_time,holding_min,size_usdt,entry_z,exit_z,entry_spread_pct,exit_spread_pct,upbit_pnl,bybit_pnl,upbit_fees,bybit_fees,net_pnl,entry_usdt_krw,exit_usdt_krw,is_liquidated
BTC,2026-02-06T10:00:00Z,2026-02-06T10:30:00Z,30,1000,2.15,0.45,0.32,0.10,1.20,-0.50,0.10,0.11,0.49,1380.50,1381.20,false
```

**시계열 CSV** (`timeseries_{timestamp}.csv`):
```csv
timestamp,coin,upbit_usdt_price,bybit_price,spread_pct,mean_spread_pct,stddev,z_score,signal,position
2026-02-06T10:00:00Z,BTC,99500.00,99800.00,0.30,0.15,0.08,1.85,NONE,NONE
```

### Phase 7: 설정 통합 및 예제

#### 7-1. 설정 파일 확장 (`config.toml`)

> **NOTE**: 현재 `arb-config`의 `parse_toml_simple`은 중첩 섹션을 지원하지 않는다.
> `toml` 크레이트를 도입하여 본격적인 TOML 파싱을 지원하거나,
> 전략 설정을 별도 파일(`strategy.toml`)로 분리하여 `arb-strategy`에서 자체 로딩하는 방안을 선택.

```toml
[strategy.zscore]
coins = ["BTC", "ETH", "XRP"]
window_size = 1440
entry_z_threshold = 2.0
exit_z_threshold = 0.5
total_capital_usdt = 10000.0
position_ratio = 0.1
leverage = 1
bybit_mmr = 0.005
backtest_period_minutes = 8640
min_stddev_threshold = 0.01
output_dir = "./output/"
```

#### 7-2. 실행 예제

```bash
# 히스토리컬 백테스트
cargo run --example zscore_backtest

# 실시간 모니터링
cargo run --example zscore_monitor
```

---

## 리뷰 반영 이력

### Trader 리뷰 #1 반영

| 피드백 | 반영 내용 |
|--------|----------|
| 스프레드 입력 모호함 | **상대 스프레드(%)** 사용으로 확정 |
| 백테스트 1일은 통계적 무의미 | **파라미터화** (기본 7일) |
| 포지션 사이징 모호함 | 양 거래소 합산 자본(`single_leg × 2`) 명시 |
| 단방향 제약 미문서화 | Upbit 공매도 불가 제약 명시 |
| USDT/KRW 시간 동기화 | 오차 인지 주석 추가 |
| API rate limit 미고려 | 페이지네이션 시 rate limit 준수 명시 |

### Trader 리뷰 #2 + Coder 리뷰 반영

| 피드백 | 반영 내용 |
|--------|----------|
| `expected_profit_after_fees` 공식 미정의 | `expected_profit_pct = (current - mean) - roundtrip_fee_pct` 명시 |
| `position_qty` 매칭 방식 미명시 | **USDT notional matching** 확정, 잔여 delta 허용 문서화 |
| USDT/KRW 환율 리스크 미문서화 | "알려진 한계" 섹션 추가 |
| Welford 슬라이딩 윈도우 수치 불안정 | **전체 재계산 방식(O(1440))** 으로 변경 |
| 미청산 포지션 unrealized PnL | `BacktestResult`에 `unrealized_pnl` 필드 추가 |
| 수렴 목표 미명시 | **mean reversion** 명시 |
| `get_candles` 페이지네이션 불가 | `get_candles_before` 메서드 trait 추가 |
| WebSocket이 trait에 없음 | `MarketStream` trait 신규 정의 |
| `StrategyError` 타입 누락 | 에러 타입 설계 추가 |
| f64/Decimal 변환 경계 미명시 | "타입 설계 원칙" 섹션 추가 |
| `csv` 크레이트 의존성 | `arb-strategy` Cargo.toml에 명시 |
| 펀딩비 처리 | **시뮬레이션 범위에서 제외** |

### Trader 리뷰 #3 + Coder 리뷰 #2 반영

| 피드백 | 반영 내용 |
|--------|----------|
| 수수료 근사치 | "알려진 한계"에 명시, 실전 전환 시 교체 필수 |
| Upbit 수수료 코인 차감 구조 | "알려진 한계"에 추가 |
| 스프레드 발산 시 강제 청산 리스크 | "알려진 한계"에 상세 문서화 |
| Bybit 마진/포지션 모드 미명시 | Isolated Margin + One-Way Mode 명시 |
| 다중 코인 자본 초과 | 가용 자본 확인 로직 추가 |
| min_stddev guard | `min_stddev_threshold` + `z_score()` guard |
| 모집단/표본 표준편차 | **모집단 표준편차 (N)** 명시 |
| 가격 기준 편향 | "알려진 한계"에 문서화 |
| timestamp align 정책 | **forward-fill** 확정 |
| 3-way 동기화 | `update()` 호출 규약 명시 |
| 캔들 구성 규칙 | 분 경계(UTC), 빈 분 처리 명시 |
| breaking change | Bithumb stub + ExchangeAdapter/매크로 수정 |
| `name()` 충돌 | `stream_name()`으로 변경 |
| unbounded channel | bounded + `StreamConfig.channel_buffer_size` |
| `arb-exchanges` 직접 의존 | **DI 패턴** |
| 에러 구조화 | `StatisticsError`/`PositionError` 서브 enum |
| 변환 유틸리티 | `common/convert.rs` 인터페이스 정의 |
| rate limiter 위치 | `arb-strategy`에서 `tokio::time::sleep` |
| 페이지네이션 merge | pseudo-code 추가 |
| 재연결 정책 | `StreamConfig` 구조체 |
| 동시성 패턴 | 단일 tokio task + `tokio::select!` |
| `expected_profit_pct` 단위 | "단일 leg notional 기준" 주석 |
| `Signal::Hold` | `Option<Signal>` 반환 방식 |
| USDT/KRW 환율 필드 | `entry_usdt_krw`, `exit_usdt_krw` 추가 |
| `ZScoreConfig::validate()` | 검증 메서드 정의 |
| CSV 출력 경로 | `output_dir` 필드 |
| TOML 파서 | `toml` 크레이트 도입 방안 명시 |
| `BacktestResult` 분석 | `daily_pnl` 필드 |
| 통계적 유의성 | 거래 < 30 시 warn |
| `with_category` 패턴 | linear 전용 인스턴스 별도 생성 |

### Trader 리뷰 #4 + Coder 리뷰 #3 반영

| 피드백 | 반영 내용 |
|--------|----------|
| `BacktestSimulator` 단일 generic | **두 개의 generic `<U, B>`** 로 변경 |
| `max_drawdown` 미정의 | equity curve peak-to-trough, USDT 절대값 공식 명시 |
| `get_candles_before` 반환 순서 미정의 | **오름차순 정렬 보장** trait 규약 + Bybit `.reverse()` 명시 |
| `before` 파라미터 inclusive/exclusive | **exclusive** 로 확정, SDK 구현에서 변환 |
| 페이지네이션 cursor 무한루프 | exclusive 규약으로 해결, cursor 로직 수정 |
| 실시간 진입/청산 가격 기준 미명시 | **매수/매도 방향 반영** (ask1/bid1) + 시뮬레이션에서는 last trade 근사치 |
| Bybit liquidation 체크 미포함 | **포함** — `bybit_liquidation_price` 필드 + `check_liquidation()` + 청산 조건 추가 |
| `ClosedPosition`에 `size_usdt` 누락 | `size_usdt` 필드 추가 |
| `ClosedPosition`에 거래소별 수수료 부재 | `upbit_fees`, `bybit_fees` 개별 필드 추가 |
| `total_pnl` vs `net_pnl` 관계 모호 | `total_pnl` = gross(수수료 전), `net_pnl` = total_pnl - total_fees 주석 명시 |
| `BacktestResult`에 `liquidated_trades` 누락 | 필드 추가 |
| 실시간 빈 분 처리 불일치 | forward-fill과 동일하게 **push** 방식으로 통일 |
| Rolling mean 이동 미문서화 | "알려진 한계"에 추가 |
| `position_ratio` × coins 경고 | `validate()`에 경고 로직 추가 |
| Upbit `ticker` vs `trade` 구독 | **`trade` 타입** 구독으로 변경 |
| Bounded channel backpressure 미정의 | **오래된 이벤트 드롭** 정책 명시 |
| CSV 시계열에 mean/stddev 누락 | `mean_spread_pct`, `stddev` 컬럼 추가 |
| `MarketData`(RPITIT) vs `MarketStream`(async_trait) | `ZScoreMonitor<U: MarketData+MarketStream, B: ...>` 시그니처 명시 |
| `arb-exchange`에서 `chrono` re-export 필요 | `pub use chrono;` 추가 명시 |
| `subscribe` 재호출 동작 미정의 | "기존 구독 종료 후 새 구독으로 대체" 명시 |
| `SpreadCalculator::update()` 반환 타입 | `Result<(), StrategyError>` 명시 |
| 분 경계 truncation 방식 | `floor` truncation 명시 |
| `arb-strategy`에 `tokio` 의존성 누락 | Cargo.toml에 `tokio` 추가 |
| DI 예시 코드 API 불일치 | 실제 `with_credentials` API로 수정 |
| `ZScoreConfig` Clone derive | `#[derive(Clone, Debug)]` 명시 |
| `PositionError::Liquidated` 변형 | 에러 타입에 추가 |
| `ClosedPosition.is_liquidated` | 강제 청산 여부 필드 추가 |
| CSV rotation | 실시간 모드 일별 파일 분할 향후 개선사항 |

### Trader 리뷰 #5 + Coder 리뷰 #4 반영

| 피드백 | 반영 내용 |
|--------|----------|
| C-3: Leg risk 시뮬레이션 범위 제외 | Phase 5에서 제거, "의도적 미반영"에 추가. 실전 전환 시 별도 스펙 |
| C-4: ExchangeName 통합 | Phase 0-0 섹션 신설, `arb-exchange`의 ExchangeName으로 canonical 통일 |
| C-5: chrono re-export | Phase 0-1에 이미 명시 확인. `pub use chrono;` re-export |
| C-6: tokio 의존성 | Phase 0-2에 `arb-exchange`, `arb-exchanges` Cargo.toml 의존성 추가 명시 |
| C-7: Phase 5 분리 | Phase 5a(기본 WebSocket + 캔들 + 시그널) / Phase 5b(재연결 + REST fallback + CancellationToken) |
| C-8: Backpressure 주체 수정 | 송신자 측 `try_send` 실패 → `try_recv` 드롭 → 재시도 패턴으로 수정 |
| M-1: 정상성 메트릭 | `BacktestResult`에 `stationarity_metrics`, `StationarityMetrics` 구조체 예약 |
| M-2: 윈도우 half-life 가이드 | `window_size` 문서에 "최적 윈도우 = half-life × 3~5" 가이드라인 추가 |
| M-3: Regime change 감지 | Phase 4 시뮬레이터에 rolling mean 2σ 이동 / 보유 시간 window_size×2 초과 경고 |
| M-4: 상관관계 경고 | `max_concurrent_positions` 필드 + `validate()`에 집중 리스크 경고 |
| M-7: Liquidation 수수료 반영 | 공식에 `bybit_taker_fee` 차감 반영, 예시 업데이트 |
| M-10: 마켓 코드 생성 | `MarketData` trait에 `market_code(base, quote)` 메서드 추가 |
| M-11: 매크로 삭제 | `impl_exchange_adapter!` 매크로 삭제, 수동 구현 단일화 |
| M-12: get_candles 정렬 통일 | "향후 개선사항"에서 Phase 0으로 격상 |
| M-13: update() 시그니처 | `SpreadCalculator::update()` 인자 상세 정의 (timestamp, 3개 Option) |
| M-14: 빈 분 감지 타이밍 | `tokio::time::interval(60s)` 타이머를 `select!` 3번째 arm으로 추가 |
| m-1: 구조적 양의 프리미엄 | "알려진 한계"에 추가 |
| m-2: skewness/kurtosis | Future Work에 추가 |
| m-3: Liquidation 동시 청산 | PnL 계산에 양쪽 동시 청산 처리 명시 |
| m-4: ROI 지표 | Future Work에 추가 |
| m-5: half_spread_bps | Future Work에 추가 |
| m-6: 잔여 delta 수치 예시 | 수치 예시 추가 |
| m-7: subscribe 반환 타입 | `tokio::sync::mpsc::Receiver<MarketEvent>` 명시 |
| m-8: Arc<str> key | Future Work에 추가 |
| m-9: 역순 수집 후 reverse | 페이지네이션 pseudo-code에 권장 주석 추가 |
| m-10: arb-strategy 의존성 | Cargo.toml에 `thiserror`, `tracing`, `serde` 추가 |
| m-11: src/exchange facade | 참고 주석 추가 |
| m-12: 분 단위 truncation | 중복 캔들 판정 시 권장 주석 추가 |
| m-13: get_candles_before default impl | Phase 0 체크리스트에 참고 주석 추가 |
| m-14: factory function 패턴 | ZScoreMonitor 참고 주석 추가 |

### 의도적으로 미반영 (사용자 결정)

| 피드백 | 사유 |
|--------|------|
| Safety net (손절) | 손절 없음 유지 (단, liquidation은 반영) |
| 슬리피지 모델 | 추가하지 않음 |
| 펀딩비 반영 | 시뮬레이션 범위에서 제외 |
| C-1: PnL 환율 분리 | 현재 방식 유지 (알려진 한계로 남겨둠) |
| C-2: 슬리피지 민감도 분석 | 미반영 |
| C-3: Leg risk 대응 | 시뮬레이션 범위 제외, 실전 전환 시 별도 스펙으로 상세화 |
| M-5: 최대 보유 시간 제한 | 미반영 |
| M-6: 백테스트 기간 상향 | 현재 6일 유지 |
| M-8: 거래소 API 다운타임 대응 | 미반영 |
| M-9: ExchangeAdapter vs MarketData 이중 경로 | 양쪽 모두 추가 유지 |

### 향후 개선사항 (Future Work)

- [ ] 펀딩비 반영 (Bybit funding rate API 연동)
- [ ] 스프레드 정상성 검증 (ADF 테스트, Hurst exponent, OU half-life 추정)
- [ ] 변동성 조정 포지션 사이징 (volatility-adjusted sizing)
- [ ] 파라미터 최적화 (grid search + walk-forward validation)
- [ ] 멀티 타임프레임 윈도우 (720분, 2880분 등) — half-life와 윈도우 크기 정합성 검증
- [ ] Sharpe ratio, Calmar ratio 등 risk-adjusted return 지표
- [ ] 양 거래소 간 자금 재분배(rebalancing) 로직
- [ ] 슬리피지 모델
- [ ] Safety net (손절 옵션, 시간 기반 stop 포함)
- [ ] 정확한 수수료 계산 (청산 시 실제 notional 기준)
- [ ] Upbit 수수료 코인 차감 반영 (`actual_qty` vs `order_qty` 구분)
- [ ] 환율 영향 분석 (`fx_impact` 필드로 환산 PnL과 전략 PnL 분리)
- [ ] `MarketStream` 부분 해제 (특정 마켓만 unsubscribe)
- [ ] 실시간 CSV 일별 파일 분할 (rotation)
- [ ] Upbit `orderbook` 구독으로 실시간 ask1/bid1 정확한 진입/청산가 사용
- [ ] Max drawdown에 unrealized PnL(mark-to-market) 포함 옵션
- [ ] `BacktestResult`에 스프레드 분포의 skewness/kurtosis 통계 추가
- [ ] `BacktestResult`에 ROI 지표 추가 (양 leg 합산 투입 자본 대비 수익률)
- [ ] `simulated_half_spread_bps` 옵션 추가 (백테스트에서 bid/ask 스프레드 시뮬레이션)
- [ ] 코인 10+개 지원 시 `Arc<str>` key 또는 enum-based key 고려 (HashMap 성능 최적화)

---

## 체크리스트

### Phase 0: 기존 SDK 확장 + 프로젝트 셋업
- [ ] `ExchangeName`을 `arb-exchange`의 것으로 canonical 통일
- [ ] `MarketData` trait에 `get_candles_before` 메서드 추가 (before=exclusive, 오름차순 반환 규약)
- [ ] `MarketData` trait에 `market_code(base, quote)` 메서드 추가
- [ ] `ExchangeAdapter` trait에 `get_candles_before` 메서드 추가 (`arb-exchange`에서 `pub use chrono;` re-export)
- [ ] `impl_exchange_adapter!` 매크로 삭제, 수동 구현 단일화
- [ ] Upbit SDK에 `get_candles_before` 구현 (`to` 파라미터, before-1sec 변환, 오름차순 정렬)
- [ ] Bybit SDK에 `get_candles_before` 구현 (`end` 파라미터, before_ms-1 변환, `.reverse()` 오름차순)
- [ ] Bithumb SDK에 `get_candles_before` stub 구현 (InternalError 반환)
  <!-- 참고: get_candles_before에 default impl 제공 고려 (unimplemented! 또는 런타임 에러) -->
- [ ] 모든 거래소 SDK의 기존 `get_candles` 반환 순서를 **오름차순(timestamp ascending)으로 통일**
- [ ] `arb-exchange`의 Cargo.toml에 `tokio = { workspace = true }` 추가 (`[dependencies]`)
- [ ] `arb-exchanges`의 Cargo.toml에 `tokio = { workspace = true }`, `tokio-tungstenite`, `futures-util` 추가
- [ ] `MarketStream` trait 정의 (`MarketEvent`, `StreamConfig`, `subscribe`, `unsubscribe`)
- [ ] `MarketStream`의 `stream_name()` 메서드
- [ ] `MarketStream`의 bounded channel + backpressure (송신자 측 `try_send` → `try_recv` 드롭 → 재시도)
- [ ] `subscribe` 재호출 시 기존 구독 종료 후 대체 동작 구현
- [ ] `arb-exchange`에 WebSocket 관련 타입/에러 추가
- [ ] `crates/arb-strategy` 크레이트 생성 및 Cargo.toml 작성 (arb-exchange, csv, chrono, rust_decimal, tokio, thiserror, tracing, serde)
- [ ] `arb-strategy`는 `arb-exchanges`에 의존하지 않음 (DI 패턴)
- [ ] 워크스페이스 Cargo.toml에 `arb-strategy` 멤버 추가, `csv` workspace 의존성 추가
- [ ] `src/lib.rs`에 `pub use arb_strategy as strategy;` re-export 추가
- [ ] `StrategyError`, `StatisticsError`, `PositionError` (Liquidated 포함) 에러 타입 정의
- [ ] 모듈 디렉토리 구조 생성 (`zscore/`, `common/`, `output/`)
- [ ] `common/convert.rs` — Decimal ↔ f64 변환 유틸리티 정의

### Phase 1: 핵심 데이터 구조 및 통계 엔진
- [ ] `ZScoreConfig` 구현 (`#[derive(Clone, Debug)]`, `bybit_mmr`, `min_stddev_threshold`, `output_dir`, `max_concurrent_positions`)
- [ ] `ZScoreConfig::validate()` 구현 (position_ratio × coins 경고, 암호화폐 상관관계 집중 리스크 경고 포함)
- [ ] `CandleWindow` (VecDeque + 전체 재계산 방식) 구현
- [ ] 통계 유틸리티 구현 (mean, stddev(모집단, N), z_score(min_stddev guard))
- [ ] 수수료 계산 모듈 구현 (라운드트립, 손익분기 spread_pct)
- [ ] Decimal ↔ f64 변환 유틸리티 구현 (`common/convert.rs`)
- [ ] Phase 1 단위 테스트 작성

### Phase 2: 스프레드 계산 엔진
- [ ] `SpreadCalculator` 구현 (`update() -> Result`, 3-way 동기화, Decimal→f64 변환 에러 처리)
- [ ] Forward-fill 정책 구현 (누락 캔들 직전 close 대체 push, 연속 5분 경고)
- [ ] Upbit `KRW-USDT` 마켓 기반 합성 가격 계산 구현
- [ ] 상대 스프레드(%) 계산 및 코인별 `spread_pct` 윈도우 관리 구현
- [ ] Phase 2 단위 테스트 작성

### Phase 3: 시그널 및 포지션 관리
- [ ] `Option<Signal>` 반환 방식 시그널 생성 로직 구현
- [ ] `VirtualPosition` 구현 (`bybit_liquidation_price` 포함)
- [ ] `PositionManager` 구현 (가용 자본 확인, `check_liquidation()`)
- [ ] Bybit liquidation price 계산 로직 구현 (수수료 반영: `1 + 1/leverage - MMR - bybit_taker_fee`)
- [ ] 진입 조건 구현 (Z-Score + profit + 포지션 + 가용 자본)
- [ ] 청산 로직 구현 (Z-Score 수렴 OR liquidation)
- [ ] PnL 계산 구현 (`size_usdt`, `upbit_fees`, `bybit_fees` 개별 기록, `is_liquidated`)
- [ ] Phase 3 단위 테스트 작성

### Phase 4: 히스토리컬 백테스트
- [ ] 페이지네이션 merge 알고리즘 구현 (오름차순 보장, exclusive cursor)
- [ ] rate limit 준수 딜레이 구현 (Upbit 100ms, Bybit 10ms)
- [ ] 캔들 3-way timestamp forward-fill 정렬 구현 (push 방식)
- [ ] 워밍업 기간 분리 → 테스트 기간만 시뮬레이션
- [ ] `BacktestSimulator<U, B>` 순차 시뮬레이션 엔진 구현 (매 분 liquidation 체크)
- [ ] `BacktestResult` 집계 구현 (max_drawdown equity curve, daily_pnl, liquidated_trades)
- [ ] `BacktestResult`에 `stationarity_metrics` (예약), `estimated_half_life` 필드 포함
- [ ] `total_pnl` = gross, `net_pnl` = total_pnl - total_fees 관계 구현
- [ ] Regime change 감지 경고 구현 (rolling mean 2σ 이동, 보유 시간 window_size×2 초과)
- [ ] 통계적 유의성 경고 (거래 < 30회)
- [ ] 콘솔 요약 출력 구현
- [ ] CSV 파일 출력 구현 (시계열에 mean/stddev 포함, output_dir 자동 생성)
- [ ] `examples/zscore_backtest.rs` 예제 작성 (실제 API 기반 DI)
- [ ] 실제 데이터로 백테스트 실행 및 결과 검증

### Phase 5a: 실시간 시뮬레이션 — 기본 WebSocket + 캔들 + 시그널
- [ ] Upbit `MarketStream` 구현 (`trade` 타입 구독, backpressure)
- [ ] Bybit `MarketStream` 구현 (`orderbook.1` 구독, backpressure)
- [ ] REST API로 초기 1440개 캔들 워밍업 로드 구현
- [ ] 실시간 캔들 구성 (UTC floor truncation, Upbit last trade / Bybit bid1, 빈 분 push)
- [ ] 단일 tokio task + `tokio::select!` 이벤트 루프 구현 (3번째 arm: `interval(60s)` 빈 분 감지 타이머)
- [ ] `ZScoreMonitor<U: MarketData+MarketStream, B: ...>` 구현
- [ ] 실시간 liquidation 체크 구현
- [ ] 실시간 거래 CSV 기록 구현
- [ ] `examples/zscore_monitor.rs` 예제 작성

### Phase 5b: 실시간 시뮬레이션 — 재연결 + REST fallback + task 종료
- [ ] `StreamConfig` 기반 exponential backoff 재연결 정책 구현 (Upbit/Bybit)
- [ ] 연속 실패 시 REST API fallback 전환 구현
- [ ] REST fallback 중 폴링으로 캔들 집계 유지
- [ ] 재연결 성공 시 WebSocket 모드 복귀
- [ ] `CancellationToken` 기반 graceful shutdown 구현

### Phase 6: 설정 및 통합
- [ ] `toml` 크레이트 도입 또는 전략 설정 별도 파일 분리
- [ ] `config.example.toml`에 전략 설정 템플릿 추가 (`bybit_mmr` 포함)
- [ ] 다중 코인 설정 검증 로직 구현
- [ ] 전체 통합 테스트 작성
- [ ] 코드 리뷰 (reviewer 에이전트)
