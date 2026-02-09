//! 시그널 생성 로직.
//!
//! Z-Score 기반 진입/청산 시그널을 생성합니다.
//! 틱 기반 시그널 평가를 위해 외부에서 current_spread, mean, stddev를
//! 직접 전달받습니다.

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use tracing::{debug, trace};

use crate::common::fee::roundtrip_fee_pct;
use crate::common::statistics;
use crate::error::{StatisticsError, StrategyError};
use crate::zscore::config::ZScoreConfig;
use crate::zscore::position::PositionManager;

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

/// 틱 수신 시 즉시 시그널을 평가합니다.
///
/// 외부에서 전달받은 current_spread, mean, stddev로 Z-Score를 계산하고
/// 진입/청산 시그널을 생성합니다.
///
/// # 인자
/// - `coin`: 코인 심볼
/// - `current_spread`: 현재 틱에서 계산한 스프레드 (%)
/// - `mean`: 분봉 기반 rolling mean (%)
/// - `stddev`: 분봉 기반 rolling stddev
/// - `position_mgr`: 포지션 매니저
/// - `config`: 전략 설정
///
/// # 반환값
/// - `Ok(Some(Signal))`: 진입 또는 청산 시그널
/// - `Ok(None)`: 시그널 없음
/// - `Err(...)`: Z-Score 계산 실패
pub fn evaluate_tick_signal(
    coin: &str,
    current_spread: f64,
    mean: f64,
    stddev: f64,
    position_mgr: &PositionManager,
    config: &ZScoreConfig,
) -> Result<Option<Signal>, StrategyError> {
    // Z-Score 계산 (min_stddev guard)
    let z = match statistics::z_score(current_spread, mean, stddev, config.min_stddev_threshold) {
        Ok(z) => {
            debug!(
                coin = coin,
                current_spread = current_spread,
                mean = mean,
                stddev = stddev,
                z_score = z,
                "Z-Score 계산 결과"
            );
            z
        }
        Err(StatisticsError::BelowThreshold { .. }) => {
            trace!(
                coin = coin,
                stddev = stddev,
                threshold = config.min_stddev_threshold,
                "stddev 임계값 미달: 시그널 생략"
            );
            return Ok(None);
        }
        Err(e) => return Err(StrategyError::Statistics(e)),
    };

    let has_position = position_mgr.has_position(coin);

    if has_position {
        // 청산 조건: z_score <= exit_z_threshold
        if z <= config.exit_z_threshold {
            debug!(
                coin = coin,
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
    } else {
        // 진입 조건:
        // 1. z_score >= entry_z_threshold
        // 2. expected_profit_pct > 0
        // 3. 해당 코인에 기존 포지션 없음 (위에서 체크)
        // 4. 가용 자본 충분
        if z >= config.entry_z_threshold {
            // expected_profit_pct 계산
            let expected_spread_change = current_spread - mean;
            let fee_pct = roundtrip_fee_pct(config.upbit_taker_fee, config.bybit_taker_fee);
            let fee_f64 = fee_pct.to_f64().unwrap_or(0.0);
            let expected_profit = expected_spread_change - fee_f64;

            if expected_profit > 0.0 {
                // 가용 자본 확인
                let size_usdt = config.total_capital_usdt * config.position_ratio;
                let required = size_usdt * Decimal::from(2u64);
                let available = position_mgr.available_capital(config.total_capital_usdt);

                // max_concurrent_positions 확인
                let max_positions = config
                    .max_concurrent_positions
                    .unwrap_or(config.coins.len());
                let current_positions = position_mgr.open_count();

                debug!(
                    coin = coin,
                    z_score = z,
                    expected_profit = expected_profit,
                    available_capital = %available,
                    required_capital = %required,
                    current_positions = current_positions,
                    max_positions = max_positions,
                    "진입 조건 평가"
                );

                if available >= required && current_positions < max_positions {
                    return Ok(Some(Signal::Enter {
                        coin: coin.to_string(),
                        z_score: z,
                        spread_pct: current_spread,
                        expected_profit_pct: expected_profit,
                    }));
                }

                debug!(
                    coin = coin,
                    capital_ok = available >= required,
                    position_ok = current_positions < max_positions,
                    "진입 조건 불충족: 자본 또는 포지션 한도"
                );
            } else {
                // 기대 수익이 수수료를 커버하지 못함
                debug!(
                    coin = coin,
                    z_score = z,
                    expected_profit = expected_profit,
                    fee_pct = fee_f64,
                    "진입 조건 불충족: 기대 수익 부족"
                );
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zscore::config::ZScoreConfig;
    use crate::zscore::position::{PositionManager, VirtualPosition};
    use chrono::Utc;
    use rust_decimal::Decimal;

    /// 진입 시그널: z >= entry_z AND profit > fee
    #[test]
    fn test_tick_entry_signal() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            exit_z_threshold: 0.5,
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };
        let pm = PositionManager::new();

        // mean=0.1, stddev=0.2, spread=0.6
        // z = (0.6 - 0.1) / 0.2 = 2.5 >= 2.0
        // expected_profit = (0.6 - 0.1) - fee(~0.21) = 0.29 > 0
        let sig = evaluate_tick_signal("BTC", 0.6, 0.1, 0.2, &pm, &config).unwrap();
        match sig {
            Some(Signal::Enter { coin, z_score, .. }) => {
                assert_eq!(coin, "BTC");
                assert!(z_score >= 2.0);
            }
            _ => panic!("진입 시그널이 생성되어야 합니다"),
        }
    }

    /// 청산 시그널: 포지션 보유 + z <= exit_z
    #[test]
    fn test_tick_exit_signal() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            exit_z_threshold: 0.5,
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };
        let mut pm = PositionManager::new();

        // BTC 포지션 추가
        pm.open_positions.insert(
            "BTC".to_string(),
            VirtualPosition {
                coin: "BTC".to_string(),
                entry_time: Utc::now(),
                upbit_entry_price: Decimal::new(100_000, 0),
                bybit_entry_price: Decimal::new(100_050, 0),
                bybit_liquidation_price: Decimal::new(199_445, 0),
                entry_usd_krw: 1380.0,
                entry_spread_pct: 0.05,
                entry_z_score: 2.5,
                size_usdt: Decimal::new(1000, 0),
            },
        );

        // mean=0.1, stddev=0.2, spread=0.15
        // z = (0.15 - 0.1) / 0.2 = 0.25 <= 0.5
        let sig = evaluate_tick_signal("BTC", 0.15, 0.1, 0.2, &pm, &config).unwrap();
        match sig {
            Some(Signal::Exit { coin, z_score, .. }) => {
                assert_eq!(coin, "BTC");
                assert!(z_score <= 0.5);
            }
            _ => panic!("청산 시그널이 생성되어야 합니다"),
        }
    }

    /// 시그널 없음: z가 entry/exit 범위 밖
    #[test]
    fn test_tick_no_signal() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            exit_z_threshold: 0.5,
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };
        let pm = PositionManager::new();

        // mean=0.1, stddev=0.2, spread=0.3
        // z = (0.3 - 0.1) / 0.2 = 1.0 -> 0.5 < z < 2.0
        let sig = evaluate_tick_signal("BTC", 0.3, 0.1, 0.2, &pm, &config).unwrap();
        assert!(sig.is_none());
    }

    /// stddev 임계값 미달: BelowThreshold -> None (에러 아님)
    #[test]
    fn test_tick_below_stddev_threshold() {
        let config = ZScoreConfig {
            min_stddev_threshold: 0.01,
            ..ZScoreConfig::default()
        };
        let pm = PositionManager::new();

        // stddev = 0.005 < 0.01
        let sig = evaluate_tick_signal("BTC", 0.5, 0.1, 0.005, &pm, &config).unwrap();
        assert!(sig.is_none());
    }

    /// 자본 부족 시 진입 거부
    #[test]
    fn test_tick_no_entry_insufficient_capital() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            total_capital_usdt: Decimal::new(100, 0),
            position_ratio: Decimal::new(5, 1), // 0.5
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };
        let mut pm = PositionManager::new();

        // 기존 포지션이 자본 소진
        pm.open_positions.insert(
            "ETH".to_string(),
            VirtualPosition {
                coin: "ETH".to_string(),
                entry_time: Utc::now(),
                upbit_entry_price: Decimal::new(3000, 0),
                bybit_entry_price: Decimal::new(3010, 0),
                bybit_liquidation_price: Decimal::new(6000, 0),
                entry_usd_krw: 1380.0,
                entry_spread_pct: 0.3,
                entry_z_score: 2.1,
                size_usdt: Decimal::new(50, 0), // 50 * 2 = 100 (전체 자본 소진)
            },
        );

        // z = (0.6 - 0.1) / 0.2 = 2.5 -> entry 조건 충족하지만 자본 부족
        let sig = evaluate_tick_signal("BTC", 0.6, 0.1, 0.2, &pm, &config).unwrap();
        assert!(sig.is_none());
    }

    /// 수수료 부족: expected_profit <= 0
    #[test]
    fn test_tick_no_entry_fee_exceeds_profit() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };
        let pm = PositionManager::new();

        // mean=0.1, stddev=0.05, spread=0.2
        // z = (0.2 - 0.1) / 0.05 = 2.0 >= 2.0
        // expected_profit = (0.2 - 0.1) - fee(~0.21) = -0.11 < 0
        let sig = evaluate_tick_signal("BTC", 0.2, 0.1, 0.05, &pm, &config).unwrap();
        assert!(sig.is_none());
    }
}
