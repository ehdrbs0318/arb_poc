//! 세션 요약 계산 및 출력.
//!
//! `ClosedPosition` 배열로부터 승률, profit factor, Sharpe Ratio 등
//! 핵심 지표를 계산하고 텍스트/JSON 형식으로 출력합니다.

use std::collections::BTreeMap;

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::Serialize;

use crate::zscore::pnl::{ClosedPosition, calculate_max_drawdown, daily_pnl};

/// 모니터링 카운터.
///
/// 세션 중 발생한 각종 이벤트를 집계합니다.
#[derive(Debug, Clone, Default)]
pub struct MonitoringCounters {
    /// 드롭된 틱 수 (처리 지연으로 스킵된 이벤트).
    pub dropped_tick_count: u64,
    /// 오더북 조회 시도 횟수.
    pub orderbook_fetch_count: u64,
    /// 오더북 조회 실패 횟수.
    pub orderbook_fetch_fail_count: u64,
    /// 오래된 캐시 스킵 횟수.
    pub stale_cache_skip_count: u64,
    /// 슬리피지로 인한 진입 거부 횟수.
    pub entry_rejected_slippage_count: u64,
    /// 부분 청산 횟수.
    pub partial_close_count: u64,
    /// 강제 청산 횟수.
    pub forced_liquidation_count: u64,
    /// 최소/최대 주문 조건 미달로 진입 거부된 횟수.
    pub entry_rejected_order_constraint_count: u64,
    /// 라운딩 후 수익성 부족으로 진입 거부된 횟수.
    pub entry_rejected_rounding_pnl_count: u64,
    /// InstrumentInfo 미존재로 라운딩 없이 청산 진행한 횟수.
    pub fallback_no_rounding_count: u64,
    /// 잔여 qty < min_order_qty로 전량 전환 시 safe volume 초과한 청산 횟수.
    pub safe_volume_exceeded_close_count: u64,
    /// stddev 필터로 실제 분류 변경된 코인 수 (초기 선택 + 재선택 + 런타임).
    pub coin_rejected_spread_stddev_count: u64,
    /// regime change 감지 횟수 (cooldown으로 무시된 것 제외).
    pub regime_change_detected_count: u64,
    /// cooldown에 의해 억제된 regime change 횟수.
    pub regime_change_suppressed_by_cooldown_count: u64,
    /// 최소 포지션 크기 미달로 진입 거부된 횟수.
    pub entry_rejected_min_position_count: u64,
    /// 최소 기대 수익률 미달로 진입 거부된 횟수.
    pub entry_rejected_min_roi_count: u64,
    /// 잔고 스냅샷 try_send 실패 (드롭) 수.
    pub balance_snapshot_dropped: u64,
}

/// 일별 PnL 기록.
#[derive(Debug, Clone, Serialize)]
pub struct DailyPnl {
    /// 날짜 (UTC 기준).
    pub date: NaiveDate,
    /// 해당 일의 순 PnL (USDT).
    #[serde(with = "rust_decimal::serde::str")]
    pub pnl: Decimal,
}

/// 코인별 PnL 집계.
#[derive(Debug, Clone, Serialize)]
pub struct CoinPnl {
    /// 코인 심볼.
    pub coin: String,
    /// 거래 수.
    pub trades: usize,
    /// 순 PnL 합계 (USDT).
    #[serde(with = "rust_decimal::serde::str")]
    pub net_pnl: Decimal,
    /// 승률 (%).
    pub win_rate: f64,
}

