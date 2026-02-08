//! 실시간 Z-Score 모니터링.
//!
//! WebSocket 스트림에서 수신한 MarketEvent를 1분 캔들로 집계하고,
//! Z-Score 기반 진입/청산 시그널을 실시간으로 감지합니다.

use std::collections::HashMap;
use std::time::Duration;

use chrono::{DateTime, Timelike, Utc};
use rust_decimal::Decimal;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, trace, warn};

use crate::common::statistics;
use crate::error::StrategyError;
use crate::output::csv;
use crate::zscore::config::ZScoreConfig;
use crate::zscore::pnl::ClosedPosition;
use crate::zscore::position::{self, PositionManager, VirtualPosition};
use crate::zscore::signal::{self, Signal};
use crate::zscore::simulator::{TimeseriesRecord, fetch_all_candles};
use crate::zscore::spread::SpreadCalculator;
use arb_exchange::{MarketData, MarketEvent, MarketStream};

/// 분 경계를 기준으로 timestamp를 truncate합니다.
fn truncate_to_minute(ts: DateTime<Utc>) -> DateTime<Utc> {
    ts.with_second(0)
        .and_then(|t| t.with_nanosecond(0))
        .unwrap_or(ts)
}

/// 분 완결 시 반환되는 데이터 (코인별 Upbit close, USDT/KRW, 코인별 Bybit close).
type MinuteCloses = (
    HashMap<String, Option<Decimal>>,
    Option<Decimal>,
    HashMap<String, Option<Decimal>>,
);

/// 코인별 현재 분의 캔들 빌더.
#[derive(Debug)]
struct MinuteCandleBuilder {
    /// 현재 분의 시작 시간.
    current_minute: Option<DateTime<Utc>>,
    /// 코인별 Upbit 마지막 체결가.
    upbit_last_trade: HashMap<String, Decimal>,
    /// USDT/KRW 마지막 체결가.
    usdt_krw_last_trade: Option<Decimal>,
    /// 코인별 Bybit best bid.
    bybit_last_bid: HashMap<String, Decimal>,
}

impl MinuteCandleBuilder {
    fn new() -> Self {
        Self {
            current_minute: None,
            upbit_last_trade: HashMap::new(),
            usdt_krw_last_trade: None,
            bybit_last_bid: HashMap::new(),
        }
    }

    /// 이벤트의 분이 변경되었는지 확인합니다.
    fn is_new_minute(&self, ts: DateTime<Utc>) -> bool {
        let minute = truncate_to_minute(ts);
        match self.current_minute {
            Some(current) => minute > current,
            None => true,
        }
    }

    /// 현재 분을 완결하고 각 코인의 close 데이터를 반환합니다.
    fn finalize_minute(&mut self, coins: &[String]) -> MinuteCloses {
        let mut upbit_closes = HashMap::new();
        let mut bybit_closes = HashMap::new();

        for coin in coins {
            upbit_closes.insert(coin.clone(), self.upbit_last_trade.remove(coin));
            bybit_closes.insert(coin.clone(), self.bybit_last_bid.remove(coin));
        }

        let usdt_krw = self.usdt_krw_last_trade.take();

        // 코인별 close 유무 요약 로그 (Vec 할당 비용을 debug 활성 시에만 부담)
        if tracing::enabled!(tracing::Level::DEBUG) {
            let coins_with_upbit: Vec<&str> = coins
                .iter()
                .filter(|c| upbit_closes.get(*c).and_then(|v| v.as_ref()).is_some())
                .map(|c| c.as_str())
                .collect();
            let coins_with_bybit: Vec<&str> = coins
                .iter()
                .filter(|c| bybit_closes.get(*c).and_then(|v| v.as_ref()).is_some())
                .map(|c| c.as_str())
                .collect();

            debug!(
                minute = ?self.current_minute,
                usdt_krw_present = usdt_krw.is_some(),
                upbit_coins = ?coins_with_upbit,
                bybit_coins = ?coins_with_bybit,
                "분 캔들 완결: 코인별 close 데이터 요약"
            );
        }

        (upbit_closes, usdt_krw, bybit_closes)
    }

    /// 새 분으로 전환합니다.
    fn start_new_minute(&mut self, minute: DateTime<Utc>) {
        self.current_minute = Some(truncate_to_minute(minute));
        self.upbit_last_trade.clear();
        self.usdt_krw_last_trade = None;
        self.bybit_last_bid.clear();
    }

