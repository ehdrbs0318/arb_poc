# 0008 — 틱 크기 및 거래 단위 라운딩

## 목적

거래소별 코인의 **가격 호가 단위(tick size)**, **수량 단위(lot size/qty step)**, **최소 주문 조건(min order qty/amt)**을 시뮬레이션에 반영하여, 실거래 전환 시 주문 거부 없이 즉시 적용 가능한 정확한 포지션 사이징을 구현한다.

추가로 PnL 계산을 **코인 수량(qty) 기반**으로 전환하여, 양 거래소에 동일 수량을 주문하는 실거래 방식과 정합성을 확보한다.

## 배경

### 현재 문제점

1. **수량 라운딩 없음**: `size_usdt`를 USDT 금액으로만 관리하며, 실제 코인 수량(qty)으로 변환 시 거래소가 요구하는 `qty_step` 단위에 맞추지 않음.
2. **가격 라운딩 없음**: Upbit KRW 마켓은 가격대별로 호가 단위가 다름 (예: 1만원 이상 → 10원 단위, 100원 미만 → 0.1원 단위). Bybit도 코인별 `tick_size`가 다름.
3. **최소 주문 검증 부재**: Upbit 최소 주문 금액(5,100 KRW), Bybit 코인별 `min_order_qty`를 검증하지 않음. 현재 `size_usdt <= 5.0` 하드코딩으로만 필터링.
4. **시뮬레이션 정확도 저하**: 라운딩 미적용으로 실거래 대비 수량/가격에 오차가 발생하여 PnL 시뮬레이션이 실제보다 낙관적.
5. **PnL 계산 구조 불일치**: 현재 `upbit_qty = size_usdt / upbit_price`, `bybit_qty = size_usdt / bybit_price`로 양 leg 수량이 다름. 실거래에서는 양 거래소에 **동일한 코인 수량**을 주문해야 차익거래 헤지가 성립함.

### 영향 범위

| 거래소 | 제약 항목 | 현재 코드 반영 여부 |
|--------|----------|------------------|
| **Upbit** | KRW 호가 단위 (가격대별 17단계) | 미반영 |
| **Upbit** | 최소 주문 금액 5,100 KRW | 미반영 |
| **Upbit** | 수량 단위 (소수점 8자리) | 미반영 |
| **Bybit** | tick_size (코인별 가격 최소 단위) | 미반영 |
| **Bybit** | qty_step (코인별 수량 최소 단위) | 미반영 |
| **Bybit** | min_order_qty (코인별 최소 주문 수량) | 미반영 |
| **Bybit** | min_notional_value (최소 주문 금액) | 미반영 |
| **Bybit** | max_order_qty (코인별 최대 주문 수량) | 미반영 |

## 요구사항

### 확정 사항 (사용자 결정)

| 항목 | 결정 |
|------|------|
| 적용 범위 | 시뮬레이션에도 즉시 반영 |
| Upbit 호가 단위 관리 | 하드코딩 테이블 (변경 빈도 극히 낮음) |
| Bybit instrument info | 시작 시 REST 조회 후 캐싱, 코인 재선택 시 자동 갱신 |
| 라운딩 방향 | 항상 내림(floor), 가격은 불리한 방향 — 보수적 접근 |
| 최소 주문 미달 처리 | 진입 거부 + 로그 + 카운터 |
| 청산 적용 | 진입 + 청산 모두 tick/lot 라운딩 적용 |
| SDK trait 설계 | 별도 `InstrumentDataProvider` trait (MarketData 오염 방지) |
| 유틸리티 배치 | `arb-strategy/src/zscore/instrument.rs`에 함수 + 캐시 배치 |
| PnL 계산 전환 | 이번 스펙에 포함 — qty 기반으로 전환 |

### 가정 및 전제

- **Bybit qty_step ≥ Upbit qty_step**: Bybit의 qty_step(0.01, 0.1 등)이 Upbit의 qty_step(0.00000001)보다 항상 거칠다고 가정한다. 따라서 Bybit qty_step으로 라운딩하면 Upbit 조건도 자동 만족. 이 가정이 깨지는 경우 **assertion 대신 warn 로그 + 진입 거부**로 방어한다 (런타임 패닉 방지).
- **가격 라운딩은 시뮬레이션 보수성을 위한 것**이며, 실거래 전환 시에는 실제 체결 가격을 사용한다.

### 1. Upbit KRW 호가 단위 테이블