/// 세션 요약 지표.
///
/// 세션 종료 시 `ClosedPosition` 배열로부터 계산되며,
/// `summary.json` 및 `summary.txt`로 저장됩니다.
#[derive(Debug, Clone, Serialize)]
pub struct SessionSummary {
    /// 세션 시작 시각 (UTC).
    pub session_start: String,
    /// 세션 종료 시각 (UTC).
    pub session_end: String,
    /// 총 실행 시간 (분).
    pub duration_minutes: i64,
    /// 모니터링 코인 목록.
    pub coins: Vec<String>,
    /// 시작 시 USD/KRW 환율.
    pub usd_krw_start: f64,
    /// 종료 시 USD/KRW 환율.
    pub usd_krw_end: f64,
    /// 총 거래 수.
    pub total_trades: usize,
    /// 승리 거래 수 (net_pnl > 0).
    pub winning_trades: usize,
    /// 패배 거래 수 (net_pnl <= 0).
    pub losing_trades: usize,
    /// 승률 (%).
    pub win_rate: f64,
    /// 순 PnL 합계 (USDT).
    #[serde(with = "rust_decimal::serde::str")]
    pub total_net_pnl: Decimal,
    /// 최대 낙폭 (USDT).
    #[serde(with = "rust_decimal::serde::str")]
    pub max_drawdown: Decimal,
    /// 총 수신 이벤트 수.
    pub total_events: u64,
    /// Profit Factor (총이익 / 총손실). 총손실이 0이면 9999.99.
    pub profit_factor: f64,
    /// 평균 보유 시간 (분).
    pub avg_holding_minutes: f64,
    /// 일별 PnL 배열.
    pub daily_pnl: Vec<DailyPnl>,
    /// 코인별 PnL 집계.
    pub coin_pnl: Vec<CoinPnl>,
    /// 일별 PnL 기반 Sharpe Ratio (무위험이자율 0 가정).
    pub sharpe_ratio: f64,
    /// 총 수수료 합계 (USDT).
    #[serde(with = "rust_decimal::serde::str")]
    pub total_fees: Decimal,
    /// 강제 청산 횟수.
    pub liquidation_count: usize,
    /// 드롭된 틱 수.
    pub dropped_tick_count: u64,
    /// 오더북 조회 시도 횟수.
    pub orderbook_fetch_count: u64,
    /// 오더북 조회 실패 횟수.
    pub orderbook_fetch_fail_count: u64,
    /// 오래된 캐시 스킵 횟수.
    pub stale_cache_skip_count: u64,
    /// 슬리피지로 인한 진입 거부 횟수.
    pub entry_rejected_slippage_count: u64,
    /// 부분 청산 횟수.
    pub partial_close_count: u64,
    /// 강제 청산 (카운터 기반) 횟수.
    pub forced_liquidation_count: u64,
    /// 주문 조건 미달 진입 거부 횟수.
    pub entry_rejected_order_constraint_count: u64,
    /// 라운딩 후 수익성 부족 진입 거부 횟수.
    pub entry_rejected_rounding_pnl_count: u64,
    /// 라운딩 미적용 청산 (InstrumentInfo 없음) 횟수.
    pub fallback_no_rounding_count: u64,
    /// safe volume 초과 전량 청산 횟수.
    pub safe_volume_exceeded_close_count: u64,
    /// stddev 필터로 분류 변경된 코인 수.
    pub coin_rejected_spread_stddev_count: u64,
    /// regime change 감지 횟수.
    pub regime_change_detected_count: u64,
    /// cooldown으로 억제된 regime change 횟수.
    pub regime_change_suppressed_by_cooldown_count: u64,
    /// 최소 포지션 크기 미달 진입 거부 횟수.
    pub entry_rejected_min_position_count: u64,
    /// 최소 기대 수익률 미달 진입 거부 횟수.
    pub entry_rejected_min_roi_count: u64,
    /// 잔고 스냅샷 드롭 횟수.
    pub balance_snapshot_dropped: u64,
}

