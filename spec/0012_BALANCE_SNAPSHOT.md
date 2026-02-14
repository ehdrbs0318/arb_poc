# 0012: 잔고 스냅샷 (Balance Snapshot)

## 1. 목적

라이브 전략 수행 중 **계좌 가치 변동을 추적**하기 위한 감사(audit) 보조 도구.

거래소 실잔고를 다음 시점에 DB에 기록하여, 시간 경과에 따른 자산 변동을 추적한다:

1. **정기 기록 (PERIODIC)**: 설정 가능한 주기 (기본 10분)
2. **포지션 진입 (POS_ENT)**: 진입 직후 양 거래소 잔고 기록
3. **포지션 청산 (POS_EXT)**: 청산 직후 양 거래소 잔고 기록

> **역할 정의**: 잔고 스냅샷은 전체 계좌 가치 변동의 **교차 검증(cross-check) 도구**이다.
> 거래별 손익의 primary source는 `positions`/`trades` 테이블이다.
> 양자 간 괴리가 발생하면 환율 변동, 펀딩비, 외부 입출금 등 비거래 요인을 조사한다.
>
> **핵심 제약**: 잔고 기록은 전략 수행에 지연을 주면 안 된다.
> 전략 이벤트 루프와 완전히 분리된 비동기 파이프라인으로 구현한다.
>
> **주의**: 운용 중 외부 입출금(추가 자본 투입, 수익 인출 등) 발생 시
> 스냅샷 기반 수익률 계산이 왜곡된다. 운용 중 외부 자금 이동을 최소화할 것.

---

## 2. 테이블 스키마

### 2.1 DDL (`V007__create_balance_snapshots.sql`)

