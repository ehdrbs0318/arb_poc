# 0011_LIVE_TRADING

## 사용자의 요청

시뮬레이션으로 검증된 Z-Score 차익거래 전략을 **실제 거래소에서 실행**할 수 있도록 전환한다.
현재 가상 체결(VirtualPosition) 기반의 이벤트 루프를 실 주문 API 호출 기반으로 변경하고,
실거래에 필수적인 안전장치(kill switch, DB 영속화, 레그 리스크 관리 등)를 구축한다.

### 배경

- spec/0000~0010에서 시뮬레이션 전략을 완성: 9단계 진입 검증, 오더북 기반 포지션 사이징, tick/lot 라운딩, 비동기 이벤트 루프
- 12.9시간 시뮬레이션 결과: 142건 전승, +373.81 USDT, 시간당 +28.98 USDT
- `OrderManagement` trait + Upbit/Bybit SDK에 `place_order`, `cancel_order`, `get_order` 이미 구현
- `arb-telegram` 크레이트 메시지 전송 인프라 준비 완료

---

## 시뮬레이션 vs 라이브 차이점

시뮬레이션에서 암묵적으로 가정하지만 라이브에서는 명시적으로 처리해야 하는 차이점:

| # | 영역 | 시뮬레이션 | 라이브 | 해결 위치 |
|---|------|-----------|--------|----------|
| 1 | **잔고 관리** | 무제한 자본, 잔고 추적 없음 | 거래소별 가용 잔고 추적 + 자본 예약 필요 | Phase 1-8 BalanceTracker |
| 2 | **체결가 괴리** | 틱 가격 = 체결가 (즉시) | 주문~체결 사이 가격 변동 (수백ms~수초) | Phase 2-4 Post-execution PnL gate |
| 3 | **수수료** | config fee rate 고정 계산 | 거래소가 실제 차감 (paid_fee), 등급별 상이 가능 | Phase 4-2 실수수료 PnL 반영 |
| 4 | **마켓 임팩트** | 가상 체결 = 시장에 영향 없음 | 실주문이 오더북 소비 -> 후속 주문에 영향 | 기존 safe_volume + Phase 2-4 IOC 슬리피지 제어 |
| 5 | **시세 신선도** | 틱 도착 즉시 처리 (지연 0) | 주문 전송~체결까지 가격 stale 가능 | Phase 2-4 IOC 가격에 max_slippage 반영 |
| 6 | **동시 진입 경합** | 순차 처리, 자본 무제한 | 2개 코인 동시 시그널 -> 잔고 경합 | Phase 1-8 자본 예약(reservation) 패턴 |
| 7 | **펀딩비** | 미고려 | 종목별 펀딩 주기/정산 시점이 상이, PnL 직접 영향 | Phase 3-4 종목별 펀딩비 모니터링 |
| 8 | **거래소 장애** | 없음 (항상 성공) | 점검, rate limit, API 에러, 입출금 정지 | Phase 1-6 에러 분류, Phase 1-7 재연결 |
| 9 | **주문 상태** | 없음 (즉시 완료) | order_id 기반 상태 추적 (Pending->Filled->Cancelled) | Phase 2-4 poll_until_filled |
| 10 | **레그 리스크** | 양 레그 항상 동시 성공 | 한쪽만 체결 -> naked exposure | Phase 2-4 비상 청산 escalation |
| 11 | **포지션 영속화** | 메모리 only (crash 시 소실 허용) | crash 시 열린 포지션 복원 필수 | Phase 1-2 PositionStore (DB) |
| 12 | **환율 신선도** | stale 허용 (PnL 영향 미미) | stale 환율 -> KRW/USDT 변환 오류 -> 손실 | Phase 1-9 환율 staleness guard |

### 현재 흐름 (시뮬레이션)

```
tick event -> signal 평가 -> 9단계 진입 검증 통과
  -> VirtualPosition 생성 (즉시, 메모리 only, 잔고 제약 없음)
  -> PnL 계산 (가상 체결가 기반, config fee rate)
```

### 목표 흐름 (실거래)

```
tick event -> signal 평가 -> 9단계 진입 검증 통과
  -> BalanceTracker.reserve(upbit_krw, bybit_usdt)  ★ 메모리 잔고 예약
  -> PositionManager에 "Opening" 상태 등록 (메모리, 즉시)
  -> LiveExecutor.execute_entry()
    ├── tokio::time::timeout(10s, tokio::join!(  ★ 전체 timeout
    │     upbit.place_order(IOC Limit, Buy),
    │     bybit.place_order(IOC Limit, Sell, "linear")  ★ 양쪽 IOC
    │   ))
    ├── ★ order_id DB 동기 기록 (crash recovery용 유일한 동기 DB write)
    ├── poll_until_filled(timeout=5s, backoff 200ms~2s)
    ├── 양쪽 Filled -> effective_qty = min(upbit_qty, bybit_qty)
    │   -> 초과분 즉시 청산 (adjustment_cost 기록)  ★ PnL 회계
    │   -> Post-execution PnL gate: 실체결가 수익 확인  ★ 체결가 괴리 방어
    │   -> "Open" 상태 전이 (메모리) + DB 비동기 반영 + 텔레그램 + BalanceTracker.commit()
    ├── 한쪽만 Filled -> emergency_close(체결된 쪽)
    │   -> IOC 재시도 -> 2분: 시장가 escalation -> 5분: kill switch  ★ 3단계
    └── 양쪽 미체결 -> 양쪽 cancel -> "Opening" 삭제 (메모리) + BalanceTracker.release()
```

---

## 설계

### 핵심 원칙

1. **바이너리 분리**: `src/main.rs` = 라이브 전용 (release 빌드, 최적 성능), `examples/zscore_sim.rs` = 시뮬레이션 (전략 실험용, 기존 가상 체결 유지). 런타임 모드 전환 없음, 동적 디스패치 없음.
2. **점진적 전환**: Phase별로 기능 추가, 각 Phase에서 독립적으로 테스트 가능
3. **안전 우선**: kill switch, DB 영속화, 잔고 관리가 주문 실행보다 먼저 구현
4. **기존 이벤트 루프 구조 존중**: monitor.rs의 select! 루프 구조 유지, LiveExecutor를 구체 타입으로 직접 사용 (vtable 오버헤드 없음)
5. **DB 우선**: 모든 영속 데이터는 MySQL(arb-db)에 저장, 파일 I/O 완전 제거
6. **실시간 판단은 메모리**: 잔고 예약, kill switch, 포지션 잠금/상태 관리 등 실시간 의사결정은 프로세스 내 메모리(Mutex/AtomicBool)로만 수행. DB는 영속화/복구 전용이며 의사결정 경로(hot path)에 포함하지 않음
7. **시뮬레이션 독립**: 시뮬레이션 example은 라이브 인프라(DB, BalanceTracker, RiskManager)에 의존하지 않음. 기존 VirtualPosition + 가상 체결 코드를 그대로 사용

### 바이너리 구조

```
┌────────────────────────────────┐     ┌──────────────────────────────┐
│    src/main.rs (라이브 전용)     │     │  examples/zscore_sim.rs      │
│    cargo run --release          │     │  cargo run --example ...     │
├────────────────────────────────┤     ├──────────────────────────────┤
│  ZScoreMonitor<U, B>           │     │  ZScoreMonitor<U, B>         │
│  + LiveExecutor<U, B> (구체)    │     │  (기존 가상 체결 그대로)       │
│  + BalanceTracker              │     │  VirtualPosition, 파일 출력   │
│  + RiskManager                 │     │  DB/잔고/리스크 인프라 없음    │
│  + AlertService                │     └──────────────────────────────┘
│  + DbPositionStore             │
├────────────────────────────────┤
│  arb-db (MySQL 영속화)          │
│  sessions / positions / trades │
│  minutes / alerts / funding    │
├────────────────────────────────┤
│  BalanceTracker (잔고 추적)     │
│  reserve / commit / release    │
├────────────────────────────────┤
│  RiskManager (Kill Switch)     │
│  AtomicBool + 비율/절대값 한도  │
├────────────────────────────────┤
│  AlertService (텔레그램)        │
│  치명적=동기, 일반=비동기       │
└────────────────────────────────┘
```

**시뮬레이션 example은 현재 코드를 그대로 사용**한다. 라이브 인프라(DB, BalanceTracker, RiskManager, AlertService)에 의존하지 않으며, 전략 파라미터를 바꿔가며 자유롭게 실험할 수 있다. `examples/zscore_sweep.rs` 등 여러 벌의 시뮬 example을 만들 수 있다.

### 실시간 상태 vs 영속 상태 (Dual-State 모델)

**실시간 상태 (메모리)** -- 의사결정의 단일 진실 소스(authoritative):
- `PositionManager` (tokio::sync::Mutex): 포지션 상태 머신, 진입/청산 판단
- `BalanceTracker` (parking_lot::Mutex): 잔고 예약/확정/해제
- `RiskManager` (AtomicBool + parking_lot::Mutex): kill switch, 리스크 한도
- `FundingScheduleCache` (HashMap): 종목별 펀딩 스케줄
- `MonitoringCounters` (parking_lot::Mutex): 카운터

**영속 상태 (DB)** -- crash recovery 전용:
- 메모리 상태 변경 후 비동기로 DB에 반영 (fire-and-forget 또는 buffered write)
- **예외**: order_id 기록은 주문 직후 동기 DB write (crash 시 주문 추적 필수)
- 시작 시 DB에서 메모리 상태 복원 -> 이후 메모리가 권위(authoritative)

```
의사결정 -> 메모리 상태 갱신 (즉시) -> DB 비동기 반영
시작 시:   DB 조회 -> 메모리 상태 복원 -> 이후 메모리만 참조
```

### 비동기 원칙 (spec/0007 계승)

spec/0007에서 확립한 **"select! 루프에서 REST/네트워크 호출 금지"** 원칙을 라이브에서도 엄격히 유지한다. 라이브에서 추가되는 REST 호출(주문 발주, 잔고 조회, reconciliation, 펀딩비 갱신)이 이벤트 루프를 블로킹하면, 틱 처리가 수백ms~수초 지연되어 실거래 기회를 놓치거나 청산 시그널을 놓친다.

**원칙**:
```
┌─────────────────────────────────────────────────────────────────┐
│  select! 루프 (허용)          │  select! 루프 (금지)              │
│  ────────────────────         │  ────────────────────             │
│  이벤트 수신 (WebSocket rx)    │  REST API 호출                    │
│  캔들 빌딩 (HashMap 갱신)      │  DB 쿼리/INSERT                   │
│  스프레드 계산 (메모리)         │  텔레그램 전송                     │
│  AtomicBool/parking_lot 확인   │  주문 발주/체결 대기                │
│  카운터 증가 (< 1μs)          │  잔고 조회 (sync_from_exchange)    │
│  tokio::spawn 트리거           │  포지션 reconciliation            │
│  mpsc 채널 수신               │  펀딩비 API 갱신                   │
│  computing flag CAS           │  WebSocket 재연결 retry            │
└─────────────────────────────────────────────────────────────────┘
```

**라이브에서 추가되는 REST 호출의 실행 위치**:

| 작업 | REST 호출 | 실행 위치 | 트리거 |
|------|----------|----------|--------|
| 주문 발주 + 체결 대기 | place_order, get_order | `spawned_check_tick_signal` 내부 (기존 spawn 활용) | 진입/청산 시그널 |
| 잔고 동기화 | get_balance × 2 | **별도 tokio::spawn** | minute_timer (5분 주기) |
| Reconciliation | get_positions, get_order × N | **별도 tokio::spawn** | minute_timer (1분 주기) |
| 펀딩비 갱신 | getTickers | **별도 tokio::spawn** | minute_timer (1분 주기) |
| WebSocket 재연결 | connect, subscribe | **별도 tokio::spawn** | 끊김 감지 시 |
| Kill switch 청산 | place_order × N | **별도 tokio::spawn** (이미 명시) | kill switch 발동 |
| 치명적 알림 | Telegram send | **spawn된 task 내에서만** 동기 전송 | kill switch, 비상 청산 실패 |

**minute_timer spawn 패턴**:
```
minute_timer.tick() => {
    // ① 메모리 전용 (select! 내, 즉시)
    finalize_and_process()   // 기존: 통계, TTL, 캔들 — 메모리 연산만
    check_ttl_positions()    // 기존: TTL 만료 체크 — 메모리 연산만

    // ② REST 호출 (spawn, select! 비블로킹)
    if 5분_경과 {
        tokio::spawn(sync_from_exchange(...))      // 잔고 동기화
    }
    tokio::spawn(reconciliation_check(...))        // 포지션 정합성
    tokio::spawn(update_funding_schedules(...))    // 펀딩비 갱신
    // → 결과는 mpsc 채널 또는 공유 상태(Arc)로 반환
}
```

### 주문 전략: 양 레그 모두 IOC 지정가

**Upbit IOC 지정가**:
- qty 직접 지정 가능, 슬리피지 상한 제어. 미체결분은 자동 취소.
- **사전 검증 필요**: Live 시작 시 Upbit API가 실제로 IOC를 지원하는지 마켓별 확인. `ord_type`에 IOC 없는 마켓은 **GTC 지정가 + timeout 후 cancel** fallback 자동 적용.
- price = `upbit_krw_price * (1 + max_slippage_pct)` (슬리피지 상한)
- **시장가 fallback** (config): `krw_amount = qty * upbit_krw_price * (1 + upbit_fee_rate)` -> `OrderRequest::market_buy(market, krw_amount)` (수량이 아닌 총 KRW 금액 기반, Upbit 특수성)

**Bybit IOC 지정가** (시장가에서 변경):
- Bybit 선물 시장가는 슬리피지 상한 없음 -> 급변동 시 수익 구간 소멸 위험
- `OrderRequest::limit_sell(symbol, qty, price).with_time_in_force(IOC)`
- price = `bybit_usdt_price * (1 - max_slippage_pct)` (매도이므로 하한)
- qty 기반 지정가 문제 없음, Upbit보다 IOC 적용이 용이

### 주문 실행 순서 전략

차익거래 레그 순서:
1. **양 레그 동시 발주**: `tokio::time::timeout(10s, tokio::join!(upbit, bybit))` -- 전체를 timeout으로 감쌈
   - 개별 `place_order`에도 `tokio::time::timeout(5s, ...)` 적용 (한쪽 지연 시 다른쪽 방치 방지)
2. 한쪽 실패 시 체결된 쪽 즉시 반대 주문(비상 청산)
3. **Client Order ID**: `format!("{coin}_{timestamp}_{seq}")` (멱등성 보장, crash 복구 시 주문 조회용)

### 잔고 관리 (BalanceTracker)

**문제**: 시뮬레이션은 자본 무제한이지만, 라이브에서는 거래소별 가용 잔고를 실시간 추적해야 한다. 특히 `spawned_check_tick_signal`이 `tokio::spawn`으로 병렬 실행되므로, 2개 코인의 진입 시그널이 동시 발생 시 잔고 경합이 발생한다.

**설계**:
```rust
pub struct BalanceTracker {
    upbit_available_krw: Mutex<Decimal>,    // parking_lot::Mutex
    bybit_available_usdt: Mutex<Decimal>,   // parking_lot::Mutex
}

impl BalanceTracker {
    /// 진입 전 잔고 예약. 성공 시 ReservationToken 반환.
    pub fn reserve(&self, upbit_krw: Decimal, bybit_usdt: Decimal) -> Option<ReservationToken>;

    /// 주문 성공 시 예약을 확정 (실 체결 금액으로 잔고 차감).
    pub fn commit(&self, token: ReservationToken, actual_upbit_krw: Decimal, actual_bybit_usdt: Decimal);

    /// 주문 실패/취소 시 예약 해제 (잔고 복원).
    pub fn release(&self, token: ReservationToken);

    /// 청산 완료 시 잔고 복원.
    pub fn on_exit(&self, received_upbit_krw: Decimal, received_bybit_usdt: Decimal);

    /// 거래소 실잔고와 동기화 (minute_timer에서 주기적 호출).
    pub async fn sync_from_exchange(&self, upbit: &U, bybit: &B, usd_krw: f64);
}
```

**동작 흐름**:
1. 시작 시: `upbit.get_balance("KRW")` + `bybit.get_balance("USDT")` -> 초기 잔고 설정
2. 진입 시그널 -> `reserve()` -> 잔고 부족이면 즉시 포기 (로그 + 카운터)
3. 주문 성공 -> `commit()` (실 체결 금액 반영)
4. 주문 실패 -> `release()` (예약 해제)
5. 청산 완료 -> `on_exit()` (잔고 복원)
6. 매분 `sync_from_exchange()` -> 내부 잔고와 실잔고 불일치 시 보정 + warn 로그

**lock order**: `BalanceTracker`의 Mutex는 `parking_lot::Mutex`이며, `position_mgr` lock **외부**에서 호출. reserve -> pm.lock -> commit/release 순서.

### Kill Switch 동시성 설계

**TOCTOU 문제**: `is_entry_allowed()` 확인 후 `position_mgr.lock()` 획득 사이에 kill switch가 발동되면 새 포지션이 방치된다.

**해결**: `position_mgr.lock()` 내부에서 `is_killed` **이중 체크**.

**★ 전체가 `spawned_check_tick_signal` (tokio::spawn된 task) 내에서 실행된다. select! 루프에서 실행하면 안 됨.**
```
[spawned_check_tick_signal 내부]
1. if !risk_manager.is_entry_allowed() -> return   // 메모리 (lock 없이, AtomicBool)
2. balance_tracker.reserve(...)                    // 메모리 잔고 예약 (parking_lot::Mutex, < 1μs)
3. position_mgr.lock().await                       // 메모리 (tokio::sync::Mutex)
4.   if risk_manager.is_killed() -> release + return  // ★ 메모리 이중 체크
5.   pm.register_opening(pos)                      // 메모리 Opening 등록
6. position_mgr.unlock()
7. live_executor.execute_entry()                   // ★ REST 호출 (place_order + poll, spawned task 내이므로 select! 비블로킹)
                                                   //   내부에서 order_id DB 동기 기록
8. position_mgr.lock().await -> 메모리 결과 반영 + DB 비동기 반영
```

**Kill switch 발동 시 배타적 청산**:
1. `is_killed.store(true, Release)` -> 신규 진입 즉시 차단
2. 정상 청산 로직도 비활성화 (PositionState가 "Closing"이 아닌 포지션만 kill switch가 청산)
3. kill switch 청산 task가 **포지션 규모(notional) 내림차순**으로 순차 청산
4. 각 포지션을 "Closing"으로 전이 -> 중복 청산 방지
5. 전체 완료 조건: `PositionManager.open_count() == 0` AND 거래소 reconciliation 통과
6. kill switch 청산 task 자체가 kill switch를 재발동하지 않도록 guard
7. "KILL SWITCH COMPLETE" 텔레그램 발송 후 이벤트 루프 종료

---

## 구현 플랜

### Phase 1: 안전 인프라 (P0 -- 주문 실행 전 필수)

#### 1-0. arb-db 모듈 생성

**목적**: 모든 영속 데이터를 MySQL에 저장. 기존 파일 I/O(WAL, positions.json, trades.csv, minutes.csv, summary.txt, alerts.log) 완전 대체.

**변경**:
- 새 workspace crate `arb-db` 생성
  - `Cargo.toml`: `sqlx = { version = "0.8", features = ["runtime-tokio", "mysql", "chrono", "rust_decimal"] }`
  - 커넥션 풀: `sqlx::mysql::MySqlPool` (초기화 시 `MySqlPool::connect()`)
- `arb-db/src/lib.rs`: `DbPool` 래퍼 타입 + 테이블별 Repository 모듈
- `arb-db/src/migration.rs`: 커스텀 마이그레이션 러너

**DB 테이블 스키마** (핵심 컬럼만 기술, exact type은 구현 시 결정):

```sql
-- 세션 메타데이터
CREATE TABLE sessions (
    id          BIGINT AUTO_INCREMENT PRIMARY KEY,
    started_at  DATETIME(3) NOT NULL,
    ended_at    DATETIME(3),
    config_json TEXT NOT NULL,           -- 세션 시작 시 config snapshot
    status      VARCHAR(20) NOT NULL     -- Running, Completed, Crashed
);

-- 포지션 상태 머신
CREATE TABLE positions (
    id                  BIGINT AUTO_INCREMENT PRIMARY KEY,
    session_id          BIGINT NOT NULL,
    coin                VARCHAR(20) NOT NULL,
    state               VARCHAR(30) NOT NULL,  -- Opening, Open, Closing, Closed, PartiallyClosedOneLeg
    qty                 DECIMAL(20,8) NOT NULL,
    upbit_entry_price   DECIMAL(20,8),
    bybit_entry_price   DECIMAL(20,8),
    upbit_order_id      VARCHAR(100),
    bybit_order_id      VARCHAR(100),
    entry_spread_pct    DOUBLE,
    entry_z_score       DOUBLE,
    entry_usd_krw       DOUBLE,
    opened_at           DATETIME(3),
    closed_at           DATETIME(3),
    realized_pnl        DECIMAL(20,8),
    -- PartiallyClosedOneLeg 상세
    succeeded_leg       VARCHAR(10),       -- 'upbit' or 'bybit'
    emergency_attempts  INT DEFAULT 0,
    created_at          DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at          DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    INDEX idx_session_state (session_id, state),
    INDEX idx_coin_state (coin, state)
);

-- 체결 기록 (기존 trades.csv 대체)
CREATE TABLE trades (
    id                  BIGINT AUTO_INCREMENT PRIMARY KEY,
    session_id          BIGINT NOT NULL,
    position_id         BIGINT NOT NULL,
    coin                VARCHAR(20) NOT NULL,
    side                VARCHAR(10) NOT NULL,  -- entry, exit, emergency_close, adjustment
    qty                 DECIMAL(20,8) NOT NULL,
    upbit_price_krw     DECIMAL(20,4),
    bybit_price_usdt    DECIMAL(20,8),
    upbit_fee           DECIMAL(20,8),
    bybit_fee           DECIMAL(20,8),
    spread_pct          DOUBLE,
    z_score             DOUBLE,
    realized_pnl        DECIMAL(20,8),
    adjustment_cost     DECIMAL(20,8),
    executed_at         DATETIME(3) NOT NULL,
    INDEX idx_session (session_id),
    INDEX idx_position (position_id)
);

-- 분봉 스프레드 데이터 (기존 minutes.csv 대체)
CREATE TABLE minutes (
    id              BIGINT AUTO_INCREMENT PRIMARY KEY,
    session_id      BIGINT NOT NULL,
    coin            VARCHAR(20) NOT NULL,
    ts              DATETIME(3) NOT NULL,
    upbit_close     DECIMAL(20,4),
    bybit_close     DECIMAL(20,8),
    spread_pct      DOUBLE,
    z_score         DOUBLE,
    mean            DOUBLE,
    stddev          DOUBLE,
    INDEX idx_session_coin_ts (session_id, coin, ts)
);

-- 알림 기록 (기존 alerts.log 대체)
CREATE TABLE alerts (
    id              BIGINT AUTO_INCREMENT PRIMARY KEY,
    session_id      BIGINT NOT NULL,
    level           VARCHAR(20) NOT NULL,  -- info, warn, critical
    event_type      VARCHAR(50) NOT NULL,
    message         TEXT NOT NULL,
    payload_json    TEXT,
    created_at      DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    INDEX idx_session_level (session_id, level)
);

-- 종목별 펀딩 스케줄
CREATE TABLE funding_schedules (
    id                  BIGINT AUTO_INCREMENT PRIMARY KEY,
    coin                VARCHAR(20) NOT NULL,
    interval_hours      INT NOT NULL,           -- 4, 8 등 (종목별 상이)
    next_funding_time   DATETIME(3) NOT NULL,
    current_rate        DOUBLE NOT NULL,
    updated_at          DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    UNIQUE INDEX idx_coin (coin)
);
```

