//! 스프레드 계산 엔진.
//!
//! 3-way 입력(Upbit coin/KRW, USDT/KRW, Bybit coin/USDT)을 동기화하여
//! 상대 스프레드(%)를 계산하고 롤링 윈도우에 유지합니다.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use tracing::{trace, warn};

use crate::common::candle_window::CandleWindow;
use crate::common::convert::decimal_to_f64;
use crate::error::StrategyError;

/// 연속 누락 감지 임계값 (분).
const CONSECUTIVE_MISSING_WARN_THRESHOLD: usize = 5;

/// 개별 소스의 forward-fill 상태를 추적합니다.
#[derive(Debug, Clone)]
struct ForwardFillState {
    /// 직전 유효 가격 (forward-fill용).
    last_value: Option<Decimal>,
    /// 연속 누락 횟수.
    consecutive_missing: usize,
}

impl ForwardFillState {
    fn new() -> Self {
        Self {
            last_value: None,
            consecutive_missing: 0,
        }
    }

    /// 값을 업데이트하고 forward-fill을 적용합니다.
    ///
    /// `Some(value)`이면 값을 갱신하고 연속 누락 카운터를 리셋합니다.
    /// `None`이면 직전 값을 유지하고 연속 누락 카운터를 증가시킵니다.
    fn update(
        &mut self,
        value: Option<Decimal>,
        source: &str,
        timestamp: DateTime<Utc>,
    ) -> Option<Decimal> {
        match value {
            Some(v) => {
                self.last_value = Some(v);
                self.consecutive_missing = 0;
                Some(v)
            }
            None => {
                self.consecutive_missing += 1;
                if self.consecutive_missing >= CONSECUTIVE_MISSING_WARN_THRESHOLD {
                    warn!(
                        "연속 {}분 캔들 누락: {} at {}",
                        self.consecutive_missing, source, timestamp
                    );
                }
                self.last_value
            }
        }
    }
}

/// 스프레드 계산기.
///
/// 코인별로 Upbit(KRW), USDT/KRW, Bybit(USDT) 가격을 동기화하여
/// 상대 스프레드(%)를 계산합니다.
#[derive(Debug, Clone)]
pub struct SpreadCalculator {
    /// 코인별 Upbit 캔들 윈도우 (coin/KRW close 가격).
    upbit_coin_windows: HashMap<String, CandleWindow>,
    /// USDT/KRW 캔들 윈도우 (Upbit KRW-USDT close 가격).
    usdt_krw_window: CandleWindow,
    /// 코인별 Bybit 캔들 윈도우 (coinUSDT linear close 가격).
    bybit_windows: HashMap<String, CandleWindow>,
    /// 코인별 스프레드(%) 윈도우 - Z-Score 계산의 입력.
    spread_pct_windows: HashMap<String, CandleWindow>,
    /// 코인별 Upbit forward-fill 상태.
    upbit_ff: HashMap<String, ForwardFillState>,
    /// USDT/KRW forward-fill 상태.
    usdt_krw_ff: ForwardFillState,
    /// 코인별 Bybit forward-fill 상태.
    bybit_ff: HashMap<String, ForwardFillState>,
    /// 윈도우 크기.
    window_size: usize,
}

impl SpreadCalculator {
    /// 새 SpreadCalculator를 생성합니다.
    pub fn new(coins: &[String], window_size: usize) -> Self {
        let mut upbit_coin_windows = HashMap::new();
        let mut bybit_windows = HashMap::new();
        let mut spread_pct_windows = HashMap::new();
        let mut upbit_ff = HashMap::new();
        let mut bybit_ff = HashMap::new();

        for coin in coins {
            upbit_coin_windows.insert(coin.clone(), CandleWindow::new(window_size));
            bybit_windows.insert(coin.clone(), CandleWindow::new(window_size));
            spread_pct_windows.insert(coin.clone(), CandleWindow::new(window_size));
            upbit_ff.insert(coin.clone(), ForwardFillState::new());
            bybit_ff.insert(coin.clone(), ForwardFillState::new());
        }

        Self {
            upbit_coin_windows,
            usdt_krw_window: CandleWindow::new(window_size),
            bybit_windows,
            spread_pct_windows,
            upbit_ff,
            usdt_krw_ff: ForwardFillState::new(),
            bybit_ff,
            window_size,
        }
    }

