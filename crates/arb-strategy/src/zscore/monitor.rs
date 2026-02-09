//! 실시간 Z-Score 모니터링.
//!
//! WebSocket 스트림에서 수신한 MarketEvent를 1분 캔들로 집계하고,
//! Z-Score 기반 진입/청산 시그널을 실시간으로 감지합니다.
//! 틱 수신 시 즉시 시그널을 평가하고, 분 완결 시에는 통계만 갱신합니다.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive as _;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, trace, warn};

use arb_exchange::{MarketData, MarketEvent, MarketStream};
use arb_forex::ForexCache;

use crate::common::candle_fetcher::fetch_all_candles;
use crate::common::convert::truncate_to_minute;
use crate::error::StrategyError;
use crate::output::summary::SessionSummary;
use crate::output::writer::{MinuteRecord, SessionWriter};
use crate::zscore::coin_selector::{CoinCandidate, CoinSelector};
use crate::zscore::config::ZScoreConfig;
use crate::zscore::pnl::ClosedPosition;
use crate::zscore::position::{self, PositionManager, VirtualPosition};
use crate::zscore::signal::{self, Signal};
use crate::zscore::spread::SpreadCalculator;

/// 분 완결 시 반환되는 데이터 (코인별 Upbit close, 코인별 Bybit close).
type MinuteCloses = (
    HashMap<String, Option<Decimal>>,
    HashMap<String, Option<Decimal>>,
);

/// 코인별 현재 분의 캔들 빌더.
#[derive(Debug)]
struct MinuteCandleBuilder {
    /// 현재 분의 시작 시간.
    current_minute: Option<DateTime<Utc>>,
    /// 코인별 Upbit 마지막 체결가.
    upbit_last_trade: HashMap<String, Decimal>,
    /// 코인별 Bybit best bid.
    bybit_last_bid: HashMap<String, Decimal>,
}

impl MinuteCandleBuilder {
    fn new() -> Self {
        Self {
            current_minute: None,
            upbit_last_trade: HashMap::new(),
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
                upbit_coins = ?coins_with_upbit,
                bybit_coins = ?coins_with_bybit,
                "분 캔들 완결: 코인별 close 데이터 요약"
            );
        }

