# 0006 — 오더북 기반 슬리피지 제어 및 동적 포지션 사이징

## 목적

소형 알트코인(낮은 TVL/유동성)에서 더 많은 차익거래 기회를 포착하되, 오더북 실시간 조회를 통해 슬리피지+수수료로 인한 손실을 원천 차단한다.

## 배경

### 현재 문제점

1. **min_volume_1h_usdt = 500,000**: 거래대금 필터가 높아 소형 알트코인이 제외됨. 실제로 소형 알트코인(AXS, BERA, ELSA)이 대형(ETH, SOL)보다 훨씬 높은 수익률을 기록.
2. **고정 포지션 크기**: `position_ratio(0.1)` × `total_capital_usdt`로 모든 코인에 동일한 크기로 진입. 유동성이 낮은 코인에서 슬리피지 손실 위험.
3. **단일 포지션**: 코인당 1개 포지션만 허용. 좋은 기회가 반복되어도 추가 진입 불가.

### 해결 방향

- 거래대금 필터를 낮춰(50,000 USDT) 소형 알트코인 포함
- 오더북 호가 시뮬레이션으로 "슬리피지+수수료 < 스프레드 수익"인 최대 볼륨만 진입
- 코인당 복수 독립 포지션 허용 (자본 한도 내 재진입)
- 청산도 동일하게 슬리피지 안전 볼륨으로 분할 청산

## 요구사항

### 1. 설정 변경

#### 변경되는 파라미터

| 파라미터 | 변경 전 | 변경 후 | 설명 |
|---------|--------|--------|------|
| `min_volume_1h_usdt` | 500,000 (기본값 1,000,000) | 50,000 | 소형 알트코인 포함 |
| `position_ratio` | 0.1 (고정 포지션 크기) | **제거** | 동적 사이징으로 대체 |

#### 신규 파라미터

| 파라미터 | 기본값 | 설명 |
|---------|-------|------|
| `max_position_ratio` | 0.2 | 코인 페어당 최대 포지션 크기 = `total_capital_usdt × max_position_ratio` |
| `safe_volume_ratio` | 동적 | 오더북 시뮬레이션 결과의 N%만 실제 진입 (거래대금 기반 동적 조정, 아래 참조) |
| `grace_period_hours` | 4 | TTL 초과 후 분할 청산 유예 기간. 유예 기간 후 슬리피지 무시 전량 청산 |
| `entry_cooldown_sec` | 10 | 같은 코인에 대한 재진입 최소 간격 (초). 시뮬레이션에서 오더북 소비 미반영 보완 |
| `max_cache_age_sec` | 5 | 오더북 캐시 최대 유효 시간 (초). 초과 시 해당 틱의 시그널 평가를 스킵 |

#### safe_volume_ratio 동적 조정

1시간 거래대금에 따라 safe_volume_ratio를 동적으로 결정한다. 유동성이 낮을수록 오더북 변동이 크므로 더 보수적으로 진입한다.

| 1시간 거래대금 (USDT) | safe_volume_ratio |
|----------------------|-------------------|
| < 100,000 | 0.5 |
| 100,000 ~ 300,000 | 0.6 |
| 300,000 ~ 500,000 | 0.7 |
| >= 500,000 | 0.8 |

#### 기존 유지 파라미터

| 파라미터 | 값 | 용도 |
|---------|---|------|
| `total_capital_usdt` | (기존) | `max_position_ratio`의 기준 자본 |
| `max_concurrent_positions` | (기존) | 전체 동시 포지션 수 제한 (포트폴리오 노출 간접 제어) |

### 2. 오더북 REST 조회 및 캐시

#### 조회 흐름

