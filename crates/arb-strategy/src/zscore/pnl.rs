//! PnL (손익) 계산.
//!
//! 청산된 포지션의 PnL 집계 및 equity curve 기반 max drawdown 계산을 포함합니다.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use tracing::debug;

/// 청산된 포지션 기록.
#[derive(Debug, Clone)]
pub struct ClosedPosition {
    /// 코인 심볼.
    pub coin: String,
    /// 진입 시간.
    pub entry_time: DateTime<Utc>,
    /// 청산 시간.
    pub exit_time: DateTime<Utc>,
    /// 보유 시간 (분).
    pub holding_minutes: u64,
    /// 포지션 크기 (USDT, 단일 leg 기준).
    pub size_usdt: Decimal,
    /// Upbit 측 PnL (현물 매수 -> 매도 손익).
    pub upbit_pnl: Decimal,
    /// Bybit 측 PnL (선물 short -> 청산 손익).
    pub bybit_pnl: Decimal,
    /// Upbit 측 수수료.
    pub upbit_fees: Decimal,
    /// Bybit 측 수수료.
    pub bybit_fees: Decimal,
    /// 총 수수료 (양 거래소 합산) = upbit_fees + bybit_fees.
    pub total_fees: Decimal,
    /// 순 PnL = upbit_pnl + bybit_pnl - total_fees.
    pub net_pnl: Decimal,
    /// 진입 시 Z-Score.
    pub entry_z_score: f64,
    /// 청산 시 Z-Score.
    pub exit_z_score: f64,
    /// 진입 시 스프레드 (%).
    pub entry_spread_pct: f64,
    /// 청산 시 스프레드 (%).
    pub exit_spread_pct: f64,
    /// 진입 시 USDT/KRW 환율.
    pub entry_usdt_krw: Decimal,
    /// 청산 시 USDT/KRW 환율.
    pub exit_usdt_krw: Decimal,
    /// 강제 청산 여부.
    pub is_liquidated: bool,
}

/// Equity curve에서 max drawdown을 계산합니다 (USDT 절대값).
///
/// equity[0] = 0
/// equity[i] = equity[i-1] + trades[i].net_pnl
/// max_drawdown = max(peak - equity[i]) for all i
pub fn calculate_max_drawdown(trades: &[ClosedPosition]) -> Decimal {
    if trades.is_empty() {
        return Decimal::ZERO;
    }

    let mut equity = Decimal::ZERO;
    let mut peak = Decimal::ZERO;
    let mut max_dd = Decimal::ZERO;

    for trade in trades {
        equity += trade.net_pnl;
        if equity > peak {
            peak = equity;
        }
        let dd = peak - equity;
        if dd > max_dd {
            max_dd = dd;
        }
    }

    // Max drawdown 계산 완료
    debug!(trade_count = trades.len(), max_drawdown = %max_dd, "max drawdown 계산 완료");

    max_dd
}

/// 거래 내역을 일별로 PnL을 집계합니다.
pub fn daily_pnl(trades: &[ClosedPosition]) -> Vec<(NaiveDate, Decimal)> {
    if trades.is_empty() {
        return Vec::new();
    }

    let mut daily: std::collections::BTreeMap<NaiveDate, Decimal> =
        std::collections::BTreeMap::new();

    for trade in trades {
        let date = trade.exit_time.date_naive();
        *daily.entry(date).or_insert(Decimal::ZERO) += trade.net_pnl;
    }

    // 일별 PnL 집계 완료
    debug!(total_days = daily.len(), "일별 PnL 집계 완료");

    daily.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_closed(net_pnl: i64, scale: u32) -> ClosedPosition {
        ClosedPosition {
            coin: "BTC".to_string(),
            entry_time: Utc::now(),
            exit_time: Utc::now(),
            holding_minutes: 30,
            size_usdt: Decimal::new(1000, 0),
            upbit_pnl: Decimal::ZERO,
            bybit_pnl: Decimal::ZERO,
            upbit_fees: Decimal::ZERO,
            bybit_fees: Decimal::ZERO,
            total_fees: Decimal::ZERO,
            net_pnl: Decimal::new(net_pnl, scale),
            entry_z_score: 2.0,
            exit_z_score: 0.5,
            entry_spread_pct: 0.3,
            exit_spread_pct: 0.1,
            entry_usdt_krw: Decimal::new(1380, 0),
            exit_usdt_krw: Decimal::new(1381, 0),
            is_liquidated: false,
        }
    }

    #[test]
    fn test_max_drawdown_empty() {
        assert_eq!(calculate_max_drawdown(&[]), Decimal::ZERO);
    }

    #[test]
    fn test_max_drawdown_all_profit() {
        // +10, +20, +5 -> equity: 10, 30, 35 -> peak always rising, dd=0
        let trades = vec![make_closed(10, 0), make_closed(20, 0), make_closed(5, 0)];
        assert_eq!(calculate_max_drawdown(&trades), Decimal::ZERO);
    }

    #[test]
    fn test_max_drawdown_with_loss() {
        // +10, -5, +3 -> equity: 10, 5, 8
        // peak: 10, 10, 10
        // dd: 0, 5, 2 -> max = 5
        let trades = vec![make_closed(10, 0), make_closed(-5, 0), make_closed(3, 0)];
        assert_eq!(calculate_max_drawdown(&trades), Decimal::new(5, 0));
    }

    #[test]
    fn test_max_drawdown_deep_drawdown() {
        // +10, -15, +2 -> equity: 10, -5, -3
        // peak: 10, 10, 10
        // dd: 0, 15, 13 -> max = 15
        let trades = vec![make_closed(10, 0), make_closed(-15, 0), make_closed(2, 0)];
        assert_eq!(calculate_max_drawdown(&trades), Decimal::new(15, 0));
    }

    #[test]
    fn test_daily_pnl_empty() {
        assert!(daily_pnl(&[]).is_empty());
    }

    #[test]
    fn test_daily_pnl_aggregation() {
        use chrono::TimeZone;
        let day1 = Utc.with_ymd_and_hms(2026, 2, 6, 10, 0, 0).unwrap();
        let day1_later = Utc.with_ymd_and_hms(2026, 2, 6, 15, 0, 0).unwrap();
        let day2 = Utc.with_ymd_and_hms(2026, 2, 7, 10, 0, 0).unwrap();

        let trades = vec![
            ClosedPosition {
                exit_time: day1,
                net_pnl: Decimal::new(10, 0),
                ..make_closed(10, 0)
            },
            ClosedPosition {
                exit_time: day1_later,
                net_pnl: Decimal::new(-3, 0),
                ..make_closed(-3, 0)
            },
            ClosedPosition {
                exit_time: day2,
                net_pnl: Decimal::new(5, 0),
                ..make_closed(5, 0)
            },
        ];

        let daily = daily_pnl(&trades);
        assert_eq!(daily.len(), 2);
        assert_eq!(daily[0].1, Decimal::new(7, 0)); // day1: 10 + (-3) = 7
        assert_eq!(daily[1].1, Decimal::new(5, 0)); // day2: 5
    }
}
