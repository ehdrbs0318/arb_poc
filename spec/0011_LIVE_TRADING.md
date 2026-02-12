# 0011_LIVE_TRADING

## 사용자의 요청

시뮬레이션으로 검증된 Z-Score 차익거래 전략을 **실제 거래소에서 실행**할 수 있도록 전환한다.
현재 가상 체결(VirtualPosition) 기반의 이벤트 루프를 실 주문 API 호출 기반으로 변경하고,
실거래에 필수적인 안전장치(kill switch, 포지션 영속화, 레그 리스크 관리 등)를 구축한다.

### 배경

- spec/0000~0010에서 시뮬레이션 전략을 완성: 9단계 진입 검증, 오더북 기반 포지션 사이징, tick/lot 라운딩, 비동기 이벤트 루프
- 12.9시간 시뮬레이션 결과: 142건 전승, +373.81 USDT, 시간당 +28.98 USDT
- `OrderManagement` trait + Upbit/Bybit SDK에 `place_order`, `cancel_order`, `get_order` 이미 구현
- `arb-telegram` 크레이트 메시지 전송 인프라 준비 완료

### 현재 흐름 (시뮬레이션)

```
tick event → signal 평가 → 9단계 진입 검증 통과
  → VirtualPosition 생성 (즉시, 메모리 only)
  → PnL 계산 (가상 체결가 기반)
```

### 목표 흐름 (실거래)

```
tick event → signal 평가 → 9단계 진입 검증 통과
  → PositionStore에 "Opening" 상태 선기록 (WAL)
  → OrderExecutor.execute_entry()
    ├── Upbit: place_order(Limit IOC, Buy, krw_amount) ──┐ (총액 기반)
    ├── Bybit: place_order(Market, Sell, "linear")  ─────┤
    │                                                     ▼
    ├── poll_until_filled(timeout=5s, backoff 200ms~2s)
    ├── 양쪽 Filled → effective_qty = min(upbit_qty, bybit_qty)
    │   → 초과분 즉시 청산 → "Open" 상태 전이 + persist + 텔레그램 알림
    ├── 한쪽만 Filled → emergency_close(체결된 쪽)
    │   → 실패 시 무한 재시도(지수 백오프, max 30s) → 5분 초과 시 kill switch
    └── 양쪽 미체결 → 양쪽 cancel → "Opening" 기록 삭제
```

---

## 설계

### 핵심 원칙

1. **실행 모드 enum**: `enum ExecutionMode { Simulation, Paper, Live }` — 두 bool 대신 명시적 3-모드
2. **점진적 전환**: Phase별로 기능 추가, 각 Phase에서 독립적으로 테스트 가능
3. **안전 우선**: kill switch, 포지션 persist, 레그 리스크 핸들링이 주문 실행보다 먼저 구현
4. **기존 아키텍처 존중**: monitor.rs의 이벤트 루프 구조 유지, OrderExecutor를 주입하는 방식

### 아키텍처 변경 개요

```
┌─────────────────────────────────────────────────────────────┐
│                     ZScoreMonitor                            │
│  (기존 이벤트 루프 유지, 시그널/검증 로직 변경 없음)           │
├─────────────────────────────────────────────────────────────┤
│              ExecutionMode (enum, NEW)                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ SimExecutor   │  │ LiveExecutor │  │ PaperExecutor│      │
│  │ (기존 가상)    │  │ (실주문)      │  │ (testnet주문) │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
├─────────────────────────────────────────────────────────────┤
│           PositionStore (NEW) + WAL 패턴                     │
│  포지션 상태 머신: Opening → Open → Closing → Closed         │
│  JSON 영속화 + temp file atomic write                        │
│  비동기 쓰기 (mpsc channel 또는 spawn_blocking)               │
├─────────────────────────────────────────────────────────────┤
│                  RiskManager (NEW)                           │
│  Kill switch (AtomicBool), drawdown, 잔고 확인               │
│  일일 리셋, 연결 상태 모니터링                                │
├─────────────────────────────────────────────────────────────┤
│                  AlertService (NEW)                          │
│  arb-telegram 연동, mpsc 비동기 전송                         │
└─────────────────────────────────────────────────────────────┘
```

### Upbit 시장가 매수의 특수성

Upbit 시장가 매수(`OrderType::Price`)는 **수량이 아닌 총 KRW 금액**을 지정한다.
실제 체결 수량은 체결 후에야 확정되며, 수수료는 코인 수량에서 차감된다.

```
# qty 기반 주문이 불가능하므로 총액 기반으로 변환
krw_amount = qty * upbit_krw_price * (1 + upbit_fee_rate)
→ Upbit place_order(type=Price, price=krw_amount)
→ 체결 후 실 수량 확인: received_qty = executed_volume (수수료 차감 후)
→ effective_qty = min(received_qty, bybit_filled_qty)
→ 초과분(bybit_filled - effective_qty) 즉시 반대 주문으로 청산
```

**대안: IOC 지정가 주문** — qty 직접 지정 가능, 슬리피지 상한 제어. 미체결분은 자동 취소.
→ **권장**: IOC 지정가를 기본으로, 시장가는 fallback 옵션으로 config 선택.

### 주문 실행 순서 전략

