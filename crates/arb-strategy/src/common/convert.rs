//! Decimal <-> f64 변환 유틸리티.
//!
//! 타입 설계 원칙에 따라 금융 데이터(Decimal)와 통계 데이터(f64) 간 변환을 집중 관리합니다.

use crate::error::{StatisticsError, StrategyError};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

/// Decimal을 f64로 변환합니다.
///
/// 변환 불가 시 `StrategyError::Statistics` 반환.
pub fn decimal_to_f64(d: Decimal) -> Result<f64, StrategyError> {
    d.to_f64().ok_or_else(|| {
        StrategyError::Statistics(StatisticsError::NanDetected(format!(
            "Decimal to f64 conversion failed: {d}"
        )))
    })
}

/// f64를 Decimal로 변환합니다.
///
/// NaN 또는 Infinity이면 `StrategyError::Statistics` 반환.
pub fn f64_to_decimal(f: f64) -> Result<Decimal, StrategyError> {
    if f.is_nan() || f.is_infinite() {
        return Err(StrategyError::Statistics(StatisticsError::NanDetected(
            format!("f64 to Decimal conversion failed: {f}"),
        )));
    }
    Decimal::try_from(f).map_err(|e| {
        StrategyError::Statistics(StatisticsError::NanDetected(format!(
            "f64 to Decimal conversion failed: {e}"
        )))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_f64_success() {
        let d = Decimal::new(12345, 2); // 123.45
        let f = decimal_to_f64(d).unwrap();
        assert!((f - 123.45).abs() < 1e-10);
    }

    #[test]
    fn test_decimal_to_f64_zero() {
        let d = Decimal::ZERO;
        let f = decimal_to_f64(d).unwrap();
        assert!((f - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_decimal_to_f64_negative() {
        let d = Decimal::new(-5050, 2); // -50.50
        let f = decimal_to_f64(d).unwrap();
        assert!((f - (-50.50)).abs() < 1e-10);
    }

    #[test]
    fn test_f64_to_decimal_success() {
        let d = f64_to_decimal(123.45).unwrap();
        assert_eq!(d, Decimal::try_from(123.45).unwrap());
    }

    #[test]
    fn test_f64_to_decimal_zero() {
        let d = f64_to_decimal(0.0).unwrap();
        assert_eq!(d, Decimal::ZERO);
    }

    #[test]
    fn test_f64_nan_to_decimal_error() {
        assert!(f64_to_decimal(f64::NAN).is_err());
    }

    #[test]
    fn test_f64_infinity_to_decimal_error() {
        assert!(f64_to_decimal(f64::INFINITY).is_err());
    }

    #[test]
    fn test_f64_neg_infinity_to_decimal_error() {
        assert!(f64_to_decimal(f64::NEG_INFINITY).is_err());
    }
}