impl SessionSummary {
    /// `ClosedPosition` 배열로부터 세션 요약 지표를 계산합니다.
    ///
    /// # 인자
    ///
    /// * `trades` - 청산된 포지션 배열
    /// * `session_start` - 세션 시작 시각
    /// * `session_end` - 세션 종료 시각
    /// * `coins` - 모니터링 코인 목록
    /// * `usd_krw_start` - 시작 시 USD/KRW 환율
    /// * `usd_krw_end` - 종료 시 USD/KRW 환율
    /// * `total_events` - 총 수신 이벤트 수
    /// * `counters` - 모니터링 카운터
    #[allow(clippy::too_many_arguments)]
    pub fn calculate(
        trades: &[ClosedPosition],
        session_start: DateTime<Utc>,
        session_end: DateTime<Utc>,
        coins: &[String],
        usd_krw_start: f64,
        usd_krw_end: f64,
        total_events: u64,
        counters: &MonitoringCounters,
    ) -> Self {
        let duration_minutes = (session_end - session_start).num_minutes();
        let total_trades = trades.len();

        // 승패 분류
        let winning_trades = trades.iter().filter(|t| t.net_pnl > Decimal::ZERO).count();
        let losing_trades = total_trades - winning_trades;

        // 승률 계산
        let win_rate = if total_trades > 0 {
            (winning_trades as f64 / total_trades as f64) * 100.0
        } else {
            0.0
        };

        // 순 PnL 합계
        let total_net_pnl: Decimal = trades.iter().map(|t| t.net_pnl).sum();

        // Max drawdown
        let max_drawdown = calculate_max_drawdown(trades);

        // Profit Factor
        let total_profit: Decimal = trades
            .iter()
            .filter(|t| t.net_pnl > Decimal::ZERO)
            .map(|t| t.net_pnl)
            .sum();
        let total_loss: Decimal = trades
            .iter()
            .filter(|t| t.net_pnl <= Decimal::ZERO)
            .map(|t| t.net_pnl.abs())
            .sum();

        // JSON 직렬화 시 f64::INFINITY는 null로 변환되므로 유한 상한값 사용
        let profit_factor = if total_loss == Decimal::ZERO {
            if total_profit > Decimal::ZERO {
                9999.99
            } else {
                0.0
            }
        } else {
            // Decimal -> f64 변환
            let profit_f64 = decimal_to_f64(total_profit);
            let loss_f64 = decimal_to_f64(total_loss);
            profit_f64 / loss_f64
        };

        // 평균 보유 시간
        let avg_holding_minutes = if total_trades > 0 {
            let total_minutes: u64 = trades.iter().map(|t| t.holding_minutes).sum();
            total_minutes as f64 / total_trades as f64
        } else {
            0.0
        };

        // 일별 PnL
        let daily = daily_pnl(trades);
        let daily_pnl_vec: Vec<DailyPnl> = daily
            .iter()
            .map(|(date, pnl)| DailyPnl {
                date: *date,
                pnl: *pnl,
            })
            .collect();

        // Sharpe Ratio (일별 PnL 기반, 무위험이자율 0)
        let sharpe_ratio = calculate_sharpe_ratio(&daily);

        // 코인별 PnL 집계
        let coin_pnl_vec = calculate_coin_pnl(trades);

        // 총 수수료
        let total_fees: Decimal = trades.iter().map(|t| t.total_fees).sum();

        // 강제 청산 횟수
        let liquidation_count = trades.iter().filter(|t| t.is_liquidated).count();

        Self {
            session_start: session_start.to_rfc3339(),
            session_end: session_end.to_rfc3339(),
            duration_minutes,
            coins: coins.to_vec(),
            usd_krw_start,
            usd_krw_end,
            total_trades,
            winning_trades,
            losing_trades,
            win_rate,
            total_net_pnl,
            max_drawdown,
            total_events,
            profit_factor,
            avg_holding_minutes,
            daily_pnl: daily_pnl_vec,
            coin_pnl: coin_pnl_vec,
            sharpe_ratio,
            total_fees,
            liquidation_count,
            dropped_tick_count: counters.dropped_tick_count,
            orderbook_fetch_count: counters.orderbook_fetch_count,
            orderbook_fetch_fail_count: counters.orderbook_fetch_fail_count,
            stale_cache_skip_count: counters.stale_cache_skip_count,
            entry_rejected_slippage_count: counters.entry_rejected_slippage_count,
            partial_close_count: counters.partial_close_count,
            forced_liquidation_count: counters.forced_liquidation_count,
            entry_rejected_order_constraint_count: counters.entry_rejected_order_constraint_count,
            entry_rejected_rounding_pnl_count: counters.entry_rejected_rounding_pnl_count,
            fallback_no_rounding_count: counters.fallback_no_rounding_count,
            safe_volume_exceeded_close_count: counters.safe_volume_exceeded_close_count,
            coin_rejected_spread_stddev_count: counters.coin_rejected_spread_stddev_count,
            regime_change_detected_count: counters.regime_change_detected_count,
            regime_change_suppressed_by_cooldown_count: counters
                .regime_change_suppressed_by_cooldown_count,
            entry_rejected_min_position_count: counters.entry_rejected_min_position_count,
            entry_rejected_min_roi_count: counters.entry_rejected_min_roi_count,
            balance_snapshot_dropped: counters.balance_snapshot_dropped,
        }
    }

