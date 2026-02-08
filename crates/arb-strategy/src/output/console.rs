//! 콘솔 출력 모듈.
//!
//! tracing 기반 구조화된 로그로 백테스트 결과를 출력합니다.

use rust_decimal::Decimal;
use tracing::info;

use crate::zscore::simulator::BacktestResult;
use crate::zscore::sweep::SweepResult;

/// 백테스트 결과 요약을 콘솔에 출력합니다.
pub fn print_backtest_summary(result: &BacktestResult) {
    info!("========================================");
    info!("         백테스트 결과 요약");
    info!("========================================");
    info!(
        "테스트 기간: {} ~ {}",
        result.test_period_start.format("%Y-%m-%d %H:%M"),
        result.test_period_end.format("%Y-%m-%d %H:%M")
    );
    info!("코인: {:?}", result.config.coins);
    info!(
        "윈도우: {} | 진입 Z: {} | 청산 Z: {}",
        result.config.window_size, result.config.entry_z_threshold, result.config.exit_z_threshold
    );
    info!(
        "총 자본: {} USDT | 포지션 비율: {}",
        result.config.total_capital_usdt, result.config.position_ratio
    );
    info!("----------------------------------------");
    info!("총 거래: {}회", result.total_trades);
    info!(
        "  수익: {}회 | 손실: {}회 | 강제청산: {}회",
        result.winning_trades, result.losing_trades, result.liquidated_trades
    );
    info!("  승률: {:.1}%", result.win_rate * 100.0);
    info!("----------------------------------------");
    info!("총 PnL (gross): {} USDT", result.total_pnl);
    info!("총 수수료: {} USDT", result.total_fees);
    info!("순 PnL: {} USDT", result.net_pnl);
    info!("최대 낙폭: {} USDT", result.max_drawdown);
    info!("평균 보유: {:.1}분", result.avg_holding_minutes);

    if !result.open_positions.is_empty() {
        info!("----------------------------------------");
        info!(
            "미청산 포지션: {}개 | Unrealized PnL: {} USDT",
            result.open_positions.len(),
            result.unrealized_pnl
        );
        for pos in &result.open_positions {
            info!(
                "  {} | 진입: {} | 크기: {} USDT | Z: {:.2}",
                pos.coin,
                pos.entry_time.format("%m-%d %H:%M"),
                pos.size_usdt,
                pos.entry_z_score
            );
        }
    }

    if !result.daily_pnl.is_empty() {
        info!("----------------------------------------");
        info!("일별 PnL:");
        for (date, pnl) in &result.daily_pnl {
            let sign = if *pnl >= Decimal::ZERO { "+" } else { "" };
            info!("  {} | {}{} USDT", date, sign, pnl);
        }
    }

    info!("========================================");
}

/// 개별 거래 내역을 콘솔에 출력합니다.
pub fn print_trade_detail(result: &BacktestResult) {
    if result.trades.is_empty() {
        info!("거래 내역이 없습니다.");
        return;
    }

    info!("========================================");
    info!("         거래 내역 상세");
    info!("========================================");

    for (i, trade) in result.trades.iter().enumerate() {
        let status = if trade.is_liquidated {
            "[LIQUIDATED]"
        } else {
            ""
        };
        info!(
            "#{:03} {} {} | {} ~ {} | {:.0}분",
            i + 1,
            trade.coin,
            status,
            trade.entry_time.format("%m-%d %H:%M"),
            trade.exit_time.format("%m-%d %H:%M"),
            trade.holding_minutes,
        );
        info!(
            "     Z: {:.2} -> {:.2} | Spread: {:.4}% -> {:.4}%",
            trade.entry_z_score, trade.exit_z_score, trade.entry_spread_pct, trade.exit_spread_pct,
        );
        info!(
            "     PnL: Upbit={} Bybit={} Fee={} Net={}",
            trade.upbit_pnl, trade.bybit_pnl, trade.total_fees, trade.net_pnl,
        );
    }
}