```
틱(Trade/BestQuote) 수신
  │
  ├─ computing_flag[exchange][coin] == true?
  │     └─ YES → 이 틱 스킵 (dropped_tick_count 증가)
  │
  └─ NO → computing_flag = true
           │
           ├─ REST API로 해당 거래소의 해당 코인 오더북 조회
           │   ├─ Upbit:  GET /v1/orderbook (depth 15, rate limit 10 req/sec)
           │   └─ Bybit:  GET /v5/market/orderbook (limit 25, rate limit 18 req/sec)
           │   └─ 실패 시: warn 로그, 캐시 유지, computing_flag = false, 이 틱 스킵
           │
           ├─ 오더북 캐시 업데이트 (exchange × coin × timestamp)
           │
           ├─ 반대쪽 캐시 신선도 확인 (max_cache_age_sec 초과 시 스킵)
           │
           ├─ 스프레드 계산 → Z-Score → 시그널 평가
           │   (캐시된 Upbit 오더북 + 캐시된 Bybit 오더북으로 슬리피지 계산)
           │
           └─ computing_flag = false
```

#### 캐시 구조

```
OrderBookCache {
    upbit: HashMap<String, CachedOrderBook>,   // coin → (OrderBook, 조회 시각)
    bybit: HashMap<String, CachedOrderBook>,   // coin → (OrderBook, 조회 시각)
}

CachedOrderBook {
    orderbook: OrderBook,
    fetched_at: Instant,
}
```

- 틱이 발생한 거래소의 오더북만 REST로 재조회
- 반대쪽 거래소는 캐시된 오더북을 사용
- **캐시 만료**: `max_cache_age_sec` (기본값 5초) 초과한 캐시는 무효. 무효 시 해당 틱의 시그널 평가를 스킵
- 캐시 초기값: 워밍업 단계에서 모든 대상 코인의 양쪽 오더북을 1회 조회 (rate limit 내 순차 처리)

#### Computing Flag

```
computing_flags: HashMap<(Exchange, String), bool>
// (거래소, 코인) 쌍별 독립적 플래그
```

- `false` → `true`: 틱 수신 시 설정
- `true` → `false`: 오더북 조회 + 시그널 평가 완료 후 해제
- 이미 `true`인 경우: 해당 틱 무시 (drop), `dropped_tick_count` 카운터 증가
- 현재 단일 tokio task 구조이므로 일반 `bool`로 충분 (향후 병렬화 시 `AtomicBool` 전환 가능)

#### REST 조회 실패 처리

| 실패 유형 | 처리 |
|----------|------|
| 네트워크 오류 | warn 로그, 캐시 유지, 이 틱 스킵 |
| Rate limit 초과 (429) | 기존 rate limiter가 사전 방지. 돌파 시 warn 로그 + 스킵 |
| 타임아웃 (3초) | warn 로그, 캐시 유지, 이 틱 스킵 |
| 빈 오더북 응답 | warn 로그, 캐시 갱신하지 않음, 이 틱 스킵 |

### 3. 슬리피지 안전 볼륨 계산

#### 진입 시 (Upbit 매수 + Bybit 숏)

오더북 호가를 **two-pointer 방식**으로 양쪽 동시 순차 소비하며, 누적 VWAP 기준으로 수익성을 검증한다.