    /// Upbit Trade 이벤트를 처리합니다.
    fn on_upbit_trade(&mut self, market: &str, price: Decimal) {
        trace!(market = market, price = %price, "Upbit trade 수신");
        if market.starts_with("KRW-USDT") {
            self.usdt_krw_last_trade = Some(price);
        } else if let Some(coin) = market.strip_prefix("KRW-") {
            self.upbit_last_trade.insert(coin.to_string(), price);
        }
    }

    /// Bybit BestQuote 이벤트를 처리합니다.
    fn on_bybit_best_quote(&mut self, market: &str, bid: Decimal) {
        trace!(market = market, bid = %bid, "Bybit best quote 수신");
        if let Some(coin) = market.strip_suffix("USDT") {
            self.bybit_last_bid.insert(coin.to_string(), bid);
        }
    }
}

/// 실시간 Z-Score 모니터.
pub struct ZScoreMonitor<U: MarketData + MarketStream, B: MarketData + MarketStream> {
    upbit: U,
    bybit: B,
    config: ZScoreConfig,
}

impl<U: MarketData + MarketStream, B: MarketData + MarketStream> ZScoreMonitor<U, B> {
    /// 새 ZScoreMonitor를 생성합니다.
    pub fn new(upbit: U, bybit: B, config: ZScoreConfig) -> Self {
        Self {
            upbit,
            bybit,
            config,
        }
    }