```sql
CREATE TABLE balance_snapshots (
    id                 BIGINT       NOT NULL AUTO_INCREMENT PRIMARY KEY,
    created_at         DATETIME(3)  NOT NULL,
    snapshot_group_id  BIGINT       NOT NULL COMMENT '같은 트리거로 생성된 행 그룹 식별자',
    session_id         BIGINT       NOT NULL,
    record_type        VARCHAR(10)  NOT NULL COMMENT 'PERIODIC | POS_ENT | POS_EXT',
    cex                VARCHAR(10)  NOT NULL COMMENT 'UPBIT | BYBIT',
    currency           VARCHAR(10)  NOT NULL COMMENT 'KRW | USDT',
    available          DECIMAL(20,8) NOT NULL COMMENT '기축통화 주문 가능 잔고',
    locked             DECIMAL(20,8) NOT NULL COMMENT '기축통화 잠긴 잔고 (주문 중)',
    coin_value         DECIMAL(20,8) NOT NULL DEFAULT 0 COMMENT '보유 코인/포지션 환산 가치',
    total              DECIMAL(20,8) NOT NULL COMMENT '총 자산 가치 (Upbit: available+locked+coin_value, Bybit: equity)',
    position_id        BIGINT       NULL     COMMENT 'POS_ENT/POS_EXT 시 positions.id FK',
    usd_krw            DOUBLE       NOT NULL COMMENT '기록 시점 USD/KRW 공시 환율',
    usdt_krw           DOUBLE       NOT NULL COMMENT '기록 시점 USDT/KRW 거래소 시세',
    total_usd          DECIMAL(20,8) NOT NULL COMMENT 'USD 환산 총자산',
    total_usdt         DECIMAL(20,8) NOT NULL COMMENT 'USDT 환산 총자산',

    INDEX idx_session_created (session_id, created_at),
    INDEX idx_session_type (session_id, record_type),
    INDEX idx_snapshot_group (snapshot_group_id),
    INDEX idx_position (position_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

> `created_at`은 `DEFAULT CURRENT_TIMESTAMP(3)`가 아닌 application level에서 설정.
> 같은 트리거의 2행이 동일한 `created_at`을 갖도록 보장한다.

### 2.2 컬럼 상세

| 컬럼 | 타입 | 설명 |
|------|------|------|
| `id` | BIGINT PK | Auto-increment |
| `created_at` | DATETIME(3) | 기록 시각 (application level에서 동일 값 강제) |
| `snapshot_group_id` | BIGINT | 같은 트리거로 생성된 2행의 그룹 식별자 (u64 시퀀스, **session 내에서만 고유**) |
| `session_id` | BIGINT | 현재 세션 ID (sessions 테이블 FK) |
| `record_type` | VARCHAR(10) | `PERIODIC` \| `POS_ENT` \| `POS_EXT` |
| `cex` | VARCHAR(10) | `UPBIT` \| `BYBIT` |
| `currency` | VARCHAR(10) | `KRW` \| `USDT` |
| `available` | DECIMAL(20,8) | 기축통화(KRW/USDT) 주문 가능 잔고 |
| `locked` | DECIMAL(20,8) | 기축통화 주문에 묶인 잔고 |
| `coin_value` | DECIMAL(20,8) | 보유 코인/포지션 환산 가치 (Section 2.3 caveat 참조) |
| `total` | DECIMAL(20,8) | 총 자산 가치 (Section 2.3 참조 — 거래소별 계산 방식 상이) |
| `position_id` | BIGINT NULL | `POS_ENT`/`POS_EXT` 시 `positions.id`, `PERIODIC` 시 NULL |
| `usd_krw` | DOUBLE | 기록 시점 USD/KRW 공시 환율 (arb-forex) |
| `usdt_krw` | DOUBLE | 기록 시점 USDT/KRW 거래소 시세 (WS 캐시) |
| `total_usd` | DECIMAL(20,8) | USD 환산 총자산 |
| `total_usdt` | DECIMAL(20,8) | USDT 환산 총자산 |

### 2.3 total 계산 공식

**거래소별 계산 방식이 다르다.**

#### Upbit (KRW)

```
total = available + locked + coin_value
```

| 필드 | 소스 |
|------|------|
| `available` | KRW 가용 잔고 |
| `locked` | KRW 잠긴 잔고 (매수 주문 중 KRW) |
| `coin_value` | Σ(보유 코인 qty × 캐시된 현재가) — 항상 >= 0 |

#### Bybit (USDT)

```
total = equity   ← API 응답에서 직접 사용 (available + locked + coin_value로 조립하지 않음)
```

| 필드 | 소스 | 비고 |
|------|------|------|
| `available` | USDT `walletBalance` | 선물 운용 기준 가용 분해 필드 |
| `locked` | USDT `locked` (Option → ZERO) | 참고용 분해 필드 |
| `coin_value` | USDT `unrealisedPnl` (Option → ZERO) | 양수/음수 모두 가능 |
| `total` | USDT `equity` | **ground truth** — `walletBalance + unrealisedPnl` |

> **Bybit에서 `available + locked + coin_value ≠ total` 일 수 있다.**
> Bybit Unified 계정에서 `availableToWithdraw`는 deprecated이며
> 실제 운영 시 0/빈값으로 내려올 수 있다.
> 따라서 Bybit `total`은 반드시 API의 `equity` 필드를 직접 사용해야 하며,
> `available`, `locked`, `coin_value`는 참고용 분해 필드로만 활용한다.

> **`coin_value` 의미 차이 주의**:
> - **Upbit**: 보유 코인의 **절대 가치** (KRW). 항상 >= 0.
> - **Bybit**: 선물 포지션의 **미실현 PnL** (USDT). 양수/음수 모두 가능.
>
> 동일 컬럼이지만 의미가 다르므로, `coin_value`를 거래소 간 직접 비교하면 안 된다.
> 거래소 간 비교는 `total` 또는 `total_usdt` 기준으로 수행한다.

### 2.4 환산 공식

| currency | total_usd | total_usdt |
|----------|-----------|------------|
| KRW | `total / usd_krw` | `total / usdt_krw` |
| USDT | `total` (그대로) | `total` (그대로) |

> **0 나누기 방어**: `usd_krw == 0.0` 또는 `usdt_krw == 0.0`이면
> `total_usd = 0`, `total_usdt = 0`으로 기록하고 `warn!` 로그를 남긴다.
> Decimal division by zero panic을 방지하기 위해 환산 전 반드시 가드 검사한다.
>
> **`f64 → Decimal` 변환 방어**: `Decimal::try_from(usd_krw)`가 실패할 수 있다
> (NaN, Infinity 등). 변환 실패 시 해당 환산값을 0으로 기록하고 `warn!` 로그를 남긴다.
> `Decimal::from_f64_retain()`을 사용하고 최종 결과를 소수점 8자리로 truncation한다.
>
> **`usdt_krw` 컬럼 추가 근거**: USD ≠ USDT. 한국 거래소의 USDT/KRW 시세는 공시 환율(USD/KRW)과 다르다 (김치 프리미엄 등).
> `total_usdt` 계산을 위해 기록 시점의 USDT/KRW 환율을 별도 저장해야 한다.
>
> **`USDT = USD` 가정**: Bybit USDT 레코드에서 `total_usd = total`로 기록한다.
> USDT/USD 시세 괴리는 극단적 시장 상황(< 0.97) 외에는 무시 가능하다.

### 2.5 기록 단위

| 트리거 | 생성 행 수 | 설명 |
|--------|-----------|------|
| PERIODIC | 2행 | Upbit KRW 1행 + Bybit USDT 1행 (동일 snapshot_group_id) |
| POS_ENT | 2행 | 동일 (position_id + snapshot_group_id 포함) |
| POS_EXT | 2행 | 동일 (position_id + snapshot_group_id 포함) |

> **Partial 스냅샷**: 한쪽 거래소 API 실패 시 1행만 기록될 수 있다.
> Section 10의 `SUM() GROUP BY snapshot_group_id` 쿼리에서 불완전한 합계가 나올 수 있으므로,
> 분석 시 `HAVING COUNT(*) = 2`로 완전한 그룹만 필터링할 것을 권장한다.

---

## 3. 데이터 소스

### 3.1 Upbit 잔고 (2회 API 호출)

#### 호출 1: 계좌 잔고 조회

- **API**: `GET /v1/accounts` (Exchange API, 인증 필요)
- **리미터**: `exchange_limiter` (25 req/sec)
- **응답**: 보유 중인 모든 화폐의 잔고 배열

**KRW 추출**:
- `available` = KRW 항목의 `balance`
- `locked` = KRW 항목의 `locked`

**보유 코인 목록 추출**:
- `currency != "KRW"` 항목 → 코인 심볼 + 보유 수량(`balance + locked`)
- **Dust 필터링**: `avg_buy_price × (balance + locked) < 1,000 KRW`인 코인은 제외
  - 과거 거래의 극소량 잔존 코인(dust)이 불필요한 ticker 조회를 유발하는 것을 방지
  - `avg_buy_price`는 매수 시점 평균가이므로 현재가와 다를 수 있음 (보수적 필터 — 과소평가 시 제외됨)
  - `avg_buy_price == 0`인 에어드랍 코인은 항상 필터링됨 (의도된 동작)

#### 호출 2: 보유 코인 현재가 조회

- **API**: `GET /v1/ticker?markets=KRW-BTC,KRW-ETH,...` (복수 마켓 1회 조회)
- **리미터**: `quotation_limiter` (8 req/sec) — 캐시 미스 시에만 소비
- **마켓 코드 변환**: Balance의 `currency` (예: `"BTC"`) → `"KRW-{currency}"` (예: `"KRW-BTC"`)
- **캐시**: **1초 TTL 캐시** 적용 (Section 3.4 참조)
  - 캐시 히트 시 REST 호출 생략 → rate limit 소비 없음
- **보유 코인 없음**: dust 필터링 후 코인이 없으면 ticker 조회 생략, `coin_value = 0`

#### 매핑

```
available   = KRW balance
locked      = KRW locked
coin_value  = Σ(코인별 (balance + locked) × trade_price)   // dust 제외
total       = available + locked + coin_value
```

> Upbit `GET /v1/accounts`는 1회 호출로 모든 화폐 잔고를 반환한다.
> `GET /v1/ticker`는 쉼표 구분으로 복수 마켓을 1회 호출로 조회 가능하다.

### 3.2 Bybit 잔고 (1회 API 호출)

- **API**: `GET /v5/account/wallet-balance?accountType=UNIFIED`
- **리미터**: `limiter` (18 req/sec)
- **추출**: coin 배열에서 `coin == "USDT"` 항목

#### 매핑

```
available   = USDT walletBalance             // 선물 운용 기준 가용 잔고
locked      = USDT locked                    // Option → None이면 Decimal::ZERO
coin_value  = USDT unrealisedPnl             // Option → None이면 Decimal::ZERO
total       = USDT equity                    // ← API 필드 직접 사용 (위 3개 합산 아님)
```

> **`total`은 `equity`를 직접 사용한다** (Section 2.3 참조).
> `available + locked + coin_value`로 조립하지 않는다.
> Bybit Unified 파생상품에서 `availableToWithdraw`는 deprecated이므로
> `available` 분해 필드는 `walletBalance` 기준으로 관리한다.
>
> **Option 필드 fallback**: Bybit API의 `locked`, `unrealisedPnl`은
> `Option<Decimal>`이다. `None`인 경우 `Decimal::ZERO`로 처리한다.
>
> **구현 필요 사항**:
> 1. `BybitCoinBalance` 구조체에 `equity` 필드 추가 파싱 (`Option<Decimal>`)
> 2. `Balance` 구조체에 `equity: Option<Decimal>` + `unrealised_pnl: Option<Decimal>` 필드 추가
>    - Upbit은 `None` (해당 없음), Bybit은 API 응답에서 채움
>    - `Balance`는 `arb-exchange` crate 소속이므로 거래소 비의존적 `Option` 설계
> 3. `convert_balance()` 함수에서 `equity`, `unrealisedPnl` 매핑 추가
>
> 추가 API 호출 없이 같은 응답에서 모든 필드를 추출 가능하다.

### 3.3 환율

| 환율 | 소스 | 갱신 방식 |
|------|------|-----------|
| `usd_krw` | arb-forex (Yahoo Finance) | 기존 TTL 캐시 (600초) + 10분 갱신 |
| `usdt_krw` | 거래소 USDT/KRW WS 시세 | **신규 구현** — WS 수신 + 메모리 캐시 |

### 3.4 코인 현재가 캐시 (Ticker Cache)

Upbit 보유 코인의 KRW 환산을 위해 현재가를 조회해야 하는데,
**전략용 API 호출에 rate limit 영향을 주면 안 된다**.

#### 설계 원칙

- **잔고 기록 전용 캐시** — 전략의 기존 API 호출 경로와 완전히 분리
- **1초 TTL** — 캐시 히트 시 REST 호출 생략, rate limit 소비 없음
- **캐시 미스 시 `quotation_limiter` 1회 소비** — PERIODIC(10분 간격)에서는 항상 캐시 미스이나, 10분당 1회이므로 rate limit에 무영향

#### 동작

```
1. handle_snapshot() 진입
2. Upbit get_balances() → 보유 코인 목록 추출 (dust 필터링 적용)
3. 보유 코인이 없으면 → coin_value = 0, ticker 조회 생략
4. 보유 코인별 캐시 조회 (HashMap<String, (Decimal, Instant)>)
   - 캐시 히트 (1초 이내): 캐시된 가격 사용 → REST 호출 생략
   - 캐시 미스: GET /v1/ticker?markets=KRW-{coin1},KRW-{coin2},... 1회 호출 → 캐시 갱신