```
입력:
  upbit_asks:  [(p1, s1), (p2, s2), ...]   # Upbit 매도호가 (내가 매수), KRW 단위
  bybit_bids:  [(p1, s1), (p2, s2), ...]   # Bybit 매수호가 (내가 숏), USDT 단위
  mean_spread_pct: f64                      # 현재 rolling mean spread
  upbit_fee: Decimal                        # Upbit taker 수수료율
  bybit_fee: Decimal                        # Bybit taker 수수료율
  usd_krw: f64                              # 현재 환율

Two-Pointer 알고리즘:
  upbit_ptr = 0, bybit_ptr = 0              # 각 오더북의 현재 호가 인덱스
  upbit_remaining = upbit_asks[0].size       # 현재 호가의 잔여 수량
  bybit_remaining = bybit_bids[0].size
  total_coins = 0                            # 누적 소비 코인 수
  upbit_cost_krw = 0                         # 누적 Upbit 매수 비용 (KRW)
  bybit_revenue_usdt = 0                     # 누적 Bybit 숏 수익 (USDT)

  반복:
    1. 이번 단계 소비량 = min(upbit_remaining, bybit_remaining)
    2. upbit_cost_krw += 이번 소비량 × upbit_asks[upbit_ptr].price
       bybit_revenue_usdt += 이번 소비량 × bybit_bids[bybit_ptr].price
       total_coins += 이번 소비량
    3. upbit_remaining -= 이번 소비량
       bybit_remaining -= 이번 소비량
    4. 잔여 0인 쪽 → 다음 호가로 이동 (ptr++), remaining 갱신
       양쪽 모두 소진 시 양쪽 다 이동
    5. 수익성 검증:
       upbit_vwap_usd = (upbit_cost_krw / total_coins) / usd_krw   # KRW → USDT 환산
       bybit_vwap = bybit_revenue_usdt / total_coins               # 이미 USDT
       effective_spread = (bybit_vwap - upbit_vwap_usd) / upbit_vwap_usd × 100
       # NOTE: effective_spread는 VWAP 기반이므로 진입 슬리피지가 내재되어 있음
       #       mean_spread_pct는 mid-price(최우선호가) 기반 통계값
       #       estimated_exit_slippage는 청산 시 추가 발생할 슬리피지 비용
       roundtrip_fee = (upbit_fee + bybit_fee) × 2 × 100
       entry_slippage_pct = (upbit_vwap_usd - best_ask_usd) / best_ask_usd × 100
                          + (best_bid - bybit_vwap) / best_bid × 100
       estimated_exit_slippage = entry_slippage_pct   # 청산 시에도 동일 규모 슬리피지 가정
       profit = (effective_spread - mean_spread_pct) - roundtrip_fee - estimated_exit_slippage
    6. profit > 0 → 계속 소비
       profit ≤ 0 → 직전 단계의 total_coins가 최대 안전 볼륨
    7. 어느 한 쪽 오더북 끝에 도달 → 현재 total_coins가 상한

결과:
  safe_volume_coins: Decimal   # 안전하게 진입 가능한 코인 수량
  safe_volume_usdt: Decimal    # USDT 환산 (safe_volume_coins × bybit_vwap)
```

#### 진입 슬리피지 계산

```
entry_slippage_pct = (upbit_vwap_usd - best_ask_usd) / best_ask_usd × 100
                   + (best_bid - bybit_vwap) / best_bid × 100
```

이 값을 `estimated_exit_slippage`로도 사용하여 라운드트립 슬리피지를 보수적으로 추정한다.

#### 청산 시 (Upbit 매도 + Bybit 롱 커버)

진입의 역방향:
- Upbit: **매수호가(bid)** 소비 (내가 매도)
- Bybit: **매도호가(ask)** 소비 (내가 매수하여 숏 청산)

수익성 기준: 포지션의 **진입 스프레드 대비 현재 스프레드 변동**에서 슬리피지+수수료를 뺀 값이 양수인 경우만 청산.

#### 최소 주문 크기 검증

계산된 안전 볼륨이 거래소 최소 주문 크기보다 작으면 진입/청산하지 않는다.

| 거래소 | 최소 주문 크기 |
|--------|--------------|
| Upbit | 5,000 KRW |
| Bybit | 코인별 상이 (API로 조회 또는 config) |

safe_volume_usdt가 양쪽 최소 주문 크기를 모두 충족해야 진입/청산 가능.

### 4. 진입 볼륨 결정

```
실제 진입 볼륨 = min(
  safe_volume_usdt × safe_volume_ratio(거래대금 기반 동적),   # 오더북 안전 볼륨
  total_capital_usdt × max_position_ratio - coin_open_total_usdt  # 코인별 자본 한도 잔여
)
```

- `coin_open_total_usdt`: 해당 코인의 모든 열린 포지션 `size_usdt` 합계
- 결과가 0 이하 또는 최소 주문 크기 미만이면 진입하지 않음

### 5. 포지션 관리 변경