**DB 마이그레이션 시스템**:

마이그레이션 파일은 `crates/arb-db/migrations/` 디렉토리에 버전별로 관리한다.

- **파일 네이밍**: `V{version}__{설명}.sql` (예: `V001__create_sessions.sql`)
- **마이그레이션 상태 테이블**:
  ```sql
  CREATE TABLE IF NOT EXISTS _migrations (
      version     INT PRIMARY KEY,
      name        VARCHAR(255) NOT NULL,
      applied_at  DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
  );
  ```
- **마이그레이션 실행**: `examples/migrate.rs` 바이너리
  1. DB 연결 (`DATABASE_URL` 환경변수)
  2. `_migrations` 테이블 존재 확인 (CREATE IF NOT EXISTS)
  3. `SELECT version FROM _migrations ORDER BY version` -> 적용된 버전 목록
  4. `migrations/` 디렉토리에서 `V{version}__{name}.sql` 파일 스캔
  5. 미적용 버전 필터링 + 버전 오름차순 정렬
  6. 각 파일: `BEGIN` -> SQL 실행 -> `INSERT INTO _migrations (version, name)` -> `COMMIT`
  7. 결과 출력: `"Applied V001__create_sessions.sql (2ms)"` 등
  8. 실패 시 해당 트랜잭션 ROLLBACK + 에러 출력 후 중단 (부분 적용 방지)

- **마이그레이션 파일 목록**:

| 파일 | 설명 |
|------|------|
| `V001__create_sessions.sql` | sessions 테이블 |
| `V002__create_positions.sql` | positions 테이블 + 인덱스 |
| `V003__create_trades.sql` | trades 테이블 + 인덱스 |
| `V004__create_minutes.sql` | minutes 테이블 + 인덱스 |
| `V005__create_alerts.sql` | alerts 테이블 + 인덱스 |
| `V006__create_funding_schedules.sql` | funding_schedules 테이블 |

**PositionStore trait** (DB 구현체):
```rust
#[async_trait]
pub trait PositionStore: Send + Sync {
    /// 포지션 INSERT (Opening 상태).
    async fn save(&self, pos: &PositionRecord) -> Result<i64, DbError>;

    /// 포지션 상태 전이 (낙관적 잠금: WHERE state = expected_state).
    async fn update_state(&self, id: i64, from: &str, to: &str, fields: UpdateFields) -> Result<bool, DbError>;

    /// 특정 세션의 non-Closed 포지션 조회 (crash recovery용).
    async fn load_open(&self, session_id: i64) -> Result<Vec<PositionRecord>, DbError>;

    /// 포지션 삭제 (Opening 미발주 건).
    async fn remove(&self, id: i64) -> Result<(), DbError>;
}
```

**상태 전이 -- DB 트랜잭션**:
- WAL 패턴 대체: `BEGIN` -> `UPDATE positions SET state = 'Open' WHERE id = ? AND state = 'Opening'` -> `COMMIT`
- 낙관적 잠금: `WHERE state = ?` 조건으로 동시 전이 방지. affected_rows == 0이면 이미 다른 경로에서 처리됨.
- `ROLLBACK`: 트랜잭션 중 에러 시 자동 롤백 (sqlx 기본 동작)

**Crash recovery**:
- "Opening" (order_id 없음): 주문 발주 전 크래시 -> DB에서 DELETE
- "Opening" (order_id 있음): `get_order()` 조회 -> 체결되었으면 "Open" UPDATE, 미체결이면 cancel + DELETE
- "Closing" (order_id 있음): `get_order()` 조회 -> Filled이면 "Closed" UPDATE, 아니면 청산 재시도
- "PartiallyClosedOneLeg": succeeded_leg/order_id 기반 비상 청산 재시도

**DB 장애 처리**:
- **시작 시 연결 실패**: 프로세스 시작 차단 (DB 없이 실거래 불가)
- **운영 중 연결 끊김**: 신규 진입 즉시 차단 + 재연결 시도 (지수 백오프 1s~30s)
- **5분 연속 실패**: kill switch 강제 발동 -> 활성 포지션 전체 청산 + 텔레그램 알림
- sqlx 커넥션 풀 health check: `minute_timer`에서 `pool.acquire()` 확인

**output/writer.rs, output/summary.rs 전환**:
- `SessionWriter` -> `DbSessionWriter`: trades/minutes INSERT로 대체 (CSV append 제거)
- `SessionSummary` -> DB 쿼리로 생성: `SELECT COUNT(*), SUM(realized_pnl), ... FROM trades WHERE session_id = ?`
- temp file atomic write 패턴 제거
- JSON 파싱 에러 처리 불필요

**AlertService fallback**: 텔레그램 실패 시 `alerts` 테이블에 INSERT (기존 alerts.log 파일 대체)

**파일**:
- `crates/arb-db/Cargo.toml` (신규)
- `crates/arb-db/src/lib.rs` (신규)
- `crates/arb-db/src/pool.rs` (신규)
- `crates/arb-db/src/positions.rs` (신규)
- `crates/arb-db/src/trades.rs` (신규)
- `crates/arb-db/src/minutes.rs` (신규)
- `crates/arb-db/src/alerts.rs` (신규)
- `crates/arb-db/src/funding.rs` (신규)
- `crates/arb-db/src/sessions.rs` (신규)
- `crates/arb-db/migrations/` (신규)
- `Cargo.toml` (workspace members에 `arb-db` 추가)

**규모**: XL

#### 1-1. unwrap 패닉 제거

**문제**: `counters.lock().unwrap()` 30+곳, `ComputingFlags` 4곳에서 Mutex poisoning 시 패닉 -> 실거래 중 크래시

**변경**:
- `std::sync::Mutex` -> `parking_lot::Mutex` 전환 (poisoning 없음, 성능 우수)
  - 대상: `MonitoringCounters`, `ComputingFlags`, `coin_selector.rs` 내 `candles` Mutex (373행)
  - **추가**: `instrument_cache` (`std::sync::RwLock`) -> `parking_lot::RwLock` 전환 (`unwrap_or_else(|e| e.into_inner())` 4곳 제거)
  - **주의**: `tokio::sync::Mutex` (position_mgr, trades, session_writer)와 `tokio::sync::RwLock` (spread_calc, ob_cache.data)은 **전환하지 않음** (.await를 넘나드는 lock hold가 있으므로)
- `unwrap_or_else(|e| e.into_inner())` 패턴은 **사용하지 않음** (poisoned 상태의 데이터가 inconsistent할 수 있으므로)
- `SystemTime::now().duration_since(UNIX_EPOCH).expect()` -> `.unwrap_or_default()` + warn 로그
  - **대상**: `bybit/auth.rs` 83행 + `bithumb/auth.rs` 47행 (둘 다 동일 패턴)
  - **연쇄 효과**: timestamp 0 시 Bybit/Bithumb 서명 거부 -> 해당 주문 차단 (정상 fallback)
- `.expect()` 제거 대상: `coin_selector.rs` 7곳, `spread.rs` 1곳 -> **테스트 코드(`#[cfg(test)]`) 한정이므로 프로덕션 위험 없음. 체크리스트에서 "테스트 코드" 명시**

**파일**: `monitor.rs`, `orderbook.rs`, `coin_selector.rs`, `spread.rs`, `bybit/auth.rs`, `bithumb/auth.rs`
**규모**: M

#### 1-2. 포지션 영속화 (PositionStore -- DB 기반)

**문제**: `PositionManager`가 순수 인메모리 -> crash 시 열린 포지션 정보 소실

**Dual-State 설계**:
- **실시간 상태**: `PositionManager` (tokio::sync::Mutex) -- 모든 진입/청산 판단, 잠금, 상태 전이의 단일 진실 소스
- **영속 상태**: `DbPositionStore` (arb-db) -- crash recovery 전용, 메모리 상태를 비동기로 shadow
- 의사결정은 **절대 DB를 참조하지 않음**. DB가 느려도/끊겨도 실시간 전략 판단에 영향 없음.
- 시작 시: DB에서 non-Closed 포지션 로드 -> 메모리에 복원 -> 이후 메모리만 참조

**DB 기록 시점** (hot path 외부):
- **동기 DB write (필수)**: order_id 기록 직후 (주문 발주 후, 체결 대기 전). crash 시 주문 추적에 필수.
- **비동기 DB write**: Opening 등록, Open 전이, Closing 전이, Closed 전이. 메모리 갱신 후 fire-and-forget.
- **DB 장애 시**: 비동기 write 실패 -> warn 로그 + 재시도 큐. 동기 write 실패 -> 주문은 이미 나간 상태이므로 메모리에는 반영하되, DB 복구 시 reconciliation으로 보정.

**변경**:
- `position_store.rs` 신규 생성
- `PositionStore` trait 정의 (Phase 1-0 참조)
- 구현체: `DbPositionStore` (arb-db의 `positions` 테이블 사용)

**포지션 상태 머신**:
```
Opening -> Open -> Closing -> Closed
           |
           └─-> PartiallyClosedOneLeg (비상 상태)
```

**상태 전이 세분화** (메모리 우선 + DB shadow):
1. **메모리**: `pm.register_opening(pos)` (즉시) -> **DB 비동기**: `INSERT positions (state='Opening', coin, qty, expected_prices)`
2. **주문 발주 후** -> **DB 동기**: `UPDATE positions SET upbit_order_id=?, bybit_order_id=? WHERE id=? AND state='Opening'` (★ 유일한 동기 DB write)
3. **체결 확인** -> **메모리**: `pm.transition_to_open(pos)` (즉시) -> **DB 비동기**: `UPDATE positions SET state='Open', qty=?, actual_prices=?`

-> 2단계 DB 동기 write 이후 크래시 시 order_id로 `get_order()` 조회 가능
-> 1단계에서 크래시 시 DB에 레코드 없을 수 있음 -> 주문도 미발주이므로 안전

**PartiallyClosedOneLeg 상태 상세**:
```rust
// positions 테이블의 추가 컬럼으로 저장
succeeded_leg: String,           // "upbit" or "bybit"
succeeded_order_id: String,      // (upbit_order_id 또는 bybit_order_id)
emergency_attempts: u32,
```
- 복구 경로: `get_order(succeeded_order_id)` -> 체결 수량 확인 -> 반대 레그 비상 청산 재시도
- 5분 초과 시 -> kill switch
- emergency_close 성공 -> `Closed` 전이