    /// 특정 코인의 캔들 데이터를 업데이트하고 spread_pct를 재계산합니다.
    ///
    /// 각 입력은 `Option` - `None`이면 forward-fill (직전 값 유지).
    /// 3개 입력이 모두 준비되었을 때만 spread_pct를 계산하고 윈도우에 push합니다.
    pub fn update(
        &mut self,
        coin: &str,
        timestamp: DateTime<Utc>,
        upbit_coin: Option<Decimal>,
        usdt_krw: Option<Decimal>,
        bybit: Option<Decimal>,
    ) -> Result<(), StrategyError> {
        // forward-fill 적용
        let upbit_ff_state = self
            .upbit_ff
            .get_mut(coin)
            .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?;
        let upbit_val = upbit_ff_state.update(upbit_coin, &format!("upbit_{coin}"), timestamp);

        let usdt_krw_val = self.usdt_krw_ff.update(usdt_krw, "usdt_krw", timestamp);

        let bybit_ff_state = self
            .bybit_ff
            .get_mut(coin)
            .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?;
        let bybit_val = bybit_ff_state.update(bybit, &format!("bybit_{coin}"), timestamp);

        // 3개 모두 유효한 경우에만 스프레드 계산
        if let (Some(upbit_krw_close), Some(usdt_krw_close), Some(bybit_close)) =
            (upbit_val, usdt_krw_val, bybit_val)
        {
            // 0 나누기 방지
            if usdt_krw_close.is_zero() {
                warn!(coin = coin, "USDT/KRW 가격 0 감지");
                return Err(StrategyError::DataAlignment(
                    "USDT/KRW price is zero".to_string(),
                ));
            }

            // Upbit USDT 환산가
            let upbit_usdt_f64 = decimal_to_f64(upbit_krw_close / usdt_krw_close)?;
            let bybit_f64 = decimal_to_f64(bybit_close)?;

            // 0 나누기 방지
            if upbit_usdt_f64 == 0.0 {
                warn!(coin = coin, "Upbit USDT 환산가 0 감지");
                return Err(StrategyError::DataAlignment(
                    "Upbit USDT price is zero".to_string(),
                ));
            }

            // 상대 스프레드 (%)
            let spread_pct = (bybit_f64 - upbit_usdt_f64) / upbit_usdt_f64 * 100.0;

            trace!(
                coin = coin,
                upbit_usdt = upbit_usdt_f64,
                bybit = bybit_f64,
                spread_pct = spread_pct,
                "스프레드 계산 완료"
            );

            // 각 윈도우에 push
            self.upbit_coin_windows
                .get_mut(coin)
                .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?
                .push(upbit_usdt_f64);

            self.usdt_krw_window.push(decimal_to_f64(usdt_krw_close)?);

            self.bybit_windows
                .get_mut(coin)
                .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?
                .push(bybit_f64);

            self.spread_pct_windows
                .get_mut(coin)
                .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?
                .push(spread_pct);
        } else {
            // 3개 입력 중 일부 누락으로 스프레드 미계산
            trace!(
                coin = coin,
                upbit_present = upbit_val.is_some(),
                usdt_krw_present = usdt_krw_val.is_some(),
                bybit_present = bybit_val.is_some(),
                "스프레드 미계산: 부분 데이터 누락"
            );
        }

        Ok(())
    }

    /// 특정 코인의 스프레드(%) 윈도우에 대한 참조를 반환합니다.
    pub fn spread_window(&self, coin: &str) -> Option<&CandleWindow> {
        self.spread_pct_windows.get(coin)
    }

    /// 특정 코인의 Upbit USDT 환산 가격 윈도우에 대한 참조를 반환합니다.
    pub fn upbit_window(&self, coin: &str) -> Option<&CandleWindow> {
        self.upbit_coin_windows.get(coin)
    }

    /// 특정 코인의 Bybit 가격 윈도우에 대한 참조를 반환합니다.
    pub fn bybit_window(&self, coin: &str) -> Option<&CandleWindow> {
        self.bybit_windows.get(coin)
    }

    /// USDT/KRW 환율 윈도우에 대한 참조를 반환합니다.
    pub fn usdt_krw_window(&self) -> &CandleWindow {
        &self.usdt_krw_window
    }

    /// 특정 코인의 최신 스프레드(%)를 반환합니다.
    pub fn last_spread_pct(&self, coin: &str) -> Option<f64> {
        self.spread_pct_windows.get(coin)?.last()
    }

    /// 특정 코인의 스프레드 윈도우가 Z-Score 계산에 필요한 만큼 데이터가 찼는지 확인합니다.
    pub fn is_ready(&self, coin: &str) -> bool {
        self.spread_pct_windows
            .get(coin)
            .map(|w| w.is_ready())
            .unwrap_or(false)
    }

    /// 윈도우 크기를 반환합니다.
    pub fn window_size(&self) -> usize {
        self.window_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(n: i64, scale: u32) -> Decimal {
        Decimal::new(n, scale)
    }

    #[test]
    fn test_new_spread_calculator() {
        let coins = vec!["BTC".to_string(), "ETH".to_string()];
        let calc = SpreadCalculator::new(&coins, 10);
        assert!(calc.spread_window("BTC").is_some());
        assert!(calc.spread_window("ETH").is_some());
        assert!(calc.spread_window("XRP").is_none());
        assert_eq!(calc.window_size(), 10);
    }

    #[test]
    fn test_update_single_data_point() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 3);
        let ts = Utc::now();

