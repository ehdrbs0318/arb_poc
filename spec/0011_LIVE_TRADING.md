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
    ├── ★ client_order_id 사전 기록 (Opening INSERT 시), 청산 시에도 exit_client_order_id 사전 기록. order_id는 체결 후 비동기 write
    ├── poll_until_filled(timeout=5s, backoff 200ms~2s)
    ├── 양쪽 Filled -> effective_qty = min(upbit_filled_qty * (1 - upbit_fee_rate), bybit_filled_qty)
    │   // Upbit은 코인 수수료를 수량에서 차감하므로, 실 수령 수량 기준으로 effective_qty를 산출한다.
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
4. **기존 이벤트 루프 구조 존중**: monitor_core.rs의 select! 루프 구조 유지, ExecutionPolicy trait 기반 컴파일 타임 분기 (시뮬/라이브)
5. **DB 우선**: 모든 영속 데이터는 MySQL(arb-db)에 저장, 라이브에서 파일 I/O 제거 (시뮬은 FileSessionWriter 유지)
6. **실시간 판단은 메모리**: 잔고 예약, kill switch, 포지션 잠금/상태 관리 등 실시간 의사결정은 프로세스 내 메모리(Mutex/AtomicBool)로만 수행. DB는 영속화/복구 전용이며 의사결정 경로(hot path)에 포함하지 않음
7. **시뮬레이션 독립**: 시뮬레이션 example은 라이브 인프라(DB, BalanceTracker, RiskManager)에 의존하지 않음. 기존 VirtualPosition + 가상 체결 코드를 그대로 사용

### 바이너리 구조

```
┌────────────────────────────────┐     ┌──────────────────────────────┐
│    src/main.rs (라이브 전용)     │     │  examples/zscore_sim.rs      │
│    cargo run --release          │     │  cargo run --example ...     │
├────────────────────────────────┤     ├──────────────────────────────┤
│  ZScoreMonitor<U, B, LivePolicy>│     │  ZScoreMonitor<U, B, SimPolicy>│
│  monitor_core + monitor_live   │     │  monitor_core + monitor_sim  │
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
- **예외**: client_order_id는 Opening INSERT 시 사전 기록 (동기). order_id는 체결 확인 후 비동기 write. DB write 실패 시 주문은 이미 발주됨 → reconciliation에서 보정.
- 시작 시 DB에서 메모리 상태 복원 -> 이후 메모리가 권위(authoritative)

```
의사결정 -> 메모리 상태 갱신 (즉시) -> DB 비동기 반영
시작 시:   DB 조회 -> 메모리 상태 복원 -> 이후 메모리만 참조
```

**DB write 순서 보장**: 모든 비동기 DB write는 단일 `mpsc::channel<DbWriteRequest>` background writer task로 전송. **전체 직렬 순서 보장** (단일 consumer, position_id별 인과 순서 자동 충족 — 현재 규모 max_concurrent=5~10에서 단일 consumer로 충분). bounded(256) + try_send 실패 시 **newest(현재 메시지) 드랍** + warn (oldest가 아님). 드랍된 write를 별도 overflow log에 기록. **드랍 발생 시 해당 position_id에 `dirty` flag를 설정한다. 다음 reconciliation에서 dirty 포지션의 DB 상태를 메모리 상태로 강제 동기화한다. 이를 통해 드랍-crash 시간 창(최대 1분)의 정합성 위험을 제거한다.** order_id 이후 상태 전이 드랍 시 reconciliation에서 감지+보정. 재시도 상한 3회, 최종 실패 시 AlertService 알림.

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
| Reconciliation | get_positions, get_order × N | **별도 tokio::spawn** | minute_timer (1분 주기, 열린 포지션 0개면 스킵, 3개 이상이면 2분 주기. Bybit WS position topic 수신 시 REST reconciliation 스킵 가능) |
| 펀딩비 갱신 | getTickers | **별도 tokio::spawn** | minute_timer (1분 주기) |
| WebSocket 재연결 | connect, subscribe | **별도 tokio::spawn** | 끊김 감지 시 |
| Kill switch 청산 | place_order × N | **별도 tokio::spawn** (이미 명시) | kill switch 발동 |
| 치명적 알림 | Telegram send | **spawn된 task 내에서만** 전송. **★ 청산 실행 완료 후 알림 전송** (알림이 청산을 블로킹하지 않도록) | kill switch, 비상 청산 실패 |

**minute_timer spawn 패턴**:
```
minute_timer.tick() => {
    // ① 메모리 전용 (select! 내, 즉시)
    finalize_and_process()   // 기존: 통계, TTL, 캔들 — 메모리 연산만
    check_ttl_positions()    // 기존: TTL 만료 체크 — 메모리 연산만
    risk_manager.cleanup_expired_losses()  // rolling_24h_losses 만료 엔트리 정리 — 메모리 연산만

    // ② REST 호출 (spawn, select! 비블로킹)
    if 5분_경과 {
        tokio::spawn(sync_from_exchange(...))      // 잔고 동기화
    }
    tokio::spawn(reconciliation_check(...))        // 포지션 정합성
    tokio::spawn(update_funding_schedules(...))    // 펀딩비 갱신
    // → 결과는 mpsc 채널 또는 공유 상태(Arc)로 반환

    // ③ select! iteration latency 자가 진단
    if select_iteration_elapsed > 10ms {
        warn!("select! iteration too slow");
        counters.slow_select_iteration += 1;
    }
}
```

### 주문 전략: 양 레그 모두 IOC 지정가

**Upbit IOC 지정가**:
- qty 직접 지정 가능, 슬리피지 상한 제어. 미체결분은 자동 취소.
- **사전 검증 필요**: Live 시작 시 Upbit API가 실제로 IOC를 지원하는지 마켓별 확인. `ord_type`에 IOC 없는 마켓은 **GTC 지정가 + timeout 후 cancel** fallback 자동 적용.
- price = `upbit_krw_price * (1 + max_slippage_pct)` (슬리피지 상한)
- **시장가 fallback** (config): `krw_amount = qty * upbit_krw_price` -> `OrderRequest::market_buy(market, krw_amount)` (Upbit 시장가 매수는 KRW 금액 기준, 수수료는 체결 후 코인 수량에서 별도 차감)

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
3. **Client Order ID**: UUID v7 사용 (session 정보 미노출, crash recovery는 DB 매핑으로 해결. 멱등성 보장, crash 복구 시 주문 조회용)

**Bybit WS 체결 확인 (REST polling 병행)**: Bybit WebSocket의 execution topic을 구독하여 체결 이벤트를 실시간 수신. REST polling은 WS 이벤트 미수신 시 fallback으로 유지. WS 체결 확인이 먼저 도착하면 polling 즉시 종료하여 latency 최소화.

### 잔고 관리 (BalanceTracker)

**문제**: 시뮬레이션은 자본 무제한이지만, 라이브에서는 거래소별 가용 잔고를 실시간 추적해야 한다. 특히 `spawned_check_tick_signal`이 `tokio::spawn`으로 병렬 실행되므로, 2개 코인의 진입 시그널이 동시 발생 시 잔고 경합이 발생한다.

**설계**:
```rust
/// 양 거래소 잔고 + 예약 상태를 단일 Mutex로 보호.
/// 단일 Mutex guard 내에서 양쪽 잔고를 동시 차감하여 원자적 예약/롤백 보장.
struct BalanceState {
    upbit_available_krw: Decimal,
    bybit_available_usdt: Decimal,
    reservations: Vec<ReservationRecord>,   // 활성 예약 목록 (TTL sweeper용)
}

pub struct BalanceTracker {
    inner: parking_lot::Mutex<BalanceState>,
}

impl BalanceTracker {
    /// 진입 전 잔고 예약. 성공 시 ReservationToken 반환.
    /// **양 거래소 원자적 예약**: 단일 Mutex guard 내에서 양쪽 잔고를 동시 차감. Bybit 부족 시 Upbit 예약도 함께 롤백.
    /// **ReservationToken RAII**: `Drop` impl에서 `try_lock()` 사용, 실패 시 sweeper 위임. 내부 `committed: bool` 플래그로 commit 후 drop 시 release 방지. panic 시 잔고 영구 차감 방지. `Arc<parking_lot::Mutex<BalanceState>>` 보유.
    /// **불변 조건**: `reserve()` 메서드 내부에서 MutexGuard를 먼저 drop한 후에만 ReservationToken을 생성/반환한다. guard 보유 중에는 ReservationToken 인스턴스가 존재하지 않으므로, Drop 시 try_lock()이 같은 스레드에서 재진입 교착을 일으키지 않는다.
    /// **ReservationToken TTL**: 생성 시각 포함, **6분** 이상 미확정 시 background sweeper가 자동 release + warn 로그 (비상 청산 5분 + 여유 1분).
    pub fn reserve(&self, upbit_krw: Decimal, bybit_usdt: Decimal) -> Option<ReservationToken>;

    /// 주문 성공 시 예약을 확정 (실 체결 금액으로 잔고 차감).
    pub fn commit(&self, token: ReservationToken, actual_upbit_krw: Decimal, actual_bybit_usdt: Decimal);

    /// 주문 실패/취소 시 예약 해제 (잔고 복원).
    pub fn release(&self, token: ReservationToken);

    /// 청산 완료 시 잔고 복원.
    pub fn on_exit(&self, received_upbit_krw: Decimal, received_bybit_usdt: Decimal);