5. coin_value = Σ(qty × cached_price)
```

#### 구조

```rust
/// 코인 현재가 캐시 (잔고 기록 전용).
///
/// 전략의 rate limit에 영향을 주지 않기 위해 1초 TTL 캐시를 유지.
struct TickerCache {
    /// coin -> (price_krw, updated_at)
    prices: HashMap<String, (Decimal, Instant)>,
    ttl: Duration,  // 기본 1초
}
```

> **향후 개선 (미래 스펙)**: 현재는 REST 기반 1초 캐시로 구현하지만,
> 추후 WS의 실시간 거래 체결 메시지(trade event)를 메모리에 캐싱하여
> 각 코인의 마지막 체결가를 REST 호출 없이 조회하는 방식으로 전환 예정.
> 이 경우 ticker REST 호출이 완전히 제거되어 rate limit 부담이 0이 된다.

---

## 4. USDT/KRW 가격 캐시 (신규 구현)

### 4.1 개요

거래소의 USDT/KRW 실시간 시세를 WebSocket으로 수신하여 메모리에 캐싱한다.

### 4.2 구현 위치

`arb-forex` crate에 추가. 기존 `ForexCache` (USD/KRW)와 동일 레이어에서 환율 캐시를 관리한다.

> `arb-strategy`가 아닌 `arb-forex`에 배치하여 DI 원칙을 유지한다.
> `UsdtKrwCache::update(price: f64)`는 거래소 비의존적이므로 `arb-forex`에 위치해도 문제없다.
> Upbit WS 이벤트 → `update()` 호출은 상위 레이어(전략/모니터)에서 주입한다.

### 4.3 구조

```rust
/// USDT/KRW 실시간 가격 캐시.
///
/// Upbit WS에서 KRW-USDT ticker를 수신하여 최신 가격을 캐시.
pub struct UsdtKrwCache {
    /// 최신 USDT/KRW 가격 (atomic).
    price: AtomicU64,  // f64를 u64로 비트 변환 저장 (bits == 0 → 미설정)
    /// 마지막 업데이트 시각.
    updated_at: AtomicI64,  // epoch millis
}
```

### 4.4 동작

1. **초기화**: `UsdtKrwCache::new()` — **빈 상태로 생성** (bits == 0).
   상위 레이어(monitor_core 또는 main)에서 Upbit REST로 초기값을 조회한 뒤 `update()` 호출로 주입.
   (`arb-forex`에서 Upbit API를 직접 호출하면 `arb-forex` → `arb-exchanges` 의존이 발생하므로 DI 위반)
2. **WS 구독**: Upbit WebSocket에 `KRW-USDT` ticker 구독 추가
   - **캔들 빌더 전파 차단**: WS 이벤트 수신 시 `KRW-USDT` 마켓이면
     `UsdtKrwCache::update()` 호출 후 **기존 캔들 빌더/코인 선택기 파이프라인으로 전파하지 않는다**.
     분기 위치: WS 이벤트 디스패처(모니터의 select! 루프)에서 마켓 코드 검사 후 조기 반환.
3. **업데이트**: ticker 이벤트 수신 시 `price` atomic 업데이트
4. **조회**: `get_usdt_krw() -> Option<f64>` — TTL (60초) 초과 또는 `bits == 0`이면 `None` 반환

### 4.5 Fallback

WS 연결 끊김 등으로 가격이 stale(60초 초과)인 경우:
- REST API 1회 조회 시도 (`GET /v1/ticker?markets=KRW-USDT`)
- 실패 시 마지막 유효 가격 사용 + warn 로그

### 4.6 Bybit USDT/KRW

Bybit은 KRW 마켓이 없으므로 USDT/KRW 시세를 직접 제공하지 않는다.
Bybit USDT 레코드에서는 `total_usdt = total` (변환 불필요)이므로,
`usdt_krw` 컬럼에는 Upbit 기준 USDT/KRW 시세를 동일하게 기록한다.

---

## 5. 비동기 기록 파이프라인

### 5.1 아키텍처

```
전략 이벤트 루프                    Background Task                    기존 인프라
┌──────────────┐                 ┌──────────────────────┐           ┌──────────┐
│              │  SnapshotMsg    │                      │  Row      │          │
│  진입/청산    │ ──────────────► │  BalanceRecorderTask │ ────────► │ DbWriter │
│  감지        │   mpsc channel  │                      │  .send()  │          │
│              │                 │  1. REST 잔고 조회    │           │ 재시도   │
└──────────────┘                 │  2. 환율 조회         │           │ 에러로깅 │
                                 │  3. Row 조립          │           │ DB에 저장│