#### 현재 구조

```rust
// 코인당 1개 포지션
open_positions: HashMap<String, VirtualPosition>
```

#### 변경 후 구조

```rust
// 코인당 복수 독립 포지션
open_positions: HashMap<String, Vec<VirtualPosition>>

// VirtualPosition에 고유 ID 추가
pub struct VirtualPosition {
    pub id: u64,                    // 시퀀스 번호 (AtomicU64 카운터에서 발급)
    pub coin: String,
    pub entry_time: DateTime<Utc>,
    pub size_usdt: Decimal,
    // ... 기존 필드 유지
}
```

- 각 진입은 독립적인 `VirtualPosition`으로 생성
- 포지션 ID는 **단조 증가 시퀀스 번호** (`AtomicU64` 카운터)로 유일성 보장
- 포지션별 독립적인 진입 가격, 스프레드, Z-Score 기록
- **부분 청산 시 잔여 포지션의 ID는 동일 유지** (size_usdt만 감소, 진입 가격 등 불변)

#### PositionManager API 변경

| 메서드 | 변경 전 | 변경 후 |
|--------|--------|--------|
| `has_position(coin)` | `contains_key()` | `get(coin).map(|v| !v.is_empty())` |
| `open_position(pos)` | `AlreadyExists` 에러 | Vec에 push (한도 검증은 호출측) |
| `close_position(coin, ...)` | 코인명으로 식별 | `close_position(coin, position_id, ...)` |
| `close_partial(coin, id, partial_size_usdt, exit_upbit_vwap, exit_bybit_vwap, exit_usd_krw, ...)` | 없음 | **신규**: 부분 청산, `(ClosedPosition, Option<VirtualPosition>)` 반환. VWAP exit price 직접 전달 |
| `used_capital()` | `.values().map(size)` | `.values().flat_map().map(size)` |
| `coin_used_capital(coin)` | 없음 | **신규**: 코인별 사용 자본 합계 |
| `open_count()` | `.len()` | `.values().map(len).sum()` |
| `check_liquidation(coin, price)` | 단일 포지션 | Vec 내 전체 순회 |

#### 재진입 조건

1. Z-Score가 진입 임계값 이상
2. 슬리피지+수수료 포함 수익성 양수 (청산 슬리피지 추정 포함)
3. `coin_used_capital(coin) < total_capital_usdt × max_position_ratio`
4. 마지막 진입 후 `entry_cooldown_sec` (10초) 경과

#### Cooldown 추적

```rust
// PositionManager에 추가
last_entry_time: HashMap<String, DateTime<Utc>>   // coin → 마지막 진입 시각
```

- `open_position()` 호출 시 갱신
- 진입 평가 시 `Utc::now() - last_entry_time[coin] >= entry_cooldown_sec` 확인

### 6. 시그널 평가 변경

#### 현재 로직 (signal.rs)

```
if has_position(coin) {
    // 청산만 평가
} else {
    // 진입만 평가
}
```

#### 변경 후 로직

```
// 진입과 청산을 독립적으로 평가

// 1. 청산 평가: 포지션이 있고 Z-Score가 exit_z 이하면 청산 시그널
if has_positions(coin) && z_score <= exit_z_threshold {
    → Signal::Exit
}

// 2. 진입 평가: Z-Score가 entry_z 이상이고, 자본 한도 내이고, cooldown 경과
if z_score >= entry_z_threshold
   && expected_profit > 0
   && coin_used_capital < max_limit
   && cooldown_elapsed {
    → Signal::Enter  (size_usdt는 monitor.rs에서 오더북 기반으로 결정)
}
```

- **signal.rs의 책임**: 진입/청산 여부만 판단 (Z-Score + 수수료 기반 수익성)
- **monitor.rs의 책임**: 실제 진입 크기 결정 (오더북 슬리피지 시뮬레이션)
- 같은 틱에서 일부 포지션 청산 + 신규 진입이 동시에 발생할 수 있음
- **처리 순서: 청산 우선(exit-first)**. 청산을 먼저 처리하여 자본을 회수한 후 진입을 평가. 이를 통해 자본 활용 효율 극대화