**Crash recovery** (DB 기반):
- `SELECT * FROM positions WHERE session_id=? AND state NOT IN ('Closed')` -> non-Closed 포지션 조회
- "Opening" (order_id NULL): 주문 발주 전 크래시 -> DELETE
- "Opening" (order_id 있음): `get_order()` 조회 -> 체결이면 "Open" UPDATE, 미체결이면 cancel + DELETE
- "Closing" (order_id 있음): `get_order()` 조회 -> Filled이면 "Closed" UPDATE, 아니면 청산 재시도
- "PartiallyClosedOneLeg": succeeded_leg 기반 비상 청산 재시도

**`VirtualPosition`에 추가 필드**:
- `#[derive(Serialize, Deserialize)]` 추가 (`serde`는 이미 dependencies에 존재)
- `upbit_order_id: Option<String>`, `bybit_order_id: Option<String>`
- `state: PositionState` (Opening, Open, Closing, Closed, PartiallyClosedOneLeg)
- `db_id: Option<i64>` (DB primary key 참조)
- **호환성 영향**: 모든 `VirtualPosition` 구조체 리터럴 생성 지점(monitor.rs, 테스트 다수)에 새 필드 추가 필요. 빌더 패턴 또는 `..Default::default()` 적용으로 노동량 최소화.

**load 실패 시**: DB 조회 실패 -> **order_id 기반 reconciliation 강제 실행** -> 거래소 실주문/포지션 기반 복원. 복원 불가 시 신규 진입 차단 + 알림.

**파일**: `position_store.rs` (신규), `position.rs`, `monitor.rs`
**규모**: L

#### 1-3. Kill Switch + Risk Manager

**변경**:
- `risk.rs` 신규 생성
- `RiskManager` 구조체:
  ```rust
  pub struct RiskManager {
      config: RiskConfig,
      inner: parking_lot::Mutex<RiskState>,  // lock order: position_mgr 해제 후
      is_killed: AtomicBool,                 // lock 없이 원자적 확인/설정
  }

  struct RiskState {
      daily_realized_pnl: Decimal,
      peak_equity: Decimal,
      current_equity: Decimal,
      last_reset: DateTime<Utc>,
  }
  ```
- **AtomicBool**: `spawned_check_tick_signal`과 이벤트 루프에서 동시 접근 가능 -> lock 없이 `Ordering::Acquire/Release`로 확인/설정
- **일일 리셋**: KST 00:00 (UTC 15:00) 기준으로 고정 (한국 시간 기준 거래일 경계)

- 메서드:
  - `is_killed(&self) -> bool`: AtomicBool 확인 (lock 불필요)
  - `is_entry_allowed(&self) -> bool`: kill switch + 연결 상태 + 진행 중 청산 확인
  - `record_trade(&self, pnl: Decimal)`: 누적 PnL 기록, kill switch 조건 체크
  - `trigger_kill_switch(&self, reason: &str)`: 강제 발동
  - `check_connection_health(&self, upbit_ok, bybit_ok)`: 한쪽 연결 불안정 시 진입 차단
  - `validate_order_size(&self, size_usdt: Decimal) -> bool`: 단일 주문 크기 상한 확인

- **리스크 한도 -- 자본 대비 비율(%) + 절대값 이중 적용**:
  ```toml
  # 비율 기반 (자본 규모에 자동 연동)
  max_daily_loss_pct = 10.0        # 자본의 10%
  max_drawdown_pct = 5.0           # 자본의 5% (daily_loss보다 작아야 함)
  max_single_loss_pct = 3.0        # 단건 자본의 3%

  # 절대값 기반 (안전망)
  max_daily_loss_usdt = 50.0       # 소액 단계: $50
  max_drawdown_usdt = 25.0         # 소액 단계: $25
  max_single_loss_usdt = 15.0      # 단건 최대 $15
  max_order_size_usdt = 2000.0     # ★ 단일 주문 크기 상한 (버그 방어)
  max_concurrent_positions = 5     # 소액 단계: 5개 (10은 $300에서 min_notional에 걸림)

  # 실효 한도 = min(비율 기반, 절대값 기반)
  ```

- **Lock order**: `RiskManager.inner`는 `position_mgr` lock 해제 후 호출.
  ```
  ob_cache -> instrument_cache -> position_mgr -> balance_tracker -> risk_manager.inner -> counters -> spread_calc
  ```
  (`is_killed` AtomicBool은 lock order 무관)

- **Kill switch 발동 시 정리 과정**:
  1. `is_killed.store(true, Release)` -- 즉시 신규 진입 차단
  2. 이벤트 루프의 정상 청산 로직 비활성화 (kill switch 전용 청산만 동작)
  3. 활성 포지션 **notional 내림차순** 순차 청산 (rate limit 준수)
  4. 각 포지션: "Closing" 전이 -> 청산 주문 -> 결과 DB 즉시 반영
  5. 전체 완료 조건: `open_count() == 0` AND 거래소 reconciliation 통과
  6. "KILL SWITCH COMPLETE" 텔레그램 발송
  7. **해제**: 수동 확인 + 프로세스 재시작으로만 가능 (자동 해제 없음)
  8. kill switch 청산은 **별도 task로 spawn** (deadlock 방지)
  9. kill switch 청산 task는 포지션 0 될 때까지 재스캔 (진행 중 spawn된 진입 방치 방지)

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
  - `BalanceInsufficient { exchange, required, available }`
  - `DbConnectionLost { retry_count }`
  - `FundingBlockEntry { coin, rate, direction }`
  - `Error { message }`
  - `DailySummary { trades, pnl, win_rate }`
- **일반 알림**: `mpsc::channel<AlertEvent>` bounded(64) + `try_send()` + warn (이벤트 루프 블로킹 방지)
- **치명적 알림** (`KillSwitchTriggered`, `EmergencyCloseFailure`): **동기적 전송** -- `send().await`로 전송 완료 확인 후 후속 처리. ★ 이 알림은 반드시 **spawn된 task 내에서만** 발생 (kill switch 청산 task, LiveExecutor 비상 청산). select! 루프에서 직접 동기 전송하면 이벤트 루프 블로킹
- **텔레그램 실패 fallback**: `alerts` 테이블에 INSERT (DB 기반, 기존 파일 fallback 대체)

**파일**: `alert.rs` (신규), `monitor.rs`, `config.rs`
**의존**: `arb-telegram`, `arb-db`
**규모**: M

#### 1-5. output/writer.rs, output/summary.rs DB 전환

**문제**: 기존 CSV/JSON 파일 I/O를 DB로 전환.

**변경**:
- `SessionWriter` -> `DbSessionWriter`:
  - `record_trade()`: `INSERT INTO trades (...) VALUES (...)` (CSV append 대체)
  - `record_minute()`: `INSERT INTO minutes (...) VALUES (...)` (CSV append 대체)
  - 세션 시작: `INSERT INTO sessions (started_at, config_json, status) VALUES (?, ?, 'Running')`
  - 세션 종료: `UPDATE sessions SET ended_at=?, status='Completed' WHERE id=?`
- `SessionSummary`: DB 쿼리로 생성
  - `SELECT COUNT(*) as trade_count, SUM(realized_pnl) as total_pnl, ... FROM trades WHERE session_id = ?`
  - `SELECT COUNT(*) FILTER (WHERE realized_pnl > 0) as win_count, ...` (또는 CASE WHEN)
  - max drawdown, profit factor 등 기존 계산 로직은 Rust 측에서 유지 (복잡한 쿼리 회피)
- 기존 파일 관련 코드 제거:
  - `File`, `OpenOptions`, `BufWriter` import 제거
  - temp file atomic write 패턴 제거
  - `OutputConfig.dir` 필드 제거 -> `OutputConfig.db_url` 또는 DB 커넥션 주입
  - JSON 파싱 에러 처리 불필요

**파일**: `output/writer.rs`, `output/summary.rs`
**규모**: L

#### 1-6. 에러 분류 강화

**변경**:
- `ExchangeError`에 `is_retryable(&self) -> bool` 메서드 추가:
  - `RateLimitExceeded`, `HttpError(timeout)` -> `true`
  - `AuthError`, `InsufficientFunds`, `InvalidParameter` -> `false`
- `spawned_check_tick_signal`의 `Box<dyn Error>` -> `StrategyError` 전환
- **`EmergencyCloseFailed` 에러 시**: warn 로그가 아닌 **RiskManager에 직접 kill switch 발동 요청** (silent failure 방지). `Arc<RiskManager>`를 spawn된 task에 전달.
- 가격 `Decimal::ZERO` fallback 시 즉시 return 가드 추가

**파일**: `error.rs` (arb-exchange), `monitor.rs`
**규모**: S

#### 1-7. WebSocket 자동 재연결 (**Phase 1 승격**)

**문제**: WebSocket 끊김 시 `select!` 루프 종료 -> 활성 포지션이 있는 상태에서 청산 시그널을 놓침

**변경**:
- WebSocket 끊김 감지 시:
  1. 활성 포지션이 있으면 -> RiskManager에 연결 상태 보고 -> 신규 진입 차단 (메모리, 즉시)
  2. **별도 tokio::spawn으로 재연결 task 시작** (★ select! 루프에서 직접 retry 하면 안 됨)
  3. 재연결 task 내에서 지수 백오프 retry (1s, 2s, 4s, 8s, max 30s)
  4. 재연결 성공 -> 새 rx 채널을 select! 루프에 전달 (mpsc 또는 채널 교체) + 재구독
  5. 재연결 5분 실패 -> kill switch 발동 -> 활성 포지션 전체 시장가 청산 + 텔레그램 알림
- select! 루프는 재연결 중에도 **나머지 거래소 이벤트 + minute_timer + reselect를 계속 처리**
- `_forex_task` JoinHandle 모니터링 + 패닉 시 재시작 (마찬가지로 spawn)

**파일**: `monitor.rs`, `stream.rs` (arb-exchange)
**규모**: M

#### 1-8. 잔고 관리 (BalanceTracker)

**문제**: 시뮬레이션은 자본 무제한이지만, 라이브에서는 거래소별 가용 잔고 추적 + 동시 진입 시 경합 방지가 필요.

**변경**:
- `balance.rs` 신규 생성
- `BalanceTracker` 구조체 (설계 섹션 참조)
- **시작 시 초기화**: `upbit.get_balance("KRW")` + `bybit.get_balance("USDT")` -> `available_capital = min(upbit_krw / usd_krw, bybit_usdt)`
  - `config.total_capital_usdt`와 실잔고 비교 -> 부족 시 warn + 실잔고 기준으로 조정
- **자본 예약 패턴**: `reserve()` -> `execute_entry()` -> `commit()` / `release()`
  - 동시 2건 시그널: 첫 번째가 `reserve()`로 잔고 차감 -> 두 번째는 남은 잔고만 사용 가능
  - 예약 실패 시: `BalanceInsufficient` 알림 + 카운터 증가
- **잔고 동기화**: `minute_timer`에서 매 5분 **tokio::spawn**으로 `sync_from_exchange()` 실행 (★ REST 호출이므로 select! 내 직접 실행 금지) -- 내부 잔고와 실잔고 비교, 10% 이상 괴리 시 warn + 실잔고로 보정
- **Bybit 가용 증거금 계산**: `wallet_balance - sum(각 포지션 initial_margin + maintenance_margin)`
  - 1x 레버리지이므로 initial_margin ~= position_value