차익거래 레그 순서:
1. **양 레그 동시 발주** (`tokio::join!`): 시간 최소화, 기본 전략
2. 한쪽 실패 시 체결된 쪽 즉시 반대 주문(비상 청산)
3. **Client Order ID (identifier)**: 네트워크 타임아웃 후 재시도 시 중복 주문 방지 (멱등성 보장)

---

## 구현 플랜

### Phase 1: 안전 인프라 (P0 — 주문 실행 전 필수)

#### 1-1. unwrap 패닉 제거

**문제**: `counters.lock().unwrap()` 30+곳, `ComputingFlags` 4곳에서 Mutex poisoning 시 패닉 → 실거래 중 크래시

**변경**:
- `std::sync::Mutex` → `parking_lot::Mutex` 전환 (poisoning 없음, 성능 우수)
  - 대상: `MonitoringCounters`, `ComputingFlags`, `coin_selector.rs`, `spread.rs` 내 `std::sync::Mutex` 사용처
  - **주의**: `tokio::sync::Mutex` (position_mgr, trades, session_writer)와 `tokio::sync::RwLock` (spread_calc, ob_cache.data)은 **전환하지 않음** (.await를 넘나드는 lock hold가 있으므로)
- `unwrap_or_else(|e| e.into_inner())` 패턴은 **사용하지 않음** (poisoned 상태의 데이터가 inconsistent할 수 있으므로)
- `SystemTime::now().duration_since(UNIX_EPOCH).expect()` → `.unwrap_or_default()` + warn 로그
  - **연쇄 효과**: timestamp 0 시 Bybit 서명 거부 → 해당 주문 차단 (정상 fallback)
- `.expect()` 제거 대상: `coin_selector.rs` 7곳, `spread.rs` 1곳

**파일**: `monitor.rs`, `orderbook.rs`, `coin_selector.rs`, `spread.rs`, `bybit/auth.rs`
**규모**: M

#### 1-2. 포지션 영속화 (PositionStore) + WAL 패턴

**문제**: `PositionManager`가 순수 인메모리 → crash 시 열린 포지션 정보 소실

**변경**:
- `position_store.rs` 신규 생성
- `PositionStore` trait: `save()`, `load()`, `remove()`, `update_state()`
- 구현체: `JsonPositionStore` (JSON 파일 기반, `output/{session}/positions.json`)

**포지션 상태 머신**:
```
Opening → Open → Closing → Closed
           │
           └─→ PartiallyClosedOneLeg (비상 상태)
```
- 주문 발주 **전** "Opening" 상태 선기록 (WAL: Write-Ahead Logging)
- 체결 완료 후 "Open"으로 전이
- 청산 시작 시 "Closing" 기록 → 완료 후 "Closed"로 전이 + 삭제
- 재시작 시 "Opening"/"Closing" 상태 포지션 → reconciliation 강제 실행

**Atomic write**: temp file + rename (POSIX atomic)
- temp 파일 존재 시(rename 전 크래시): 시작 시 temp 파일 감지 → 복구 시도

**비동기 I/O**: `PositionManager` lock 내에서 blocking 파일 I/O 방지
- `mpsc::channel<PersistCommand>`로 쓰기 위임 → 별도 task에서 `spawn_blocking` 실행
- (AlertService와 동일한 비동기 패턴)

**`VirtualPosition`에 추가 필드**:
- `#[derive(Serialize, Deserialize)]` 추가 (serde)
- `upbit_order_id: Option<String>`, `bybit_order_id: Option<String>`
- `state: PositionState` (Opening, Open, Closing, Closed, PartiallyClosedOneLeg)
- 기존 테스트 호환: `Option<String>` 기본값 `None`, `state` 기본값 `Open`

**load_positions() 실패 시**: 빈 상태로 시작하지 않고, **reconciliation 강제 실행** → 거래소 실포지션 기반 복원. 복원 불가 시 신규 진입 차단 + 알림.

**파일**: `position_store.rs` (신규), `position.rs`, `monitor.rs`
**규모**: L

#### 1-3. Kill Switch + Risk Manager

**변경**:
- `risk.rs` 신규 생성
- `RiskManager` 구조체:
  ```rust
  pub struct RiskManager {
      config: RiskConfig,
      daily_realized_pnl: Decimal,
      peak_equity: Decimal,
      current_equity: Decimal,
      is_killed: AtomicBool,            // lock 없이 원자적 확인/설정
      last_reset: DateTime<Utc>,        // 일일 리셋 기준
  }
  ```
- **AtomicBool**: `spawned_check_tick_signal`과 이벤트 루프에서 동시 접근 가능 → lock 없이 `Ordering::Acquire/Release`로 확인/설정
- **일일 리셋**: `last_reset` + 24h 경과 시 `daily_realized_pnl` 리셋 (UTC 기준)

- 메서드:
  - `record_trade(pnl)`: 누적 PnL 기록, kill switch 조건 체크
  - `is_entry_allowed() -> bool`: kill switch 상태 + 연결 상태 확인
  - `trigger_kill_switch(reason)`: 강제 발동
  - `check_connection_health(upbit_ok, bybit_ok)`: 한쪽 연결 불안정 시 진입 차단

