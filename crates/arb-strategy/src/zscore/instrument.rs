//! 거래 규격 (instrument) 라운딩 유틸리티 및 캐시.
//!
//! 거래소별 호가 단위(tick size), 수량 단위(qty step)에 맞춰
//! 가격과 수량을 라운딩하는 함수를 제공합니다.
//! Upbit KRW 호가 단위 테이블과 Bybit instrument info 캐시를 포함합니다.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use rust_decimal::Decimal;
use tracing::{info, warn};

use arb_exchange::{InstrumentDataProvider, InstrumentInfoResponse};

// ---------------------------------------------------------------------------
// 라운딩 유틸리티 함수
// ---------------------------------------------------------------------------

/// step 단위로 내림 (floor).
///
/// step이 0이면 원본 반환 + warn 로그를 남깁니다.
/// value가 이미 step의 정확한 배수이면 그대로 반환합니다.
/// value가 음수이면 `Decimal::ZERO`를 반환합니다.
///
/// # 인자
///
/// * `value` - 라운딩할 값.
/// * `step` - 라운딩 단위 (최소 증분).
///
/// # 반환값
///
/// step 단위로 내림한 `Decimal`.
///
/// # 예제
///
/// ```
/// use rust_decimal::Decimal;
/// use arb_strategy::zscore::instrument::floor_to_step;
///
/// let result = floor_to_step(Decimal::new(123456, 3), Decimal::new(1, 2));
/// assert_eq!(result, Decimal::new(12345, 2)); // 123.456 -> 123.45
/// ```
pub fn floor_to_step(value: Decimal, step: Decimal) -> Decimal {
    if step.is_zero() {
        warn!("floor_to_step: step is zero, returning original value");
        return value;
    }
    if value < Decimal::ZERO {
        return Decimal::ZERO;
    }
    let remainder = value % step;
    if remainder.is_zero() {
        value
    } else {
        value - remainder
    }
}

/// step 단위로 올림 (ceil).
///
/// step이 0이면 원본 반환 + warn 로그를 남깁니다.
/// value가 이미 step의 정확한 배수이면 그대로 반환합니다.
/// value가 음수이면 `Decimal::ZERO`를 반환합니다.
///
/// # 인자
///
/// * `value` - 라운딩할 값.
/// * `step` - 라운딩 단위 (최소 증분).
///
/// # 반환값
///
/// step 단위로 올림한 `Decimal`.
///
/// # 예제
///
/// ```
/// use rust_decimal::Decimal;
/// use arb_strategy::zscore::instrument::ceil_to_step;
///
/// let result = ceil_to_step(Decimal::new(123451, 3), Decimal::new(1, 2));
/// assert_eq!(result, Decimal::new(12346, 2)); // 123.451 -> 123.46
/// ```
pub fn ceil_to_step(value: Decimal, step: Decimal) -> Decimal {
    if step.is_zero() {
        warn!("ceil_to_step: step is zero, returning original value");
        return value;
    }
    if value < Decimal::ZERO {
        return Decimal::ZERO;
    }
    let remainder = value % step;
    if remainder.is_zero() {
        value
    } else {
        value + (step - remainder)
    }
}

// ---------------------------------------------------------------------------
// Upbit KRW 호가 단위 테이블
// ---------------------------------------------------------------------------