- **시뮬레이션 example**: BalanceTracker를 사용하지 않음 (기존 무제한 자본 동작 유지)
- **Upbit 수수료 차감 주의**: Upbit은 코인 수수료를 수량에서 차감 -> 실 수령 수량 = `executed_volume * (1 - fee_rate)`. KRW 잔고 차감은 주문 금액 기준.

**파일**: `balance.rs` (신규), `monitor.rs`, `config.rs`
**규모**: M

#### 1-9. 환율 staleness guard (**Phase 4에서 승격**)

**문제**: 차익거래 스프레드가 0.3~0.5% 수준인데, 환율 변동 0.5%로 수익 구간 소멸 가능. 30분 staleness는 실거래에서 위험.

**변경**:
- 환율 캐시 age > **10분**이면 신규 진입 차단 (30분에서 단축)
- 환율 급변 감지: 전분 대비 **0.2%** 이상 변동 시 경고 알림 (0.5%에서 하향)
- `_forex_task` 패닉 시 즉시 환율 staleness 상태 진입 -> 진입 차단

**파일**: `monitor.rs`
**규모**: S

---

### Phase 2: 주문 실행 엔진

#### 2-1. Bybit 선물 API 확장 (**Phase 2-2보다 선행**)

**문제**: Bybit SDK의 `DEFAULT_CATEGORY = "spot"` 하드코딩. 선물 short 필요.

**변경**:
- `BybitClient`에 전용 메서드 추가 (추상화 위반 방지):
  - `place_order_linear(&self, request)`: category="linear" 고정
  - `get_order_linear(&self, order_id)`, `cancel_order_linear(&self, order_id)`
  - 기존 `place_order()` (spot)은 변경 없음
- 선물 전용 API:
  - `set_leverage(symbol, leverage)`: 레버리지 설정
  - `switch_margin_mode(symbol, mode)`: Isolated/Cross 전환
    - **실패 시**: 현재 모드 확인 -> 이미 Isolated이면 정상 진행, Cross이면 시작 차단 + 에러
  - `get_positions(symbol)`: 실 포지션 조회 -> `PositionInfo` 반환
- `PositionInfo` 타입 추가 (types.rs):
  ```rust
  pub struct PositionInfo {
      pub symbol: String,
      pub side: String,           // "Buy" or "Sell"
      pub size: Decimal,
      pub entry_price: Decimal,
      pub leverage: Decimal,
      pub unrealised_pnl: Decimal,
      pub liq_price: Decimal,
  }
  ```

**파일**: `bybit/client.rs`, `bybit/types.rs`, `arb-exchange/types.rs`
**규모**: M

#### 2-2. Rate Limiter 분리 (Bybit) (**Phase 1과 병행 권장**)

**문제**: Bybit 단일 limiter(18 req/sec)를 public/private API가 공유 -> kill switch 청산 시 오더북 조회와 경합

**변경**:
- `BybitClient`: `public_limiter` (10 req/sec) + `private_limiter` (10 req/sec) 분리
- 오더북 조회 -> `public_limiter`, 주문/잔고/포지션 -> `private_limiter`

**파일**: `bybit/client.rs`
**규모**: S

#### 2-3. LiveExecutor 구현 (구체 타입, trait 없음)

**설계 결정**: `OrderExecutor` trait + `SimExecutor` 방식을 사용하지 않는다. 시뮬레이션은 별도 example 바이너리에서 기존 가상 체결 코드를 그대로 사용하고, 라이브는 `src/main.rs`에서 `LiveExecutor<U, B>`를 구체 타입으로 직접 사용한다. 동적 디스패치(`Arc<dyn OrderExecutor>`) 없이 컴파일 타임 모노모피즘으로 hot path 성능을 최적화한다.

**변경**:
- `live_executor.rs` 신규 생성
- `LiveExecutor<U: OrderManagement, B: OrderManagement>` 제네릭 구조체
- `ZScoreMonitor<U, B>`에서 `LiveExecutor<U, B>` 필드로 직접 보유 (trait object 아님)

- **요청/응답 타입**:
  ```rust
  pub struct EntryRequest {
      pub coin: String,
      pub qty: Decimal,
      pub upbit_krw_price: Decimal,
      pub bybit_usdt_price: Decimal,
      pub usd_krw: f64,
      pub instrument_info: InstrumentInfo,
  }

  pub struct ExitRequest {
      pub coin: String,
      pub qty: Decimal,
      pub instrument_info: InstrumentInfo,
  }

  pub struct ExecutedEntry {
      pub upbit_order_id: String,
      pub bybit_order_id: String,
      pub upbit_filled_qty: Decimal,
      pub bybit_filled_qty: Decimal,
      pub upbit_avg_price_krw: Decimal,
      pub bybit_avg_price: Decimal,
      pub upbit_fee: Decimal,
      pub bybit_fee: Decimal,
      pub effective_qty: Decimal,
      pub adjustment_cost: Decimal,  // ★ 초과분 청산 비용
  }
  ```

- **OrderExecutionError** 에러 타입:
  ```rust
  pub enum OrderExecutionError {
      BothUnfilled,
      SingleLegFilled { leg: Leg, emergency_closed: bool },
      EmergencyCloseFailed { leg: Leg, order_id: String },
      QtyMismatch { upbit_qty: Decimal, bybit_qty: Decimal },
      InsufficientBalance { exchange: String, available: Decimal },
      ExchangeError(ExchangeError),
      Timeout {
          leg: Option<Leg>,
          pending_order_id: Option<String>,
          other_leg_status: Option<LegStatus>,
      },
  }
  ```
- **진입 실행 흐름**:
  1. `RiskManager.validate_order_size(size_usdt)` -- 단일 주문 크기 상한 확인
  2. Upbit 주문 준비:
     - **IOC 지정가 (기본)**: `price = upbit_krw_price * (1 + max_slippage_pct)`
     - **Upbit IOC 미지원 시 fallback**: GTC 지정가 + `tokio::time::timeout(3s)` 후 cancel
     - **시장가 (config fallback)**: `krw_amount = qty * upbit_krw_price * (1 + upbit_fee_rate)`
  3. Bybit 주문 준비:
     - **IOC 지정가**: `price = bybit_usdt_price * (1 - max_slippage_pct)` (매도 하한)
  4. Client Order ID 생성: `format!("{coin}_{timestamp}_{seq}")`
  5. 양 레그 동시 발주:
     ```rust
     let result = tokio::time::timeout(Duration::from_secs(10),
         tokio::join!(
             tokio::time::timeout(Duration::from_secs(5), upbit.place_order(buy)),
             tokio::time::timeout(Duration::from_secs(5), bybit.place_order(sell)),
         )
     ).await;
     ```
  6. 체결 대기: `poll_until_filled(order_id, timeout=5s)` -- polling 간격 200ms->500ms->1s->2s
  7. 결과 처리:
     - 양쪽 Filled -> `effective_qty = min(upbit_filled, bybit_filled)`, 초과분 청산
       - 초과분 청산 비용 -> `adjustment_cost`에 기록 -> 포지션 effective entry price에 반영
       - **Partial fill 초과분 청산 재시도 상한**: 최대 3회, 수렴하지 않으면 잔여 수량 수동 처리 + kill switch
     - 한쪽 Filled + 한쪽 미체결 -> 미체결 쪽 cancel -> 체결된 쪽 비상 청산
     - 양쪽 미체결 -> 양쪽 cancel, 진입 포기
  8. **Post-execution PnL gate**: 실체결가 기반 스프레드 확인, 수익 구간 밖이면 즉시 청산
  9. 결과 반환: `ExecutedEntry`

- **비상 청산 3단계 escalation**:
  1. **0~2분**: IOC 지정가 재시도 (지수 백오프 1s, 2s, 4s, 8s...)
  2. **2~5분**: **시장가로 전환** + 텔레그램 알림 (중간 단계 추가)
  3. **5분 초과**: **kill switch 강제 발동** + `EmergencyCloseFailure` 알림
  4. 비상 청산 손실은 `RiskManager.record_trade()`에 반드시 포함

- **Cancel 실패 처리**: cancel 실패 -> `get_order()` 재확인 -> PartiallyFilled면 실체결 수량으로 effective_qty 조정

- **Computing flag lifetime 확장**: LiveExecutor 사용 시 computing flag를 주문 완료(체결/실패 확정)까지 유지. 주문 진행 중 같은 코인에 새 시그널 spawn 방지.

- **비정상 거래 기록**: `ExecutedEntry`의 `adjustment_cost > 0`이거나 한쪽만 체결 후 비상 청산 시, `trades` 테이블에 `side='adjustment'`로 기록 (정상 포지션과 구분).

**파일**: `live_executor.rs` (신규)
**규모**: XL

---

### Phase 3: monitor.rs 통합

#### 3-1. LiveExecutor 통합

**변경**:
- `ZScoreMonitor<U, B>`에 `LiveExecutor<U, B>` 필드 추가 (구체 타입, trait object 아님)
- `src/main.rs`에서 `LiveExecutor::new(upbit.clone(), bybit.clone())` 생성 후 ZScoreMonitor에 전달
- 기존 가상 체결 코드 블록을 `live_executor.execute_entry()` / `execute_exit()` 호출로 교체
- **시뮬레이션 example**: 기존 monitor.rs의 가상 체결 코드를 example에서 그대로 사용 (LiveExecutor 불필요)

**진입 시 TOCTOU 해결** (Kill Switch 동시성 설계 참조):
```
[★ 전체가 spawned_check_tick_signal 내에서 실행. select! 루프 비블로킹]
1. risk_manager.is_entry_allowed()           // AtomicBool (lock 없이)
2. balance_tracker.reserve(upbit_krw, bybit_usdt)  // parking_lot::Mutex (< 1μs)
3. position_mgr.lock().await                 // tokio::sync::Mutex
4.   risk_manager.is_killed() 이중 체크       // AtomicBool
5.   pm.register_opening(pos)                // 메모리 Opening 등록
6. position_mgr.unlock()
7. live_executor.execute_entry()            // ★ REST 호출 (spawn 내이므로 select! 비블로킹)
8. position_mgr.lock().await -> 메모리 결과 반영 + DB 비동기 반영
```

**청산 발생 지점 3곳** (전부 select! 루프 외부에서 실행):
1. 정상 Z-Score 시그널 청산 → `spawned_check_tick_signal` 내 (이미 spawn됨)
2. TTL 만료 청산 → **tokio::spawn으로 분리** (★ live_executor.execute_exit()가 REST 호출이므로 minute_timer에서 직접 실행 금지)
3. Kill switch 강제 청산 → 별도 spawn task (이미 명시)
- kill switch 발동 시: 1, 2번 경로 비활성화, 3번만 동작 (이중 청산 방지)
- PositionState "Closing" 전이로 중복 청산 방지

**파일**: `monitor.rs`
**규모**: L

#### 3-2. 시작 시 초기화 강화

**변경**:
- (Live 모드) Bybit 선물 설정 검증:
  - `set_leverage(1x)`: 실패 시 현재 레버리지 확인, 1x가 아니면 시작 차단
  - `switch_margin_mode("Isolated")`: 실패 시 현재 모드 확인, 이미 Isolated이면 정상, Cross이면 시작 차단
- (Live 모드) BalanceTracker 초기화:
  - `upbit_krw = upbit.get_balance("KRW")`
  - `bybit_usdt = bybit.get_balance("USDT")`
  - `available_capital = min(upbit_krw / usd_krw, bybit_usdt)`
  - 잔고 부족 시 경고 + `config.total_capital_usdt`를 실잔고 기준으로 조정