- **Kill switch 발동 시 정리 과정**:
  1. `is_killed.store(true, Ordering::Release)` — 즉시 신규 진입 차단
  2. 활성 포지션 **순차 청산** (rate limit 준수, 한 포지션씩)
  3. 각 청산 결과를 즉시 PositionStore에 반영
  4. 전체 청산 완료 후 "KILL SWITCH COMPLETE" 텔레그램 발송
  5. **해제**: 수동 확인 + 프로세스 재시작으로만 가능 (자동 해제 없음)
  6. kill switch 청산은 **별도 task로 spawn** (현재 lock chain 외부에서 실행, deadlock 방지)

- **Lock order**: `RiskManager`는 lock으로 보호하되, `is_killed`는 AtomicBool이므로 lock order에 영향 없음. `record_trade()` 등은 `position_mgr` lock 해제 후 호출.

**설정 추가** (`strategy.toml`):
```toml
kill_switch_enabled = true
max_daily_loss_usdt = 500.0       # 일일 손실 한도
max_drawdown_usdt = 300.0         # 최대 drawdown 한도
max_single_loss_usdt = 100.0      # 단일 거래 최대 손실
max_concurrent_positions = 10     # 동시 오픈 포지션 수 한도
```

**파일**: `risk.rs` (신규), `config.rs`, `monitor.rs`
**규모**: M

#### 1-4. 텔레그램 알림 연동 (AlertService)

**변경**:
- `alert.rs` 신규 생성
- `AlertService` 구조체: `arb-telegram::TelegramClient` 래핑
- 이벤트 타입:
  - `EntryExecuted { coin, qty, prices, expected_pnl }`
  - `ExitExecuted { coin, qty, realized_pnl }`
  - `KillSwitchTriggered { reason, daily_pnl }`
  - `KillSwitchComplete { closed_count, total_pnl }`
  - `LegFailure { coin, succeeded_leg, failed_leg, action_taken }`
  - `EmergencyCloseFailure { coin, retry_count, naked_exposure }`
  - `ReconciliationMismatch { coin, internal_qty, exchange_qty }`
  - `ConnectionLost { exchange, has_open_positions }`
  - `Error { message }`
  - `DailySummary { trades, pnl, win_rate }`
- `mpsc::channel<AlertEvent>`로 비동기 전송 (이벤트 루프 블로킹 방지)
- `ZScoreMonitor`에 `AlertService` 주입

**파일**: `alert.rs` (신규), `monitor.rs`, `config.rs`
**의존**: `arb-telegram`
**규모**: M

#### 1-5. 에러 분류 강화

**변경**:
- `ExchangeError`에 `is_retryable(&self) -> bool` 메서드 추가:
  - `RateLimitExceeded`, `HttpError(timeout)` → `true`
  - `AuthError`, `InsufficientFunds`, `InvalidParameter` → `false`
- `spawned_check_tick_signal`의 `Box<dyn Error>` → `StrategyError` 전환
- 가격 `Decimal::ZERO` fallback 시 즉시 return 가드 추가

**파일**: `error.rs` (arb-exchange), `monitor.rs`
**규모**: S

#### 1-6. WebSocket 자동 재연결 (**H3 → Phase 1 승격**)

**문제**: WebSocket 끊김 시 `select!` 루프 종료 → 활성 포지션이 있는 상태에서 청산 시그널을 놓침

**변경**:
- WebSocket 끊김 감지 시:
  1. 활성 포지션이 있으면 → RiskManager에 연결 상태 보고 → 신규 진입 차단
  2. 재연결 시도 (지수 백오프: 1s, 2s, 4s, 8s, max 30s)
  3. 재연결 성공 → 재구독 (모니터링 코인 전체)
  4. 재연결 5분 실패 → 활성 포지션 전체 시장가 청산 + 텔레그램 알림
- `_forex_task` JoinHandle 모니터링 + 패닉 시 재시작

**파일**: `monitor.rs`, `stream.rs` (arb-exchange)
**규모**: M

---

### Phase 2: 주문 실행 엔진

#### 2-1. Bybit 선물 API 확장 (**Phase 2-2보다 선행**)

**문제**: Bybit SDK의 `DEFAULT_CATEGORY = "spot"` 하드코딩. 선물 short 필요.

**변경**:
- `BybitClient`에 요청별 category override 지원:
  - 오더북/시세 조회: `"spot"` 유지 (시뮬레이션과 동일)
  - 주문/포지션 조회: `"linear"` 사용
  - `place_order`에 `category` 파라미터 추가 또는 `with_category()` 빌더 활용
- 선물 전용 API 추가 (Bybit 전용, Upbit은 현물이므로 해당 없음):
  - `set_leverage(symbol, leverage)`: 레버리지 설정
  - `switch_margin_mode(symbol, mode)`: Isolated/Cross 전환
  - `get_positions(symbol)`: 실 포지션 조회 → `PositionInfo` 반환
- `PositionInfo` 타입 추가 (types.rs):
  ```rust
  pub struct PositionInfo {
      pub symbol: String,
      pub side: String,           // "Buy" or "Sell"
      pub size: Decimal,
      pub entry_price: Decimal,
      pub leverage: Decimal,      // String → Decimal (타입 안전)
      pub unrealised_pnl: Decimal,
      pub liq_price: Decimal,
  }
  ```
