//! 실시간 Z-Score 모니터링.
//!
//! WebSocket 스트림에서 수신한 MarketEvent를 1분 캔들로 집계하고,
//! Z-Score 기반 진입/청산 시그널을 실시간으로 감지합니다.
//! 틱 수신 시 즉시 시그널을 평가하고, 분 완결 시에는 통계만 갱신합니다.
//!
//! ## 아키텍처 (0007 EVENT_LOOP_ASYNC_DECOUPLING)
//!
//! select! 루프는 이벤트 소비 전용으로, 무거운 REST 호출(오더북 조회,
//! 코인 재선택 워밍업)은 `tokio::spawn`으로 분리하여 채널 오버플로우를 방지합니다.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
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
use crate::output::summary::MonitoringCounters;
use crate::output::summary::SessionSummary;
use crate::output::writer::{MinuteRecord, SessionWriter};
use crate::zscore::coin_selector::{CoinCandidate, CoinSelector};
use crate::zscore::config::ZScoreConfig;
use crate::zscore::orderbook;
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

/// 재선택 task의 결과.
///
/// select! 루프에서 수신하여 `current_coins`와 `dropped_at`을 갱신합니다.
struct ReselectionResult {
    /// 갱신된 코인 목록.
    new_coins: Vec<String>,
    /// 탈락 코인별 탈락 시각 (기존 dropped_at에 merge).
    dropped_at_updates: HashMap<String, DateTime<Utc>>,
    /// 제거된 코인 (dropped_at에서 삭제 대상).
    removed_coins: Vec<String>,
}

/// 실시간 Z-Score 모니터.
///
/// `tokio::spawn`으로 REST 호출을 분리하기 위해 필드를 Arc로 래핑합니다.
pub struct ZScoreMonitor<U, B>
where
    U: MarketData + MarketStream + Send + Sync + 'static,
    B: MarketData + MarketStream + Send + Sync + 'static,
{
    upbit: Arc<U>,
    bybit: Arc<B>,
    config: Arc<ZScoreConfig>,
    forex_cache: Arc<ForexCache>,
}

