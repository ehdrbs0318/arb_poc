# 0007 — 이벤트 루프 비동기 분리 (Event Loop Async Decoupling)

## 목적

`select!` 루프에서 REST 호출(오더북 조회, 코인 재선택 워밍업)이 블로킹하여 채널 오버플로우가 발생하는 문제를 해결한다. `check_tick_signal`과 코인 재선택을 `tokio::spawn`으로 분리하여 select! 루프가 이벤트를 중단 없이 소비하도록 한다.

## 배경

### 현재 문제점

`monitor.rs`의 `select!` 루프는 **싱글 태스크 순차 실행** 구조다:

```
select! 루프:
  event 수신 → handle_event().await → check_tick_signal().await
                                          ↓
                                     get_orderbook().await (REST ~50~200ms)
                                          ↓
                                     (select! 루프 정지 → 채널 소비 중단)
```

- WebSocket에서 5개 코인 × 초당 ~10 이벤트 = 초당 ~50 이벤트 유입
- REST 대기 동안 select! 루프가 멈추면 채널에 이벤트가 누적
- 코인 재선택 워밍업(코인당 REST 다수) + 대량 청산(포지션당 오더북 REST)이 겹치면 수분간 정체
- `channel_buffer_size` = 10,000 초과 시 이벤트 드롭 (WARN 로그)

### computing flag 무효 문제

현재 `OrderBookCache`의 computing flag는 동시성 환경에서만 의미 있다. select! 루프가 싱글 태스크 순차 실행이므로 REST `.await` 중에 다른 이벤트가 `is_computing()` 을 체크할 기회가 없어 flag가 실질적으로 무효이다.

### 영향 범위

Upbit/Bybit **양쪽 모두** 동일한 구조로, 한쪽에서 REST 블로킹이 발생하면 **양쪽 채널 모두** 소비가 중단된다.

## 요구사항

### 0. ZScoreMonitor 구조체 변경 (전제 조건)

`tokio::spawn`은 `'static` lifetime을 요구하므로, `ZScoreMonitor`의 필드를 `Arc`로 감싸야 한다.

**변경 전:**
```rust
pub struct ZScoreMonitor<U: MarketData + MarketStream, B: MarketData + MarketStream> {
    upbit: U,
    bybit: B,
    config: ZScoreConfig,
    forex_cache: Arc<ForexCache>,
}
```

**변경 후:**
```rust
pub struct ZScoreMonitor<U, B>
where
    U: MarketData + MarketStream + Send + Sync + 'static,
    B: MarketData + MarketStream + Send + Sync + 'static,
{
    upbit: Arc<U>,
    bybit: Arc<B>,
    config: Arc<ZScoreConfig>,
    forex_cache: Arc<ForexCache>,
}
```

- `MarketData` trait은 RPITIT 패턴으로 object-safe하지 않으므로 `Arc<dyn MarketData>` 불가 → 제네릭 `Arc<U>` 유지
- `new()` 시그니처는 기존 `U`, `B` 값을 받아 내부에서 `Arc::new()`로 감싸도록 유지하여 하위 호환

### 1. check_tick_signal 비동기 분리

| 항목 | 설정 |
|------|------|
| spawn 범위 | `check_tick_signal` 전체를 `tokio::spawn`으로 분리 |
| 동시성 제어 | 코인당 최대 1개 task만 실행 (computing flag via Arc) |
| 결과 처리 | task 내부에서 직접 처리 (포지션 진입/청산, CSV 기록) |
| 에러 처리 | task 내부에서 warn 로그 + 무시 (메인 루프에 전파하지 않음) |

**변경 전:**
```rust
// select! 루프
Some(event) = upbit_rx.recv() => {
    Self::handle_event(..., &mut ob_cache, &mut counters, ...).await?;
    // ↑ check_tick_signal 내부에서 get_orderbook().await → 전체 루프 블로킹
}
```

**변경 후:**
```rust
// select! 루프
Some(event) = upbit_rx.recv() => {
    // 캔들 업데이트 등 가벼운 작업은 동기 처리
    // check_tick_signal은 tokio::spawn으로 분리 → 즉시 다음 이벤트 소비
}
```

### 2. 코인 재선택 비동기 분리