- Bybit 전용이므로 `traits.rs`에 `AccountManagement` trait 추가 대신, `BybitClient`에 직접 구현

**파일**: `bybit/client.rs`, `bybit/types.rs`, `arb-exchange/types.rs`
**규모**: M

#### 2-2. Rate Limiter 분리 (Bybit)

**문제**: Bybit 단일 limiter(18 req/sec)를 public/private API가 공유 → 주문 시 오더북 조회와 경합

**변경**:
- `BybitClient`: `public_limiter` (10 req/sec) + `private_limiter` (10 req/sec) 분리
- 오더북 조회 → `public_limiter`, 주문/잔고/포지션 → `private_limiter`

**파일**: `bybit/client.rs`
**규모**: S

#### 2-3. OrderExecutor trait + SimExecutor

**변경**:
- `order_executor.rs` 신규 생성
- `async_trait`을 `[dependencies]`로 승격 (현재 `[dev-dependencies]`만)
- trait 정의:
  ```rust
  /// 주문 실행 요청 (구현체 중립)
  pub struct EntryRequest {
      pub coin: String,
      pub qty: Decimal,
      pub upbit_krw_price: Decimal,   // SimExecutor: 가상 체결가, LiveExecutor: IOC 지정가/총액 계산용
      pub bybit_usdt_price: Decimal,  // SimExecutor: 가상 체결가, LiveExecutor: 참고값(시장가 시 무시)
      pub usd_krw: f64,
      pub instrument_info: InstrumentInfo,
  }

  pub struct ExitRequest {
      pub coin: String,
      pub qty: Decimal,
      pub instrument_info: InstrumentInfo,
  }

  #[async_trait]
  pub trait OrderExecutor: Send + Sync {
      async fn execute_entry(&self, req: EntryRequest) -> Result<ExecutedEntry, OrderExecutionError>;
      async fn execute_exit(&self, req: ExitRequest) -> Result<ExecutedExit, OrderExecutionError>;
  }
  ```
- **OrderExecutionError** 에러 타입:
  ```rust
  pub enum OrderExecutionError {
      BothUnfilled,                                     // 양쪽 미체결 (안전, 진입 포기)
      SingleLegFilled { leg: Leg, emergency_closed: bool }, // 한쪽만 체결
      EmergencyCloseFailed { leg: Leg, order_id: String },  // 비상 청산 실패 (최대 위험)
      QtyMismatch { upbit_qty: Decimal, bybit_qty: Decimal }, // 수량 불일치 (조정 필요)
      InsufficientBalance { exchange: String, available: Decimal },
      ExchangeError(ExchangeError),
      Timeout,
  }
  ```
- `ExecutedEntry` / `ExecutedExit`:
  ```rust
  pub struct ExecutedEntry {
      pub upbit_order_id: String,
      pub bybit_order_id: String,
      pub upbit_filled_qty: Decimal,
      pub bybit_filled_qty: Decimal,
      pub upbit_avg_price_krw: Decimal,  // KRW 실체결가
      pub bybit_avg_price: Decimal,      // USDT 실체결가
      pub upbit_fee: Decimal,            // 실 수수료 (Order.paid_fee)
      pub bybit_fee: Decimal,            // 실 수수료
      pub effective_qty: Decimal,         // min(upbit_qty, bybit_qty)
  }
  ```
- `SimExecutor`: 기존 가상 체결 로직을 OrderExecutor로 래핑 (현재 동작 100% 유지)
- `PaperExecutor`: Bybit testnet + Upbit은 실 API 조회만(주문 미실행) → 체결은 가상 시뮬레이션
  - Upbit testnet 없음 → PaperExecutor에서 Upbit 주문만 가상 처리

**파일**: `order_executor.rs` (신규)
**규모**: L

#### 2-4. LiveExecutor 구현

**변경**:
- `live_executor.rs` 신규 생성
- **진입 실행 흐름**:
  1. 잔고 확인: `upbit.get_balance("KRW")`, `bybit.get_balance("USDT")`
     - Bybit 가용 증거금: `total - sum(각 포지션 증거금)` 계산
  2. Upbit 주문 준비:
     - **IOC 지정가 (기본)**: `OrderRequest::limit_buy(market, qty, price).with_time_in_force(IOC)`
       - price = `upbit_krw_price * (1 + max_slippage_pct)` (슬리피지 상한)
     - **시장가 (fallback)**: `krw_amount = qty * upbit_krw_price * (1 + upbit_fee_rate)`
       → `OrderRequest::market_buy(market, krw_amount)`
  3. Client Order ID 생성: `format!("{coin}_{timestamp}_{seq}")` (멱등성 보장)
  4. 양 레그 동시 발주: `tokio::join!(upbit.place_order(buy), bybit.place_order(sell))`
  5. 체결 대기: `poll_until_filled(order_id, timeout=5s)`
     - polling 간격: 200ms → 500ms → 1s → 2s (지수 백오프)
     - 시장가 미체결 5초 = 거래소 장애 의미 → 즉시 cancel 시도
  6. 결과 처리:
     - 양쪽 Filled → `effective_qty = min(upbit_filled, bybit_filled)`, 초과분 청산
     - 한쪽 Filled + 한쪽 미체결 → 미체결 쪽 cancel → 체결된 쪽 비상 청산
     - 양쪽 미체결 → 양쪽 cancel, 진입 포기
  7. **Post-execution PnL gate**: 실체결가 기반 스프레드 확인, 수익 구간 밖이면 즉시 청산
  8. 결과 반환: `ExecutedEntry`