    /// 사람이 읽기 쉬운 텍스트 형식으로 요약을 출력합니다.
    pub fn to_text(&self) -> String {
        let mut s = String::new();

        s.push_str("=== 세션 요약 ===\n");
        s.push_str(&format!(
            "기간: {} ~ {} ({}분)\n",
            self.session_start, self.session_end, self.duration_minutes
        ));
        s.push_str(&format!("코인: {}\n", self.coins.join(", ")));
        s.push_str(&format!(
            "환율: {:.2} -> {:.2}\n",
            self.usd_krw_start, self.usd_krw_end
        ));

        s.push('\n');
        s.push_str(&format!(
            "거래: {}건 (승 {} / 패 {}, 승률 {:.1}%)\n",
            self.total_trades, self.winning_trades, self.losing_trades, self.win_rate
        ));
        s.push_str(&format!(
            "순 PnL: {} USDT\n",
            format_decimal_signed(self.total_net_pnl)
        ));
        s.push_str(&format!(
            "Max DD: {} USDT\n",
            format_decimal_signed(-self.max_drawdown)
        ));
        s.push_str(&format!("Profit Factor: {:.2}\n", self.profit_factor));
        s.push_str(&format!("Sharpe Ratio: {:.2}\n", self.sharpe_ratio));

        // 코인별 집계
        if !self.coin_pnl.is_empty() {
            s.push_str("\n코인별:\n");
            for cp in &self.coin_pnl {
                s.push_str(&format!(
                    "  {}: {}건, {} USDT (승률 {:.1}%)\n",
                    cp.coin,
                    cp.trades,
                    format_decimal_signed(cp.net_pnl),
                    cp.win_rate
                ));
            }
        }

        // 일별 PnL
        if !self.daily_pnl.is_empty() {
            s.push_str("\n일별 PnL:\n");
            for dp in &self.daily_pnl {
                s.push_str(&format!(
                    "  {}: {} USDT\n",
                    dp.date,
                    format_decimal_signed(dp.pnl)
                ));
            }
        }

        s.push_str(&format!(
            "\n총 수수료: {} USDT\n",
            format_decimal_signed(self.total_fees)
        ));
        s.push_str(&format!("강제 청산: {}건\n", self.liquidation_count));
        s.push_str(&format!(
            "총 이벤트: {}건\n",
            format_number(self.total_events)
        ));

        // 모니터링 카운터
        s.push_str("\n--- 모니터링 카운터 ---\n");
        s.push_str(&format!(
            "드롭 틱: {}건\n",
            format_number(self.dropped_tick_count)
        ));
        s.push_str(&format!(
            "오더북 조회: {}건 (실패 {}건)\n",
            format_number(self.orderbook_fetch_count),
            format_number(self.orderbook_fetch_fail_count)
        ));
        s.push_str(&format!(
            "캐시 만료 스킵: {}건\n",
            format_number(self.stale_cache_skip_count)
        ));
        s.push_str(&format!(
            "슬리피지 진입 거부: {}건\n",
            format_number(self.entry_rejected_slippage_count)
        ));
        s.push_str(&format!(
            "부분 청산: {}건\n",
            format_number(self.partial_close_count)
        ));
        s.push_str(&format!(
            "강제 청산 (카운터): {}건\n",
            format_number(self.forced_liquidation_count)
        ));
        s.push_str(&format!(
            "주문 조건 미달 진입 거부: {}건\n",
            format_number(self.entry_rejected_order_constraint_count)
        ));
        s.push_str(&format!(
            "라운딩 PnL 진입 거부: {}건\n",
            format_number(self.entry_rejected_rounding_pnl_count)
        ));
        s.push_str(&format!(
            "라운딩 미적용 청산: {}건\n",
            format_number(self.fallback_no_rounding_count)
        ));
        s.push_str(&format!(
            "safe volume 초과 청산: {}건\n",
            format_number(self.safe_volume_exceeded_close_count)
        ));
        s.push_str(&format!(
            "stddev 필터 제외: {}건\n",
            format_number(self.coin_rejected_spread_stddev_count)
        ));
        s.push_str(&format!(
            "regime change 감지: {}건\n",
            format_number(self.regime_change_detected_count)
        ));
        s.push_str(&format!(
            "regime change 억제 (cooldown): {}건\n",
            format_number(self.regime_change_suppressed_by_cooldown_count)
        ));
        s.push_str(&format!(
            "최소 포지션 미달 진입 거부: {}건\n",
            format_number(self.entry_rejected_min_position_count)
        ));
        s.push_str(&format!(
            "최소 ROI 미달 진입 거부: {}건\n",
            format_number(self.entry_rejected_min_roi_count)
        ));
        s.push_str(&format!(
            "잔고 스냅샷 드롭: {}건\n",
            format_number(self.balance_snapshot_dropped)
        ));

        s
    }
}

