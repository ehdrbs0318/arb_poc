//! 스프레드 계산 엔진.
//!
//! 2-way 입력(Upbit coin/KRW + USD/KRW 환율, Bybit coin/USDT)을 동기화하여
//! 상대 스프레드(%)를 계산하고 롤링 윈도우에 유지합니다.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use tracing::{debug, trace, warn};

use crate::common::candle_window::CandleWindow;
use crate::common::convert::decimal_to_f64;
use crate::error::StrategyError;

/// 연속 누락 경고 임계값 (분).
///
/// 저유동성 코인은 수 분간 거래가 없을 수 있으므로, 짧은 갭은 debug로 기록하고
/// 이 임계값 이상의 연속 갭만 warn으로 기록합니다.
const CONSECUTIVE_MISSING_WARN_THRESHOLD: usize = 30;

/// 개별 소스의 forward-fill 상태를 추적합니다.
#[derive(Debug, Clone)]
struct ForwardFillState {
    /// 직전 유효 가격 (forward-fill용).
    last_value: Option<Decimal>,
    /// 연속 누락 횟수.
    consecutive_missing: usize,
    /// 현재 갭에서 이미 경고를 출력했는지 여부.
    warned_current_gap: bool,
}

impl ForwardFillState {
    fn new() -> Self {
        Self {
            last_value: None,
            consecutive_missing: 0,
            warned_current_gap: false,
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
                // 갭이 끝남 → 갭 길이가 의미 있었으면 요약 로그
                if self.consecutive_missing >= CONSECUTIVE_MISSING_WARN_THRESHOLD {
                    warn!(
                        "캔들 갭 종료: {} — 총 {}분 연속 누락 후 데이터 복구 at {}",
                        source, self.consecutive_missing, timestamp
                    );
                } else if self.consecutive_missing >= 5 {
                    debug!(
                        "캔들 갭 종료: {} — {}분 연속 누락 후 복구 at {}",
                        source, self.consecutive_missing, timestamp
                    );
                }
                self.last_value = Some(v);
                self.consecutive_missing = 0;
                self.warned_current_gap = false;
                Some(v)
            }
            None => {
                self.consecutive_missing += 1;
                if self.consecutive_missing >= CONSECUTIVE_MISSING_WARN_THRESHOLD
                    && !self.warned_current_gap
                {
                    warn!(
                        "연속 {}분 캔들 누락 진행 중: {} at {}",
                        self.consecutive_missing, source, timestamp
                    );
                    self.warned_current_gap = true;
                }
                self.last_value
            }
        }
    }
}

/// 분봉 push 시 O(1)로 갱신되는 증분 통계 캐시.
///
/// 윈도우의 mean과 stddev를 running sum 방식으로 유지하여,
/// 틱 경로에서 O(1)로 통계를 조회할 수 있습니다.
#[derive(Debug, Clone)]
struct IncrementalStats {
    /// 합계 (running sum).
    running_sum: f64,
    /// 제곱합 (running sum of squares).
    running_sum_sq: f64,
    /// 현재 데이터 개수.
    count: usize,
}

impl IncrementalStats {
    fn new() -> Self {
        Self {
            running_sum: 0.0,
            running_sum_sq: 0.0,
            count: 0,
        }
    }

    /// 새 값 push, 윈도우 초과 시 pop된 값 반영.
    ///
    /// `popped_val`이 `Some`이면 윈도우에서 제거된 값을 차감합니다.
    fn push(&mut self, new_val: f64, popped_val: Option<f64>) {
        // pop된 값 차감
        if let Some(old) = popped_val {
            self.running_sum -= old;
            self.running_sum_sq -= old * old;
            self.count -= 1;
        }

        // 새 값 추가
        self.running_sum += new_val;
        self.running_sum_sq += new_val * new_val;
        self.count += 1;
    }

    /// 현재 mean.
    fn mean(&self) -> f64 {
        if self.count == 0 {
            return 0.0;
        }
        self.running_sum / self.count as f64
    }