- **비상 청산 실패 escalation**:
  1. 비상 청산 재시도: 지수 백오프 (1s, 2s, 4s, 8s, 16s, 30s...)
  2. 5분 경과 후 모든 재시도 실패 → **kill switch 강제 발동**
  3. 텔레그램 `EmergencyCloseFailure` 알림 (naked exposure 경고)
  4. 비상 청산 손실은 `RiskManager.record_trade()`에 반드시 포함

- **Cancel 실패 처리**: cancel 실패 → `get_order()` 재확인 → 이미 체결되었으면 정상 포지션으로 등록

- **청산 실행 흐름**: 진입과 동일 패턴, 방향 반전 (Upbit 매도 + Bybit 매수)
  - Upbit 매도: 수량 기반 시장가 가능 (`OrderRequest::market_sell(market, qty)`)

- **주문 유형**: IOC 지정가 기본 (qty 제어 가능), 시장가 fallback (config)
- **재시도**: `is_retryable()` 에러 시 최대 `max_retry_count`회 재시도 (지수 백오프)

**파일**: `live_executor.rs` (신규)
**규모**: XL

---

### Phase 3: monitor.rs 통합

#### 3-1. 모드 분기 적용

**변경**:
- `ZScoreMonitor`에 `Box<dyn OrderExecutor>` 필드 추가
- `config.execution_mode` 값에 따라:
  - `Simulation` → `SimExecutor` 주입 (기존 동작 100% 유지)
  - `Paper` → `PaperExecutor` 주입
  - `Live` → `LiveExecutor` 주입
- 진입/청산 코드 블록을 `order_executor.execute_entry()` / `execute_exit()` 호출로 교체

**청산 발생 지점 3곳**:
1. 정상 Z-Score 시그널 청산 (monitor.rs ~:1615)
2. TTL 만료 청산 (`check_ttl_positions`, minute_timer에서 호출)
3. Kill switch 강제 청산 (별도 spawn task)

**OrderExecutor를 spawned task에 전달**: `Arc<dyn OrderExecutor>` 클로닝

**VirtualPosition 호환성**: `order_id` 필드를 `Option<String>`으로 추가, 기존 테스트에서 `Default` 또는 빌더 패턴으로 호환 유지.

**파일**: `monitor.rs`
**규모**: L

#### 3-2. 시작 시 초기화 강화

**변경**:
- (Live 모드) Bybit 선물 설정 검증: `set_leverage(1x)`, `switch_margin_mode("Isolated")`
- (Live 모드) 잔고 확인:
  - `upbit_krw = upbit.get_balance("KRW")`
  - `bybit_usdt = bybit.get_balance("USDT")`
  - `available_capital = min(upbit_krw / usd_krw, bybit_usdt)`
  - 잔고 부족 시 경고 + 진입 불가
- 미청산 포지션 복원: `PositionStore::load()`
  - load 실패 시 → reconciliation 강제 실행
  - "Opening"/"Closing" 상태 포지션 → reconciliation 강제 실행
- NTP 시간 동기화 확인: 서버 시간과 로컬 시간 차이 > 5초 → warn + 진행

**파일**: `monitor.rs`
**규모**: M

#### 3-3. 포지션 Reconciliation

**변경**:
- `minute_timer` 주기(1분)에 포지션 동기화 체크 추가
- Bybit `get_positions()` → 내부 `PositionManager` 상태와 비교
- Upbit `get_balances()` → 코인 잔고 확인
- 불일치 시:
  - warn 로그 + 텔레그램 `ReconciliationMismatch` 알림
  - **신규 진입 차단** (불일치 해소까지)
  - 자동 수정: 거래소 실포지션 기준으로 내부 상태 조정

**파일**: `monitor.rs`
**규모**: M

#### 3-4. Bybit 펀딩비 모니터링 (**Phase 4에서 승격**)

**변경**:
- `minute_timer`에서 8시간마다 Bybit 펀딩비 조회
- `unrealized_funding_fee` 추적 → RiskManager에 반영
- 펀딩비가 포지션 수익 대비 과도(예: >50%)할 경우 경고 알림

**파일**: `monitor.rs`, `bybit/client.rs`
**규모**: S

---

### Phase 4: 설정 및 부가 기능

#### 4-1. Config 확장

**추가 설정** (`strategy.toml`):
```toml
[zscore]
# 실행 모드: "simulation", "paper", "live"
execution_mode = "simulation"
bybit_category = "linear"        # 선물 카테고리

# 주문 실행
order_timeout_sec = 5             # 주문 체결 대기 타임아웃 (시장가 기준)
max_retry_count = 2               # 재시도 횟수
order_type = "limit_ioc"          # "limit_ioc" or "market"
max_slippage_pct = 0.1            # IOC 지정가 시 최대 슬리피지 %

# 리스크 관리
kill_switch_enabled = true
max_daily_loss_usdt = 500.0       # 일일 손실 한도
max_drawdown_usdt = 300.0         # 최대 drawdown 한도
max_single_loss_usdt = 100.0      # 단일 거래 최대 손실
max_concurrent_positions = 10     # 동시 포지션 수 한도

# 텔레그램 알림
telegram_enabled = true
telegram_chat_id = ""             # 환경변수 우선
```

