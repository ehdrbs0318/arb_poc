//! 시그널 생성 로직.
//!
//! Z-Score 기반 진입/청산 시그널을 독립적으로 평가합니다.
//! 진입과 청산을 별도 함수로 분리하여, 같은 틱에서
//! 일부 포지션 청산 + 신규 진입이 동시에 발생할 수 있습니다.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use tracing::{debug, info, trace};

use crate::common::fee::roundtrip_fee_pct;
use crate::common::statistics;
use crate::error::{StatisticsError, StrategyError};
use crate::zscore::config::ZScoreConfig;

/// 트레이딩 시그널.
#[derive(Debug, Clone)]
pub enum Signal {
    /// 진입: Upbit 현물 매수 + Bybit 선물 short.
    Enter {
        /// 코인 심볼.
        coin: String,
        /// 현재 Z-Score.
        z_score: f64,
        /// 현재 스프레드 (%).
        spread_pct: f64,
        /// 기대 수익률 (%, 수수료 차감 후, 단일 leg notional 기준).
        expected_profit_pct: f64,
    },
    /// 청산: 양쪽 포지션 종료.
    Exit {
        /// 코인 심볼.
        coin: String,
        /// 현재 Z-Score.
        z_score: f64,
        /// 현재 스프레드 (%).
        spread_pct: f64,
    },
}

/// 청산 시그널을 평가합니다.
///
/// 포지션이 있고 Z-Score가 exit_z_threshold 이하면 청산 시그널을 생성합니다.
///
/// # 인자
/// - `coin`: 코인 심볼
/// - `current_spread`: 현재 틱에서 계산한 스프레드 (%)
/// - `mean`: 분봉 기반 rolling mean (%)
/// - `stddev`: 분봉 기반 rolling stddev
/// - `has_positions`: 해당 코인에 포지션이 존재하는지 여부
/// - `config`: 전략 설정
///
/// # 반환값
/// - `Ok(Some(Signal::Exit))`: 청산 시그널
/// - `Ok(None)`: 시그널 없음
/// - `Err(...)`: Z-Score 계산 실패
pub fn evaluate_exit_signal(
    coin: &str,
    current_spread: f64,
    mean: f64,
    stddev: f64,
    has_positions: bool,
    config: &ZScoreConfig,
) -> Result<Option<Signal>, StrategyError> {
    // 포지션이 없으면 청산 불가
    if !has_positions {
        return Ok(None);
    }

    // Z-Score 계산 (min_stddev guard)
    let z = match statistics::z_score(current_spread, mean, stddev, config.min_stddev_threshold) {
        Ok(z) => {
            debug!(
                coin,
                current_spread,
                mean,
                stddev,
                z_score = z,
                "청산 시그널 평가: Z-Score 계산 결과"
            );
            z
        }
        Err(StatisticsError::BelowThreshold { .. }) => {
            trace!(
                coin,
                stddev,
                threshold = config.min_stddev_threshold,
                "stddev 임계값 미달: 청산 시그널 생략"
            );
            return Ok(None);
        }
        Err(e) => return Err(StrategyError::Statistics(e)),
    };

    // 청산 조건: z_score <= exit_z_threshold
    if z <= config.exit_z_threshold {
        debug!(
            coin,
            z_score = z,
            exit_z_threshold = config.exit_z_threshold,
            spread_pct = current_spread,
            "청산 시그널 생성"
        );
        return Ok(Some(Signal::Exit {
            coin: coin.to_string(),
            z_score: z,
            spread_pct: current_spread,
        }));
    }

    Ok(None)
}