/// Upbit KRW 가격에 대한 호가 단위를 반환합니다.
///
/// 경계값은 이상(>=) ~ 미만(<) 규칙을 따릅니다.
/// 15단계 테이블을 내림차순 선형 탐색합니다.
///
/// # 인자
///
/// * `krw_price` - KRW 기준 가격.
///
/// # 반환값
///
/// 해당 가격 구간의 호가 단위 `Decimal`.
///
/// # 예제
///
/// ```
/// use rust_decimal::Decimal;
/// use arb_strategy::zscore::instrument::upbit_tick_size;
///
/// assert_eq!(upbit_tick_size(Decimal::new(10_000, 0)), Decimal::new(10, 0));
/// assert_eq!(upbit_tick_size(Decimal::new(9_999, 0)), Decimal::new(5, 0));
/// ```
pub fn upbit_tick_size(krw_price: Decimal) -> Decimal {
    // (하한 경계, 호가 단위) — 내림차순. 첫 번째 match 반환.
    // >= 1,000,000 → 1,000 (원래 >= 2,000,000 구간과 병합)
    // >= 100 → 1 (원래 >= 1,000 구간과 병합, 동일 호가 단위)
    let table: [(Decimal, Decimal); 15] = [
        (Decimal::new(1_000_000, 0), Decimal::new(1_000, 0)),
        (Decimal::new(500_000, 0), Decimal::new(500, 0)),
        (Decimal::new(100_000, 0), Decimal::new(100, 0)),
        (Decimal::new(50_000, 0), Decimal::new(50, 0)),
        (Decimal::new(10_000, 0), Decimal::new(10, 0)),
        (Decimal::new(5_000, 0), Decimal::new(5, 0)),
        (Decimal::new(1_000, 0), Decimal::new(1, 0)),
        (Decimal::new(100, 0), Decimal::new(1, 0)),
        (Decimal::new(10, 0), Decimal::new(1, 1)), // 0.1
        (Decimal::new(1, 0), Decimal::new(1, 2)),  // 0.01
        (Decimal::new(1, 1), Decimal::new(1, 3)),  // 0.001
        (Decimal::new(1, 2), Decimal::new(1, 4)),  // 0.0001
        (Decimal::new(1, 3), Decimal::new(1, 5)),  // 0.00001
        (Decimal::new(1, 4), Decimal::new(1, 6)),  // 0.000001
        (Decimal::new(1, 5), Decimal::new(1, 7)),  // 0.0000001
    ];

    for &(threshold, tick) in &table {
        if krw_price >= threshold {
            return tick;
        }
    }

    // < 0.00001 → 최소 단위 0.00000001
    Decimal::new(1, 8)
}

/// 보수적 가격 라운딩 (불리한 방향).
///
/// 매수(is_buy=true)일 때 올림(ceil), 매도(is_buy=false)일 때 내림(floor).
/// 체결을 보장하면서 불리한 가격으로 라운딩하여 보수적으로 계산합니다.
///
/// # 인자
///
/// * `price` - 라운딩할 가격.
/// * `tick_size` - 호가 단위.
/// * `is_buy` - 매수 여부 (true=매수, false=매도).
///
/// # 반환값
///
/// 보수적으로 라운딩된 가격 `Decimal`.
pub fn round_price_conservative(price: Decimal, tick_size: Decimal, is_buy: bool) -> Decimal {
    if is_buy {
        ceil_to_step(price, tick_size)
    } else {
        floor_to_step(price, tick_size)
    }
}

/// 수량을 qty_step으로 floor 라운딩.
///
/// `floor_to_step`의 의미적 별칭(semantic alias)입니다.
/// 진입/청산 수량 라운딩 시 가독성을 위해 사용합니다.
///
/// # 인자
///
/// * `qty` - 라운딩할 수량.
/// * `qty_step` - 수량 최소 단위.
///
/// # 반환값
///
/// qty_step 단위로 내림한 수량 `Decimal`.
pub fn round_qty_floor(qty: Decimal, qty_step: Decimal) -> Decimal {
    floor_to_step(qty, qty_step)
}

// ---------------------------------------------------------------------------
// InstrumentInfo / InstrumentCache
// ---------------------------------------------------------------------------

/// 코인별 거래 규격 정보.
///
/// Bybit REST API에서 조회한 tick_size, qty_step 등을 보관합니다.
/// Upbit은 하드코딩 호가 테이블을 사용하므로 이 구조체에 포함하지 않습니다.
#[derive(Debug, Clone, Default)]
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

/// 코인별 InstrumentInfo 캐시.
///
/// `Arc<parking_lot::RwLock<InstrumentCache>>`로 래핑하여 공유합니다.
/// 쓰기는 드물고(시작 시 + 재선택 시), 읽기는 매 틱마다 발생하므로
/// lock 내부에서 `.await`가 불필요한 `parking_lot::RwLock`을 사용합니다.
#[derive(Debug)]
pub struct InstrumentCache {
    /// 코인 심볼 → InstrumentInfo 매핑.
    cache: HashMap<String, InstrumentInfo>,
}