**파일**: `config.rs`
**규모**: M

#### 4-2. 환율 staleness 가드

**변경**:
- 환율 캐시 age > 30분이면 신규 진입 차단
- 환율 급변 감지: 전분 대비 0.5% 이상 변동 시 경고 알림

**파일**: `monitor.rs`
**규모**: S

#### 4-3. 실체결 수수료 PnL 반영

**변경**:
- `ClosedPosition`에 `actual_upbit_fee`, `actual_bybit_fee` 필드 추가
- Live 모드: `Order.paid_fee` 기반 실 수수료 사용
- Simulation 모드: 기존 config fee rate 기반 (변경 없음)
- `ClosedPosition`에 `funding_fee` 필드 추가, PnL 계산에 포함

**파일**: `pnl.rs`, `position.rs`, `bybit/client.rs`
**규모**: S

---

## 파일 변경 목록

### 신규 파일

| 파일 | Phase | 설명 |
|------|-------|------|
| `crates/arb-strategy/src/zscore/order_executor.rs` | 2 | OrderExecutor trait + SimExecutor + PaperExecutor |
| `crates/arb-strategy/src/zscore/live_executor.rs` | 2 | LiveExecutor 실주문 구현 |
| `crates/arb-strategy/src/zscore/position_store.rs` | 1 | 포지션 영속화 + WAL + 상태 머신 |
| `crates/arb-strategy/src/zscore/risk.rs` | 1 | RiskManager + Kill Switch (AtomicBool) |
| `crates/arb-strategy/src/zscore/alert.rs` | 1 | AlertService (텔레그램 연동) |

### 변경 파일

| 파일 | Phase | 규모 | 핵심 변경 |
|------|-------|------|----------|
| `monitor.rs` | 1,3 | XL | unwrap 제거, WebSocket 재연결, OrderExecutor 주입, 초기화/reconciliation |
| `position.rs` | 1,2 | L | Serialize/Deserialize, PositionState, order_id, from_executed() |
| `config.rs` | 1,4 | M | ExecutionMode enum, kill switch, 주문 파라미터 설정 추가 |
| `orderbook.rs` | 1 | S | `std::sync::Mutex` → `parking_lot::Mutex` |
| `coin_selector.rs` | 1 | S | `.expect()` 7곳 + `.lock().unwrap()` 1곳 제거 |
| `spread.rs` | 1 | S | `.expect()` 1곳 제거 |
| `bybit/client.rs` | 2 | M | category 요청별 override, 선물 API, rate limiter 분리 |
| `bybit/types.rs` | 2 | S | PositionInfo 응답 타입 |
| `arb-exchange/types.rs` | 2 | S | PositionInfo 공통 타입 |
| `arb-exchange/error.rs` | 1 | S | is_retryable() 메서드 |
| `arb-exchange/src/lib.rs` | 2 | S | 신규 타입 re-export |
| `arb-strategy/Cargo.toml` | 1 | S | `parking_lot` 추가, `async_trait` dev→dependencies 승격, `serde` 추가 |
| `pnl.rs` | 4 | S | 실수수료 + funding_fee 포함 PnL |
| `output/writer.rs` | 3 | S | order_id, 실체결가 기록 |
| `mod.rs` | 2 | S | 신규 모듈 등록 |

### 변경하지 않는 파일

| 파일 | 이유 |
|------|------|
| `signal.rs` | 시그널 생성 로직은 실거래에서도 동일 |
| `instrument.rs` | 라운딩 유틸리티 그대로 사용 |

---

## 리스크 매트릭스

### Critical (실거래 전 필수 해결)

| ID | 항목 | 현재 상태 | 대응 | Phase |
|----|------|----------|------|-------|
| C1 | `lock().unwrap()` 30+ 지점 | Mutex poison 시 패닉 | `parking_lot::Mutex` 전환 (std::sync만 대상) | 1-1 |
| C2 | ComputingFlags unwrap | 패닉 시 틱 영구 중단 | 동일 | 1-1 |
| C3 | SystemTime expect | NTP 역전 시 패닉 | `unwrap_or_default` + timestamp 0 시 주문 차단 | 1-1 |
| C4 | 포지션 메모리 only | crash 시 소실 | PositionStore + WAL + 상태 머신 | 1-2 |
| C5 | WebSocket 재연결 없음 | 단절 시 프로세스 종료 | 자동 재연결 + 포지션 보호 | 1-6 |

### High (실거래 품질 필수)

