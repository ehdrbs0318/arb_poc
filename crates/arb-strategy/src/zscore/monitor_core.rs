//! 실시간 Z-Score 모니터링 코어.
//!
//! WebSocket 스트림에서 수신한 MarketEvent를 1분 캔들로 집계하고,
//! Z-Score 기반 진입/청산 시그널을 실시간으로 감지합니다.
//! 틱 수신 시 즉시 시그널을 평가하고, 분 완결 시에는 통계만 갱신합니다.
//!
//! ## 아키텍처 (0007 EVENT_LOOP_ASYNC_DECOUPLING)
//!
//! select! 루프는 이벤트 소비 전용으로, 무거운 REST 호출(오더북 조회,
//! 코인 재선택 워밍업)은 `tokio::spawn`으로 분리하여 채널 오버플로우를 방지합니다.
//!
//! ## ExecutionPolicy 분기 (0011 LIVE_TRADING)
//!
//! `ExecutionPolicy` trait을 통해 시뮬레이션/라이브 체결 로직을 컴파일타임에 결정합니다.
//! - `SimPolicy`: 가상 체결 (VirtualPosition 즉시 생성)
//! - `LivePolicy`: 실주문 (LiveExecutor를 통한 IOC 지정가)

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive as _;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, trace, warn};

use arb_exchange::{InstrumentDataProvider, MarketData, MarketEvent, MarketStream};
use arb_forex::ForexCache;

use crate::common::candle_fetcher::fetch_all_candles;
use crate::common::convert::truncate_to_minute;
use crate::error::StrategyError;
use crate::output::summary::MonitoringCounters;
use crate::output::summary::SessionSummary;
use crate::output::writer::{MinuteRecord, SessionWriter};
use crate::zscore::coin_selector::{CoinCandidate, CoinSelector};
use crate::zscore::config::ZScoreConfig;
use crate::zscore::execution_policy::{
    EntryContext, ExecutionPolicy, ExitContext, SharedResources, TtlExpiryContext, TtlPosition,
};
use crate::zscore::instrument::{self, InstrumentCache, fetch_instruments};
use crate::zscore::orderbook;
use crate::zscore::pnl::ClosedPosition;
use crate::zscore::position::PositionManager;
#[cfg(test)]
use crate::zscore::position::VirtualPosition;
use crate::zscore::signal::{self, Signal};
use crate::zscore::spread::SpreadCalculator;

/// 분 완결 시 반환되는 데이터 (코인별 Upbit close, 코인별 Bybit close).
pub(crate) type MinuteCloses = (
    HashMap<String, Option<Decimal>>,
    HashMap<String, Option<Decimal>>,
);

/// 코인별 현재 분의 캔들 빌더.
#[derive(Debug)]
pub(crate) struct MinuteCandleBuilder {
    /// 현재 분의 시작 시간.
    pub current_minute: Option<DateTime<Utc>>,
    /// 코인별 Upbit 마지막 체결가.
    pub upbit_last_trade: HashMap<String, Decimal>,
    /// 코인별 Bybit best bid.
    pub bybit_last_bid: HashMap<String, Decimal>,
}

impl MinuteCandleBuilder {
    pub fn new() -> Self {
        Self {
            current_minute: None,
            upbit_last_trade: HashMap::new(),
            bybit_last_bid: HashMap::new(),
        }
    }

    /// 이벤트의 분이 변경되었는지 확인합니다.
    pub fn is_new_minute(&self, ts: DateTime<Utc>) -> bool {
        let minute = truncate_to_minute(ts);
        match self.current_minute {
            Some(current) => minute > current,
            None => true,
        }
    }