    /// 거래소 실잔고와 동기화 (minute_timer에서 주기적 호출).
    /// **expected = available + reserved_total**과 실잔고 비교. reserved_total을 고려하여 drift만 보정. reserve 중인 금액이 덮어쓰여지는 TOCTOU 방지.
    /// **in_flight 스킵**: in_flight 상태(reserve 후 commit/release 전)인 예약이 1건 이상이면, 해당 sync 주기를 스킵한다. 거래소 API 응답은 수 초 전 스냅샷이므로, 주문 진행 중 비교는 거짓 양성(false positive) 불일치를 유발한다.
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

**lock order**: `balance_tracker → position_mgr` 순서 (TOCTOU 흐름과 일치). `BalanceTracker`의 Mutex는 `parking_lot::Mutex`이며, `position_mgr` lock **외부**에서 호출. reserve -> pm.lock -> commit/release 순서.

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
                                                   //   내부에서 order_id DB 비동기 기록 (client_order_id는 사전 기록 완료)
8. position_mgr.lock().await -> 메모리 결과 반영 + DB 비동기 반영
```

**★ Opening in_flight 방어**: execute_entry() 진행 중인 포지션에 `in_flight: bool` 플래그. kill switch 청산 task는 in_flight 포지션이 Open 또는 삭제로 전이될 때까지 1~2초 간격으로 재스캔 (최대 30회 = 1분).

**★ Closing timeout 인수**: 재스캔에서 `state == Closing && (now - closing_started_at) > 15s`인 포지션을 kill switch가 인수. 15초는 정상 청산의 IOC timeout(5초) + polling 재시도를 고려한 예상 최대 소요 시간의 약 2배. kill switch 인수 시 정상 청산 task에 `CancellationToken`을 전송하여 추가 주문 발행을 중지시킨다. `PositionManager`에 `try_transition_to_closing(pos_id) -> bool` 메서드 추가. `VirtualPosition`에 `closing_started_at: Option<DateTime<Utc>>` 필드 추가.

**Kill switch 발동 시 배타적 청산**:
1. `is_killed.store(true, Release)` -> 신규 진입 즉시 차단
2. 정상 청산 로직도 비활성화 (PositionState가 "Closing"이 아닌 포지션만 kill switch가 청산)
3. kill switch 청산 task가 **포지션 규모(notional) 내림차순**으로 순차 청산
4. 각 포지션을 "Closing"으로 전이 -> 중복 청산 방지
5. 전체 완료 조건: `PositionManager.open_count() == 0` AND 거래소 reconciliation 통과
6. kill switch 청산 task 자체가 kill switch를 재발동하지 않도록 guard
7. "KILL SWITCH COMPLETE" 텔레그램 발송 후 이벤트 루프 종료
8. kill switch 청산은 **별도 task로 spawn** (deadlock 방지)
9. kill switch 재스캔 간격: 1~2초, 최대 30회. in_flight 포지션 완료 대기. **30회 초과 시 해당 포지션을 PendingExchangeRecovery로 전이 + 강제 cancel 시도.**

**Rolling 24h 누적 손실 한도**: 일일 리셋(KST 00:00) 경계 악용 방지. 직전 24시간 sliding window 누적 손실이 `max_rolling_24h_loss_usdt`를 초과하면 kill switch. 일일 한도와 독립 적용. **minute_timer에서도 `cleanup_expired_losses()` 호출** (만료된 rolling_24h_losses 엔트리 정리).

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
- **커넥션 풀**: `max_connections = 10`, `min_connections = 2`, `acquire_timeout = 5s`. config로 조정 가능.

**DB 테이블 스키마** (핵심 컬럼만 기술, exact type은 구현 시 결정):

```sql
-- 세션 메타데이터
CREATE TABLE sessions (
    id                  BIGINT AUTO_INCREMENT PRIMARY KEY,
    parent_session_id   BIGINT UNSIGNED NULL,    -- crash recovery 시 이전 세션 참조
    started_at          DATETIME(3) NOT NULL,
    ended_at            DATETIME(3),
    config_json         TEXT NOT NULL,           -- 세션 시작 시 config snapshot (민감 필드 redact)
    status              VARCHAR(20) NOT NULL     -- Running, Completed, Crashed, GracefulStop
);

-- 포지션 상태 머신
CREATE TABLE positions (
    id                  BIGINT AUTO_INCREMENT PRIMARY KEY,
    session_id          BIGINT NOT NULL,
    coin                VARCHAR(20) NOT NULL,
    state               VARCHAR(30) NOT NULL,  -- Opening, Open, Closing, Closed, PartiallyClosedOneLeg, PendingExchangeRecovery
    upbit_qty           DECIMAL(20,8) NOT NULL,  -- 수수료 차감/dust로 인한 미세 차이 추적
    bybit_qty           DECIMAL(20,8) NOT NULL,  -- 수수료 차감/dust로 인한 미세 차이 추적
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
    exit_upbit_order_id VARCHAR(100),       -- 청산 주문 추적
    exit_bybit_order_id VARCHAR(100),      -- 청산 주문 추적
    client_order_id     VARCHAR(100),       -- crash recovery 시 거래소 검색용 (진입)
    exit_client_order_id VARCHAR(64) NULL,  -- crash recovery 시 청산 주문 거래소 검색용
    in_flight           BOOLEAN DEFAULT FALSE, -- 주문 진행 중 플래그
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
    exit_usd_krw        DOUBLE,                -- 청산 시점 환율 (Phase 4 이후 세금 산정용)
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

- **주의**: MySQL DDL(CREATE TABLE)은 트랜잭션 내에서 암묵적 COMMIT을 발생시킴. 마이그레이션 파일당 하나의 DDL만 포함.

- **마이그레이션 파일 목록**:

| 파일 | 설명 |
|------|------|
| `V001__create_sessions.sql` | sessions 테이블 |
| `V002__create_positions.sql` | positions 테이블 + 인덱스 |
| `V003__create_trades.sql` | trades 테이블 + 인덱스 |
| `V004__create_minutes.sql` | minutes 테이블 + 인덱스 |
| `V005__create_alerts.sql` | alerts 테이블 + 인덱스 |
| `V006__create_funding_schedules.sql` | funding_schedules 테이블 |

**Background DB Writer**: `mpsc::channel<DbWriteRequest>` bounded(256) 단일 consumer task. **전체 직렬 순서 보장** (단일 consumer). try_send 실패 시 newest 드랍 + warn + overflow log. 재시도 3회 상한, 최종 실패 시 AlertService 알림.

**PositionStore trait** (DB 구현체):
```rust
// 코드베이스 전체와 일관성을 위해 RPITIT 패턴 사용. 테스트에서는 MockPositionStore 구체 타입을 직접 사용한다.
pub trait PositionStore: Send + Sync {
    /// 포지션 INSERT (Opening 상태).
    fn save(&self, pos: &PositionRecord) -> impl Future<Output = Result<i64, DbError>> + Send;

    /// 포지션 상태 전이 (낙관적 잠금: WHERE state = expected_state).
    fn update_state(&self, id: i64, from: &str, to: &str, fields: UpdateFields) -> impl Future<Output = Result<TransitionResult, DbError>> + Send;

    /// 특정 세션의 non-Closed 포지션 조회 (crash recovery용).
    fn load_open(&self, session_id: i64) -> impl Future<Output = Result<Vec<PositionRecord>, DbError>> + Send;

    /// 포지션 삭제 (Opening 미발주 건).
    fn remove(&self, id: i64) -> impl Future<Output = Result<(), DbError>> + Send;
}
```

**상태 전이 -- DB 트랜잭션**:
- WAL 패턴 대체: `BEGIN` -> `UPDATE positions SET state = 'Open' WHERE id = ? AND state = 'Opening'` -> `COMMIT`
- 낙관적 잠금: `WHERE state = ?` 조건으로 동시 전이 방지. affected_rows == 0이면 이미 다른 경로에서 처리됨.
- **affected_rows 처리**: `update_state()`는 `Result<TransitionResult, DbError>`를 반환. `TransitionResult::Applied`는 정상 전이, `TransitionResult::AlreadyTransitioned(PositionState)`는 이미 다른 경로에서 처리됨을 의미. 예상된 전이(kill switch가 이미 Closing 처리)면 무시, 비예상 전이면 warn + AlertService 알림.
- `ROLLBACK`: 트랜잭션 중 에러 시 자동 롤백 (sqlx 기본 동작)

**Upbit client_order_id 사전 검증**: Upbit의 `/v1/orders/chance` API에서 client_order_id 기반 검색 지원 여부를 Phase 1-0에서 사전 검증한다. 미지원 시 Opening 상태 미체결 주문을 timestamp+coin 기반으로 매칭하는 fallback 로직을 구현한다. Client Order ID 형식은 **UUID v7** 사용 (session 정보 미노출, crash recovery는 DB 매핑으로 해결).

**Crash recovery**:
- "Opening" (order_id 없음): 주문 발주 전 크래시 -> DB에서 DELETE. **추가 안전장치**: client_order_id가 있으면 거래소 `get_open_orders()`로 해당 주문 검색 (order_id DB write 실패 + crash 시 orphan order 방지)
- "Opening" (order_id 있음): 양쪽 order_id 각각 `get_order()` 조회. 한쪽만 체결된 경우 PartiallyClosedOneLeg 전이 후 비상 청산.
- "Closing" (order_id 있음): exit_upbit_order_id, exit_bybit_order_id 각각 조회하여 양 레그 상태 개별 확인. Filled이면 "Closed" UPDATE, 아니면 청산 재시도. exit_order_id가 NULL이지만 exit_client_order_id가 있으면 거래소에서 client_order_id로 검색하여 주문 상태 확인.
- "PartiallyClosedOneLeg": succeeded_leg/order_id 기반 비상 청산 재시도

**Crash recovery 8개 시나리오 체크리스트**:
1. Opening (order_id NULL) + crash → 재시작 후 삭제
2. Opening (order_id 한쪽만) + crash → 해당 레그 get_order 후 처리
3. Opening (양쪽 order_id) + 한쪽만 Filled + crash → PartiallyClosedOneLeg 전이
4. Open + crash → 정상 복원
5. Closing (exit_order_id 양쪽) + 한쪽 Filled + crash → 비대칭 처리
6. PartiallyClosedOneLeg + crash → 비상 청산 재시도
7. DB write 실패 + crash → client_order_id 기반 거래소 검색
8. Session ID 마이그레이션 → trades cross-session 집계 정합성

**Session ID 연속성**: crash recovery 시 이전 세션을 `status='Crashed'`로 마감 + 새 session_id 생성. 미청산 포지션은 `parent_session_id`를 참조하는 새 세션에서 관리 (session_id 직접 마이그레이션 대신). `SessionSummary` 쿼리에서 position_id 기반 cross-session trades 집계.

**DB 장애 처리**:
- **시작 시 연결 실패**: 프로세스 시작 차단 (DB 없이 실거래 불가)
- **운영 중 연결 끊김**: 신규 진입 즉시 차단 + 재연결 시도 (지수 백오프 1s~30s)
- **5분 연속 실패**: kill switch 강제 발동 -> 활성 포지션 전체 청산 + 텔레그램 알림
- sqlx 커넥션 풀 health check: `minute_timer`에서 `pool.acquire()` 확인

**output/writer.rs, output/summary.rs 전환**:
- **SessionWriter trait 추상화**: `SessionWriter` trait 정의 -> `FileSessionWriter`(시뮬, 기존 CSV/JSON), `DbSessionWriter`(라이브, DB INSERT). 시뮬 example은 FileSessionWriter를 사용하여 파일 출력 유지. 라이브는 DbSessionWriter 사용.
- `SessionSummary` -> DB 쿼리로 생성: `SELECT COUNT(*), SUM(realized_pnl), ... FROM trades WHERE session_id = ?`
- temp file atomic write 패턴 제거
- JSON 파싱 에러 처리 불필요

**AlertService fallback**: 텔레그램 실패 시 `alerts` 테이블에 INSERT (기존 alerts.log 파일 대체)

**Triple failure fallback** (즉시 죽음이 아닌, 최선을 다한 후 종료):
  1. `is_killed.store(true, Release)` — 신규 진입 차단
  2. Kill switch 청산 시도 (활성 포지션 보호)
  3. Graceful shutdown 시그널 전송
  4. 타임아웃 30초 후에도 미완료 시 그때 `std::process::exit(1)` (외부 모니터링 감지용)
  5. stderr + `/tmp/arb_emergency.log`에 기록

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

- **API 키 Debug 마스킹 (Phase 4-3에서 승격)**: 실거래 시작 전 `#[derive(Debug)]` → 수동 Debug impl으로 API 키 마스킹 적용 필수. `BybitCredentials`, `UpbitCredentials`에 수동 `Debug` impl (키 마스킹). 향후: `secrecy::SecretString` 전환.

**파일**: `monitor.rs`, `orderbook.rs`, `coin_selector.rs`, `spread.rs`, `bybit/auth.rs`, `bithumb/auth.rs`, `upbit/auth.rs`
**규모**: M

#### 1-2. 포지션 영속화 (PositionStore -- DB 기반)

**문제**: `PositionManager`가 순수 인메모리 -> crash 시 열린 포지션 정보 소실

**Dual-State 설계**:
- **실시간 상태**: `PositionManager` (tokio::sync::Mutex) -- 모든 진입/청산 판단, 잠금, 상태 전이의 단일 진실 소스
- **영속 상태**: `DbPositionStore` (arb-db) -- crash recovery 전용, 메모리 상태를 비동기로 shadow
- 의사결정은 **절대 DB를 참조하지 않음**. DB가 느려도/끊겨도 실시간 전략 판단에 영향 없음.
- 시작 시: DB에서 non-Closed 포지션 로드 -> 메모리에 복원 -> 이후 메모리만 참조

**DB 기록 시점** (hot path 외부):
- **동기 DB write (필수)**: client_order_id만 사전 기록 (Opening INSERT 시). crash 시 client_order_id로 거래소에서 주문 검색 가능. **client_order_id DB INSERT가 실패하면 해당 진입을 차단한다(주문 발주 거부). crash recovery 시 orphan order 감지가 불가능하므로, DB 장애 시 신규 진입 차단 정책과 동일하게 처리한다.**
- **비동기 DB write**: order_id 기록 (체결 확인 후), Opening 등록, Open 전이, Closing 전이, Closed 전이. 메모리 갱신 후 fire-and-forget.
- **DB 장애 시**: 비동기 write 실패 -> warn 로그 + 재시도 큐. order_id write 실패 -> 주문은 이미 나간 상태이므로 메모리에는 반영하되, DB 복구 시 reconciliation으로 보정.

**변경**:
- `position_store.rs` 신규 생성
- `PositionStore` trait 정의 (Phase 1-0 참조)
- 구현체: `DbPositionStore` (arb-db의 `positions` 테이블 사용)

**포지션 상태 머신**:
```
Opening → Open → Closing → Closed
          |        |    \
          |        |     └→ PendingExchangeRecovery → Closed
          |        └→ PartiallyClosedOneLeg → Closed
          |                                    ↑
          └→ PartiallyClosedOneLeg ──→ PendingExchangeRecovery → Closed
```
Closing 상태에서 양쪽 거래소 모두 응답하지 않는 경우(API timeout, 점검 등), PartiallyClosedOneLeg를 거치지 않고 직접 PendingExchangeRecovery로 전이한다.

**상태 전이 세분화** (메모리 우선 + DB shadow):
1. **메모리**: `pm.register_opening(pos)` (즉시) -> **DB 동기**: `INSERT positions (state='Opening', coin, qty, client_order_id, expected_prices)` (★ client_order_id 사전 기록)
2. **주문 발주 후** -> **DB 비동기**: `UPDATE positions SET upbit_order_id=?, bybit_order_id=? WHERE id=? AND state='Opening'` (체결 확인 후. DB write 실패 → reconciliation 보정)
3. **체결 확인** -> **메모리**: `pm.transition_to_open(pos)` (즉시) -> **DB 비동기**: `UPDATE positions SET state='Open', qty=?, actual_prices=?`

-> 1단계에서 client_order_id 기록 후 크래시 시 거래소에서 client_order_id로 주문 검색 가능
-> 1단계 INSERT 전 크래시 시 DB에 레코드 없을 수 있음 -> 주문도 미발주이므로 안전

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
- **Partial fill 세부 전이**: 비상 청산 IOC에서 partial fill 발생 시, 체결 수량만큼 정리 완료 + 미체결 수량으로 PartiallyClosedOneLeg 유지 + emergency_attempts 증가 + 잔여 수량만 재시도.

**PendingExchangeRecovery 상태**:
- Kill switch 발동 시 장애 거래소 쪽 포지션은 `PendingExchangeRecovery` 상태로 전이
- 거래소 복구 감지 시 자동으로 잔여 leg 청산 → Closed 전이
- AlertService에 수동 액션 가이드 포함 (어떤 거래소의 어떤 코인이 미청산인지 상세 안내)
- **복구 감지**: 30초마다 해당 거래소 health check API 호출 (또는 `get_order()` 시도)
- **최대 체류 시간**: `pending_recovery_timeout_hours = 2` (config). 2시간 초과 시 수동 처리 알림(텔레그램) + 해당 포지션 잔고 예약 해제

**Crash recovery** (DB 기반):
- `SELECT * FROM positions WHERE session_id=? AND state NOT IN ('Closed')` -> non-Closed 포지션 조회
- "Opening" (order_id NULL): 주문 발주 전 크래시 -> DELETE
- "Opening" (order_id 있음): `get_order()` 조회 -> 체결이면 "Open" UPDATE, 미체결이면 cancel + DELETE
- "Closing" (order_id 있음): `get_order()` 조회 -> Filled이면 "Closed" UPDATE, 아니면 청산 재시도
- "PartiallyClosedOneLeg": succeeded_leg 기반 비상 청산 재시도

**`VirtualPosition`에 추가 필드**:
- `#[derive(Serialize, Deserialize)]` 추가 (`serde`는 이미 dependencies에 존재)
- `upbit_order_id: Option<String>`, `bybit_order_id: Option<String>`
- `in_flight: bool` (주문 진행 중 플래그)
- `state: PositionState` (Opening, Open, Closing, Closed, PartiallyClosedOneLeg, PendingExchangeRecovery)
- `closing_started_at: Option<DateTime<Utc>>` (Closing 전이 시각, kill switch timeout 인수용)
- `db_id: Option<i64>` (DB primary key 참조)
- **호환성 영향**: 필드 추가 시 기존 257개 테스트 영향. `..Default::default()` 패턴 또는 빌더 패턴으로 하위 호환성 유지. **Phase 1-2 작업량을 L→XL로 상향.**

**load 실패 시**: DB 조회 실패 -> **order_id 기반 reconciliation 강제 실행** -> 거래소 실주문/포지션 기반 복원. 복원 불가 시 신규 진입 차단 + 알림.

**파일**: `position_store.rs` (신규), `position.rs`, `monitor.rs`
**규모**: XL

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
      rolling_24h_losses: VecDeque<(DateTime<Utc>, Decimal)>,  // 24h sliding window. 최대 10,000건 제한. 초과 시 oldest pop_front.
      hwm_daily_peaks: VecDeque<(DateTime<Utc>, Decimal)>,     // Rolling 7d HWM (일별 equity 고점). 일일 리셋(KST 00:00) 시 전일 peak_equity를 push_back, 7일 초과 pop_front. record_trade() 시 당일 peak_equity 갱신.
  }
  ```
- **AtomicBool**: `spawned_check_tick_signal`과 이벤트 루프에서 동시 접근 가능 -> lock 없이 `Ordering::Acquire/Release`로 확인/설정
- **일일 리셋**: KST 00:00 (UTC 15:00) 기준으로 고정 (한국 시간 기준 거래일 경계)
- **Drawdown 기준**: Rolling 7d High-Water Mark(HWM) 방식. `hwm_window_days = 7` (config). 최근 7일 내 최고 equity를 HWM으로 사용. 8일 전 고점은 무시. `VecDeque<(DateTime<Utc>, Decimal)>`로 일별 equity 고점 기록. 일일 리셋 시 HWM은 초기화하지 않음 (7일 sliding window로 자동 만료). **Cold start 보호**: 세션 시작 후 첫 24시간 또는 거래 10건 미만일 때는 drawdown kill switch를 비활성화하고, 절대값 한도(`max_drawdown_usdt`)만 적용한다. 이는 초기 소수 거래의 슬리피지/체결 지연으로 인한 조기 kill switch 발동을 방지한다.

- 메서드:
  - `is_killed(&self) -> bool`: AtomicBool 확인 (lock 불필요)
  - `is_entry_allowed(&self) -> bool`: kill switch + 연결 상태 + 진행 중 청산 확인
  - `record_trade(&self, pnl: Decimal)`: 누적 PnL 기록 + rolling_24h_losses 갱신 + kill switch 조건 체크 (일일 한도 OR rolling 24h OR 드로다운 OR 단건)
  - `trigger_kill_switch(&self, reason: &str)`: 강제 발동
  - `check_connection_health(&self, upbit_ok, bybit_ok)`: 한쪽 연결 불안정 시 진입 차단
  - `validate_order_size(&self, size_usdt: Decimal) -> bool`: 단일 주문 크기 상한 확인
  - `check_unrealized_exposure(&self, positions: &[UnrealizedPnlSnapshot]) -> bool`: 전체 미실현 손실 한도 확인. minute_timer에서 매분 호출. 전체 미실현 손실이 `max_unrealized_loss_pct` (config, 기본값 7%)를 초과하면 kill switch 발동. 공식:
    ```
    unrealized_pnl_i = (entry_spread_pct - current_spread_pct) * position_size_usdt / 100
                       - estimated_exit_fees
                       - position_size_usdt * abs(current_usd_krw - entry_usd_krw) / entry_usd_krw
    ```
    스프레드 축소 시 이익(양수), 환율 변동 리스크 항 포함. 부호 규칙: 양수=이익, 음수=손실.

- **리스크 한도 -- 자본 대비 비율(%) + 절대값 이중 적용**:
  ```toml
  # 비율 기반 (자본 규모에 자동 연동)
  max_daily_loss_pct = 10.0        # 자본의 10%
  max_drawdown_pct = 5.0           # 자본의 5% (daily_loss보다 작아야 함)
  max_single_loss_pct = 3.0        # 단건 자본의 3%
  # **측정 시점**: 실현 손실(realized loss) 기준. 진입 시점이 아닌 청산 완료 시 판정.

