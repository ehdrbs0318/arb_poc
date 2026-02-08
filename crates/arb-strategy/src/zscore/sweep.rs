//! 파라미터 sweep 설정 및 결과 타입.
//!
//! 다양한 entry_z / exit_z 조합을 한 번의 데이터 수집으로 일괄 백테스트합니다.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use tracing::{info, warn};

use arb_exchange::MarketData;

use crate::error::StrategyError;
use crate::zscore::config::ZScoreConfig;
use crate::zscore::simulator::{BacktestResult, fetch_candle_data, simulate_with_cache};

/// Sweep 설정.
#[derive(Clone, Debug)]
pub struct SweepConfig {
    /// 기본 전략 설정 (entry_z / exit_z는 sweep 값으로 오버라이드).
    pub base_config: ZScoreConfig,
    /// 테스트할 entry_z 값 목록.
    pub entry_z_values: Vec<f64>,
    /// 테스트할 exit_z 값 목록 (비어있으면 base_config.exit_z_threshold 사용).
    pub exit_z_values: Vec<f64>,
    /// 최대 조합 수 (기본 50).
    pub max_combinations: usize,
}

impl SweepConfig {
    /// 설정 유효성 검증.
    pub fn validate(&self) -> Result<(), StrategyError> {
        if self.entry_z_values.is_empty() {
            return Err(StrategyError::Config(
                "entry_z_values must not be empty".to_string(),
            ));
        }

        for &z in &self.entry_z_values {
            if z <= 0.0 {
                return Err(StrategyError::Config(format!(
                    "entry_z must be positive, got {z}"
                )));
            }
        }

        let exit_count = if self.exit_z_values.is_empty() {
            1
        } else {
            self.exit_z_values.len()
        };
        let total = self.entry_z_values.len() * exit_count;
        if total > self.max_combinations {
            return Err(StrategyError::Config(format!(
                "total combinations ({total}) exceeds max_combinations ({})",
                self.max_combinations
            )));
        }

        Ok(())
    }
}

/// Sweep 결과 행 (단일 파라미터 조합의 백테스트 결과 요약).
#[derive(Clone, Debug)]
pub struct SweepResultRow {
    /// entry_z 값.
    pub entry_z: f64,
    /// exit_z 값.
    pub exit_z: f64,
    /// 총 거래 횟수.
    pub total_trades: usize,
    /// 수익 거래 횟수.
    pub winning_trades: usize,
    /// 손실 거래 횟수.
    pub losing_trades: usize,
    /// 강제 청산 거래 횟수.
    pub liquidated_trades: usize,
    /// 승률.
    pub win_rate: f64,
    /// 총 gross PnL.
    pub total_pnl: Decimal,
    /// 총 수수료.
    pub total_fees: Decimal,
    /// 순 PnL.
    pub net_pnl: Decimal,
    /// 최대 낙폭.
    pub max_drawdown: Decimal,
    /// 평균 보유 시간 (분).
    pub avg_holding_minutes: f64,
    /// 실현 ROI (%) = net_pnl / total_capital * 100.
    pub realized_roi_pct: f64,
    /// 총 ROI (%) = (net_pnl + unrealized_pnl) / total_capital * 100.
    pub total_roi_pct: f64,
    /// 미청산 포지션 수.
    pub open_position_count: usize,
    /// 미청산 PnL.
    pub unrealized_pnl: Decimal,
    /// Profit factor = 총이익 / 총손실 (손실 0이면 f64::INFINITY).
    pub profit_factor: f64,
    /// Return / Max Drawdown 비율 (DD 0이면 f64::INFINITY).
    pub return_max_dd_ratio: f64,
}

impl SweepResultRow {
    /// BacktestResult에서 요약 행을 생성합니다.
    pub fn from_backtest_result(result: &BacktestResult, total_capital: Decimal) -> Self {
        // profit_factor 계산: 이익 거래 net_pnl 합 / 손실 거래 net_pnl 합(절대값)
        let gross_profit: Decimal = result
            .trades
            .iter()
            .filter(|t| t.net_pnl > Decimal::ZERO)
            .map(|t| t.net_pnl)
            .sum();
        let gross_loss: Decimal = result
            .trades
            .iter()
            .filter(|t| t.net_pnl < Decimal::ZERO)
            .map(|t| t.net_pnl.abs())
            .sum();

        let profit_factor = if gross_loss == Decimal::ZERO {
            f64::INFINITY
        } else {
            // Decimal -> f64 변환
            let p = gross_profit.to_string().parse::<f64>().unwrap_or(0.0);
            let l = gross_loss.to_string().parse::<f64>().unwrap_or(1.0);
            p / l
        };

        let net_pnl_f64 = result.net_pnl.to_string().parse::<f64>().unwrap_or(0.0);
        let max_dd_f64 = result
            .max_drawdown
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        let capital_f64 = total_capital.to_string().parse::<f64>().unwrap_or(1.0);
        let unrealized_f64 = result
            .unrealized_pnl
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);