Upbit 공식 문서(https://docs.upbit.com/kr/docs/krw-market-info) 기반.
경계값은 **이상(≥) ~ 미만(<)** 규칙:

| KRW 가격 구간 | 호가 단위 |
|--------------|----------|
| 2,000,000 이상 | 1,000 |
| 1,000,000 ~ 2,000,000 미만 | 1,000 |
| 500,000 ~ 1,000,000 미만 | 500 |
| 100,000 ~ 500,000 미만 | 100 |
| 50,000 ~ 100,000 미만 | 50 |
| 10,000 ~ 50,000 미만 | 10 |
| 5,000 ~ 10,000 미만 | 5 |
| 1,000 ~ 5,000 미만 | 1 |
| 100 ~ 1,000 미만 | 1 |
| 10 ~ 100 미만 | 0.1 |
| 1 ~ 10 미만 | 0.01 |
| 0.1 ~ 1 미만 | 0.001 |
| 0.01 ~ 0.1 미만 | 0.0001 |
| 0.001 ~ 0.01 미만 | 0.00001 |
| 0.0001 ~ 0.001 미만 | 0.000001 |
| 0.00001 ~ 0.0001 미만 | 0.0000001 |
| 0.00001 미만 | 0.00000001 |

> **구현 참고**: 첫 두 구간(≥2,000,000과 ≥1,000,000)은 호가 단위가 동일(1,000)하므로 `>= 1,000,000 → 1,000`으로 병합 가능.

- **최소 주문 금액**: 5,100 KRW (공식 최소 5,000 KRW + 2% 안전 마진, 환율 변동 대비). 환율 갱신 주기가 10분이므로 최대 10분간의 환율 변동을 커버해야 하며, 일중 USD/KRW 변동률은 보통 0.3~0.5%(극단 1~2%)이므로 2%는 충분한 마진임.
- **수량 정밀도**: 소수점 8자리 (`qty_step = 0.00000001`)

### 2. Bybit Instrument Info

Bybit `/v5/market/instruments-info` API에서 다음 필드를 사용:

```json
{
  "priceFilter": {
    "tickSize": "0.0001"
  },
  "lotSizeFilter": {
    "minOrderQty": "0.1",
    "qtyStep": "0.1",
    "maxOrderQty": "1190.000",
    "minNotionalValue": "5"
  }
}
```

- **조회 시점**: 모니터 시작 시 + 코인 재선택 시 (새 코인에 대해)
- **캐싱**: `InstrumentCache` 구조체에 코인별로 저장, 세션 동안 유지
- **재선택 시 edge case**: InstrumentCache에 해당 코인 info가 없으면 진입 거부 + 로그. `get(coin) == None`이 자연스러운 거부 역할을 하므로 별도 flag 불필요.
- **API 실패 시**: REST 호출 실패(rate limit, 네트워크 타임아웃 등) → warn 로그 + 해당 코인만 InstrumentCache에 미등록. 다른 코인은 정상 진행. 모니터 자체는 중단하지 않음. 특정 코인이 API에 없는 경우(상장 폐지 등)도 동일 처리.

### 3. 라운딩 정책

#### 수량 라운딩: 항상 floor

```
floor_to_step(qty, qty_step)
  예: qty=123.456, qty_step=0.01 → 123.45
```

#### 가격 라운딩: 불리한 방향 (시뮬레이션 보수성)

```
진입 시:
  Upbit 매수가: ceil_to_tick(price, tick_size)   ← 더 비싸게 삼
  Bybit short:  floor_to_tick(price, tick_size)  ← 더 싸게 팜

청산 시:
  Upbit 매도가: floor_to_tick(price, tick_size)  ← 더 싸게 팜
  Bybit close:  ceil_to_tick(price, tick_size)   ← 더 비싸게 삼
```

#### 방어 조건

- `step == 0`: 라운딩 스킵 (원본 반환) + warn 로그
- `value < 0`: `Decimal::ZERO` 반환
- `qty == 0` (floor 결과): 진입/청산 거부

### 4. 적용 지점

#### 4.1 진입 시 (spawned_check_tick_signal)

```
1. 오더북 기반 safe_volume_usdt 계산 (기존)
2. size_usdt 결정 (기존)
3. ★ 신규: size_usdt → 코인 수량(qty) 변환
     qty = size_usdt / bybit_price
4. ★ 신규: qty를 Bybit qty_step으로 floor 라운딩
     qty = floor_to_step(qty, bybit_qty_step)
5. ★ 신규: qty가 0이면 진입 거부
6. ★ 신규: qty × bybit_price로 실제 size_usdt 재계산 (라운딩 전 가격 기준, 7단계 검증용)
     size_usdt = qty × bybit_price
7. ★ 신규: 최소/최대 주문 검증
     - qty == 0 → 거부
     - Bybit: qty < min_order_qty → 거부
     - Bybit: qty > max_order_qty → 거부
     - Bybit: size_usdt < min_notional_value → 거부
     - Upbit: qty × upbit_krw_price < 5,100 KRW → 거부
     → 미달 시 entry_rejected_order_constraint_count++ + 로그
8. ★ 신규: 진입 가격 라운딩 (시뮬레이션용)
     upbit_entry_krw = ceil_to_tick(upbit_krw, upbit_tick_size(upbit_krw))
     upbit_entry_usd = upbit_entry_krw / usd_krw   ← KRW 라운딩 후 USD 변환 (순서 중요)
     bybit_entry = floor_to_tick(bybit_price, bybit_tick_size)
9. ★ 신규: 라운딩 후 수익성 재검증 (post-rounding PnL gate)
     라운딩으로 인한 스프레드 비용을 기존 기대 수익에서 차감한다.
     스프레드 정의: spread = (bybit_usd - upbit_usd) / upbit_usd × 100
     original_spread = signal에서 전달된 스프레드 (원시 가격 기반, Signal::Enter.spread_pct)
     adjusted_spread = (bybit_entry - upbit_entry_usd) / upbit_entry_usd × 100
     rounding_cost = original_spread - adjusted_spread   (≥0, 라운딩은 항상 불리한 방향)
     adjusted_profit = original_expected_profit - rounding_cost
     adjusted_profit ≤ 0 → 진입 거부 + entry_rejected_rounding_pnl_count++ + 로그 ("라운딩 후 수익성 부족")
     ※ mean_spread와 무관하게 라운딩 비용만 차감하므로 통계적 노이즈가 혼입되지 않음
10. VirtualPosition 생성 (qty 포함, size_usdt는 derived 메서드로 자동 계산)
```

기존 `size_usdt <= 5.0` 하드코딩은 InstrumentInfo 기반 검증으로 **교체**한다.

#### 4.2 청산 시 (close_position / close_partial)

```
0. InstrumentInfo 조회
     InstrumentCache에서 해당 코인의 info를 조회한다.
     ★ 없을 경우 (재선택 직후 등): 라운딩 없이 원시 가격으로 청산 진행 + warn 로그.
     포지션이 열린 이상 청산을 거부하면 안 되므로, info 미존재를 이유로 청산을 실패시키지 않는다.
     ⚠ PnL 비대칭 주의: 진입은 보수적 라운딩, 청산은 원시 가격이면 PnL이 과대 추정될 수 있음.
     fallback_no_rounding_count++ 카운터로 빈도를 추적하여 영향을 모니터링한다.
1. 청산 가격 라운딩 (시뮬레이션용, spawned_check_tick_signal 내부에서 수행)
     upbit_exit_krw = floor_to_tick(upbit_krw, upbit_tick_size(upbit_krw))
     upbit_exit_usd = upbit_exit_krw / usd_krw   ← KRW 라운딩 후 USD 변환 (순서 중요)
     bybit_exit = ceil_to_tick(bybit_price, bybit_tick_size)
     라운딩된 가격을 close_position/close_partial에 전달한다.
1.5. ★ 신규: safe_volume USDT → qty 변환 (monitor.rs 호출부에서 수행)
     safe volume 비교는 기존대로 USDT 단위를 유지 (SafeVolumeResult.safe_volume_usdt).
     전량/부분 청산 분기도 기존과 동일하게 USDT 기준:
       remaining_safe_usdt >= p.size_usdt() → 전량 청산 (pos.qty 사용, 변환 불필요)
       remaining_safe_usdt <  p.size_usdt() → 부분 청산:
         partial_qty = remaining_safe_usdt / bybit_exit(라운딩 전 원시 가격)
         ※ 라운딩 전 가격 사용 이유: 라운딩 후(ceil) 가격은 더 높으므로 qty가 작아져 보수적.
           step 2에서 floor_to_step으로 추가 라운딩되므로 이중 보수성 확보.
2. 분할 청산 수량 라운딩 (PositionManager 내부에서 처리)
     partial_qty = floor_to_step(partial_qty, bybit_qty_step)
     → 0이 되면 청산 스킵 + 로그
     → 잔여 qty < min_order_qty이면 전량 청산으로 자동 전환
       ★ 전량 전환 시 오더북 safe volume을 초과할 수 있음 (예: safe=7.5, pos=10, min=3 → 전량 10 청산).
       포지션이 열린 이상 청산 거부는 더 위험하므로 전량 청산을 진행하되,
       safe_volume_exceeded_close_count++ 카운터로 빈도를 추적한다.
     close_partial()에 instrument_info 파라미터를 추가하여,
     PM 내부에서 잔량 체크 → 전량 청산 전환을 캡슐화한다.
3. PnL 계산은 qty 기반 + 라운딩된 가격으로 수행
```

#### 4.3 TTL 청산 경로 (check_ttl_positions)

TTL 만료 포지션 청산 시에도 동일하게 가격 라운딩을 적용한다.
2단계 강제 청산(grace period 초과)은 슬리피지 무시이나 가격 라운딩은 적용.
`check_ttl_positions` 파라미터에 `InstrumentCache` (Arc)를 추가하여 라운딩 정보를 참조한다.
InstrumentInfo 미존재 시: 4.2와 동일하게 라운딩 없이 청산 진행.
PnL 계산도 5절의 qty 기반으로 동일 적용 (`close_position`이 qty 기반으로 변경되므로 자동 적용).
현재 코드의 `p.size_usdt` 직접 참조를 `p.size_usdt()` 메서드 또는 `p.qty`로 변경해야 함.

#### 4.4 청산금(Liquidation) 경로

`check_liquidation()` (PositionManager 메서드)은 가격 비교만 수행하며 직접 변경하지 않는다.
실제 청산 실행은 `monitor.rs`의 `finalize_and_process`에서 수행하므로, **이 경로에 InstrumentCache를 전달**하여 가격 라운딩을 적용한다.
InstrumentInfo 미존재 시: 라운딩 없이 청산 진행 + fallback_no_rounding_count++.

### 5. PnL 계산 qty 기반 전환

#### 5.1 현재 방식 (USDT notional matching)

```rust
// build_closed_position() 현재 코드
let upbit_qty = close_size_usdt / pos.upbit_entry_price;
let bybit_qty = close_size_usdt / pos.bybit_entry_price;
let upbit_pnl = (exit_upbit_price - pos.upbit_entry_price) * upbit_qty;
let bybit_pnl = (pos.bybit_entry_price - exit_bybit_price) * bybit_qty;
```

양 leg의 USDT 금액은 같지만, 코인 수량이 서로 다르다. 이는 방향성 노출(directional exposure)을 발생시켜 순수 차익거래가 아닌 투기 성격이 된다.

#### 5.2 변경 방식 (coin qty matching)

```rust
// 변경 후
let qty = close_qty;  // 양 leg 동일 수량
let upbit_pnl = (exit_upbit_price - pos.upbit_entry_price) * qty;
let bybit_pnl = (pos.bybit_entry_price - exit_bybit_price) * qty;
let upbit_fees = (pos.upbit_entry_price * qty + exit_upbit_price * qty) * upbit_taker_fee;
let bybit_fees = (pos.bybit_entry_price * qty + exit_bybit_price * qty) * bybit_taker_fee;
```

- **수량**: `pos.qty`를 사용하여 양 leg 동일
- **수수료**: 진입/청산 각각의 가격에 수수료를 개별 적용 (기존 `size_usdt * fee * 2` 근사 대체). 양 leg 모두 USDT 기준으로 계산 (Upbit 수수료는 실제로는 KRW 기준이나 시뮬레이션에서는 USD 환산 가격으로 근사)
- **close_partial**: USDT 금액이 아닌 `partial_qty` (코인 수량) 기반으로 분할. `close_size_usdt` 파라미터를 `close_qty`로 변경

#### 5.3 close_partial 인터페이스 변경

```rust
// 현재
pub fn close_partial(&mut self, coin: &str, id: u64, partial_size_usdt: Decimal, ...) -> ...

// 변경
pub fn close_partial(
    &mut self,
    coin: &str,
    id: u64,
    partial_qty: Decimal,
    instrument_info: Option<&InstrumentInfo>,  // ★ 신규: min_order_qty 판단용
    ...
) -> ...
```

PositionManager 내부의 pseudo-code:
```rust
fn close_partial(&mut self, coin, id, partial_qty, instrument_info, ...) {
    let close_qty = if let Some(info) = instrument_info {
        let rounded = floor_to_step(partial_qty, info.qty_step);
        if rounded.is_zero() { return Ok(None); }  // 청산 스킵
        let remaining = pos.qty.saturating_sub(rounded);  // 음수 방어
        if remaining < info.min_order_qty && remaining > Decimal::ZERO {
            pos.qty  // 전량 청산으로 전환
        } else {
            rounded
        }
    } else {
        partial_qty  // fallback: 라운딩 없이 원본 qty
    };

    let closed = build_closed_position(pos, close_qty, ...);
    pos.qty = pos.qty.saturating_sub(close_qty);  // 잔여 qty 업데이트
    if pos.qty.is_zero() { positions.remove(idx); }
    Ok(Some(closed))
}
```

이를 통해 호출부(monitor.rs)는 단순히 partial_qty와 instrument_info를 전달하기만 하면 된다.

#### 5.4 close_position 인터페이스 변경

```rust
// 현재
pub fn close_position(&mut self, coin: &str, id: u64, ...) -> ...
// → size_usdt 기반으로 build_closed_position 호출

// 변경
pub fn close_position(&mut self, coin: &str, id: u64, ...) -> ...
// → pos.qty 전체를 close_qty로 사용하여 build_closed_position 호출
```

전량 청산이므로 라운딩 없이 `pos.qty`를 그대로 사용한다 — 진입 시 이미 qty_step으로 라운딩된 값이 저장되어 있으므로 재라운딩이 불필요하다. `build_closed_position()`의 `close_size_usdt` 파라미터를 `close_qty: Decimal`로 변경하여, close_position과 close_partial 모두 qty 기반으로 통일한다.

### 6. 데이터 구조 변경

#### 6.1 InstrumentInfo (신규)

```rust
/// 코인별 거래 규격 정보.
///
/// Bybit REST API에서 조회한 tick_size, qty_step 등을 보관합니다.
/// Upbit은 하드코딩 테이블이므로 이 구조체에 포함하지 않습니다.
pub struct InstrumentInfo {
    /// 가격 최소 단위 (Bybit priceFilter.tickSize).
    pub tick_size: Decimal,
    /// 수량 최소 단위 (Bybit lotSizeFilter.qtyStep).
    pub qty_step: Decimal,
    /// 최소 주문 수량 (Bybit lotSizeFilter.minOrderQty).
    pub min_order_qty: Decimal,
    /// 최소 주문 금액 USDT (Bybit lotSizeFilter.minNotionalValue).
    pub min_notional: Decimal,
    /// 최대 주문 수량 (Bybit lotSizeFilter.maxOrderQty).
    pub max_order_qty: Decimal,
}
```

필드명을 거래소 중립적으로 명명 (`bybit_` prefix 제거).

#### 6.2 InstrumentCache (신규)

```rust
/// 코인별 InstrumentInfo 캐시.
///
/// `Arc<std::sync::RwLock<InstrumentCache>>`로 래핑하여 공유합니다.
/// 쓰기는 드물고(시작 시 + 재선택 시), 읽기는 매 틱마다 발생합니다.
/// lock 내부에서 .await가 불필요하므로 std::sync::RwLock을 사용합니다.
pub struct InstrumentCache {
    cache: HashMap<String, InstrumentInfo>,
}

impl InstrumentCache {
    /// 코인의 instrument info 조회. 없으면 None (진입 거부).
    pub fn get(&self, coin: &str) -> Option<&InstrumentInfo>;

    /// 코인의 instrument info를 직접 삽입.
    pub fn insert(&mut self, coin: String, info: InstrumentInfo);
}

/// InstrumentCache를 갱신하는 free function.
///
/// std::sync::RwLock write guard에서 .await를 호출할 수 없으므로,
/// lock 밖에서 REST 호출 후 결과만 lock 안에서 삽입하는 패턴을 사용한다.
///
/// ```rust
/// // 1단계: lock 밖에서 REST 호출
/// let mut infos = Vec::new();
/// for coin in coins {
///     let resp = bybit.get_instrument_info(&symbol).await?;
///     infos.push((coin, InstrumentInfo::from(resp)));
/// }
/// // 2단계: lock 안에서 데이터 삽입 (순간적, non-async)
/// // poisoned lock 복구: 이전 writer가 패닉했더라도 캐시 데이터는 유효하므로 복구한다
/// let mut cache = instrument_cache.write().unwrap_or_else(|e| e.into_inner());
/// for (coin, info) in infos {
///     cache.insert(coin, info);
/// }
/// ```
pub async fn fetch_instruments(
    bybit: &impl InstrumentDataProvider,
    cache: &Arc<std::sync::RwLock<InstrumentCache>>,
    coins: &[String],
) -> Result<(), ...>;
```

#### 6.3 VirtualPosition 변경

```rust
pub struct VirtualPosition {
    // ... 기존 필드 ...
    // size_usdt 필드 제거 → derived 메서드로 전환
    /// ★ 신규: 실제 코인 수량 (qty_step 라운딩 적용, 양 leg 동일).
    pub qty: Decimal,
}

impl VirtualPosition {
    /// USDT 기준 포지션 크기 (qty × bybit_entry_price).
    /// 기존 size_usdt 필드를 대체하는 derived 메서드.
    /// ⚠ Bybit leg 기준이므로 Upbit leg notional과 스프레드만큼 차이남.
    pub fn size_usdt(&self) -> Decimal {
        self.qty * self.bybit_entry_price
    }
}
```

**가격 필드 의미**:
- `upbit_entry_price`: **라운딩 후** KRW→USD 환산 가격 (`ceil_to_tick(krw) / usd_krw`). 보수적 시뮬레이션을 위해 불리한 방향으로 라운딩된 가격이 저장됨.
- `bybit_entry_price`: **라운딩 후** Bybit short 가격 (`floor_to_tick(price, tick_size)`).

`used_capital()`, `coin_used_capital()` 등은 `size_usdt()` (Bybit leg 기준)을 사용한다.

`size_usdt` 필드를 제거하고 `size_usdt()` 메서드로 전환하여 qty와의 불일치를 원천 차단한다.
`used_capital()`, `coin_used_capital()` 등 기존 호출부는 `.size_usdt` → `.size_usdt()`로 변경한다.

기존 테스트의 struct literal에서 `size_usdt` 필드를 제거하고 `qty: Decimal` 필드를 추가한다.
**PnL 계산을 검증하는 테스트**에서는 `qty`에 실제 값을 설정해야 한다 (qty 기반 PnL이므로 `ZERO`면 PnL=0).

#### 6.4 ClosedPosition 변경

```rust
pub struct ClosedPosition {
    // ... 기존 필드 ...
    /// ★ 신규: 청산 수량 (양 leg 동일).
    pub qty: Decimal,
    /// ★ 유지: USDT 기준 포지션 크기 (qty × bybit_entry_price).
    /// build_closed_position에서 `close_qty * bybit_entry_price`로 계산하여 설정.
    /// 기존 출력/분석 코드와의 호환성을 위해 필드로 유지.
    pub size_usdt: Decimal,
}
```

trades.csv/JSON 출력에도 `qty` 필드를 포함. `size_usdt`는 기존대로 출력 유지.

### 7. SDK 변경 (arb-exchange, arb-exchanges)

#### 7.1 별도 trait: InstrumentDataProvider (신규)

`MarketData` trait을 오염시키지 않고, Bybit만 구현하는 별도 trait을 생성한다.
Bithumb은 구현 불필요.

`ZScoreMonitor<A, B>`의 `B`(Bybit) 제네릭 파라미터에 `InstrumentDataProvider` bound를 추가한다:
```rust
impl<A: MarketData + ..., B: MarketData + InstrumentDataProvider + ...> ZScoreMonitor<A, B>
```

```rust
// arb-exchange/src/traits.rs
/// 거래 규격(instrument info) 조회 trait.
///
/// Bybit 등 instrument info API를 제공하는 거래소만 구현합니다.
pub trait InstrumentDataProvider: Send + Sync {
    /// 심볼의 거래 규격을 조회합니다.
    fn get_instrument_info(&self, symbol: &str)
        -> impl Future<Output = Result<InstrumentInfoResponse, ExchangeError>> + Send;
}
```

```rust
// arb-exchange/src/types.rs
/// 거래 규격 API 응답 (거래소 중립).
///
/// API 응답을 파싱할 때 Decimal로 즉시 변환합니다.
/// 사용 시점마다 재파싱이 불필요하며, 파싱 실패를 초기에 잡을 수 있습니다.
pub struct InstrumentInfoResponse {
    pub tick_size: Decimal,
    pub qty_step: Decimal,
    pub min_order_qty: Decimal,
    pub max_order_qty: Decimal,
    pub min_notional: Decimal,
}
```

#### 7.2 Bybit 구현

`/v5/market/instruments-info?category=linear&symbol={symbol}` 호출하여 `priceFilter.tickSize`, `lotSizeFilter.qtyStep`, `lotSizeFilter.minOrderQty`, `lotSizeFilter.minNotionalValue`, `lotSizeFilter.maxOrderQty` 파싱.

**주의**: `BybitClient::DEFAULT_CATEGORY`는 `"spot"`이므로, `InstrumentDataProvider` 구현에서는 **`category="linear"`을 하드코딩**해야 한다 (선물 계약의 instrument info 조회). `get_public()` 메서드로 REST 호출하되, category 파라미터를 spot이 아닌 linear로 직접 지정한다.

#### 7.3 Upbit

Upbit에는 instrument info API가 없으므로 `InstrumentDataProvider`를 구현하지 않음.
Upbit 호가 단위는 `arb-strategy/instrument.rs`의 하드코딩 테이블로 처리.

### 8. 라운딩 유틸리티 함수

`crates/arb-strategy/src/zscore/instrument.rs`에 순수 함수로 구현:

```rust
/// step 단위로 내림 (floor).
/// step == 0이면 원본 반환 + warn 로그.
/// value가 이미 step의 정확한 배수이면 그대로 반환.
/// 예: floor_to_step(123.456, 0.01) → 123.45
/// 예: floor_to_step(0.3, 0.1) → 0.3 (정확한 배수)
pub fn floor_to_step(value: Decimal, step: Decimal) -> Decimal;

/// step 단위로 올림 (ceil).
/// step == 0이면 원본 반환 + warn 로그.
/// value가 이미 step의 정확한 배수이면 그대로 반환.
/// 예: ceil_to_step(123.451, 0.01) → 123.46
pub fn ceil_to_step(value: Decimal, step: Decimal) -> Decimal;

/// Upbit KRW 가격에 대한 호가 단위를 반환.
/// 경계값: 이상(≥) ~ 미만(<) 규칙.
/// 예: upbit_tick_size(10000) → 10, upbit_tick_size(9999) → 5
pub fn upbit_tick_size(krw_price: Decimal) -> Decimal;

/// 보수적 가격 라운딩 (불리한 방향).
/// is_buy=true → ceil, is_buy=false → floor
pub fn round_price_conservative(price: Decimal, tick_size: Decimal, is_buy: bool) -> Decimal;

/// 수량을 qty_step으로 floor 라운딩.
/// `floor_to_step`의 semantic alias — 의미 명확화를 위해 진입/청산 수량 라운딩 시 사용.
/// 내부 구현은 `floor_to_step(qty, qty_step)` 호출.
pub fn round_qty_floor(qty: Decimal, qty_step: Decimal) -> Decimal;
```

### 9. 모니터링 카운터 변경

`MonitoringCounters`에 다음 카운터 추가:

| 카운터 | 설명 |
|--------|------|
| `entry_rejected_order_constraint_count` | 최소/최대 주문 조건 미달로 진입 거부된 횟수 (qty, notional, KRW 최소, max_order_qty 모두 포함) |
| `entry_rejected_rounding_pnl_count` | 라운딩 후 수익성 부족(post-rounding PnL gate)으로 진입 거부된 횟수 |
| `fallback_no_rounding_count` | InstrumentInfo 미존재로 라운딩 없이 청산 진행한 횟수 |
| `safe_volume_exceeded_close_count` | 잔여 qty < min_order_qty로 전량 전환 시 safe volume 초과한 청산 횟수 |

기존 `entry_rejected_slippage_count`에서 최소 주문 미달 분을 분리:
- `entry_rejected_slippage_count`: 오더북 안전 볼륨 없음 / safe volume 계산 실패
- `entry_rejected_order_constraint_count`: qty/notional/KRW 최소 조건 미달 + max_order_qty 초과
- `entry_rejected_rounding_pnl_count`: post-rounding PnL gate 미달

## 구현 계획

### Phase 1: 라운딩 유틸리티 + Upbit 호가 테이블 (병렬 가능)

- `crates/arb-strategy/src/zscore/instrument.rs` 신규 생성
- `floor_to_step`, `ceil_to_step`, `upbit_tick_size`, `round_price_conservative`, `round_qty_floor` 구현
- 테스트:
  - 각 함수별 경계값 (step=0, value<step, value가 step의 정확한 배수)
  - Upbit 호가 테이블 17단계 전 구간 경계값 (정확히 경계 가격에서의 tick, 예: 999,999 KRW → tick=500)
  - 큰 값 (BTC 1억원대), 매우 작은 step (0.00000001)

### Phase 2: Bybit SDK + InstrumentCache (Phase 1과 병렬 가능)

- `arb-exchange/src/traits.rs`에 `InstrumentDataProvider` trait 추가
- `arb-exchange/src/types.rs`에 `InstrumentInfoResponse` 타입 추가
- `arb-exchanges/src/bybit/client.rs`에 `InstrumentDataProvider` 구현
- `arb-strategy/src/zscore/instrument.rs`에 `InstrumentInfo`, `InstrumentCache` 구현
- `impl From<InstrumentInfoResponse> for InstrumentInfo` 변환 구현
- `InstrumentCache`를 `Arc<std::sync::RwLock<...>>`로 래핑 (lock 내 `.await` 불필요)

### Phase 3: VirtualPosition qty 필드 + PnL qty 기반 전환

**이 Phase가 가장 복잡하며 regression 리스크가 높음. TDD 접근 권장: 테스트 먼저 작성 → 구현.**

- `VirtualPosition`에서 `size_usdt` 필드 제거 → `size_usdt()` derived 메서드로 전환
- `VirtualPosition`에 `qty: Decimal` 필드 추가
- `used_capital()`, `coin_used_capital()` 등 기존 `.size_usdt` 접근을 `.size_usdt()` 호출로 변경
- `ClosedPosition`에 `qty: Decimal` 필드 추가
- `build_closed_position()`의 PnL 계산을 qty 기반으로 변경
  - `close_size_usdt` 파라미터를 `close_qty: Decimal`로 변경
  - `upbit_qty = bybit_qty = close_qty` (동일 수량)
  - 수수료: 진입/청산 가격 개별 적용
- `close_position()` 인터페이스: `pos.qty` 전체를 `close_qty`로 전달
- `close_partial()` 인터페이스를 `partial_qty` 기반으로 변경
  - `instrument_info: Option<&InstrumentInfo>` 파라미터 추가
  - 잔여 qty < min_order_qty이면 전량 청산 전환
- `output/writer.rs`: CSV/JSON에 qty 필드 포함
- 기존 테스트 struct literal: `size_usdt` 제거, `qty` 추가 (약 40~50곳: struct literal ~20곳 + `.size_usdt` 필드 접근 ~25곳)
- `signal.rs`에서 `size_usdt` 필드를 직접 참조하는 부분이 없는지 확인 (signal은 qty 도입 전 레이어이므로 참조가 없어야 함)

### Phase 4: 진입/청산 경로 적용

- `spawned_check_tick_signal`에서 qty 변환 + 라운딩 + 최소 주문 검증
- `InstrumentCache`를 모니터 시작 시 초기화, 코인 재선택 시 갱신
  - `spawned_check_tick_signal` 파라미터에 `InstrumentCache` (Arc) 추가
  - 재선택 완료 후 InstrumentCache 로드 전까지 해당 코인 진입 거부 (`cache.get(coin) == None`이 자연 거부, 별도 flag 불필요)
- 기존 `size_usdt <= 5.0` 하드코딩을 InstrumentInfo 기반 검증으로 교체
- 청산 경로: `close_position`, `close_partial` 가격 라운딩 적용
- TTL 청산 / Liquidation 청산 경로에도 동일 적용 (InstrumentCache 파라미터 추가)
- `entry_rejected_order_constraint_count` + `entry_rejected_rounding_pnl_count` 카운터 분리

### Phase 5: 통합 테스트 + 라이브 검증

- 기존 166개 테스트 호환성 확인
- 신규 테스트 추가 (instrument, qty 기반 PnL)
- 라이브 실행으로 라운딩 적용 확인

## 파일 변경 목록

| 파일 | 변경 유형 | 설명 |
|------|-----------|------|
| `crates/arb-strategy/src/zscore/instrument.rs` | **신규** | 라운딩 유틸리티, Upbit 호가 테이블, InstrumentInfo, InstrumentCache |
| `crates/arb-strategy/src/zscore/mod.rs` | **수정** | `pub mod instrument;` 추가 |
| `crates/arb-strategy/src/zscore/position.rs` | **수정** | VirtualPosition `size_usdt` 필드→derived 메서드 전환, `qty` 필드 추가, `build_closed_position`/`close_position`/`close_partial` qty 기반 전환 |
| `crates/arb-strategy/src/zscore/monitor.rs` | **수정** | InstrumentCache 초기화, 진입/청산 라운딩 적용, 카운터 분리, TTL/Liquidation 경로의 `p.size_usdt` 직접 접근을 `p.size_usdt()` 메서드 또는 `p.qty`로 전환 |
| `crates/arb-strategy/src/output/writer.rs` | **수정** | CSV/JSON에 qty 필드 포함 (CSV: size_usdt 컬럼 바로 뒤에 qty 컬럼 추가) |
| `crates/arb-exchange/src/traits.rs` | **수정** | `InstrumentDataProvider` trait 추가 (별도 trait, MarketData 변경 없음) |
| `crates/arb-exchange/src/types.rs` | **수정** | `InstrumentInfoResponse` 타입 추가 (필드: `Decimal`) |
| `crates/arb-exchange/src/lib.rs` | **수정** | `InstrumentDataProvider` re-export 추가 |
| `crates/arb-exchanges/src/bybit/client.rs` | **수정** | `InstrumentDataProvider` 구현 (`category="linear"` 하드코딩) |

### 변경하지 않는 파일

| 파일 | 이유 |
|------|------|
| `config.rs` | 신규 설정 불필요 (instrument info는 API에서 동적 조회) |
| `signal.rs` | 통계/수수료 레이어, 가격 라운딩과 무관 |
| `orderbook.rs` | 오더북 데이터 자체는 변경 없음 |
| `spread.rs` | 스프레드/Z-Score 계산은 원시 가격 기반 유지 (라운딩 전 시장 가격이 통계적으로 정확) |
| `upbit/client.rs` | Upbit은 InstrumentDataProvider 미구현 (하드코딩 테이블) |
| `bithumb/client.rs` | Bithumb은 InstrumentDataProvider 미구현 (별도 trait이므로 영향 없음) |

### 테스트 주의사항

- **MockMarket**: 기존 `MockMarket` (테스트 유틸리티)에 `InstrumentDataProvider` trait 구현을 추가해야 한다. 고정된 `InstrumentInfoResponse`를 반환하는 stub 구현.
- **라운딩 유틸리티 테스트**: `floor_to_step`/`ceil_to_step`에서 value가 step의 정확한 배수인 경우 (`0.3 / 0.1`, `1.0 / 0.5` 등) 자기 자신을 반환하는지 검증. `rust_decimal`은 10진수 기반이므로 이진 부동소수점 문제는 없으나, 배수 판정 테스트를 Phase 1에 명시적으로 포함.
- **PnL 테스트**: qty 기반 PnL 전환 후 기존 테스트의 기대값이 변경될 수 있다. `make_closed()` 등 헬퍼에 qty 파라미터를 추가하고, PnL assertion 값을 qty 기반으로 재계산한다.
- **파라미터 수**: `spawned_check_tick_signal` (16 → 17+), `check_ttl_positions`, `check_liquidation` 등에 `InstrumentCache` 파라미터가 추가되어 인자 수가 증가한다. `#[allow(clippy::too_many_arguments)]` 적용. 향후 Context 구조체 도입을 고려할 수 있으나 이번 스펙 범위 밖.

## 검증

1. `cargo test -p arb-strategy` — 기존 + 신규 테스트 통과
2. `cargo clippy --workspace` — 경고 0
3. 라이브 실행 60초+ 후 확인:
   - 라운딩 적용 로그 출력 (debug 레벨)
   - `entry_rejected_order_constraint_count` 카운터 동작 확인
   - 진입 수량이 qty_step 배수인지 확인
   - ClosedPosition.qty가 trades.csv에 출력되는지 확인
   - PnL이 이전 세션 대비 소폭 보수적인지 확인 (라운딩 + qty matching 효과)

## 알려진 한계

- **실거래 시 부분 체결**: 시뮬레이션에서는 주문 수량 = 체결 수량이지만, 실거래에서는 양쪽 거래소의 체결 수량이 다를 수 있다. 이 경우 잔여 수량 처리는 실거래 스펙에서 별도로 다룬다.
- **Upbit 호가 테이블 변경**: 변경 시 코드 수정 필요. 변경 빈도가 극히 낮아(연 1회 미만) 하드코딩이 합리적.
- **환율 변환 정밀도**: `usd_krw`가 f64이므로 KRW→USD 변환 시 미세한 오차 발생. 차익거래 PnL 대비 무시할 수준(1e-10% 이하).
- **라운딩 비용**: 양 leg 합산 trade당 약 0.03~0.15% PnL 감소 추정. 이는 의도된 보수적 시뮬레이션.
- **환율 변동 리스크**: Upbit KRW 최소 주문 검증 시 `usd_krw` 환율을 사용하여 USDT→KRW 환산하는데, 환율이 세션 중 변동하면 검증 시점과 실제 주문 시점의 KRW 금액이 다를 수 있다. 5,100 KRW 안전 마진으로 일부 완화하지만, 급격한 환율 변동(>2%) 시에는 여전히 괴리 가능.
- **max_order_qty 초과 시 분할 미지원**: 현재 스펙에서는 `qty > max_order_qty`면 단순 진입 거부한다. 실거래 전환 시에는 대형 주문을 여러 건으로 분할하는 로직이 필요할 수 있으며, 이는 라이브 트레이딩 스펙에서 별도로 다룬다.
- **양 leg notional 차이**: `size_usdt()` = `qty × bybit_entry_price`이지만, Upbit leg의 실제 투입 자본은 `qty × upbit_entry_usd`이다. 스프레드(0.1~0.5%)만큼 양 leg notional이 차이나며, `used_capital()` 계산은 Bybit 기준이다. 자본 관리 관점에서 소폭 부정확하나, 스프레드 수준의 오차이므로 시뮬레이션에서는 무시할 수 있다.
- **InstrumentDataProvider object safety**: RPITIT(`-> impl Future`)를 사용하므로 `dyn InstrumentDataProvider`로 사용할 수 없다. 현재 코드에서는 제네릭 바운드(`B: InstrumentDataProvider`)로 사용하므로 문제없으나, 향후 동적 디스패치가 필요해지면 `async-trait` 또는 별도 어댑터 패턴이 필요하다.
- **close_partial exit_time 비대칭**: `close_partial()`의 `ClosedPosition.exit_time`은 호출부에서 전달받지만, 현재 `build_closed_position`이 `pos.entry_time`과 독립적으로 설정된다. 분할 청산 시 exit_time이 마지막 분할의 시간이 되며, 이전 분할과 동일 포지션에 대해 서로 다른 exit_time을 가질 수 있다. trades.csv에서 각 분할이 독립 행으로 기록되므로 분석 시 주의 필요.

## 참고

- [Upbit KRW 마켓 호가 단위](https://docs.upbit.com/kr/docs/krw-market-info)
- [Bybit Instruments Info API](https://bybit-exchange.github.io/docs/v5/market/instrument)
- [Bybit Trading Fee Structure](https://www.bybit.com/en/help-center/article/Trading-Fee-Structure/)