| 항목 | 설정 |
|------|------|
| spawn 범위 | 재선택 전체 (CoinSelector::select + 워밍업 + 구독 교체) |
| 재선택 중 동작 | 기존 코인 리스트로 이벤트 소비 및 시그널 평가 계속 |
| 완료 시 | current_coins 업데이트 + 스트림 구독 교체 |

**변경 전:**
```rust
// select! 루프 - 재선택 브랜치
_ = reselect_timer.tick() => {
    let candidates = selector.select(...).await;           // REST 다수 호출
    for coin in diff.to_add {
        self.warmup_single_coin(coin, ...).await;          // REST 다수 호출
        self.upbit.subscribe_markets(...).await;            // WebSocket 명령
        self.bybit.subscribe_markets(...).await;
        upbit.get_orderbook(...).await;                     // REST
        bybit.get_orderbook(...).await;                     // REST
    }
    // ↑ 이 모든 .await 동안 select! 루프 정지
}
```

**변경 후:**
```rust
// select! 루프 - 재선택 브랜치
_ = reselect_timer.tick() => {
    // tokio::spawn으로 분리
    // 완료 시 mpsc 채널로 결과 전송 → select! 루프에서 수신하여 current_coins 교체
}
```

### 3. 공유 상태 관리

`check_tick_signal`이 별도 task에서 실행되므로, 기존 `&mut` 참조 대신 `Arc<Mutex<>>` (개별)로 공유해야 한다.

#### 3.1 check_tick_signal이 쓰는 상태 (Arc<Mutex<>> 필수)

| 상태 | 타입 | 접근 패턴 | 비고 |
|------|------|----------|------|
| `position_mgr` | `PositionManager` | R/W (open/close/partial) | check_tick_signal, finalize_and_process, check_ttl |
| `trades` | `Vec<ClosedPosition>` | W (push) | check_tick_signal, finalize_and_process, check_ttl |
| `ob_cache` | `OrderBookCache` | R/W (update, get, computing flag) | check_tick_signal, reselection |
| `counters` | `MonitoringCounters` | W (increment) | check_tick_signal, check_ttl |
| `session_writer` | `Option<SessionWriter>` | W (append_trade, append_minute) | check_tick_signal, finalize_and_process |

#### 3.2 check_tick_signal이 읽는 상태 (스냅샷 방식)

`candle_builder`와 `spread_calc`는 select! 루프에서 매 틱마다 쓰기(캔들 업데이트)가 발생하므로, Lock으로 감싸면 초당 ~50회 lock 경합이 생긴다. **스냅샷 방식을 채택**한다:

```rust
// select! 루프에서 spawn 전에 필요한 값을 Copy로 추출
let upbit_price = candle_builder.upbit_last_trade.get(coin).copied();
let bybit_price = candle_builder.bybit_last_bid.get(coin).copied();
let stats = spread_calc.cached_stats(coin);  // (mean, stddev) 튜플
// → 이 스칼라 값들을 spawned task에 전달 (Clone/Copy, lock 불필요)
```

| 상태 | 타입 | 전달 방식 | 비고 |
|------|------|----------|------|
| `candle_builder` | `MinuteCandleBuilder` | 스냅샷 (price Copy) | select! 루프 로컬 변수 유지, Arc 불필요 |
| `spread_calc` | `SpreadCalculator` | 스냅샷 (mean/stddev Copy) | 재선택에서만 Arc<Mutex<>> 필요 (add/remove_coin) |

> `candle_builder`는 select! 루프 전용 로컬 변수로 유지. `spread_calc`는 재선택 task에서 `add_coin`/`remove_coin`을 호출하므로 `Arc<tokio::sync::RwLock<SpreadCalculator>>`로 감싼다 (읽기 위주이므로 RwLock).

#### 3.3 재선택 관련 상태 (채널 기반 — Lock 불필요)

재선택 task의 결과는 `mpsc` 채널로 select! 루프에 전송하여, 아래 상태를 **select! 루프의 로컬 변수로 유지**한다:

| 상태 | 타입 | 접근 패턴 | Lock 필요 여부 |
|------|------|----------|---------------|
| `current_coins` | `Vec<String>` | 재선택 완료 시 교체, 매 틱 읽기 | **불필요** — select! 루프 로컬, 채널로 교체 |
| `dropped_at` | `HashMap<String, DateTime<Utc>>` | 재선택 시 갱신, check_ttl 시 읽기 | **불필요** — `ReselectionResult`에 갱신 내용 포함, select! 루프에서 반영 |

#### 3.4 재선택 task 내부에서 접근하는 공유 상태

| 상태 | 타입 | 접근 패턴 | 비고 |
|------|------|----------|------|
| `spread_calc` | `SpreadCalculator` | W (add_coin/remove_coin/update) | `Arc<tokio::sync::RwLock<>>` |
| `ob_cache` | `OrderBookCache` | W (프리페치 결과 저장) | 3.1의 Arc<Mutex<>> 공유 |
| `counters` | `MonitoringCounters` | W (orderbook_fetch_count) | 3.1의 Arc<Mutex<>> 공유 |
| `position_mgr` | `PositionManager` | R (diff_coins에서 포지션 확인) | 3.1의 Arc<Mutex<>> 공유 |

#### 3.5 Atomic으로 충분한 상태

| 상태 | 타입 | 변경 후 |
|------|------|--------|
| `total_event_count` | `u64` | `Arc<AtomicU64>` (lock-free increment, `Ordering::Relaxed`) |

#### 3.6 Mutex 타입 선택 기준

| 상태 | Mutex 타입 | 이유 |
|------|-----------|------|
| `position_mgr` | `tokio::sync::Mutex` | spawn task + select! 루프 양쪽 접근, `.await` 사용 |
| `trades` | `tokio::sync::Mutex` | 동일 |
| `ob_cache` (데이터) | `tokio::sync::RwLock` | 읽기 빈번(캐시 조회), 쓰기는 REST 완료 시만 |
| `session_writer` | `tokio::sync::Mutex` | blocking I/O 포함하지만 CSV write는 수 μs로 실용적 문제 없음 |
| `counters` | `std::sync::Mutex` | 단순 increment, lock 시간 극히 짧음, `.await` 불필요 |
| `spread_calc` | `tokio::sync::RwLock` | 재선택 task에서 쓰기, tick에서 읽기 |
| `computing_flags` | 별도 분리 (아래 4절 참조) | lock-free 또는 전용 Mutex |

> **핵심 규칙**: `std::sync::Mutex`를 tokio async context에서 사용할 때, lock hold 시간이 수 μs 이내여야 하며, lock 내에서 `.await`를 절대 호출하면 안 된다.

#### 3.7 minute_records

| 상태 | 타입 | 접근 패턴 |
|------|------|----------|
| `minute_records` | `Vec<MinuteRecord>` | `finalize_and_process`에서만 push |

`minute_records`는 spawn task에서 접근하지 않으므로 select! 루프 로컬 변수로 유지. `finalize_and_process`도 select! 루프 내에서 직접 실행되므로 Arc 불필요.

### 4. computing flag 동시성 안전 전환

`OrderBookCache`의 computing flag를 **데이터 캐시와 분리**하여 lock 경합을 감소시킨다.

**구조 분리:**
```rust
// 기존: OrderBookCache에 데이터 + flag 혼합
// 변경: 두 관심사 분리

struct SharedObCache {
    data: Arc<tokio::sync::RwLock<ObCacheData>>,   // 오더북 스냅샷
    computing: Arc<ComputingFlags>,                  // lock-free 또는 별도 Mutex
}
```

**computing flag의 atomic check-and-set:**

```rust
/// 원자적 check-and-set. 이미 computing 중이면 true 반환 (스킵).
pub fn try_set_computing(&self, exchange: Exchange, coin: &str) -> bool {
    let mut flags = self.inner.lock().unwrap(); // std::sync::Mutex — 극히 짧은 lock
    let entry = flags.entry((exchange, coin.to_string())).or_insert(false);
    if *entry {
        true  // 이미 computing 중
    } else {
        *entry = true;
        false  // 설정 성공, 이 task가 REST 수행
    }
}
```