| ID | 항목 | 현재 상태 | 대응 | Phase |
|----|------|----------|------|-------|
| H1 | Bybit rate limiter 공유 | 주문 vs 오더북 경합 | public/private 분리 | 2-2 |
| H2 | 양 레그 원자성 없음 | naked leg 노출 위험 | LiveExecutor + 비상 청산 escalation | 2-4 |
| H3 | Decimal::ZERO fallback | 0가격 진입 위험 | ZERO 가드 추가 | 1-5 |
| H4 | Upbit 시장가 매수 총액 기반 | qty 제어 불가 | IOC 지정가 기본 + 총액 변환 로직 | 2-4 |
| H5 | 비상 청산 이중 실패 | naked exposure 무제한 | 5분 초과 → kill switch 강제 발동 | 2-4 |

### Medium

| ID | 항목 | 대응 | Phase |
|----|------|------|-------|
| M1 | Box<dyn Error> erased | StrategyError 전환 | 1-5 |
| M2 | forex_task JoinHandle 무시 | 모니터링 + 재시작 | 1-6 |
| M3 | API 키 평문 메모리 | secrecy 크레이트 (향후) | — |
| M4 | is_retryable 없음 | ExchangeError에 추가 | 1-5 |
| M5 | Bybit category "spot" 하드코딩 | 요청별 category override | 2-1 |
| M6 | 중복 주문 위험 | Client Order ID (identifier) | 2-4 |
| M7 | 거래소 점검/입출금 정지 | 상태 API 조회 + 진입 차단 | 3-2 |
| M8 | Bybit 강제 청산 실시간 감지 | WS position 이벤트 수신 (향후) | — |

---

## 단계적 롤아웃 계획

```
Phase 0 (현재) ─── 시뮬레이션 검증 완료
     │
Phase 1 ───────── 안전 인프라 구축
     │              ├── unwrap 패닉 제거 (parking_lot)
     │              ├── PositionStore (영속화 + WAL + 상태 머신)
     │              ├── RiskManager (kill switch, AtomicBool)
     │              ├── AlertService (텔레그램)
     │              ├── 에러 분류 강화
     │              └── WebSocket 자동 재연결 ★ 승격
     │
Phase 2 ───────── 주문 실행 엔진
     │              ├── Bybit 선물 API 확장 ★ 선행
     │              ├── Rate limiter 분리
     │              ├── OrderExecutor trait + SimExecutor + PaperExecutor
     │              └── LiveExecutor 구현 (IOC 지정가 기본)
     │
Phase 3 ───────── monitor.rs 통합
     │              ├── ExecutionMode 분기 (Sim/Paper/Live)
     │              ├── 시작 시 초기화 강화
     │              ├── 포지션 reconciliation (불일치 시 진입 차단)
     │              └── Bybit 펀딩비 모니터링 ★ 승격
     │
Phase 4 ───────── 설정 및 부가 기능
     │              ├── Config 확장
     │              ├── 환율 staleness/급변 가드
     │              └── 실체결 수수료 + 펀딩비 PnL 반영
     │
     ▼
검증 단계 (각 단계별 합격 기준 명시):
  ① Paper Trading ─── Bybit testnet + Upbit 가상
     합격: API 호출 성공률 > 99%, 레그 실패 복구 정상, 24h 무크래시
  ② 소액 실거래 ($300~500) ── 양 레그 원자성 검증
     합격: 10건 이상 진입/청산 성공, 레그 실패 0건, 시뮬 대비 PnL 오차 < 30%
  ③ 중액 실거래 ($1,000~3,000) ── 슬리피지/마켓임팩트 실측
     합격: 시간당 거래 수 안정, kill switch 테스트 통과, 24h 무중단
  ④ 풀자본 ($10,000) ── 모니터링 안정 확인 후
     합격: 72h 무중단, reconciliation 불일치 0건, 수익률 시뮬 대비 50% 이상
```

---

## 체크리스트

### Phase 1: 안전 인프라
- [ ] `parking_lot` 크레이트 추가, `std::sync::Mutex` → `parking_lot::Mutex` 전환 (monitor.rs, orderbook.rs, coin_selector.rs, spread.rs)
- [ ] `tokio::sync::Mutex`/`RwLock`은 변경하지 않음 확인
- [ ] `SystemTime::expect()` → `unwrap_or_default()` + warn 로그 (bybit/auth.rs) + timestamp 0 시 주문 차단
- [ ] `.expect()` 제거: coin_selector.rs 7곳, spread.rs 1곳
- [ ] `Decimal::ZERO` fallback 시 즉시 return 가드 (monitor.rs)
- [ ] `position_store.rs` 생성: `PositionStore` trait + `JsonPositionStore` + WAL 패턴
- [ ] `VirtualPosition`에 `#[derive(Serialize, Deserialize)]`, `PositionState`, `order_id` 추가
- [ ] 포지션 상태 머신: Opening → Open → Closing → Closed + PartiallyClosedOneLeg
- [ ] 비동기 쓰기: mpsc 채널 기반 persist (spawn_blocking)
- [ ] temp 파일 복구 로직 (시작 시 temp 파일 감지)
- [ ] `load_positions()` 실패 시 reconciliation 강제 + 진입 차단
- [ ] `risk.rs` 생성: `RiskManager` + AtomicBool kill switch
- [ ] `config.rs`에 kill switch 설정 + `max_single_loss_usdt` + `max_concurrent_positions`
- [ ] kill switch 발동 시 별도 task spawn + 순차 청산 + PositionStore 즉시 반영
- [ ] kill switch 해제 = 수동 + 프로세스 재시작만
- [ ] RiskManager 일일 리셋 로직 (UTC 기준)
- [ ] `alert.rs` 생성: `AlertService` + mpsc 비동기 전송
- [ ] 알림 이벤트 타입 10종 구현
- [ ] `ExchangeError::is_retryable()` 메서드 추가
- [ ] `spawned_check_tick_signal` 에러 타입 `Box<dyn Error>` → `StrategyError` 전환
- [ ] WebSocket 자동 재연결 + 재구독 (지수 백오프)
- [ ] 재연결 5분 실패 시 활성 포지션 전체 청산
- [ ] `_forex_task` JoinHandle 모니터링 + 패닉 시 재시작
- [ ] lock order 문서 갱신: RiskManager 위치 명시
- [ ] `cargo test -p arb-strategy` 전체 통과
- [ ] `cargo clippy` 경고 0