    /// 현재 분을 완결하고 각 코인의 close 데이터를 반환합니다.
    pub fn finalize_minute(&mut self, coins: &[String]) -> MinuteCloses {
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
    pub fn start_new_minute(&mut self, minute: DateTime<Utc>) {
        self.current_minute = Some(truncate_to_minute(minute));
        self.upbit_last_trade.clear();
        self.bybit_last_bid.clear();
    }

    /// Upbit Trade 이벤트를 처리합니다.
    ///
    /// KRW-USDT 마켓은 무시합니다 (ForexCache 사용).
    pub fn on_upbit_trade(&mut self, market: &str, price: Decimal) {
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
    pub fn on_bybit_best_quote(&mut self, market: &str, bid: Decimal) {
        trace!(market = market, bid = %bid, "Bybit best quote 수신");
        if let Some(coin) = market.strip_suffix("USDT") {
            self.bybit_last_bid.insert(coin.to_string(), bid);
        }
    }
}

/// 코인 diff 결과.
///
/// 현재 목록과 새 후보 목록을 비교하여 추가/제거/유지를 분류합니다.
pub(crate) struct CoinDiff {
    /// 새로 추가할 코인 (새 목록에만 있는 코인).
    pub to_add: Vec<String>,
    /// 제거할 코인 (현재 목록에만 있고 포지션 없는 코인).
    pub to_remove: Vec<String>,
    /// 유지할 코인 (현재 목록에만 있지만 포지션 보유 중).
    pub to_keep_with_position: Vec<String>,
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
pub(crate) fn diff_coins(
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
pub(crate) struct ReselectionResult {
    /// 갱신된 코인 목록.
    pub new_coins: Vec<String>,
    /// 탈락 코인별 탈락 시각 (기존 dropped_at에 merge).
    pub dropped_at_updates: HashMap<String, DateTime<Utc>>,
    /// 제거된 코인 (dropped_at에서 삭제 대상).
    pub removed_coins: Vec<String>,
}

/// Regime change 감지 배수.
/// max_spread_stddev * 이 값을 초과하면 코인 제거 대상.
const REGIME_CHANGE_MULTIPLIER: f64 = 1.5;

/// finalize_and_process의 regime change 감지 결과.
pub(crate) struct RegimeChangeResult {
    /// 포지션 없어서 즉시 제거할 코인.
    pub immediate_remove: Vec<String>,
    /// 포지션 있어서 dropped_at에 등록할 코인.
    pub dropped_coins: Vec<String>,
}

/// 워밍업 후 stddev 기준으로 코인을 필터링합니다.
///
/// # 인자
/// * `coin_stats` - 코인별 (mean, stddev). None이면 워밍업 실패로 제거.
/// * `max_spread_stddev` - stddev 상한 (0.0이면 필터링 없이 stddev 오름차순 max_coins개 반환)
/// * `max_coins` - 최대 코인 수
///
/// # 반환값
/// (유지 코인, 제거 코인)
pub(crate) fn filter_coins_by_stddev(
    coin_stats: &[(String, Option<(f64, f64)>)],
    max_spread_stddev: f64,
    max_coins: usize,
) -> (Vec<String>, Vec<String>) {
    // cached_stats=None인 코인 제거
    let mut valid: Vec<(String, f64)> = coin_stats
        .iter()
        .filter_map(|(coin, stats)| stats.map(|(_, stddev)| (coin.clone(), stddev)))
        .collect();

    // None인 코인을 제거 목록에 추가
    let mut removed: Vec<String> = coin_stats
        .iter()
        .filter(|(_, stats)| stats.is_none())
        .map(|(coin, _)| coin.clone())
        .collect();

    // stddev 오름차순 정렬
    valid.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    if max_spread_stddev == 0.0 {
        // 필터링 없이 stddev 오름차순 max_coins개만 반환
        let kept: Vec<String> = valid
            .iter()
            .take(max_coins)
            .map(|(c, _)| c.clone())
            .collect();
        let excess: Vec<String> = valid
            .iter()
            .skip(max_coins)
            .map(|(c, _)| c.clone())
            .collect();
        removed.extend(excess);
        return (kept, removed);
    }

    // stddev 필터 적용
    let mut passed: Vec<(String, f64)> = Vec::new();
    let mut failed: Vec<(String, f64)> = Vec::new();

    for (coin, stddev) in &valid {
        if *stddev <= max_spread_stddev {
            passed.push((coin.clone(), *stddev));
        } else {
            failed.push((coin.clone(), *stddev));
        }
    }

    if passed.is_empty() {
        // Fallback: 전체 초과 시 stddev 오름차순 max_coins개 강제 선택
        warn!(
            max_spread_stddev = max_spread_stddev,
            total_coins = valid.len(),
            "모든 코인 stddev 초과, fallback: stddev 오름차순 {}개 강제 선택",
            max_coins
        );
        let kept: Vec<String> = valid
            .iter()
            .take(max_coins)
            .map(|(c, _)| c.clone())
            .collect();
        let excess: Vec<String> = valid
            .iter()
            .skip(max_coins)
            .map(|(c, _)| c.clone())
            .collect();
        removed.extend(excess);
        return (kept, removed);
    }

    // 통과한 코인에서 max_coins개 선택
    let kept: Vec<String> = passed
        .iter()
        .take(max_coins)
        .map(|(c, _)| c.clone())
        .collect();
    let excess: Vec<String> = passed
        .iter()
        .skip(max_coins)
        .map(|(c, _)| c.clone())
        .collect();
    removed.extend(excess);
    removed.extend(failed.iter().map(|(c, _)| c.clone()));

    (kept, removed)
}

/// 실시간 Z-Score 모니터.
///
/// `P: ExecutionPolicy`로 시뮬레이션/라이브 체결을 컴파일타임에 결정합니다.
/// `tokio::spawn`으로 REST 호출을 분리하기 위해 필드를 Arc로 래핑합니다.
pub struct ZScoreMonitor<U, B, P>
where
    U: MarketData + MarketStream + Send + Sync + 'static,
    B: MarketData + MarketStream + InstrumentDataProvider + Send + Sync + 'static,
    P: ExecutionPolicy,
{
    upbit: Arc<U>,
    bybit: Arc<B>,
    config: Arc<ZScoreConfig>,
    forex_cache: Arc<ForexCache>,
    policy: Arc<P>,
}

impl<U, B, P> ZScoreMonitor<U, B, P>
where
    U: MarketData + MarketStream + Send + Sync + 'static,
    B: MarketData + MarketStream + InstrumentDataProvider + Send + Sync + 'static,
    P: ExecutionPolicy,
{
    /// 새 ZScoreMonitor를 생성합니다.
    ///
    /// 기존 값 타입 파라미터를 받아 내부에서 Arc로 감쌉니다.
    pub fn new(
        upbit: U,
        bybit: B,
        config: ZScoreConfig,
        forex_cache: Arc<ForexCache>,
        policy: P,
    ) -> Self {
        Self {
            upbit: Arc::new(upbit),
            bybit: Arc::new(bybit),
            config: Arc::new(config),
            forex_cache,
            policy: Arc::new(policy),
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
            // 확대 선택: stddev 필터 + pruning 여유분 확보
            let expanded_count = self.config.max_coins * 2;
            let candidates = selector
                .select(
                    expanded_count,
                    self.config.min_volume_1h_usdt,
                    &self.config.blacklist,
                    usd_krw_for_select,
                )
                .await?;

            if candidates.is_empty() {
                warn!("자동 선택 결과 후보 코인이 없습니다. 볼륨 조건을 확인하세요.");
            }

            let mut expanded_coins: Vec<String> =
                candidates.iter().map(|c| c.coin.clone()).collect();
            expanded_coins.sort();
            expanded_coins.dedup();
            info!(coins = ?expanded_coins, count = expanded_coins.len(), "확대 후보 코인 선택 완료");
            expanded_coins
        } else {
            self.config.coins.clone()
        };

        // 시작 시점 환율 기록
        let usd_krw_start = self.forex_cache.get_cached_rate().unwrap_or(0.0);

        info!("실시간 모니터링 시작: 워밍업 데이터 로드 중...");

        // 2. 워밍업: REST API로 캔들 사전 로드 (로컬 변수)
        let mut spread_calc_local = if self.config.auto_select {
            // auto_select: 빈 상태로 생성 (warmup_single_coin_standalone이 add_coin 호출)
            let mut sc = SpreadCalculator::new(&[], self.config.window_size);
            for coin in &current_coins {
                if let Err(e) = Self::warmup_single_coin_standalone(
                    self.upbit.as_ref(),
                    self.bybit.as_ref(),
                    &self.config,
                    &self.forex_cache,
                    coin,
                    &mut sc,
                )
                .await
                {
                    warn!(coin = coin.as_str(), error = %e, "워밍업 실패, 해당 코인 스킵");
                    sc.remove_coin(coin);
                }
            }
            sc
        } else {
            // 수동 선택: 기존 warmup 유지 (실패 시 즉시 에러)
            let mut sc = SpreadCalculator::new(&current_coins, self.config.window_size);
            Self::warmup(
                self.upbit.as_ref(),
                self.bybit.as_ref(),
                &self.config,
                &self.forex_cache,
                &current_coins,
                &mut sc,
            )
            .await?;
            sc
        };

        // auto_select: stddev 필터링
        if self.config.auto_select && self.config.max_spread_stddev >= 0.0 {
            let coin_stats: Vec<(String, Option<(f64, f64)>)> = current_coins
                .iter()
                .map(|c| (c.clone(), spread_calc_local.cached_stats(c)))
                .collect();

            let (kept, removed) = filter_coins_by_stddev(
                &coin_stats,
                self.config.max_spread_stddev,
                self.config.max_coins,
            );

            if kept.is_empty() {
                let all_none = coin_stats.iter().all(|(_, s)| s.is_none());
                let msg = if all_none {
                    "모든 후보 코인의 워밍업이 실패했습니다 (REST API 장애 가능)"
                } else {
                    "워밍업은 성공했지만 모든 코인의 spread stddev가 임계값을 초과합니다"
                };
                return Err(StrategyError::Config(msg.to_string()));
            }

            // 제거된 코인 정리
            for coin in &removed {
                spread_calc_local.remove_coin(coin);
                debug!(coin = coin.as_str(), "stddev 필터: 코인 제거");
            }

            // stddev 로그
            for coin in &kept {
                if let Some((_, stddev)) = spread_calc_local.cached_stats(coin) {
                    info!(
                        coin = coin.as_str(),
                        stddev = stddev,
                        max_spread_stddev = self.config.max_spread_stddev,
                        "stddev 필터 통과"
                    );
                }
            }

            current_coins = kept;
            info!(
                coins = ?current_coins,
                removed_count = removed.len(),
                "stddev 필터 적용 후 최종 코인 목록"
            );
        }

        // InstrumentCache 초기화 (필터 후 코인만)
        let instrument_cache = Arc::new(parking_lot::RwLock::new(InstrumentCache::default()));
        fetch_instruments(self.bybit.as_ref(), &instrument_cache, &current_coins).await;

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
        let counters = Arc::new(parking_lot::Mutex::new(counters_local));
        let session_writer = Arc::new(tokio::sync::Mutex::new(session_writer_local));
        let spread_calc = Arc::new(tokio::sync::RwLock::new(spread_calc_local));
        let total_event_count = Arc::new(AtomicU64::new(0));

        // ExecutionPolicy에 공유 상태 바인딩 (SimPolicy: OnceLock 설정, LivePolicy: no-op)
        self.policy.bind_shared_resources(SharedResources {
            config: Arc::clone(&self.config),
            position_mgr: Arc::clone(&position_mgr),
            trades: Arc::clone(&trades),
            counters: Arc::clone(&counters),
            session_writer: Arc::clone(&session_writer),
        });
        debug!("ExecutionPolicy에 공유 상태 바인딩 완료");

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

        // regime change cooldown 상태
        let mut regime_cooldown_until: Option<DateTime<Utc>> = None;
        let mut consecutive_regime_changes: u32 = 0;
        const MAX_COOLDOWN_MIN: u64 = 60;

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
                        &instrument_cache,
                        &counters,
                        &self.policy,
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
                        &ob_cache,
                        &counters,
                        &self.upbit,
                        &self.bybit,
                        &instrument_cache,
                        &self.policy,
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
                        &instrument_cache,
                        &counters,
                        &self.policy,
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
                        &ob_cache,
                        &counters,
                        &self.upbit,
                        &self.bybit,
                        &instrument_cache,
                        &self.policy,
                    ).await;
                }
                _ = minute_timer.tick() => {
                    let now = Utc::now();
                    if candle_builder.is_new_minute(now) {
                        match Self::finalize_and_process(
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
                            &instrument_cache,
                            &counters,
                            &self.policy,
                        ).await {
                            Ok(Some(regime)) => {
                                let in_cooldown = regime_cooldown_until
                                    .map(|until| Utc::now() < until)
                                    .unwrap_or(false);
                                if in_cooldown {
                                    debug!("regime change 감지되었으나 cooldown 중, 무시");
                                    counters.lock()
                                        .regime_change_suppressed_by_cooldown_count += 1;
                                } else {
                                    for coin in &regime.immediate_remove {
                                        {
                                            let mut sc = spread_calc.write().await;
                                            sc.remove_coin(coin);
                                        }
                                        {
                                            let mut data = ob_cache.data.write().await;
                                            data.remove_coin(coin);
                                        }
                                        ob_cache.computing.remove_coin(coin);
                                        let upbit_market = format!("KRW-{coin}");
                                        let bybit_market = format!("{coin}USDT");
                                        self.upbit.unsubscribe_markets(&[&upbit_market]).await.ok();
                                        self.bybit.unsubscribe_markets(&[&bybit_market]).await.ok();
                                        current_coins.retain(|c| c != coin);
                                        info!(coin = coin.as_str(), "regime change로 코인 즉시 제거");
                                    }
                                    for coin in &regime.dropped_coins {
                                        dropped_at.entry(coin.clone()).or_insert(Utc::now());
                                    }
                                    counters.lock().regime_change_detected_count += 1;
                                    if current_coins.len() < self.config.max_coins
                                        && self.config.auto_select
                                        && !reselecting
                                    {
                                        reselecting = true;
                                        consecutive_regime_changes += 1;
                                        let backoff_min = (self.config.reselect_interval_min)
                                            .saturating_mul(
                                                1u64 << (consecutive_regime_changes - 1).min(6),
                                            )
                                            .min(MAX_COOLDOWN_MIN);
                                        regime_cooldown_until = Some(
                                            Utc::now()
                                                + chrono::Duration::minutes(backoff_min as i64),
                                        );
                                        info!(
                                            consecutive = consecutive_regime_changes,
                                            cooldown_min = backoff_min,
                                            "regime change → 즉시 재선택 (지수적 백오프 cooldown)"
                                        );
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
                                }
                            }
                            Ok(None) => {
                                if consecutive_regime_changes > 0 {
                                    let cooldown_expired = regime_cooldown_until
                                        .map(|until| Utc::now() >= until)
                                        .unwrap_or(true);
                                    if cooldown_expired {
                                        consecutive_regime_changes = 0;
                                        regime_cooldown_until = None;
                                    }
                                }
                            }
                            Err(e) => {
                                warn!(error = %e, "finalize_and_process 실패");
                            }
                        }
                    }

                    // TTL 만료 포지션 체크 (모든 코인)
                    if let Err(e) = Self::check_ttl_positions(
                        &self.config,
                        &position_mgr,
                        &spread_calc,
                        &counters,
                        &dropped_at,
                        &self.forex_cache,
                        &instrument_cache,
                        &self.policy,
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
                    current_coins = result.new_coins.clone();
                    for (coin, ts) in result.dropped_at_updates {
                        dropped_at.entry(coin).or_insert(ts);
                    }
                    for coin in &result.removed_coins {
                        dropped_at.remove(coin);
                    }
                    // 새 코인에 대한 InstrumentInfo 로드
                    let new_coins_to_fetch: Vec<String> = {
                        let cache = instrument_cache.read();
                        result.new_coins.iter()
                            .filter(|c| cache.get(c).is_none())
                            .cloned()
                            .collect()
                    };
                    if !new_coins_to_fetch.is_empty() {
                        fetch_instruments(self.bybit.as_ref(), &instrument_cache, &new_coins_to_fetch).await;
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
        // parking_lot::Mutex guard를 .await 전에 drop (Send가 아니므로 필수)
        let counters_snapshot = counters.lock().clone();
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
        instrument_cache: &Arc<parking_lot::RwLock<InstrumentCache>>,
        counters: &Arc<parking_lot::Mutex<MonitoringCounters>>,
        policy: &Arc<P>,
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
                // regime change 결과는 update_candle_and_spread 경로에서 무시
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
                    instrument_cache,
                    counters,
                    policy,
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
        ob_cache: &orderbook::SharedObCache,
        counters: &Arc<parking_lot::Mutex<MonitoringCounters>>,
        upbit_client: &Arc<U>,
        bybit_client: &Arc<B>,
        instrument_cache: &Arc<parking_lot::RwLock<InstrumentCache>>,
        policy: &Arc<P>,
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
            counters.lock().dropped_tick_count += 1;
            return;
        }
        if ob_cache.computing.is_computing(other_exchange, &coin) {
            // 반대쪽에서 computing 중 → 스킵 (동일 코인 동시 진입/청산 방지)
            ob_cache.computing.clear_computing(source_exchange, &coin);
            counters.lock().dropped_tick_count += 1;
            return;
        }

        // 5. tokio::spawn
        debug!(coin = coin.as_str(), "check_tick_signal task spawn");

        let coin_clone = coin.clone();
        let config = Arc::clone(config);
        let position_mgr = Arc::clone(position_mgr);
        let ob_cache = ob_cache.clone();
        let counters = Arc::clone(counters);
        let upbit_client = Arc::clone(upbit_client);
        let bybit_client = Arc::clone(bybit_client);
        let instrument_cache = Arc::clone(instrument_cache);
        let policy = Arc::clone(policy);

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
                ob_cache.clone(),
                counters,
                upbit_client,
                bybit_client,
                instrument_cache,
                policy,
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
    /// 진입/청산 실행은 `ExecutionPolicy` 콜백을 통해 수행합니다.
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
        ob_cache: orderbook::SharedObCache,
        counters: Arc<parking_lot::Mutex<MonitoringCounters>>,
        upbit_client: Arc<U>,
        bybit_client: Arc<B>,
        instrument_cache: Arc<parking_lot::RwLock<InstrumentCache>>,
        policy: Arc<P>,
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
                counters.lock().orderbook_fetch_count += 1;
            }
            Err(e) => {
                warn!(coin = coin.as_str(), exchange = ?source_exchange, error = %e, "오더북 조회 실패");
                counters.lock().orderbook_fetch_fail_count += 1;
                return Ok(());
            }
        }

        // 반대쪽 캐시 신선도 확인
        {
            let data = ob_cache.data.read().await;
            if !data.is_fresh(other_exchange, &coin, config.max_cache_age_sec) {
                counters.lock().stale_cache_skip_count += 1;
                return Ok(());
            }
        }

        // USD 환산된 Upbit 가격
        let upbit_f64 = upbit_price.to_f64().unwrap_or(0.0);
        let upbit_usd = upbit_f64 / usd_krw;
        let upbit_usd_dec = Decimal::try_from(upbit_usd).unwrap_or(Decimal::ZERO);

        // 청산/진입 공통: InstrumentInfo 조회
        let inst_info = {
            let cache = instrument_cache.read();
            cache.get(&coin).cloned()
        };

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
                // 청산 가격 라운딩
                let (exit_upbit_usd, exit_bybit) = if let Some(ref inst) = inst_info {
                    // Upbit 매도: floor (더 싸게 팔음 = 불리한 방향)
                    let exit_upbit_krw = instrument::floor_to_step(
                        upbit_price,
                        instrument::upbit_tick_size(upbit_price),
                    );
                    let exit_upbit_usd_val =
                        Decimal::try_from(exit_upbit_krw.to_f64().unwrap_or(0.0) / usd_krw)
                            .unwrap_or(upbit_usd_dec);

                    // Bybit close (매수): ceil (더 비싸게 삼 = 불리한 방향)
                    let exit_bybit_val =
                        instrument::round_price_conservative(bybit_price, inst.tick_size, true);
                    (exit_upbit_usd_val, exit_bybit_val)
                } else {
                    counters.lock().fallback_no_rounding_count += 1;
                    (upbit_usd_dec, bybit_price)
                };

                info!(
                    coin = c.as_str(),
                    z_score = z_score,
                    spread_pct = sp,
                    exit_upbit_usd = %exit_upbit_usd,
                    exit_bybit = %exit_bybit,
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

                    // ExitContext 구성 → policy 콜백
                    let exit_ctx = ExitContext {
                        coin: c.clone(),
                        z_score,
                        spread_pct: sp,
                        exit_upbit_usd,
                        exit_bybit,
                        usd_krw,
                        exit_safe_volume_usdt: exit_safe.map(|sv| sv.safe_volume_usdt),
                        mean,
                        instrument_info: inst_info.clone(),
                        bybit_price,
                    };

                    if let Err(e) = policy.on_exit_signal(exit_ctx).await {
                        warn!(coin = c.as_str(), error = %e, "청산 정책 실행 실패");
                    }
                } else {
                    drop(data);
                }
            }
        }

        // 5. 진입 시그널 평가
        if !policy.is_entry_allowed() {
            return Ok(());
        }

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
                // InstrumentInfo 필수 체크
                let Some(ref inst) = inst_info else {
                    debug!(coin = c.as_str(), "진입 거부: InstrumentInfo 없음");
                    counters.lock().entry_rejected_order_constraint_count += 1;
                    return Ok(());
                };

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
                            // pm 락 → 자본 확인
                            let entry_result = {
                                let pm = position_mgr.lock().await;
                                let used = pm.coin_used_capital(&c).to_f64().unwrap_or(0.0);
                                let remaining_cap = max_coin_cap - used;
                                drop(pm);

                                let size_usdt_f64 =
                                    (sv.safe_volume_usdt * ratio).min(remaining_cap);

                                // 9단계 검증
                                Self::validate_entry(
                                    &c,
                                    size_usdt_f64,
                                    bybit_price,
                                    upbit_price,
                                    usd_krw,
                                    sp,
                                    expected_profit_pct,
                                    inst,
                                    &config,
                                )
                            };

                            match entry_result {
                                EntryValidation::Accepted {
                                    qty,
                                    upbit_entry_usd,
                                    bybit_entry,
                                    adjusted_profit,
                                } => {
                                    // EntryContext 구성 → policy 콜백
                                    let entry_ctx = EntryContext {
                                        coin: c.clone(),
                                        z_score,
                                        spread_pct: sp,
                                        expected_profit_pct,
                                        adjusted_profit_pct: adjusted_profit,
                                        upbit_price_krw: upbit_price,
                                        upbit_entry_usd,
                                        bybit_entry,
                                        qty,
                                        usd_krw,
                                        mean,
                                        stddev,
                                        instrument_info: inst.clone(),
                                        safe_volume_usdt: sv.safe_volume_usdt,
                                        volume_ratio: ratio,
                                    };

                                    info!(
                                        coin = c.as_str(),
                                        z_score = z_score,
                                        spread_pct = sp,
                                        expected_profit = expected_profit_pct,
                                        adjusted_profit = adjusted_profit,
                                        qty = %qty,
                                        upbit_entry_usd = %upbit_entry_usd,
                                        bybit_entry = %bybit_entry,
                                        "[틱] 진입 시그널 (라운딩 적용)"
                                    );

                                    if let Err(e) = policy.on_entry_signal(entry_ctx).await {
                                        warn!(
                                            coin = c.as_str(),
                                            error = %e,
                                            "진입 정책 실행 실패"
                                        );
                                    }
                                }
                                EntryValidation::Rejected(reason) => match reason.as_str() {
                                    "order_constraint" => {
                                        counters.lock().entry_rejected_order_constraint_count += 1;
                                    }
                                    "rounding_pnl" => {
                                        counters.lock().entry_rejected_rounding_pnl_count += 1;
                                    }
                                    "min_position" => {
                                        counters.lock().entry_rejected_min_position_count += 1;
                                    }
                                    "min_roi" => {
                                        counters.lock().entry_rejected_min_roi_count += 1;
                                    }
                                    _ => {
                                        counters.lock().entry_rejected_slippage_count += 1;
                                    }
                                },
                            }
                        }
                        None => {
                            counters.lock().entry_rejected_slippage_count += 1;
                            debug!(coin = c.as_str(), "진입 거부: 오더북 안전 볼륨 없음");
                        }
                    }
                } else {
                    drop(data);
                    counters.lock().stale_cache_skip_count += 1;
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
        spread_calc: &tokio::sync::RwLock<SpreadCalculator>,
        counters: &parking_lot::Mutex<MonitoringCounters>,
        dropped_at: &HashMap<String, DateTime<Utc>>,
        forex_cache: &ForexCache,
        instrument_cache: &Arc<parking_lot::RwLock<InstrumentCache>>,
        policy: &Arc<P>,
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

            let positions: Vec<(u64, Decimal, Decimal)> = {
                let pm = position_mgr.lock().await;
                pm.open_positions
                    .get(coin.as_str())
                    .map(|ps| ps.iter().map(|p| (p.id, p.size_usdt(), p.qty)).collect())
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

            // TTL 청산 가격 라운딩
            let inst_info = {
                let cache = instrument_cache.read();
                cache.get(coin).cloned()
            };
            let (exit_upbit_usd, exit_bybit) = if let Some(ref inst) = inst_info {
                let exit_bybit_val =
                    instrument::round_price_conservative(bybit_price_dec, inst.tick_size, true);
                (upbit_usdt, exit_bybit_val)
            } else {
                counters.lock().fallback_no_rounding_count += 1;
                (upbit_usdt, bybit_price_dec)
            };

            let force_close = elapsed > ttl + grace;

            // TtlExpiryContext 구성 → policy 콜백
            let z_score = {
                let sc = spread_calc.read().await;
                sc.cached_stats(coin)
                    .map(|(m, s)| {
                        if s > 0.0 {
                            (spread_pct_val - m) / s
                        } else {
                            0.0
                        }
                    })
                    .unwrap_or(0.0)
            };

            let ttl_positions: Vec<TtlPosition> = positions
                .iter()
                .map(|(id, size_usdt, qty)| TtlPosition {
                    id: *id,
                    size_usdt: *size_usdt,
                    qty: *qty,
                })
                .collect();

            let ttl_ctx = TtlExpiryContext {
                coin: coin.clone(),
                positions: ttl_positions,
                usd_krw,
                current_spread_pct: spread_pct_val,
                z_score,
                instrument_info: inst_info.clone(),
                exit_upbit_usd,
                exit_bybit,
                force_close,
            };

            if force_close {
                warn!(coin = coin.as_str(), "2단계 강제 청산: grace period 초과");
            } else {
                warn!(coin = coin.as_str(), "1단계 TTL 청산 시도");
            }

            if let Err(e) = policy.on_ttl_expiry(ttl_ctx).await {
                warn!(coin = coin.as_str(), error = %e, "TTL 청산 정책 실행 실패");
            }
        }

        Ok(())
    }

    /// 현재 분을 완결하고 통계를 갱신합니다.
    ///
    /// 시그널 평가는 틱에서 처리하므로, 여기서는 SpreadCalculator 업데이트와
    /// Liquidation 체크만 수행합니다.
    /// regime change 감지 시 `Some(RegimeChangeResult)` 반환.
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
        instrument_cache: &Arc<parking_lot::RwLock<InstrumentCache>>,
        counters: &parking_lot::Mutex<MonitoringCounters>,
        policy: &Arc<P>,
    ) -> Result<Option<RegimeChangeResult>, StrategyError> {
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

                // Liquidation 가격 라운딩 (Bybit만, Upbit은 USD 상태이므로 원본)
                let liq_inst = {
                    let cache = instrument_cache.read();
                    cache.get(coin).cloned()
                };
                let exit_bybit_liq = if let Some(ref inst) = liq_inst {
                    instrument::round_price_conservative(bybit_price, inst.tick_size, true)
                } else {
                    bybit_price
                };

                let mut pm = position_mgr.lock().await;
                let liquidated_ids = pm.check_liquidation(coin, bybit_price);
                if !liquidated_ids.is_empty() {
                    let upbit_usdt = upbit_usdt_snapshot;
                    let spread_pct_val = spread_pct_snapshot;

                    for pid in liquidated_ids {
                        let db_id = pm
                            .open_positions
                            .get(coin.as_str())
                            .and_then(|positions| positions.iter().find(|p| p.id == pid))
                            .and_then(|p| p.db_id);

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
                            exit_bybit_liq,
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
                                policy.on_trade_closed(&closed, db_id).await;
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
            let stats_snapshot = {
                let sc = spread_calc.read().await;
                sc.cached_stats(coin).map(|(mean, stddev_val)| {
                    let last_spread = sc.last_spread_pct(coin).unwrap_or(0.0);
                    let upbit_close = sc.upbit_window(coin).and_then(|w| w.last()).unwrap_or(0.0);
                    let bybit_close = sc.bybit_window(coin).and_then(|w| w.last()).unwrap_or(0.0);
                    (mean, stddev_val, last_spread, upbit_close, bybit_close)
                })
            };

            if let Some((mean, stddev_val, last_spread, upbit_close_f64, bybit_close_f64)) =
                stats_snapshot
            {
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
                policy.on_minute_closed(&record).await;
            }
        }

        // regime change 감지
        let mut regime_result: Option<RegimeChangeResult> = None;
        if config.max_spread_stddev > 0.0 && config.auto_select {
            let regime_threshold = config.max_spread_stddev * REGIME_CHANGE_MULTIPLIER;

            let stddev_snapshot: Vec<(String, f64)> = {
                let sc = spread_calc.read().await;
                current_coins
                    .iter()
                    .filter_map(|coin| {
                        let stddev = sc
                            .cached_short_stats(coin)
                            .or_else(|| sc.cached_stats(coin))
                            .map(|(_, s)| s);
                        stddev
                            .filter(|s| *s > regime_threshold)
                            .map(|s| (coin.clone(), s))
                    })
                    .collect()
            };

            if !stddev_snapshot.is_empty() {
                let mut immediate_remove = Vec::new();
                let mut dropped_coins = Vec::new();

                let pm = position_mgr.lock().await;
                for (coin, stddev) in &stddev_snapshot {
                    warn!(
                        coin = coin.as_str(),
                        stddev = stddev,
                        threshold = regime_threshold,
                        "regime change 감지: stddev 급등"
                    );
                    if pm.has_position(coin) {
                        dropped_coins.push(coin.clone());
                    } else {
                        immediate_remove.push(coin.clone());
                    }
                }
                drop(pm);

                counters.lock().coin_rejected_spread_stddev_count += stddev_snapshot.len() as u64;

                regime_result = Some(RegimeChangeResult {
                    immediate_remove,
                    dropped_coins,
                });
            }
        }

        candle_builder.start_new_minute(new_minute_ts);
        Ok(regime_result)
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
        counters: Arc<parking_lot::Mutex<MonitoringCounters>>,
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
                    config.max_coins * 2,
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

            let mut diff = {
                let pm = position_mgr.lock().await;
                diff_coins(&current_coins_snapshot, &new_candidates, &pm)
            };

            // 기존 코인 stddev 체크: spread_calc read → 값 복사 → drop (lock order 준수)
            if config.max_spread_stddev > 0.0 {
                let stddev_snapshot: Vec<(String, f64)> = {
                    let sc = spread_calc.read().await;
                    current_coins_snapshot
                        .iter()
                        .filter_map(|coin| {
                            sc.cached_stats(coin)
                                .filter(|(_, s)| *s > config.max_spread_stddev)
                                .map(|(_, s)| (coin.clone(), s))
                        })
                        .collect()
                }; // sc drop

                if !stddev_snapshot.is_empty() {
                    let pm = position_mgr.lock().await;
                    let mut added_count = 0u64;
                    for (coin, _stddev) in &stddev_snapshot {
                        if pm.has_position(coin) {
                            if !diff.to_keep_with_position.contains(coin) {
                                diff.to_keep_with_position.push(coin.clone());
                                added_count += 1;
                            }
                        } else if !diff.to_remove.contains(coin) {
                            diff.to_remove.push(coin.clone());
                            added_count += 1;
                        }
                    }
                    drop(pm);
                    counters.lock().coin_rejected_spread_stddev_count += added_count;
                }

                diff.to_add.retain(|coin| !diff.to_remove.contains(coin));
            }

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

                let upbit_market = format!("KRW-{coin}");
                let bybit_market = format!("{coin}USDT");
                if let Err(e) = upbit.unsubscribe_markets(&[&upbit_market]).await {
                    warn!(coin = coin.as_str(), error = %e, "Upbit 구독 해제 실패");
                }
                if let Err(e) = bybit.unsubscribe_markets(&[&bybit_market]).await {
                    warn!(coin = coin.as_str(), error = %e, "Bybit 구독 해제 실패");
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
                        let upbit_market = format!("KRW-{coin}");
                        let bybit_market = format!("{coin}USDT");
                        if let Err(e) = upbit.subscribe_markets(&[&upbit_market]).await {
                            warn!(coin = coin.as_str(), error = %e, "Upbit 구독 추가 실패");
                            let mut sc = spread_calc.write().await;
                            sc.remove_coin(coin);
                            continue;
                        }
                        if let Err(e) = bybit.subscribe_markets(&[&bybit_market]).await {
                            warn!(coin = coin.as_str(), error = %e, "Bybit 구독 추가 실패");
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
                            counters.lock().orderbook_fetch_count += 1;
                        }
                        if let Ok(ob) = bybit.get_orderbook(&bybit_ob_market, Some(25)).await {
                            let mut data = ob_cache.data.write().await;
                            data.update(orderbook::Exchange::Bybit, coin, ob);
                            drop(data);
                            counters.lock().orderbook_fetch_count += 1;
                        }

                        // 새 코인 stddev 체크
                        if config.max_spread_stddev > 0.0 {
                            let exceeds = {
                                let sc = spread_calc.read().await;
                                sc.cached_stats(coin)
                                    .map(|(_, stddev)| stddev > config.max_spread_stddev)
                                    .unwrap_or(false)
                            };
                            if exceeds {
                                warn!(
                                    coin = coin.as_str(),
                                    max = config.max_spread_stddev,
                                    "재선택 코인 stddev 초과, 건너뜀"
                                );
                                spread_calc.write().await.remove_coin(coin);
                                counters.lock().coin_rejected_spread_stddev_count += 1;
                                continue;
                            }
                        }

                        info!(coin = coin.as_str(), "코인 추가 완료");
                    }
                    Err(e) => {
                        warn!(coin = coin.as_str(), error = %e, "코인 워밍업 실패, 건너뜀");
                        let mut sc = spread_calc.write().await;
                        sc.remove_coin(coin);
                    }
                }
            }

            // 최종 코인 수집 전 초과 코인 pruning
            let new_coins: Vec<String> = {
                let sc = spread_calc.read().await;
                let mut active: Vec<(String, f64)> = sc
                    .active_coins()
                    .iter()
                    .filter_map(|coin| {
                        sc.cached_stats(coin)
                            .map(|(_, stddev)| (coin.to_string(), stddev))
                    })
                    .collect();
                active.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
                active.into_iter().map(|(coin, _)| coin).collect()
            };

            if new_coins.len() > config.max_coins {
                let excess: Vec<String> = new_coins[config.max_coins..].to_vec();

                {
                    let mut sc = spread_calc.write().await;
                    for coin in &excess {
                        sc.remove_coin(coin);
                    }
                }
                {
                    let mut data = ob_cache.data.write().await;
                    for coin in &excess {
                        data.remove_coin(coin);
                    }
                }

                for coin in &excess {
                    let upbit_market = format!("KRW-{coin}");
                    let bybit_market = format!("{coin}USDT");
                    upbit.unsubscribe_markets(&[&upbit_market]).await.ok();
                    bybit.unsubscribe_markets(&[&bybit_market]).await.ok();
                    removed_coins.push(coin.clone());
                }
            }

            let final_coins: Vec<String> = {
                let sc = spread_calc.read().await;
                sc.active_coins().iter().map(|s| s.to_string()).collect()
            };

            let result = ReselectionResult {
                new_coins: final_coins,
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

// ---------------------------------------------------------------------------
// 진입 검증 (9단계)
// ---------------------------------------------------------------------------

/// 진입 검증 결과.
enum EntryValidation {
    /// 검증 통과.
    Accepted {
        qty: Decimal,
        upbit_entry_usd: Decimal,
        bybit_entry: Decimal,
        adjusted_profit: f64,
    },
    /// 검증 실패 (사유).
    Rejected(String),
}

impl<U, B, P> ZScoreMonitor<U, B, P>
where
    U: MarketData + MarketStream + Send + Sync + 'static,
    B: MarketData + MarketStream + InstrumentDataProvider + Send + Sync + 'static,
    P: ExecutionPolicy,
{
    /// 진입 9단계 검증.
    #[allow(clippy::too_many_arguments)]
    fn validate_entry(
        coin: &str,
        size_usdt_f64: f64,
        bybit_price: Decimal,
        upbit_price: Decimal,
        usd_krw: f64,
        spread_pct: f64,
        expected_profit_pct: f64,
        inst: &instrument::InstrumentInfo,
        config: &ZScoreConfig,
    ) -> EntryValidation {
        let size_usdt = Decimal::try_from(size_usdt_f64).unwrap_or(Decimal::ZERO);

        // 1. USDT notional → qty 변환 + qty_step 라운딩
        let raw_qty = if bybit_price > Decimal::ZERO {
            size_usdt / bybit_price
        } else {
            Decimal::ZERO
        };
        let qty = instrument::round_qty_floor(raw_qty, inst.qty_step);

        // 2. qty == 0 → 진입 거부
        if qty.is_zero() {
            debug!(
                coin = coin,
                raw_qty = %raw_qty,
                qty_step = %inst.qty_step,
                "진입 거부: 라운딩 후 qty = 0"
            );
            return EntryValidation::Rejected("order_constraint".to_string());
        }

        // 3. 최소/최대 주문 검증
        let actual_size_usdt = qty * bybit_price;
        if qty < inst.min_order_qty
            || qty > inst.max_order_qty
            || actual_size_usdt < inst.min_notional
        {
            debug!(
                coin = coin,
                qty = %qty,
                min_order_qty = %inst.min_order_qty,
                max_order_qty = %inst.max_order_qty,
                actual_size_usdt = %actual_size_usdt,
                min_notional = %inst.min_notional,
                "진입 거부: Bybit 주문 조건 미달"
            );
            return EntryValidation::Rejected("order_constraint".to_string());
        }

        // 4. Upbit KRW 최소 주문 검증 (5100원)
        let upbit_krw_notional = qty * upbit_price;
        if upbit_krw_notional < Decimal::new(5100, 0) {
            debug!(
                coin = coin,
                upbit_krw_notional = %upbit_krw_notional,
                "진입 거부: Upbit KRW 최소 주문 미달"
            );
            return EntryValidation::Rejected("order_constraint".to_string());
        }

        // 5. 가격 라운딩
        // Upbit 매수: ceil (더 비싸게 삼 = 불리한 방향)
        let upbit_entry_krw =
            instrument::ceil_to_step(upbit_price, instrument::upbit_tick_size(upbit_price));
        let upbit_entry_usd = Decimal::try_from(upbit_entry_krw.to_f64().unwrap_or(0.0) / usd_krw)
            .unwrap_or(Decimal::ZERO);
        // Bybit short (매도): floor
        let bybit_entry = instrument::round_price_conservative(bybit_price, inst.tick_size, false);

        // 6. Post-rounding PnL gate
        let adjusted_spread = if upbit_entry_usd > Decimal::ZERO {
            let bybit_f = bybit_entry.to_f64().unwrap_or(0.0);
            let upbit_f = upbit_entry_usd.to_f64().unwrap_or(0.0);
            (bybit_f - upbit_f) / upbit_f * 100.0
        } else {
            0.0
        };
        let rounding_cost = spread_pct - adjusted_spread;
        let adjusted_profit = expected_profit_pct - rounding_cost;
        if adjusted_profit <= 0.0 {
            debug!(
                coin = coin,
                original_profit = expected_profit_pct,
                adjusted_profit = adjusted_profit,
                rounding_cost = rounding_cost,
                "진입 거부: 라운딩 후 수익성 부족"
            );
            return EntryValidation::Rejected("rounding_pnl".to_string());
        }

        // 7. 최소 포지션 크기 체크
        if config.min_position_usdt > Decimal::ZERO
            && (qty * bybit_entry) < config.min_position_usdt
        {
            debug!(
                coin = coin,
                rounded_size_usdt = %(qty * bybit_entry),
                min_position_usdt = %config.min_position_usdt,
                "진입 거부: 최소 포지션 크기 미달"
            );
            return EntryValidation::Rejected("min_position".to_string());
        }

        // 8. 최소 기대 수익률 체크
        if config.min_expected_roi > 0.0 && adjusted_profit < config.min_expected_roi {
            debug!(
                coin = coin,
                adjusted_profit = adjusted_profit,
                min_expected_roi = config.min_expected_roi,
                "진입 거부: 최소 기대 수익률 미달"
            );
            return EntryValidation::Rejected("min_roi".to_string());
        }

        // 9. 모든 검증 통과
        EntryValidation::Accepted {
            qty,
            upbit_entry_usd,
            bybit_entry,
            adjusted_profit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zscore::coin_selector::CoinCandidate;
    use crate::zscore::monitor_sim::SimPolicy;

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
        assert!(!builder.upbit_last_trade.contains_key("USDT"));
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
            qty: Decimal::ONE,
            ..Default::default()
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
    #[allow(clippy::type_complexity)]
    fn make_shared_state() -> (
        Arc<ZScoreConfig>,
        Arc<ForexCache>,
        Arc<tokio::sync::Mutex<PositionManager>>,
        orderbook::SharedObCache,
        Arc<parking_lot::Mutex<MonitoringCounters>>,
        Arc<parking_lot::RwLock<InstrumentCache>>,
        Arc<SimPolicy>,
    ) {
        let config = Arc::new(ZScoreConfig::default());
        let forex_cache = Arc::new(ForexCache::new(Duration::from_secs(600)));
        let position_mgr = Arc::new(tokio::sync::Mutex::new(PositionManager::new()));
        let ob_cache = orderbook::SharedObCache::new();
        let counters = Arc::new(parking_lot::Mutex::new(MonitoringCounters::default()));
        let instrument_cache = Arc::new(parking_lot::RwLock::new(InstrumentCache::default()));
        let policy = Arc::new(SimPolicy::new());
        (
            config,
            forex_cache,
            position_mgr,
            ob_cache,
            counters,
            instrument_cache,
            policy,
        )
    }

    #[tokio::test]
    async fn test_maybe_spawn_no_data() {
        // candle_builder에 데이터 없음 -> spawn 안 됨 (즉시 리턴)
        let (config, forex_cache, position_mgr, ob_cache, counters, instrument_cache, policy) =
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

        // candle_builder는 비어있으므로 upbit_last_trade에 BTC 없음 -> 즉시 리턴
        ZScoreMonitor::<MockMarket, MockMarket, SimPolicy>::maybe_spawn_tick_signal(
            &event,
            &coins,
            &candle_builder,
            &spread_calc,
            &config,
            &forex_cache,
            &position_mgr,
            &ob_cache,
            &counters,
            &upbit,
            &bybit,
            &instrument_cache,
            &policy,
        )
        .await;

        // spawn 안 됐으므로 computing flag는 false
        assert!(
            !ob_cache
                .computing
                .is_computing(orderbook::Exchange::Upbit, "BTC")
        );
    }

    #[tokio::test]
    async fn test_maybe_spawn_partial_data() {
        // Upbit 데이터만 있고 Bybit 없음 -> spawn 안 됨
        let (config, forex_cache, position_mgr, ob_cache, counters, instrument_cache, policy) =
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

        ZScoreMonitor::<MockMarket, MockMarket, SimPolicy>::maybe_spawn_tick_signal(
            &event,
            &coins,
            &candle_builder,
            &spread_calc,
            &config,
            &forex_cache,
            &position_mgr,
            &ob_cache,
            &counters,
            &upbit,
            &bybit,
            &instrument_cache,
            &policy,
        )
        .await;

        // bybit_last_bid에 BTC 없으므로 즉시 리턴
        assert!(
            !ob_cache
                .computing
                .is_computing(orderbook::Exchange::Upbit, "BTC")
        );
    }

    #[tokio::test]
    async fn test_maybe_spawn_window_not_ready() {
        // 양쪽 데이터 있지만 SpreadCalculator 윈도우 미충족 -> spawn 안 됨
        let (config, forex_cache, position_mgr, ob_cache, counters, instrument_cache, policy) =
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

        ZScoreMonitor::<MockMarket, MockMarket, SimPolicy>::maybe_spawn_tick_signal(
            &event,
            &coins,
            &candle_builder,
            &spread_calc,
            &config,
            &forex_cache,
            &position_mgr,
            &ob_cache,
            &counters,
            &upbit,
            &bybit,
            &instrument_cache,
            &policy,
        )
        .await;

        // cached_stats가 None이므로 spawn 전에 리턴
        assert!(
            !ob_cache
                .computing
                .is_computing(orderbook::Exchange::Upbit, "BTC")
        );
    }

    #[tokio::test]
    async fn test_maybe_spawn_coin_not_in_list() {
        // 이벤트 코인이 current_coins에 없음 -> spawn 안 됨
        let (config, forex_cache, position_mgr, ob_cache, counters, instrument_cache, policy) =
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

        ZScoreMonitor::<MockMarket, MockMarket, SimPolicy>::maybe_spawn_tick_signal(
            &event,
            &coins,
            &candle_builder,
            &spread_calc,
            &config,
            &forex_cache,
            &position_mgr,
            &ob_cache,
            &counters,
            &upbit,
            &bybit,
            &instrument_cache,
            &policy,
        )
        .await;

        assert!(
            !ob_cache
                .computing
                .is_computing(orderbook::Exchange::Upbit, "ETH")
        );
    }

    #[tokio::test]
    async fn test_computing_flag_prevents_duplicate_spawn() {
        // computing flag가 이미 설정된 경우 -> dropped_tick_count 증가
        let (config, forex_cache, position_mgr, ob_cache, counters, instrument_cache, policy) =
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
        assert!(
            !ob_cache
                .computing
                .try_set_computing(orderbook::Exchange::Upbit, "BTC")
        );

        let event = MarketEvent::Trade {
            timestamp: Utc::now(),
            market: "KRW-BTC".to_string(),
            price: Decimal::new(138_000_000, 0),
            volume: Decimal::new(1, 2),
        };

        ZScoreMonitor::<MockMarket, MockMarket, SimPolicy>::maybe_spawn_tick_signal(
            &event,
            &coins,
            &candle_builder,
            &spread_calc,
            &config,
            &forex_cache,
            &position_mgr,
            &ob_cache,
            &counters,
            &upbit,
            &bybit,
            &instrument_cache,
            &policy,
        )
        .await;

        // computing flag가 이미 설정되어 있으므로 dropped_tick_count가 증가
        let c = counters.lock();
        assert_eq!(c.dropped_tick_count, 1);

        // flag 정리
        ob_cache
            .computing
            .clear_computing(orderbook::Exchange::Upbit, "BTC");
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

    impl InstrumentDataProvider for MockMarket {
        async fn get_instrument_info(
            &self,
            _symbol: &str,
        ) -> Result<arb_exchange::InstrumentInfoResponse, arb_exchange::ExchangeError> {
            unimplemented!()
        }
    }

    // --- filter_coins_by_stddev 테스트 ---

    #[test]
    fn test_filter_coins_by_stddev_basic() {
        let coin_stats = vec![
            ("BTC".to_string(), Some((0.0, 0.1))),
            ("ETH".to_string(), Some((0.0, 0.3))),
            ("XRP".to_string(), Some((0.0, 0.6))),
        ];
        let (kept, removed) = filter_coins_by_stddev(&coin_stats, 0.5, 5);
        assert_eq!(kept, vec!["BTC", "ETH"]);
        assert_eq!(removed, vec!["XRP"]);
    }

    #[test]
    fn test_filter_coins_by_stddev_max_coins_limit() {
        let coin_stats = vec![
            ("BTC".to_string(), Some((0.0, 0.1))),
            ("ETH".to_string(), Some((0.0, 0.2))),
            ("XRP".to_string(), Some((0.0, 0.3))),
        ];
        // 전부 통과하지만 max_coins=2
        let (kept, removed) = filter_coins_by_stddev(&coin_stats, 0.5, 2);
        assert_eq!(kept, vec!["BTC", "ETH"]);
        assert_eq!(removed, vec!["XRP"]);
    }

    #[test]
    fn test_filter_coins_by_stddev_none_stats_removed() {
        let coin_stats = vec![
            ("BTC".to_string(), Some((0.0, 0.1))),
            ("ETH".to_string(), None),
            ("XRP".to_string(), Some((0.0, 0.2))),
        ];
        let (kept, removed) = filter_coins_by_stddev(&coin_stats, 0.5, 5);
        assert_eq!(kept, vec!["BTC", "XRP"]);
        assert!(removed.contains(&"ETH".to_string()));
    }

    #[test]
    fn test_filter_coins_by_stddev_fallback_all_exceed() {
        let coin_stats = vec![
            ("BTC".to_string(), Some((0.0, 1.0))),
            ("ETH".to_string(), Some((0.0, 2.0))),
            ("XRP".to_string(), Some((0.0, 3.0))),
        ];
        // 전부 0.5 초과 -> fallback: stddev 오름차순 2개
        let (kept, removed) = filter_coins_by_stddev(&coin_stats, 0.5, 2);
        assert_eq!(kept, vec!["BTC", "ETH"]);
        assert_eq!(removed, vec!["XRP"]);
    }

    #[test]
    fn test_filter_coins_by_stddev_zero_disables_filter() {
        let coin_stats = vec![
            ("BTC".to_string(), Some((0.0, 5.0))),
            ("ETH".to_string(), Some((0.0, 0.1))),
            ("XRP".to_string(), Some((0.0, 2.0))),
        ];
        // max_spread_stddev=0.0이면 필터링 없이 stddev 오름차순 max_coins개
        let (kept, removed) = filter_coins_by_stddev(&coin_stats, 0.0, 2);
        // stddev 오름차순: ETH(0.1), XRP(2.0), BTC(5.0)
        assert_eq!(kept, vec!["ETH", "XRP"]);
        assert_eq!(removed, vec!["BTC"]);
    }

    #[test]
    fn test_filter_coins_by_stddev_all_none() {
        let coin_stats = vec![("BTC".to_string(), None), ("ETH".to_string(), None)];
        let (kept, removed) = filter_coins_by_stddev(&coin_stats, 0.5, 5);
        assert!(kept.is_empty());
        assert_eq!(removed.len(), 2);
    }

    #[test]
    fn test_filter_coins_by_stddev_sorted_by_stddev() {
        let coin_stats = vec![
            ("C".to_string(), Some((0.0, 0.4))),
            ("A".to_string(), Some((0.0, 0.1))),
            ("B".to_string(), Some((0.0, 0.2))),
        ];
        let (kept, _removed) = filter_coins_by_stddev(&coin_stats, 0.5, 5);
        // stddev 오름차순: A(0.1), B(0.2), C(0.4)
        assert_eq!(kept, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_filter_coins_by_stddev_exact_threshold() {
        let coin_stats = vec![
            ("BTC".to_string(), Some((0.0, 0.5))),
            ("ETH".to_string(), Some((0.0, 0.50001))),
        ];
        // 0.5 이하면 통과, 0.50001은 초과
        let (kept, removed) = filter_coins_by_stddev(&coin_stats, 0.5, 5);
        assert_eq!(kept, vec!["BTC"]);
        assert_eq!(removed, vec!["ETH"]);
    }
}