        let return_max_dd_ratio = if max_dd_f64 == 0.0 {
            f64::INFINITY
        } else {
            net_pnl_f64 / max_dd_f64
        };

        let realized_roi_pct = net_pnl_f64 / capital_f64 * 100.0;
        let total_roi_pct = (net_pnl_f64 + unrealized_f64) / capital_f64 * 100.0;

        Self {
            entry_z: result.config.entry_z_threshold,
            exit_z: result.config.exit_z_threshold,
            total_trades: result.total_trades,
            winning_trades: result.winning_trades,
            losing_trades: result.losing_trades,
            liquidated_trades: result.liquidated_trades,
            win_rate: result.win_rate,
            total_pnl: result.total_pnl,
            total_fees: result.total_fees,
            net_pnl: result.net_pnl,
            max_drawdown: result.max_drawdown,
            avg_holding_minutes: result.avg_holding_minutes,
            realized_roi_pct,
            total_roi_pct,
            open_position_count: result.open_positions.len(),
            unrealized_pnl: result.unrealized_pnl,
            profit_factor,
            return_max_dd_ratio,
        }
    }
}

/// Sweep 결과 (모든 파라미터 조합의 결과).
pub struct SweepResult {
    /// 각 조합별 결과 행.
    pub rows: Vec<SweepResultRow>,
    /// 대상 코인.
    pub coin: String,
    /// 테스트 기간 시작.
    pub period_start: DateTime<Utc>,
    /// 테스트 기간 종료.
    pub period_end: DateTime<Utc>,
    /// 총 자본금 (USDT).
    pub total_capital_usdt: Decimal,
}