- (Live 모드) DB 연결 확인: `pool.acquire()` 성공 확인, 실패 시 시작 차단
- (Live 모드) Upbit IOC 지원 여부 사전 검증: 테스트 주문(최소 금액) -> IOC 지원 확인
- 미청산 포지션 복원: `PositionStore::load_open(session_id)`
  - DB 조회 실패 시 -> reconciliation 강제 실행
  - "Opening"/"Closing" 상태 포지션 -> order_id 기반 `get_order()` 조회 후 복구
- NTP 시간 동기화 확인: Bybit 서버 시간과 로컬 시간 차이 > 5초 -> warn + 진행

**파일**: `monitor.rs`
**규모**: M

#### 3-3. 포지션 Reconciliation

**변경**:
- `minute_timer`에서 **tokio::spawn으로 reconciliation task 실행** (★ REST 호출 다수이므로 select! 내 직접 실행 금지)
- 결과는 mpsc 채널 또는 Arc<Mutex> 공유 상태로 반환 -> select! 루프에서 결과 수신 후 상태 갱신
- **Bybit**: `get_positions()` -> 내부 `PositionManager` 상태와 비교
- **Upbit**: **order_id 기반** `get_order()` 조회 -> 해당 주문의 체결 수량 확인
  - (잔고 비교는 참고 수준 -- 사용자 개인 보유 코인과 구분 불가)
- 불일치 시:
  - warn 로그 + 텔레그램 `ReconciliationMismatch` 알림
  - **신규 진입 차단** (불일치 해소까지)
  - **자동 수정 하지 않음** -- 불일치 원인이 다양(사용자 수동 거래, 이중 체결, 비상 청산 부분 체결)하므로 자동 수정은 위험. "차단 + 알림 + 수동 확인" 전략.

**파일**: `monitor.rs`
**규모**: M

#### 3-4. 종목별 펀딩비 모니터링

**문제**: 기존 스펙은 8시간 고정 주기로 가정했으나, 실제로 Bybit 종목별 펀딩 정산 주기가 상이 (4h, 8h 등). 종목별로 정산 시점과 현재 rate를 추적해야 한다.

**설계**:

**펀딩 스케줄 조회 (코인 선택 시)**:
- `coin_selector`에서 코인이 선택될 때 Bybit `getTickers` (category=linear) API로 해당 코인의 펀딩 정보 조회:
  - `fundingRate`: 현재 펀딩비율
  - `nextFundingTime`: 다음 정산 시각 (ms timestamp)
  - `fundingIntervalHour`: 펀딩 정산 주기 (시간 단위, 정수)
- `getInstrumentsInfo` (category=linear) API에서도 `fundingInterval` (분 단위) 조회 가능 (보조 검증용)

**`FundingSchedule` 구조체**:
```rust
pub struct FundingSchedule {
    pub coin: String,
    pub interval_hours: u32,           // 4, 8 등 (종목별 상이)
    pub next_funding_time: DateTime<Utc>,
    pub current_rate: f64,             // 양수: long 지불, 음수: short 지불
    pub updated_at: DateTime<Utc>,
}
```

**종목별 펀딩 캐시**:
- `HashMap<String, FundingSchedule>` -- 모니터링 중인 코인별 펀딩 스케줄
- **초기 로드**: `coin_selector`에서 코인 선택 직후, 선택된 코인들의 펀딩 스케줄 일괄 조회
- **매분 갱신**: `minute_timer`에서 **tokio::spawn으로 펀딩 갱신 task 실행** (★ REST 호출이므로 select! 내 직접 실행 금지). 모니터링 중인 전 코인의 `getTickers` -> `fundingRate`, `nextFundingTime` 업데이트. 결과는 공유 `FundingScheduleCache`(Arc)에 갱신
- **코인 재선택 시**: 새 코인 추가/교체 시 해당 코인 펀딩 스케줄 조회 + 캐시 갱신

**진입 차단 로직**:
- 정산 **30분 전**부터 펀딩비가 **불리한 방향**이면 해당 코인만 진입 차단
  - 불리한 방향: 우리 포지션은 Bybit short -> `fundingRate > 0`이면 short가 지불 -> 불리
  - 즉, `current_rate > 0 AND time_to_funding < 30min` -> 해당 코인 진입 차단
  - 반대로 `current_rate < 0`이면 short가 수취 -> 유리 -> 차단 안 함
- **차단 시**: `FundingBlockEntry` 알림 + 카운터 증가

**DB 연동**:
- `funding_schedules` 테이블에 종목별 스케줄 저장 (갱신 시 UPSERT)
- crash recovery 시 DB에서 마지막 스케줄 로드 (API 재조회 없이 즉시 사용)

**실 펀딩비 추적**:
- 정산 시점 전후로 Bybit `getClosedPnl` 또는 `getTransactionLog`에서 실 펀딩비 확인
- `RiskManager.record_trade()`에 펀딩비 포함
- 펀딩비가 포지션 수익 대비 과도(>50%)할 경우 경고 알림

**파일**: `monitor.rs`, `coin_selector.rs`, `bybit/client.rs`, `crates/arb-db/src/funding.rs`
**규모**: M

---

### Phase 4: 설정 및 부가 기능

#### 4-1. Config 확장

**추가 설정** (`strategy.toml`):
```toml
[zscore]
# 라이브 전용 설정 (src/main.rs에서만 사용, 시뮬 example은 기존 config 그대로)
bybit_category = "linear"        # 선물 카테고리

# 주문 실행
order_timeout_sec = 5             # 주문 체결 대기 타임아웃
max_retry_count = 2               # 재시도 횟수
order_type = "limit_ioc"          # "limit_ioc", "limit_gtc_cancel", "market"
max_slippage_pct = 0.1            # IOC/GTC 지정가 시 최대 슬리피지 %

# 리스크 관리 (비율 + 절대값 이중)
kill_switch_enabled = true
max_daily_loss_pct = 10.0
max_drawdown_pct = 5.0
max_single_loss_pct = 3.0
max_daily_loss_usdt = 50.0
max_drawdown_usdt = 25.0
max_single_loss_usdt = 15.0
max_order_size_usdt = 2000.0
max_concurrent_positions = 5

# 환율 (Phase 1-9에서 적용)
max_forex_age_min = 10            # 환율 캐시 최대 수명 (분)
forex_change_alert_pct = 0.2      # 환율 급변 알림 임계치 (%)

# 펀딩비
funding_block_minutes = 30        # 정산 N분 전부터 불리 시 진입 차단
funding_alert_ratio = 0.5         # 펀딩비 > 수익의 50%이면 경고

# DB
db_url = "mysql://user:pass@localhost:3306/arb_poc"

# 텔레그램 알림
telegram_enabled = true
telegram_chat_id = ""             # 환경변수 우선
```

**파일**: `config.rs`
**규모**: M

#### 4-2. 실체결 수수료 + 펀딩비 PnL 반영

**변경**:
- `ClosedPosition`에 `actual_upbit_fee`, `actual_bybit_fee`, `funding_fee`, `adjustment_cost` 필드 추가
- Live 모드: `Order.paid_fee` 기반 실 수수료 사용
- 시뮬레이션 example: 기존 config fee rate 기반 (변경 없음, 라이브 인프라 불필요)

**파일**: `pnl.rs`, `position.rs`, `bybit/client.rs`
**규모**: S

#### 4-3. API 키 보안 강화

**변경**:
- `BybitCredentials`, `UpbitCredentials`의 `#[derive(Debug)]` -> 수동 Debug impl (키 마스킹)
- 향후: `secrecy::SecretString` 전환

**파일**: `bybit/auth.rs`, `upbit/auth.rs`
**규모**: S

---

## 파일 변경 목록

### 신규 파일

| 파일 | Phase | 설명 |
|------|-------|------|
| `crates/arb-db/Cargo.toml` | 1-0 | arb-db workspace crate 설정 |
| `crates/arb-db/src/lib.rs` | 1-0 | DB 모듈 루트 + DbPool 래퍼 |
| `crates/arb-db/src/pool.rs` | 1-0 | 커넥션 풀 관리, health check |
| `crates/arb-db/src/positions.rs` | 1-0 | DbPositionStore 구현 |
| `crates/arb-db/src/trades.rs` | 1-0 | trades 테이블 Repository |
| `crates/arb-db/src/minutes.rs` | 1-0 | minutes 테이블 Repository |
| `crates/arb-db/src/alerts.rs` | 1-0 | alerts 테이블 Repository |
| `crates/arb-db/src/funding.rs` | 1-0 | funding_schedules 테이블 Repository |
| `crates/arb-db/src/sessions.rs` | 1-0 | sessions 테이블 Repository |
| `crates/arb-db/src/migration.rs` | 1-0 | 커스텀 마이그레이션 러너 |
| `crates/arb-db/migrations/V001__create_sessions.sql` | 1-0 | sessions 테이블 |
| `crates/arb-db/migrations/V002__create_positions.sql` | 1-0 | positions 테이블 + 인덱스 |
| `crates/arb-db/migrations/V003__create_trades.sql` | 1-0 | trades 테이블 + 인덱스 |
| `crates/arb-db/migrations/V004__create_minutes.sql` | 1-0 | minutes 테이블 + 인덱스 |
| `crates/arb-db/migrations/V005__create_alerts.sql` | 1-0 | alerts 테이블 + 인덱스 |
| `crates/arb-db/migrations/V006__create_funding_schedules.sql` | 1-0 | funding_schedules 테이블 |
| `examples/migrate.rs` | 1-0 | 마이그레이션 실행 바이너리 |
| `crates/arb-strategy/src/zscore/position_store.rs` | 1-2 | PositionStore trait + DbPositionStore 연동 |
| `crates/arb-strategy/src/zscore/risk.rs` | 1-3 | RiskManager + Kill Switch (AtomicBool) |
| `crates/arb-strategy/src/zscore/alert.rs` | 1-4 | AlertService (텔레그램 + DB fallback) |
| `crates/arb-strategy/src/zscore/balance.rs` | 1-8 | BalanceTracker (거래소별 잔고 관리) |
| `crates/arb-strategy/src/zscore/live_executor.rs` | 2-3 | LiveExecutor<U, B> 실주문 구현 (구체 타입, trait 없음) |
| `examples/zscore_sim.rs` | 3-1 | 시뮬레이션 example (기존 가상 체결 코드 분리) |

### 변경 파일

