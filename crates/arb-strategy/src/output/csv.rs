//! CSV 파일 출력 모듈.
//!
//! 거래 내역과 시계열 데이터를 CSV 파일로 저장합니다.

use std::fs;
use std::io::Write;
use std::path::Path;

use chrono::Utc;
use tracing::info;

use crate::error::StrategyError;
use crate::zscore::pnl::ClosedPosition;
use crate::zscore::simulator::TimeseriesRecord;
use crate::zscore::sweep::SweepResult;

/// 거래 내역을 CSV 파일로 저장합니다.
///
/// 파일명: `trades_{timestamp}.csv`
pub fn write_trades_csv(
    output_dir: &Path,
    trades: &[ClosedPosition],
) -> Result<String, StrategyError> {
    fs::create_dir_all(output_dir)
        .map_err(|e| StrategyError::Config(format!("output directory creation failed: {e}")))?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("trades_{timestamp}.csv");
    let filepath = output_dir.join(&filename);

    let mut wtr = csv::Writer::from_path(&filepath)?;

    // 헤더
    wtr.write_record([
        "coin",
        "entry_time",
        "exit_time",
        "holding_min",
        "size_usdt",
        "entry_z",
        "exit_z",
        "entry_spread_pct",
        "exit_spread_pct",
        "upbit_pnl",
        "bybit_pnl",
        "upbit_fees",
        "bybit_fees",
        "net_pnl",
        "entry_usdt_krw",
        "exit_usdt_krw",
        "is_liquidated",
    ])?;

    for trade in trades {
        wtr.write_record([
            &trade.coin,
            &trade.entry_time.to_rfc3339(),
            &trade.exit_time.to_rfc3339(),
            &trade.holding_minutes.to_string(),
            &trade.size_usdt.to_string(),
            &format!("{:.4}", trade.entry_z_score),
            &format!("{:.4}", trade.exit_z_score),
            &format!("{:.6}", trade.entry_spread_pct),
            &format!("{:.6}", trade.exit_spread_pct),
            &trade.upbit_pnl.to_string(),
            &trade.bybit_pnl.to_string(),
            &trade.upbit_fees.to_string(),
            &trade.bybit_fees.to_string(),
            &trade.net_pnl.to_string(),
            &trade.entry_usdt_krw.to_string(),
            &trade.exit_usdt_krw.to_string(),
            &trade.is_liquidated.to_string(),
        ])?;
    }

    wtr.flush()?;

    info!(
        filepath = filepath.display().to_string(),
        records = trades.len(),
        "거래 내역 CSV 저장 완료"
    );

    Ok(filepath.display().to_string())
}

/// 시계열 데이터를 CSV 파일로 저장합니다.
///
/// 파일명: `timeseries_{timestamp}.csv`
pub fn write_timeseries_csv(
    output_dir: &Path,
    records: &[TimeseriesRecord],
) -> Result<String, StrategyError> {
    fs::create_dir_all(output_dir)
        .map_err(|e| StrategyError::Config(format!("output directory creation failed: {e}")))?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("timeseries_{timestamp}.csv");
    let filepath = output_dir.join(&filename);

    let mut wtr = csv::Writer::from_path(&filepath)?;

    // 헤더
    wtr.write_record([
        "timestamp",
        "coin",
        "upbit_usdt_price",
        "bybit_price",
        "spread_pct",
        "mean_spread_pct",
        "stddev",
        "z_score",
        "signal",
        "position",
    ])?;

    for record in records {
        wtr.write_record([
            &record.timestamp.to_rfc3339(),
            &record.coin,
            &format!("{:.2}", record.upbit_usdt_price),
            &format!("{:.2}", record.bybit_price),
            &format!("{:.6}", record.spread_pct),
            &format!("{:.6}", record.mean_spread_pct),
            &format!("{:.6}", record.stddev),
            &format!("{:.4}", record.z_score),
            &record.signal,
            &record.position,
        ])?;
    }

    wtr.flush()?;

    info!(
        filepath = filepath.display().to_string(),
        records = records.len(),
        "시계열 CSV 저장 완료"
    );

    Ok(filepath.display().to_string())
}