/// Sharpe Ratio 계산 (일별 PnL 기반, 무위험이자율 0 가정).
///
/// std가 0이면 0.0을 반환합니다.
fn calculate_sharpe_ratio(daily: &[(NaiveDate, Decimal)]) -> f64 {
    if daily.is_empty() {
        return 0.0;
    }

    let n = daily.len() as f64;
    let pnl_values: Vec<f64> = daily.iter().map(|(_, pnl)| decimal_to_f64(*pnl)).collect();

    let mean = pnl_values.iter().sum::<f64>() / n;
    let variance = pnl_values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    let std = variance.sqrt();

    if std == 0.0 {
        return 0.0;
    }

    mean / std
}

/// 코인별 PnL 집계.
fn calculate_coin_pnl(trades: &[ClosedPosition]) -> Vec<CoinPnl> {
    let mut grouped: BTreeMap<String, Vec<&ClosedPosition>> = BTreeMap::new();
    for trade in trades {
        grouped.entry(trade.coin.clone()).or_default().push(trade);
    }

    grouped
        .into_iter()
        .map(|(coin, coin_trades)| {
            let trades_count = coin_trades.len();
            let net_pnl: Decimal = coin_trades.iter().map(|t| t.net_pnl).sum();
            let wins = coin_trades
                .iter()
                .filter(|t| t.net_pnl > Decimal::ZERO)
                .count();
            let win_rate = if trades_count > 0 {
                (wins as f64 / trades_count as f64) * 100.0
            } else {
                0.0
            };

            CoinPnl {
                coin,
                trades: trades_count,
                net_pnl,
                win_rate,
            }
        })
        .collect()
}

/// `Decimal`을 `f64`로 변환합니다.
fn decimal_to_f64(d: Decimal) -> f64 {
    use rust_decimal::prelude::ToPrimitive;
    d.to_f64().unwrap_or(0.0)
}

/// `Decimal` 값을 부호 포함 문자열로 포맷합니다.
fn format_decimal_signed(d: Decimal) -> String {
    if d > Decimal::ZERO {
        format!("+{d}")
    } else {
        format!("{d}")
    }
}