| 파일 | Phase | 규모 | 핵심 변경 |
|------|-------|------|----------|
| `Cargo.toml` (root) | 1-0 | S | workspace members에 `arb-db` 추가 |
| `monitor.rs` | 1,3 | XL | unwrap 제거, 재연결, LiveExecutor 직접 통합, TOCTOU 이중 체크, 초기화/reconciliation, 환율 guard, 잔고 연동, 펀딩비 |
| `position.rs` | 1-2 | L | Serialize/Deserialize, PositionState(5종), order_id, db_id, 빌더 패턴 |
| `config.rs` | 1,4 | M | 리스크 한도(비율+절대값), 주문/환율/펀딩/DB 파라미터 |
| `output/writer.rs` | 1-5 | L | CSV/JSON 파일 I/O -> DB INSERT 전환, SessionWriter -> DbSessionWriter |
| `output/summary.rs` | 1-5 | L | 파일 기반 요약 -> DB 쿼리 기반 요약 전환 |
| `orderbook.rs` | 1-1 | S | `std::sync::Mutex` -> `parking_lot::Mutex` |
| `coin_selector.rs` | 1-1,3-4 | M | `candles` Mutex parking_lot 전환, 펀딩 스케줄 조회 연동 |
| `spread.rs` | 1-1 | S | (프로덕션 `.expect()` 없음 -- 변경 최소) |
| `bybit/client.rs` | 2 | M | `place_order_linear()`, 선물 API, rate limiter 분리, margin_mode 검증, getTickers 펀딩 조회 |
| `bybit/types.rs` | 2 | S | PositionInfo 응답 타입, 펀딩 응답 타입 |
| `bybit/auth.rs` | 1-1 | S | SystemTime expect 제거, Debug impl 마스킹 |
| `bithumb/auth.rs` | 1-1 | S | SystemTime expect 제거 |
| `arb-exchange/types.rs` | 2 | S | PositionInfo 공통 타입 |
| `arb-exchange/error.rs` | 1-6 | S | is_retryable() 메서드 |
| `arb-exchange/src/lib.rs` | 2 | S | 신규 타입 re-export |
| `arb-strategy/Cargo.toml` | 1 | S | `parking_lot`, `arb-db` 추가 |
| `src/main.rs` | 3-1 | L | 라이브 전용 entry point, 모든 인프라 wiring |
| `pnl.rs` | 4-2 | S | 실수수료 + funding_fee + adjustment_cost 포함 PnL |
| `mod.rs` | 1,2 | S | 신규 모듈 등록 |

### 삭제/제거 대상

| 항목 | 이유 |
|------|------|
| WAL 패턴 (positions.json, temp file) | DB 트랜잭션으로 대체 |
| trades.csv | `trades` 테이블로 대체 |
| minutes.csv | `minutes` 테이블로 대체 |
| summary.txt / summary.json | DB 쿼리로 대체 |
| alerts.log | `alerts` 테이블로 대체 |
| `OutputConfig.dir` 필드 | 파일 출력 디렉토리 불필요 |
| JSON 파싱 에러 처리 | DB 사용으로 불필요 |
| temp file atomic write | DB 트랜잭션으로 대체 |

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
| C1 | `lock().unwrap()` 30+ 지점 | Mutex poison 시 패닉 | `parking_lot::Mutex` 전환 | 1-1 |
| C2 | ComputingFlags unwrap | 패닉 시 틱 영구 중단 | 동일 | 1-1 |
| C3 | SystemTime expect | NTP 역전 시 패닉 | `unwrap_or_default` + timestamp 0 시 주문 차단 | 1-1 |
| C4 | 포지션 메모리 only | crash 시 소실 | DB 기반 PositionStore + 상태 머신 + order_id 복구 | 1-0, 1-2 |
| C5 | WebSocket 재연결 없음 | 단절 시 프로세스 종료 | 자동 재연결 + 포지션 보호 | 1-7 |
| C6 | Kill switch + 진입 TOCTOU | 새 포지션 방치 가능 | pm 락 내 이중 체크 + 포지션 0 재스캔 | 1-3, 3-1 |
| C7 | LiveExecutor 거래소 타입 접근 | RPITIT -> dyn 불가 | 제네릭 LiveExecutor<U, B> 구체 타입 직접 사용 (trait/dyn 없음) | 2-3 |
| C8 | Reconciliation 자동 수정 위험 | 외부 거래 구분 불가 | 자동 수정 제거 -> 차단+알림+수동 | 3-3 |
| C9 | DB 장애 시 포지션 유실 | 영속화 불가 | 시작 차단 + 운영 중 재연결 + 5분 실패 kill switch | 1-0 |

### High (실거래 품질 필수)

| ID | 항목 | 현재 상태 | 대응 | Phase |
|----|------|----------|------|-------|
| H1 | Bybit rate limiter 공유 | 주문 vs 오더북 경합 | public/private 분리 | 2-2 |
| H2 | 양 레그 원자성 없음 | naked leg 노출 위험 | LiveExecutor + 3단계 비상 청산 escalation | 2-4 |
| H3 | Decimal::ZERO fallback | 0가격 진입 위험 | ZERO 가드 추가 | 1-6 |
| H4 | Bybit 시장가 슬리피지 미제어 | 급변동 시 수익 소멸 | 양 레그 모두 IOC 지정가 | 2-4 |
| H5 | 비상 청산 이중 실패 | naked exposure 무제한 | 3단계: IOC->시장가->kill switch | 2-4 |
| H6 | Upbit IOC 미지원 가능성 | 주문 거부 위험 | Live 시작 시 사전 검증 + GTC+cancel fallback | 2-4, 3-2 |
| H7 | 잔고 경합 (동시 진입) | 잔고 부족 주문 거부 | BalanceTracker 자본 예약 패턴 | 1-8 |
| H8 | instrument_cache RwLock unwrap | poisoned 데이터 사용 | `parking_lot::RwLock` 전환 | 1-1 |
| H9 | 리스크 한도 자본 비연동 | $300에서 $500 한도 = 166% | 비율(%) + 절대값 이중 | 1-3 |
| H10 | RiskManager lock order 미확정 | deadlock 위험 | lock order 문서에 위치 명시 | 1-3 |
| H11 | EmergencyCloseFailed silent | warn 로그만 | kill switch 직접 발동 | 1-6 |
| H12 | 환율 staleness 30분 | 수익 구간 소멸 가능 | 10분으로 단축, Phase 1 승격 | 1-9 |
| H13 | 주문 크기 상한 없음 | 버그 시 비정상 주문 | max_order_size_usdt 추가 | 1-3 |
| H14 | VirtualPosition 필드 추가 영향 | 모든 리터럴 수정 필요 | 빌더 패턴 도입 | 1-2 |
| H15 | PartiallyClosedOneLeg 복구 미정의 | 재시작 시 방치 | succeeded_leg/order_id 필드 + DB 기반 복구 | 1-2 |
| H16 | 펀딩비 8시간 고정 가정 | 4h/8h 종목 혼재 | 종목별 `FundingSchedule` + DB 저장 | 3-4 |

### Medium

| ID | 항목 | 대응 | Phase |
|----|------|------|-------|
| M1 | Box\<dyn Error\> erased | StrategyError 전환 | 1-6 |
| M2 | forex_task JoinHandle 무시 | 모니터링 + 재시작 | 1-7 |
| M3 | API 키 평문 메모리 | Debug 마스킹, 향후 secrecy | 4-3 |
| M4 | is_retryable 없음 | ExchangeError에 추가 | 1-6 |
| M5 | Bybit category "spot" 하드코딩 | place_order_linear() 추가 | 2-1 |
| M6 | 중복 주문 위험 | Client Order ID | 2-4 |
| M7 | 거래소 점검/입출금 정지 | 상태 API 조회 + 진입 차단 | 3-2 |
| M8 | Bybit 강제 청산 실시간 감지 | WS position 이벤트 수신 (향후) | -- |
| M9 | 펀딩비 종목별 주기 미추적 | 종목별 FundingSchedule + 매분 갱신 | 3-4 |
| M10 | computing flag lifetime | 주문 완료까지 확장 | 2-4 |
| M11 | Partial fill 초과분 무한 연쇄 | 최대 3회 재시도 + kill switch | 2-4 |
| M12 | 체결가 괴리 | Post-execution PnL gate | 2-4 |
| M13 | 마켓 임팩트 | 기존 safe_volume + IOC 슬리피지 제어 | 2-4 |
| M14 | 파일 I/O 잔류 코드 | output/writer.rs, summary.rs DB 전환 | 1-5 |

---

## 단계적 롤아웃 계획

```
Phase 0 (현재) ─── 시뮬레이션 검증 완료
     │
Phase 1 ───────── 안전 인프라 구축
     │              ├── arb-db 모듈 생성 (MySQL, sqlx) ★ 최우선
     │              ├── unwrap 패닉 제거 (parking_lot + RwLock)
     │              ├── PositionStore (DB 기반, 상태 머신 + order_id 복구)
     │              ├── RiskManager (kill switch TOCTOU, 비율+절대값 한도)
     │              ├── AlertService (치명적 동기 + DB fallback)
     │              ├── output/writer.rs, summary.rs DB 전환
     │              ├── 에러 분류 강화 (EmergencyClose -> kill switch)
     │              ├── WebSocket 자동 재연결 ★
     │              ├── BalanceTracker (잔고 추적 + 예약)
     │              └── 환율 staleness guard (10분) ★ 승격
     │
Phase 2 ───────── 주문 실행 엔진
     │              ├── Bybit 선물 API 확장 + margin_mode 검증
     │              ├── Rate limiter 분리
     │              └── LiveExecutor<U,B> 구체 타입 (양 레그 IOC, 3단계 escalation)
     │
Phase 3 ───────── monitor.rs 통합
     │              ├── LiveExecutor 직접 통합 + TOCTOU 해결
     │              ├── src/main.rs 라이브 entry point + examples/zscore_sim.rs 시뮬 분리
     │              ├── 시작 초기화 (잔고+포지션 복원+IOC 검증+DB 확인)
     │              ├── Reconciliation (차단+알림, 자동 수정 없음)
     │              └── 종목별 펀딩비 모니터링 (FundingSchedule + 매분 갱신)
     │
Phase 4 ───────── 설정 및 부가 기능
     │              ├── Config 확장 (비율+절대값 이중, 펀딩비, DB URL)
     │              ├── 실수수료 + 펀딩비 + adjustment_cost PnL
     │              └── API 키 보안 강화
     │
     ▼
검증 단계 (각 단계별 KPI):
  ① 소액 실거래 ($300~500, max_concurrent=5)
     KPI: 10건+ 성공, 레그 실패 0건
          API 성공률 > 99%, 24h 무크래시
          양 레그 체결 시간 차이 p95 < 2s
          슬리피지 p95 < 0.1%
          partial fill 발생률 < 5%
          Sim 병행 실행 -> PnL 오차 분해(slippage/fee/timing)
          잔고 추적 오차 < 1%
          DB 기록 정합성 100%
  ② 중액 실거래 ($1,000~3,000, max_concurrent=8)
     KPI: 시간당 거래 안정, kill switch 테스트 통과, 24h 무중단
          reconciliation 불일치 0건
          펀딩비 진입 차단 정상 동작
  ③ 풀자본 ($10,000, max_concurrent=10)
     KPI: 72h 무중단, 수익률 > 0 AND 시뮬 대비 50%+
```

---

## 체크리스트

### Phase 1: 안전 인프라

**1-0. arb-db 모듈**
- [ ] `crates/arb-db/` 디렉토리 및 `Cargo.toml` 생성
- [ ] `sqlx` 의존성 추가 (mysql, runtime-tokio, chrono, rust_decimal)
- [ ] `sessions` 테이블 마이그레이션 작성
- [ ] `positions` 테이블 마이그레이션 작성 (상태 머신 5종, order_id, succeeded_leg)
- [ ] `trades` 테이블 마이그레이션 작성 (side: entry/exit/emergency_close/adjustment)
- [ ] `minutes` 테이블 마이그레이션 작성
- [ ] `alerts` 테이블 마이그레이션 작성
- [ ] `funding_schedules` 테이블 마이그레이션 작성
- [ ] DbPool 래퍼 구현: `connect()`, `health_check()`, `acquire()`
- [ ] DbPositionStore 구현: `save()`, `update_state()`, `load_open()`, `remove()`
- [ ] 상태 전이 낙관적 잠금: `UPDATE ... WHERE state = ?` + affected_rows 확인
- [ ] trades/minutes/alerts/funding Repository 구현
- [ ] sessions Repository 구현 (시작/종료/상태 갱신)
- [ ] `migration.rs`: 커스텀 마이그레이션 러너 구현 (_migrations 테이블 + 버전 스캔 + 순차 적용)
- [ ] `V001`~`V006` 마이그레이션 SQL 파일 작성
- [ ] `examples/migrate.rs`: 마이그레이션 실행 바이너리 (DATABASE_URL -> connect -> run)
- [ ] DB 장애 처리: 시작 시 연결 실패 -> 차단, 운영 중 5분 실패 -> kill switch
- [ ] workspace `Cargo.toml`에 `arb-db` 추가
- [ ] `arb-strategy/Cargo.toml`에 `arb-db` 의존 추가