/// Sweep 결과를 CSV 파일로 저장합니다.
///
/// 파일명: `sweep_{timestamp}.csv`
/// 첫 줄에 메타데이터 코멘트(#)를 포함하고, 이후 헤더+데이터 행을 기록합니다.
pub fn write_sweep_csv(output_dir: &Path, result: &SweepResult) -> Result<String, StrategyError> {
    fs::create_dir_all(output_dir)
        .map_err(|e| StrategyError::Config(format!("output directory creation failed: {e}")))?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("sweep_{timestamp}.csv");
    let filepath = output_dir.join(&filename);

    // 메타데이터 코멘트를 먼저 기록
    let mut file = fs::File::create(&filepath)?;
    writeln!(
        file,
        "# coin={},period_start={},period_end={},total_capital_usdt={}",
        result.coin,
        result.period_start.to_rfc3339(),
        result.period_end.to_rfc3339(),
        result.total_capital_usdt
    )?;
    drop(file);

    // append 모드로 csv writer 생성
    let file = fs::OpenOptions::new().append(true).open(&filepath)?;
    let mut wtr = csv::Writer::from_writer(file);

    // 헤더
    wtr.write_record([
        "entry_z",
        "exit_z",
        "total_trades",
        "winning_trades",
        "losing_trades",
        "liquidated_trades",
        "win_rate",
        "total_pnl",
        "total_fees",
        "net_pnl",
        "max_drawdown",
        "avg_holding_min",
        "realized_roi_pct",
        "total_roi_pct",
        "open_positions",
        "unrealized_pnl",
        "profit_factor",
        "return_max_dd_ratio",
    ])?;

    for row in &result.rows {
        let pf_str = if row.profit_factor.is_infinite() {
            "Inf".to_string()
        } else {
            format!("{:.4}", row.profit_factor)
        };
        let retdd_str = if row.return_max_dd_ratio.is_infinite() {
            "Inf".to_string()
        } else {
            format!("{:.4}", row.return_max_dd_ratio)
        };

        wtr.write_record([
            &format!("{:.2}", row.entry_z),
            &format!("{:.2}", row.exit_z),
            &row.total_trades.to_string(),
            &row.winning_trades.to_string(),
            &row.losing_trades.to_string(),
            &row.liquidated_trades.to_string(),
            &format!("{:.4}", row.win_rate),
            &row.total_pnl.to_string(),
            &row.total_fees.to_string(),
            &row.net_pnl.to_string(),
            &row.max_drawdown.to_string(),
            &format!("{:.1}", row.avg_holding_minutes),
            &format!("{:.4}", row.realized_roi_pct),
            &format!("{:.4}", row.total_roi_pct),
            &row.open_position_count.to_string(),
            &row.unrealized_pnl.to_string(),
            &pf_str,
            &retdd_str,
        ])?;
    }

    wtr.flush()?;

    info!(
        filepath = filepath.display().to_string(),
        rows = result.rows.len(),
        "Sweep CSV 저장 완료"
    );

    Ok(filepath.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use tempfile::TempDir;

    use crate::zscore::pnl::ClosedPosition;
    use crate::zscore::simulator::TimeseriesRecord;

    fn make_test_trade() -> ClosedPosition {
        ClosedPosition {
            coin: "BTC".to_string(),
            entry_time: Utc::now(),
            exit_time: Utc::now(),
            holding_minutes: 30,
            size_usdt: Decimal::new(1000, 0),
            upbit_pnl: Decimal::new(120, 2),
            bybit_pnl: Decimal::new(-50, 2),
            upbit_fees: Decimal::new(10, 2),
            bybit_fees: Decimal::new(11, 2),
            total_fees: Decimal::new(21, 2),
            net_pnl: Decimal::new(49, 2),
            entry_z_score: 2.15,
            exit_z_score: 0.45,
            entry_spread_pct: 0.32,
            exit_spread_pct: 0.10,
            entry_usdt_krw: Decimal::new(13805, 1),
            exit_usdt_krw: Decimal::new(13812, 1),
            is_liquidated: false,
        }
    }

    fn make_test_timeseries() -> TimeseriesRecord {
        TimeseriesRecord {
            timestamp: Utc::now(),
            coin: "BTC".to_string(),
            upbit_usdt_price: 99500.0,
            bybit_price: 99800.0,
            spread_pct: 0.30,
            mean_spread_pct: 0.15,
            stddev: 0.08,
            z_score: 1.85,
            signal: "NONE".to_string(),
            position: "NONE".to_string(),
        }
    }

    #[test]
    fn test_write_trades_csv() {
        let dir = TempDir::new().unwrap();
        let trades = vec![make_test_trade()];
        let result = write_trades_csv(dir.path(), &trades);
        assert!(result.is_ok());

        // 파일이 생성되었는지 확인
        let filepath = result.unwrap();
        assert!(Path::new(&filepath).exists());
    }

    #[test]
    fn test_write_trades_csv_empty() {
        let dir = TempDir::new().unwrap();
        let result = write_trades_csv(dir.path(), &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_timeseries_csv() {
        let dir = TempDir::new().unwrap();
        let records = vec![make_test_timeseries()];
        let result = write_timeseries_csv(dir.path(), &records);
        assert!(result.is_ok());

        let filepath = result.unwrap();
        assert!(Path::new(&filepath).exists());
    }

    #[test]
    fn test_write_timeseries_csv_empty() {
        let dir = TempDir::new().unwrap();
        let result = write_timeseries_csv(dir.path(), &[]);
        assert!(result.is_ok());
    }

    use crate::zscore::sweep::{SweepResult, SweepResultRow};

    fn make_test_sweep_row(entry_z: f64, exit_z: f64, trades: usize) -> SweepResultRow {
        SweepResultRow {
            entry_z,
            exit_z,
            total_trades: trades,
            winning_trades: trades / 2,
            losing_trades: trades - trades / 2,
            liquidated_trades: 0,
            win_rate: if trades > 0 { 0.5 } else { 0.0 },
            total_pnl: Decimal::new(100, 0),
            total_fees: Decimal::new(20, 0),
            net_pnl: Decimal::new(80, 0),
            max_drawdown: Decimal::new(30, 0),
            avg_holding_minutes: 45.0,
            realized_roi_pct: 0.8,
            total_roi_pct: 0.9,
            open_position_count: 0,
            unrealized_pnl: Decimal::new(10, 0),
            profit_factor: 2.5,
            return_max_dd_ratio: 2.67,
        }
    }

    #[test]
    fn test_write_sweep_csv() {
        let dir = TempDir::new().unwrap();
        let sweep_result = SweepResult {
            rows: vec![
                make_test_sweep_row(2.0, 0.5, 10),
                make_test_sweep_row(2.5, 0.5, 5),
            ],
            coin: "BTC".to_string(),
            period_start: Utc::now(),
            period_end: Utc::now(),
            total_capital_usdt: Decimal::new(10000, 0),
        };

        let result = write_sweep_csv(dir.path(), &sweep_result);
        assert!(result.is_ok());

        let filepath = result.unwrap();
        assert!(Path::new(&filepath).exists());

        // 파일 내용 검증: 메타데이터 코멘트 + 헤더 + 2행
        let content = std::fs::read_to_string(&filepath).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        // 코멘트(1줄) + 헤더(1줄) + 데이터(2줄) = 4줄
        assert_eq!(lines.len(), 4);
        assert!(lines[0].starts_with("# coin=BTC"));
        assert!(lines[1].contains("entry_z"));
    }

    #[test]
    fn test_write_sweep_csv_empty() {
        let dir = TempDir::new().unwrap();
        let sweep_result = SweepResult {
            rows: vec![],
            coin: "ETH".to_string(),
            period_start: Utc::now(),
            period_end: Utc::now(),
            total_capital_usdt: Decimal::new(5000, 0),
        };

        let result = write_sweep_csv(dir.path(), &sweep_result);
        assert!(result.is_ok());

        // 파일 존재 확인
        let filepath = result.unwrap();
        assert!(Path::new(&filepath).exists());

        // 코멘트(1줄) + 헤더(1줄) = 2줄
        let content = std::fs::read_to_string(&filepath).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2);
    }
}