  # 절대값 기반 (안전망)
  max_daily_loss_usdt = 50.0       # 소액 단계: $50
  max_drawdown_usdt = 25.0         # 소액 단계: $25
  max_single_loss_usdt = 15.0      # 단건 최대 $15
  max_order_size_usdt = 2000.0     # ★ 단일 주문 크기 상한 (버그 방어)
  max_concurrent_positions = 5     # 소액 단계: 5개 (10은 $300에서 min_notional에 걸림)
  max_rolling_24h_loss_usdt = 80.0 # rolling 24h 누적 손실 상한
  max_unrealized_loss_pct = 7.0    # 전체 미실현 손실 한도 (자본의 %, 초과 시 kill switch)

  # 실효 한도 = min(비율 기반, 절대값 기반)
  ```

- **단계별 리스크 한도 참고 테이블**:

  | 자본 | max_daily_loss | max_drawdown | max_single_loss | max_concurrent | max_order_size |
  |------|---------------|-------------|----------------|---------------|---------------|
  | $300 | 10% / $30 | 5% / $15 | 3% / $9 | 3 | $200 |
  | $2,000 | 10% / $200 | 5% / $100 | 3% / $60 | 5 | $500 |
  | $5,000 | 8% / $400 | 4% / $200 | 2% / $100 | 8 | $1,000 |
  | $10,000 | 5% / $500 | 3% / $300 | 2% / $200 | 10 | $2,000 |

- **Lock order**: `RiskManager.inner`는 `position_mgr` lock 해제 후 호출.
  ```
  ob_cache -> instrument_cache -> balance_tracker -> position_mgr -> risk_manager.inner -> trades -> session_writer -> counters -> spread_calc
  ```
  (`is_killed` AtomicBool은 lock order 무관)

- **Kill switch 발동 시 정리 과정**:
  1. `is_killed.store(true, Release)` -- 즉시 신규 진입 차단
  2. 이벤트 루프의 정상 청산 로직 비활성화 (kill switch 전용 청산만 동작)
  3. 활성 포지션 **notional 내림차순** 순차 청산 (rate limit 준수)
  4. 각 포지션: "Closing" 전이 -> 청산 주문 -> 결과 DB 즉시 반영. **Kill switch 청산 결과의 DB 기록이 실패해도 청산 프로세스를 중단하지 않는다. DB 실패 시 결과를 `/tmp/arb_emergency.log` 및 stderr에 기록한다. Triple failure fallback과 동일한 fallback 경로를 사용.**
  5. 전체 완료 조건: `open_count() == 0` AND 거래소 reconciliation 통과
  6. "KILL SWITCH COMPLETE" 텔레그램 발송
  7. **해제**: 수동 확인 + 프로세스 재시작으로만 가능 (자동 해제 없음)
  8. kill switch 청산은 **별도 task로 spawn** (deadlock 방지)
  9. kill switch 재스캔 간격: 1~2초, 최대 30회. in_flight 포지션 완료 대기. **30회 초과 시 해당 포지션을 PendingExchangeRecovery로 전이 + 강제 cancel 시도.**

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
- **치명적 알림** (`KillSwitchTriggered`, `EmergencyCloseFailure`): **★ 청산 먼저, 알림 나중 원칙**: 비상 청산/kill switch 청산 실행 완료 후 알림 전송. 알림 전송 지연(텔레그램 수초~수십초)이 청산을 블로킹하지 않음. 알림은 spawn된 task 내에서 청산 완료 후 `send().await`.
- **텔레그램 실패 fallback**: `alerts` 테이블에 INSERT (DB 기반, 기존 파일 fallback 대체)
- **Triple failure 최종 fallback** (즉시 죽음이 아닌, 최선을 다한 후 종료): 텔레그램 실패 + DB INSERT 실패 시, ① `is_killed.store(true, Release)` 신규 진입 차단 ② kill switch 청산 시도 (활성 포지션 보호) ③ graceful shutdown 시그널 전송 ④ 타임아웃 30초 후에도 미완료 시 `std::process::exit(1)` ⑤ stderr + `/tmp/arb_emergency.log` 파일 기록 (외부 프로세스 모니터링이 감지).

**파일**: `alert.rs` (신규), `monitor.rs`, `config.rs`
**의존**: `arb-telegram`, `arb-db`
**규모**: M

#### 1-5. output/writer.rs, output/summary.rs DB 전환

**문제**: 기존 CSV/JSON 파일 I/O를 DB로 전환.

**변경**:
- `SessionWriter` **trait 추상화**:
  - `trait SessionWriter { fn record_trade(...); fn record_minute(...); ... }`
  - `FileSessionWriter`: 기존 CSV/JSON 파일 출력 구현 (시뮬 example 전용, 기존 코드 유지)
  - `DbSessionWriter`: trades/minutes INSERT, 세션 시작/종료 DB 기록 (라이브 전용)
  - 시뮬 example은 `FileSessionWriter`를, 라이브 `src/main.rs`는 `DbSessionWriter`를 주입
  - 기존 파일 I/O 코드는 `FileSessionWriter`로 이동 (제거하지 않음)
- `SessionSummary`: DB 쿼리로 생성
  - `SELECT COUNT(*) as trade_count, SUM(realized_pnl) as total_pnl, ... FROM trades WHERE session_id = ?`
  - `SELECT COUNT(*) FILTER (WHERE realized_pnl > 0) as win_count, ...` (또는 CASE WHEN)
  - max drawdown, profit factor 등 기존 계산 로직은 Rust 측에서 유지 (복잡한 쿼리 회피)
- 라이브 바이너리에서 파일 관련 코드 제거:
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
- **한쪽 WebSocket만 끊김 시 가격 staleness guard**: 끊긴 거래소의 마지막 가격 timestamp 추적. 가격 age > 30초이면 해당 코인의 시그널 평가 중단 (스프레드 계산 신뢰도 부족). TTL 청산은 유지, Z-Score 시그널만 중단.
- **채널 교체 패턴**: 재연결 + 재구독 완료 후 새 rx를 select! 루프에 atomic swap. 교체 순간 이벤트 유실 방지를 위해 재구독이 완전히 완료된 후 한꺼번에 교체.
- **부분 구독 실패 처리**: 특정 코인 구독 실패 시 해당 코인만 모니터링 제외 + warn 로그.
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
- **잔고 동기화**: `minute_timer`에서 매 5분 **tokio::spawn**으로 `sync_from_exchange()` 실행 (★ REST 호출이므로 select! 내 직접 실행 금지) -- **expected = available + reserved_total**과 실잔고 비교. 5% 이상 괴리 시 warn + 보정 (소액 $300 기준 10%=$30은 너무 넓음). 주문 실패(InsufficientFunds) 시 즉시 sync 트리거.
- **Decimal 안전 연산**: `reserve()`, `on_exit()`에서 `checked_sub()`/`checked_add()` 사용. 음수 잔고 시 warn + 0으로 clamp.
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
- 환율 급변 감지: 전분 대비 **0.2% 이상 변동 시 환율 안정될 때까지 진입 일시 중지** (`forex_stabilization_minutes` (config, 기본값 5)분간 변동 < 0.05% 시 해제). 단순 경고가 아닌 진입 차단.
- `_forex_task` 패닉 시 즉시 환율 staleness 상태 진입 -> 진입 차단
- **NTP 시간 동기화 강화**: Bybit 서버 시간 차이 > 3초 warn, > 5초 **시작 차단** (Bybit API가 서명 거부하므로 warn이 아닌 차단). 기존 '> 5초 warn + 진행'에서 변경.

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
- `BybitClient`: `public_limiter` (10 req/sec) + `private_limiter` (10 req/sec) + `emergency_limiter` (burst 허용) 분리
- 오더북 조회 -> `public_limiter`, 주문/잔고/포지션 -> `private_limiter`
- **Kill switch 청산은 별도 `emergency_limiter`** 사용. 정상 `private_limiter`와 분리하여 burst 허용. Upbit에도 동일하게 `emergency_limiter` 추가.

**파일**: `bybit/client.rs`
**규모**: S

#### 2-3. LiveExecutor 구현 (구체 타입, trait 없음)

**설계 결정**: `OrderExecutor` trait + `SimExecutor` 방식을 사용하지 않는다. 시뮬레이션은 별도 example 바이너리에서 기존 가상 체결 코드를 그대로 사용하고, 라이브는 `src/main.rs`에서 `LiveExecutor<U, B>`를 구체 타입으로 직접 사용한다. 동적 디스패치(`Arc<dyn OrderExecutor>`) 없이 컴파일 타임 모노모피즘으로 hot path 성능을 최적화한다.

**변경**:
- `live_executor.rs` 신규 생성
- `LiveExecutor<U, B>` 제네릭 구조체:
  ```rust
  pub struct LiveExecutor<U, B>
  where
      U: MarketData + OrderManagement + Send + Sync + 'static,
      B: MarketData + OrderManagement + InstrumentDataProvider + Send + Sync + 'static,
  ```
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
  4. Client Order ID 생성: UUID v7 사용 (session 정보 미노출, crash recovery는 DB 매핑으로 해결)
  4.5. **Client Order ID DB 사전 기록**: 진입 주문뿐 아니라 **청산 주문에도** client_order_id(UUID v7)를 사전 기록한다. 진입 시 client_order_id는 Opening INSERT 시 기록. 청산 시 exit_client_order_id를 DB에 기록한 후 주문 발주. crash recovery에서 exit_order_id가 NULL이지만 exit_client_order_id가 있으면 거래소에서 검색 가능. order_id는 체결 확인 후 비동기 write. "DB write 실패 시 주문은 이미 발주됨 → reconciliation에서 보정". order_id 동기 write 실패 시에도 client_order_id로 거래소에서 주문 검색 가능.
  5. 양 레그 동시 발주:
     ```rust
     let result = tokio::time::timeout(Duration::from_secs(10),
         tokio::join!(
             tokio::time::timeout(Duration::from_secs(5), upbit.place_order(buy)),
             tokio::time::timeout(Duration::from_secs(5), bybit.place_order(sell)),
         )
     ).await;
     ```
  6. 체결 대기: **poll_until_filled + Bybit WS execution topic 병행**: Bybit는 WS execution 이벤트를 우선 확인, 미수신 시 REST polling fallback. Upbit는 REST polling (WS 체결 이벤트 미제공). polling 간격 200ms->500ms->1s->2s
  7. 결과 처리:
     - 양쪽 Filled -> `effective_qty = min(upbit_filled_qty * (1 - upbit_fee_rate), bybit_filled_qty)`, 초과분 청산
       - Upbit은 코인 수수료를 수량에서 차감하므로, 실 수령 수량 기준으로 effective_qty를 산출한다.
       - 초과분 청산 비용 -> `adjustment_cost`에 기록 -> 포지션 effective entry price에 반영
       - **Partial fill 초과분 청산 재시도 상한**: 최대 3회, 수렴하지 않으면 잔여 수량 수동 처리 + kill switch
     - 한쪽 Filled + 한쪽 미체결 -> 미체결 쪽 cancel -> 체결된 쪽 비상 청산
     - 양쪽 미체결 -> 양쪽 cancel, 진입 포기
  8. **Post-execution PnL gate 임계치**: 실체결가 기반 스프레드가 roundtrip 수수료 이하이면 즉시 청산. 다만 **스프레드가 음수(역전)가 아니고 수수료의 `post_exec_pnl_gate_ratio` (config, 기본값 0.5 = 50%) 이상이면 보유** (즉시 청산 시 왕복 수수료 + 슬리피지 추가 부담이 더 클 수 있음). 체결가 괴리가 3회 연속 발생하면 해당 코인 모니터링 제외.
  9. 결과 반환: `ExecutedEntry`

- **비상 청산 3단계 escalation**:
  1. **0~2분**: IOC 지정가 재시도 (**각 재시도마다 최신 best bid/ask 조회하여 가격 갱신**). 기준 시각 = **레그 실패 감지 시각**. (지수 백오프 1s, 2s, 4s, 8s...)
  2. **2~5분**: **넓은 IOC 지정가로 전환** + 텔레그램 알림. 가격 소스: REST `get_ticker()` 조회 결과. REST 실패 시 마지막 WS 가격 + `emergency_price_fallback_margin_pct` (config, 기본값 5.0) 마진 적용 (매수 시 +margin%, 매도 시 -margin%). 슬리피지 범위 3단계: 첫 시도 2%, 두 번째 3%, 세 번째 5%. 매 재시도마다 최신 가격 갱신. config: `emergency_wide_ioc_slippage_pct = [2.0, 3.0, 5.0]`
  3. **5분 초과**: **kill switch 강제 발동** + `EmergencyCloseFailure` 알림
  4. 비상 청산 손실은 `RiskManager.record_trade()`에 반드시 포함
  5. kill switch 청산은 **별도 `emergency_limiter`** 사용. 정상 `private_limiter`와 분리하여 burst 허용.

- **Partial fill dust threshold + min_order_qty 처리**:
  - 초과분 < `max_dust_usdt` (config, 기본값 5.0): 즉시 adjustment_cost로 기록하고 포지션에서 제외. 회계 관점에서 명확.
  - 초과분 $5~$50: 3회 재시도
  - 초과분 > $50: 3회 재시도 + 해당 코인 모니터링 제외
  - min_order_qty 미만 잔량: dust로 간주, 포지션 강제 Closed 전이, `adjustment_cost`에 기록
  - Kill switch: **비상 청산 자체가 5분 초과 실패한 경우만** 발동 (dust 잔량은 kill switch 트리거하지 않음)

- **Cancel 실패 처리**: cancel 실패 -> `get_order()` 재확인 -> PartiallyFilled면 실체결 수량으로 effective_qty 조정

- **Computing flag lifetime 확장**: LiveExecutor 사용 시 computing flag를 주문 완료(체결/실패 확정)까지 유지. 주문 진행 중 같은 코인에 새 시그널 spawn 방지.

- **비정상 거래 기록**: `ExecutedEntry`의 `adjustment_cost > 0`이거나 한쪽만 체결 후 비상 청산 시, `trades` 테이블에 `side='adjustment'`로 기록 (정상 포지션과 구분).

**파일**: `live_executor.rs` (신규)
**규모**: XL

---

### Phase 3: monitor.rs 통합

#### 3-1. monitor.rs 분리 + ExecutionPolicy 통합

**설계**: monitor.rs를 3개 파일로 분리하여 시뮬/라이브 코드 중복을 최소화한다.

**파일 구조**:
- `monitor_core.rs`: select! 루프 골격, 캔들 빌딩, 스프레드 계산, 시그널 평가. `ExecutionPolicy` trait 콜백 호출. **candle_builder는 select! 루프의 로컬 변수로 유지한다. ExecutionPolicy 콜백은 Copy 가능한 스냅샷 값(EntryContext, ExitContext)만 받으며, candle_builder에 대한 참조를 전달하지 않는다.**
- `monitor_live.rs`: `LivePolicy` 구현. LiveExecutor, BalanceTracker, RiskManager, DB 연동, reconciliation, 펀딩비.
- `monitor_sim.rs`: `SimPolicy` 구현. 기존 가상 체결 코드, VirtualPosition 즉시 생성.

**ExecutionPolicy trait** (컴파일 타임 디스패치, vtable 없음):
```rust
// RPITIT 패턴 사용 — tokio::spawn 내 호출을 위해 Send 필수.
// 현재 codebase의 MarketData trait 동일 패턴.
pub trait ExecutionPolicy: Send + Sync + 'static {
    fn on_entry_signal(&self, ctx: EntryContext) -> impl Future<Output = Result<(), StrategyError>> + Send;
    fn on_exit_signal(&self, ctx: ExitContext) -> impl Future<Output = Result<(), StrategyError>> + Send;
    /// `TtlExpiryContext`는 owned 스냅샷 타입으로, `EntryContext`/`ExitContext`와 동일한 패턴. `tokio::spawn`의 `'static` 요구사항을 충족한다.
    fn on_ttl_expiry(&self, ctx: TtlExpiryContext) -> impl Future<Output = Result<(), StrategyError>> + Send;
    fn is_entry_allowed(&self) -> bool;
}
```
- `ZScoreMonitor<U, B, P: ExecutionPolicy>`: 제네릭 3개. LivePolicy/SimPolicy 컴파일 타임 선택.
- `src/main.rs`: `ZScoreMonitor<UpbitClient, BybitClient, LivePolicy<UpbitClient, BybitClient>>`
- `examples/zscore_sim.rs`: `ZScoreMonitor<UpbitClient, BybitClient, SimPolicy>`

**진입 시 TOCTOU 해결** (Kill Switch 동시성 설계 참조):
```
[★ 전체가 spawned_check_tick_signal 내에서 실행. select! 루프 비블로킹]
1. risk_manager.is_entry_allowed()           // AtomicBool (lock 없이)
2. balance_tracker.reserve(upbit_krw, bybit_usdt)  // parking_lot::Mutex (< 1μs)
3. position_mgr.lock().await                 // tokio::sync::Mutex
4.   risk_manager.is_killed() 이중 체크       // AtomicBool
4.5. pm.open_count() >= config.max_concurrent 확인 // ★ TOCTOU 방지: pm.lock() 내부에서 확인
5.   pm.register_opening(pos)                // 메모리 Opening 등록
6. position_mgr.unlock()
7. live_executor.execute_entry()            // ★ REST 호출 (spawn 내이므로 select! 비블로킹)
8. position_mgr.lock().await -> 메모리 결과 반영 + DB 비동기 반영
```

**청산 발생 지점 3곳** (전부 select! 루프 외부에서 실행):
1. 정상 Z-Score 시그널 청산 -> `spawned_check_tick_signal` 내 (이미 spawn됨)
2. TTL 만료 청산 -> **tokio::spawn으로 분리** (★ live_executor.execute_exit()가 REST 호출이므로 minute_timer에서 직접 실행 금지)
3. Kill switch 강제 청산 -> 별도 spawn task (이미 명시)
- kill switch 발동 시: 1, 2번 경로 비활성화, 3번만 동작 (이중 청산 방지)
- PositionState "Closing" 전이로 중복 청산 방지

**tokio::spawn panic 방어**: spawn된 task(spawned_check_tick_signal, kill switch 청산 등)의 JoinHandle을 수집하여, panic 발생 시 warn 로그 + 해당 포지션 상태 점검. `std::panic::catch_unwind` 또는 JoinHandle 모니터링.

**파일**: `monitor_core.rs` (신규), `monitor_live.rs` (신규), `monitor_sim.rs` (신규), `monitor.rs` (삭제 또는 re-export)
**규모**: XL

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
- NTP 시간 동기화 확인: Bybit 서버 시간과 로컬 시간 차이 > 3초 warn, > 5초 **시작 차단** (Bybit 서명 거부)
- (Live 모드) **Upbit 마켓 상태 확인**: 선택된 코인의 입출금 상태 조회. 입출금 정지된 코인은 모니터링에서 제외 (비정상 김치 프리미엄 시그널 방지).
- (Live 모드) **Bybit margin mode + crash recovery 상호작용**: 이미 열린 포지션이 있으면 `switch_margin_mode()` 거부됨. 포지션 복원 후 현재 margin mode 확인 -> 이미 Isolated이면 skip.
- (Live 모드) **Graceful shutdown 핸들러**: `tokio::signal::ctrl_c()` + SIGTERM 핸들러 등록. 수신 시 `shutdown_policy` config에 따라 동작:
  - `"keep"` (기본): 신규 진입 즉시 차단 + 현재 진행 중 주문 완료 대기 + 포지션 DB 유지 + 세션 DB 'GracefulStop' 마감. 다음 세션에서 crash recovery 동일 경로로 복원. **systemd restart와 함께 사용 권장.**
  - `"close_all"`: 모든 포지션 비상 청산 후 종료. 유지보수/장기 정지 시 사용.
  - `"close_if_profitable"`: 수익 포지션만 청산 + 손실 포지션은 DB에 유지. 부분 정리용.
  - **외부 watchdog**: sessions 테이블 heartbeat 5분 미갱신 시 별도 프로세스가 kill switch 수행.
- (Live 모드) **외부 프로세스 모니터링**: sessions 테이블 heartbeat (매분 `updated_at` 갱신). **DB heartbeat를 별도 tokio::spawn task에서 실행. select! 루프 blocked 상태도 감지 가능.** 외부 cron/systemd가 heartbeat 5분 미갱신 시 알림.

**파일**: `monitor.rs`
**규모**: M

#### 3-3. 포지션 Reconciliation

**변경**:
- `minute_timer`에서 **tokio::spawn으로 reconciliation task 실행** (★ REST 호출 다수이므로 select! 내 직접 실행 금지)
- 결과는 **단일 `mpsc::channel<BackgroundTaskResult>` enum 채널**로 반환 -> select! 루프에서 결과 수신 후 상태 갱신
  ```rust
  enum BackgroundTaskResult {
      ReconciliationComplete { mismatches: Vec<ReconciliationMismatch> },
      BalanceSynced { upbit_drift: Decimal, bybit_drift: Decimal },
      FundingUpdated { updates: Vec<(String, FundingSchedule)> },
      ReconnectionComplete { exchange: ExchangeName, new_rx: mpsc::Receiver<MarketEvent> },
  }
  ```
- **Bybit**: `get_positions()` -> 내부 `PositionManager` 상태와 비교
- **Upbit**: **order_id 기반** `get_order()` 조회 -> 해당 주문의 체결 수량 확인
  - (잔고 비교는 참고 수준 -- 사용자 개인 보유 코인과 구분 불가)
- 불일치 시:
  - warn 로그 + 텔레그램 `ReconciliationMismatch` 알림
  - **신규 진입 차단** (불일치 해소까지)
  - **자동 수정 하지 않음** -- 불일치 원인이 다양(사용자 수동 거래, 이중 체결, 비상 청산 부분 체결)하므로 자동 수정은 위험. "차단 + 알림 + 수동 확인" 전략.
  - **tolerance band**: 0.1% 이하의 수량 차이는 Decimal precision/수수료 타이밍 차이로 무시.
  - **차단 범위 차등 적용**: 잔고 불일치(Bybit USDT 증거금 부족 등) -> 전체 진입 차단. 특정 코인 포지션 수량 불일치 -> 해당 코인만 차단.
  - **청산은 항상 허용**: 불일치 상태에서도 기존 포지션의 청산은 허용 (진입만 차단).
  - **자동 해소 조건**: 연속 5회 통과 시에만 자동 해제. 직전 10회 reconciliation 중 3회 이상 불일치가 발생했으면 자동 해제 불가 (수동 확인 필요).

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
- 정산 **1시간 전~정산 후 15분**까지 펀딩비가 **불리한 방향**이면 해당 코인만 진입 차단
  - 불리한 방향: 우리 포지션은 Bybit short -> `fundingRate > 0`이면 short가 지불 -> 불리
  - 즉, `current_rate > 0 AND time_to_funding < 60min` -> 해당 코인 진입 차단 (정산 후 15분까지 유지)
  - 반대로 `current_rate < 0`이면 short가 수취 -> 유리 -> 차단 안 함
- **차단 시**: `FundingBlockEntry` 알림 + 카운터 증가

**DB 연동**:
- `funding_schedules` 테이블에 종목별 스케줄 저장 (갱신 시 UPSERT)
- crash recovery 시 DB에서 마지막 스케줄 로드 (API 재조회 없이 즉시 사용)

**실 펀딩비 추적**:
- 정산 시점 전후로 Bybit `getClosedPnl` 또는 `getTransactionLog`에서 실 펀딩비 확인
- `RiskManager.record_trade()`에 펀딩비 포함
- 펀딩비가 포지션 수익 대비 **20% 초과 시 경고**, **50% 초과 시 해당 코인 모니터링 제외**

**기존 포지션 펀딩비 강제 청산 (코인별 차등)**:
- **Major 코인** (BTC, ETH): 정산 `funding_force_close_minutes_major` (기본 15분) 전 강제 청산
- **Alt 코인** (나머지): 정산 `funding_force_close_minutes_alt` (기본 30분) 전 강제 청산
- 24h 거래대금 기준 분류. major 목록은 config에서 설정: `funding_major_coins = ["BTC", "ETH"]`
- 보유 중인 포지션이 불리한 펀딩(rate > 0, short 지불)이면 **해당 포지션만 강제 청산**
- 강제 청산 시 `FundingForceClose` 알림
- config에서 비활성화 가능 (`funding_force_close_enabled = false`)

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
post_exec_pnl_gate_ratio = 0.5   # Post-execution PnL gate: 수수료의 N% 이상이면 보유 (0.5 = 50%)
emergency_wide_ioc_slippage_pct = [2.0, 3.0, 5.0]  # 비상 청산 IOC 슬리피지 단계 (%)
emergency_price_fallback_margin_pct = 5.0  # REST 실패 시 비상 가격 마진 % (매수 +, 매도 -)
max_dust_usdt = 5.0               # Dust threshold: 이하 잔량은 즉시 adjustment_cost로 기록

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
max_rolling_24h_loss_usdt = 80.0   # rolling 24h 누적 손실 상한
hwm_window_days = 7                # HWM drawdown 측정 window (최근 N일 내 최고 equity 기준)

# 환율 (Phase 1-9에서 적용)
max_forex_age_min = 10            # 환율 캐시 최대 수명 (분)
forex_change_alert_pct = 0.2      # 환율 급변 알림 임계치 (%)
forex_stabilization_minutes = 5   # 환율 급변 후 N분간 변동 < 0.05% 시 해제

# 펀딩비
funding_block_before_min = 60      # 정산 N분 전부터 진입 차단
funding_block_after_min = 15       # 정산 후 N분까지 진입 차단
funding_force_close_enabled = true           # 정산 전 불리 포지션 강제 청산
funding_force_close_minutes_major = 15       # Major 코인 (BTC, ETH) 정산 N분 전 강제 청산
funding_force_close_minutes_alt = 30         # Alt 코인 정산 N분 전 강제 청산
funding_major_coins = ["BTC", "ETH"]         # Major 코인 목록
funding_alert_ratio = 0.2         # 펀딩비 > 수익의 20%이면 경고 (기존 0.5 -> 0.2)
funding_exclude_ratio = 0.5        # 펀딩비 > 수익의 50%이면 코인 제외

# 시간대 제한 (P2, 향후)
# trading_hours_utc = "00:00-23:59"  # 거래 허용 시간대 (UTC). 범위 외 시 진입 차단.

# DB (db_url은 strategy.toml에 포함하지 않음 — DATABASE_URL 환경변수에서만 읽기)
# db_url = "mysql://..." → 환경변수 DATABASE_URL 전용

# PendingExchangeRecovery
pending_recovery_timeout_hours = 2  # 최대 체류 시간 (초과 시 수동 처리 알림 + 잔고 예약 해제)

# Graceful shutdown
shutdown_policy = "keep"          # "keep" | "close_all" | "close_if_profitable" (기본값 "keep")

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

#### 4-3. 프로덕션 로깅

**변경**:
- `tracing-appender` RollingFileAppender 일별 로테이션
- 프로덕션 기본 로그 레벨: `info` (debug는 환경변수로 활성화)
- 로그 파일 경로: config 또는 환경변수로 지정

**파일**: `logging/mod.rs`
**규모**: S

#### 4-4. (Phase 1-1로 이동됨)

~~API 키 보안 강화~~ → Phase 1-1에서 처리 (실거래 시작 전 필수).

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
| `crates/arb-strategy/src/zscore/execution_policy.rs` | 3-1 | ExecutionPolicy trait + SimPolicy + LivePolicy |
| `crates/arb-strategy/src/zscore/monitor_core.rs` | 3-1 | select! 루프 골격, 캔들, 시그널 평가 |
| `crates/arb-strategy/src/zscore/monitor_live.rs` | 3-1 | LivePolicy 구현 (LiveExecutor, DB, 잔고, 리스크) |
| `crates/arb-strategy/src/zscore/monitor_sim.rs` | 3-1 | SimPolicy 구현 (기존 가상 체결) |
| `examples/zscore_sim.rs` | 3-1 | 시뮬레이션 example (기존 가상 체결 코드 분리) |

### 변경 파일

| 파일 | Phase | 규모 | 핵심 변경 |
|------|-------|------|----------|
| `Cargo.toml` (root) | 1-0 | S | workspace members에 `arb-db` 추가 |
| `monitor.rs` | 1,3 | XL | 삭제 또는 re-export (monitor_core/live/sim으로 분리). unwrap 제거, 재연결, TOCTOU 이중 체크, 초기화/reconciliation, 환율 guard, 잔고 연동, 펀딩비 |
| `position.rs` | 1-2 | XL | Serialize/Deserialize, PositionState(6종), order_id, db_id, closing_started_at, 빌더 패턴 |
| `config.rs` | 1,4 | M | 리스크 한도(비율+절대값), 주문/환율/펀딩/DB 파라미터 |
| `output/writer.rs` | 1-5 | L | SessionWriter trait 추상화, FileSessionWriter + DbSessionWriter |
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
| `arb-strategy/Cargo.toml` | 1 | S | `parking_lot`, `arb-db = { ..., optional = true }` 추가. `features = ["live"]`로 라이브 인프라 활성화. 시뮬레이션 example은 live feature 없이 빌드. |
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
| C10 | order_id write 실패 + crash | orphan order 발생 | client_order_id 사전 기록 + 거래소 검색 fallback | 1-0, 2-3 |
| C11 | ReservationToken leak | 잔고 영구 차감 | RAII Drop impl + TTL sweeper | 1-8 |

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
| H17 | 치명적 알림이 청산 블로킹 | naked exposure 연장 | 청산 먼저, 알림 나중 | 1-4 |
| H18 | Closing exit_order_id 미추적 | crash recovery 불완전 | exit_order_id 컬럼 추가 | 1-0, 1-2 |
| H19 | WebSocket 한쪽 끊김 stale 시그널 | 잘못된 청산 | 가격 age > 30초 시 시그널 중단 | 1-7 |
| H20 | Opening in_flight kill switch 우회 | 포지션 방치 | in_flight 플래그 + 재스캔 | 1-3, 3-1 |

### Medium

| ID | 항목 | 대응 | Phase |
|----|------|------|-------|
| M1 | Box\<dyn Error\> erased | StrategyError 전환 | 1-6 |
| M2 | forex_task JoinHandle 무시 | 모니터링 + 재시작 | 1-7 |
| M3 | API 키 평문 메모리 | Debug 마스킹, 향후 secrecy | **1-1** (승격) |
| M4 | is_retryable 없음 | ExchangeError에 추가 | 1-6 |
| M5 | Bybit category "spot" 하드코딩 | place_order_linear() 추가 | 2-1 |
| M6 | 중복 주문 위험 | Client Order ID | 2-4 |
| M7 | 거래소 점검/입출금 정지 | 상태 API 조회 + 진입 차단 | 3-2 |
| M8 | Bybit 강제 청산/ADL 실시간 감지 | Bybit WS `/v5/private/position` 토픽 `adlRankIndicator` 모니터링. ADL 발생 시 즉시 해당 코인 Upbit 초과분 매도하여 수량 일치. Reconciliation에서도 ADL로 인한 수량 변동 감지. | 2-3, 3-3 |
| M9 | 펀딩비 종목별 주기 미추적 | 종목별 FundingSchedule + 매분 갱신 | 3-4 |
| M10 | computing flag lifetime | 주문 완료까지 확장 | 2-4 |
| M11 | Partial fill 초과분 무한 연쇄 | 최대 3회 재시도 + kill switch | 2-4 |
| M12 | 체결가 괴리 | Post-execution PnL gate | 2-4 |
| M13 | 마켓 임팩트 | 기존 safe_volume + IOC 슬리피지 제어 | 2-4 |
| M14 | 파일 I/O 잔류 코드 | output/writer.rs, summary.rs DB 전환 | 1-5 |
| M15 | monitor.rs 4000줄+ 예상 | 유지보수 불가 | monitor_core/live/sim 분리 | 3-1 |
| M16 | Triple failure 알림 불가 | 운영 블라인드 스팟 | stderr + 파일 + exit code | 1-4 |
| M17 | DB write 순서 미보장 | 정합성 깨짐 | 단일 mpsc 채널 직렬화 | 1-0 |

---

## 단계적 롤아웃 계획

```
사전조건:
  - Bybit 서브계정 생성 + API 키 발급
  - MySQL DB 준비 + 마이그레이션 완료

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
     │              ├── monitor_core/live/sim 분리 + ExecutionPolicy trait
     │              ├── LiveExecutor 직접 통합 + TOCTOU 해결
     │              ├── src/main.rs 라이브 entry point + examples/zscore_sim.rs 시뮬 분리
     │              ├── 시작 초기화 (잔고+포지션 복원+IOC 검증+DB 확인)
     │              ├── Reconciliation (차단+알림, 자동 수정 없음)
     │              └── 종목별 펀딩비 모니터링 (FundingSchedule + 매분 갱신)
     │
