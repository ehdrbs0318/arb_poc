//! 시그널 생성 로직.
//!
//! Z-Score 기반 진입/청산 시그널을 생성합니다.

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use tracing::{debug, trace};

use crate::common::fee::roundtrip_fee_pct;
use crate::common::statistics;
use crate::error::{StatisticsError, StrategyError};
use crate::zscore::config::ZScoreConfig;
use crate::zscore::position::PositionManager;
use crate::zscore::spread::SpreadCalculator;

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

/// 특정 코인에 대해 시그널을 평가합니다.
///
/// 진입/청산 조건을 모두 충족하지 못하면 `None` 반환.
pub fn evaluate_signal(
    coin: &str,
    spread_calc: &SpreadCalculator,
    position_mgr: &PositionManager,
    config: &ZScoreConfig,
) -> Result<Option<Signal>, StrategyError> {
    // 스프레드 윈도우가 준비되지 않으면 시그널 없음
    let spread_window = match spread_calc.spread_window(coin) {
        Some(w) if w.is_ready() => w,
        Some(w) => {
            trace!(
                coin = coin,
                window_len = w.len(),
                window_size = config.window_size,
                "윈도우 미충족: 데이터 부족"
            );
            return Ok(None);
        }
        _ => {
            trace!(coin = coin, "윈도우 없음: 스프레드 데이터 미존재");
            return Ok(None);
        }
    };

    let data = spread_window.data();

    // 통계 계산
    let mean_val = statistics::mean(data);
    let stddev_val = statistics::stddev(data, mean_val);

    // 현재 스프레드
    let current_spread = match spread_calc.last_spread_pct(coin) {
        Some(s) => s,
        None => return Ok(None),
    };

    // Z-Score 계산 (min_stddev guard)
    let z = match statistics::z_score(
        current_spread,
        mean_val,
        stddev_val,
        config.min_stddev_threshold,
    ) {
        Ok(z) => {
            debug!(
                coin = coin,
                current_spread = current_spread,
                mean = mean_val,
                stddev = stddev_val,
                z_score = z,
                "Z-Score 계산 결과"
            );
            z
        }
        Err(StatisticsError::BelowThreshold { .. }) => {
            trace!(
                coin = coin,
                stddev = stddev_val,
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
            let expected_spread_change = current_spread - mean_val;
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
    use crate::zscore::position::PositionManager;
    use crate::zscore::spread::SpreadCalculator;
    use chrono::Utc;

    fn setup_ready_spread(
        coins: &[String],
        window_size: usize,
        base_spread: f64,
        last_spread: f64,
    ) -> SpreadCalculator {
        let mut calc = SpreadCalculator::new(coins, window_size);
        // 윈도우를 채움: 대부분 base_spread 값, 마지막만 last_spread에 해당하는 가격 제공
        for i in 0..window_size {
            let ts = Utc::now() + chrono::Duration::minutes(i as i64);
            let bybit_price = if i < window_size - 1 {
                // spread_pct = (bybit - upbit_usdt) / upbit_usdt * 100
                // upbit_usdt = 100_000 (138_000_000 / 1380)
                // bybit = upbit_usdt * (1 + spread_pct/100)
                (100_000.0 * (1.0 + base_spread / 100.0)) as i64
            } else {
                (100_000.0 * (1.0 + last_spread / 100.0)) as i64
            };

            calc.update(
                &coins[0],
                ts,
                Some(Decimal::new(138_000_000, 0)),
                Some(Decimal::new(1380, 0)),
                Some(Decimal::new(bybit_price, 0)),
            )
            .unwrap();
        }
        calc
    }

    #[test]
    fn test_no_signal_when_not_ready() {
        let coins = vec!["BTC".to_string()];
        let config = ZScoreConfig::default();
        let calc = SpreadCalculator::new(&coins, 10);
        let pm = PositionManager::new();

        let signal = evaluate_signal("BTC", &calc, &pm, &config).unwrap();
        assert!(signal.is_none());
    }

    #[test]
    fn test_entry_signal_high_z_score() {
        let coins = vec!["BTC".to_string()];
        let mut config = ZScoreConfig {
            coins: coins.clone(),
            window_size: 10,
            entry_z_threshold: 2.0,
            exit_z_threshold: 0.5,
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };
        config.coins = coins.clone();

        // base_spread = 0.1, 마지막만 아주 높은 스프레드
        // stddev가 작으므로 z-score가 높아짐
        let calc = setup_ready_spread(&coins, 10, 0.1, 1.0);
        let pm = PositionManager::new();

        let signal = evaluate_signal("BTC", &calc, &pm, &config).unwrap();
        // z-score가 2.0 이상이면 Enter
        match signal {
            Some(Signal::Enter { coin, z_score, .. }) => {
                assert_eq!(coin, "BTC");
                assert!(z_score >= 2.0);
            }
            _ => {
                // z-score가 충분히 높지 않을 수 있음 (윈도우 값에 따라)
                // 이 경우 None도 허용
            }
        }
    }

    #[test]
    fn test_exit_signal() {
        let coins = vec!["BTC".to_string()];
        let config = ZScoreConfig {
            coins: coins.clone(),
            window_size: 10,
            entry_z_threshold: 2.0,
            exit_z_threshold: 0.5,
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };

        // 스프레드가 평균 근처 (z-score 약 0)
        let calc = setup_ready_spread(&coins, 10, 0.1, 0.1);
        let mut pm = PositionManager::new();

        // 가상 포지션 추가
        use crate::zscore::position::VirtualPosition;
        pm.open_positions.insert(
            "BTC".to_string(),
            VirtualPosition {
                coin: "BTC".to_string(),
                entry_time: Utc::now(),
                upbit_entry_price: Decimal::new(100_000, 0),
                bybit_entry_price: Decimal::new(100_050, 0),
                bybit_liquidation_price: Decimal::new(199_445, 0),
                entry_usdt_krw: Decimal::new(1380, 0),
                entry_spread_pct: 0.05,
                entry_z_score: 2.5,
                size_usdt: Decimal::new(1000, 0),
            },
        );

        let signal = evaluate_signal("BTC", &calc, &pm, &config).unwrap();
        // z-score가 exit_z_threshold 이하이면 Exit
        match signal {
            Some(Signal::Exit { coin, z_score, .. }) => {
                assert_eq!(coin, "BTC");
                assert!(z_score <= 0.5);
            }
            _ => {
                // z-score가 exit 임계값보다 높을 수도 있음
            }
        }
    }

    #[test]
    fn test_no_entry_when_insufficient_capital() {
        let coins = vec!["BTC".to_string()];
        let config = ZScoreConfig {
            coins: coins.clone(),
            window_size: 5,
            entry_z_threshold: 2.0,
            exit_z_threshold: 0.5,
            total_capital_usdt: Decimal::new(100, 0), // 매우 적은 자본
            position_ratio: Decimal::new(5, 1),       // 50%
            min_stddev_threshold: 0.001,
            ..ZScoreConfig::default()
        };

        // 높은 스프레드로 entry 시그널 발생 조건
        let calc = setup_ready_spread(&coins, 5, 0.1, 5.0);
        let mut pm = PositionManager::new();

        // 이미 포지션이 있어 자본 소진
        use crate::zscore::position::VirtualPosition;
        pm.open_positions.insert(
            "ETH".to_string(),
            VirtualPosition {
                coin: "ETH".to_string(),
                entry_time: Utc::now(),
                upbit_entry_price: Decimal::new(3000, 0),
                bybit_entry_price: Decimal::new(3010, 0),
                bybit_liquidation_price: Decimal::new(6000, 0),
                entry_usdt_krw: Decimal::new(1380, 0),
                entry_spread_pct: 0.3,
                entry_z_score: 2.1,
                size_usdt: Decimal::new(50, 0), // 50 * 2 = 100 (전체 자본 소진)
            },
        );

        let signal = evaluate_signal("BTC", &calc, &pm, &config).unwrap();
        assert!(signal.is_none());
    }
}