┌──────────────┐                 │                      │           └──────────┘
│ Periodic     │  SnapshotMsg    │                      │
│ Timer        │ ──────────────► │                      │
│ (10분 기본)   │   같은 channel  │                      │
└──────────────┘                 └──────────────────────┘
```

> DB INSERT는 기존 `DbWriter` 채널을 통해 수행한다 (`DbWriteRequest::InsertBalanceSnapshot`).
> `BalanceRecorderTask`는 MySqlPool을 직접 소유하지 않으며,
> DbWriter의 재시도/에러 로깅 인프라를 재활용한다.

### 5.2 메시지 타입

```rust
/// 잔고 스냅샷 요청 메시지.
pub enum SnapshotMsg {
    /// 정기 기록 (타이머 트리거).
    Periodic,
    /// 포지션 진입 직후.
    PositionEntry { position_id: i64 },
    /// 포지션 청산 직후.
    PositionExit { position_id: i64 },
    /// 종료 요청.
    Shutdown,
}
```

### 5.3 BalanceRecorderTask

```rust
pub struct BalanceRecorderTask {
    rx: mpsc::Receiver<SnapshotMsg>,
    session_id: i64,
    // 거래소 클라이언트 (잔고 + 시세 조회용)
    upbit: Arc<dyn ExchangeAdapter>,   // get_balances() + get_ticker()
    bybit: Arc<dyn ExchangeAdapter>,   // get_balances() (equity 포함)
    // 환율 소스
    forex: Arc<ForexCache>,            // USD/KRW
    usdt_cache: Arc<UsdtKrwCache>,     // USDT/KRW
    // DB 기록
    db_writer: DbWriter,               // 기존 DbWriter 채널 활용
    // 내부 상태
    ticker_cache: TickerCache,         // 코인 현재가 1초 캐시
    next_group_id: u64,                // snapshot_group_id 시퀀스 (단일 task → Atomic 불필요)
}
```

> `Arc<dyn ExchangeAdapter>`를 사용한다.
> `OrderManagement` trait은 RPITIT 사용으로 object-safe하지 않으므로 `dyn`으로 사용 불가.
> `ExchangeAdapter`는 `async_trait` 기반으로 이미 object-safe하며,
> `get_balances()`와 `get_ticker()` 모두 포함한다.
>
> `next_group_id`는 단순 `u64`이다. `BalanceRecorderTask`는 단일 consumer task이므로
> `&mut self`로 접근 가능하여 `AtomicU64`가 불필요하다.
>
> **`BalanceTracker`와의 관계**: 기존 `balance.rs`의 `BalanceTracker`는 전략 실행 중
> 가용 잔고 추적 + 예약(reserve/commit/release) 패턴을 담당하는 **내부 장부**이다.
> `BalanceRecorderTask`는 거래소 REST API로 **실잔고를 직접 조회**하여 DB에 기록하는
> **감사(audit) 도구**이다. 두 모듈은 서로 참조하지 않으며 독립적으로 동작한다.

### 5.4 처리 흐름

1. `rx.recv()` 대기
2. 메시지 수신 시:
   a. `snapshot_group_id` 채번 + `created_at` 생성 (application level 동일 값)
   b. Upbit `get_balances()` + Bybit `get_balances()` **동시 호출** (`tokio::join!`)
   c. Upbit: KRW 잔고 추출 + 보유 코인 목록 (dust 필터링) → ticker cache로 coin_value 계산
   d. Bybit: USDT coin의 walletBalance(→available) + equity(→total) + locked/unrealisedPnl 추출 (Option → ZERO fallback)
   e. `forex.get_usd_krw()` + `usdt_cache.get_usdt_krw()` 조회
   f. 환산값 계산 (`total_usd`, `total_usdt`) — 0 나누기 가드 포함
   g. 2행 Row 조립 → `db_writer.send()` 로 INSERT 위임
3. `Shutdown` 메시지 수신 시 **최종 Periodic 스냅샷 1회 기록 후** 루프 종료

### 5.5 Periodic Timer

- `BalanceRecorderTask` 내부에서 `tokio::time::interval` 사용
- `select! biased;`로 채널 메시지를 interval보다 우선 처리 (Shutdown 즉시 반응)

```rust
loop {
    tokio::select! {
        biased;

        msg = rx.recv() => {
            match msg {
                Some(SnapshotMsg::Shutdown) | None => {
                    // 종료 전 최종 스냅샷 기록
                    self.handle_snapshot(SnapshotMsg::Periodic).await;
                    break;
                }
                Some(msg) => self.handle_snapshot(msg).await,
            }
        }
        _ = interval.tick() => {
            self.handle_snapshot(SnapshotMsg::Periodic).await;
        }
    }
}
```

### 5.6 채널 용량

- `mpsc::channel(32)` — 충분한 버퍼 (포지션 진입/청산은 고빈도가 아님)
- 채널 가득 찬 경우 `try_send` 실패 → `warn!` 로그, 스냅샷 드롭 (전략 블로킹 방지)
- `MonitoringCounters`에 `balance_snapshot_dropped` 카운터 추가

### 5.7 전략 측 인터페이스

```rust
/// 잔고 스냅샷 전송 핸들.
#[derive(Clone)]
pub struct BalanceSnapshotSender {
    tx: mpsc::Sender<SnapshotMsg>,
}