Phase 4 ───────── 설정 및 부가 기능
     │              ├── Config 확장 (비율+절대값 이중, 펀딩비, shutdown_policy 등)
     │              ├── 실수수료 + 펀딩비 + adjustment_cost PnL
     │              ├── 프로덕션 로깅 (tracing-appender 일별 로테이션)
     │              └── API 키 보안 강화 → Phase 1-1로 승격 완료
     │
     ▼
검증 단계 (각 단계별 KPI):
  ① 소액 실거래 ($300~500, max_concurrent=3)
     KPI: **최소 72시간 + 50건 이상** 성공, 레그 실패 시 **비상 청산 100% 성공**
          API 성공률 > 99%, 24h 무크래시
          양 레그 체결 시간 차이 p95 < 2s
          슬리피지 p95 < 0.1%
          partial fill 발생률 < 5%
          Sim 병행 실행 -> PnL 오차 분해(slippage/fee/timing)
          잔고 추적 오차 < 1%
          DB 기록 정합성 100%, **ReservationToken leak 0건**
          **시간대별 성과 분석** (한국 주간/미국 주간/주말)
  ② 중액 실거래 ($1,000~**2,000**, max_concurrent=8)
     KPI: 시간당 거래 안정, kill switch 테스트 통과, 24h 무중단
          reconciliation 불일치 0건
          펀딩비 진입 차단 정상 동작
  ②-b 중간 단계 ($2,000~5,000, max_concurrent=8)
     KPI: 48h 무중단, 수익률 > 0, 슬리피지 안정화 확인
  ③ 풀자본 ($10,000, max_concurrent=10)
     KPI: 72h 무중단, 수익률 > 0 AND 시뮬 대비 50%+