        (upbit_closes, bybit_closes)
    }

    /// 새 분으로 전환합니다.
    fn start_new_minute(&mut self, minute: DateTime<Utc>) {
        self.current_minute = Some(truncate_to_minute(minute));
        self.upbit_last_trade.clear();
        self.bybit_last_bid.clear();
    }

    /// Upbit Trade 이벤트를 처리합니다.
    ///
    /// KRW-USDT 마켓은 무시합니다 (ForexCache 사용).
    fn on_upbit_trade(&mut self, market: &str, price: Decimal) {
        trace!(market = market, price = %price, "Upbit trade 수신");
        if market.starts_with("KRW-USDT") {
            // KRW-USDT는 ForexCache에서 관리하므로 무시
            return;
        }
        if let Some(coin) = market.strip_prefix("KRW-") {
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

/// 코인 diff 결과.
///
/// 현재 목록과 새 후보 목록을 비교하여 추가/제거/유지를 분류합니다.
struct CoinDiff {
    /// 새로 추가할 코인 (새 목록에만 있는 코인).
    to_add: Vec<String>,
    /// 제거할 코인 (현재 목록에만 있고 포지션 없는 코인).
    to_remove: Vec<String>,
    /// 유지할 코인 (현재 목록에만 있지만 포지션 보유 중).
    to_keep_with_position: Vec<String>,
}

/// 현재 코인 목록과 새 후보를 비교하여 diff를 계산합니다.
///
/// # 인자
///
/// * `current` - 현재 감시 중인 코인 목록
/// * `new_candidates` - 새로 선택된 코인 후보
/// * `position_mgr` - 포지션 매니저 (오픈 포지션 확인용)
///
/// # 반환값
///
/// 추가/제거/유지 분류 결과
fn diff_coins(
    current: &[String],
    new_candidates: &[CoinCandidate],
    position_mgr: &PositionManager,
) -> CoinDiff {
    use std::collections::HashSet;

    let current_set: HashSet<&str> = current.iter().map(|s| s.as_str()).collect();
    let new_set: HashSet<&str> = new_candidates.iter().map(|c| c.coin.as_str()).collect();

    // 새 목록에만 있는 코인 -> 추가
    let to_add: Vec<String> = new_set
        .difference(&current_set)
        .map(|s| s.to_string())
        .collect();

    // 현재 목록에만 있는 코인 -> 포지션 유무에 따라 제거 또는 유지
    let mut to_remove = Vec::new();
    let mut to_keep_with_position = Vec::new();

    for coin in current_set.difference(&new_set) {
        if position_mgr.has_position(coin) {
            to_keep_with_position.push(coin.to_string());
        } else {
            to_remove.push(coin.to_string());
        }
    }

    CoinDiff {
        to_add,
        to_remove,
        to_keep_with_position,
    }
}

/// 실시간 Z-Score 모니터.
pub struct ZScoreMonitor<U: MarketData + MarketStream, B: MarketData + MarketStream> {
    upbit: U,
    bybit: B,
    config: ZScoreConfig,
    forex_cache: Arc<ForexCache>,
}

impl<U: MarketData + MarketStream, B: MarketData + MarketStream> ZScoreMonitor<U, B> {
    /// 새 ZScoreMonitor를 생성합니다.
    pub fn new(upbit: U, bybit: B, config: ZScoreConfig, forex_cache: Arc<ForexCache>) -> Self {
        Self {
            upbit,
            bybit,
            config,
            forex_cache,
        }
    }

    /// 실시간 모니터링을 시작합니다.
    ///
    /// CancellationToken이 cancel되면 graceful shutdown합니다.
    /// `auto_select = true`이면 CoinSelector를 사용하여 자동 코인 선택 및
    /// 주기적 재선택을 수행합니다.
    pub async fn run(
        &self,
        cancel_token: CancellationToken,
    ) -> Result<Vec<ClosedPosition>, StrategyError> {
        self.config.validate()?;

        // 세션 시작 시각 및 환율 기록
        let session_start = Utc::now();

        // SessionWriter 초기화
        let mut session_writer = SessionWriter::new(&self.config.output)
            .map_err(|e| StrategyError::Config(format!("SessionWriter init failed: {e}")))?;

        // 환율 초기 로드
        self.forex_cache.refresh_if_expired().await.map_err(|e| {
            StrategyError::DataAlignment(format!("Initial forex refresh failed: {e}"))
        })?;
        info!(
            usd_krw = self.forex_cache.get_cached_rate().unwrap_or(0.0),
            "USD/KRW 환율 초기화 완료"
        );

        // 환율 갱신 task (10분 간격, cancel_token으로 종료)
        let forex_for_refresh = Arc::clone(&self.forex_cache);
        let forex_cancel = cancel_token.clone();
        let _forex_task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(600));
            loop {
                tokio::select! {
                    _ = forex_cancel.cancelled() => {
                        debug!("환율 갱신 task 종료");
                        break;
                    }
                    _ = interval.tick() => {
                        if let Err(e) = forex_for_refresh.refresh_if_expired().await {
                            warn!(error = %e, "USD/KRW 환율 갱신 실패, 캐시 값 유지");
                        }
                    }
                }
            }
        });

        // 1. 코인 목록 결정
        let mut current_coins: Vec<String> = if self.config.auto_select {
            info!("자동 코인 선택 활성화: 초기 코인 선택 중...");
            let selector = CoinSelector::new(&self.upbit, &self.bybit);
            let usd_krw_for_select = self.forex_cache.get_cached_rate().unwrap_or(0.0);
            let candidates = selector
                .select(
                    self.config.max_coins,
                    self.config.min_volume_1h_usdt,
                    &self.config.blacklist,
                    usd_krw_for_select,
                )
                .await?;

            if candidates.is_empty() {
                warn!("자동 선택 결과 후보 코인이 없습니다. 볼륨 조건을 확인하세요.");
            }

            let coins: Vec<String> = candidates.iter().map(|c| c.coin.clone()).collect();
            info!(coins = ?coins, "초기 코인 자동 선택 완료");
            coins
        } else {
            self.config.coins.clone()
        };

        // 시작 시점 환율 기록
        let usd_krw_start = self.forex_cache.get_cached_rate().unwrap_or(0.0);

        info!("실시간 모니터링 시작: 워밍업 데이터 로드 중...");

        // 2. 워밍업: REST API로 캔들 사전 로드
        let mut spread_calc = SpreadCalculator::new(&current_coins, self.config.window_size);
        self.warmup(&current_coins, &mut spread_calc).await?;

        // 워밍업 완료 후 요약 레코드 생성 및 기록
        let mut minute_records: Vec<MinuteRecord> = Vec::new();
        if let Some(ref mut writer) = session_writer {
            let warmup_records = Self::generate_warmup_records(
                &spread_calc,
                &current_coins,
                &self.config,
                &self.forex_cache,
            );
            minute_records.extend(warmup_records.iter().cloned());
            if let Err(e) = writer.append_minutes_batch(&warmup_records) {
                warn!(error = %e, "워밍업 요약 기록 실패");
            }
        }

        info!("워밍업 완료. WebSocket 연결 중...");

        // 3. WebSocket 구독 (KRW-USDT 제거)
        let upbit_markets: Vec<String> = current_coins.iter().map(|c| format!("KRW-{c}")).collect();
        let bybit_markets: Vec<String> = current_coins.iter().map(|c| format!("{c}USDT")).collect();

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

        // 4. 이벤트 루프
        let mut position_mgr = PositionManager::new();
        let mut candle_builder = MinuteCandleBuilder::new();
        let mut minute_timer = tokio::time::interval(Duration::from_secs(60));
        let mut trades: Vec<ClosedPosition> = Vec::new();

        // heartbeat 관련 상태
        let mut total_event_count: u64 = 0;
        let mut heartbeat_timer = tokio::time::interval(Duration::from_secs(300));

        // 재선택 타이머 (auto_select=true일 때만 사용)
        let reselect_interval = Duration::from_secs(self.config.reselect_interval_min * 60);
        let mut reselect_timer = tokio::time::interval(reselect_interval);
        // 첫 tick 소모 (interval은 즉시 첫 번째 tick을 발생시킴)
        reselect_timer.tick().await;

        // 탈락 코인 TTL 추적: 탈락 시각 기록
        let mut dropped_at: HashMap<String, DateTime<Utc>> = HashMap::new();

        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    info!("종료 요청 수신. 모니터링 종료 중...");
                    break;
                }
                Some(event) = upbit_rx.recv() => {
                    total_event_count += 1;
                    Self::handle_event(
                        &self.config,
                        &event,
                        &mut candle_builder,
                        &mut spread_calc,
                        &mut position_mgr,
                        &mut trades,
                        &current_coins,
                        &self.forex_cache,
                        &mut session_writer,
                        &mut minute_records,
                    )?;
                }
                Some(event) = bybit_rx.recv() => {
                    total_event_count += 1;
                    Self::handle_event(
                        &self.config,
                        &event,
                        &mut candle_builder,
                        &mut spread_calc,
                        &mut position_mgr,
                        &mut trades,
                        &current_coins,
                        &self.forex_cache,
                        &mut session_writer,
                        &mut minute_records,
                    )?;
                }
                _ = minute_timer.tick() => {
                    // 빈 분 감지: forward-fill 트리거
                    let now = Utc::now();
                    if candle_builder.is_new_minute(now) {
                        Self::finalize_and_process(
                            &self.config,
                            &mut candle_builder,
                            &mut spread_calc,
                            &mut position_mgr,
                            &mut trades,
                            now,
                            &current_coins,
                            &self.forex_cache,
                            &mut session_writer,
                            &mut minute_records,
                        )?;
                    }
                }
                _ = reselect_timer.tick(), if self.config.auto_select => {
                    // 자동 코인 재선택
                    info!("코인 재선택 시작...");
                    let selector = CoinSelector::new(&self.upbit, &self.bybit);
                    let usd_krw_for_reselect = self
                        .forex_cache
                        .get_cached_rate()
                        .unwrap_or(0.0);
                    let new_candidates = match selector
                        .select(
                            self.config.max_coins,
                            self.config.min_volume_1h_usdt,
                            &self.config.blacklist,
                            usd_krw_for_reselect,
                        )
                        .await
                    {
                        Ok(candidates) => candidates,
                        Err(e) => {
                            warn!(error = %e, "코인 재선택 실패, 이전 목록 유지");
                            continue;
                        }
                    };

                    let diff = diff_coins(&current_coins, &new_candidates, &position_mgr);

                    info!(
                        to_add = ?diff.to_add,
                        to_remove = ?diff.to_remove,
                        to_keep_with_position = ?diff.to_keep_with_position,
                        "코인 재선택 diff 결과"
                    );

                    // 제거 코인 (포지션 없음): 즉시 정리
                    for coin in &diff.to_remove {
                        spread_calc.remove_coin(coin);
                        dropped_at.remove(coin);

                        // WebSocket 구독 해제
                        let upbit_market = format!("KRW-{coin}");
                        let bybit_market = format!("{coin}USDT");
                        if let Err(e) = self
                            .upbit
                            .unsubscribe_markets(&[&upbit_market])
                            .await
                        {
                            warn!(
                                coin = coin.as_str(),
                                error = %e,
                                "Upbit 구독 해제 실패"
                            );
                        }
                        if let Err(e) = self
                            .bybit
                            .unsubscribe_markets(&[&bybit_market])
                            .await
                        {
                            warn!(
                                coin = coin.as_str(),
                                error = %e,
                                "Bybit 구독 해제 실패"
                            );
                        }

                        info!(coin = coin.as_str(), "코인 제거 완료");
                    }

                    // 탈락 + 포지션 코인: 탈락 시각 기록
                    for coin in &diff.to_keep_with_position {
                        dropped_at.entry(coin.clone()).or_insert(Utc::now());
                        info!(
                            coin = coin.as_str(),
                            "탈락 코인이지만 포지션 보유 중, 감시 유지"
                        );
                    }

                    // TTL 만료 체크
                    let ttl = chrono::Duration::hours(self.config.position_ttl_hours as i64);
                    let now = Utc::now();
                    let expired: Vec<String> = dropped_at
                        .iter()
                        .filter(|(_, time)| now - **time > ttl)
                        .map(|(coin, _)| coin.clone())
                        .collect();

                    for coin in &expired {
                        warn!(coin = coin.as_str(), "TTL 만료: 강제 청산");

                        // 강제 청산: 현재 가격으로 시장가 청산
                        let upbit_usdt = spread_calc
                            .upbit_window(coin)
                            .and_then(|w| w.last())
                            .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                            .unwrap_or(Decimal::ZERO);
                        let bybit_price = spread_calc
                            .bybit_window(coin)
                            .and_then(|w| w.last())
                            .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                            .unwrap_or(Decimal::ZERO);
                        let spread_pct = spread_calc.last_spread_pct(coin).unwrap_or(0.0);

                        // ForexCache에서 환율 조회
                        let exit_usd_krw = self.forex_cache.get_cached_rate().unwrap_or(0.0);

                        match position_mgr.close_position(
                            coin,
                            now,
                            upbit_usdt,
                            bybit_price,
                            exit_usd_krw,
                            spread_pct,
                            f64::NAN,
                            self.config.upbit_taker_fee,
                            self.config.bybit_taker_fee,
                            false,
                        ) {
                            Ok(closed) => {
                                info!(
                                    coin = coin.as_str(),
                                    net_pnl = %closed.net_pnl,
                                    "TTL 만료 강제 청산 완료"
                                );
                                if let Some(ref mut w) = session_writer
                                    && let Err(e) = w.append_trade(&closed)
                                {
                                    warn!(coin = coin.as_str(), error = %e, "TTL 만료 거래 CSV 기록 실패");
                                }
                                trades.push(closed);
                            }
                            Err(e) => {
                                warn!(
                                    coin = coin.as_str(),
                                    error = %e,
                                    "TTL 만료 강제 청산 실패"
                                );
                            }
                        }

                        // 데이터 정리
                        spread_calc.remove_coin(coin);
                        dropped_at.remove(coin);

                        // WebSocket 구독 해제
                        let upbit_market = format!("KRW-{coin}");
                        let bybit_market = format!("{coin}USDT");
                        self.upbit.unsubscribe_markets(&[&upbit_market]).await.ok();
                        self.bybit.unsubscribe_markets(&[&bybit_market]).await.ok();
                    }

                    // 추가 코인: 워밍업 후 구독
                    for coin in &diff.to_add {
                        match self
                            .warmup_single_coin(coin, &mut spread_calc)
                            .await
                        {
                            Ok(()) => {
                                // WebSocket 구독 추가
                                let upbit_market = format!("KRW-{coin}");
                                let bybit_market = format!("{coin}USDT");
                                if let Err(e) = self
                                    .upbit
                                    .subscribe_markets(&[&upbit_market])
                                    .await
                                {
                                    warn!(
                                        coin = coin.as_str(),
                                        error = %e,
                                        "Upbit 구독 추가 실패"
                                    );
                                    // 워밍업은 했지만 구독 실패 -> 데이터 정리
                                    spread_calc.remove_coin(coin);
                                    continue;
                                }
                                if let Err(e) = self
                                    .bybit
                                    .subscribe_markets(&[&bybit_market])
                                    .await
                                {
                                    warn!(
                                        coin = coin.as_str(),
                                        error = %e,
                                        "Bybit 구독 추가 실패"
                                    );
                                    // Upbit는 구독 성공했지만 Bybit 실패 -> 롤백
                                    self.upbit
                                        .unsubscribe_markets(&[&upbit_market])
                                        .await
                                        .ok();
                                    spread_calc.remove_coin(coin);
                                    continue;
                                }

                                info!(coin = coin.as_str(), "코인 추가 완료");
                            }
                            Err(e) => {
                                warn!(
                                    coin = coin.as_str(),
                                    error = %e,
                                    "코인 워밍업 실패, 건너뜀"
                                );
                                // 워밍업 중 add_coin으로 추가된 빈 윈도우 정리
                                spread_calc.remove_coin(coin);
                            }
                        }
                    }

                    // current_coins 업데이트: SpreadCalculator에 실제 존재하는 코인만
                    // (워밍업/구독 실패한 코인은 이미 remove_coin으로 제거됨)
                    current_coins = spread_calc
                        .active_coins()
                        .iter()
                        .map(|s| s.to_string())
                        .collect();
                    info!(coins = ?current_coins, "코인 목록 업데이트 완료");
                }
                _ = heartbeat_timer.tick() => {
                    // 5분마다 heartbeat 로그
                    info!(
                        total_events = total_event_count,
                        open_positions = position_mgr.open_count(),
                        total_trades = trades.len(),
                        coins = ?current_coins,
                        usd_krw = self.forex_cache.get_cached_rate().unwrap_or(0.0),
                        "[heartbeat] 실시간 모니터 상태"
                    );
                }
            }
        }

        // 정리: WebSocket 구독 해제
        self.upbit.unsubscribe().await.ok();
        self.bybit.unsubscribe().await.ok();

        // 세션 종료 시 JSON 일괄 저장 및 요약 출력
        if let Some(ref mut writer) = session_writer {
            let session_end = Utc::now();
            let usd_krw_end = self.forex_cache.get_cached_rate().unwrap_or(0.0);

            let summary = SessionSummary::calculate(
                &trades,
                session_start,
                session_end,
                &current_coins,
                usd_krw_start,
                usd_krw_end,
                total_event_count,
            );

            if let Err(e) = writer.finalize(&trades, &minute_records, &summary) {
                warn!(error = %e, "세션 파일 저장 실패");
            }

            // 콘솔에도 요약 출력
            println!("\n{}", summary.to_text());
        }

        info!(trades = trades.len(), "실시간 모니터링 종료");

        Ok(trades)
    }

    /// REST API로 전체 코인의 워밍업 데이터를 로드합니다.
    async fn warmup(
        &self,
        coins: &[String],
        spread_calc: &mut SpreadCalculator,
    ) -> Result<(), StrategyError> {
        for coin in coins {
            self.warmup_single_coin(coin, spread_calc).await?;
        }
        Ok(())
    }

    /// 단일 코인의 워밍업 데이터를 REST API로 로드합니다.
    ///
    /// SpreadCalculator에 해당 코인이 없으면 `add_coin`으로 추가합니다.
    /// 환율은 ForexCache의 일봉 데이터를 사용합니다.
    async fn warmup_single_coin(
        &self,
        coin: &str,
        spread_calc: &mut SpreadCalculator,
    ) -> Result<(), StrategyError> {
        let end_time = Utc::now();
        let window_size = self.config.window_size;

        // SpreadCalculator에 코인이 없으면 추가 (idempotent)
        spread_calc.add_coin(coin);

        debug!(
            coin = coin,
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

        // ForexCache에서 일봉 환율 조회
        let warmup_days = (window_size as i64 / (24 * 60)) + 2; // 여유 2일
        let from = end_time - chrono::Duration::days(warmup_days.max(2));
        let daily_rates = self
            .forex_cache
            .get_daily_rates(from, end_time)
            .await
            .map_err(|e| StrategyError::DataAlignment(format!("Forex warmup failed: {e}")))?;

        info!(
            coin = coin,
            upbit = upbit_candles.len(),
            bybit = bybit_candles.len(),
            daily_rates = daily_rates.len(),
            "워밍업 데이터 로드"
        );

        // 분 단위로 타임스탬프 정규화 (Upbit 초정밀도 / Bybit 밀리초 정밀도 정렬)
        let upbit_map: std::collections::HashMap<DateTime<Utc>, Decimal> = upbit_candles
            .iter()
            .map(|c| (truncate_to_minute(c.timestamp), c.close))
            .collect();
        let bybit_map: std::collections::HashMap<DateTime<Utc>, Decimal> = bybit_candles
            .iter()
            .map(|c| (truncate_to_minute(c.timestamp), c.close))
            .collect();

        // 일봉 환율을 날짜별 맵으로 변환
        let daily_rate_map: std::collections::HashMap<chrono::NaiveDate, f64> = daily_rates
            .iter()
            .map(|(dt, rate)| (dt.date_naive(), *rate))
            .collect();

        // 2개 소스의 공통 시간 범위 계산 (환율은 일봉이므로 제외)
        let common_start = [
            upbit_map.keys().min().copied(),
            bybit_map.keys().min().copied(),
        ]
        .iter()
        .filter_map(|t| *t)
        .max();

        // 합집합 타임스탬프 (공통 범위 내, 정규화된 분 단위)
        use std::collections::BTreeSet;
        let mut timestamps = BTreeSet::new();
        for ts in upbit_map.keys() {
            timestamps.insert(*ts);
        }
        for ts in bybit_map.keys() {
            timestamps.insert(*ts);
        }

        // 공통 범위 이전의 타임스탬프 제거
        let pre_filter_count = timestamps.len();
        if let Some(start) = common_start {
            timestamps.retain(|ts| *ts >= start);
        }

        debug!(
            coin = coin,
            total_timestamps = pre_filter_count,
            common_range_timestamps = timestamps.len(),
            trimmed = pre_filter_count - timestamps.len(),
            upbit_candles = upbit_map.len(),
            bybit_candles = bybit_map.len(),
            daily_rates = daily_rate_map.len(),
            common_start = ?common_start,
            "워밍업 타임스탬프 정규화 완료"
        );

        // 환율 forward-fill: 각 분봉 timestamp의 날짜에 해당하는 일봉 환율 사용
        let mut last_usd_krw: f64 = daily_rates.last().map(|(_, r)| *r).unwrap_or(0.0);

        for ts in &timestamps {
            // 해당 날짜의 일봉 환율이 있으면 갱신
            let date = ts.date_naive();
            if let Some(&rate) = daily_rate_map.get(&date) {
                last_usd_krw = rate;
            }

            spread_calc.update(
                coin,
                *ts,
                upbit_map.get(ts).copied(),
                last_usd_krw,
                bybit_map.get(ts).copied(),
            )?;
        }

        debug!(
            coin = coin,
            is_ready = spread_calc.is_ready(coin),
            "워밍업 완료"
        );

        Ok(())
    }

    /// MarketEvent를 처리합니다.
    #[allow(clippy::too_many_arguments)]
    fn handle_event(
        config: &ZScoreConfig,
        event: &MarketEvent,
        candle_builder: &mut MinuteCandleBuilder,
        spread_calc: &mut SpreadCalculator,
        position_mgr: &mut PositionManager,
        trades: &mut Vec<ClosedPosition>,
        current_coins: &[String],
        forex_cache: &ForexCache,
        session_writer: &mut Option<SessionWriter>,
        minute_records: &mut Vec<MinuteRecord>,
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
                Self::finalize_and_process(
                    config,
                    candle_builder,
                    spread_calc,
                    position_mgr,
                    trades,
                    event_ts,
                    current_coins,
                    forex_cache,
                    session_writer,
                    minute_records,
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

        // 틱 시그널 평가: 해당 코인 추출 후 check_tick_signal 호출
        let coin = match event {
            MarketEvent::Trade { market, .. } => market.strip_prefix("KRW-").map(|s| s.to_string()),
            MarketEvent::BestQuote { market, .. } => {
                market.strip_suffix("USDT").map(|s| s.to_string())
            }
        };

        if let Some(coin) = coin
            && current_coins.iter().any(|c| c == &coin)
            && let Err(e) = Self::check_tick_signal(
                &coin,
                config,
                candle_builder,
                spread_calc,
                position_mgr,
                trades,
                forex_cache,
                session_writer,
            )
        {
            warn!(coin = coin.as_str(), error = %e, "틱 시그널 평가 실패");
        }

        Ok(())
    }

    /// 틱 수신 시 즉시 시그널을 평가합니다.
    #[allow(clippy::too_many_arguments)]
    fn check_tick_signal(
        coin: &str,
        config: &ZScoreConfig,
        candle_builder: &MinuteCandleBuilder,
        spread_calc: &SpreadCalculator,
        position_mgr: &mut PositionManager,
        trades: &mut Vec<ClosedPosition>,
        forex_cache: &ForexCache,
        session_writer: &mut Option<SessionWriter>,
    ) -> Result<(), StrategyError> {
        // 1. 양쪽 last_trade가 모두 있어야 함
        let upbit_krw = match candle_builder.upbit_last_trade.get(coin) {
            Some(price) => *price,
            None => return Ok(()), // 아직 데이터 부족
        };
        let bybit_usd = match candle_builder.bybit_last_bid.get(coin) {
            Some(price) => *price,
            None => return Ok(()),
        };
        let usd_krw = match forex_cache.get_cached_rate() {
            Some(rate) => rate,
            None => return Ok(()), // 환율 캐시 미초기화
        };

        // 2. 현재 스프레드 계산
        let upbit_krw_f64 = upbit_krw.to_f64().unwrap_or(0.0);
        let upbit_usd = upbit_krw_f64 / usd_krw;
        let bybit_f64 = bybit_usd.to_f64().unwrap_or(0.0);

        if upbit_usd == 0.0 {
            return Ok(());
        }

        let current_spread = (bybit_f64 - upbit_usd) / upbit_usd * 100.0;

        trace!(
            coin = coin,
            upbit_krw = upbit_krw_f64,
            upbit_usd = upbit_usd,
            bybit_usd = bybit_f64,
            usd_krw = usd_krw,
            spread_pct = current_spread,
            "틱 스프레드 계산"
        );

        // 3. 캐시된 mean/stddev 조회 (O(1))
        let (mean, stddev) = match spread_calc.cached_stats(coin) {
            Some(stats) => stats,
            None => return Ok(()), // 윈도우 미충족
        };

        trace!(
            coin = coin,
            mean = mean,
            stddev = stddev,
            spread = current_spread,
            z_approx = if stddev > 0.0 {
                (current_spread - mean) / stddev
            } else {
                0.0
            },
            "틱 시그널 평가 입력"
        );

        // 4. 시그널 평가
        let sig =
            signal::evaluate_tick_signal(coin, current_spread, mean, stddev, position_mgr, config)?;

        // 5. 시그널 처리
        match sig {
            Some(Signal::Enter {
                coin: c,
                z_score,
                spread_pct,
                expected_profit_pct,
            }) => {
                let size_usdt = config.total_capital_usdt * config.position_ratio;
                let upbit_price = Decimal::try_from(upbit_usd).unwrap_or(Decimal::ZERO);
                let bybit_price_dec = bybit_usd;

                let liq_price = position::calculate_liquidation_price(
                    bybit_price_dec,
                    config.leverage,
                    config.bybit_mmr,
                    config.bybit_taker_fee,
                );

                let pos = VirtualPosition {
                    coin: c.clone(),
                    entry_time: Utc::now(),
                    upbit_entry_price: upbit_price,
                    bybit_entry_price: bybit_price_dec,
                    bybit_liquidation_price: liq_price,
                    entry_usd_krw: usd_krw,
                    entry_spread_pct: spread_pct,
                    entry_z_score: z_score,
                    size_usdt,
                };

                info!(
                    coin = c.as_str(),
                    z_score = z_score,
                    spread_pct = spread_pct,
                    expected_profit = expected_profit_pct,
                    "[틱] 진입 시그널"
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
                info!(
                    coin = c.as_str(),
                    z_score = z_score,
                    spread_pct = spread_pct,
                    "[틱] 청산 시그널"
                );

                let upbit_price = Decimal::try_from(upbit_usd).unwrap_or(Decimal::ZERO);

                match position_mgr.close_position(
                    &c,
                    Utc::now(),
                    upbit_price,
                    bybit_usd,
                    usd_krw,
                    spread_pct,
                    z_score,
                    config.upbit_taker_fee,
                    config.bybit_taker_fee,
                    false,
                ) {
                    Ok(closed) => {
                        info!(coin = c.as_str(), net_pnl = %closed.net_pnl, "[틱] 포지션 청산 완료");
                        if let Some(w) = session_writer
                            && let Err(e) = w.append_trade(&closed)
                        {
                            warn!(coin = c.as_str(), error = %e, "거래 내역 CSV 기록 실패");
                        }
                        trades.push(closed);
                    }
                    Err(e) => {
                        warn!(coin = c.as_str(), error = %e, "포지션 청산 실패");
                    }
                }
            }
            None => {}
        }

        Ok(())
    }

    /// 현재 분을 완결하고 통계를 갱신합니다.
    ///
    /// 시그널 평가는 틱에서 처리하므로, 여기서는 SpreadCalculator 업데이트와
    /// Liquidation 체크만 수행합니다.
    #[allow(clippy::too_many_arguments)]
    fn finalize_and_process(
        config: &ZScoreConfig,
        candle_builder: &mut MinuteCandleBuilder,
        spread_calc: &mut SpreadCalculator,
        position_mgr: &mut PositionManager,
        trades: &mut Vec<ClosedPosition>,
        new_minute_ts: DateTime<Utc>,
        current_coins: &[String],
        forex_cache: &ForexCache,
        session_writer: &mut Option<SessionWriter>,
        minute_records: &mut Vec<MinuteRecord>,
    ) -> Result<(), StrategyError> {
        let ts = candle_builder
            .current_minute
            .unwrap_or_else(|| truncate_to_minute(new_minute_ts));

        debug!(
            timestamp = %ts,
            coins = ?current_coins,
            "분 완결 시작: 통계 갱신"
        );

        let (upbit_closes, bybit_closes) = candle_builder.finalize_minute(current_coins);

        // ForexCache에서 환율 가져오기
        let usd_krw: f64 = forex_cache.get_cached_rate().unwrap_or(0.0);

        for coin in current_coins {
            let upbit_close = upbit_closes.get(coin).copied().flatten();
            let bybit_close = bybit_closes.get(coin).copied().flatten();

            // usd_krw가 0이면 SpreadCalculator가 에러를 반환하므로 스킵
            if usd_krw == 0.0 {
                trace!(
                    coin = coin.as_str(),
                    "USD/KRW 환율 미수신, 스프레드 계산 스킵"
                );
                continue;
            }

            // SpreadCalculator 업데이트 (forward-fill 내장)
            spread_calc.update(coin, ts, upbit_close, usd_krw, bybit_close)?;

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
                let spread_pct = spread_calc.last_spread_pct(coin).unwrap_or(0.0);

                warn!(coin = coin.as_str(), "Bybit 강제 청산 발생");

                match position_mgr.close_position(
                    coin,
                    ts,
                    upbit_usdt,
                    bybit_price,
                    usd_krw,
                    spread_pct,
                    f64::NAN,
                    config.upbit_taker_fee,
                    config.bybit_taker_fee,
                    true,
                ) {
                    Ok(closed) => {
                        if let Some(w) = session_writer
                            && let Err(e) = w.append_trade(&closed)
                        {
                            warn!(coin = coin.as_str(), error = %e, "강제 청산 거래 CSV 기록 실패");
                        }
                        trades.push(closed);
                    }
                    Err(e) => {
                        warn!(coin = coin.as_str(), error = %e, "강제 청산 처리 실패");
                    }
                }
            }

            // 시그널 평가는 틱에서 처리 -- 통계 로그만 출력
            if let Some((mean, stddev)) = spread_calc.cached_stats(coin) {
                let last_spread = spread_calc.last_spread_pct(coin).unwrap_or(0.0);
                let z = if stddev >= config.min_stddev_threshold {
                    (last_spread - mean) / stddev
                } else {
                    0.0
                };

                let position_str = if position_mgr.has_position(coin) {
                    "OPEN"
                } else {
                    "NONE"
                };

                debug!(
                    coin = coin.as_str(),
                    spread_pct = last_spread,
                    mean = mean,
                    stddev = stddev,
                    z_score = z,
                    position = position_str,
                    "분 완결: 통계 갱신 완료 (시그널은 틱에서 처리)"
                );

                // MinuteRecord 생성 및 CSV 기록
                let upbit_close_f64 = spread_calc
                    .upbit_window(coin)
                    .and_then(|w| w.last())
                    .unwrap_or(0.0);
                let bybit_close_f64 = spread_calc
                    .bybit_window(coin)
                    .and_then(|w| w.last())
                    .unwrap_or(0.0);

                let record = MinuteRecord {
                    timestamp: ts.to_rfc3339(),
                    coin: coin.clone(),
                    upbit_close: upbit_close_f64,
                    bybit_close: bybit_close_f64,
                    usd_krw,
                    spread_pct: last_spread,
                    mean,
                    stddev,
                    z_score: z,
                    position: position_str.to_string(),
                    source: "live".to_string(),
                };

                minute_records.push(record.clone());
                if let Some(w) = session_writer
                    && let Err(e) = w.append_minute(&record)
                {
                    warn!(coin = coin.as_str(), error = %e, "분봉 통계 CSV 기록 실패");
                }
            }
        }

        candle_builder.start_new_minute(new_minute_ts);
        Ok(())
    }

    /// 워밍업 완료 후 각 코인의 현재 통계를 `MinuteRecord`로 생성합니다.
    fn generate_warmup_records(
        spread_calc: &SpreadCalculator,
        coins: &[String],
        config: &ZScoreConfig,
        forex_cache: &ForexCache,
    ) -> Vec<MinuteRecord> {
        let mut records = Vec::new();
        let usd_krw = forex_cache.get_cached_rate().unwrap_or(0.0);

        for coin in coins {
            if let Some((mean, stddev)) = spread_calc.cached_stats(coin) {
                let spread = spread_calc.last_spread_pct(coin).unwrap_or(0.0);
                let z = if stddev >= config.min_stddev_threshold {
                    (spread - mean) / stddev
                } else {
                    0.0
                };
                let upbit_close = spread_calc
                    .upbit_window(coin)
                    .and_then(|w| w.last())
                    .unwrap_or(0.0);
                let bybit_close = spread_calc
                    .bybit_window(coin)
                    .and_then(|w| w.last())
                    .unwrap_or(0.0);

                records.push(MinuteRecord {
                    timestamp: Utc::now().to_rfc3339(),
                    coin: coin.clone(),
                    upbit_close,
                    bybit_close,
                    usd_krw,
                    spread_pct: spread,
                    mean,
                    stddev,
                    z_score: z,
                    position: "NONE".to_string(),
                    source: "warmup".to_string(),
                });
            }
        }

        records
    }

    /// 설정에 접근합니다.
    pub fn config(&self) -> &ZScoreConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zscore::coin_selector::CoinCandidate;

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
        // KRW-USDT는 무시됨 (ForexCache 사용)
        builder.on_upbit_trade("KRW-USDT", Decimal::new(1380, 0));

        assert_eq!(
            builder.upbit_last_trade.get("BTC"),
            Some(&Decimal::new(138_000_000, 0))
        );
        // KRW-USDT는 저장되지 않음
        assert!(builder.upbit_last_trade.get("USDT").is_none());
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
        builder.on_bybit_best_quote("BTCUSDT", Decimal::new(100_050, 0));

        let coins = vec!["BTC".to_string()];
        let (upbit, bybit) = builder.finalize_minute(&coins);

        assert_eq!(upbit.get("BTC"), Some(&Some(Decimal::new(138_000_000, 0))));
        assert_eq!(bybit.get("BTC"), Some(&Some(Decimal::new(100_050, 0))));

        // finalize 후 내부 상태 클리어 확인
        assert!(builder.upbit_last_trade.is_empty());
        assert!(builder.bybit_last_bid.is_empty());
    }

    #[test]
    fn test_candle_builder_start_new_minute() {
        let mut builder = MinuteCandleBuilder::new();
        let ts = Utc::now();

        // 데이터 추가
        builder.on_upbit_trade("KRW-BTC", Decimal::new(138_000_000, 0));
        builder.on_bybit_best_quote("BTCUSDT", Decimal::new(100_050, 0));

        // 새 분으로 전환
        builder.start_new_minute(ts);

        // 이전 데이터가 클리어됨
        assert!(builder.upbit_last_trade.is_empty());
        assert!(builder.bybit_last_bid.is_empty());
        assert!(builder.current_minute.is_some());
    }

    #[test]
    fn test_candle_builder_krw_usdt_ignored() {
        let mut builder = MinuteCandleBuilder::new();

        // KRW-USDT 이벤트는 무시되어야 함
        builder.on_upbit_trade("KRW-USDT", Decimal::new(1380, 0));

        // upbit_last_trade에 저장되지 않음
        assert!(builder.upbit_last_trade.is_empty());
    }

    // --- diff_coins 테스트 ---

    #[test]
    fn test_diff_coins_with_position() {
        let mut pm = PositionManager::new();
        // XRP에 포지션 오픈
        pm.open_position(VirtualPosition {
            coin: "XRP".to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(1, 0),
            bybit_entry_price: Decimal::new(1, 0),
            bybit_liquidation_price: Decimal::new(2, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            size_usdt: Decimal::new(1000, 0),
        })
        .unwrap();

        let current = vec![
            "BTC".to_string(),
            "ETH".to_string(),
            "XRP".to_string(),
            "DOGE".to_string(),
        ];
        let new_candidates = vec![
            CoinCandidate {
                coin: "BTC".to_string(),
                volume_1h_usdt: 1_000_000.0,
                volatility_24h_pct: 5.0,
            },
            CoinCandidate {
                coin: "ETH".to_string(),
                volume_1h_usdt: 500_000.0,
                volatility_24h_pct: 4.0,
            },
            CoinCandidate {
                coin: "AVAX".to_string(),
                volume_1h_usdt: 300_000.0,
                volatility_24h_pct: 8.0,
            },
        ];

        let diff = diff_coins(&current, &new_candidates, &pm);

        // AVAX는 새로 추가
        assert!(diff.to_add.contains(&"AVAX".to_string()));

        // DOGE는 포지션 없으므로 제거
        assert!(diff.to_remove.contains(&"DOGE".to_string()));

        // XRP는 포지션 있으므로 유지
        assert!(diff.to_keep_with_position.contains(&"XRP".to_string()));

        // BTC, ETH는 양쪽 모두 있으므로 diff에 나타나지 않음
        assert!(!diff.to_add.contains(&"BTC".to_string()));
        assert!(!diff.to_add.contains(&"ETH".to_string()));
        assert!(!diff.to_remove.contains(&"BTC".to_string()));
        assert!(!diff.to_remove.contains(&"ETH".to_string()));
    }

    #[test]
    fn test_diff_coins_without_position() {
        let pm = PositionManager::new();

        let current = vec!["BTC".to_string(), "ETH".to_string()];
        let new_candidates = vec![CoinCandidate {
            coin: "SOL".to_string(),
            volume_1h_usdt: 500_000.0,
            volatility_24h_pct: 6.0,
        }];

        let diff = diff_coins(&current, &new_candidates, &pm);

        // SOL 추가
        assert!(diff.to_add.contains(&"SOL".to_string()));
        // BTC, ETH 포지션 없으므로 제거
        assert!(diff.to_remove.contains(&"BTC".to_string()));
        assert!(diff.to_remove.contains(&"ETH".to_string()));
        // 포지션 보유 중인 코인 없음
        assert!(diff.to_keep_with_position.is_empty());
    }

    #[test]
    fn test_diff_coins_no_change() {
        let pm = PositionManager::new();

        let current = vec!["BTC".to_string(), "ETH".to_string()];
        let new_candidates = vec![
            CoinCandidate {
                coin: "BTC".to_string(),
                volume_1h_usdt: 1_000_000.0,
                volatility_24h_pct: 5.0,
            },
            CoinCandidate {
                coin: "ETH".to_string(),
                volume_1h_usdt: 500_000.0,
                volatility_24h_pct: 4.0,
            },
        ];

        let diff = diff_coins(&current, &new_candidates, &pm);

        // 변경 없음
        assert!(diff.to_add.is_empty());
        assert!(diff.to_remove.is_empty());
        assert!(diff.to_keep_with_position.is_empty());
    }

    #[test]
    fn test_diff_coins_empty_new_candidates() {
        let pm = PositionManager::new();

        let current = vec!["BTC".to_string(), "ETH".to_string()];
        let new_candidates: Vec<CoinCandidate> = vec![];

        let diff = diff_coins(&current, &new_candidates, &pm);

        // 모든 코인이 제거 대상
        assert!(diff.to_add.is_empty());
        assert_eq!(diff.to_remove.len(), 2);
        assert!(diff.to_keep_with_position.is_empty());
    }

    #[test]
    fn test_diff_coins_empty_current() {
        let pm = PositionManager::new();

        let current: Vec<String> = vec![];
        let new_candidates = vec![
            CoinCandidate {
                coin: "BTC".to_string(),
                volume_1h_usdt: 1_000_000.0,
                volatility_24h_pct: 5.0,
            },
            CoinCandidate {
                coin: "ETH".to_string(),
                volume_1h_usdt: 500_000.0,
                volatility_24h_pct: 4.0,
            },
        ];

        let diff = diff_coins(&current, &new_candidates, &pm);

        // 모든 코인이 추가 대상
        assert_eq!(diff.to_add.len(), 2);
        assert!(diff.to_remove.is_empty());
        assert!(diff.to_keep_with_position.is_empty());
    }

    #[test]
    fn test_check_tick_signal_no_data() {
        use std::time::Duration;

        let config = ZScoreConfig::default();
        let candle_builder = MinuteCandleBuilder::new();
        let spread_calc = SpreadCalculator::new(&["BTC".to_string()], 10);
        let mut position_mgr = PositionManager::new();
        let mut trades = Vec::new();
        let forex_cache = ForexCache::new(Duration::from_secs(600));
        let mut session_writer: Option<SessionWriter> = None;
        // 캐시 비어있음 -> 즉시 Ok(()) 반환
        let result = ZScoreMonitor::<MockMarket, MockMarket>::check_tick_signal(
            "BTC",
            &config,
            &candle_builder,
            &spread_calc,
            &mut position_mgr,
            &mut trades,
            &forex_cache,
            &mut session_writer,
        );
        assert!(result.is_ok());
        assert!(trades.is_empty());
    }

    #[test]
    fn test_check_tick_signal_partial_data() {
        use std::time::Duration;

        let config = ZScoreConfig::default();
        let mut candle_builder = MinuteCandleBuilder::new();

        // Upbit 데이터만 있고 Bybit 없음
        candle_builder
            .upbit_last_trade
            .insert("BTC".to_string(), Decimal::new(138_000_000, 0));

        let spread_calc = SpreadCalculator::new(&["BTC".to_string()], 10);
        let mut position_mgr = PositionManager::new();
        let mut trades = Vec::new();
        let forex_cache = ForexCache::new(Duration::from_secs(600));
        forex_cache.update_cache_for_test(1450.0);
        let mut session_writer: Option<SessionWriter> = None;

        let result = ZScoreMonitor::<MockMarket, MockMarket>::check_tick_signal(
            "BTC",
            &config,
            &candle_builder,
            &spread_calc,
            &mut position_mgr,
            &mut trades,
            &forex_cache,
            &mut session_writer,
        );
        assert!(result.is_ok());
        assert!(trades.is_empty());
    }

    #[test]
    fn test_check_tick_signal_window_not_ready() {
        use std::time::Duration;

        let config = ZScoreConfig::default();
        let mut candle_builder = MinuteCandleBuilder::new();

        // 양쪽 데이터 있음
        candle_builder
            .upbit_last_trade
            .insert("BTC".to_string(), Decimal::new(138_000_000, 0));
        candle_builder
            .bybit_last_bid
            .insert("BTC".to_string(), Decimal::new(100_050, 0));

        // SpreadCalculator 윈도우 미충족 (window_size=10, 데이터 0개)
        let spread_calc = SpreadCalculator::new(&["BTC".to_string()], 10);
        let mut position_mgr = PositionManager::new();
        let mut trades = Vec::new();
        let forex_cache = ForexCache::new(Duration::from_secs(600));
        forex_cache.update_cache_for_test(1450.0);
        let mut session_writer: Option<SessionWriter> = None;

        let result = ZScoreMonitor::<MockMarket, MockMarket>::check_tick_signal(
            "BTC",
            &config,
            &candle_builder,
            &spread_calc,
            &mut position_mgr,
            &mut trades,
            &forex_cache,
            &mut session_writer,
        );
        assert!(result.is_ok());
        // 윈도우 미충족이므로 시그널 없음
        assert!(trades.is_empty());
    }

    /// 테스트용 mock MarketData + MarketStream 구현.
    struct MockMarket;

    impl MarketData for MockMarket {
        fn name(&self) -> &str {
            "mock"
        }
        async fn get_ticker(
            &self,
            _markets: &[&str],
        ) -> Result<Vec<arb_exchange::Ticker>, arb_exchange::ExchangeError> {
            unimplemented!()
        }
        async fn get_all_tickers(
            &self,
        ) -> Result<Vec<arb_exchange::Ticker>, arb_exchange::ExchangeError> {
            unimplemented!()
        }
        async fn get_orderbook(
            &self,
            _market: &str,
            _depth: Option<u32>,
        ) -> Result<arb_exchange::OrderBook, arb_exchange::ExchangeError> {
            unimplemented!()
        }
        async fn get_candles(
            &self,
            _market: &str,
            _interval: arb_exchange::CandleInterval,
            _count: u32,
        ) -> Result<Vec<arb_exchange::Candle>, arb_exchange::ExchangeError> {
            unimplemented!()
        }
        async fn get_candles_before(
            &self,
            _market: &str,
            _interval: arb_exchange::CandleInterval,
            _count: u32,
            _before: chrono::DateTime<chrono::Utc>,
        ) -> Result<Vec<arb_exchange::Candle>, arb_exchange::ExchangeError> {
            unimplemented!()
        }
        fn market_code(_base: &str, _quote: &str) -> String {
            unimplemented!()
        }
    }

    #[async_trait::async_trait]
    impl MarketStream for MockMarket {
        fn stream_name(&self) -> &str {
            "mock"
        }
        async fn subscribe(
            &self,
            _markets: &[&str],
        ) -> Result<tokio::sync::mpsc::Receiver<MarketEvent>, arb_exchange::ExchangeError> {
            unimplemented!()
        }
        async fn unsubscribe(&self) -> Result<(), arb_exchange::ExchangeError> {
            unimplemented!()
        }
    }
}