    /// 실시간 모니터링을 시작합니다.
    ///
    /// CancellationToken이 cancel되면 graceful shutdown합니다.
    pub async fn run(
        &self,
        cancel_token: CancellationToken,
    ) -> Result<Vec<ClosedPosition>, StrategyError> {
        self.config.validate()?;

        info!("실시간 모니터링 시작: 워밍업 데이터 로드 중...");

        // 1. 워밍업: REST API로 캔들 사전 로드
        let mut spread_calc = SpreadCalculator::new(&self.config.coins, self.config.window_size);
        self.warmup(&mut spread_calc).await?;

        info!("워밍업 완료. WebSocket 연결 중...");

        // 2. WebSocket 구독
        let upbit_markets: Vec<String> = {
            let mut markets: Vec<String> = self
                .config
                .coins
                .iter()
                .map(|c| format!("KRW-{c}"))
                .collect();
            markets.push("KRW-USDT".to_string());
            markets
        };
        let bybit_markets: Vec<String> = self
            .config
            .coins
            .iter()
            .map(|c| format!("{c}USDT"))
            .collect();

        info!(
            upbit_markets = ?upbit_markets,
            bybit_markets = ?bybit_markets,
            "WebSocket 구독 마켓 목록"
        );

        let upbit_market_refs: Vec<&str> = upbit_markets.iter().map(|s| s.as_str()).collect();
        let bybit_market_refs: Vec<&str> = bybit_markets.iter().map(|s| s.as_str()).collect();

        let mut upbit_rx = self.upbit.subscribe(&upbit_market_refs).await?;
        let mut bybit_rx = self.bybit.subscribe(&bybit_market_refs).await?;

        info!("WebSocket 연결 완료. 이벤트 루프 시작.");

        // 3. 이벤트 루프
        let mut position_mgr = PositionManager::new();
        let mut candle_builder = MinuteCandleBuilder::new();
        let mut minute_timer = tokio::time::interval(Duration::from_secs(60));
        let mut timeseries_records: Vec<TimeseriesRecord> = Vec::new();
        let mut trades_for_csv: Vec<ClosedPosition> = Vec::new();

        // heartbeat 관련 상태
        let mut total_event_count: u64 = 0;
        let mut heartbeat_timer = tokio::time::interval(Duration::from_secs(300));

        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    info!("종료 요청 수신. 모니터링 종료 중...");
                    break;
                }
                Some(event) = upbit_rx.recv() => {
                    total_event_count += 1;
                    self.handle_event(
                        &event,
                        &mut candle_builder,
                        &mut spread_calc,
                        &mut position_mgr,
                        &mut timeseries_records,
                        &mut trades_for_csv,
                    )?;
                }
                Some(event) = bybit_rx.recv() => {
                    total_event_count += 1;
                    self.handle_event(
                        &event,
                        &mut candle_builder,
                        &mut spread_calc,
                        &mut position_mgr,
                        &mut timeseries_records,
                        &mut trades_for_csv,
                    )?;
                }
                _ = minute_timer.tick() => {
                    // 빈 분 감지: forward-fill 트리거
                    let now = Utc::now();
                    if candle_builder.is_new_minute(now) {
                        self.finalize_and_process(
                            &mut candle_builder,
                            &mut spread_calc,
                            &mut position_mgr,
                            &mut timeseries_records,
                            &mut trades_for_csv,
                            now,
                        )?;
                    }
                }
                _ = heartbeat_timer.tick() => {
                    // 5분마다 heartbeat 로그
                    info!(
                        total_events = total_event_count,
                        open_positions = position_mgr.open_count(),
                        total_trades = trades_for_csv.len(),
                        timeseries_records = timeseries_records.len(),
                        "[heartbeat] 실시간 모니터 상태"
                    );
                }
            }
        }

        // 정리: WebSocket 구독 해제
        self.upbit.unsubscribe().await.ok();
        self.bybit.unsubscribe().await.ok();

        // CSV 저장
        if !trades_for_csv.is_empty() {
            debug!(count = trades_for_csv.len(), dir = %self.config.output_dir.display(), "CSV 저장: 거래 내역");
            csv::write_trades_csv(&self.config.output_dir, &trades_for_csv)?;
        }
        if !timeseries_records.is_empty() {
            debug!(count = timeseries_records.len(), dir = %self.config.output_dir.display(), "CSV 저장: 시계열 기록");
            csv::write_timeseries_csv(&self.config.output_dir, &timeseries_records)?;
        }

        info!(trades = trades_for_csv.len(), "실시간 모니터링 종료");

        Ok(trades_for_csv)
    }

    /// REST API로 워밍업 데이터를 로드합니다.
    async fn warmup(&self, spread_calc: &mut SpreadCalculator) -> Result<(), StrategyError> {
        let end_time = Utc::now();
        let window_size = self.config.window_size;

        for coin in &self.config.coins {
            debug!(
                coin = coin.as_str(),
                window_size = window_size,
                "워밍업 시작: 캔들 데이터 로드"
            );

            let upbit_market = format!("KRW-{coin}");
            let bybit_market = format!("{coin}USDT");

            let upbit_candles = fetch_all_candles(
                &self.upbit,
                &upbit_market,
                self.config.candle_interval,
                window_size,
                end_time,
                200,
                Duration::from_millis(100),
            )
            .await?;

            let bybit_candles = fetch_all_candles(
                &self.bybit,
                &bybit_market,
                self.config.candle_interval,
                window_size,
                end_time,
                1000,
                Duration::from_millis(10),
            )
            .await?;

            let usdt_krw_candles = fetch_all_candles(
                &self.upbit,
                "KRW-USDT",
                self.config.candle_interval,
                window_size,
                end_time,
                200,
                Duration::from_millis(100),
            )
            .await?;

            info!(
                coin = coin.as_str(),
                upbit = upbit_candles.len(),
                bybit = bybit_candles.len(),
                usdt_krw = usdt_krw_candles.len(),
                "워밍업 데이터 로드"
            );

            // 공통 타임스탬프 기준으로 SpreadCalculator 업데이트
            use std::collections::BTreeSet;
            let mut timestamps = BTreeSet::new();
            for c in &upbit_candles {
                timestamps.insert(c.timestamp);
            }
            for c in &bybit_candles {
                timestamps.insert(c.timestamp);
            }
            for c in &usdt_krw_candles {
                timestamps.insert(c.timestamp);
            }

            debug!(
                coin = coin.as_str(),
                common_timestamps = timestamps.len(),
                "워밍업 공통 타임스탬프 수"
            );

            for ts in timestamps {
                let upbit_close = upbit_candles
                    .iter()
                    .find(|c| c.timestamp == ts)
                    .map(|c| c.close);
                let usdt_krw_close = usdt_krw_candles
                    .iter()
                    .find(|c| c.timestamp == ts)
                    .map(|c| c.close);
                let bybit_close = bybit_candles
                    .iter()
                    .find(|c| c.timestamp == ts)
                    .map(|c| c.close);

                spread_calc.update(coin, ts, upbit_close, usdt_krw_close, bybit_close)?;
            }

            debug!(
                coin = coin.as_str(),
                is_ready = spread_calc.is_ready(coin),
                "워밍업 완료"
            );
        }

        Ok(())
    }

    /// MarketEvent를 처리합니다.
    fn handle_event(
        &self,
        event: &MarketEvent,
        candle_builder: &mut MinuteCandleBuilder,
        spread_calc: &mut SpreadCalculator,
        position_mgr: &mut PositionManager,
        timeseries: &mut Vec<TimeseriesRecord>,
        trades: &mut Vec<ClosedPosition>,
    ) -> Result<(), StrategyError> {
        let (event_ts, event_market, event_type) = match event {
            MarketEvent::Trade {
                timestamp, market, ..
            } => (*timestamp, market.as_str(), "Trade"),
            MarketEvent::BestQuote {
                timestamp, market, ..
            } => (*timestamp, market.as_str(), "BestQuote"),
        };

        trace!(
            market = event_market,
            event_type = event_type,
            ts = %event_ts,
            "이벤트 수신"
        );

        // 분 경계 변경 시 이전 분 완결
        if candle_builder.is_new_minute(event_ts) {
            if candle_builder.current_minute.is_some() {
                debug!(
                    prev_minute = ?candle_builder.current_minute,
                    new_minute = %truncate_to_minute(event_ts),
                    "분 경계 변경 감지: 이전 분 완결 처리"
                );
                self.finalize_and_process(
                    candle_builder,
                    spread_calc,
                    position_mgr,
                    timeseries,
                    trades,
                    event_ts,
                )?;
            }
            candle_builder.start_new_minute(event_ts);
        }

        // 이벤트 데이터 축적
        match event {
            MarketEvent::Trade { market, price, .. } => {
                candle_builder.on_upbit_trade(market, *price);
            }
            MarketEvent::BestQuote { market, bid, .. } => {
                candle_builder.on_bybit_best_quote(market, *bid);
            }
        }

        Ok(())
    }

    /// 현재 분을 완결하고 시그널 평가를 수행합니다.
    fn finalize_and_process(
        &self,
        candle_builder: &mut MinuteCandleBuilder,
        spread_calc: &mut SpreadCalculator,
        position_mgr: &mut PositionManager,
        timeseries: &mut Vec<TimeseriesRecord>,
        trades: &mut Vec<ClosedPosition>,
        new_minute_ts: DateTime<Utc>,
    ) -> Result<(), StrategyError> {
        let ts = candle_builder
            .current_minute
            .unwrap_or_else(|| truncate_to_minute(new_minute_ts));

        debug!(
            timestamp = %ts,
            coins = ?self.config.coins,
            "분 완결 시작: 시그널 평가 수행"
        );

        let (upbit_closes, usdt_krw, bybit_closes) =
            candle_builder.finalize_minute(&self.config.coins);

        for coin in &self.config.coins {
            let upbit_close = upbit_closes.get(coin).copied().flatten();
            let bybit_close = bybit_closes.get(coin).copied().flatten();

            // SpreadCalculator 업데이트 (forward-fill 내장)
            spread_calc.update(coin, ts, upbit_close, usdt_krw, bybit_close)?;

            // Liquidation 체크
            if let Some(bybit_price) = bybit_close.or_else(|| {
                spread_calc.bybit_window(coin).and_then(|w| {
                    w.last()
                        .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                })
            }) && position_mgr.check_liquidation(coin, bybit_price)
            {
                let upbit_usdt = spread_calc
                    .upbit_window(coin)
                    .and_then(|w| w.last())
                    .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                    .unwrap_or(Decimal::ZERO);
                let usdt_krw_val = spread_calc
                    .usdt_krw_window()
                    .last()
                    .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                    .unwrap_or(Decimal::ZERO);
                let spread_pct = spread_calc.last_spread_pct(coin).unwrap_or(0.0);

                warn!(coin = coin.as_str(), "Bybit 강제 청산 발생");

                match position_mgr.close_position(
                    coin,
                    ts,
                    upbit_usdt,
                    bybit_price,
                    usdt_krw_val,
                    spread_pct,
                    f64::NAN,
                    self.config.upbit_taker_fee,
                    self.config.bybit_taker_fee,
                    true,
                ) {
                    Ok(closed) => {
                        trades.push(closed);
                    }
                    Err(e) => {
                        warn!(coin = coin.as_str(), error = %e, "강제 청산 처리 실패");
                    }
                }
            }

            // 시그널 평가
            let sig = signal::evaluate_signal(coin, spread_calc, position_mgr, &self.config)?;

            // 시계열 기록
            let spread_window = spread_calc.spread_window(coin);
            let (mean_pct, stddev_val, z_val) = if let Some(w) = spread_window {
                if w.is_ready() {
                    let data = w.data();
                    let m = statistics::mean(data);
                    let s = statistics::stddev(data, m);
                    let z = if s >= self.config.min_stddev_threshold {
                        spread_calc
                            .last_spread_pct(coin)
                            .map(|sp| (sp - m) / s)
                            .unwrap_or(0.0)
                    } else {
                        0.0
                    };
                    (m, s, z)
                } else {
                    (0.0, 0.0, 0.0)
                }
            } else {
                (0.0, 0.0, 0.0)
            };

            let signal_str = match &sig {
                Some(Signal::Enter { .. }) => "ENTER",
                Some(Signal::Exit { .. }) => "EXIT",
                None => "NONE",
            };

            let position_str = if position_mgr.has_position(coin) {
                "OPEN"
            } else {
                "NONE"
            };

            // 코인별 스프레드/Z-Score/시그널 결과 로그
            debug!(
                coin = coin.as_str(),
                spread_pct = spread_calc.last_spread_pct(coin).unwrap_or(0.0),
                mean = mean_pct,
                stddev = stddev_val,
                z_score = z_val,
                signal = signal_str,
                position = position_str,
                "분 완결: 코인별 통계 및 시그널"
            );

            timeseries.push(TimeseriesRecord {
                timestamp: ts,
                coin: coin.clone(),
                upbit_usdt_price: spread_calc
                    .upbit_window(coin)
                    .and_then(|w| w.last())
                    .unwrap_or(0.0),
                bybit_price: spread_calc
                    .bybit_window(coin)
                    .and_then(|w| w.last())
                    .unwrap_or(0.0),
                spread_pct: spread_calc.last_spread_pct(coin).unwrap_or(0.0),
                mean_spread_pct: mean_pct,
                stddev: stddev_val,
                z_score: z_val,
                signal: signal_str.to_string(),
                position: position_str.to_string(),
            });

            // 시그널 처리
            match sig {
                Some(Signal::Enter {
                    coin: c,
                    z_score,
                    spread_pct,
                    expected_profit_pct,
                }) => {
                    let size_usdt = self.config.total_capital_usdt * self.config.position_ratio;
                    let upbit_usdt = spread_calc
                        .upbit_window(&c)
                        .and_then(|w| w.last())
                        .unwrap_or(0.0);
                    let bybit_f64 = spread_calc
                        .bybit_window(&c)
                        .and_then(|w| w.last())
                        .unwrap_or(0.0);
                    let usdt_krw_val = spread_calc.usdt_krw_window().last().unwrap_or(0.0);

                    let upbit_price = Decimal::try_from(upbit_usdt).unwrap_or(Decimal::ZERO);
                    let bybit_price_dec = Decimal::try_from(bybit_f64).unwrap_or(Decimal::ZERO);
                    let usdt_krw_dec = Decimal::try_from(usdt_krw_val).unwrap_or(Decimal::ZERO);

                    let liq_price = position::calculate_liquidation_price(
                        bybit_price_dec,
                        self.config.leverage,
                        self.config.bybit_mmr,
                        self.config.bybit_taker_fee,
                    );

                    let pos = VirtualPosition {
                        coin: c.clone(),
                        entry_time: ts,
                        upbit_entry_price: upbit_price,
                        bybit_entry_price: bybit_price_dec,
                        bybit_liquidation_price: liq_price,
                        entry_usdt_krw: usdt_krw_dec,
                        entry_spread_pct: spread_pct,
                        entry_z_score: z_score,
                        size_usdt,
                    };

                    info!(
                        coin = c.as_str(),
                        z_score = z_score,
                        spread_pct = spread_pct,
                        expected_profit = expected_profit_pct,
                        "[실시간] 진입 시그널"
                    );

                    debug!(
                        coin = c.as_str(),
                        size_usdt = %size_usdt,
                        upbit_usdt = upbit_usdt,
                        bybit_price = bybit_f64,
                        usdt_krw = usdt_krw_val,
                        liquidation_price = %liq_price,
                        "진입 상세: 포지션 사이즈 및 가격 정보"
                    );

                    if let Err(e) = position_mgr.open_position(pos) {
                        warn!(coin = c.as_str(), error = %e, "포지션 오픈 실패");
                    }
                }
                Some(Signal::Exit {
                    coin: c,
                    z_score,
                    spread_pct,
                }) => {
                    let upbit_usdt = spread_calc
                        .upbit_window(&c)
                        .and_then(|w| w.last())
                        .unwrap_or(0.0);
                    let bybit_f64 = spread_calc
                        .bybit_window(&c)
                        .and_then(|w| w.last())
                        .unwrap_or(0.0);
                    let usdt_krw_val = spread_calc.usdt_krw_window().last().unwrap_or(0.0);

                    info!(
                        coin = c.as_str(),
                        z_score = z_score,
                        spread_pct = spread_pct,
                        "[실시간] 청산 시그널"
                    );

                    debug!(
                        coin = c.as_str(),
                        upbit_usdt = upbit_usdt,
                        bybit_price = bybit_f64,
                        usdt_krw = usdt_krw_val,
                        "청산 상세: 현재 가격 정보"
                    );

                    match position_mgr.close_position(
                        &c,
                        ts,
                        Decimal::try_from(upbit_usdt).unwrap_or(Decimal::ZERO),
                        Decimal::try_from(bybit_f64).unwrap_or(Decimal::ZERO),
                        Decimal::try_from(usdt_krw_val).unwrap_or(Decimal::ZERO),
                        spread_pct,
                        z_score,
                        self.config.upbit_taker_fee,
                        self.config.bybit_taker_fee,
                        false,
                    ) {
                        Ok(closed) => {
                            trades.push(closed);
                        }
                        Err(e) => {
                            warn!(coin = c.as_str(), error = %e, "포지션 청산 실패");
                        }
                    }
                }
                None => {}
            }
        }

        candle_builder.start_new_minute(new_minute_ts);
        Ok(())
    }

    /// 설정에 접근합니다.
    pub fn config(&self) -> &ZScoreConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_to_minute() {
        use chrono::TimeZone;
        let ts = Utc.with_ymd_and_hms(2026, 2, 6, 10, 30, 45).unwrap();
        let truncated = truncate_to_minute(ts);
        assert_eq!(truncated.second(), 0);
        assert_eq!(truncated.minute(), 30);
        assert_eq!(truncated.nanosecond(), 0);
    }

    #[test]
    fn test_candle_builder_new_minute() {
        let builder = MinuteCandleBuilder::new();
        let ts = Utc::now();
        assert!(builder.is_new_minute(ts));
    }

    #[test]
    fn test_candle_builder_same_minute() {
        let mut builder = MinuteCandleBuilder::new();
        let ts = Utc::now();
        builder.start_new_minute(ts);

        // 같은 분 내의 시간
        let same_minute = truncate_to_minute(ts);
        assert!(!builder.is_new_minute(same_minute));
    }

    #[test]
    fn test_candle_builder_on_upbit_trade() {
        let mut builder = MinuteCandleBuilder::new();
        builder.on_upbit_trade("KRW-BTC", Decimal::new(138_000_000, 0));
        builder.on_upbit_trade("KRW-USDT", Decimal::new(1380, 0));

        assert_eq!(
            builder.upbit_last_trade.get("BTC"),
            Some(&Decimal::new(138_000_000, 0))
        );
        assert_eq!(builder.usdt_krw_last_trade, Some(Decimal::new(1380, 0)));
    }

    #[test]
    fn test_candle_builder_on_bybit_quote() {
        let mut builder = MinuteCandleBuilder::new();
        builder.on_bybit_best_quote("BTCUSDT", Decimal::new(100_050, 0));

        assert_eq!(
            builder.bybit_last_bid.get("BTC"),
            Some(&Decimal::new(100_050, 0))
        );
    }

    #[test]
    fn test_candle_builder_finalize() {
        let mut builder = MinuteCandleBuilder::new();
        builder.on_upbit_trade("KRW-BTC", Decimal::new(138_000_000, 0));
        builder.on_upbit_trade("KRW-USDT", Decimal::new(1380, 0));
        builder.on_bybit_best_quote("BTCUSDT", Decimal::new(100_050, 0));

        let coins = vec!["BTC".to_string()];
        let (upbit, usdt_krw, bybit) = builder.finalize_minute(&coins);

        assert_eq!(upbit.get("BTC"), Some(&Some(Decimal::new(138_000_000, 0))));
        assert_eq!(usdt_krw, Some(Decimal::new(1380, 0)));
        assert_eq!(bybit.get("BTC"), Some(&Some(Decimal::new(100_050, 0))));

        // finalize 후 내부 상태 클리어 확인
        assert!(builder.upbit_last_trade.is_empty());
        assert!(builder.usdt_krw_last_trade.is_none());
        assert!(builder.bybit_last_bid.is_empty());
    }
}