impl BalanceSnapshotSender {
    /// 포지션 진입 시 스냅샷 요청 (non-blocking).
    pub fn on_position_entry(&self, position_id: i64) {
        if position_id > 0 {
            let _ = self.tx.try_send(SnapshotMsg::PositionEntry { position_id });
        }
    }

    /// 포지션 청산 시 스냅샷 요청 (non-blocking).
    pub fn on_position_exit(&self, position_id: i64) {
        if position_id > 0 {
            let _ = self.tx.try_send(SnapshotMsg::PositionExit { position_id });
        }
    }

    /// 종료 요청 (30초 타임아웃).
    pub async fn shutdown(&self) {
        if tokio::time::timeout(
            Duration::from_secs(30),
            self.tx.send(SnapshotMsg::Shutdown),
        ).await.is_err() {
            warn!("BalanceRecorderTask shutdown 전송 타임아웃 (30초)");
        }
    }
}
```

> `try_send`로 전략 이벤트 루프를 절대 블로킹하지 않는다.
> `try_send` 실패 시 `MonitoringCounters::balance_snapshot_dropped` 카운터를 증가시킨다.
> `position_id <= 0` (DB INSERT 실패 시)이면 스냅샷 전송을 스킵한다.
> `shutdown()`은 30초 타임아웃을 적용하여, REST hung 상태에서 프로세스 종료가 무한 대기하는 것을 방지한다.
> `tx.send()`가 수신자 drop으로 즉시 `Err`를 반환하는 경우에도 타임아웃과 동일하게 warn 처리한다.

---

## 6. 설정

### 6.1 strategy.toml

```toml
[balance_snapshot]
# 정기 기록 주기 (초). 기본값: 600 (10분)
interval_sec = 600
```

### 6.2 Rust 설정 구조체

```rust
/// 잔고 스냅샷 설정.
#[derive(Debug, Clone)]
pub struct BalanceSnapshotConfig {
    /// 정기 기록 주기 (초). 기본값: 600.
    pub interval_sec: u64,
}