**Rollback 조건**: 각 단계에서 KPI 미달 시 이전 단계로 복귀. 구체적으로:
  - 24h 내 kill switch 2회 이상 발동
  - 비상 청산 실패 1건 이상
  - reconciliation 불일치 연속 3회
  - 수익률이 **동시 실행 시뮬 대비 30% 미만** (과거 시뮬이 아닌 병행 실행 시뮬 기준)
  -> 자본 50% 감축 + 원인 분석 후 재진입
```

---

## 체크리스트

### Phase 1: 안전 인프라

**1-0. arb-db 모듈**
- [ ] `crates/arb-db/` 디렉토리 및 `Cargo.toml` 생성
- [ ] `sqlx` 의존성 추가 (mysql, runtime-tokio, chrono, rust_decimal)
- [ ] `sessions` 테이블 마이그레이션 작성
- [ ] `positions` 테이블 마이그레이션 작성 (상태 머신 6종: Opening/Open/Closing/Closed/PartiallyClosedOneLeg/PendingExchangeRecovery, order_id, succeeded_leg)
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
- [ ] `arb-strategy/Cargo.toml`에 `arb-db = { ..., optional = true }`, `features = ["live"]`로 라이브 인프라 활성화
- [ ] positions 테이블에 `exit_upbit_order_id`, `exit_bybit_order_id`, `client_order_id`, `exit_client_order_id`, `in_flight` 컬럼
- [ ] positions 테이블 qty를 `upbit_qty`, `bybit_qty` 양 레그 분리 (수수료 차감/dust 미세 차이 추적)
- [ ] Background DB Writer: mpsc(256) 단일 consumer, 전체 직렬 순서, try_send 실패 시 newest 드랍 + overflow log + dirty flag, 재시도 3회
- [ ] DB 커넥션 풀 설정: max=10, min=2, acquire_timeout=5s
- [ ] MySQL DDL 마이그레이션: 파일당 하나의 DDL만
- [ ] Crash recovery: client_order_id로 거래소 orphan order 검색
- [ ] Crash recovery: Opening 양 레그 비대칭 처리 (한쪽만 체결 시 PartiallyClosedOneLeg)
- [ ] Crash recovery: Closing exit_order_id 양 레그 개별 확인
- [ ] Session ID 연속성: 이전 세션 Crashed 마감 + 새 session_id (parent_session_id로 참조)
- [ ] sessions 테이블에 `parent_session_id BIGINT UNSIGNED NULL` 컬럼
- [ ] sessions.config_json 저장 시 민감 필드(db_url, api_key 등) redact
- [ ] `db_url`은 `DATABASE_URL` 환경변수에서만 읽기 (strategy.toml에 미포함)
- [ ] Upbit client_order_id 사전 검증: `/v1/orders/chance` API에서 client_order_id 기반 검색 지원 여부 확인
- [ ] Client Order ID: UUID v7 형식
- [ ] SessionWriter trait: FileSessionWriter + DbSessionWriter 추상화
- [ ] Triple failure fallback: kill switch → graceful shutdown → 30초 timeout → exit(1) + stderr + /tmp/arb_emergency.log

**1-1. unwrap 패닉 제거**
- [ ] `parking_lot` 크레이트 추가, `std::sync::Mutex` -> `parking_lot::Mutex` 전환 (monitor.rs, orderbook.rs, coin_selector.rs)
- [ ] `instrument_cache` `std::sync::RwLock` -> `parking_lot::RwLock` 전환 (monitor.rs 4곳)
- [ ] `tokio::sync::Mutex`/`RwLock`은 변경하지 않음 확인
- [ ] `SystemTime::expect()` -> `unwrap_or_default()` + warn 로그 (bybit/auth.rs + bithumb/auth.rs)
- [ ] coin_selector.rs, spread.rs의 `.expect()`는 `#[cfg(test)]` 한정 -- 프로덕션 영향 없음 확인
- [ ] `Decimal::ZERO` fallback 시 즉시 return 가드 (monitor.rs)
- [ ] API 키 Debug 마스킹: `BybitCredentials`, `UpbitCredentials`에 수동 `Debug` impl (키 마스킹)