**요구사항:**
- `is_computing()` + `set_computing(true)`가 원자적(atomic)이어야 함 (단일 lock 내 check-and-set)
- 코인당 1개 task만 실행되도록 보장
- `set_computing(false)` 해제는 task 완료 시 호출 (에러 시에도 반드시 해제)

### 5. 하위 호환

- `auto_select = false`인 경우에도 동일하게 동작해야 함 (재선택 없음, tick signal만 spawn)
- 기존 테스트 153개 모두 통과
- config 변경 없음 (신규 설정 필드 없음)

## 설계

### 아키텍처 변경 개요

```
변경 전:
┌──────────────────────────────────────────────────────┐
│ select! loop (싱글 태스크)                              │
│  ├─ recv() → handle_event() → check_tick_signal()    │
│  │                               └─ get_orderbook()  │  ← 블로킹
│  ├─ reselect_timer → 재선택 워밍업                      │  ← 블로킹
│  ├─ minute_timer → finalize_and_process              │
│  └─ heartbeat_timer → 로그                            │
└──────────────────────────────────────────────────────┘

변경 후:
┌──────────────────────────────────────────────────────┐
│ select! loop (이벤트 소비 전용)                          │
│  ├─ recv() → handle_event() → spawn check_tick_signal │  ← 비블로킹
│  ├─ reselect_timer → spawn 재선택                       │  ← 비블로킹
│  ├─ reselect_result_rx.recv() → current_coins 교체      │  ← 재선택 완료 수신
│  ├─ minute_timer → finalize_and_process               │
│  └─ heartbeat_timer → 로그                             │
└──────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────┐
│ spawned tasks (tokio task pool)                       │
│  ├─ check_tick_signal: REST 조회 + 시그널 평가 + 포지션   │
│  └─ 재선택: CoinSelector + 워밍업 + 구독 교체             │
└──────────────────────────────────────────────────────┘
```

### select! 루프 변경

```rust
loop {
    tokio::select! {
        _ = cancel_token.cancelled() => { break; }

        Some(event) = upbit_rx.recv() => {
            total_event_count.fetch_add(1, Ordering::Relaxed);
            // 1. 캔들 업데이트 (가벼운 동기 작업)
            //    candle_builder 업데이트, 분 경계 체크
            // 2. check_tick_signal을 tokio::spawn
            //    computing flag check-and-set → 이미 실행 중이면 스킵
            //    필요한 Arc clone + 스냅샷 데이터 전달
        }

        Some(event) = bybit_rx.recv() => {
            // 동일 패턴
        }

        _ = reselect_timer.tick(), if auto_select && !reselecting => {
            // reselecting = true 설정
            // tokio::spawn으로 재선택 task 시작
            // 완료 시 reselect_result_tx로 결과 전송
        }

        Some(result) = reselect_result_rx.recv() => {
            // current_coins 교체
            // reselecting = false 설정
        }

        _ = minute_timer.tick() => {
            // finalize_and_process (기존과 동일, lock 사용)
            // check_ttl_positions (lock 사용)
        }

        _ = heartbeat_timer.tick() => {
            // 로그 출력 (lock으로 상태 읽기)
        }
    }
}
```

### check_tick_signal 변경

```rust
// 기존: async fn check_tick_signal(&mut 여러 개) -> Result<()>
// 변경: 스냅샷 데이터 + Arc 공유 상태를 받아 spawned task에서 처리

fn spawn_check_tick_signal(
    coin: String,
    config: Arc<ZScoreConfig>,
    // 스냅샷 데이터 (select! 루프에서 Copy로 추출)
    upbit_price: Decimal,
    bybit_price: Decimal,
    usd_krw: f64,
    current_spread: f64,
    mean: f64,
    stddev: f64,
    source_exchange: orderbook::Exchange,
    // Arc 공유 상태 (clone으로 전달)
    position_mgr: Arc<tokio::sync::Mutex<PositionManager>>,
    trades: Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
    ob_cache: Arc<SharedObCache>,  // 데이터(RwLock) + flag(분리) 포함
    counters: Arc<std::sync::Mutex<MonitoringCounters>>,
    session_writer: Arc<tokio::sync::Mutex<Option<SessionWriter>>>,
    // 거래소 클라이언트 (Arc, 내부 불변)
    upbit_client: Arc<U>,
    bybit_client: Arc<B>,
) {
    tokio::spawn(async move {
        // 에러 발생 시 warn 로그 + computing flag 해제 보장
        if let Err(e) = async {
            // 1. computing flag check-and-set (atomic) → 이미 실행 중이면 return
            // 2. get_orderbook REST 호출
            // 3. ob_cache.data write lock → 갱신
            // 4. 청산 시그널 평가 + 실행 (position_mgr lock)
            // 5. 진입 시그널 평가 + 실행 (position_mgr lock)
            // 6. computing flag 해제
            Ok::<(), Box<dyn std::error::Error + Send>>(())
        }.await {
            warn!(coin = coin.as_str(), error = %e, "check_tick_signal task 에러");
        }
        // ↑ 에러든 정상이든 여기서 computing flag 해제 보장
    });
}
```