impl Default for BalanceSnapshotConfig {
    fn default() -> Self {
        Self { interval_sec: 600 }
    }
}
```

---

## 7. 에러 처리

| 상황 | 처리 |
|------|------|
| 거래소 잔고 API 실패 | 1회 재시도 (500ms 딜레이, 네트워크/5xx만) → 실패 시 해당 거래소 행 스킵 + `warn!` |
| 한쪽 거래소만 성공 | 성공한 거래소 행만 기록 (partial snapshot). 분석 시 `HAVING COUNT(*) = 2`로 완전 그룹 필터 |
| DB INSERT 실패 | DbWriter의 기존 재시도 로직 활용 (3회 재시도) |
| USDT/KRW 캐시 stale | REST fallback 1회 → 실패 시 마지막 유효값 사용 + `warn!` |
| USD/KRW 환율 없음 | `total_usd = 0`, `total_usdt = 0` 기록 + `warn!` |
| `usd_krw` 또는 `usdt_krw` = 0 | 해당 환산값 = 0 기록 (0 나누기 방지) + `warn!` |
| `Decimal::try_from(f64)` 실패 | 해당 환산값 = 0 기록 + `warn!` (NaN/Infinity 방어) |
| 채널 가득 참 | `try_send` 실패 → 스냅샷 드롭 + `warn!` + `balance_snapshot_dropped` 카운터 |
| Bybit `locked`/`unrealisedPnl`/`equity` = None | `Decimal::ZERO`로 fallback |
| `position_id` <= 0 | 스냅샷 전송 스킵 (DB INSERT 실패한 포지션) |
| `shutdown()` 채널 닫힘 (RecvError) | warn 로그 후 정상 종료 처리 |

> 모든 에러는 전략 수행을 중단시키지 않는다.

---

## 8. 통합 포인트

### 8.1 monitor_live.rs

```rust
// 포지션 진입 성공 후
let db_position_id = db_writer.insert_position(&position).await?;
balance_sender.on_position_entry(db_position_id);

// 포지션 청산 성공 후
db_writer.update_position_exit(&position).await?;
balance_sender.on_position_exit(db_position_id);
```

> **Propagation delay 주의**: `on_position_entry()` / `on_position_exit()`는
> `try_send`로 메시지를 큐잉하고, 실제 잔고 REST 조회는 `BalanceRecorderTask`에서
> 비동기로 수행된다. 따라서 "진입/청산 직후" 잔고가 아니라 **수백ms~수초 후** 잔고를 기록한다.
>
> - 거래소 API 응답에 체결이 반영되기까지 propagation delay 존재 (Upbit: 수백ms, Bybit: 비동기 정산)
> - 큐 대기 + `tokio::join!` 조회 시간만큼 추가 지연
> - 주문 부분체결 또는 in_flight 상태에서 스냅샷이 찍힐 수 있음
>
> 이 한계는 비동기 파이프라인의 본질적 특성이며, 크로스체크 시
> `created_at`과 포지션의 `opened_at`/`closed_at` 간 시간 차를 감안해야 한다.

### 8.2 시작/종료

```rust
// main.rs 또는 monitor_live 초기화
let (snapshot_sender, recorder_task): (BalanceSnapshotSender, JoinHandle<()>) =
    BalanceRecorderTask::spawn(
    session_id,
    upbit_adapter.clone(),    // Arc<dyn ExchangeAdapter>
    bybit_adapter.clone(),    // Arc<dyn ExchangeAdapter>
    forex_cache.clone(),
    usdt_krw_cache.clone(),
    db_writer.clone(),        // DbWriter 채널 공유
    config.balance_snapshot.interval_sec,
);