/// 파라미터 sweep 결과를 비교 테이블로 출력합니다.
pub fn print_sweep_summary(result: &SweepResult) {
    info!("========================================");
    info!("    Z-Score 파라미터 Sweep 결과");
    info!("========================================");
    info!(
        "코인: {} | 기간: {} ~ {} | 자본: {} USDT",
        result.coin,
        result.period_start.format("%Y-%m-%d"),
        result.period_end.format("%Y-%m-%d"),
        result.total_capital_usdt
    );
    info!("");

    // 헤더
    info!(
        " entry_z | exit_z | 거래수 | 승률   | 순 PnL      | 실현ROI | 총ROI  | PF   | Ret/DD | Max DD     | 평균보유"
    );
    info!(
        "---------|--------|--------|--------|-------------|---------|--------|------|--------|------------|--------"
    );

    for row in &result.rows {
        // entry_z < 1.25 경고 마크
        let warn_mark = if row.entry_z < 1.25 { "!" } else { " " };

        let win_rate_str = if row.total_trades > 0 {
            format!("{:.1}%", row.win_rate * 100.0)
        } else {
            "N/A".to_string()
        };

        let pf_str = if row.total_trades == 0 {
            "N/A".to_string()
        } else if row.profit_factor.is_infinite() {
            "Inf".to_string()
        } else {
            format!("{:.2}", row.profit_factor)
        };

        let retdd_str = if row.total_trades == 0 {
            "N/A".to_string()
        } else if row.return_max_dd_ratio.is_infinite() {
            "Inf".to_string()
        } else {
            format!("{:.2}", row.return_max_dd_ratio)
        };

        let avg_hold_str = if row.total_trades > 0 {
            format!("{:.1}분", row.avg_holding_minutes)
        } else {
            "N/A".to_string()
        };

        info!(
            "{}{:>6.2} | {:>6.2} | {:>6} | {:>6} | {:>+9} USDT | {:>+6.2}% | {:>+5.2}% | {:>4} | {:>6} | {:>8} USDT | {:>6}",
            warn_mark,
            row.entry_z,
            row.exit_z,
            row.total_trades,
            win_rate_str,
            row.net_pnl,
            row.realized_roi_pct,
            row.total_roi_pct,
            pf_str,
            retdd_str,
            row.max_drawdown,
            avg_hold_str
        );
    }

    // 중복 결과 감지 (연속된 행의 net_pnl과 거래 수가 동일하면 수수료 floor에 의한 중복)
    let mut dup_count = 0usize;
    for i in 1..result.rows.len() {
        if result.rows[i].total_trades == result.rows[i - 1].total_trades
            && result.rows[i].net_pnl == result.rows[i - 1].net_pnl
        {
            dup_count += 1;
        }
    }
    if dup_count > 0 {
        info!("");
        info!(
            "! {dup_count}개 조합이 동일 결과: 수수료 대비 변동성이 낮아 \
             expected_profit 필터가 entry_z보다 강하게 작용합니다."
        );
        info!("  더 높은 entry_z 범위로 sweep하거나, 수수료가 낮은 구간을 탐색하세요.");
    }

    // entry_z < 1.25 경고
    if result.rows.iter().any(|r| r.entry_z < 1.25) {
        info!("");
        info!("! entry_z < 1.25: 노이즈 거래 다수 발생 가능, 수수료 손실 주의");
    }

    // 최적 파라미터 (total_roi_pct 기준, tiebreaker: profit_factor > return_max_dd_ratio)
    if let Some(best) = result.rows.iter().max_by(|a, b| {
        a.total_roi_pct
            .partial_cmp(&b.total_roi_pct)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(
                a.profit_factor
                    .partial_cmp(&b.profit_factor)
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
            .then(
                a.return_max_dd_ratio
                    .partial_cmp(&b.return_max_dd_ratio)
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
    }) {
        info!("");
        info!(
            "* 최적: entry_z={:.2}, exit_z={:.2} (총ROI {:+.2}%, PF {}, {}거래)",
            best.entry_z,
            best.exit_z,
            best.total_roi_pct,
            if best.profit_factor.is_infinite() {
                "Inf".to_string()
            } else {
                format!("{:.2}", best.profit_factor)
            },
            best.total_trades
        );
    }

    // 과적합 경고 (항상 출력)
    info!("");
    info!("! 과적합 주의: sweep 결과는 탐색적 분석 목적으로만 활용하세요.");
    info!("  walk-forward validation 없이 실전 적용은 위험합니다.");
    info!("========================================");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zscore::config::ZScoreConfig;
    use crate::zscore::sweep::SweepResultRow;
    use chrono::Utc;

    #[test]
    fn test_print_backtest_summary_no_panic() {
        let result = BacktestResult {
            config: ZScoreConfig::default(),
            test_period_start: Utc::now(),
            test_period_end: Utc::now(),
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            liquidated_trades: 0,
            win_rate: 0.0,
            total_pnl: Decimal::ZERO,
            total_fees: Decimal::ZERO,
            net_pnl: Decimal::ZERO,
            max_drawdown: Decimal::ZERO,
            avg_holding_minutes: 0.0,
            trades: vec![],
            open_positions: vec![],
            unrealized_pnl: Decimal::ZERO,
            daily_pnl: vec![],
            stationarity_metrics: None,
            estimated_half_life: None,
        };
        // panic 없이 실행되면 성공
        print_backtest_summary(&result);
        print_trade_detail(&result);
    }

    /// 거래가 있는 행과 없는 행을 포함한 SweepResult를 출력해도 panic하지 않는지 검증.
    #[test]
    fn test_print_sweep_summary_no_panic() {
        let row_with_trades = SweepResultRow {
            entry_z: 2.0,
            exit_z: 0.5,
            total_trades: 10,
            winning_trades: 6,
            losing_trades: 4,
            liquidated_trades: 0,
            win_rate: 0.6,
            total_pnl: Decimal::new(200, 0),
            total_fees: Decimal::new(30, 0),
            net_pnl: Decimal::new(170, 0),
            max_drawdown: Decimal::new(50, 0),
            avg_holding_minutes: 45.0,
            realized_roi_pct: 1.7,
            total_roi_pct: 1.8,
            open_position_count: 0,
            unrealized_pnl: Decimal::new(10, 0),
            profit_factor: 2.5,
            return_max_dd_ratio: 3.4,
        };

        let row_no_trades = SweepResultRow {
            entry_z: 1.0,
            exit_z: 0.3,
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            liquidated_trades: 0,
            win_rate: 0.0,
            total_pnl: Decimal::ZERO,
            total_fees: Decimal::ZERO,
            net_pnl: Decimal::ZERO,
            max_drawdown: Decimal::ZERO,
            avg_holding_minutes: 0.0,
            realized_roi_pct: 0.0,
            total_roi_pct: 0.0,
            open_position_count: 0,
            unrealized_pnl: Decimal::ZERO,
            profit_factor: f64::INFINITY,
            return_max_dd_ratio: f64::INFINITY,
        };

        let sweep_result = SweepResult {
            rows: vec![row_with_trades, row_no_trades],
            coin: "BTC".to_string(),
            period_start: Utc::now(),
            period_end: Utc::now(),
            total_capital_usdt: Decimal::new(10000, 0),
        };

        // panic 없이 실행되면 성공
        print_sweep_summary(&sweep_result);
    }

    /// 빈 rows로도 panic하지 않는지 검증.
    #[test]
    fn test_print_sweep_summary_empty_rows() {
        let sweep_result = SweepResult {
            rows: vec![],
            coin: "ETH".to_string(),
            period_start: Utc::now(),
            period_end: Utc::now(),
            total_capital_usdt: Decimal::new(5000, 0),
        };

        // panic 없이 실행되면 성공
        print_sweep_summary(&sweep_result);
    }
}