### 재선택 비동기 분리

`warmup_single_coin`은 현재 `&self` 메서드이므로, standalone 연관 함수로 리팩터링하여 `Arc`된 필드를 직접 받도록 한다.

```rust
fn spawn_reselection(
    config: Arc<ZScoreConfig>,
    upbit_client: Arc<U>,
    bybit_client: Arc<B>,
    forex_cache: Arc<ForexCache>,
    // 공유 상태
    spread_calc: Arc<tokio::sync::RwLock<SpreadCalculator>>,
    ob_cache: Arc<SharedObCache>,
    counters: Arc<std::sync::Mutex<MonitoringCounters>>,
    position_mgr: Arc<tokio::sync::Mutex<PositionManager>>,
    current_coins_snapshot: Vec<String>,  // 현재 코인 리스트 복사
    dropped_at_snapshot: HashMap<String, DateTime<Utc>>,  // 현재 dropped_at 복사
    // 결과 전송
    result_tx: mpsc::Sender<ReselectionResult>,
) {
    tokio::spawn(async move {
        // 1. CoinSelector::select() → REST 호출
        // 2. diff_coins 계산 (position_mgr read lock)
        // 3. 제거 코인: spread_calc write lock → remove_coin + 구독 해제
        // 4. 추가 코인: warmup_single_coin (standalone fn) + 구독 추가 + 오더북 프리페치
        // 5. 결과 전송
        let result = ReselectionResult {
            new_coins: spread_calc.read().await.active_coins().to_vec(),
            dropped_at_updates,  // 갱신된 dropped_at 항목
        };
        result_tx.send(result).await.ok();
    });
}

struct ReselectionResult {
    new_coins: Vec<String>,
    dropped_at_updates: HashMap<String, DateTime<Utc>>,
}

// warmup_single_coin을 standalone 연관 함수로 리팩터링
async fn warmup_single_coin(
    upbit: &U,
    bybit: &B,
    config: &ZScoreConfig,
    forex_cache: &ForexCache,
    coin: &str,
    spread_calc: &tokio::sync::RwLock<SpreadCalculator>,
) -> Result<(), StrategyError> { ... }
```

### Lock 순서 규약

데드락을 방지하기 위해 여러 Mutex를 동시에 잡아야 할 경우 아래 순서를 준수한다:

```
1. ob_cache        (computing flag check-and-set 먼저)
2. position_mgr    (포지션 조회/수정)
3. trades          (결과 기록)
4. session_writer  (CSV 기록)
5. counters        (카운터 증가)
6. spread_calc     (통계 조회)
7. dropped_at      (TTL 추적)
```

> 가능하면 한 번에 하나의 lock만 잡고, 빠르게 해제한다.

### finalize_and_process async 전환

`finalize_and_process`는 현재 동기 함수(`fn`)지만, 내부에서 `tokio::sync::Mutex`/`RwLock`의 `.lock().await`를 사용해야 하므로 `async fn`으로 변경한다.

```rust
// 변경 전: fn finalize_and_process(...) -> Result<()>
// 변경 후: async fn finalize_and_process(...) -> Result<()>
```

`minute_timer` 브랜치에서 `.await`로 호출:
```rust
_ = minute_timer.tick() => {
    Self::finalize_and_process(...).await?;
    Self::check_ttl_positions(...).await?;
}
```