// 종료 시 (shutdown 30초 + task 60초 타임아웃)
snapshot_sender.shutdown().await;
match tokio::time::timeout(Duration::from_secs(60), recorder_task).await {
    Ok(Ok(())) => info!("BalanceRecorderTask 정상 종료"),
    Ok(Err(e)) => warn!(error = %e, "BalanceRecorderTask 종료 에러"),
    Err(_) => warn!("BalanceRecorderTask 종료 타임아웃 (60초), task 포기"),
}
```

---

## 9. 구현 체크리스트

### Phase 1: USDT/KRW 가격 캐시

- [ ] `arb-forex`에 `UsdtKrwCache` 추가
  - [ ] `UsdtKrwCache` 구조체 (AtomicU64 + AtomicI64)
  - [ ] `new()` — **빈 상태 생성** (bits == 0, REST 호출 없음)
  - [ ] `update(price: f64)` — atomic 업데이트
  - [ ] `get_usdt_krw() -> Option<f64>` — TTL 검사 + bits==0 가드
  - [ ] REST fallback 메서드
  - [ ] 테스트용 `with_ttl()` 생성자
- [ ] 상위 레이어(monitor_core/main)에서 Upbit REST로 초기값 조회 → `usdt_cache.update()` 호출
- [ ] Upbit WS 구독에 `KRW-USDT` ticker 추가
  - [ ] ticker 이벤트 수신 시 `UsdtKrwCache::update()` 호출
  - [ ] WS 이벤트 디스패처에서 `KRW-USDT` 마켓 조기 반환 — 캔들 빌더/코인 선택기로 전파 차단
- [ ] 단위 테스트: TTL 만료, atomic 정합성, bits==0 미설정 상태

### Phase 2: DB 레이어

- [ ] `crates/arb-db/migrations/V007__create_balance_snapshots.sql`
- [ ] `crates/arb-db/src/balance_snapshots.rs`
  - [ ] `BalanceSnapshotRow` 구조체
  - [ ] `insert_snapshot(pool, row) -> Result<i64, DbError>`
  - [ ] `insert_snapshot_pair(pool, upbit_row, bybit_row) -> Result<(), DbError>`
- [ ] `DbWriteRequest`에 `InsertBalanceSnapshot` variant 추가
- [ ] `DbWriter::new()` 시그니처에 `BalanceSnapshotRepository` 파라미터 추가 (기존 패턴 유지)
- [ ] `execute_request()`에 `InsertBalanceSnapshot` 분기 추가
- [ ] `crates/arb-db/src/lib.rs`에 모듈 등록
- [ ] `examples/migrate.rs`로 마이그레이션 테스트

### Phase 3: 거래소 타입 확장

- [ ] `arb-exchange/src/types.rs`: `Balance` 구조체에 필드 추가
  - [ ] `equity: Option<Decimal>` — Bybit: API equity, Upbit: None
  - [ ] `unrealised_pnl: Option<Decimal>` — Bybit: API unrealisedPnl, Upbit: None
- [ ] `arb-exchanges/src/bybit/types.rs`: `BybitCoinBalance`에 `equity` 필드 추가 파싱
- [ ] `arb-exchanges/src/bybit/client.rs`: `convert_balance()`에서 equity, unrealisedPnl 매핑
- [ ] `ExchangeAdapter`가 `get_balances()` + `get_ticker()` 모두 지원하는지 확인
- [ ] 기존 Bybit 테스트 업데이트 (Balance 구조체 변경 반영)

### Phase 4: 비동기 파이프라인

- [ ] `arb-strategy/src/zscore/balance_recorder.rs` 생성
  - [ ] `SnapshotMsg` enum
  - [ ] `BalanceSnapshotSender` (Clone, try_send, position_id > 0 가드)
  - [ ] `BalanceRecorderTask` 구조체 (`Arc<dyn ExchangeAdapter>`, DbWriter, `next_group_id: u64`)
  - [ ] `TickerCache` (HashMap 기반, 1초 TTL, dust 필터링)
  - [ ] `spawn() -> (BalanceSnapshotSender, JoinHandle<()>)` — tokio::spawn + channel 생성
  - [ ] `handle_snapshot()`:
    - snapshot_group_id 채번 (`self.next_group_id += 1`) + created_at 생성
    - Upbit: get_balances() → KRW 추출 + 코인 목록 (dust 필터) → ticker cache → coin_value → total 합산
    - Bybit: get_balances() → Balance.equity → total 직접 사용 (합산 아님)
    - f64→Decimal 변환 가드 + 0 나누기 가드 포함 환산
    - 한쪽 실패 시 성공한 쪽만 기록 (partial)
    - Row 조립 → db_writer.send()
  - [ ] `select! biased;` 루프 (채널 우선 + interval 타이머)
  - [ ] Shutdown 시 최종 Periodic 강제 기록
  - [ ] `MonitoringCounters::balance_snapshot_dropped` 추가
- [ ] 설정: `BalanceSnapshotConfig` 추가 (strategy.toml 파싱)

### Phase 5: 전략 통합

- [ ] `monitor_live.rs`에 `BalanceSnapshotSender` 주입
- [ ] 포지션 진입 후 `on_position_entry()` 호출
- [ ] 포지션 청산 후 `on_position_exit()` 호출
- [ ] 시작 시 `BalanceRecorderTask::spawn()` 호출
- [ ] 종료 시 `shutdown()` + task await

### Phase 6: 검증

- [ ] 단위 테스트: 환산 공식 (KRW→USD, KRW→USDT, 0 나누기 가드)
- [ ] 단위 테스트: f64→Decimal 변환 실패 케이스 (NaN, Infinity → 0 fallback)
- [ ] 단위 테스트: dust 필터링 (1,000 KRW 미만 제외, avg_buy_price=0 케이스)
- [ ] 단위 테스트: coin_value 계산 (Upbit 코인 합산, Bybit unrealisedPnl)
- [ ] 단위 테스트: Bybit total = equity 직접 사용 (available+locked+coin_value와 별개)
- [ ] 단위 테스트: partial 스냅샷 (한쪽 거래소 실패 시 1행만 기록)
- [ ] 통합 테스트: 채널 send → DbWriter → DB INSERT 확인
- [ ] 라이브 테스트: 10분 PERIODIC 기록 확인
- [ ] 라이브 테스트: 포지션 진입/청산 시 스냅샷 확인
- [ ] 라이브 테스트: snapshot_group_id로 2행 그룹핑 확인 (session_id 범위 내)

---

## 10. 데이터 활용 예시

> **주의**: `total_usd`/`total_usdt` 시계열 비교 시, `usd_krw`/`usdt_krw` 환율 변동 효과가
> 포함되어 있음에 유의. 순수 거래 수익을 보려면 동일 currency(KRW/USDT) 기준 `total` 변동을
> 확인하거나, `positions` 테이블의 PnL을 참조할 것.

### 10.1 세션별 총자산 변동 조회

```sql
SELECT
    snapshot_group_id,
    created_at,
    record_type,
    SUM(total_usd) AS total_portfolio_usd,
    SUM(total_usdt) AS total_portfolio_usdt