**1-1. unwrap 패닉 제거**
- [ ] `parking_lot` 크레이트 추가, `std::sync::Mutex` -> `parking_lot::Mutex` 전환 (monitor.rs, orderbook.rs, coin_selector.rs)
- [ ] `instrument_cache` `std::sync::RwLock` -> `parking_lot::RwLock` 전환 (monitor.rs 4곳)
- [ ] `tokio::sync::Mutex`/`RwLock`은 변경하지 않음 확인
- [ ] `SystemTime::expect()` -> `unwrap_or_default()` + warn 로그 (bybit/auth.rs + bithumb/auth.rs)
- [ ] coin_selector.rs, spread.rs의 `.expect()`는 `#[cfg(test)]` 한정 -- 프로덕션 영향 없음 확인
- [ ] `Decimal::ZERO` fallback 시 즉시 return 가드 (monitor.rs)

**1-2. 포지션 영속화 (Dual-State: 메모리 + DB)**
- [ ] `position_store.rs` 생성: `PositionStore` trait 정의
- [ ] `DbPositionStore` 구현 (arb-db 연동)
- [ ] Dual-State 설계: 메모리(PositionManager) = authoritative, DB = async shadow
- [ ] order_id 기록만 동기 DB write, 나머지 상태 전이는 비동기 DB write
- [ ] 상태 전이 3단계: 메모리 Opening -> DB 동기(order_id) -> 메모리 Open + DB 비동기
- [ ] `VirtualPosition`에 `Serialize/Deserialize`, `PositionState`, `order_id`, `db_id` 추가 + 빌더 패턴
- [ ] 포지션 상태 머신: Opening -> Open -> Closing -> Closed + PartiallyClosedOneLeg(상세 필드)
- [ ] Crash recovery: Opening(order_id) -> get_order() 조회, Closing -> 청산 주문 확인
- [ ] `load_open()` 실패 시 order_id 기반 reconciliation 강제 + 진입 차단

**1-3. Kill Switch + Risk Manager**
- [ ] `risk.rs` 생성: `RiskManager` + AtomicBool kill switch
- [ ] 리스크 한도: 비율(%) + 절대값 이중, `max_order_size_usdt` 추가
- [ ] kill switch TOCTOU: pm 락 내 이중 체크
- [ ] kill switch 청산: 별도 task spawn, notional 내림차순, 정상 청산 비활성화, 포지션 0 재스캔
- [ ] kill switch COMPLETE 조건: open_count==0 AND reconciliation 통과
- [ ] RiskManager 일일 리셋: KST 00:00 (UTC 15:00) 기준
- [ ] lock order 갱신: `ob_cache -> instrument_cache -> position_mgr -> balance_tracker -> risk_manager.inner -> counters -> spread_calc`

**1-4. AlertService**
- [ ] `alert.rs` 생성: 일반 알림 mpsc(64) try_send, 치명적 알림 동기 전송
- [ ] 텔레그램 실패 시 `alerts` 테이블에 INSERT (DB fallback)
- [ ] 알림 이벤트 타입 13종 구현 (DbConnectionLost, FundingBlockEntry 추가)

**1-5. output DB 전환**
- [ ] `SessionWriter` -> `DbSessionWriter` 전환 (trades/minutes INSERT)
- [ ] `SessionSummary` DB 쿼리 기반으로 전환
- [ ] 세션 시작/종료 DB 기록
- [ ] CSV/JSON 파일 I/O 코드 제거
- [ ] temp file atomic write 패턴 제거
- [ ] `OutputConfig.dir` -> DB 커넥션 주입으로 변경

**1-6. 에러 분류**
- [ ] `ExchangeError::is_retryable()` 메서드 추가
- [ ] `EmergencyCloseFailed` 에러 -> kill switch 직접 발동 (silent failure 방지)
- [ ] `spawned_check_tick_signal` 에러 타입 `Box<dyn Error>` -> `StrategyError` 전환

**1-7. WebSocket 재연결**
- [ ] WebSocket 재연결을 **별도 tokio::spawn** task로 실행 (★ select! 루프에서 retry 금지)
- [ ] 재연결 성공 시 새 rx 채널을 select! 루프에 전달
- [ ] 재연결 중에도 select! 루프가 나머지 이벤트 계속 처리 확인
- [ ] 재연결 5분 실패 시 kill switch 발동 -> 활성 포지션 전체 청산
- [ ] `_forex_task` JoinHandle 모니터링 + 패닉 시 spawn으로 재시작

**1-8. BalanceTracker**
- [ ] `balance.rs` 생성: `BalanceTracker` (reserve/commit/release/on_exit/sync)
- [ ] 시작 시 잔고 초기화 (Upbit KRW + Bybit USDT)
- [ ] 동시 진입 잔고 경합 방지 (reservation 패턴)
- [ ] 시뮬레이션 example은 BalanceTracker 미사용 확인

**1-9. 환율 guard**
- [ ] 환율 staleness guard: 캐시 age > 10분 시 진입 차단, 급변 > 0.2% 시 알림

**Phase 1 완료 조건**
- [ ] `cargo test -p arb-strategy` 전체 통과
- [ ] `cargo test -p arb-db` 전체 통과
- [ ] `cargo clippy` 경고 0

### Phase 2: 주문 실행 엔진
- [ ] Bybit 선물 API: `place_order_linear()`, `set_leverage()`, `switch_margin_mode()`, `get_positions()`
- [ ] margin_mode 변경 실패 -> 현재 모드 확인 -> Cross면 시작 차단
- [ ] `PositionInfo` 타입 추가
- [ ] Bybit rate limiter `public_limiter` / `private_limiter` 분리
- [ ] `live_executor.rs` 생성: `LiveExecutor<U, B>` 구체 타입 (trait/dyn 없음)
- [ ] `EntryRequest`/`ExitRequest`/`ExecutedEntry`/`ExecutedExit` 타입 정의
- [ ] `OrderExecutionError` enum (Timeout에 leg/order_id/other_leg 상세)
- [ ] `ExecutedEntry`에 `adjustment_cost` 필드
- [ ] 양 레그 모두 IOC 지정가 (Upbit + Bybit)
- [ ] `tokio::time::timeout` 개별(5s) + 전체(10s) 이중 timeout
- [ ] Client Order ID 멱등성
- [ ] 비상 청산 3단계: IOC(0~2분) -> 시장가(2~5분) -> kill switch(5분+)
- [ ] Cancel 실패 -> get_order -> PartiallyFilled 수량 조정
- [ ] Partial fill 초과분 청산 최대 3회 재시도, 수렴 안 하면 kill switch
- [ ] Post-execution PnL gate
- [ ] Computing flag lifetime -> 주문 완료까지 확장
- [ ] 비정상 거래: `trades` 테이블에 `side='adjustment'`로 기록
- [ ] 비상 청산 손실 -> RiskManager.record_trade() 포함
- [ ] `RiskManager.validate_order_size()` 발주 전 확인
- [ ] 단위 테스트: 양쪽 성공/한쪽 실패/타임아웃/partial fill
- [ ] `cargo test` 전체 통과 + `cargo clippy` 경고 0

### Phase 3: monitor.rs 통합 + 바이너리 분리
- [ ] `ZScoreMonitor<U, B>`에 `LiveExecutor<U, B>` 필드 추가 (구체 타입)
- [ ] `src/main.rs`: 라이브 전용 entry point (DB, BalanceTracker, RiskManager, AlertService, LiveExecutor 전부 wiring)
- [ ] `examples/zscore_sim.rs`: 기존 시뮬레이션 코드 분리 (라이브 인프라 의존 없음)
- [ ] 진입 TOCTOU: reserve -> pm.lock -> is_killed 이중 체크 -> register_opening -> unlock -> execute
- [ ] 청산 3곳 전부 select! 외부(spawn) 실행 확인: ① spawned_check_tick_signal, ② TTL spawn, ③ kill switch spawn
- [ ] 청산 3곳: kill switch 시 1,2번 비활성화, PositionState "Closing" 중복 방지
- [ ] 시작 시 Bybit 설정 검증 (leverage, margin_mode 실패 처리)
- [ ] 시작 시 BalanceTracker 초기화 + config.total_capital_usdt 조정
- [ ] 시작 시 DB 연결 확인
- [ ] 시작 시 Upbit IOC 지원 여부 사전 검증
- [ ] 미청산 포지션 복원 (DB load_open + order_id 기반 get_order() 조회)
- [ ] `minute_timer` REST 작업 전부 tokio::spawn으로 분리 (★ select! 비블로킹 확인)
- [ ] reconciliation: spawn + 결과 mpsc/Arc 반환, order_id 기반, 자동 수정 없음, 불일치 시 진입 차단
- [ ] 잔고 동기화: spawn + 매 5분 sync_from_exchange(), 10% 괴리 시 보정+warn
- [ ] 종목별 펀딩비: 코인 선택 시 FundingSchedule 초기 조회
- [ ] 펀딩비: spawn + 매분 getTickers -> fundingRate/nextFundingTime 갱신
- [ ] 펀딩비: 정산 30분 전 + 불리(short가 지불) 시 해당 코인 진입 차단
- [ ] 펀딩비: DB funding_schedules 테이블 UPSERT
- [ ] 시뮬 example이 기존 동작 100% 유지 확인 (`cargo run --example zscore_sim`)
- [ ] 통합 테스트: LiveExecutor + PositionStore + RiskManager + BalanceTracker end-to-end
- [ ] Crash recovery 테스트 (DB 기반 각 상태에서 크래시 시뮬레이션)
- [ ] RiskManager 테스트 (비율+절대값 한도, TOCTOU, kill switch 재스캔)
- [ ] `cargo test` 전체 통과 + `cargo clippy` 경고 0

### Phase 4: 설정 및 부가 기능
- [ ] `config.rs`에 라이브 전용 설정 추가 (order_timeout_sec, max_slippage_pct, 환율, 펀딩비, db_url 등)
- [ ] `ClosedPosition`에 `actual_fees`, `funding_fee`, `adjustment_cost` 추가
- [ ] Order.paid_fee 기반 실수수료 PnL 계산
- [ ] API 키 Debug impl 마스킹 (bybit/auth.rs, upbit/auth.rs)
- [ ] `cargo test` 전체 통과 + `cargo clippy` 경고 0

### 검증
- [ ] 소액 ($300-500): 10건+ 성공, 레그 실패 0건, API 성공률 > 99%, 24h 무크래시, 체결 latency p95 < 2s, partial fill < 5%, Sim 병행 PnL 오차 분해, 잔고 오차 < 1%, DB 기록 정합성 100%
- [ ] 중액 ($1,000-3,000): 24h 무중단, kill switch 통과, reconciliation 불일치 0건, 펀딩비 진입 차단 정상 동작
- [ ] 풀자본 ($10,000): 72h 무중단, 수익률 > 0 AND 시뮬 대비 50%+