    /// 현재 population stddev.
    fn stddev(&self) -> f64 {
        if self.count == 0 {
            return 0.0;
        }
        let n = self.count as f64;
        let variance = (self.running_sum_sq / n) - (self.running_sum / n).powi(2);
        // 부동소수점 오차로 음수가 될 수 있으므로 0 보정
        if variance < 0.0 { 0.0 } else { variance.sqrt() }
    }
}

/// IncrementalStats / WelfordStats 공통 인터페이스.
///
/// **stddev 규약**: 모든 구현체는 **population stddev** (분모=n)를 반환합니다.
trait StatsAccumulator {
    /// 새 값 push. `popped`는 윈도우에서 제거된 값.
    fn push(&mut self, value: f64, popped: Option<f64>);
    /// 현재 mean.
    fn mean(&self) -> f64;
    /// Population stddev (분모=n).
    fn stddev(&self) -> f64;
    /// 윈도우 전체 데이터로 통계를 재계산합니다.
    ///
    /// WelfordStats: pop 발생 시 전체 재계산.
    /// IncrementalStats: no-op.
    fn rebuild(&mut self, _data: &[f64]) {}
}

impl StatsAccumulator for IncrementalStats {
    fn push(&mut self, value: f64, popped: Option<f64>) {
        IncrementalStats::push(self, value, popped);
    }

    fn mean(&self) -> f64 {
        IncrementalStats::mean(self)
    }

    fn stddev(&self) -> f64 {
        IncrementalStats::stddev(self)
    }
    // rebuild는 기본 no-op 사용
}

/// 단기 regime change 감지용 윈도우 크기 (60분).
const SHORT_WINDOW_SIZE: usize = 60;

/// Welford's online algorithm 기반 rolling statistics.
///
/// IncrementalStats의 naive (sum_sq - sum^2) 방식 대비
/// catastrophic cancellation에 면역이며, 소규모 윈도우(60개)에서 특히 안정적.
#[derive(Debug, Clone)]
struct WelfordStats {
    count: usize,
    mean_val: f64,
    m2: f64,
}

impl WelfordStats {
    fn new() -> Self {
        Self {
            count: 0,
            mean_val: 0.0,
            m2: 0.0,
        }
    }
}

impl StatsAccumulator for WelfordStats {
    fn push(&mut self, value: f64, _popped: Option<f64>) {
        // Welford push만 수행, pop은 rebuild에서 처리
        self.count += 1;
        let delta = value - self.mean_val;
        self.mean_val += delta / self.count as f64;
        let delta2 = value - self.mean_val;
        self.m2 += delta * delta2;
    }

    fn mean(&self) -> f64 {
        self.mean_val
    }

    /// Population stddev (분모=n). IncrementalStats와 동일 규약.
    fn stddev(&self) -> f64 {
        if self.count < 2 {
            0.0
        } else {
            let variance = self.m2 / self.count as f64;
            if variance < 0.0 { 0.0 } else { variance.sqrt() }
        }
    }

    /// 윈도우 전체 데이터로 재계산.
    fn rebuild(&mut self, data: &[f64]) {
        self.count = 0;
        self.mean_val = 0.0;
        self.m2 = 0.0;
        for &v in data {
            self.count += 1;
            let delta = v - self.mean_val;
            self.mean_val += delta / self.count as f64;
            let delta2 = v - self.mean_val;
            self.m2 += delta * delta2;
        }
    }
}

/// 윈도우와 통계에 값을 동시에 push합니다.
///
/// IncrementalStats: push로 O(1) 갱신.
/// WelfordStats: pop 발생 시 rebuild(window.data()) O(n).
fn push_to_window_and_stats<S: StatsAccumulator>(
    window: &mut CandleWindow,
    stats: &mut S,
    value: f64,
) {
    let had_pop = window.data().len() >= window.window_size();
    let popped = if had_pop {
        Some(window.data()[0])
    } else {
        None
    };
    stats.push(value, popped);
    window.push(value);
    if had_pop {
        let data: Vec<f64> = window.data().iter().copied().collect();
        stats.rebuild(&data);
    }
}