### 7. 분할 청산

#### 청산 시그널 발생 시 처리 흐름

```
1. Z-Score가 exit_z_threshold 이하로 복귀 (또는 position_ttl_hours 초과)

2. 해당 코인의 오더북 캐시로 청산 안전 볼륨 계산
   (Upbit bid + Bybit ask 방향, two-pointer 동일 알고리즘)

3. 열린 포지션을 수익률 내림차순 정렬
   수익률 = (현재 스프레드 - 진입 스프레드) / size_usdt 기준

4. 수익률 높은 포지션부터 순서대로:
   a. 남은 안전 볼륨 ≥ 포지션 size_usdt → 전량 청산
   b. 남은 안전 볼륨 < 포지션 size_usdt → 부분 청산
      - 안전 볼륨만큼 청산 (ClosedPosition 생성)
      - 나머지는 축소된 VirtualPosition으로 유지
   c. 남은 안전 볼륨 = 0 → 이후 포지션은 다음 틱으로 이월
```

#### 2단계 강제 청산

| 단계 | 조건 | 동작 |
|------|------|------|
| 1단계 | `position_ttl_hours` 초과 | 안전 볼륨 기반 분할 청산 시도 (일반 청산과 동일) |
| 2단계 | `position_ttl_hours + grace_period_hours` 초과 | 슬리피지 무시, **전량 즉시 시장가 청산** |

- `grace_period_hours` 기본값: 4시간
- 2단계 도달 시 safe_volume_ratio를 1.0으로 설정하고, 오더북 전체 depth를 소비하여 강제 청산
- 이는 "TTL 안전장치가 무력화되지 않도록" 하는 최후 방어선

#### 부분 청산 시 PnL 계산 (VWAP 기반)

부분 청산 시 단순 비례 배분이 아닌, 오더북에서 계산된 **VWAP exit price**를 사용하여 정밀 계산한다.

```
부분 청산 200 USDT (원래 포지션 500 USDT):

1. 오더북에서 200 USDT분의 VWAP 계산:
   exit_upbit_price = VWAP(Upbit bids, partial_coins)
   exit_bybit_price = VWAP(Bybit asks, partial_coins)

2. PnL 계산 (기존 close_position 로직 동일, partial_size 기준):
   upbit_qty = partial_size_usdt / entry_upbit_price
   bybit_qty = partial_size_usdt / entry_bybit_price
   upbit_pnl = (exit_upbit_price - entry_upbit_price) × upbit_qty
   bybit_pnl = (entry_bybit_price - exit_bybit_price) × bybit_qty
   fees = partial_size_usdt × (upbit_fee + bybit_fee) × 2

3. 결과:
   ClosedPosition { size_usdt: 200, exit prices: VWAP 값, ... }
   잔여 VirtualPosition { size_usdt: 300, 진입 가격 등 동일 유지 }
```

### 8. TTL 체크 타이밍

`position_ttl_hours` 만료 체크는 코인 재선택 타이머와 **분리**하여 독립적으로 수행한다.

- **분봉 타이머(1분)**: 분봉 캔들 기록과 함께 TTL 만료 포지션을 검사
- **코인 재선택 타이머**: `auto_select = false`일 때에도 TTL 체크가 누락되지 않도록 분리
- TTL 체크 시 2단계 강제 청산 로직도 함께 평가

### 9. 비동기 처리 구조

현재 `monitor.rs`의 이벤트 루프는 `tokio::select!` 기반이다. 오더북 REST 조회는 비동기로 처리한다.