        // upbit_coin_krw = 138_000_000, usdt_krw = 1380, bybit = 100_050
        // upbit_usdt = 138_000_000 / 1380 = 100_000
        // spread_pct = (100_050 - 100_000) / 100_000 * 100 = 0.05%
        calc.update(
            "BTC",
            ts,
            Some(dec(138_000_000, 0)),
            Some(dec(1380, 0)),
            Some(dec(100_050, 0)),
        )
        .unwrap();

        let spread = calc.last_spread_pct("BTC").unwrap();
        assert!((spread - 0.05).abs() < 1e-6);
    }

    #[test]
    fn test_forward_fill() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts1 = Utc::now();

        // 첫 번째: 모든 값 제공
        calc.update(
            "BTC",
            ts1,
            Some(dec(138_000_000, 0)),
            Some(dec(1380, 0)),
            Some(dec(100_050, 0)),
        )
        .unwrap();

        // 두 번째: upbit만 None (forward-fill)
        let ts2 = ts1 + chrono::Duration::minutes(1);
        calc.update("BTC", ts2, None, Some(dec(1380, 0)), Some(dec(100_100, 0)))
            .unwrap();

        // forward-fill로 upbit_krw = 138_000_000 유지
        // upbit_usdt = 100_000, bybit = 100_100
        // spread_pct = (100_100 - 100_000) / 100_000 * 100 = 0.1%
        let spread = calc.last_spread_pct("BTC").unwrap();
        assert!((spread - 0.1).abs() < 1e-6);
    }

    #[test]
    fn test_no_spread_before_first_values() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        // 모든 값 None이면 스프레드 계산 불가 (forward-fill할 직전 값이 없음)
        calc.update("BTC", ts, None, None, None).unwrap();
        assert!(calc.last_spread_pct("BTC").is_none());
    }

    #[test]
    fn test_partial_values_no_spread() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        // upbit만 제공, usdt_krw/bybit 없음 -> 스프레드 계산 불가
        calc.update("BTC", ts, Some(dec(138_000_000, 0)), None, None)
            .unwrap();
        assert!(calc.last_spread_pct("BTC").is_none());
    }

    #[test]
    fn test_is_ready() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 3);

        for i in 0..3 {
            let ts = Utc::now() + chrono::Duration::minutes(i);
            calc.update(
                "BTC",
                ts,
                Some(dec(138_000_000, 0)),
                Some(dec(1380, 0)),
                Some(dec(100_000 + i as i64 * 10, 0)),
            )
            .unwrap();
        }

        assert!(calc.is_ready("BTC"));
    }

    #[test]
    fn test_unknown_coin_returns_error() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        let result = calc.update(
            "UNKNOWN",
            ts,
            Some(dec(1, 0)),
            Some(dec(1, 0)),
            Some(dec(1, 0)),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_usdt_krw_returns_error() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        let result = calc.update(
            "BTC",
            ts,
            Some(dec(138_000_000, 0)),
            Some(Decimal::ZERO),
            Some(dec(100_000, 0)),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_spread() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        // bybit < upbit_usdt -> 음의 스프레드
        // upbit_usdt = 138_000_000 / 1380 = 100_000
        // bybit = 99_900
        // spread = (99_900 - 100_000) / 100_000 * 100 = -0.1%
        calc.update(
            "BTC",
            ts,
            Some(dec(138_000_000, 0)),
            Some(dec(1380, 0)),
            Some(dec(99_900, 0)),
        )
        .unwrap();

        let spread = calc.last_spread_pct("BTC").unwrap();
        assert!((spread - (-0.1)).abs() < 1e-6);
    }

    #[test]
    fn test_multi_coin() {
        let coins = vec!["BTC".to_string(), "ETH".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        calc.update(
            "BTC",
            ts,
            Some(dec(138_000_000, 0)),
            Some(dec(1380, 0)),
            Some(dec(100_050, 0)),
        )
        .unwrap();

        calc.update(
            "ETH",
            ts,
            Some(dec(4_140_000, 0)),
            Some(dec(1380, 0)),
            Some(dec(3_010, 0)),
        )
        .unwrap();

        // BTC: upbit_usdt = 100_000, bybit = 100_050, spread = 0.05%
        assert!(calc.last_spread_pct("BTC").is_some());
        // ETH: upbit_usdt = 3_000, bybit = 3_010, spread = (3010-3000)/3000*100 = 0.333...%
        let eth_spread = calc.last_spread_pct("ETH").unwrap();
        assert!((eth_spread - 10.0 / 3000.0 * 100.0).abs() < 1e-6);
    }
}