/// 스프레드 계산기.
///
/// 코인별로 Upbit(KRW) + USD/KRW 환율, Bybit(USDT) 가격을 동기화하여
/// 상대 스프레드(%)를 계산합니다.
#[derive(Debug, Clone)]
pub struct SpreadCalculator {
    /// 코인별 Upbit 캔들 윈도우 (USD 환산 close 가격).
    upbit_coin_windows: HashMap<String, CandleWindow>,
    /// 코인별 Bybit 캔들 윈도우 (coinUSDT linear close 가격).
    bybit_windows: HashMap<String, CandleWindow>,
    /// 코인별 스프레드(%) 윈도우 - Z-Score 계산의 입력.
    spread_pct_windows: HashMap<String, CandleWindow>,
    /// 코인별 증분 통계 캐시 (스프레드 윈도우용).
    spread_stats: HashMap<String, IncrementalStats>,
    /// 코인별 단기 스프레드 윈도우 (regime change 감지용, 60분).
    short_spread_windows: HashMap<String, CandleWindow>,
    /// 코인별 단기 통계 (Welford's online algorithm).
    short_spread_stats: HashMap<String, WelfordStats>,
    /// 코인별 Upbit forward-fill 상태.
    upbit_ff: HashMap<String, ForwardFillState>,
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
        let mut spread_stats = HashMap::new();
        let mut short_spread_windows = HashMap::new();
        let mut short_spread_stats = HashMap::new();
        let mut upbit_ff = HashMap::new();
        let mut bybit_ff = HashMap::new();

        for coin in coins {
            upbit_coin_windows.insert(coin.clone(), CandleWindow::new(window_size));
            bybit_windows.insert(coin.clone(), CandleWindow::new(window_size));
            spread_pct_windows.insert(coin.clone(), CandleWindow::new(window_size));
            spread_stats.insert(coin.clone(), IncrementalStats::new());
            short_spread_windows.insert(coin.clone(), CandleWindow::new(SHORT_WINDOW_SIZE));
            short_spread_stats.insert(coin.clone(), WelfordStats::new());
            upbit_ff.insert(coin.clone(), ForwardFillState::new());
            bybit_ff.insert(coin.clone(), ForwardFillState::new());
        }