/// 진입 시그널을 평가합니다.
///
/// Z-Score 임계값, 수수료 기반 수익성, 자본 한도, cooldown을 확인합니다.
/// size_usdt는 포함하지 않습니다 (monitor.rs에서 오더북 기반으로 결정).
///
/// # 인자
/// - `coin`: 코인 심볼
/// - `current_spread`: 현재 틱에서 계산한 스프레드 (%)
/// - `mean`: 분봉 기반 rolling mean (%)
/// - `stddev`: 분봉 기반 rolling stddev
/// - `coin_used_capital`: 해당 코인의 기존 포지션 사용 자본 합계
/// - `max_coin_capital`: 코인당 최대 허용 자본 (total_capital × max_position_ratio)
/// - `current_open_count`: 현재 열린 포지션 총 수
/// - `last_entry_at`: 해당 코인의 마지막 진입 시각 (cooldown 확인용)
/// - `config`: 전략 설정
///
/// # 반환값
/// - `Ok(Some(Signal::Enter))`: 진입 시그널
/// - `Ok(None)`: 시그널 없음
/// - `Err(...)`: Z-Score 계산 실패
#[allow(clippy::too_many_arguments)]
pub fn evaluate_entry_signal(
    coin: &str,
    current_spread: f64,
    mean: f64,
    stddev: f64,
    coin_used_capital: Decimal,
    max_coin_capital: Decimal,
    current_open_count: usize,
    last_entry_at: Option<DateTime<Utc>>,
    config: &ZScoreConfig,
) -> Result<Option<Signal>, StrategyError> {
    // Z-Score 계산 (min_stddev guard)
    let z = match statistics::z_score(current_spread, mean, stddev, config.min_stddev_threshold) {
        Ok(z) => {
            debug!(
                coin,
                current_spread,
                mean,
                stddev,
                z_score = z,
                "진입 시그널 평가: Z-Score 계산 결과"
            );
            z
        }
        Err(StatisticsError::BelowThreshold { .. }) => {
            trace!(
                coin,
                stddev,
                threshold = config.min_stddev_threshold,
                "stddev 임계값 미달: 진입 시그널 생략"
            );
            return Ok(None);
        }
        Err(e) => return Err(StrategyError::Statistics(e)),
    };

    // 1. Z-Score 임계값 확인
    if z < config.entry_z_threshold {
        return Ok(None);
    }

    // 2. 수수료 기반 수익성 확인
    let expected_spread_change = current_spread - mean;
    let fee_pct = roundtrip_fee_pct(config.upbit_taker_fee, config.bybit_taker_fee);
    let fee_f64 = fee_pct.to_f64().unwrap_or(0.0);
    let expected_profit = expected_spread_change - fee_f64;

    if expected_profit <= 0.0 {
        info!(
            coin,
            z_score = z,
            current_spread_pct = current_spread,
            mean_spread_pct = mean,
            expected_spread_change_pct = expected_spread_change,
            expected_profit,
            fee_pct = fee_f64,
            profit_gap_to_zero = expected_profit,
            filter = "expected_profit",
            "진입 거부: z-score 통과 후 기대 수익 부족"
        );
        return Ok(None);
    }

    // 3. 코인별 자본 한도 확인
    if coin_used_capital >= max_coin_capital {
        let remaining_coin_capital = max_coin_capital - coin_used_capital;
        info!(
            coin,
            z_score = z,
            coin_used_capital = %coin_used_capital,
            max_coin_capital = %max_coin_capital,
            remaining_coin_capital = %remaining_coin_capital,
            filter = "coin_capital_limit",
            "진입 거부: z-score 통과 후 코인 자본 한도 초과"
        );
        return Ok(None);
    }

    // 4. max_concurrent_positions 확인
    let max_positions = config
        .max_concurrent_positions
        .unwrap_or(config.coins.len().max(config.max_coins));
    if current_open_count >= max_positions {
        let remaining_slots = max_positions.saturating_sub(current_open_count);
        info!(
            coin,
            z_score = z,
            current_open_count,
            max_positions,
            remaining_slots,
            filter = "max_concurrent_positions",
            "진입 거부: z-score 통과 후 최대 동시 포지션 수 초과"
        );
        return Ok(None);
    }

    // 5. cooldown 확인
    if let Some(last_time) = last_entry_at {
        let elapsed = (Utc::now() - last_time).num_seconds();
        if elapsed < config.entry_cooldown_sec as i64 {
            let remaining_cooldown_sec = (config.entry_cooldown_sec as i64) - elapsed;
            info!(
                coin,
                z_score = z,
                elapsed,
                cooldown = config.entry_cooldown_sec,
                remaining_cooldown_sec,
                filter = "entry_cooldown",
                "진입 거부: z-score 통과 후 코인 쿨다운 적용 중"
            );
            return Ok(None);
        }
    }

    debug!(
        coin,
        z_score = z,
        expected_profit,
        coin_used_capital = %coin_used_capital,
        max_coin_capital = %max_coin_capital,
        current_open_count,
        max_positions,
        "진입 시그널 생성"
    );

    Ok(Some(Signal::Enter {
        coin: coin.to_string(),
        z_score: z,
        spread_pct: current_spread,
        expected_profit_pct: expected_profit,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zscore::config::ZScoreConfig;
    use chrono::Utc;
    use rust_decimal::Decimal;

    // --- 청산 시그널 테스트 ---

    /// 청산 시그널: 포지션 보유 + z <= exit_z
    #[test]
    fn test_exit_signal_basic() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            exit_z_threshold: 0.5,
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };

        // mean=0.1, stddev=0.2, spread=0.15
        // z = (0.15 - 0.1) / 0.2 = 0.25 <= 0.5
        let sig = evaluate_exit_signal("BTC", 0.15, 0.1, 0.2, true, &config).unwrap();
        match sig {
            Some(Signal::Exit { coin, z_score, .. }) => {
                assert_eq!(coin, "BTC");
                assert!(z_score <= 0.5);
            }
            _ => panic!("청산 시그널이 생성되어야 합니다"),
        }
    }

    /// 포지션이 없으면 청산 시그널 None
    #[test]
    fn test_exit_signal_no_position() {
        let config = ZScoreConfig {
            exit_z_threshold: 0.5,
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };

        // z <= exit_z이지만 포지션 없음
        let sig = evaluate_exit_signal("BTC", 0.15, 0.1, 0.2, false, &config).unwrap();
        assert!(sig.is_none());
    }

    // --- 진입 시그널 테스트 ---

    /// 진입 시그널: z >= entry_z, profit > 0, 자본 충분
    #[test]
    fn test_entry_signal_basic() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            exit_z_threshold: 0.5,
            min_stddev_threshold: 0.001,
            entry_cooldown_sec: 0,
            ..ZScoreConfig::default()
        };

        let max_coin_capital = config.total_capital_usdt * config.max_position_ratio;

        // mean=0.1, stddev=0.2, spread=0.6
        // z = (0.6 - 0.1) / 0.2 = 2.5 >= 2.0
        // expected_profit = (0.6 - 0.1) - fee(~0.21) = 0.29 > 0
        let sig = evaluate_entry_signal(
            "BTC",
            0.6,
            0.1,
            0.2,
            Decimal::ZERO,    // 코인 사용 자본 없음
            max_coin_capital, // 최대 코인 자본
            0,                // 현재 포지션 수
            None,             // 마지막 진입 없음
            &config,
        )
        .unwrap();

        match sig {
            Some(Signal::Enter { coin, z_score, .. }) => {
                assert_eq!(coin, "BTC");
                assert!(z_score >= 2.0);
            }
            _ => panic!("진입 시그널이 생성되어야 합니다"),
        }
    }

    /// 수수료 부족: expected_profit <= 0 -> None
    #[test]
    fn test_entry_signal_fee_exceeds_profit() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            min_stddev_threshold: 0.001,
            entry_cooldown_sec: 0,
            ..ZScoreConfig::default()
        };

        let max_coin_capital = config.total_capital_usdt * config.max_position_ratio;

        // mean=0.1, stddev=0.05, spread=0.2
        // z = (0.2 - 0.1) / 0.05 = 2.0 >= 2.0
        // expected_profit = (0.2 - 0.1) - fee(~0.21) = -0.11 < 0
        let sig = evaluate_entry_signal(
            "BTC",
            0.2,
            0.1,
            0.05,
            Decimal::ZERO,
            max_coin_capital,
            0,
            None,
            &config,
        )
        .unwrap();
        assert!(sig.is_none());
    }

    /// 코인 자본 한도 초과 -> None
    #[test]
    fn test_entry_signal_capital_exceeded() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            total_capital_usdt: Decimal::new(10000, 0),
            max_position_ratio: Decimal::new(2, 1), // 0.2 -> max_coin_capital = 2000
            min_stddev_threshold: 0.001,
            entry_cooldown_sec: 0,
            ..ZScoreConfig::default()
        };

        let max_coin_capital = config.total_capital_usdt * config.max_position_ratio;

        // 코인 사용 자본이 max_coin_capital 이상
        let sig = evaluate_entry_signal(
            "BTC",
            0.6,
            0.1,
            0.2,
            Decimal::new(2000, 0), // 이미 2000 사용 -> 한도 도달
            max_coin_capital,      // 2000
            0,
            None,
            &config,
        )
        .unwrap();
        assert!(sig.is_none());
    }

    /// 쿨다운 미경과 -> None
    #[test]
    fn test_entry_signal_cooldown() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            min_stddev_threshold: 0.001,
            entry_cooldown_sec: 60, // 60초 쿨다운
            ..ZScoreConfig::default()
        };

        let max_coin_capital = config.total_capital_usdt * config.max_position_ratio;

        // 마지막 진입이 방금 전 (쿨다운 미경과)
        let last_entry = Some(Utc::now());

        let sig = evaluate_entry_signal(
            "BTC",
            0.6,
            0.1,
            0.2,
            Decimal::ZERO,
            max_coin_capital,
            0,
            last_entry,
            &config,
        )
        .unwrap();
        assert!(sig.is_none());
    }

    /// stddev 임계값 미달: BelowThreshold -> None (진입)
    #[test]
    fn test_entry_signal_below_stddev() {
        let config = ZScoreConfig {
            min_stddev_threshold: 0.01,
            entry_cooldown_sec: 0,
            ..ZScoreConfig::default()
        };

        let max_coin_capital = config.total_capital_usdt * config.max_position_ratio;

        // stddev = 0.005 < 0.01
        let sig = evaluate_entry_signal(
            "BTC",
            0.5,
            0.1,
            0.005,
            Decimal::ZERO,
            max_coin_capital,
            0,
            None,
            &config,
        )
        .unwrap();
        assert!(sig.is_none());
    }

    /// stddev 임계값 미달: BelowThreshold -> None (청산)
    #[test]
    fn test_exit_signal_below_stddev() {
        let config = ZScoreConfig {
            min_stddev_threshold: 0.01,
            ..ZScoreConfig::default()
        };

        // stddev = 0.005 < 0.01 -> 청산도 None
        let sig = evaluate_exit_signal("BTC", 0.15, 0.1, 0.005, true, &config).unwrap();
        assert!(sig.is_none());
    }

    /// 포지션 보유 + z > exit_z -> 청산 시그널 없음
    #[test]
    fn test_exit_signal_z_above_threshold() {
        let config = ZScoreConfig {
            exit_z_threshold: 0.5,
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };

        // z = (0.5 - 0.1) / 0.2 = 2.0 > 0.5
        let sig = evaluate_exit_signal("BTC", 0.5, 0.1, 0.2, true, &config).unwrap();
        assert!(sig.is_none());
    }

    /// max_concurrent_positions 초과 시 진입 거부
    #[test]
    fn test_entry_signal_max_positions_exceeded() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            min_stddev_threshold: 0.001,
            max_concurrent_positions: Some(2),
            entry_cooldown_sec: 0,
            ..ZScoreConfig::default()
        };

        let max_coin_capital = config.total_capital_usdt * config.max_position_ratio;

        // 이미 2개 포지션이 열려 있음 (max=2)
        let sig = evaluate_entry_signal(
            "BTC",
            0.6,
            0.1,
            0.2,
            Decimal::ZERO,
            max_coin_capital,
            2, // 현재 2개 -> max 도달
            None,
            &config,
        )
        .unwrap();
        assert!(sig.is_none());
    }

    /// 쿨다운 경과 후 정상 진입
    #[test]
    fn test_entry_signal_cooldown_elapsed() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            min_stddev_threshold: 0.001,
            entry_cooldown_sec: 5,
            ..ZScoreConfig::default()
        };

        let max_coin_capital = config.total_capital_usdt * config.max_position_ratio;

        // 마지막 진입이 10초 전 -> 쿨다운(5초) 경과
        let last_entry = Some(Utc::now() - chrono::Duration::seconds(10));

        let sig = evaluate_entry_signal(
            "BTC",
            0.6,
            0.1,
            0.2,
            Decimal::ZERO,
            max_coin_capital,
            0,
            last_entry,
            &config,
        )
        .unwrap();
        assert!(sig.is_some());
    }
}