FROM balance_snapshots
WHERE session_id = ?
GROUP BY snapshot_group_id, created_at, record_type
HAVING COUNT(*) = 2   -- 완전한 그룹만 (partial 제외)
ORDER BY created_at;
```

### 10.2 포지션별 잔고 변동 (진입 vs 청산)

```sql
SELECT
    position_id,
    record_type,
    cex,
    available,
    locked,
    coin_value,
    total,
    total_usd,
    total_usdt,
    created_at
FROM balance_snapshots
WHERE position_id = ?
ORDER BY created_at;
```

### 10.3 세션 수익률 계산 (currency별 분리 — 권장)

> **`total_usdt` 합산 수익률은 환율 변동(김치 프리미엄)이 혼재**되므로,
> 순수 거래 수익을 보려면 각 거래소의 기축통화 기준 `total` 변동을 분리해서 확인한다.
> 정확한 거래 PnL은 `positions` 테이블의 `SUM(realized_pnl)`을 참조할 것.

```sql
-- 거래소별 자산 변동 (환율 효과 제외)
WITH boundaries AS (
    SELECT
        cex,
        currency,
        FIRST_VALUE(total) OVER (PARTITION BY cex ORDER BY created_at ASC) AS start_total,
        FIRST_VALUE(total) OVER (PARTITION BY cex ORDER BY created_at DESC) AS end_total
    FROM balance_snapshots
    WHERE session_id = ?
      AND record_type = 'PERIODIC'
)
SELECT DISTINCT
    cex,
    currency,
    start_total,
    end_total,
    end_total - start_total AS delta
FROM boundaries;
```

```sql
-- 포트폴리오 합산 수익률 (환율 변동 포함 — 참고용)
-- 주의: total_usdt는 usdt_krw 환율 변동 효과가 포함됨
WITH ranked AS (
    SELECT
        snapshot_group_id,
        created_at,
        SUM(total_usdt) AS portfolio_usdt,
        ROW_NUMBER() OVER (ORDER BY created_at ASC) AS rn_asc,
        ROW_NUMBER() OVER (ORDER BY created_at DESC) AS rn_desc
    FROM balance_snapshots
    WHERE session_id = ?
    GROUP BY snapshot_group_id, created_at
    HAVING COUNT(*) = 2   -- 완전한 그룹만
)
SELECT
    CASE WHEN rn_asc = 1 THEN 'start' ELSE 'end' END AS point,
    portfolio_usdt
FROM ranked
WHERE rn_asc = 1 OR rn_desc = 1;
```