        Self {
            upbit_coin_windows,
            bybit_windows,
            spread_pct_windows,
            spread_stats,
            short_spread_windows,
            short_spread_stats,
            upbit_ff,
            bybit_ff,
            window_size,
        }
    }

    /// 새 코인을 추가하고 빈 윈도우를 초기화합니다.
    ///
    /// idempotent: 이미 존재하는 코인은 데이터를 유지하고 무시합니다.
    pub fn add_coin(&mut self, coin: &str) {
        if self.upbit_coin_windows.contains_key(coin) {
            return;
        }
        self.upbit_coin_windows
            .insert(coin.to_string(), CandleWindow::new(self.window_size));
        self.bybit_windows
            .insert(coin.to_string(), CandleWindow::new(self.window_size));
        self.spread_pct_windows
            .insert(coin.to_string(), CandleWindow::new(self.window_size));
        self.spread_stats
            .insert(coin.to_string(), IncrementalStats::new());
        self.short_spread_windows
            .insert(coin.to_string(), CandleWindow::new(SHORT_WINDOW_SIZE));
        self.short_spread_stats
            .insert(coin.to_string(), WelfordStats::new());
        self.upbit_ff
            .insert(coin.to_string(), ForwardFillState::new());
        self.bybit_ff
            .insert(coin.to_string(), ForwardFillState::new());
    }

    /// 코인을 제거하고 관련 데이터를 삭제합니다.
    ///
    /// 존재하지 않는 코인은 패닉 없이 무시합니다.
    pub fn remove_coin(&mut self, coin: &str) {
        self.upbit_coin_windows.remove(coin);
        self.bybit_windows.remove(coin);
        self.spread_pct_windows.remove(coin);
        self.spread_stats.remove(coin);
        self.short_spread_windows.remove(coin);
        self.short_spread_stats.remove(coin);
        self.upbit_ff.remove(coin);
        self.bybit_ff.remove(coin);
    }

    /// 현재 감시 중인 코인 목록을 반환합니다.
    pub fn active_coins(&self) -> Vec<&str> {
        self.spread_pct_windows.keys().map(|k| k.as_str()).collect()
    }

    /// 특정 코인의 캔들 데이터를 업데이트하고 spread_pct를 재계산합니다.
    ///
    /// 각 코인 입력은 `Option` - `None`이면 forward-fill (직전 값 유지).
    /// `usd_krw`는 Yahoo Finance에서 가져온 환율로, 항상 유효한 f64 값이어야 합니다.
    /// 2개 코인 입력이 모두 준비되었을 때만 spread_pct를 계산하고 윈도우에 push합니다.
    pub fn update(
        &mut self,
        coin: &str,
        timestamp: DateTime<Utc>,
        upbit_coin: Option<Decimal>,
        usd_krw: f64,
        bybit: Option<Decimal>,
    ) -> Result<(), StrategyError> {
        // USD/KRW 환율 0 체크
        if usd_krw == 0.0 {
            warn!(coin = coin, "USD/KRW 환율 0 감지");
            return Err(StrategyError::DataAlignment(
                "USD/KRW rate is zero".to_string(),
            ));
        }

        // forward-fill 적용
        let upbit_ff_state = self
            .upbit_ff
            .get_mut(coin)
            .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?;
        let upbit_val = upbit_ff_state.update(upbit_coin, &format!("upbit_{coin}"), timestamp);

        let bybit_ff_state = self
            .bybit_ff
            .get_mut(coin)
            .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?;
        let bybit_val = bybit_ff_state.update(bybit, &format!("bybit_{coin}"), timestamp);

        // 2개 모두 유효한 경우에만 스프레드 계산
        if let (Some(upbit_krw_close), Some(bybit_close)) = (upbit_val, bybit_val) {
            // Upbit USD 환산가 (2-way: KRW 가격을 USD/KRW로 나눔)
            let upbit_krw_f64 = decimal_to_f64(upbit_krw_close)?;
            let upbit_usd_f64 = upbit_krw_f64 / usd_krw;
            let bybit_f64 = decimal_to_f64(bybit_close)?;

            // 0 나누기 방지
            if upbit_usd_f64 == 0.0 {
                warn!(coin = coin, "Upbit USD 환산가 0 감지");
                return Err(StrategyError::DataAlignment(
                    "Upbit USD price is zero".to_string(),
                ));
            }

            // 상대 스프레드 (%)
            let spread_pct = (bybit_f64 - upbit_usd_f64) / upbit_usd_f64 * 100.0;

            trace!(
                coin = coin,
                upbit_usd = upbit_usd_f64,
                bybit = bybit_f64,
                spread_pct = spread_pct,
                "스프레드 계산 완료"
            );

            // 각 윈도우에 push
            self.upbit_coin_windows
                .get_mut(coin)
                .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?
                .push(upbit_usd_f64);

            self.bybit_windows
                .get_mut(coin)
                .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?
                .push(bybit_f64);

            // 장기 윈도우 + stats push
            {
                let spread_window = self
                    .spread_pct_windows
                    .get_mut(coin)
                    .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?;
                let stats = self
                    .spread_stats
                    .get_mut(coin)
                    .ok_or_else(|| StrategyError::Config(format!("unknown coin: {coin}")))?;
                push_to_window_and_stats(spread_window, stats, spread_pct);
            }

            // 단기 윈도우 + stats push (방어적: 없으면 skip)
            if let (Some(short_window), Some(short_stats)) = (
                self.short_spread_windows.get_mut(coin),
                self.short_spread_stats.get_mut(coin),
            ) {
                push_to_window_and_stats(short_window, short_stats, spread_pct);
            }
        } else {
            // 2개 입력 중 일부 누락으로 스프레드 미계산
            trace!(
                coin = coin,
                upbit_present = upbit_val.is_some(),
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

    /// 특정 코인의 Upbit USD 환산 가격 윈도우에 대한 참조를 반환합니다.
    pub fn upbit_window(&self, coin: &str) -> Option<&CandleWindow> {
        self.upbit_coin_windows.get(coin)
    }

    /// 특정 코인의 Bybit 가격 윈도우에 대한 참조를 반환합니다.
    pub fn bybit_window(&self, coin: &str) -> Option<&CandleWindow> {
        self.bybit_windows.get(coin)
    }

    /// 특정 코인의 캐시된 통계(mean, stddev)를 O(1)로 반환합니다.
    ///
    /// 윈도우가 준비되지 않았으면 `None` 반환.
    pub fn cached_stats(&self, coin: &str) -> Option<(f64, f64)> {
        // 윈도우가 준비(가득 참)되어 있는지 확인
        let window = self.spread_pct_windows.get(coin)?;
        if !window.is_ready() {
            return None;
        }
        let stats = self.spread_stats.get(coin)?;
        Some((stats.mean(), stats.stddev()))
    }

    /// 단기 윈도우(60분)의 (mean, stddev)를 반환합니다.
    ///
    /// 윈도우 데이터가 `SHORT_WINDOW_SIZE`(60)개 미만이면 `None` 반환.
    /// 세션 시작 후 최초 60분간은 항상 `None`을 반환하므로,
    /// regime change 감지는 장기 stddev로 fallback됩니다.
    pub fn cached_short_stats(&self, coin: &str) -> Option<(f64, f64)> {
        let window = self.short_spread_windows.get(coin)?;
        if !window.is_ready() {
            return None;
        }
        let stats = self.short_spread_stats.get(coin)?;
        Some((stats.mean(), stats.stddev()))
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
    use crate::common::statistics;
    use std::collections::VecDeque;

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

        // upbit_coin_krw = 138_000_000, usd_krw = 1380.0, bybit = 100_050
        // upbit_usd = 138_000_000 / 1380.0 = 100_000
        // spread_pct = (100_050 - 100_000) / 100_000 * 100 = 0.05%
        calc.update(
            "BTC",
            ts,
            Some(dec(138_000_000, 0)),
            1380.0,
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
            1380.0,
            Some(dec(100_050, 0)),
        )
        .unwrap();

        // 두 번째: upbit만 None (forward-fill), usd_krw는 항상 유효
        let ts2 = ts1 + chrono::Duration::minutes(1);
        calc.update("BTC", ts2, None, 1380.0, Some(dec(100_100, 0)))
            .unwrap();

        // forward-fill로 upbit_krw = 138_000_000 유지
        // upbit_usd = 100_000, bybit = 100_100
        // spread_pct = (100_100 - 100_000) / 100_000 * 100 = 0.1%
        let spread = calc.last_spread_pct("BTC").unwrap();
        assert!((spread - 0.1).abs() < 1e-6);
    }

    #[test]
    fn test_no_spread_before_first_values() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        // 코인 값 모두 None이면 스프레드 계산 불가 (forward-fill할 직전 값이 없음)
        calc.update("BTC", ts, None, 1380.0, None).unwrap();
        assert!(calc.last_spread_pct("BTC").is_none());
    }

    #[test]
    fn test_partial_values_no_spread() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        // upbit만 제공, bybit 없음 -> 스프레드 계산 불가
        calc.update("BTC", ts, Some(dec(138_000_000, 0)), 1380.0, None)
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
                1380.0,
                Some(dec(100_000 + i * 10, 0)),
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

        let result = calc.update("UNKNOWN", ts, Some(dec(1, 0)), 1380.0, Some(dec(1, 0)));
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_usd_krw_returns_error() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        let result = calc.update(
            "BTC",
            ts,
            Some(dec(138_000_000, 0)),
            0.0,
            Some(dec(100_000, 0)),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_spread() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        // bybit < upbit_usd -> 음의 스프레드
        // upbit_usd = 138_000_000 / 1380.0 = 100_000
        // bybit = 99_900
        // spread = (99_900 - 100_000) / 100_000 * 100 = -0.1%
        calc.update(
            "BTC",
            ts,
            Some(dec(138_000_000, 0)),
            1380.0,
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
            1380.0,
            Some(dec(100_050, 0)),
        )
        .unwrap();

        calc.update(
            "ETH",
            ts,
            Some(dec(4_140_000, 0)),
            1380.0,
            Some(dec(3_010, 0)),
        )
        .unwrap();

        // BTC: upbit_usd = 100_000, bybit = 100_050, spread = 0.05%
        assert!(calc.last_spread_pct("BTC").is_some());
        // ETH: upbit_usd = 3_000, bybit = 3_010, spread = (3010-3000)/3000*100 = 0.333...%
        let eth_spread = calc.last_spread_pct("ETH").unwrap();
        assert!((eth_spread - 10.0 / 3000.0 * 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_add_coin_new() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);

        // 새 코인 추가
        calc.add_coin("ETH");

        // 추가된 코인에 대해 윈도우가 존재해야 함
        assert!(calc.spread_window("ETH").is_some());
        assert!(calc.upbit_window("ETH").is_some());
        assert!(calc.bybit_window("ETH").is_some());

        // 추가된 코인으로 업데이트 가능해야 함
        let ts = Utc::now();
        let result = calc.update(
            "ETH",
            ts,
            Some(dec(4_140_000, 0)),
            1380.0,
            Some(dec(3_010, 0)),
        );
        assert!(result.is_ok());
        assert!(calc.last_spread_pct("ETH").is_some());
    }

    #[test]
    fn test_add_coin_idempotent() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);
        let ts = Utc::now();

        // 기존 데이터 추가
        calc.update(
            "BTC",
            ts,
            Some(dec(138_000_000, 0)),
            1380.0,
            Some(dec(100_050, 0)),
        )
        .unwrap();

        let spread_before = calc.last_spread_pct("BTC").unwrap();

        // 이미 존재하는 코인 재추가 -> 데이터가 보존되어야 함
        calc.add_coin("BTC");

        let spread_after = calc.last_spread_pct("BTC").unwrap();
        assert!((spread_before - spread_after).abs() < 1e-12);
    }

    #[test]
    fn test_remove_coin() {
        let coins = vec!["BTC".to_string(), "ETH".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);

        // ETH 제거
        calc.remove_coin("ETH");

        // 제거된 코인에 대해 윈도우가 없어야 함
        assert!(calc.spread_window("ETH").is_none());
        assert!(calc.upbit_window("ETH").is_none());
        assert!(calc.bybit_window("ETH").is_none());

        // 제거된 코인으로 업데이트하면 에러
        let ts = Utc::now();
        let result = calc.update(
            "ETH",
            ts,
            Some(dec(4_140_000, 0)),
            1380.0,
            Some(dec(3_010, 0)),
        );
        assert!(result.is_err());

        // BTC는 영향 없음
        assert!(calc.spread_window("BTC").is_some());
    }

    #[test]
    fn test_remove_coin_nonexistent() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);

        // 존재하지 않는 코인 제거 -> 패닉 없이 무시
        calc.remove_coin("XRP");

        // BTC 영향 없음
        assert!(calc.spread_window("BTC").is_some());
    }

    #[test]
    fn test_active_coins() {
        let coins = vec!["BTC".to_string(), "ETH".to_string()];
        let calc = SpreadCalculator::new(&coins, 10);

        let mut active = calc.active_coins();
        active.sort();
        assert_eq!(active, vec!["BTC", "ETH"]);
    }

    #[test]
    fn test_active_coins_after_add_remove() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 10);

        calc.add_coin("ETH");
        calc.add_coin("XRP");

        let mut active = calc.active_coins();
        active.sort();
        assert_eq!(active, vec!["BTC", "ETH", "XRP"]);

        calc.remove_coin("ETH");

        let mut active = calc.active_coins();
        active.sort();
        assert_eq!(active, vec!["BTC", "XRP"]);
    }

    // --- IncrementalStats 테스트 ---

    #[test]
    fn test_incremental_stats_basic() {
        // 여러 값 push 후 mean/stddev가 statistics 모듈 결과와 일치하는지 확인
        let mut stats = IncrementalStats::new();
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mut deque: VecDeque<f64> = VecDeque::new();

        for v in &values {
            stats.push(*v, None);
            deque.push_back(*v);
        }

        let expected_mean = statistics::mean(&deque);
        let expected_stddev = statistics::stddev(&deque, expected_mean);

        assert!(
            (stats.mean() - expected_mean).abs() < 1e-10,
            "mean 불일치: got {}, expected {}",
            stats.mean(),
            expected_mean
        );
        assert!(
            (stats.stddev() - expected_stddev).abs() < 1e-10,
            "stddev 불일치: got {}, expected {}",
            stats.stddev(),
            expected_stddev
        );
    }

    #[test]
    fn test_incremental_stats_with_pop() {
        // 윈도우 크기 3, 5개 값 push -> pop이 발생해도 정확한 통계 유지
        let window_size = 3;
        let mut stats = IncrementalStats::new();
        let mut deque: VecDeque<f64> = VecDeque::new();

        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];

        for v in &values {
            let popped = if deque.len() >= window_size {
                Some(deque.pop_front().unwrap())
            } else {
                None
            };
            stats.push(*v, popped);
            deque.push_back(*v);
        }

        // 최종 윈도우: [30, 40, 50]
        assert_eq!(deque.len(), window_size);
        let expected_mean = statistics::mean(&deque);
        let expected_stddev = statistics::stddev(&deque, expected_mean);

        assert!(
            (stats.mean() - expected_mean).abs() < 1e-10,
            "mean 불일치 (pop 후): got {}, expected {}",
            stats.mean(),
            expected_mean
        );
        assert!(
            (stats.stddev() - expected_stddev).abs() < 1e-10,
            "stddev 불일치 (pop 후): got {}, expected {}",
            stats.stddev(),
            expected_stddev
        );
    }

    #[test]
    fn test_cached_stats() {
        // cached_stats() 반환값이 수동 계산과 일치하는지 확인
        let coins = vec!["BTC".to_string()];
        let window_size = 5;
        let mut calc = SpreadCalculator::new(&coins, window_size);

        // 윈도우가 가득 차기 전에는 None
        let ts = Utc::now();
        for i in 0..4 {
            calc.update(
                "BTC",
                ts + chrono::Duration::minutes(i),
                Some(dec(138_000_000 + i * 100_000, 0)),
                1380.0,
                Some(dec(100_000 + i * 10, 0)),
            )
            .unwrap();
        }
        assert!(calc.cached_stats("BTC").is_none(), "윈도우 미충족 시 None");

        // 5번째 push로 윈도우 충족
        calc.update(
            "BTC",
            ts + chrono::Duration::minutes(4),
            Some(dec(138_500_000, 0)),
            1380.0,
            Some(dec(100_040, 0)),
        )
        .unwrap();

        let (cached_mean, cached_stddev) = calc.cached_stats("BTC").expect("윈도우 충족 후 Some");

        // 수동 계산과 비교
        let data = calc.spread_window("BTC").unwrap().data();
        let manual_mean = statistics::mean(data);
        let manual_stddev = statistics::stddev(data, manual_mean);

        assert!(
            (cached_mean - manual_mean).abs() < 1e-10,
            "cached mean 불일치: got {}, expected {}",
            cached_mean,
            manual_mean
        );
        assert!(
            (cached_stddev - manual_stddev).abs() < 1e-10,
            "cached stddev 불일치: got {}, expected {}",
            cached_stddev,
            manual_stddev
        );
    }

    #[test]
    fn test_cached_stats_unknown_coin() {
        let coins = vec!["BTC".to_string()];
        let calc = SpreadCalculator::new(&coins, 5);
        assert!(calc.cached_stats("UNKNOWN").is_none());
    }

    #[test]
    fn test_incremental_stats_empty() {
        let stats = IncrementalStats::new();
        assert_eq!(stats.mean(), 0.0);
        assert_eq!(stats.stddev(), 0.0);
        assert_eq!(stats.count, 0);
    }

    #[test]
    fn test_incremental_stats_single_value() {
        let mut stats = IncrementalStats::new();
        stats.push(42.0, None);
        assert!((stats.mean() - 42.0).abs() < 1e-10);
        assert!((stats.stddev() - 0.0).abs() < 1e-10);
    }

    // --- WelfordStats 테스트 ---

    #[test]
    fn test_welford_stats_basic() {
        let mut stats = WelfordStats::new();
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        for v in &values {
            stats.push(*v, None);
        }
        let deque: VecDeque<f64> = values.into_iter().collect();
        let expected_mean = statistics::mean(&deque);
        let expected_stddev = statistics::stddev(&deque, expected_mean);
        assert!((stats.mean() - expected_mean).abs() < 1e-10);
        assert!((stats.stddev() - expected_stddev).abs() < 1e-10);
    }

    #[test]
    fn test_welford_stats_rebuild() {
        let data = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let mut stats = WelfordStats::new();
        stats.rebuild(&data);

        let mut fresh = WelfordStats::new();
        for &v in &data {
            fresh.push(v, None);
        }
        assert!((stats.mean() - fresh.mean()).abs() < 1e-10);
        assert!((stats.stddev() - fresh.stddev()).abs() < 1e-10);
    }

    #[test]
    fn test_welford_matches_incremental() {
        // 두 알고리즘의 결과가 일치하는지 확인 (population stddev)
        let mut welford = WelfordStats::new();
        let mut incremental = IncrementalStats::new();
        let values = vec![2.5, 3.7, 1.2, 4.8, 0.9, 3.3, 2.1];
        for &v in &values {
            welford.push(v, None);
            incremental.push(v, None);
        }
        assert!(
            (welford.mean() - incremental.mean()).abs() < 1e-10,
            "mean 불일치: welford={}, incremental={}",
            welford.mean(),
            incremental.mean()
        );
        assert!(
            (welford.stddev() - incremental.stddev()).abs() < 1e-10,
            "stddev 불일치: welford={}, incremental={}",
            welford.stddev(),
            incremental.stddev()
        );
    }

    #[test]
    fn test_welford_rebuild_after_pop() {
        let mut window = CandleWindow::new(3);
        let mut stats = WelfordStats::new();
        // 5개 값 push, 윈도우는 마지막 3개만 유지
        for v in [10.0, 20.0, 30.0, 40.0, 50.0] {
            push_to_window_and_stats(&mut window, &mut stats, v);
        }
        // 윈도우: [30, 40, 50]
        let data: Vec<f64> = window.data().iter().copied().collect();
        assert_eq!(data, vec![30.0, 40.0, 50.0]);
        let expected_mean = 40.0;
        assert!((stats.mean() - expected_mean).abs() < 1e-10);
    }

    #[test]
    fn test_cached_short_stats_not_ready() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 1440);
        // 60개 미만이면 None
        for i in 0..59 {
            let ts = Utc::now() + chrono::Duration::minutes(i);
            calc.update(
                "BTC",
                ts,
                Some(dec(138_000_000 + i * 1000, 0)),
                1380.0,
                Some(dec(100_000 + i * 10, 0)),
            )
            .unwrap();
        }
        assert!(calc.cached_short_stats("BTC").is_none());
    }

    #[test]
    fn test_cached_short_stats_ready() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 1440);
        for i in 0..60 {
            let ts = Utc::now() + chrono::Duration::minutes(i);
            calc.update(
                "BTC",
                ts,
                Some(dec(138_000_000 + i * 1000, 0)),
                1380.0,
                Some(dec(100_000 + i * 10, 0)),
            )
            .unwrap();
        }
        let stats = calc.cached_short_stats("BTC");
        assert!(stats.is_some(), "60개 push 후 Some이어야 함");
    }

    #[test]
    fn test_short_window_remove_coin() {
        let coins = vec!["BTC".to_string()];
        let mut calc = SpreadCalculator::new(&coins, 1440);
        assert!(calc.short_spread_windows.contains_key("BTC"));
        calc.remove_coin("BTC");
        assert!(!calc.short_spread_windows.contains_key("BTC"));
    }

    #[test]
    fn test_short_window_add_coin() {
        let mut calc = SpreadCalculator::new(&[], 1440);
        calc.add_coin("ETH");
        assert!(calc.short_spread_windows.contains_key("ETH"));
        assert!(calc.short_spread_stats.contains_key("ETH"));
    }
}