impl<U, B> ZScoreMonitor<U, B>
where
    U: MarketData + MarketStream + Send + Sync + 'static,
    B: MarketData + MarketStream + Send + Sync + 'static,
{
    /// 새 ZScoreMonitor를 생성합니다.
    ///
    /// 기존 값 타입 파라미터를 받아 내부에서 Arc로 감쌉니다.
    pub fn new(upbit: U, bybit: B, config: ZScoreConfig, forex_cache: Arc<ForexCache>) -> Self {
        Self {
            upbit: Arc::new(upbit),
            bybit: Arc::new(bybit),
            config: Arc::new(config),
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

        // SessionWriter 초기화 (로컬 변수로 시작, 이후 Arc 래핑)
        let session_writer_local = SessionWriter::new(&self.config.output)
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
            let selector = CoinSelector::new(self.upbit.as_ref(), self.bybit.as_ref());
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

        // 2. 워밍업: REST API로 캔들 사전 로드 (로컬 변수)
        let mut spread_calc_local = SpreadCalculator::new(&current_coins, self.config.window_size);
        Self::warmup(
            self.upbit.as_ref(),
            self.bybit.as_ref(),
            &self.config,
            &self.forex_cache,
            &current_coins,
            &mut spread_calc_local,
        )
        .await?;

        // 워밍업 완료 후 요약 레코드 생성 및 기록
        let mut minute_records: Vec<MinuteRecord> = Vec::new();
        {
            let warmup_records = Self::generate_warmup_records(
                &spread_calc_local,
                &current_coins,
                &self.config,
                &self.forex_cache,
            );
            minute_records.extend(warmup_records.iter().cloned());
            // 워밍업 기록은 Arc 래핑 후에 session_writer에 기록
        }

        // OrderBookCache 로컬 변수로 프리페치
        let mut ob_cache_local = orderbook::OrderBookCache::new();
        let mut counters_local = MonitoringCounters::default();

        // 워밍업 완료 후 오더북 프리페치
        for coin in &current_coins {
            let upbit_market = format!("KRW-{coin}");
            let bybit_market = format!("{coin}USDT");
            if let Ok(ob) = self.upbit.get_orderbook(&upbit_market, Some(15)).await {
                ob_cache_local.update(orderbook::Exchange::Upbit, coin, ob);
                counters_local.orderbook_fetch_count += 1;
            }
            if let Ok(ob) = self.bybit.get_orderbook(&bybit_market, Some(25)).await {
                ob_cache_local.update(orderbook::Exchange::Bybit, coin, ob);
                counters_local.orderbook_fetch_count += 1;
            }
        }

        info!("워밍업 완료. WebSocket 연결 중...");

        // 3. WebSocket 구독
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

        // 4. 공유 상태를 Arc로 래핑
        let position_mgr = Arc::new(tokio::sync::Mutex::new(PositionManager::new()));
        let trades = Arc::new(tokio::sync::Mutex::new(Vec::<ClosedPosition>::new()));
        let ob_cache = orderbook::SharedObCache::new();
        // 프리페치 데이터를 SharedObCache에 복사
        for coin in &current_coins {
            if let Some(cached) = ob_cache_local.get(orderbook::Exchange::Upbit, coin) {
                let mut data = ob_cache.data.write().await;
                data.update(orderbook::Exchange::Upbit, coin, cached.orderbook.clone());
            }
            if let Some(cached) = ob_cache_local.get(orderbook::Exchange::Bybit, coin) {
                let mut data = ob_cache.data.write().await;
                data.update(orderbook::Exchange::Bybit, coin, cached.orderbook.clone());
            }
        }
        let counters = Arc::new(std::sync::Mutex::new(counters_local));
        let session_writer = Arc::new(tokio::sync::Mutex::new(session_writer_local));
        let spread_calc = Arc::new(tokio::sync::RwLock::new(spread_calc_local));
        let total_event_count = Arc::new(AtomicU64::new(0));

        // 워밍업 레코드를 session_writer에 기록
        {
            let mut sw = session_writer.lock().await;
            if let Some(ref mut writer) = *sw {
                let warmup_records = &minute_records;
                if let Err(e) = writer.append_minutes_batch(warmup_records) {
                    warn!(error = %e, "워밍업 요약 기록 실패");
                }
            }
        }

        // 이벤트 루프용 로컬 변수
        let mut candle_builder = MinuteCandleBuilder::new();
        let mut minute_timer = tokio::time::interval(Duration::from_secs(60));

        // heartbeat 관련 상태
        let mut heartbeat_timer = tokio::time::interval(Duration::from_secs(300));

        // 재선택 타이머 (auto_select=true일 때만 사용)
        let reselect_interval = Duration::from_secs(self.config.reselect_interval_min * 60);
        let mut reselect_timer = tokio::time::interval(reselect_interval);
        // 첫 tick 소모 (interval은 즉시 첫 번째 tick을 발생시킴)
        reselect_timer.tick().await;

        // 탈락 코인 TTL 추적: 탈락 시각 기록
        let mut dropped_at: HashMap<String, DateTime<Utc>> = HashMap::new();

        // 재선택 상태 관리
        let (reselect_tx, mut reselect_rx) = tokio::sync::mpsc::channel::<ReselectionResult>(1);
        let mut reselecting = false;

        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    info!("종료 요청 수신. 모니터링 종료 중...");
                    break;
                }
                Some(event) = upbit_rx.recv() => {
                    total_event_count.fetch_add(1, Ordering::Relaxed);
                    // 캔들 업데이트 (가벼운 동기 작업)
                    Self::update_candle_and_spread(
                        &event,
                        &mut candle_builder,
                        &spread_calc,
                        &self.config,
                        &current_coins,
                        &self.forex_cache,
                        &session_writer,
                        &mut minute_records,
                        &position_mgr,
                        &trades,
                    ).await;
                    // check_tick_signal을 tokio::spawn으로 분리
                    Self::maybe_spawn_tick_signal(
                        &event,
                        &current_coins,
                        &candle_builder,
                        &spread_calc,
                        &self.config,
                        &self.forex_cache,
                        &position_mgr,
                        &trades,
                        &ob_cache,
                        &counters,
                        &session_writer,
                        &self.upbit,
                        &self.bybit,
                    ).await;
                }
                Some(event) = bybit_rx.recv() => {
                    total_event_count.fetch_add(1, Ordering::Relaxed);
                    // 캔들 업데이트 (가벼운 동기 작업)
                    Self::update_candle_and_spread(
                        &event,
                        &mut candle_builder,
                        &spread_calc,
                        &self.config,
                        &current_coins,
                        &self.forex_cache,
                        &session_writer,
                        &mut minute_records,
                        &position_mgr,
                        &trades,
                    ).await;
                    // check_tick_signal을 tokio::spawn으로 분리
                    Self::maybe_spawn_tick_signal(
                        &event,
                        &current_coins,
                        &candle_builder,
                        &spread_calc,
                        &self.config,
                        &self.forex_cache,
                        &position_mgr,
                        &trades,
                        &ob_cache,
                        &counters,
                        &session_writer,
                        &self.upbit,
                        &self.bybit,
                    ).await;
                }
                _ = minute_timer.tick() => {
                    let now = Utc::now();
                    if candle_builder.is_new_minute(now)
                        && let Err(e) = Self::finalize_and_process(
                            &self.config,
                            &mut candle_builder,
                            &spread_calc,
                            &position_mgr,
                            &trades,
                            now,
                            &current_coins,
                            &self.forex_cache,
                            &session_writer,
                            &mut minute_records,
                        ).await
                    {
                        warn!(error = %e, "finalize_and_process 실패");
                    }

                    // TTL 만료 포지션 체크 (모든 코인)
                    if let Err(e) = Self::check_ttl_positions(
                        &self.config,
                        &position_mgr,
                        &trades,
                        &spread_calc,
                        &counters,
                        &dropped_at,
                        &self.forex_cache,
                        &session_writer,
                    ).await {
                        warn!(error = %e, "check_ttl_positions 실패");
                    }
                }
                _ = reselect_timer.tick(), if self.config.auto_select && !reselecting => {
                    reselecting = true;
                    info!("코인 재선택 시작...");
                    Self::spawn_reselection(
                        Arc::clone(&self.config),
                        Arc::clone(&self.upbit),
                        Arc::clone(&self.bybit),
                        Arc::clone(&self.forex_cache),
                        Arc::clone(&spread_calc),
                        ob_cache.clone(),
                        Arc::clone(&counters),
                        Arc::clone(&position_mgr),
                        current_coins.clone(),
                        dropped_at.clone(),
                        reselect_tx.clone(),
                    );
                }
                Some(result) = reselect_rx.recv() => {
                    current_coins = result.new_coins;
                    for (coin, ts) in result.dropped_at_updates {
                        dropped_at.entry(coin).or_insert(ts);
                    }
                    for coin in &result.removed_coins {
                        dropped_at.remove(coin);
                    }
                    reselecting = false;
                    info!(coins = ?current_coins, "코인 목록 업데이트 완료");
                }
                _ = heartbeat_timer.tick() => {
                    // 5분마다 heartbeat 로그 (lock 순서: position_mgr -> trades)
                    let open_count = {
                        let pm = position_mgr.lock().await;
                        pm.open_count()
                    };
                    let trade_count = {
                        let tr = trades.lock().await;
                        tr.len()
                    };
                    info!(
                        total_events = total_event_count.load(Ordering::Relaxed),
                        open_positions = open_count,
                        total_trades = trade_count,
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
        // 루프 종료 후이므로 spawn된 task가 없어 lock 경합 없음
        // std::sync::Mutex guard를 .await 전에 해제
        let counters_snapshot = counters.lock().unwrap().clone();
        {
            let trades_final = trades.lock().await;
            let mut sw = session_writer.lock().await;

            if let Some(ref mut writer) = *sw {
                let session_end = Utc::now();
                let usd_krw_end = self.forex_cache.get_cached_rate().unwrap_or(0.0);

                let summary = SessionSummary::calculate(
                    &trades_final,
                    session_start,
                    session_end,
                    &current_coins,
                    usd_krw_start,
                    usd_krw_end,
                    total_event_count.load(Ordering::Relaxed),
                    &counters_snapshot,
                );

                if let Err(e) = writer.finalize(&trades_final, &minute_records, &summary) {
                    warn!(error = %e, "세션 파일 저장 실패");
                }

                // 콘솔에도 요약 출력
                println!("\n{}", summary.to_text());
            }
        }

        let trades_final = trades.lock().await;
        info!(trades = trades_final.len(), "실시간 모니터링 종료");

        Ok(trades_final.clone())
    }

    /// REST API로 전체 코인의 워밍업 데이터를 로드합니다.
    async fn warmup(
        upbit: &U,
        bybit: &B,
        config: &ZScoreConfig,
        forex_cache: &ForexCache,
        coins: &[String],
        spread_calc: &mut SpreadCalculator,
    ) -> Result<(), StrategyError> {
        for coin in coins {
            Self::warmup_single_coin_standalone(
                upbit,
                bybit,
                config,
                forex_cache,
                coin,
                spread_calc,
            )
            .await?;
        }
        Ok(())
    }

    /// 단일 코인의 워밍업 데이터를 REST API로 로드합니다 (standalone).
    ///
    /// SpreadCalculator에 해당 코인이 없으면 `add_coin`으로 추가합니다.
    /// 환율은 ForexCache의 일봉 데이터를 사용합니다.
    async fn warmup_single_coin_standalone(
        upbit: &U,
        bybit: &B,
        config: &ZScoreConfig,
        forex_cache: &ForexCache,
        coin: &str,
        spread_calc: &mut SpreadCalculator,
    ) -> Result<(), StrategyError> {
        let end_time = Utc::now();
        let window_size = config.window_size;

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
            upbit,
            &upbit_market,
            config.candle_interval,
            window_size,
            end_time,
            200,
            Duration::from_millis(100),
        )
        .await?;

        let bybit_candles = fetch_all_candles(
            bybit,
            &bybit_market,
            config.candle_interval,
            window_size,
            end_time,
            1000,
            Duration::from_millis(10),
        )
        .await?;

        // ForexCache에서 일봉 환율 조회
        let warmup_days = (window_size as i64 / (24 * 60)) + 2; // 여유 2일
        let from = end_time - chrono::Duration::days(warmup_days.max(2));
        let daily_rates = forex_cache
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

    /// 단일 코인의 워밍업 (RwLock 래핑된 SpreadCalculator용).
    ///
    /// 재선택 task에서 사용합니다.
    async fn warmup_single_coin_with_lock(
        upbit: &U,
        bybit: &B,
        config: &ZScoreConfig,
        forex_cache: &ForexCache,
        coin: &str,
        spread_calc: &tokio::sync::RwLock<SpreadCalculator>,
    ) -> Result<(), StrategyError> {
        let end_time = Utc::now();
        let window_size = config.window_size;

        // SpreadCalculator에 코인 추가 (write lock)
        {
            let mut sc = spread_calc.write().await;
            sc.add_coin(coin);
        }

        debug!(
            coin = coin,
            window_size = window_size,
            "워밍업 시작: 캔들 데이터 로드"
        );

        let upbit_market = format!("KRW-{coin}");
        let bybit_market = format!("{coin}USDT");

        let upbit_candles = fetch_all_candles(
            upbit,
            &upbit_market,
            config.candle_interval,
            window_size,
            end_time,
            200,
            Duration::from_millis(100),
        )
        .await?;

        let bybit_candles = fetch_all_candles(
            bybit,
            &bybit_market,
            config.candle_interval,
            window_size,
            end_time,
            1000,
            Duration::from_millis(10),
        )
        .await?;

        // ForexCache에서 일봉 환율 조회
        let warmup_days = (window_size as i64 / (24 * 60)) + 2;
        let from = end_time - chrono::Duration::days(warmup_days.max(2));
        let daily_rates = forex_cache
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

        let upbit_map: std::collections::HashMap<DateTime<Utc>, Decimal> = upbit_candles
            .iter()
            .map(|c| (truncate_to_minute(c.timestamp), c.close))
            .collect();
        let bybit_map: std::collections::HashMap<DateTime<Utc>, Decimal> = bybit_candles
            .iter()
            .map(|c| (truncate_to_minute(c.timestamp), c.close))
            .collect();

        let daily_rate_map: std::collections::HashMap<chrono::NaiveDate, f64> = daily_rates
            .iter()
            .map(|(dt, rate)| (dt.date_naive(), *rate))
            .collect();

        let common_start = [
            upbit_map.keys().min().copied(),
            bybit_map.keys().min().copied(),
        ]
        .iter()
        .filter_map(|t| *t)
        .max();

        use std::collections::BTreeSet;
        let mut timestamps = BTreeSet::new();
        for ts in upbit_map.keys() {
            timestamps.insert(*ts);
        }
        for ts in bybit_map.keys() {
            timestamps.insert(*ts);
        }

        if let Some(start) = common_start {
            timestamps.retain(|ts| *ts >= start);
        }

        let mut last_usd_krw: f64 = daily_rates.last().map(|(_, r)| *r).unwrap_or(0.0);

        // write lock으로 SpreadCalculator 업데이트
        let mut sc = spread_calc.write().await;
        for ts in &timestamps {
            let date = ts.date_naive();
            if let Some(&rate) = daily_rate_map.get(&date) {
                last_usd_krw = rate;
            }

            sc.update(
                coin,
                *ts,
                upbit_map.get(ts).copied(),
                last_usd_krw,
                bybit_map.get(ts).copied(),
            )?;
        }

        debug!(coin = coin, is_ready = sc.is_ready(coin), "워밍업 완료");

        Ok(())
    }

    /// 이벤트에서 캔들 업데이트 + 분 경계 처리를 수행합니다 (가벼운 동기 작업).
    ///
    /// `candle_builder`는 select! 루프 로컬 변수로 유지합니다.
    /// 분 경계 감지 시 `finalize_and_process`를 호출합니다.
    #[allow(clippy::too_many_arguments)]
    async fn update_candle_and_spread(
        event: &MarketEvent,
        candle_builder: &mut MinuteCandleBuilder,
        spread_calc: &Arc<tokio::sync::RwLock<SpreadCalculator>>,
        config: &Arc<ZScoreConfig>,
        current_coins: &[String],
        forex_cache: &Arc<ForexCache>,
        session_writer: &Arc<tokio::sync::Mutex<Option<SessionWriter>>>,
        minute_records: &mut Vec<MinuteRecord>,
        position_mgr: &Arc<tokio::sync::Mutex<PositionManager>>,
        trades: &Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
    ) {
        let event_ts = match event {
            MarketEvent::Trade { timestamp, .. } => *timestamp,
            MarketEvent::BestQuote { timestamp, .. } => *timestamp,
        };

        // 분 경계 변경 시 이전 분 완결
        if candle_builder.is_new_minute(event_ts) {
            if candle_builder.current_minute.is_some() {
                debug!(
                    prev_minute = ?candle_builder.current_minute,
                    new_minute = %truncate_to_minute(event_ts),
                    "분 경계 변경 감지: 이전 분 완결 처리"
                );
                if let Err(e) = Self::finalize_and_process(
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
                )
                .await
                {
                    warn!(error = %e, "분 경계 finalize_and_process 실패");
                }
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
    }

    /// computing flag 체크 후 check_tick_signal을 tokio::spawn합니다.
    ///
    /// 캔들 빌더에서 Copy 값을 추출하고, spread_calc에서 read lock으로
    /// cached_stats를 가져온 후 spawn합니다. 이미 computing 중이면 즉시 리턴합니다.
    #[allow(clippy::too_many_arguments)]
    async fn maybe_spawn_tick_signal(
        event: &MarketEvent,
        current_coins: &[String],
        candle_builder: &MinuteCandleBuilder,
        spread_calc: &Arc<tokio::sync::RwLock<SpreadCalculator>>,
        config: &Arc<ZScoreConfig>,
        forex_cache: &Arc<ForexCache>,
        position_mgr: &Arc<tokio::sync::Mutex<PositionManager>>,
        trades: &Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
        ob_cache: &orderbook::SharedObCache,
        counters: &Arc<std::sync::Mutex<MonitoringCounters>>,
        session_writer: &Arc<tokio::sync::Mutex<Option<SessionWriter>>>,
        upbit_client: &Arc<U>,
        bybit_client: &Arc<B>,
    ) {
        // 1. 이벤트에서 코인 및 소스 거래소 추출
        let (coin, source_exchange) = match event {
            MarketEvent::Trade { market, .. } => {
                let coin = market.strip_prefix("KRW-").map(|s| s.to_string());
                (coin, orderbook::Exchange::Upbit)
            }
            MarketEvent::BestQuote { market, .. } => {
                let coin = market.strip_suffix("USDT").map(|s| s.to_string());
                (coin, orderbook::Exchange::Bybit)
            }
        };

        let Some(coin) = coin else { return };
        if !current_coins.iter().any(|c| c == &coin) {
            return;
        }

        // 2. 스냅샷 데이터 추출 (Copy)
        let upbit_price = match candle_builder.upbit_last_trade.get(&coin) {
            Some(p) => *p,
            None => return,
        };
        let bybit_price = match candle_builder.bybit_last_bid.get(&coin) {
            Some(p) => *p,
            None => return,
        };
        let usd_krw = match forex_cache.get_cached_rate() {
            Some(r) => r,
            None => return,
        };

        let upbit_f64 = upbit_price.to_f64().unwrap_or(0.0);
        let upbit_usd = upbit_f64 / usd_krw;
        let bybit_f64 = bybit_price.to_f64().unwrap_or(0.0);
        if upbit_usd == 0.0 {
            return;
        }
        let current_spread = (bybit_f64 - upbit_usd) / upbit_usd * 100.0;

        // 3. spread_calc read lock -> cached_stats Copy
        let (mean, stddev) = {
            let sc = spread_calc.read().await;
            match sc.cached_stats(&coin) {
                Some(s) => s,
                None => return,
            }
        };

        trace!(
            coin = coin.as_str(),
            spread_pct = current_spread,
            mean = mean,
            stddev = stddev,
            z_approx = if stddev > 0.0 {
                (current_spread - mean) / stddev
            } else {
                0.0
            },
            "틱 시그널 평가 입력"
        );

        // 4. computing flag check-and-set (atomic CAS)
        // 양쪽 거래소 모두 체크하여 같은 코인의 동시 spawn 방지
        let other_exchange = match source_exchange {
            orderbook::Exchange::Upbit => orderbook::Exchange::Bybit,
            orderbook::Exchange::Bybit => orderbook::Exchange::Upbit,
        };
        if ob_cache.computing.try_set_computing(source_exchange, &coin) {
            // 이미 같은 쪽에서 computing 중 → 스킵
            counters.lock().unwrap().dropped_tick_count += 1;
            return;
        }
        if ob_cache.computing.is_computing(other_exchange, &coin) {
            // 반대쪽에서 computing 중 → 스킵 (동일 코인 동시 진입/청산 방지)
            ob_cache.computing.clear_computing(source_exchange, &coin);
            counters.lock().unwrap().dropped_tick_count += 1;
            return;
        }

        // 5. tokio::spawn
        debug!(coin = coin.as_str(), "check_tick_signal task spawn");

        let coin_clone = coin.clone();
        let config = Arc::clone(config);
        let position_mgr = Arc::clone(position_mgr);
        let trades = Arc::clone(trades);
        let ob_cache = ob_cache.clone();
        let counters = Arc::clone(counters);
        let session_writer = Arc::clone(session_writer);
        let upbit_client = Arc::clone(upbit_client);
        let bybit_client = Arc::clone(bybit_client);

        tokio::spawn(async move {
            let result = Self::spawned_check_tick_signal(
                coin_clone.clone(),
                config,
                upbit_price,
                bybit_price,
                usd_krw,
                current_spread,
                mean,
                stddev,
                source_exchange,
                position_mgr,
                trades,
                ob_cache.clone(),
                counters,
                session_writer,
                upbit_client,
                bybit_client,
            )
            .await;

            // computing flag 해제 보장
            ob_cache
                .computing
                .clear_computing(source_exchange, &coin_clone);
            debug!(
                coin = coin_clone.as_str(),
                "check_tick_signal task 완료, computing flag 해제"
            );

            if let Err(e) = result {
                warn!(
                    coin = coin_clone.as_str(),
                    error = %e,
                    "check_tick_signal task 에러"
                );
            }
        });
    }

    /// spawned task에서 실행되는 틱 시그널 평가.
    ///
    /// 오더북 REST 조회, 청산/진입 시그널 평가를 수행합니다.
    /// computing flag는 호출자(maybe_spawn_tick_signal)에서 관리합니다.
    #[allow(clippy::too_many_arguments)]
    async fn spawned_check_tick_signal(
        coin: String,
        config: Arc<ZScoreConfig>,
        upbit_price: Decimal,
        bybit_price: Decimal,
        usd_krw: f64,
        current_spread: f64,
        mean: f64,
        stddev: f64,
        source_exchange: orderbook::Exchange,
        position_mgr: Arc<tokio::sync::Mutex<PositionManager>>,
        trades: Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
        ob_cache: orderbook::SharedObCache,
        counters: Arc<std::sync::Mutex<MonitoringCounters>>,
        session_writer: Arc<tokio::sync::Mutex<Option<SessionWriter>>>,
        upbit_client: Arc<U>,
        bybit_client: Arc<B>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let other_exchange = match source_exchange {
            orderbook::Exchange::Upbit => orderbook::Exchange::Bybit,
            orderbook::Exchange::Bybit => orderbook::Exchange::Upbit,
        };

        // 소스 거래소 오더북 REST 조회
        let market = match source_exchange {
            orderbook::Exchange::Upbit => format!("KRW-{coin}"),
            orderbook::Exchange::Bybit => format!("{coin}USDT"),
        };
        let depth = match source_exchange {
            orderbook::Exchange::Upbit => Some(15),
            orderbook::Exchange::Bybit => Some(25),
        };

        let ob_result = match source_exchange {
            orderbook::Exchange::Upbit => upbit_client.get_orderbook(&market, depth).await,
            orderbook::Exchange::Bybit => bybit_client.get_orderbook(&market, depth).await,
        };

        match ob_result {
            Ok(ob) => {
                let mut data = ob_cache.data.write().await;
                data.update(source_exchange, &coin, ob);
                drop(data);
                counters.lock().unwrap().orderbook_fetch_count += 1;
            }
            Err(e) => {
                warn!(coin = coin.as_str(), exchange = ?source_exchange, error = %e, "오더북 조회 실패");
                counters.lock().unwrap().orderbook_fetch_fail_count += 1;
                return Ok(());
            }
        }

        // 반대쪽 캐시 신선도 확인
        {
            let data = ob_cache.data.read().await;
            if !data.is_fresh(other_exchange, &coin, config.max_cache_age_sec) {
                counters.lock().unwrap().stale_cache_skip_count += 1;
                return Ok(());
            }
        }

        // USD 환산된 Upbit 가격
        let upbit_f64 = upbit_price.to_f64().unwrap_or(0.0);
        let upbit_usd = upbit_f64 / usd_krw;
        let upbit_usd_dec = Decimal::try_from(upbit_usd).unwrap_or(Decimal::ZERO);

        // 4. 청산 시그널 평가 (exit-first)
        {
            let pm = position_mgr.lock().await;
            let has_positions = pm.has_position(&coin);
            drop(pm);

            if let Some(Signal::Exit {
                coin: c,
                z_score,
                spread_pct: sp,
            }) = signal::evaluate_exit_signal(
                &coin,
                current_spread,
                mean,
                stddev,
                has_positions,
                &config,
            )? {
                info!(
                    coin = c.as_str(),
                    z_score = z_score,
                    spread_pct = sp,
                    "[틱] 청산 시그널"
                );

                // 오더북 기반 청산 안전 볼륨 계산
                let data = ob_cache.data.read().await;
                let upbit_cached = data.get(orderbook::Exchange::Upbit, &c);
                let bybit_cached = data.get(orderbook::Exchange::Bybit, &c);

                if let (Some(upbit_ob), Some(bybit_ob)) = (upbit_cached, bybit_cached) {
                    let upbit_bids = orderbook::levels_to_f64(&upbit_ob.orderbook, false);
                    let bybit_asks = orderbook::levels_to_f64(&bybit_ob.orderbook, true);
                    drop(data);

                    let exit_safe = orderbook::calculate_exit_safe_volume(
                        &upbit_bids,
                        &bybit_asks,
                        mean,
                        config.upbit_taker_fee.to_f64().unwrap_or(0.0),
                        config.bybit_taker_fee.to_f64().unwrap_or(0.0),
                        usd_krw,
                    );

                    // pm 락 내에서 청산 수행 → 결과만 수집 후 락 해제
                    let (closed_positions, partial_count) = {
                        let mut pm = position_mgr.lock().await;
                        let positions: Vec<(u64, Decimal, f64)> = pm
                            .open_positions
                            .get(c.as_str())
                            .map(|ps| {
                                ps.iter()
                                    .map(|p| {
                                        let profit_rate = (sp - p.entry_spread_pct)
                                            / p.size_usdt.to_f64().unwrap_or(1.0);
                                        (p.id, p.size_usdt, profit_rate)
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();
                        let mut sorted = positions;
                        sorted.sort_by(|a, b| {
                            b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal)
                        });

                        let mut remaining_safe_usdt =
                            exit_safe.map(|sv| sv.safe_volume_usdt).unwrap_or(0.0);
                        let mut closed_results: Vec<ClosedPosition> = Vec::new();
                        let mut partial_cnt: u64 = 0;

                        for (pid, size, _) in &sorted {
                            if remaining_safe_usdt <= 0.0 {
                                break;
                            }
                            let size_f64 = size.to_f64().unwrap_or(0.0);

                            if remaining_safe_usdt >= size_f64 {
                                // 전량 청산
                                remaining_safe_usdt -= size_f64;
                                match pm.close_position(
                                    &c,
                                    *pid,
                                    Utc::now(),
                                    upbit_usd_dec,
                                    bybit_price,
                                    usd_krw,
                                    sp,
                                    z_score,
                                    config.upbit_taker_fee,
                                    config.bybit_taker_fee,
                                    false,
                                ) {
                                    Ok(closed) => closed_results.push(closed),
                                    Err(e) => warn!(error = %e, "청산 실패"),
                                }
                            } else {
                                // 부분 청산
                                let partial = Decimal::try_from(remaining_safe_usdt)
                                    .unwrap_or(Decimal::ZERO);
                                remaining_safe_usdt = 0.0;
                                match pm.close_partial(
                                    &c,
                                    *pid,
                                    partial,
                                    upbit_usd_dec,
                                    bybit_price,
                                    usd_krw,
                                    sp,
                                    z_score,
                                    config.upbit_taker_fee,
                                    config.bybit_taker_fee,
                                    false,
                                ) {
                                    Ok((closed, _rem)) => {
                                        closed_results.push(closed);
                                        partial_cnt += 1;
                                    }
                                    Err(e) => warn!(error = %e, "부분 청산 실패"),
                                }
                            }
                        }
                        (closed_results, partial_cnt)
                    };
                    // pm 락 해제 후 trades → session_writer → counters 순서로 기록
                    for closed in &closed_positions {
                        trades.lock().await.push(closed.clone());
                        let mut sw = session_writer.lock().await;
                        if let Some(ref mut w) = *sw
                            && let Err(e) = w.append_trade(closed)
                        {
                            warn!(error = %e, "CSV 기록 실패");
                        }
                    }
                    if partial_count > 0 {
                        counters.lock().unwrap().partial_close_count += partial_count;
                    }
                } else {
                    drop(data);
                }
            }
        }

        // 5. 진입 시그널 평가
        {
            let pm = position_mgr.lock().await;
            let max_coin_capital = config.total_capital_usdt * config.max_position_ratio;
            let coin_used = pm.coin_used_capital(&coin);
            let open_count = pm.open_count();
            let last_entry = pm.last_entry_at(&coin);
            drop(pm);

            if let Some(Signal::Enter {
                coin: c,
                z_score,
                spread_pct: sp,
                expected_profit_pct,
            }) = signal::evaluate_entry_signal(
                &coin,
                current_spread,
                mean,
                stddev,
                coin_used,
                max_coin_capital,
                open_count,
                last_entry,
                &config,
            )? {
                // 오더북 기반 진입 안전 볼륨 계산
                let data = ob_cache.data.read().await;
                let upbit_cached = data.get(orderbook::Exchange::Upbit, &c);
                let bybit_cached = data.get(orderbook::Exchange::Bybit, &c);

                if let (Some(upbit_ob), Some(bybit_ob)) = (upbit_cached, bybit_cached) {
                    let upbit_asks = orderbook::levels_to_f64(&upbit_ob.orderbook, true);
                    let bybit_bids = orderbook::levels_to_f64(&bybit_ob.orderbook, false);
                    drop(data);

                    let entry_safe = orderbook::calculate_entry_safe_volume(
                        &upbit_asks,
                        &bybit_bids,
                        mean,
                        config.upbit_taker_fee.to_f64().unwrap_or(0.0),
                        config.bybit_taker_fee.to_f64().unwrap_or(0.0),
                        usd_krw,
                    );

                    match entry_safe {
                        Some(sv) => {
                            let volume_1h = config.min_volume_1h_usdt.to_f64().unwrap_or(50_000.0);
                            let ratio = orderbook::safe_volume_ratio_from_volume(volume_1h);

                            let max_coin_cap = (config.total_capital_usdt
                                * config.max_position_ratio)
                                .to_f64()
                                .unwrap_or(0.0);
                            // pm 락 → 자본 확인 + 포지션 오픈 → 즉시 해제
                            let entry_rejected = {
                                let mut pm = position_mgr.lock().await;
                                let used = pm.coin_used_capital(&c).to_f64().unwrap_or(0.0);
                                let remaining_cap = max_coin_cap - used;

                                let size_usdt_f64 =
                                    (sv.safe_volume_usdt * ratio).min(remaining_cap);

                                if size_usdt_f64 <= 5.0 {
                                    debug!(
                                        coin = c.as_str(),
                                        size_usdt = size_usdt_f64,
                                        "진입 거부: 안전 볼륨 최소 주문 크기 미달"
                                    );
                                    true
                                } else {
                                    let size_usdt = Decimal::try_from(size_usdt_f64)
                                        .unwrap_or(Decimal::ZERO);

                                    let liq_price = position::calculate_liquidation_price(
                                        bybit_price,
                                        config.leverage,
                                        config.bybit_mmr,
                                        config.bybit_taker_fee,
                                    );

                                    let pos = VirtualPosition {
                                        id: 0,
                                        coin: c.clone(),
                                        entry_time: Utc::now(),
                                        upbit_entry_price: upbit_usd_dec,
                                        bybit_entry_price: bybit_price,
                                        bybit_liquidation_price: liq_price,
                                        entry_usd_krw: usd_krw,
                                        entry_spread_pct: sp,
                                        entry_z_score: z_score,
                                        size_usdt,
                                    };

                                    info!(
                                        coin = c.as_str(),
                                        z_score = z_score,
                                        spread_pct = sp,
                                        expected_profit = expected_profit_pct,
                                        size_usdt = %size_usdt,
                                        "[틱] 진입 시그널 (오더북 기반 사이징)"
                                    );

                                    if let Err(e) = pm.open_position(pos) {
                                        warn!(
                                            coin = c.as_str(),
                                            error = %e,
                                            "포지션 오픈 실패"
                                        );
                                    }
                                    false
                                }
                            };
                            // pm 락 해제 후 counters 접근
                            if entry_rejected {
                                counters.lock().unwrap().entry_rejected_slippage_count += 1;
                            }
                        }
                        None => {
                            counters.lock().unwrap().entry_rejected_slippage_count += 1;
                            debug!(coin = c.as_str(), "진입 거부: 오더북 안전 볼륨 없음");
                        }
                    }
                } else {
                    drop(data);
                    counters.lock().unwrap().stale_cache_skip_count += 1;
                    debug!(coin = c.as_str(), "진입 거부: 오더북 캐시 없음");
                }
            }
        }

        Ok(())
    }

    /// TTL 만료 포지션을 체크하고 청산합니다.
    ///
    /// 탈락 코인 중 TTL이 만료된 포지션을 grace period에 따라 분할/전량 청산합니다.
    /// - 1단계 (TTL ~ TTL+grace): 전량 청산 시도
    /// - 2단계 (TTL+grace 초과): 슬리피지 무시 전량 강제 청산
    #[allow(clippy::too_many_arguments)]
    async fn check_ttl_positions(
        config: &ZScoreConfig,
        position_mgr: &tokio::sync::Mutex<PositionManager>,
        trades: &tokio::sync::Mutex<Vec<ClosedPosition>>,
        spread_calc: &tokio::sync::RwLock<SpreadCalculator>,
        counters: &std::sync::Mutex<MonitoringCounters>,
        dropped_at: &HashMap<String, DateTime<Utc>>,
        forex_cache: &ForexCache,
        session_writer: &tokio::sync::Mutex<Option<SessionWriter>>,
    ) -> Result<(), StrategyError> {
        let ttl = chrono::Duration::hours(config.position_ttl_hours as i64);
        let grace = chrono::Duration::hours(config.grace_period_hours as i64);
        let now = Utc::now();
        let usd_krw = forex_cache.get_cached_rate().unwrap_or(0.0);

        // 모든 열린 포지션의 코인 수집
        let coins_with_positions: Vec<String> = {
            let pm = position_mgr.lock().await;
            pm.open_positions.keys().cloned().collect()
        };

        for coin in &coins_with_positions {
            // 탈락 코인이 아니면 TTL 미적용
            let Some(drop_time) = dropped_at.get(coin) else {
                continue;
            };
            let elapsed = now - *drop_time;

            if elapsed <= ttl {
                continue;
            }

            let positions: Vec<(u64, Decimal)> = {
                let pm = position_mgr.lock().await;
                pm.open_positions
                    .get(coin.as_str())
                    .map(|ps| ps.iter().map(|p| (p.id, p.size_usdt)).collect())
                    .unwrap_or_default()
            };

            if positions.is_empty() {
                continue;
            }

            let (upbit_usdt, bybit_price_dec, spread_pct_val) = {
                let sc = spread_calc.read().await;
                let upbit_usdt = sc
                    .upbit_window(coin)
                    .and_then(|w| w.last())
                    .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                    .unwrap_or(Decimal::ZERO);
                let bybit_price_dec = sc
                    .bybit_window(coin)
                    .and_then(|w| w.last())
                    .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                    .unwrap_or(Decimal::ZERO);
                let spread_pct_val = sc.last_spread_pct(coin).unwrap_or(0.0);
                (upbit_usdt, bybit_price_dec, spread_pct_val)
            };

            if elapsed > ttl + grace {
                // 2단계: 전량 강제 청산 (슬리피지 무시)
                warn!(coin = coin.as_str(), "2단계 강제 청산: grace period 초과");
                let closed_positions: Vec<ClosedPosition> = {
                    let mut pm = position_mgr.lock().await;
                    let mut result = Vec::new();
                    for (pid, _size) in &positions {
                        match pm.close_position(
                            coin,
                            *pid,
                            now,
                            upbit_usdt,
                            bybit_price_dec,
                            usd_krw,
                            spread_pct_val,
                            f64::NAN,
                            config.upbit_taker_fee,
                            config.bybit_taker_fee,
                            true,
                        ) {
                            Ok(closed) => result.push(closed),
                            Err(e) => warn!(error = %e, "2단계 강제 청산 실패"),
                        }
                    }
                    result
                };
                // pm 락 해제 후 trades → session_writer → counters 순서로 기록
                for closed in &closed_positions {
                    trades.lock().await.push(closed.clone());
                    let mut sw = session_writer.lock().await;
                    if let Some(ref mut w) = *sw
                        && let Err(e) = w.append_trade(closed)
                    {
                        warn!(error = %e, "강제 청산 CSV 기록 실패");
                    }
                }
                counters.lock().unwrap().forced_liquidation_count +=
                    closed_positions.len() as u64;
            } else {
                // 1단계: 전량 청산 시도
                warn!(coin = coin.as_str(), "1단계 TTL 분할 청산 시도");
                let closed_positions: Vec<ClosedPosition> = {
                    let mut pm = position_mgr.lock().await;
                    let mut result = Vec::new();
                    for (pid, _size) in &positions {
                        match pm.close_position(
                            coin,
                            *pid,
                            now,
                            upbit_usdt,
                            bybit_price_dec,
                            usd_krw,
                            spread_pct_val,
                            f64::NAN,
                            config.upbit_taker_fee,
                            config.bybit_taker_fee,
                            false,
                        ) {
                            Ok(closed) => result.push(closed),
                            Err(e) => warn!(error = %e, "1단계 TTL 청산 실패"),
                        }
                    }
                    result
                };
                // pm 락 해제 후 trades → session_writer 순서로 기록
                for closed in &closed_positions {
                    trades.lock().await.push(closed.clone());
                    let mut sw = session_writer.lock().await;
                    if let Some(ref mut w) = *sw
                        && let Err(e) = w.append_trade(closed)
                    {
                        warn!(error = %e, "TTL 청산 CSV 기록 실패");
                    }
                }
            }
        }

        Ok(())
    }

    /// 현재 분을 완결하고 통계를 갱신합니다.
    ///
    /// 시그널 평가는 틱에서 처리하므로, 여기서는 SpreadCalculator 업데이트와
    /// Liquidation 체크만 수행합니다.
    #[allow(clippy::too_many_arguments)]
    async fn finalize_and_process(
        config: &ZScoreConfig,
        candle_builder: &mut MinuteCandleBuilder,
        spread_calc: &tokio::sync::RwLock<SpreadCalculator>,
        position_mgr: &tokio::sync::Mutex<PositionManager>,
        trades: &tokio::sync::Mutex<Vec<ClosedPosition>>,
        new_minute_ts: DateTime<Utc>,
        current_coins: &[String],
        forex_cache: &ForexCache,
        session_writer: &tokio::sync::Mutex<Option<SessionWriter>>,
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

            if usd_krw == 0.0 {
                trace!(
                    coin = coin.as_str(),
                    "USD/KRW 환율 미수신, 스프레드 계산 스킵"
                );
                continue;
            }

            // SpreadCalculator 업데이트 (write lock)
            {
                let mut sc = spread_calc.write().await;
                sc.update(coin, ts, upbit_close, usd_krw, bybit_close)?;
            }

            // Liquidation 체크: bybit_close가 없으면 spread_calc에서 조회
            let bybit_price_final = match bybit_close {
                Some(p) => Some(p),
                None => {
                    let sc = spread_calc.read().await;
                    sc.bybit_window(coin).and_then(|w| {
                        w.last()
                            .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                    })
                }
            };

            if let Some(bybit_price) = bybit_price_final {
                // lock 순서 준수: spread_calc 먼저 읽고 drop → position_mgr lock
                let (upbit_usdt_snapshot, spread_pct_snapshot) = {
                    let sc = spread_calc.read().await;
                    let u = sc
                        .upbit_window(coin)
                        .and_then(|w| w.last())
                        .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                        .unwrap_or(Decimal::ZERO);
                    let sp = sc.last_spread_pct(coin).unwrap_or(0.0);
                    (u, sp)
                };

                let mut pm = position_mgr.lock().await;
                let liquidated_ids = pm.check_liquidation(coin, bybit_price);
                if !liquidated_ids.is_empty() {
                    let upbit_usdt = upbit_usdt_snapshot;
                    let spread_pct_val = spread_pct_snapshot;

                    for pid in liquidated_ids {
                        warn!(
                            coin = coin.as_str(),
                            position_id = pid,
                            "Bybit 강제 청산 발생"
                        );

                        match pm.close_position(
                            coin,
                            pid,
                            ts,
                            upbit_usdt,
                            bybit_price,
                            usd_krw,
                            spread_pct_val,
                            f64::NAN,
                            config.upbit_taker_fee,
                            config.bybit_taker_fee,
                            true,
                        ) {
                            Ok(closed) => {
                                // lock 순서: trades -> session_writer
                                trades.lock().await.push(closed.clone());
                                let mut sw = session_writer.lock().await;
                                if let Some(ref mut w) = *sw
                                    && let Err(e) = w.append_trade(&closed)
                                {
                                    warn!(
                                        coin = coin.as_str(),
                                        error = %e,
                                        "강제 청산 거래 CSV 기록 실패"
                                    );
                                }
                            }
                            Err(e) => {
                                warn!(
                                    coin = coin.as_str(),
                                    error = %e,
                                    "강제 청산 처리 실패"
                                );
                            }
                        }
                    }
                }
                drop(pm);
            }

            // 통계 로그 + MinuteRecord 생성
            // lock 순서 준수: spread_calc → position_mgr 역순 방지
            // spread_calc read lock을 먼저 획득하고 값을 복사한 후 drop
            let stats_snapshot = {
                let sc = spread_calc.read().await;
                sc.cached_stats(coin).map(|(mean, stddev_val)| {
                    let last_spread = sc.last_spread_pct(coin).unwrap_or(0.0);
                    let upbit_close = sc.upbit_window(coin).and_then(|w| w.last()).unwrap_or(0.0);
                    let bybit_close = sc.bybit_window(coin).and_then(|w| w.last()).unwrap_or(0.0);
                    (mean, stddev_val, last_spread, upbit_close, bybit_close)
                })
            };

            if let Some((mean, stddev_val, last_spread, upbit_close_f64, bybit_close_f64)) = stats_snapshot {
                let z = if stddev_val >= config.min_stddev_threshold {
                    (last_spread - mean) / stddev_val
                } else {
                    0.0
                };

                let pm = position_mgr.lock().await;
                let position_str = if pm.has_position(coin) {
                    "OPEN"
                } else {
                    "NONE"
                };
                drop(pm);

                debug!(
                    coin = coin.as_str(),
                    spread_pct = last_spread,
                    mean = mean,
                    stddev = stddev_val,
                    z_score = z,
                    position = position_str,
                    "분 완결: 통계 갱신 완료 (시그널은 틱에서 처리)"
                );

                let record = MinuteRecord {
                    timestamp: ts.to_rfc3339(),
                    coin: coin.clone(),
                    upbit_close: upbit_close_f64,
                    bybit_close: bybit_close_f64,
                    usd_krw,
                    spread_pct: last_spread,
                    mean,
                    stddev: stddev_val,
                    z_score: z,
                    position: position_str.to_string(),
                    source: "live".to_string(),
                };

                minute_records.push(record.clone());
                let mut sw = session_writer.lock().await;
                if let Some(ref mut w) = *sw
                    && let Err(e) = w.append_minute(&record)
                {
                    warn!(
                        coin = coin.as_str(),
                        error = %e,
                        "분봉 통계 CSV 기록 실패"
                    );
                }
            }
        }

        candle_builder.start_new_minute(new_minute_ts);
        Ok(())
    }

    /// 재선택을 tokio::spawn으로 분리합니다.
    #[allow(clippy::too_many_arguments)]
    fn spawn_reselection(
        config: Arc<ZScoreConfig>,
        upbit: Arc<U>,
        bybit: Arc<B>,
        forex_cache: Arc<ForexCache>,
        spread_calc: Arc<tokio::sync::RwLock<SpreadCalculator>>,
        ob_cache: orderbook::SharedObCache,
        counters: Arc<std::sync::Mutex<MonitoringCounters>>,
        position_mgr: Arc<tokio::sync::Mutex<PositionManager>>,
        current_coins_snapshot: Vec<String>,
        dropped_at_snapshot: HashMap<String, DateTime<Utc>>,
        result_tx: tokio::sync::mpsc::Sender<ReselectionResult>,
    ) {
        tokio::spawn(async move {
            let selector = CoinSelector::new(upbit.as_ref(), bybit.as_ref());
            let usd_krw_for_reselect = forex_cache.get_cached_rate().unwrap_or(0.0);

            let new_candidates = match selector
                .select(
                    config.max_coins,
                    config.min_volume_1h_usdt,
                    &config.blacklist,
                    usd_krw_for_reselect,
                )
                .await
            {
                Ok(candidates) => candidates,
                Err(e) => {
                    warn!(error = %e, "코인 재선택 실패, 이전 목록 유지");
                    // 실패 시에도 현재 코인 목록을 그대로 전송하여 reselecting 해제
                    let _ = result_tx
                        .send(ReselectionResult {
                            new_coins: current_coins_snapshot,
                            dropped_at_updates: HashMap::new(),
                            removed_coins: Vec::new(),
                        })
                        .await;
                    return;
                }
            };

            let diff = {
                let pm = position_mgr.lock().await;
                diff_coins(&current_coins_snapshot, &new_candidates, &pm)
            };

            info!(
                to_add = ?diff.to_add,
                to_remove = ?diff.to_remove,
                to_keep_with_position = ?diff.to_keep_with_position,
                "코인 재선택 diff 결과"
            );

            let mut dropped_at_updates: HashMap<String, DateTime<Utc>> = HashMap::new();
            let mut removed_coins: Vec<String> = Vec::new();

            // 제거 코인 (포지션 없음): 즉시 정리
            for coin in &diff.to_remove {
                {
                    let mut sc = spread_calc.write().await;
                    sc.remove_coin(coin);
                }

                // WebSocket 구독 해제
                let upbit_market = format!("KRW-{coin}");
                let bybit_market = format!("{coin}USDT");
                if let Err(e) = upbit.unsubscribe_markets(&[&upbit_market]).await {
                    warn!(
                        coin = coin.as_str(),
                        error = %e,
                        "Upbit 구독 해제 실패"
                    );
                }
                if let Err(e) = bybit.unsubscribe_markets(&[&bybit_market]).await {
                    warn!(
                        coin = coin.as_str(),
                        error = %e,
                        "Bybit 구독 해제 실패"
                    );
                }

                removed_coins.push(coin.clone());
                info!(coin = coin.as_str(), "코인 제거 완료");
            }

            // 탈락 + 포지션 코인: 탈락 시각 기록
            for coin in &diff.to_keep_with_position {
                if !dropped_at_snapshot.contains_key(coin) {
                    dropped_at_updates.insert(coin.clone(), Utc::now());
                }
                info!(
                    coin = coin.as_str(),
                    "탈락 코인이지만 포지션 보유 중, 감시 유지"
                );
            }

            // 추가 코인: 워밍업 후 구독
            for coin in &diff.to_add {
                match Self::warmup_single_coin_with_lock(
                    upbit.as_ref(),
                    bybit.as_ref(),
                    &config,
                    &forex_cache,
                    coin,
                    &spread_calc,
                )
                .await
                {
                    Ok(()) => {
                        // WebSocket 구독 추가
                        let upbit_market = format!("KRW-{coin}");
                        let bybit_market = format!("{coin}USDT");
                        if let Err(e) = upbit.subscribe_markets(&[&upbit_market]).await {
                            warn!(
                                coin = coin.as_str(),
                                error = %e,
                                "Upbit 구독 추가 실패"
                            );
                            let mut sc = spread_calc.write().await;
                            sc.remove_coin(coin);
                            continue;
                        }
                        if let Err(e) = bybit.subscribe_markets(&[&bybit_market]).await {
                            warn!(
                                coin = coin.as_str(),
                                error = %e,
                                "Bybit 구독 추가 실패"
                            );
                            upbit.unsubscribe_markets(&[&upbit_market]).await.ok();
                            let mut sc = spread_calc.write().await;
                            sc.remove_coin(coin);
                            continue;
                        }

                        // 오더북 프리페치
                        let upbit_ob_market = format!("KRW-{coin}");
                        let bybit_ob_market = format!("{coin}USDT");
                        if let Ok(ob) = upbit.get_orderbook(&upbit_ob_market, Some(15)).await {
                            let mut data = ob_cache.data.write().await;
                            data.update(orderbook::Exchange::Upbit, coin, ob);
                            drop(data);
                            counters.lock().unwrap().orderbook_fetch_count += 1;
                        }
                        if let Ok(ob) = bybit.get_orderbook(&bybit_ob_market, Some(25)).await {
                            let mut data = ob_cache.data.write().await;
                            data.update(orderbook::Exchange::Bybit, coin, ob);
                            drop(data);
                            counters.lock().unwrap().orderbook_fetch_count += 1;
                        }

                        info!(coin = coin.as_str(), "코인 추가 완료");
                    }
                    Err(e) => {
                        warn!(
                            coin = coin.as_str(),
                            error = %e,
                            "코인 워밍업 실패, 건너뜀"
                        );
                        let mut sc = spread_calc.write().await;
                        sc.remove_coin(coin);
                    }
                }
            }

            // current_coins 업데이트: SpreadCalculator에 실제 존재하는 코인만
            let new_coins: Vec<String> = {
                let sc = spread_calc.read().await;
                sc.active_coins().iter().map(|s| s.to_string()).collect()
            };

            let result = ReselectionResult {
                new_coins,
                dropped_at_updates,
                removed_coins,
            };

            if result_tx.send(result).await.is_err() {
                warn!("재선택 결과 전송 실패 (receiver dropped)");
            }
        });
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
            id: 0,
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

    // --- maybe_spawn_tick_signal 관련 테스트 ---
    // spec/0007: check_tick_signal이 tokio::spawn으로 분리되었으므로
    // 스냅샷 추출 단계(spawn 이전)의 조기 리턴을 테스트합니다.

    /// 테스트 공용 헬퍼: Arc 래핑된 기본 공유 상태를 생성합니다.
    fn make_shared_state() -> (
        Arc<ZScoreConfig>,
        Arc<ForexCache>,
        Arc<tokio::sync::Mutex<PositionManager>>,
        Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
        orderbook::SharedObCache,
        Arc<std::sync::Mutex<MonitoringCounters>>,
        Arc<tokio::sync::Mutex<Option<SessionWriter>>>,
    ) {
        let config = Arc::new(ZScoreConfig::default());
        let forex_cache = Arc::new(ForexCache::new(Duration::from_secs(600)));
        let position_mgr = Arc::new(tokio::sync::Mutex::new(PositionManager::new()));
        let trades = Arc::new(tokio::sync::Mutex::new(Vec::<ClosedPosition>::new()));
        let ob_cache = orderbook::SharedObCache::new();
        let counters = Arc::new(std::sync::Mutex::new(MonitoringCounters::default()));
        let session_writer = Arc::new(tokio::sync::Mutex::new(None::<SessionWriter>));
        (config, forex_cache, position_mgr, trades, ob_cache, counters, session_writer)
    }

    #[tokio::test]
    async fn test_maybe_spawn_no_data() {
        // candle_builder에 데이터 없음 → spawn 안 됨 (즉시 리턴)
        let (config, forex_cache, position_mgr, trades, ob_cache, counters, session_writer) =
            make_shared_state();
        let spread_calc = Arc::new(tokio::sync::RwLock::new(SpreadCalculator::new(
            &["BTC".to_string()],
            10,
        )));
        let candle_builder = MinuteCandleBuilder::new();
        let upbit = Arc::new(MockMarket);
        let bybit = Arc::new(MockMarket);
        let coins = vec!["BTC".to_string()];

        let event = MarketEvent::Trade {
            timestamp: Utc::now(),
            market: "KRW-BTC".to_string(),
            price: Decimal::new(138_000_000, 0),
            volume: Decimal::new(1, 2),
        };

        // candle_builder는 비어있으므로 upbit_last_trade에 BTC 없음 → 즉시 리턴
        ZScoreMonitor::<MockMarket, MockMarket>::maybe_spawn_tick_signal(
            &event,
            &coins,
            &candle_builder,
            &spread_calc,
            &config,
            &forex_cache,
            &position_mgr,
            &trades,
            &ob_cache,
            &counters,
            &session_writer,
            &upbit,
            &bybit,
        )
        .await;

        // spawn 안 됐으므로 computing flag는 false
        assert!(!ob_cache.computing.is_computing(orderbook::Exchange::Upbit, "BTC"));
        // trades에 아무것도 없음
        assert!(trades.lock().await.is_empty());
    }

    #[tokio::test]
    async fn test_maybe_spawn_partial_data() {
        // Upbit 데이터만 있고 Bybit 없음 → spawn 안 됨
        let (config, forex_cache, position_mgr, trades, ob_cache, counters, session_writer) =
            make_shared_state();
        forex_cache.update_cache_for_test(1450.0);

        let spread_calc = Arc::new(tokio::sync::RwLock::new(SpreadCalculator::new(
            &["BTC".to_string()],
            10,
        )));
        let mut candle_builder = MinuteCandleBuilder::new();
        // Upbit 데이터만 넣음
        candle_builder
            .upbit_last_trade
            .insert("BTC".to_string(), Decimal::new(138_000_000, 0));
        let upbit = Arc::new(MockMarket);
        let bybit = Arc::new(MockMarket);
        let coins = vec!["BTC".to_string()];

        let event = MarketEvent::Trade {
            timestamp: Utc::now(),
            market: "KRW-BTC".to_string(),
            price: Decimal::new(138_000_000, 0),
            volume: Decimal::new(1, 2),
        };

        ZScoreMonitor::<MockMarket, MockMarket>::maybe_spawn_tick_signal(
            &event,
            &coins,
            &candle_builder,
            &spread_calc,
            &config,
            &forex_cache,
            &position_mgr,
            &trades,
            &ob_cache,
            &counters,
            &session_writer,
            &upbit,
            &bybit,
        )
        .await;

        // bybit_last_bid에 BTC 없으므로 즉시 리턴
        assert!(!ob_cache.computing.is_computing(orderbook::Exchange::Upbit, "BTC"));
    }

    #[tokio::test]
    async fn test_maybe_spawn_window_not_ready() {
        // 양쪽 데이터 있지만 SpreadCalculator 윈도우 미충족 → spawn 안 됨
        let (config, forex_cache, position_mgr, trades, ob_cache, counters, session_writer) =
            make_shared_state();
        forex_cache.update_cache_for_test(1450.0);

        let spread_calc = Arc::new(tokio::sync::RwLock::new(SpreadCalculator::new(
            &["BTC".to_string()],
            10,
        )));
        let mut candle_builder = MinuteCandleBuilder::new();
        candle_builder
            .upbit_last_trade
            .insert("BTC".to_string(), Decimal::new(138_000_000, 0));
        candle_builder
            .bybit_last_bid
            .insert("BTC".to_string(), Decimal::new(100_050, 0));
        let upbit = Arc::new(MockMarket);
        let bybit = Arc::new(MockMarket);
        let coins = vec!["BTC".to_string()];

        let event = MarketEvent::Trade {
            timestamp: Utc::now(),
            market: "KRW-BTC".to_string(),
            price: Decimal::new(138_000_000, 0),
            volume: Decimal::new(1, 2),
        };

        ZScoreMonitor::<MockMarket, MockMarket>::maybe_spawn_tick_signal(
            &event,
            &coins,
            &candle_builder,
            &spread_calc,
            &config,
            &forex_cache,
            &position_mgr,
            &trades,
            &ob_cache,
            &counters,
            &session_writer,
            &upbit,
            &bybit,
        )
        .await;

        // cached_stats가 None이므로 spawn 전에 리턴
        assert!(!ob_cache.computing.is_computing(orderbook::Exchange::Upbit, "BTC"));
    }

    #[tokio::test]
    async fn test_maybe_spawn_coin_not_in_list() {
        // 이벤트 코인이 current_coins에 없음 → spawn 안 됨
        let (config, forex_cache, position_mgr, trades, ob_cache, counters, session_writer) =
            make_shared_state();
        let spread_calc = Arc::new(tokio::sync::RwLock::new(SpreadCalculator::new(
            &["BTC".to_string()],
            10,
        )));
        let candle_builder = MinuteCandleBuilder::new();
        let upbit = Arc::new(MockMarket);
        let bybit = Arc::new(MockMarket);
        let coins = vec!["BTC".to_string()]; // ETH가 없음

        let event = MarketEvent::Trade {
            timestamp: Utc::now(),
            market: "KRW-ETH".to_string(),
            price: Decimal::new(5_000_000, 0),
            volume: Decimal::new(1, 2),
        };

        ZScoreMonitor::<MockMarket, MockMarket>::maybe_spawn_tick_signal(
            &event,
            &coins,
            &candle_builder,
            &spread_calc,
            &config,
            &forex_cache,
            &position_mgr,
            &trades,
            &ob_cache,
            &counters,
            &session_writer,
            &upbit,
            &bybit,
        )
        .await;

        assert!(!ob_cache.computing.is_computing(orderbook::Exchange::Upbit, "ETH"));
    }

    #[tokio::test]
    async fn test_computing_flag_prevents_duplicate_spawn() {
        // computing flag가 이미 설정된 경우 → dropped_tick_count 증가
        let (config, forex_cache, position_mgr, trades, ob_cache, counters, session_writer) =
            make_shared_state();
        forex_cache.update_cache_for_test(1450.0);

        // SpreadCalculator에 데이터를 충분히 넣어 cached_stats가 Some을 반환하도록
        let mut sc = SpreadCalculator::new(&["BTC".to_string()], 3);
        let base_time = Utc::now() - chrono::Duration::minutes(5);
        for i in 0..5 {
            let ts = base_time + chrono::Duration::minutes(i);
            sc.update(
                "BTC",
                ts,
                Some(Decimal::new(138_000_000, 0)),
                1450.0,
                Some(Decimal::new(100_050, 0)),
            )
            .unwrap();
        }
        let spread_calc = Arc::new(tokio::sync::RwLock::new(sc));

        let mut candle_builder = MinuteCandleBuilder::new();
        candle_builder
            .upbit_last_trade
            .insert("BTC".to_string(), Decimal::new(138_000_000, 0));
        candle_builder
            .bybit_last_bid
            .insert("BTC".to_string(), Decimal::new(100_050, 0));
        let upbit = Arc::new(MockMarket);
        let bybit = Arc::new(MockMarket);
        let coins = vec!["BTC".to_string()];

        // computing flag를 먼저 설정
        assert!(!ob_cache.computing.try_set_computing(orderbook::Exchange::Upbit, "BTC"));

        let event = MarketEvent::Trade {
            timestamp: Utc::now(),
            market: "KRW-BTC".to_string(),
            price: Decimal::new(138_000_000, 0),
            volume: Decimal::new(1, 2),
        };

        ZScoreMonitor::<MockMarket, MockMarket>::maybe_spawn_tick_signal(
            &event,
            &coins,
            &candle_builder,
            &spread_calc,
            &config,
            &forex_cache,
            &position_mgr,
            &trades,
            &ob_cache,
            &counters,
            &session_writer,
            &upbit,
            &bybit,
        )
        .await;

        // computing flag가 이미 설정되어 있으므로 dropped_tick_count가 증가
        let c = counters.lock().unwrap();
        assert_eq!(c.dropped_tick_count, 1);

        // flag 정리
        ob_cache.computing.clear_computing(orderbook::Exchange::Upbit, "BTC");
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