impl InstrumentCache {
    /// 빈 InstrumentCache를 생성합니다.
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// 코인의 instrument info를 조회합니다.
    ///
    /// 캐시에 없으면 `None`을 반환합니다 (진입 거부 사유).
    pub fn get(&self, coin: &str) -> Option<&InstrumentInfo> {
        self.cache.get(coin)
    }

    /// 코인의 instrument info를 삽입합니다.
    ///
    /// 이미 존재하면 덮어씁니다.
    pub fn insert(&mut self, coin: String, info: InstrumentInfo) {
        self.cache.insert(coin, info);
    }
}

impl Default for InstrumentCache {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// InstrumentInfoResponse → InstrumentInfo 변환
// ---------------------------------------------------------------------------

impl From<InstrumentInfoResponse> for InstrumentInfo {
    fn from(resp: InstrumentInfoResponse) -> Self {
        Self {
            tick_size: resp.tick_size,
            qty_step: resp.qty_step,
            min_order_qty: resp.min_order_qty,
            min_notional: resp.min_notional,
            max_order_qty: resp.max_order_qty,
        }
    }
}

// ---------------------------------------------------------------------------
// InstrumentCache 갱신 함수
// ---------------------------------------------------------------------------

/// InstrumentCache를 갱신합니다.
///
/// lock 밖에서 REST 호출 후, lock 안에서 삽입하는 패턴입니다.
/// 실패한 코인은 경고 로그를 남기고 캐시에 추가하지 않습니다.
///
/// # 인자
///
/// * `bybit` - InstrumentDataProvider 구현체 (Bybit 클라이언트)
/// * `cache` - 공유 InstrumentCache
/// * `coins` - 조회할 코인 목록
pub async fn fetch_instruments(
    bybit: &(impl InstrumentDataProvider + ?Sized),
    cache: &Arc<RwLock<InstrumentCache>>,
    coins: &[String],
) {
    let mut infos = Vec::new();
    for coin in coins {
        let symbol = format!("{coin}USDT");
        match bybit.get_instrument_info(&symbol).await {
            Ok(resp) => {
                let inst = InstrumentInfo::from(resp);
                info!(
                    coin = coin.as_str(),
                    tick_size = %inst.tick_size,
                    qty_step = %inst.qty_step,
                    min_order_qty = %inst.min_order_qty,
                    min_notional = %inst.min_notional,
                    "instrument info 로드 완료"
                );
                infos.push((coin.clone(), inst));
            }
            Err(e) => {
                warn!(
                    coin = coin.as_str(),
                    error = %e,
                    "instrument info 로드 실패, 해당 코인 진입 불가"
                );
            }
        }
    }
    // lock 안에서 삽입 (non-async, 순간적)
    let mut cache_guard = cache.write();
    for (coin, inst) in infos {
        cache_guard.insert(coin, inst);
    }
}

// ---------------------------------------------------------------------------
// 테스트
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // =======================================================================
    // floor_to_step 테스트
    // =======================================================================

    #[test]
    fn test_floor_to_step_normal() {
        // 123.456을 0.01 단위로 내림 → 123.45
        let result = floor_to_step(Decimal::new(123456, 3), Decimal::new(1, 2));
        assert_eq!(result, Decimal::new(12345, 2));
    }

    #[test]
    fn test_floor_to_step_exact_multiple() {
        // 0.3은 0.1의 정확한 배수 → 0.3 그대로
        let result = floor_to_step(Decimal::new(3, 1), Decimal::new(1, 1));
        assert_eq!(result, Decimal::new(3, 1));
    }

    #[test]
    fn test_floor_to_step_zero_step() {
        // step=0이면 원본 반환
        let value = Decimal::new(42, 0);
        let result = floor_to_step(value, Decimal::ZERO);
        assert_eq!(result, value);
    }