**1-2. 포지션 영속화 (Dual-State: 메모리 + DB)**
- [ ] `position_store.rs` 생성: `PositionStore` trait 정의
- [ ] `DbPositionStore` 구현 (arb-db 연동)
- [ ] Dual-State 설계: 메모리(PositionManager) = authoritative, DB = async shadow
- [ ] client_order_id 사전 기록 동기 DB write (Opening INSERT 시), 청산 시 exit_client_order_id도 사전 기록. order_id는 체결 후 비동기 write
- [ ] client_order_id DB INSERT 실패 시 해당 진입 차단 (주문 발주 거부)
- [ ] 상태 전이 3단계: DB 동기(Opening + client_order_id) -> 주문 발주 -> DB 비동기(order_id) -> 메모리 Open + DB 비동기
- [ ] `VirtualPosition`에 `Serialize/Deserialize`, `PositionState`, `order_id`, `db_id` 추가 + 빌더 패턴
- [ ] 포지션 상태 머신: Opening -> Open -> Closing -> Closed + PartiallyClosedOneLeg + PendingExchangeRecovery (6종)
- [ ] `closing_started_at: Option<DateTime<Utc>>` 필드 추가
- [ ] Crash recovery: Opening(order_id) -> get_order() 조회, Closing -> 청산 주문 확인
- [ ] `load_open()` 실패 시 order_id 기반 reconciliation 강제 + 진입 차단

