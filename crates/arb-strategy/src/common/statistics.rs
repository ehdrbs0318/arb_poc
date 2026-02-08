//! 통계 유틸리티.
//!
//! Z-Score 계산을 위한 평균, 표준편차, z-score 함수를 제공합니다.
//! 모든 연산은 f64 도메인에서 수행됩니다.

use crate::error::StatisticsError;
use std::collections::VecDeque;

/// 데이터의 산술 평균을 계산합니다.
///
/// 빈 데이터에 대해서는 0.0을 반환합니다.
pub fn mean(data: &VecDeque<f64>) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

/// 모집단 표준편차를 계산합니다 (N으로 나눔).
///
/// N=1440에서 N과 N-1의 차이는 0.07%로 무시 가능하며,
/// 일관성을 위해 모집단 표준편차를 사용합니다.
pub fn stddev(data: &VecDeque<f64>, mean_val: f64) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let variance: f64 =
        data.iter().map(|x| (x - mean_val).powi(2)).sum::<f64>() / data.len() as f64;
    variance.sqrt()
}

/// Z-Score를 계산합니다.
///
/// stddev가 min_stddev 미만이면 `StatisticsError::BelowThreshold` 반환.
/// stddev가 극도로 작으면 Z-Score가 과도하게 증폭되어 의미 없는 진입 신호를 방지합니다.
pub fn z_score(
    current: f64,
    mean_val: f64,
    stddev_val: f64,
    min_stddev: f64,
) -> Result<f64, StatisticsError> {
    if stddev_val < min_stddev {
        return Err(StatisticsError::BelowThreshold {
            value: stddev_val,
            threshold: min_stddev,
        });
    }
    Ok((current - mean_val) / stddev_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_basic() {
        let data: VecDeque<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0].into();
        assert!((mean(&data) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_mean_empty() {
        let data: VecDeque<f64> = VecDeque::new();
        assert_eq!(mean(&data), 0.0);
    }

    #[test]
    fn test_mean_single_element() {
        let data: VecDeque<f64> = vec![42.0].into();
        assert!((mean(&data) - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_stddev_basic() {
        let data: VecDeque<f64> = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0].into();
        let m = mean(&data);
        let s = stddev(&data, m);
        assert!((s - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_stddev_empty() {
        let data: VecDeque<f64> = VecDeque::new();
        assert_eq!(stddev(&data, 0.0), 0.0);
    }

    #[test]
    fn test_stddev_uniform_data() {
        let data: VecDeque<f64> = vec![5.0, 5.0, 5.0, 5.0].into();
        let m = mean(&data);
        let s = stddev(&data, m);
        assert!((s - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_z_score_basic() {
        let result = z_score(5.0, 3.0, 2.0, 0.01).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_z_score_negative() {
        let result = z_score(1.0, 3.0, 2.0, 0.01).unwrap();
        assert!((result - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_z_score_zero() {
        let result = z_score(3.0, 3.0, 2.0, 0.01).unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_z_score_below_threshold() {
        let result = z_score(5.0, 3.0, 0.005, 0.01);
        assert!(result.is_err());
        match result {
            Err(StatisticsError::BelowThreshold { value, threshold }) => {
                assert!((value - 0.005).abs() < 1e-10);
                assert!((threshold - 0.01).abs() < 1e-10);
            }
            _ => panic!("Expected BelowThreshold error"),
        }
    }

    #[test]
    fn test_z_score_at_threshold() {
        let result = z_score(5.0, 3.0, 0.01, 0.01).unwrap();
        assert!((result - 200.0).abs() < 1e-10);
    }
}
