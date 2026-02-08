//! 수수료 계산 모듈.
//!
//! 라운드트립 수수료 및 손익분기 스프레드 계산을 제공합니다.

use rust_decimal::Decimal;

/// 라운드트립 수수료율(%)을 계산합니다.
///
/// 진입 + 청산, 양 거래소 합산.
/// `roundtrip_fee_pct = (upbit_fee + bybit_fee) * 2 * 100`
pub fn roundtrip_fee_pct(upbit_taker_fee: Decimal, bybit_taker_fee: Decimal) -> Decimal {
    (upbit_taker_fee + bybit_taker_fee) * Decimal::from(2) * Decimal::from(100)
}

/// 손익분기 스프레드(%)를 계산합니다.
///
/// 스프레드가 이 값 이상이어야 수익이 발생합니다.
pub fn breakeven_spread_pct(upbit_taker_fee: Decimal, bybit_taker_fee: Decimal) -> Decimal {
    roundtrip_fee_pct(upbit_taker_fee, bybit_taker_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_fee_typical() {
        let upbit_fee = Decimal::new(5, 4); // 0.0005 = 0.05%
        let bybit_fee = Decimal::new(55, 5); // 0.00055 = 0.055%
        let fee = roundtrip_fee_pct(upbit_fee, bybit_fee);
        // (0.0005 + 0.00055) * 2 * 100 = 0.21
        assert_eq!(fee, Decimal::new(21, 2));
    }

    #[test]
    fn test_roundtrip_fee_zero() {
        let fee = roundtrip_fee_pct(Decimal::ZERO, Decimal::ZERO);
        assert_eq!(fee, Decimal::ZERO);
    }

    #[test]
    fn test_roundtrip_fee_symmetric() {
        let fee_a = Decimal::new(1, 3); // 0.001 = 0.1%
        let fee_b = Decimal::new(1, 3); // 0.001 = 0.1%
        let fee = roundtrip_fee_pct(fee_a, fee_b);
        // (0.001 + 0.001) * 2 * 100 = 0.4
        assert_eq!(fee, Decimal::new(4, 1));
    }

    #[test]
    fn test_breakeven_spread_equals_roundtrip() {
        let upbit_fee = Decimal::new(5, 4);
        let bybit_fee = Decimal::new(55, 5);
        assert_eq!(
            breakeven_spread_pct(upbit_fee, bybit_fee),
            roundtrip_fee_pct(upbit_fee, bybit_fee)
        );
    }
}