```
tokio::select! {
    event = rx.recv() => {
        // 틱 수신
        if !computing_flag[exchange][coin] {
            computing_flag[exchange][coin] = true;

            // 오더북 조회 (async, 실패 시 스킵)
            match exchange_client.get_orderbook(market, depth).await {
                Ok(ob) => orderbook_cache.update(exchange, coin, ob),
                Err(e) => {
                    warn!("오더북 조회 실패: {e}");
                    computing_flag[exchange][coin] = false;
                    continue;
                }
            }

            // 반대쪽 캐시 신선도 확인
            if !orderbook_cache.is_fresh(other_exchange, coin, max_cache_age_sec) {
                computing_flag[exchange][coin] = false;
                continue;
            }

            // 스프레드/Z-Score/시그널 평가 (기존 로직)
            // 진입/청산 시 안전 볼륨 계산

            computing_flag[exchange][coin] = false;
        } else {
            dropped_tick_count += 1;
        }
    }
    ...
}
```

### 10. 모니터링 카운터

시스템 성능 파악을 위해 다음 카운터를 **`MonitoringCounters` 구조체**로 묶어 관리한다.

```rust
pub struct MonitoringCounters {
    pub dropped_tick_count: u64,
    pub orderbook_fetch_count: u64,
    pub orderbook_fetch_fail_count: u64,
    pub stale_cache_skip_count: u64,
    pub entry_rejected_slippage_count: u64,
    pub partial_close_count: u64,
    pub forced_liquidation_count: u64,
}
```

| 카운터 | 설명 |
|--------|------|
| `dropped_tick_count` | computing flag로 스킵된 틱 수 |
| `orderbook_fetch_count` | 성공한 오더북 REST 조회 수 |
| `orderbook_fetch_fail_count` | 실패한 오더북 REST 조회 수 |
| `stale_cache_skip_count` | 캐시 만료로 스킵된 평가 수 |
| `entry_rejected_slippage_count` | 슬리피지 부족으로 진입 거부된 횟수 |
| `partial_close_count` | 부분 청산 횟수 |
| `forced_liquidation_count` | 2단계 강제 청산 횟수 |

세션 요약에 `MonitoringCounters`의 모든 필드를 포함한다.

## 설정 예시 (strategy.toml)

```toml
[zscore]
auto_select = true
max_coins = 5
min_volume_1h_usdt = 50000.0         # 변경: 500,000 → 50,000
total_capital_usdt = 10000.0
max_position_ratio = 0.2              # 신규: 코인당 최대 자본 비율
# safe_volume_ratio: 거래대금 기반 동적 (0.5 ~ 0.8)
grace_period_hours = 4                # 신규: 강제 청산 유예 기간
entry_cooldown_sec = 10               # 신규: 재진입 최소 간격
max_cache_age_sec = 5                 # 신규: 오더북 캐시 최대 유효 시간
# position_ratio 제거됨
```

## 파일 변경 목록

### 신규 파일

| 파일 | 설명 |
|------|------|
| `crates/arb-strategy/src/zscore/orderbook.rs` | 오더북 캐시, two-pointer 슬리피지 안전 볼륨 계산, VWAP 계산 |

### 수정 파일

| 파일 | 변경 내용 |
|------|----------|
| `config.rs` | `position_ratio` 제거, `max_position_ratio` + `grace_period_hours` + `entry_cooldown_sec` + `max_cache_age_sec` 추가, `min_volume_1h_usdt` 기본값 변경 |
| `position.rs` | `HashMap<String, VirtualPosition>` → `HashMap<String, Vec<VirtualPosition>>`, `id: u64` 필드 추가, `last_entry_time: HashMap<String, DateTime<Utc>>` 추가, `close_partial()` 메서드 추가, `coin_used_capital()` 메서드 추가, `available_capital()` 제거 (monitor.rs에서 직접 계산) |
| `signal.rs` | 배타적 분기 제거 → 진입/청산 독립 평가, `position_ratio` 제거, cooldown 체크 추가 |
| `monitor.rs` | computing flag, 오더북 REST 조회, 캐시 관리, 안전 볼륨 기반 진입/청산, VWAP exit price, 청산 우선(exit-first) 처리 순서, `MonitoringCounters` 구조체, TTL 체크 분봉 타이머 분리 |
| `pnl.rs` | `id: u64` 필드 추가 |
| `error.rs` | `AlreadyExists` 에러 variant 제거 (복수 포지션 허용으로 불필요) |
| `output/summary.rs` | `MonitoringCounters` 7개 필드를 SessionSummary에 추가 |
| `output/writer.rs` | `ClosedPosition`에 `id: u64` 필드 추가로 인한 CSV/JSON 컬럼 대응 |