`check_ttl_positions`도 `position_mgr` 등 공유 상태에 접근하므로 동일하게 `async fn`으로 변경.

## 파일 변경 목록

| 파일 | 변경 유형 | 설명 |
|------|-----------|------|
| `crates/arb-strategy/src/zscore/monitor.rs` | **대규모 수정** | select! 루프 리팩터링, spawn 분리, Arc 적용, finalize_and_process/check_ttl async 전환 |
| `crates/arb-strategy/src/zscore/orderbook.rs` | **수정** | OrderBookCache → SharedObCache (데이터/flag 분리), try_set_computing atomic CAS |
| `crates/arb-strategy/src/zscore/mod.rs` | **수정 가능** | 필요 시 re-export 추가 |
| `crates/arb-strategy/Cargo.toml` | **수정 가능** | 필요 시 의존성 추가 |

### 변경하지 않는 파일

| 파일 | 이유 |
|------|------|
| `config.rs` | 신규 설정 필드 없음 |
| `signal.rs` | 순수 함수, 변경 불필요 |
| `position.rs` | 구조 변경 없음, Mutex로 감싸기만 함 |
| `pnl.rs` | 변경 불필요 |
| `stream.rs` (Upbit/Bybit) | WebSocket 레이어 변경 없음 |
| `crates/arb-exchange/src/traits.rs` | `MarketData` trait 변경 불필요 (제네릭 `Arc<U>` 유지) |

## 검증

1. `cargo test -p arb-strategy` — 기존 153개 + 신규 테스트 통과
2. `cargo clippy -p arb-strategy` — 경고 0
3. 라이브 테스트 (60초):
   - 이벤트 드롭 0건 확인 (WARN 로그 없음)
   - 오더북 REST 호출이 select! 루프를 블로킹하지 않는지 확인
   - 코인 재선택 동안 기존 코인의 시그널 평가가 계속되는지 확인
   - 포지션 진입/청산 정상 동작 확인
4. 부하 테스트 시나리오:
   - 5개 코인 동시 모니터링 + 재선택 트리거
   - 재선택 중 청산 시그널 발생 시 정상 처리 확인

## 테스트 전략

### 기존 테스트 수정

기존 153개 테스트에서 `MockMarket`의 `get_orderbook`이 `unimplemented!()`로 되어 있으면 spawn 시 panic 발생. 적절한 mock 반환값을 추가해야 한다.

### 신규 테스트

| 테스트 | 검증 대상 |
|--------|----------|
| `test_computing_flag_try_set_atomic` | `try_set_computing`의 원자적 check-and-set 동작 |
| `test_concurrent_check_tick_signal` | 여러 코인의 spawn된 task가 position_mgr에 경합 없이 접근 |
| `test_reselection_during_tick_processing` | 재선택 중 기존 코인의 시그널 평가가 정상 동작 |
| `test_computing_flag_release_on_error` | REST 실패 시 computing flag가 반드시 해제 |

### 데드락 방지 검증

모든 async 테스트에 `tokio::time::timeout`을 감싸서 데드락 발생 시 테스트 실패 처리:
```rust
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_no_deadlock() {
    tokio::time::timeout(Duration::from_secs(5), async { ... }).await.unwrap();
}
```

## 알려진 한계

- **Lock 경합**: 틱 빈도가 매우 높을 경우(코인 10개+) Mutex lock 경합이 발생할 수 있다. 이 경우 lock-free 자료구조로 전환을 고려한다.
- **순서 보장 없음**: spawn된 task들의 완료 순서가 보장되지 않는다. 예: tick A의 REST가 tick B보다 늦게 완료될 수 있다. 현재 시뮬레이션(가상 포지션)에서는 엄격한 순서가 불필요하므로 허용한다.
- **테스트 복잡도 증가**: Arc<Mutex<>> 구조로 인해 단위 테스트에서 상태 설정/검증이 복잡해진다.
- **SessionWriter blocking I/O**: `tokio::sync::Mutex` 내에서 `BufWriter<File>` write/flush 수행. CSV write는 수 μs이므로 실용적으로 문제없지만, 디스크 I/O 지연 시 lock hold가 길어질 수 있다. 향후 전용 writer task 분리를 고려.