**1-3. Kill Switch + Risk Manager**
- [ ] `risk.rs` 생성: `RiskManager` + AtomicBool kill switch
- [ ] 리스크 한도: 비율(%) + 절대값 이중, `max_order_size_usdt` 추가
- [ ] kill switch TOCTOU: pm 락 내 이중 체크
- [ ] kill switch 청산: 별도 task spawn, notional 내림차순, 정상 청산 비활성화, 포지션 0 재스캔
- [ ] kill switch COMPLETE 조건: open_count==0 AND reconciliation 통과
- [ ] RiskManager 일일 리셋: KST 00:00 (UTC 15:00) 기준
- [ ] lock order 갱신: `ob_cache -> instrument_cache -> balance_tracker -> position_mgr -> risk_manager.inner -> trades -> session_writer -> counters -> spread_calc`
- [ ] Rolling 24h 누적 손실 한도 (VecDeque sliding window)
- [ ] Drawdown HWM rolling 7d 방식 (`hwm_window_days = 7`, `VecDeque<(DateTime, Decimal)>` 일별 고점, cold start 24h/10건 미만 시 비율 kill switch 비활성화)
- [ ] Rolling loss VecDeque 상한: 최대 10,000건, 초과 시 oldest pop_front
- [ ] `check_unrealized_exposure()`: 전체 미실현 손실 > `max_unrealized_loss_pct` 시 kill switch
- [ ] Opening in_flight 플래그 + kill switch 재스캔 (1~2초, 최대 30회, 초과 시 PendingExchangeRecovery 전이)
- [ ] Kill switch 청산 중 DB 장애 시 청산 계속 + `/tmp/arb_emergency.log` fallback
- [ ] Closing timeout 인수: `state == Closing && (now - closing_started_at) > 15s` → kill switch 인수 + CancellationToken 전송
- [ ] `try_transition_to_closing(pos_id) -> bool` 메서드 추가

**1-4. AlertService**
- [ ] `alert.rs` 생성: 일반 알림 mpsc(64) try_send, 치명적 알림 동기 전송
- [ ] 텔레그램 실패 시 `alerts` 테이블에 INSERT (DB fallback)
- [ ] 알림 이벤트 타입 13종 구현 (DbConnectionLost, FundingBlockEntry 추가)
- [ ] "청산 먼저, 알림 나중" 원칙 적용 (알림이 청산 블로킹 금지)
- [ ] Triple failure: 텔레그램 + DB 모두 실패 시 kill switch → graceful → 30초 timeout → exit(1)

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
- [ ] 한쪽 WebSocket 끊김 시 가격 age > 30초이면 해당 코인 시그널 중단
- [ ] 채널 교체: 재구독 완료 후 atomic swap
- [ ] 부분 구독 실패 시 해당 코인만 제외

**1-8. BalanceTracker**
- [ ] `balance.rs` 생성: `BalanceTracker` (reserve/commit/release/on_exit/sync)
- [ ] 시작 시 잔고 초기화 (Upbit KRW + Bybit USDT)
- [ ] 동시 진입 잔고 경합 방지 (reservation 패턴)
- [ ] 시뮬레이션 example은 BalanceTracker 미사용 확인
- [ ] ReservationToken RAII: Drop impl + committed 플래그
- [ ] ReservationToken TTL: 6분 sweeper (비상 청산 5분 + 여유 1분)
- [ ] 양 거래소 원자적 예약 (단일 Mutex, 롤백)
- [ ] sync_from_exchange: expected = available + reserved_total 비교, in_flight 예약 있으면 해당 주기 스킵
- [ ] 괴리 임계값 5% (소액 기준), InsufficientFunds 시 즉시 sync
- [ ] Decimal checked_sub/checked_add, 음수 clamp
- [ ] BalanceTracker 동시성 테스트: `reserve()` 100회 동시 호출 → 총 예약 ≤ 초기 잔고
- [ ] BalanceTracker 동시성 테스트: `reserve() + commit()`과 `reserve() + release()` 교차 → 잔고 정합성
- [ ] BalanceTracker 동시성 테스트: ReservationToken Drop → 잔고 복원
- [ ] BalanceTracker 동시성 테스트: TTL sweeper 미확정 토큰 해제

**1-9. 환율 guard**
- [ ] 환율 staleness guard: 캐시 age > 10분 시 진입 차단, 급변 > 0.2% 시 알림
- [ ] 환율 0.2% 급변 시 진입 일시 중지 (안정 시 해제)
- [ ] NTP > 3초 warn, > 5초 시작 차단

**Phase 1 완료 조건**
- [ ] `cargo test -p arb-strategy` 전체 통과
- [ ] `cargo test -p arb-db` 전체 통과
- [ ] `cargo clippy` 경고 0