    #[test]
    fn test_floor_to_step_negative_value() {
        // 음수 → ZERO
        let result = floor_to_step(Decimal::new(-5, 0), Decimal::new(1, 0));
        assert_eq!(result, Decimal::ZERO);
    }

    #[test]
    fn test_floor_to_step_large_value() {
        // 100,000,000을 1000 단위로 → 100,000,000 (정확한 배수)
        let result = floor_to_step(Decimal::new(100_000_000, 0), Decimal::new(1000, 0));
        assert_eq!(result, Decimal::new(100_000_000, 0));
    }

    #[test]
    fn test_floor_to_step_very_small_step() {
        // 0.123456789를 0.00000001 단위로 → 0.12345678
        let result = floor_to_step(Decimal::new(123456789, 9), Decimal::new(1, 8));
        assert_eq!(result, Decimal::new(12345678, 8));
    }

    #[test]
    fn test_floor_to_step_value_less_than_step() {
        // 0.005를 0.01 단위로 → 0.0
        let result = floor_to_step(Decimal::new(5, 3), Decimal::new(1, 2));
        assert_eq!(result, Decimal::ZERO);
    }

    #[test]
    fn test_floor_to_step_zero_value() {
        // 0을 임의 step으로 → 0
        let result = floor_to_step(Decimal::ZERO, Decimal::new(1, 2));
        assert_eq!(result, Decimal::ZERO);
    }

    #[test]
    fn test_floor_to_step_integer_step() {
        // 12345를 1000 단위로 → 12000
        let result = floor_to_step(Decimal::new(12345, 0), Decimal::new(1000, 0));
        assert_eq!(result, Decimal::new(12000, 0));
    }

    // =======================================================================
    // ceil_to_step 테스트
    // =======================================================================

    #[test]
    fn test_ceil_to_step_normal() {
        // 123.451을 0.01 단위로 올림 → 123.46
        let result = ceil_to_step(Decimal::new(123451, 3), Decimal::new(1, 2));
        assert_eq!(result, Decimal::new(12346, 2));
    }

    #[test]
    fn test_ceil_to_step_exact_multiple() {
        // 1.0은 0.5의 정확한 배수 → 1.0 그대로
        let result = ceil_to_step(Decimal::new(10, 1), Decimal::new(5, 1));
        assert_eq!(result, Decimal::new(10, 1));
    }

    #[test]
    fn test_ceil_to_step_zero_step() {
        // step=0이면 원본 반환
        let value = Decimal::new(42, 0);
        let result = ceil_to_step(value, Decimal::ZERO);
        assert_eq!(result, value);
    }

    #[test]
    fn test_ceil_to_step_negative_value() {
        // 음수 → ZERO
        let result = ceil_to_step(Decimal::new(-5, 0), Decimal::new(1, 0));
        assert_eq!(result, Decimal::ZERO);
    }

    #[test]
    fn test_ceil_to_step_small_remainder() {
        // 100.001을 0.01 단위로 올림 → 100.01
        let result = ceil_to_step(Decimal::new(100001, 3), Decimal::new(1, 2));
        assert_eq!(result, Decimal::new(10001, 2));
    }

    #[test]
    fn test_ceil_to_step_integer_step() {
        // 12001을 1000 단위로 올림 → 13000
        let result = ceil_to_step(Decimal::new(12001, 0), Decimal::new(1000, 0));
        assert_eq!(result, Decimal::new(13000, 0));
    }

    #[test]
    fn test_ceil_to_step_zero_value() {
        // 0을 임의 step으로 → 0
        let result = ceil_to_step(Decimal::ZERO, Decimal::new(1, 2));
        assert_eq!(result, Decimal::ZERO);
    }

    // =======================================================================
    // upbit_tick_size 테스트 (경계값 전수 확인)
    // =======================================================================