/// 파라미터 sweep을 실행합니다.
///
/// 데이터를 1회만 수집하고, 모든 entry_z / exit_z 조합에 대해 시뮬레이션을 반복합니다.
pub async fn run_sweep<U: MarketData, B: MarketData>(
    upbit: &U,
    bybit: &B,
    sweep_config: &SweepConfig,
) -> Result<SweepResult, StrategyError> {
    sweep_config.validate()?;

    // entry_z < 1.25 경고
    if sweep_config.entry_z_values.iter().any(|&z| z < 1.25) {
        warn!("entry_z < 1.25 포함: 노이즈 거래 다수 발생 가능, 수수료 손실 주의");
    }

    // 데이터 1회 수집
    let cache = fetch_candle_data(upbit, bybit, &sweep_config.base_config).await?;

    let mut rows = Vec::new();
    let mut sorted_entry_z = sweep_config.entry_z_values.clone();
    sorted_entry_z.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let exit_z_values = if sweep_config.exit_z_values.is_empty() {
        vec![sweep_config.base_config.exit_z_threshold]
    } else {
        sweep_config.exit_z_values.clone()
    };

    let total_combinations = sorted_entry_z.len() * exit_z_values.len();
    info!(total_combinations, "파라미터 sweep 시작");

    for &entry_z in &sorted_entry_z {
        for &exit_z in &exit_z_values {
            let mut config = sweep_config.base_config.clone();
            config.entry_z_threshold = entry_z;
            config.exit_z_threshold = exit_z;

            match simulate_with_cache(&config, &cache) {
                Ok(result) => {
                    let row = SweepResultRow::from_backtest_result(
                        &result,
                        sweep_config.base_config.total_capital_usdt,
                    );
                    info!(entry_z, exit_z, trades = row.total_trades, net_pnl = %row.net_pnl, "조합 완료");
                    rows.push(row);
                }
                Err(e) => {
                    warn!(entry_z, exit_z, error = %e, "조합 실패 -- 건너뜀");
                }
            }
        }
    }

    if rows.is_empty() {
        return Err(StrategyError::Config(
            "All parameter combinations failed".to_string(),
        ));
    }

    // period_start/end는 캐시의 timestamps에서 추출
    let period_start = cache.timestamps.first().copied().unwrap_or_else(Utc::now);
    let period_end = cache.timestamps.last().copied().unwrap_or_else(Utc::now);

    Ok(SweepResult {
        rows,
        coin: sweep_config
            .base_config
            .coins
            .first()
            .cloned()
            .unwrap_or_default(),
        period_start,
        period_end,
        total_capital_usdt: sweep_config.base_config.total_capital_usdt,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sweep_config_validate_empty_entry_z() {
        let config = SweepConfig {
            base_config: ZScoreConfig::default(),
            entry_z_values: vec![],
            exit_z_values: vec![],
            max_combinations: 50,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_sweep_config_validate_negative_entry_z() {
        let config = SweepConfig {
            base_config: ZScoreConfig::default(),
            entry_z_values: vec![2.0, -1.0],
            exit_z_values: vec![],
            max_combinations: 50,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_sweep_config_validate_zero_entry_z() {
        let config = SweepConfig {
            base_config: ZScoreConfig::default(),
            entry_z_values: vec![0.0],
            exit_z_values: vec![],
            max_combinations: 50,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_sweep_config_validate_exceeds_max_combinations() {
        let config = SweepConfig {
            base_config: ZScoreConfig::default(),
            entry_z_values: vec![1.5, 2.0, 2.5],
            exit_z_values: vec![0.3, 0.5, 0.7],
            max_combinations: 5, // 3 * 3 = 9 > 5
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_sweep_config_validate_success() {
        let config = SweepConfig {
            base_config: ZScoreConfig::default(),
            entry_z_values: vec![1.5, 2.0, 2.5],
            exit_z_values: vec![0.3, 0.5],
            max_combinations: 50,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_sweep_config_validate_empty_exit_z() {
        // exit_z가 비어있으면 base_config의 exit_z 1개만 사용 -> 조합 = entry_z * 1
        let config = SweepConfig {
            base_config: ZScoreConfig::default(),
            entry_z_values: vec![2.0],
            exit_z_values: vec![],
            max_combinations: 50,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_sweep_result_row_from_backtest_result() {
        let config = ZScoreConfig {
            entry_z_threshold: 2.0,
            exit_z_threshold: 0.5,
            ..ZScoreConfig::default()
        };
        let total_capital = Decimal::new(10000, 0);

        let result = BacktestResult {
            config,
            test_period_start: Utc::now(),
            test_period_end: Utc::now(),
            total_trades: 5,
            winning_trades: 3,
            losing_trades: 2,
            liquidated_trades: 0,
            win_rate: 0.6,
            total_pnl: Decimal::new(150, 0),
            total_fees: Decimal::new(20, 0),
            net_pnl: Decimal::new(130, 0),
            max_drawdown: Decimal::new(50, 0),
            avg_holding_minutes: 45.0,
            trades: vec![],
            open_positions: vec![],
            unrealized_pnl: Decimal::new(10, 0),
            daily_pnl: vec![],
            stationarity_metrics: None,
            estimated_half_life: None,
        };

        let row = SweepResultRow::from_backtest_result(&result, total_capital);
        assert_eq!(row.entry_z, 2.0);
        assert_eq!(row.exit_z, 0.5);
        assert_eq!(row.total_trades, 5);
        assert_eq!(row.winning_trades, 3);
        assert_eq!(row.net_pnl, Decimal::new(130, 0));
        // realized_roi_pct = 130 / 10000 * 100 = 1.3%
        assert!((row.realized_roi_pct - 1.3).abs() < 0.01);
        // total_roi_pct = (130 + 10) / 10000 * 100 = 1.4%
        assert!((row.total_roi_pct - 1.4).abs() < 0.01);
        // return_max_dd_ratio = 130 / 50 = 2.6
        assert!((row.return_max_dd_ratio - 2.6).abs() < 0.01);
    }

    #[test]
    fn test_sweep_result_row_zero_drawdown() {
        let config = ZScoreConfig::default();
        let result = BacktestResult {
            config,
            test_period_start: Utc::now(),
            test_period_end: Utc::now(),
            total_trades: 1,
            winning_trades: 1,
            losing_trades: 0,
            liquidated_trades: 0,
            win_rate: 1.0,
            total_pnl: Decimal::new(10, 0),
            total_fees: Decimal::ZERO,
            net_pnl: Decimal::new(10, 0),
            max_drawdown: Decimal::ZERO,
            avg_holding_minutes: 10.0,
            trades: vec![],
            open_positions: vec![],
            unrealized_pnl: Decimal::ZERO,
            daily_pnl: vec![],
            stationarity_metrics: None,
            estimated_half_life: None,
        };

        let row = SweepResultRow::from_backtest_result(&result, Decimal::new(10000, 0));
        assert!(row.return_max_dd_ratio.is_infinite());
        assert!(row.profit_factor.is_infinite());
    }
}