/// 천 단위 구분 기호를 포함한 숫자 포맷팅.
fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use rust_decimal::Decimal;

    /// 테스트용 `ClosedPosition` 생성 헬퍼.
    fn make_trade(
        coin: &str,
        net_pnl: Decimal,
        total_fees: Decimal,
        holding_minutes: u64,
        exit_time: DateTime<Utc>,
        is_liquidated: bool,
    ) -> ClosedPosition {
        ClosedPosition {
            id: 0,
            coin: coin.to_string(),
            entry_time: exit_time - chrono::Duration::minutes(holding_minutes as i64),
            exit_time,
            holding_minutes,
            qty: Decimal::new(10, 3), // 0.010
            size_usdt: Decimal::new(1000, 0),
            upbit_entry_price: Decimal::new(95_000, 0),
            bybit_entry_price: Decimal::new(95_100, 0),
            upbit_exit_price: Decimal::new(95_050, 0),
            bybit_exit_price: Decimal::new(95_080, 0),
            upbit_pnl: Decimal::ZERO,
            bybit_pnl: Decimal::ZERO,
            upbit_fees: Decimal::ZERO,
            bybit_fees: Decimal::ZERO,
            total_fees,
            net_pnl,
            entry_z_score: 2.0,
            exit_z_score: 0.5,
            entry_spread_pct: 0.3,
            exit_spread_pct: 0.1,
            entry_usd_krw: 1380.0,
            exit_usd_krw: 1381.0,
            is_liquidated,
            actual_upbit_fee: None,
            actual_bybit_fee: None,
            funding_fee: None,
            adjustment_cost: None,
        }
    }

    #[test]
    fn test_summary_empty_trades() {
        let start = Utc.with_ymd_and_hms(2026, 2, 9, 20, 30, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 2, 10, 8, 30, 0).unwrap();
        let coins = vec!["BTC".to_string()];

        let summary = SessionSummary::calculate(
            &[],
            start,
            end,
            &coins,
            1463.33,
            1465.20,
            54320,
            &MonitoringCounters::default(),
        );

        assert_eq!(summary.total_trades, 0);
        assert_eq!(summary.winning_trades, 0);
        assert_eq!(summary.losing_trades, 0);
        assert_eq!(summary.win_rate, 0.0);
        assert_eq!(summary.total_net_pnl, Decimal::ZERO);
        assert_eq!(summary.max_drawdown, Decimal::ZERO);
        assert_eq!(summary.profit_factor, 0.0);
        assert_eq!(summary.avg_holding_minutes, 0.0);
        assert_eq!(summary.sharpe_ratio, 0.0);
        assert_eq!(summary.total_fees, Decimal::ZERO);
        assert_eq!(summary.liquidation_count, 0);
        assert!(summary.daily_pnl.is_empty());
        assert!(summary.coin_pnl.is_empty());
        assert_eq!(summary.duration_minutes, 720);
    }

    #[test]
    fn test_summary_with_trades() {
        let start = Utc.with_ymd_and_hms(2026, 2, 9, 20, 30, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 2, 10, 8, 30, 0).unwrap();
        let exit1 = Utc.with_ymd_and_hms(2026, 2, 9, 22, 0, 0).unwrap();
        let exit2 = Utc.with_ymd_and_hms(2026, 2, 10, 2, 0, 0).unwrap();
        let exit3 = Utc.with_ymd_and_hms(2026, 2, 10, 5, 0, 0).unwrap();
        let coins = vec!["BTC".to_string()];

        let trades = vec![
            // 승리: +10 USDT, 수수료 2
            make_trade(
                "BTC",
                Decimal::new(10, 0),
                Decimal::new(2, 0),
                30,
                exit1,
                false,
            ),
            // 패배: -3 USDT, 수수료 1
            make_trade(
                "BTC",
                Decimal::new(-3, 0),
                Decimal::new(1, 0),
                45,
                exit2,
                false,
            ),
            // 승리: +5 USDT, 수수료 1.5
            make_trade(
                "BTC",
                Decimal::new(5, 0),
                Decimal::new(15, 1),
                60,
                exit3,
                false,
            ),
        ];

        let summary = SessionSummary::calculate(
            &trades,
            start,
            end,
            &coins,
            1463.33,
            1465.20,
            10000,
            &MonitoringCounters::default(),
        );

        // net_pnl 합산: 10 + (-3) + 5 = 12
        assert_eq!(summary.total_net_pnl, Decimal::new(12, 0));
        assert_eq!(summary.total_trades, 3);
        assert_eq!(summary.winning_trades, 2);
        assert_eq!(summary.losing_trades, 1);

        // 승률: 2/3 * 100 = 66.66...%
        let expected_win_rate = 2.0 / 3.0 * 100.0;
        assert!((summary.win_rate - expected_win_rate).abs() < 0.01);

        // profit_factor: 총이익(15) / 총손실(3) = 5.0
        assert!((summary.profit_factor - 5.0).abs() < 0.01);

        // 총 수수료: 2 + 1 + 1.5 = 4.5
        assert_eq!(summary.total_fees, Decimal::new(45, 1));

        // 평균 보유 시간: (30 + 45 + 60) / 3 = 45.0
        assert!((summary.avg_holding_minutes - 45.0).abs() < 0.01);
    }

    #[test]
    fn test_summary_to_text() {
        let start = Utc.with_ymd_and_hms(2026, 2, 9, 20, 30, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 2, 10, 8, 30, 0).unwrap();
        let exit1 = Utc.with_ymd_and_hms(2026, 2, 9, 22, 0, 0).unwrap();
        let coins = vec!["BTC".to_string(), "XRP".to_string()];

        let trades = vec![make_trade(
            "BTC",
            Decimal::new(10, 0),
            Decimal::new(2, 0),
            30,
            exit1,
            false,
        )];

        let summary = SessionSummary::calculate(
            &trades,
            start,
            end,
            &coins,
            1463.33,
            1465.20,
            54320,
            &MonitoringCounters::default(),
        );
        let text = summary.to_text();

        // 핵심 문자열 포함 확인
        assert!(text.contains("세션 요약"));
        assert!(text.contains("720분"));
        assert!(text.contains("BTC"));
        assert!(text.contains("XRP"));
        assert!(text.contains("승률"));
        assert!(text.contains("순 PnL"));
        assert!(text.contains("Max DD"));
        assert!(text.contains("Profit Factor"));
        assert!(text.contains("Sharpe Ratio"));
        assert!(text.contains("총 수수료"));
        assert!(text.contains("강제 청산"));
        assert!(text.contains("총 이벤트"));
        assert!(text.contains("54,320"));
    }

    #[test]
    fn test_daily_pnl_grouping() {
        let start = Utc.with_ymd_and_hms(2026, 2, 9, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 2, 11, 0, 0, 0).unwrap();
        let coins = vec!["BTC".to_string()];

        // 2월 9일: 2건, 2월 10일: 1건
        let exit_day1_a = Utc.with_ymd_and_hms(2026, 2, 9, 10, 0, 0).unwrap();
        let exit_day1_b = Utc.with_ymd_and_hms(2026, 2, 9, 15, 0, 0).unwrap();
        let exit_day2 = Utc.with_ymd_and_hms(2026, 2, 10, 12, 0, 0).unwrap();

        let trades = vec![
            make_trade(
                "BTC",
                Decimal::new(10, 0),
                Decimal::ZERO,
                30,
                exit_day1_a,
                false,
            ),
            make_trade(
                "BTC",
                Decimal::new(-3, 0),
                Decimal::ZERO,
                30,
                exit_day1_b,
                false,
            ),
            make_trade(
                "BTC",
                Decimal::new(5, 0),
                Decimal::ZERO,
                30,
                exit_day2,
                false,
            ),
        ];

        let summary = SessionSummary::calculate(
            &trades,
            start,
            end,
            &coins,
            1400.0,
            1410.0,
            0,
            &MonitoringCounters::default(),
        );

        assert_eq!(summary.daily_pnl.len(), 2);
        // 2월 9일: 10 + (-3) = 7
        assert_eq!(
            summary.daily_pnl[0].date,
            NaiveDate::from_ymd_opt(2026, 2, 9).unwrap()
        );
        assert_eq!(summary.daily_pnl[0].pnl, Decimal::new(7, 0));
        // 2월 10일: 5
        assert_eq!(
            summary.daily_pnl[1].date,
            NaiveDate::from_ymd_opt(2026, 2, 10).unwrap()
        );
        assert_eq!(summary.daily_pnl[1].pnl, Decimal::new(5, 0));
    }

    #[test]
    fn test_coin_pnl_grouping() {
        let start = Utc.with_ymd_and_hms(2026, 2, 9, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 2, 10, 0, 0, 0).unwrap();
        let coins = vec!["BTC".to_string(), "XRP".to_string()];

        let exit = Utc.with_ymd_and_hms(2026, 2, 9, 12, 0, 0).unwrap();

        let trades = vec![
            make_trade("BTC", Decimal::new(10, 0), Decimal::ZERO, 30, exit, false),
            make_trade("BTC", Decimal::new(-5, 0), Decimal::ZERO, 30, exit, false),
            make_trade("XRP", Decimal::new(3, 0), Decimal::ZERO, 30, exit, false),
        ];

        let summary = SessionSummary::calculate(
            &trades,
            start,
            end,
            &coins,
            1400.0,
            1410.0,
            0,
            &MonitoringCounters::default(),
        );

        assert_eq!(summary.coin_pnl.len(), 2);

        // BTreeMap 순서: BTC -> XRP
        let btc = &summary.coin_pnl[0];
        assert_eq!(btc.coin, "BTC");
        assert_eq!(btc.trades, 2);
        assert_eq!(btc.net_pnl, Decimal::new(5, 0)); // 10 + (-5) = 5
        assert!((btc.win_rate - 50.0).abs() < 0.01); // 1/2 = 50%

        let xrp = &summary.coin_pnl[1];
        assert_eq!(xrp.coin, "XRP");
        assert_eq!(xrp.trades, 1);
        assert_eq!(xrp.net_pnl, Decimal::new(3, 0));
        assert!((xrp.win_rate - 100.0).abs() < 0.01); // 1/1 = 100%
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(123), "123");
        assert_eq!(format_number(1234), "1,234");
        assert_eq!(format_number(54320), "54,320");
        assert_eq!(format_number(1000000), "1,000,000");
    }

    #[test]
    fn test_sharpe_ratio_single_day() {
        // 일별 PnL이 1건이면 std=0 -> sharpe=0.0
        let daily = vec![(
            NaiveDate::from_ymd_opt(2026, 2, 9).unwrap(),
            Decimal::new(10, 0),
        )];
        assert_eq!(calculate_sharpe_ratio(&daily), 0.0);
    }

    #[test]
    fn test_profit_factor_no_loss() {
        // 손실이 0이면 profit_factor = 9999.99 (JSON 직렬화 가능한 상한값)
        let start = Utc.with_ymd_and_hms(2026, 2, 9, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 2, 10, 0, 0, 0).unwrap();
        let exit = Utc.with_ymd_and_hms(2026, 2, 9, 12, 0, 0).unwrap();

        let trades = vec![make_trade(
            "BTC",
            Decimal::new(10, 0),
            Decimal::ZERO,
            30,
            exit,
            false,
        )];

        let summary = SessionSummary::calculate(
            &trades,
            start,
            end,
            &["BTC".to_string()],
            1400.0,
            1410.0,
            0,
            &MonitoringCounters::default(),
        );

        assert!((summary.profit_factor - 9999.99).abs() < 0.01);
        // JSON 직렬화 시 null이 아닌 숫자로 출력 확인
        let json = serde_json::to_string(&summary.profit_factor).unwrap();
        assert!(json.contains("9999.99"));
    }

    #[test]
    fn test_liquidation_count() {
        let start = Utc.with_ymd_and_hms(2026, 2, 9, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2026, 2, 10, 0, 0, 0).unwrap();
        let exit = Utc.with_ymd_and_hms(2026, 2, 9, 12, 0, 0).unwrap();

        let trades = vec![
            make_trade("BTC", Decimal::new(-5, 0), Decimal::ZERO, 30, exit, true),
            make_trade("BTC", Decimal::new(10, 0), Decimal::ZERO, 30, exit, false),
            make_trade("XRP", Decimal::new(-3, 0), Decimal::ZERO, 30, exit, true),
        ];

        let summary = SessionSummary::calculate(
            &trades,
            start,
            end,
            &["BTC".to_string()],
            1400.0,
            1410.0,
            0,
            &MonitoringCounters::default(),
        );

        assert_eq!(summary.liquidation_count, 2);
    }
}