    #[test]
    fn test_upbit_tick_size_2m() {
        // >= 1,000,000 구간 (2M은 이 구간 내)
        assert_eq!(
            upbit_tick_size(Decimal::new(2_000_000, 0)),
            Decimal::new(1000, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_1_5m() {
        assert_eq!(
            upbit_tick_size(Decimal::new(1_500_000, 0)),
            Decimal::new(1000, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_1m_boundary() {
        // 경계: 정확히 1,000,000 → 1,000
        assert_eq!(
            upbit_tick_size(Decimal::new(1_000_000, 0)),
            Decimal::new(1000, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_999999() {
        // 경계 직전: 999,999 → 500
        assert_eq!(
            upbit_tick_size(Decimal::new(999_999, 0)),
            Decimal::new(500, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_500k_boundary() {
        assert_eq!(
            upbit_tick_size(Decimal::new(500_000, 0)),
            Decimal::new(500, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_499999() {
        assert_eq!(
            upbit_tick_size(Decimal::new(499_999, 0)),
            Decimal::new(100, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_100k_boundary() {
        assert_eq!(
            upbit_tick_size(Decimal::new(100_000, 0)),
            Decimal::new(100, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_99999() {
        assert_eq!(
            upbit_tick_size(Decimal::new(99_999, 0)),
            Decimal::new(50, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_50k_boundary() {
        assert_eq!(
            upbit_tick_size(Decimal::new(50_000, 0)),
            Decimal::new(50, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_49999() {
        assert_eq!(
            upbit_tick_size(Decimal::new(49_999, 0)),
            Decimal::new(10, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_10k_boundary() {
        assert_eq!(
            upbit_tick_size(Decimal::new(10_000, 0)),
            Decimal::new(10, 0)
        );
    }

    #[test]
    fn test_upbit_tick_size_9999() {
        assert_eq!(upbit_tick_size(Decimal::new(9_999, 0)), Decimal::new(5, 0));
    }

    #[test]
    fn test_upbit_tick_size_5k_boundary() {
        assert_eq!(upbit_tick_size(Decimal::new(5_000, 0)), Decimal::new(5, 0));
    }

    #[test]
    fn test_upbit_tick_size_4999() {
        assert_eq!(upbit_tick_size(Decimal::new(4_999, 0)), Decimal::new(1, 0));
    }

    #[test]
    fn test_upbit_tick_size_1000_boundary() {
        assert_eq!(upbit_tick_size(Decimal::new(1_000, 0)), Decimal::new(1, 0));
    }

    #[test]
    fn test_upbit_tick_size_100_boundary() {
        // >= 100 → 호가 단위 1 (1000 구간과 병합)
        assert_eq!(upbit_tick_size(Decimal::new(100, 0)), Decimal::new(1, 0));
    }

    #[test]
    fn test_upbit_tick_size_99() {
        // < 100 → 0.1
        assert_eq!(upbit_tick_size(Decimal::new(99, 0)), Decimal::new(1, 1));
    }

    #[test]
    fn test_upbit_tick_size_10_boundary() {
        assert_eq!(upbit_tick_size(Decimal::new(10, 0)), Decimal::new(1, 1));
    }

    #[test]
    fn test_upbit_tick_size_9() {
        assert_eq!(upbit_tick_size(Decimal::new(9, 0)), Decimal::new(1, 2));
    }

    #[test]
    fn test_upbit_tick_size_1_boundary() {
        assert_eq!(upbit_tick_size(Decimal::new(1, 0)), Decimal::new(1, 2));
    }

    #[test]
    fn test_upbit_tick_size_0_9() {
        // 0.9 → 0.001
        assert_eq!(upbit_tick_size(Decimal::new(9, 1)), Decimal::new(1, 3));
    }

    #[test]
    fn test_upbit_tick_size_0_1_boundary() {
        assert_eq!(upbit_tick_size(Decimal::new(1, 1)), Decimal::new(1, 3));
    }

    #[test]
    fn test_upbit_tick_size_0_09() {
        // 0.09 → 0.0001
        assert_eq!(upbit_tick_size(Decimal::new(9, 2)), Decimal::new(1, 4));
    }

    #[test]
    fn test_upbit_tick_size_0_009() {
        // 0.009 → 0.00001
        assert_eq!(upbit_tick_size(Decimal::new(9, 3)), Decimal::new(1, 5));
    }

    #[test]
    fn test_upbit_tick_size_0_0009() {
        // 0.0009 → 0.000001
        assert_eq!(upbit_tick_size(Decimal::new(9, 4)), Decimal::new(1, 6));
    }

    #[test]
    fn test_upbit_tick_size_0_00009() {
        // 0.00009 → 0.0000001
        assert_eq!(upbit_tick_size(Decimal::new(9, 5)), Decimal::new(1, 7));
    }

    #[test]
    fn test_upbit_tick_size_0_000009() {
        // 0.000009 → 0.00000001 (최소 단위)
        assert_eq!(upbit_tick_size(Decimal::new(9, 6)), Decimal::new(1, 8));
    }

    #[test]
    fn test_upbit_tick_size_very_small() {
        // 0.0000001 → 0.00000001
        assert_eq!(upbit_tick_size(Decimal::new(1, 7)), Decimal::new(1, 8));
    }

    // =======================================================================
    // round_price_conservative 테스트
    // =======================================================================

    #[test]
    fn test_round_price_conservative_buy_ceil() {
        // 매수 → 올림 (불리한 방향)
        let price = Decimal::new(50123, 1); // 5012.3
        let tick = Decimal::new(10, 0); // 10
        let result = round_price_conservative(price, tick, true);
        assert_eq!(result, Decimal::new(5020, 0));
    }

    #[test]
    fn test_round_price_conservative_sell_floor() {
        // 매도 → 내림 (불리한 방향)
        let price = Decimal::new(50123, 1); // 5012.3
        let tick = Decimal::new(10, 0); // 10
        let result = round_price_conservative(price, tick, false);
        assert_eq!(result, Decimal::new(5010, 0));
    }

    #[test]
    fn test_round_price_conservative_exact_multiple() {
        // 정확한 배수 → 변경 없음
        let price = Decimal::new(5010, 0);
        let tick = Decimal::new(10, 0);
        assert_eq!(round_price_conservative(price, tick, true), price);
        assert_eq!(round_price_conservative(price, tick, false), price);
    }

    // =======================================================================
    // round_qty_floor 테스트
    // =======================================================================

    #[test]
    fn test_round_qty_floor_basic() {
        // floor_to_step과 동일 결과 확인
        let qty = Decimal::new(12345, 4); // 1.2345
        let step = Decimal::new(1, 2); // 0.01
        assert_eq!(round_qty_floor(qty, step), floor_to_step(qty, step));
    }

    #[test]
    fn test_round_qty_floor_exact() {
        let qty = Decimal::new(10, 1); // 1.0
        let step = Decimal::new(1, 1); // 0.1
        assert_eq!(round_qty_floor(qty, step), Decimal::new(10, 1));
    }

    #[test]
    fn test_round_qty_floor_bybit_style() {
        // Bybit qtyStep=0.001 스타일
        let qty = Decimal::new(12345678, 7); // 1.2345678
        let step = Decimal::new(1, 3); // 0.001
        assert_eq!(round_qty_floor(qty, step), Decimal::new(1234, 3)); // 1.234
    }

    // =======================================================================
    // InstrumentCache 테스트
    // =======================================================================

    #[test]
    fn test_instrument_cache_insert_and_get() {
        let mut cache = InstrumentCache::new();
        let info = InstrumentInfo {
            tick_size: Decimal::new(1, 2),     // 0.01
            qty_step: Decimal::new(1, 3),      // 0.001
            min_order_qty: Decimal::new(1, 3), // 0.001
            min_notional: Decimal::new(5, 0),  // 5
            max_order_qty: Decimal::new(1000, 0),
        };

        cache.insert("BTC".to_string(), info.clone());
        let got = cache.get("BTC");
        assert!(got.is_some());
        let got = got.unwrap();
        assert_eq!(got.tick_size, Decimal::new(1, 2));
        assert_eq!(got.qty_step, Decimal::new(1, 3));
        assert_eq!(got.min_order_qty, Decimal::new(1, 3));
        assert_eq!(got.min_notional, Decimal::new(5, 0));
        assert_eq!(got.max_order_qty, Decimal::new(1000, 0));
    }

    #[test]
    fn test_instrument_cache_get_missing() {
        let cache = InstrumentCache::new();
        assert!(cache.get("ETH").is_none());
    }

    #[test]
    fn test_instrument_cache_overwrite() {
        let mut cache = InstrumentCache::new();
        let info1 = InstrumentInfo {
            tick_size: Decimal::new(1, 2),
            qty_step: Decimal::new(1, 3),
            min_order_qty: Decimal::new(1, 3),
            min_notional: Decimal::new(5, 0),
            max_order_qty: Decimal::new(1000, 0),
        };
        let info2 = InstrumentInfo {
            tick_size: Decimal::new(1, 1), // 변경: 0.1
            qty_step: Decimal::new(1, 2),  // 변경: 0.01
            min_order_qty: Decimal::new(1, 2),
            min_notional: Decimal::new(10, 0),
            max_order_qty: Decimal::new(500, 0),
        };

        cache.insert("BTC".to_string(), info1);
        cache.insert("BTC".to_string(), info2);

        let got = cache.get("BTC").unwrap();
        assert_eq!(got.tick_size, Decimal::new(1, 1));
        assert_eq!(got.qty_step, Decimal::new(1, 2));
    }

    #[test]
    fn test_instrument_cache_multiple_coins() {
        let mut cache = InstrumentCache::new();
        let btc_info = InstrumentInfo {
            tick_size: Decimal::new(1, 2),
            qty_step: Decimal::new(1, 5),
            min_order_qty: Decimal::new(1, 5),
            min_notional: Decimal::new(5, 0),
            max_order_qty: Decimal::new(100, 0),
        };
        let eth_info = InstrumentInfo {
            tick_size: Decimal::new(1, 1),
            qty_step: Decimal::new(1, 3),
            min_order_qty: Decimal::new(1, 3),
            min_notional: Decimal::new(1, 0),
            max_order_qty: Decimal::new(10000, 0),
        };

        cache.insert("BTC".to_string(), btc_info);
        cache.insert("ETH".to_string(), eth_info);

        assert!(cache.get("BTC").is_some());
        assert!(cache.get("ETH").is_some());
        assert!(cache.get("SOL").is_none());

        assert_eq!(cache.get("BTC").unwrap().qty_step, Decimal::new(1, 5));
        assert_eq!(cache.get("ETH").unwrap().qty_step, Decimal::new(1, 3));
    }

    #[test]
    fn test_instrument_cache_default() {
        // Default trait 구현 확인
        let cache = InstrumentCache::default();
        assert!(cache.get("ANY").is_none());
    }

    // =======================================================================
    // From<InstrumentInfoResponse> 테스트
    // =======================================================================

    #[test]
    fn test_from_instrument_info_response() {
        let resp = InstrumentInfoResponse {
            tick_size: Decimal::new(1, 2),     // 0.01
            qty_step: Decimal::new(1, 3),      // 0.001
            min_order_qty: Decimal::new(1, 3), // 0.001
            max_order_qty: Decimal::new(1000, 0),
            min_notional: Decimal::new(5, 0),
        };

        let info: InstrumentInfo = resp.into();
        assert_eq!(info.tick_size, Decimal::new(1, 2));
        assert_eq!(info.qty_step, Decimal::new(1, 3));
        assert_eq!(info.min_order_qty, Decimal::new(1, 3));
        assert_eq!(info.max_order_qty, Decimal::new(1000, 0));
        assert_eq!(info.min_notional, Decimal::new(5, 0));
    }

    #[test]
    fn test_from_instrument_info_response_preserves_precision() {
        // 정밀도 보존 확인
        let resp = InstrumentInfoResponse {
            tick_size: Decimal::new(5, 1),     // 0.5
            qty_step: Decimal::new(1, 5),      // 0.00001
            min_order_qty: Decimal::new(1, 5), // 0.00001
            max_order_qty: Decimal::new(50000, 0),
            min_notional: Decimal::new(1, 0),
        };

        let info = InstrumentInfo::from(resp);
        assert_eq!(info.tick_size, Decimal::new(5, 1));
        assert_eq!(info.qty_step, Decimal::new(1, 5));
    }
}