### Phase 2: 주문 실행 엔진
- [ ] Bybit 선물 API: `set_leverage()`, `switch_margin_mode()`, `get_positions()` 구현
- [ ] Bybit category 요청별 override (오더북=spot, 주문=linear)
- [ ] `PositionInfo` 타입 추가 (leverage: Decimal)
- [ ] Bybit rate limiter `public_limiter` / `private_limiter` 분리
- [ ] `async_trait` dev-dependencies → dependencies 승격
- [ ] `order_executor.rs` 생성: `OrderExecutor` trait + `EntryRequest`/`ExitRequest` 구조체
- [ ] `OrderExecutionError` enum 정의 (7 variant)
- [ ] `ExecutedEntry` / `ExecutedExit` 타입 (실수수료 필드 포함)
- [ ] `SimExecutor` 구현: 기존 가상 체결 로직 래핑
- [ ] `PaperExecutor` 구현: Bybit testnet + Upbit 가상
- [ ] `live_executor.rs` 생성: `LiveExecutor` 구현
- [ ] Upbit IOC 지정가 기본 + 시장가 fallback (config)
- [ ] Client Order ID (identifier) 멱등성 보장
- [ ] 양 레그 동시 발주 (`tokio::join!`) + 체결 polling (200ms 지수 백오프)
- [ ] 비상 청산 escalation: 재시도 → 5분 → kill switch
- [ ] Cancel 실패 → get_order 재확인 → 체결 시 정상 등록
- [ ] Partial fill → effective_qty 조정 + 초과분 청산 (비용 회계 포함)
- [ ] Post-execution PnL gate
- [ ] 비상 청산 손실 → RiskManager.record_trade() 포함
- [ ] 단위 테스트: SimExecutor, 양쪽 성공/한쪽 실패/타임아웃/partial fill
- [ ] `cargo test` 전체 통과 + `cargo clippy` 경고 0

### Phase 3: monitor.rs 통합
- [ ] `ZScoreMonitor`에 `Arc<dyn OrderExecutor>` 필드 추가
- [ ] `ExecutionMode` enum 분기: Simulation/Paper/Live
- [ ] 진입 블록: `order_executor.execute_entry()` 호출로 교체 (spawned task에 Arc 전달)
- [ ] 청산 블록 3곳: 정상 시그널(:1615), TTL 만료, kill switch
- [ ] 시작 시 Bybit 선물 설정 검증 (leverage, margin_mode)
- [ ] 시작 시 잔고 확인 + `available_capital` 계산 (Upbit KRW + Bybit USDT 분리)
- [ ] 미청산 포지션 복원 (Opening/Closing 상태 → reconciliation 강제)
- [ ] `minute_timer`에 포지션 reconciliation + 불일치 시 진입 차단
- [ ] Bybit 펀딩비 주기적 조회 + RiskManager 반영
- [ ] `Simulation` 모드 시 기존 동작 100% 유지 확인
- [ ] 통합 테스트: SimExecutor + PositionStore + RiskManager end-to-end
- [ ] PositionStore crash recovery 테스트 (파일 기록→로드→일치 확인, 손상 파일 처리)
- [ ] RiskManager 테스트 (daily loss 한도, drawdown, kill switch 작동)
- [ ] `cargo test` 전체 통과 + `cargo clippy` 경고 0

### Phase 4: 설정 및 부가 기능
- [ ] `config.rs`에 나머지 설정 추가 (order_timeout_sec, max_slippage_pct 등)
- [ ] 환율 캐시 age > 30분 시 진입 차단 + 급변(0.5%) 경고
- [ ] `ClosedPosition`에 `actual_upbit_fee`, `actual_bybit_fee`, `funding_fee` 추가
- [ ] Live 모드: Order.paid_fee 기반 실수수료 PnL 계산
- [ ] `cargo test` 전체 통과 + `cargo clippy` 경고 0

### 검증
- [ ] Paper Trading: API 성공률 > 99%, 레그 복구 정상, 24h 무크래시
- [ ] 소액 ($300-500): 10건+ 성공, 레그 실패 0건, PnL 오차 < 30%
- [ ] 중액 ($1,000-3,000): 시간당 거래 안정, kill switch 통과, 24h 무중단
- [ ] 풀자본 ($10,000): 72h 무중단, reconciliation 불일치 0건, 수익률 시뮬 50%+