### 변경하지 않는 파일

| 파일 | 이유 |
|------|------|
| `spread.rs` | 스프레드 계산 로직 변경 없음 (틱 가격 기반 유지) |
| `statistics.rs` | IncrementalStats 변경 없음 |
| `coin_selector.rs` | `min_volume_1h_usdt` 값만 config에서 변경, 로직 변경 없음 |
| 거래소 SDK | `get_orderbook()` 이미 구현 완료 |

## 검증

1. `cargo test -p arb-strategy` — 기존 테스트 수정 + 신규 테스트 통과
2. `cargo clippy -p arb-strategy` — 경고 0
3. 라이브 테스트:
   - 오더북 REST 조회가 rate limit 내에서 정상 동작 확인
   - computing flag로 중복 조회 방지 확인
   - 소형 알트코인(min_volume_1h_usdt = 50,000) 포함 확인
   - 복수 포지션 진입/청산 정상 동작 확인
   - 부분 청산 시 VWAP PnL 정확성 확인
   - 2단계 강제 청산 동작 확인
   - entry_cooldown_sec 재진입 제한 확인
   - 모니터링 카운터 summary 출력 확인

## 알려진 한계

- **오더북 스냅샷 지연**: REST 조회 시점과 실제 주문 실행 시점 사이에 오더북이 변할 수 있음. 거래대금 기반 동적 `safe_volume_ratio`로 완화.
- **반대쪽 오더북 캐시 신선도**: 틱이 발생한 거래소만 재조회하므로, 반대쪽 오더북은 이전 캐시를 사용. `max_cache_age_sec` (5초) 초과 시 스킵으로 완화.
- **부분 청산 누적**: 슬리피지가 지속적으로 높으면 포지션이 오래 열려 있을 수 있음. 2단계 강제 청산(`grace_period_hours`)으로 무한 이월 방지.
- **Leg Risk**: 가상 포지션(시뮬레이션)이므로 실제 주문 실행의 한쪽 leg 실패 리스크는 반영하지 않음. 실제 주문 실행으로 전환 시 별도 처리 필요.
- **Computing flag 시그널 손실**: 오더북 REST 조회 중 도착하는 틱은 스킵됨. `dropped_tick_count`로 모니터링하여 과도한 손실 시 rate limit 또는 depth 조정.
- **Upbit depth 15 상한**: Upbit 오더북 15단계가 실질적 유동성 추정 상한. 이는 보수적 추정이므로 안전한 방향.
- **시뮬레이션 오더북 미소비**: 시뮬레이션에서 실제 주문을 내지 않으므로 내 주문에 의한 오더북 소비가 반영되지 않음. `entry_cooldown_sec` (10초)로 완화.
- **이벤트 루프 블로킹**: 오더북 REST 조회가 이벤트 루프 내에서 await되므로, 조회 지연(~100ms) 동안 다른 코인의 틱 처리가 지연됨. 현재 단일 task 구조의 본질적 한계이며, 향후 코인별 독립 task 분리로 해결 가능.
- **유동성 급감 대응**: 오더북이 순간적으로 비어있거나 depth가 극단적으로 얇아지는 상황(flash crash 등)에서는 safe_volume이 0으로 계산되어 진입/청산 불가. 이는 안전한 방향의 동작.
- **거래소 장애 대응**: 한쪽 거래소의 REST API가 장기간 장애인 경우, 해당 거래소의 캐시가 만료되어 모든 시그널 평가가 스킵됨. 장애 복구 후 자동으로 정상 동작 재개.