### Phase 2: 주문 실행 엔진
- [ ] Bybit 선물 API: `place_order_linear()`, `set_leverage()`, `switch_margin_mode()`, `get_positions()`
- [ ] margin_mode 변경 실패 -> 현재 모드 확인 -> Cross면 시작 차단
- [ ] `PositionInfo` 타입 추가
- [ ] Bybit rate limiter `public_limiter` / `private_limiter` / `emergency_limiter` 분리
- [ ] Upbit에도 `emergency_limiter` 추가
- [ ] `live_executor.rs` 생성: `LiveExecutor<U, B>` 구체 타입 (trait/dyn 없음)
- [ ] `EntryRequest`/`ExitRequest`/`ExecutedEntry`/`ExecutedExit` 타입 정의
- [ ] `OrderExecutionError` enum (Timeout에 leg/order_id/other_leg 상세)
- [ ] `ExecutedEntry`에 `adjustment_cost` 필드
- [ ] 양 레그 모두 IOC 지정가 (Upbit + Bybit)
- [ ] `tokio::time::timeout` 개별(5s) + 전체(10s) 이중 timeout
- [ ] Client Order ID 멱등성 (UUID v7)
- [ ] 비상 청산 3단계: IOC(0~2분) -> 시장가(2~5분) -> kill switch(5분+)
- [ ] Cancel 실패 -> get_order -> PartiallyFilled 수량 조정
- [ ] Partial fill 초과분 청산 최대 3회 재시도, 수렴 안 하면 kill switch
- [ ] Post-execution PnL gate
- [ ] Computing flag lifetime -> 주문 완료까지 확장
- [ ] 비정상 거래: `trades` 테이블에 `side='adjustment'`로 기록
- [ ] 비상 청산 손실 -> RiskManager.record_trade() 포함
- [ ] `RiskManager.validate_order_size()` 발주 전 확인
- [ ] 단위 테스트: 양쪽 성공/한쪽 실패/타임아웃/partial fill
- [ ] Upbit `/v1/orders/chance` API로 마켓별 order_types 확인. IOC 미지원 시 GTC fallback 자동 선택.
- [ ] Bybit WS execution topic 체결 확인 (REST polling 병행)
- [ ] Bybit WS `/v5/private/position` 토픽 구독. 포지션 외부 변경(강제 청산, ADL) 즉시 감지. execution topic과 함께 private WS 채널에 구독.
- [ ] Client Order ID: UUID v7 (session 정보 미노출, DB 매핑으로 해결)
- [ ] Client Order ID DB 사전 기록 (진입: Opening INSERT 시, 청산: exit_client_order_id 사전 기록)
- [ ] 비상 청산 IOC 재시도: 매 재시도마다 최신 가격 갱신
- [ ] 비상 청산 넓은 IOC 3단계: 2% → 3% → 5% (`emergency_wide_ioc_slippage_pct` config)
- [ ] 비상 청산 타이머 기준점 = 레그 실패 감지 시각
- [ ] Post-execution PnL gate 임계치: 수수료 이하 -> 즉시 청산, `post_exec_pnl_gate_ratio` 이상 -> 보유
- [ ] Partial fill dust threshold: < `max_dust_usdt` 즉시 adjustment_cost 기록, $5~$50 3회 재시도, >$50 3회 재시도+코인 제외
- [ ] min_order_qty 미만 잔량: dust → 포지션 강제 Closed, adjustment_cost 기록
- [ ] Upbit 시장가 매수 KRW 금액 기준 (수수료 별도)
- [ ] `cargo test` 전체 통과 + `cargo clippy` 경고 0
- [ ] Bybit testnet E2E 테스트: testnet 환경에서 LiveExecutor 양 레그 주문/체결/비상 청산 전체 흐름 검증

### Phase 3: monitor.rs 통합 + 바이너리 분리

**3-1. monitor.rs 분리 + ExecutionPolicy**
- [ ] `monitor_core.rs`: select! 골격, 캔들, 시그널 평가
- [ ] `monitor_live.rs`: LivePolicy 구현 (LiveExecutor, DB, 잔고, 리스크)
- [ ] `monitor_sim.rs`: SimPolicy 구현 (기존 가상 체결)
- [ ] `ExecutionPolicy` trait: on_entry_signal, on_exit_signal, on_ttl_expiry, is_entry_allowed
- [ ] `ZScoreMonitor<U, B, P: ExecutionPolicy>` 제네릭 3개
- [ ] tokio::spawn panic 방어: JoinHandle 모니터링
- [ ] select! iteration latency 자가 진단 (10ms warn)
- [ ] `src/main.rs`: 라이브 전용 entry point (DB, BalanceTracker, RiskManager, AlertService, LiveExecutor 전부 wiring)
- [ ] `examples/zscore_sim.rs`: 기존 시뮬레이션 코드 분리 (라이브 인프라 의존 없음)
- [ ] 진입 TOCTOU: reserve -> pm.lock -> is_killed 이중 체크 -> max_concurrent 확인 (pm.lock 내부) -> register_opening -> unlock -> execute
- [ ] 청산 3곳 전부 select! 외부(spawn) 실행 확인: ① spawned_check_tick_signal, ② TTL spawn, ③ kill switch spawn
- [ ] 청산 3곳: kill switch 시 1,2번 비활성화, PositionState "Closing" 중복 방지

**3-2. 시작 시 초기화**
- [ ] 시작 시 Bybit 설정 검증 (leverage, margin_mode 실패 처리)
- [ ] 시작 시 BalanceTracker 초기화 + config.total_capital_usdt 조정
- [ ] 시작 시 DB 연결 확인
- [ ] 시작 시 Upbit IOC 지원 여부 사전 검증
- [ ] 미청산 포지션 복원 (DB load_open + order_id 기반 get_order() 조회)
- [ ] NTP > 3초 warn, > 5초 시작 차단
- [ ] Upbit 마켓 상태 확인 (입출금 정지 코인 제외)
- [ ] Bybit margin mode + crash recovery 상호작용
- [ ] Graceful shutdown: SIGTERM -> `shutdown_policy` config에 따라 동작 ("keep"/"close_all"/"close_if_profitable")
- [ ] 외부 watchdog: heartbeat 5분 미갱신 시 별도 프로세스가 kill switch 수행
- [ ] 외부 모니터링: sessions 테이블 heartbeat (매분, 별도 tokio::spawn task에서 실행)

**3-3. Reconciliation**
- [ ] `minute_timer` REST 작업 전부 tokio::spawn으로 분리 (★ select! 비블로킹 확인)
- [ ] reconciliation: spawn + 결과 mpsc/Arc 반환, order_id 기반, 자동 수정 없음, 불일치 시 진입 차단
- [ ] 잔고 동기화: spawn + 매 5분 sync_from_exchange()
- [ ] Reconciliation tolerance band (0.1%)
- [ ] 차단 범위: 잔고 불일치 -> 전체, 코인 불일치 -> 해당 코인만
- [ ] 불일치 상태에서 청산 허용 (진입만 차단)
- [ ] 연속 5회 reconciliation 통과 시 자동 해제 (직전 10회 중 3회 이상 불일치 시 자동 해제 불가)
- [ ] ADL 감지: Bybit WS `/v5/private/position` `adlRankIndicator` 모니터링. ADL 발생 시 Upbit 초과분 매도.
- [ ] PendingExchangeRecovery: 거래소 장애 시 해당 leg 포지션 전이, 복구 후 자동 청산
- [ ] PendingExchangeRecovery: 30초마다 health check, 2시간 초과 시 수동 처리 알림 + 잔고 예약 해제
- [ ] Closing → PendingExchangeRecovery 직접 전이 (양쪽 거래소 장애 시)

**3-4. 펀딩비**
- [ ] 종목별 펀딩비: 코인 선택 시 FundingSchedule 초기 조회
- [ ] 펀딩비: spawn + 매분 getTickers -> fundingRate/nextFundingTime 갱신
- [ ] 펀딩비 진입 차단: 정산 1시간 전~정산 후 15분
- [ ] 기존 포지션 펀딩비 강제 청산: Major 코인 15분 전, Alt 코인 30분 전 + 불리 시 (`funding_major_coins` config)
- [ ] 펀딩비 경고 20%, 모니터링 제외 50%
- [ ] 펀딩비: DB funding_schedules 테이블 UPSERT

**Phase 3 공통**
- [ ] 시뮬 example이 기존 동작 100% 유지 확인 (`cargo run --example zscore_sim`)
- [ ] 통합 테스트: LiveExecutor + PositionStore + RiskManager + BalanceTracker end-to-end
- [ ] Crash recovery 테스트: 8개 시나리오 (Opening/order_id NULL, Opening/한쪽만, Opening/양쪽+한쪽Filled, Open, Closing/한쪽Filled, PartiallyClosedOneLeg, DB write 실패, Session ID cross-session)
- [ ] RiskManager 테스트 (비율+절대값 한도, TOCTOU, kill switch 재스캔)
- [ ] RiskManager 일일 리셋 시나리오 테스트: 23:50 KST -$40 → 00:00 리셋 → 00:10 -$20 → 일일 $20, rolling 24h $60
- [ ] HWM rolling 7d 경계값 테스트: 7일 전 고점 만료, 6일 전 고점 유지 확인
- [ ] `cargo test` 전체 통과 + `cargo clippy` 경고 0

### Phase 4: 설정 및 부가 기능
- [ ] `config.rs`에 라이브 전용 설정 추가 (order_timeout_sec, max_slippage_pct, 환율, 펀딩비, db_url 등)
- [ ] rolling_24h_loss_usdt config 추가
- [ ] 펀딩 강제청산 config (enabled, minutes_major/minutes_alt, major_coins, alert/exclude ratio)
- [ ] `ClosedPosition`에 `actual_fees`, `funding_fee`, `adjustment_cost` 추가
- [ ] Order.paid_fee 기반 실수수료 PnL 계산
- [ ] API 키 Debug impl 마스킹 → Phase 1-1로 이동 완료
- [ ] `shutdown_policy` config ("keep"/"close_all"/"close_if_profitable")
- [ ] `max_unrealized_loss_pct` config (기본값 7%)
- [ ] `hwm_window_days` config (기본값 7)
- [ ] `post_exec_pnl_gate_ratio` config (기본값 0.5)
- [ ] `emergency_wide_ioc_slippage_pct` config ([2.0, 3.0, 5.0])
- [ ] `max_dust_usdt` config (기본값 5.0)
- [ ] `emergency_price_fallback_margin_pct` config (기본값 5.0)
- [ ] `pending_recovery_timeout_hours` config (기본값 2)
- [ ] `forex_stabilization_minutes` config (기본값 5)
- [ ] `funding_force_close_minutes_major` / `funding_force_close_minutes_alt` config
- [ ] `funding_major_coins` config (["BTC", "ETH"])
- [ ] tracing-appender RollingFileAppender 일별 로테이션
- [ ] trades 테이블 `exit_usd_krw` 컬럼 (세금 산정용)
- [ ] `cargo test` 전체 통과 + `cargo clippy` 경고 0

### 검증
- [ ] 소액 ($300-500, max_concurrent=3): 최소 72시간 + 50건 이상 성공, 비상 청산 100% 성공, API 성공률 > 99%, 24h 무크래시, 체결 latency p95 < 2s, partial fill < 5%, Sim 병행 PnL 오차 분해, 잔고 오차 < 1%, DB 기록 정합성 100%, ReservationToken leak 0건, 시간대별 성과 분석
- [ ] 중액 ($1,000-2,000): 24h 무중단, kill switch 통과, reconciliation 불일치 0건, 펀딩비 진입 차단 정상 동작
- [ ] 중간 ($2,000-5,000): 48h 무중단, 수익률 > 0, 슬리피지 안정화 확인
- [ ] 풀자본 ($10,000): 72h 무중단, 수익률 > 0 AND 시뮬 대비 50%+
