//! 히스토리컬 백테스트 시뮬레이터.
//!
//! REST API로 캔들 데이터를 수집하고, 워밍업 후 순차 시뮬레이션을 수행합니다.

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::time::Duration;

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use tracing::{debug, info, warn};

use arb_exchange::MarketData;
use arb_exchange::types::{Candle, CandleInterval};

use crate::common::statistics;
use crate::error::StrategyError;
use crate::zscore::config::ZScoreConfig;
use crate::zscore::pnl::{self, ClosedPosition};
use crate::zscore::position::{self, PositionManager, VirtualPosition};
use crate::zscore::signal::{self, Signal};
use crate::zscore::slippage;
use crate::zscore::spread::SpreadCalculator;

/// 정상성 검증 메트릭 (예약).
#[derive(Debug, Clone)]
pub struct StationarityMetrics {
    /// ADF 검정 p-value (< 0.05이면 정상성).
    pub adf_p_value: Option<f64>,
    /// Hurst exponent (< 0.5이면 mean-reverting).
    pub hurst_exponent: Option<f64>,
    /// OU 프로세스 half-life 추정치 (분 단위).
    pub ou_half_life: Option<f64>,
}

/// 백테스트 결과.
#[derive(Debug, Clone)]
pub struct BacktestResult {
    /// 사용된 설정.
    pub config: ZScoreConfig,
    /// 테스트 기간 시작.
    pub test_period_start: DateTime<Utc>,
    /// 테스트 기간 종료.
    pub test_period_end: DateTime<Utc>,
    /// 총 거래 횟수.
    pub total_trades: usize,
    /// 수익 거래 횟수.
    pub winning_trades: usize,
    /// 손실 거래 횟수.
    pub losing_trades: usize,
    /// 강제 청산된 거래 횟수.
    pub liquidated_trades: usize,
    /// 승률.
    pub win_rate: f64,
    /// 총 gross PnL (수수료 차감 전).
    pub total_pnl: Decimal,
    /// 총 수수료.
    pub total_fees: Decimal,
    /// 순 PnL (수수료 차감 후).
    pub net_pnl: Decimal,
    /// 최대 낙폭 (USDT 절대값, equity curve peak-to-trough).
    pub max_drawdown: Decimal,
    /// 평균 보유 시간 (분).
    pub avg_holding_minutes: f64,
    /// 청산된 거래 내역.
    pub trades: Vec<ClosedPosition>,
    /// 미청산 포지션.
    pub open_positions: Vec<VirtualPosition>,
    /// 미청산 포지션의 unrealized PnL.
    pub unrealized_pnl: Decimal,
    /// 일별 PnL 집계.
    pub daily_pnl: Vec<(NaiveDate, Decimal)>,
    /// 정상성 메트릭 (예약).
    pub stationarity_metrics: Option<StationarityMetrics>,
    /// 실측 half-life (분 단위).
    pub estimated_half_life: Option<f64>,
}

/// 캔들 데이터 캐시.
///
/// API에서 수집한 캔들 데이터와 정렬된 공통 타임스탬프를 보관합니다.
/// sweep 등에서 데이터를 한 번만 수집하고 여러 번 시뮬레이션할 때 사용합니다.
pub struct CandleDataCache {
    /// Upbit 코인별 캔들 (key: 코인 심볼).
    pub upbit_coin_candles: HashMap<String, Vec<Candle>>,
    /// Upbit USDT/KRW 캔들.
    pub upbit_usdt_candles: Vec<Candle>,
    /// Bybit 코인별 캔들 (key: 코인 심볼).
    pub bybit_candles: HashMap<String, Vec<Candle>>,
    /// 정렬된 공통 타임스탬프.
    pub timestamps: Vec<DateTime<Utc>>,
}

/// 페이지네이션으로 전체 기간 캔들을 수집합니다.
///
/// `get_candles_before`는 `before` exclusive이고 결과를 timestamp 오름차순 반환.
pub async fn fetch_all_candles<M: MarketData>(
    client: &M,
    market: &str,
    interval: CandleInterval,
    total_count: usize,
    end_time: DateTime<Utc>,
    page_size: u32,
    delay: Duration,
) -> Result<Vec<Candle>, StrategyError> {
    let mut collected: Vec<Candle> = Vec::new();
    let mut cursor = end_time;

    while collected.len() < total_count {
        tokio::time::sleep(delay).await;

        let batch = client
            .get_candles_before(market, interval, page_size, cursor)
            .await?;

        if batch.is_empty() {
            break;
        }

        // batch는 오름차순 (trait 규약)
        // 중복 제거: collected에 이미 있는 데이터보다 이전인 캔들만 유지
        let filtered: Vec<Candle> = if collected.is_empty() {
            batch
        } else {
            let earliest = collected.first().unwrap().timestamp;
            batch
                .into_iter()
                .filter(|c| c.timestamp < earliest)
                .collect()
        };

        if filtered.is_empty() {
            break;
        }

        // 커서를 가장 오래된 캔들 시간으로 이동
        cursor = filtered.first().unwrap().timestamp;

        // prepend: 역순 수집 후 나중에 정렬
        let mut new_collected = filtered;
        new_collected.append(&mut collected);
        collected = new_collected;
    }

    // 필요 개수 초과분 제거 (앞쪽에서 자름)
    if collected.len() > total_count {
        collected = collected.split_off(collected.len() - total_count);
    }

    debug!(
        market = market,
        collected = collected.len(),
        total_needed = total_count,
        "캔들 데이터 수집 완료"
    );

    Ok(collected)
}

/// 시간 정렬된 캔들 데이터에서 특정 timestamp의 close 가격을 조회합니다.
/// 없으면 None 반환 (forward-fill은 SpreadCalculator가 처리).
fn find_close_at(candles: &[Candle], target: DateTime<Utc>) -> Option<Decimal> {
    candles
        .iter()
        .find(|c| c.timestamp == target)
        .map(|c| c.close)
}

/// 시간 정렬된 캔들 데이터에서 특정 timestamp의 volume을 조회합니다.
fn find_volume_at(candles: &[Candle], target: DateTime<Utc>) -> Option<Decimal> {
    candles
        .iter()
        .find(|c| c.timestamp == target)
        .map(|c| c.volume)
}

/// 모든 시계열에서 합집합 타임스탬프를 오름차순으로 생성합니다.
fn build_aligned_timestamps(
    upbit_coin: &HashMap<String, Vec<Candle>>,
    usdt_krw: &[Candle],
    bybit: &HashMap<String, Vec<Candle>>,
) -> Vec<DateTime<Utc>> {
    let mut all_timestamps = BTreeSet::new();

    for candles in upbit_coin.values() {
        for c in candles {
            all_timestamps.insert(c.timestamp);
        }
    }
    for c in usdt_krw {
        all_timestamps.insert(c.timestamp);
    }
    for candles in bybit.values() {
        for c in candles {
            all_timestamps.insert(c.timestamp);
        }
    }

    all_timestamps.into_iter().collect()
}

/// BacktestSimulator를 거치지 않고 직접 캔들 데이터를 수집합니다.
///
/// sweep 등에서 BacktestSimulator 소유권 문제 없이 데이터를 수집할 때 사용합니다.
pub async fn fetch_candle_data<U: MarketData, B: MarketData>(
    upbit: &U,
    bybit: &B,
    config: &ZScoreConfig,
) -> Result<CandleDataCache, StrategyError> {
    config.validate()?;

    let total_candles = config.total_candles_needed();
    let end_time = Utc::now();

    info!(
        coins = ?config.coins,
        window_size = config.window_size,
        backtest_minutes = config.backtest_period_minutes,
        total_candles = total_candles,
        "캔들 데이터 수집 시작"
    );

    // 코인별 캔들 수집
    let mut upbit_coin_candles: HashMap<String, Vec<Candle>> = HashMap::new();
    let mut bybit_candles: HashMap<String, Vec<Candle>> = HashMap::new();

    for coin in &config.coins {
        let upbit_market = format!("KRW-{coin}");
        let bybit_market = format!("{coin}USDT");

        let upbit_data = fetch_all_candles(
            upbit,
            &upbit_market,
            config.candle_interval,
            total_candles,
            end_time,
            200,                        // Upbit 페이지 크기
            Duration::from_millis(100), // Upbit rate limit
        )
        .await?;

        let bybit_data = fetch_all_candles(
            bybit,
            &bybit_market,
            config.candle_interval,
            total_candles,
            end_time,
            1000,                      // Bybit 페이지 크기
            Duration::from_millis(10), // Bybit rate limit
        )
        .await?;

        info!(
            coin = coin.as_str(),
            upbit_count = upbit_data.len(),
            bybit_count = bybit_data.len(),
            "코인 캔들 데이터 수집 완료"
        );

        upbit_coin_candles.insert(coin.clone(), upbit_data);
        bybit_candles.insert(coin.clone(), bybit_data);
    }

    // USDT/KRW 캔들 수집
    let upbit_usdt_candles = fetch_all_candles(
        upbit,
        "KRW-USDT",
        config.candle_interval,
        total_candles,
        end_time,
        200,
        Duration::from_millis(100),
    )
    .await?;

    info!(
        usdt_krw_count = upbit_usdt_candles.len(),
        "USDT/KRW 캔들 데이터 수집 완료"
    );

    // 시간 정렬
    let timestamps =
        build_aligned_timestamps(&upbit_coin_candles, &upbit_usdt_candles, &bybit_candles);

    Ok(CandleDataCache {
        upbit_coin_candles,
        upbit_usdt_candles,
        bybit_candles,
        timestamps,
    })
}

/// 캐시된 데이터로 시뮬레이션을 실행합니다 (API 호출 없음).
///
/// sweep에서 동일 데이터로 여러 파라미터 조합을 테스트할 때 사용합니다.
#[allow(clippy::too_many_lines)]
pub fn simulate_with_cache(
    config: &ZScoreConfig,
    cache: &CandleDataCache,
) -> Result<BacktestResult, StrategyError> {
    config.validate()?;

    let timestamps = &cache.timestamps;

    if timestamps.len() < config.window_size {
        return Err(StrategyError::DataAlignment(format!(
            "aligned timestamps ({}) < window_size ({})",
            timestamps.len(),
            config.window_size
        )));
    }

    info!(
        total_timestamps = timestamps.len(),
        warmup = config.window_size,
        simulation = timestamps.len().saturating_sub(config.window_size),
        "시간 정렬 완료"
    );

    // SpreadCalculator + PositionManager 초기화
    let mut spread_calc = SpreadCalculator::new(&config.coins, config.window_size);
    let mut position_mgr = PositionManager::new();

    // 시계열 기록 (CSV 출력용)
    let mut timeseries_records: Vec<TimeseriesRecord> = Vec::new();

    // 이전 mean 추적 (regime change 감지)
    let mut prev_means: HashMap<String, f64> = HashMap::new();

    // 워밍업 + 시뮬레이션
    for (idx, &ts) in timestamps.iter().enumerate() {
        let is_warmup = idx < config.window_size;

        for coin in &config.coins {
            // 각 소스에서 해당 timestamp의 close 조회
            let upbit_close = cache
                .upbit_coin_candles
                .get(coin)
                .and_then(|candles| find_close_at(candles, ts));
            let usdt_krw_close = find_close_at(&cache.upbit_usdt_candles, ts);
            let bybit_close = cache
                .bybit_candles
                .get(coin)
                .and_then(|candles| find_close_at(candles, ts));

            // SpreadCalculator 업데이트 (forward-fill 내장)
            spread_calc.update(coin, ts, upbit_close, usdt_krw_close, bybit_close)?;

            if is_warmup {
                continue;
            }

            // Bybit liquidation 체크
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

                warn!(
                    coin = coin.as_str(),
                    bybit_price = %bybit_price,
                    "Bybit 강제 청산 발생"
                );

                position_mgr
                    .close_position(
                        coin,
                        ts,
                        upbit_usdt,
                        bybit_price,
                        usdt_krw_val,
                        spread_pct,
                        f64::NAN, // liquidation 시 z-score 무의미
                        config.upbit_taker_fee,
                        config.bybit_taker_fee,
                        true,
                    )
                    .ok(); // 이미 없으면 무시
            }

            // 시그널 평가
            let sig = signal::evaluate_signal(coin, &spread_calc, &position_mgr, config)?;

            // 시계열 기록
            let spread_window = spread_calc.spread_window(coin);
            let (mean_pct, stddev_val, z_val) = if let Some(w) = spread_window {
                if w.is_ready() {
                    let data = w.data();
                    let m = statistics::mean(data);
                    let s = statistics::stddev(data, m);
                    let z = if s >= config.min_stddev_threshold {
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

            timeseries_records.push(TimeseriesRecord {
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

            // Regime change 감지
            if spread_window.map(|w| w.is_ready()).unwrap_or(false) {
                if let Some(&prev_mean) = prev_means.get(coin)
                    && stddev_val > 0.0
                    && (mean_pct - prev_mean).abs() > 2.0 * stddev_val
                {
                    warn!(
                        coin = coin.as_str(),
                        prev_mean = prev_mean,
                        current_mean = mean_pct,
                        "Regime change 감지: rolling mean이 2σ 이상 이동"
                    );
                }
                prev_means.insert(coin.clone(), mean_pct);
            }

            // 포지션 보유 시간 경고
            if let Some(pos) = position_mgr.open_positions.get(coin) {
                let holding = (ts - pos.entry_time).num_minutes().unsigned_abs();
                if holding > (config.window_size * 2) as u64 {
                    warn!(
                        coin = coin.as_str(),
                        holding_minutes = holding,
                        "포지션 보유 시간이 window_size의 2배 초과: {}분",
                        holding
                    );
                }
            }

            // 시그널 처리
            match sig {
                Some(Signal::Enter {
                    coin: c,
                    z_score,
                    spread_pct,
                    expected_profit_pct,
                }) => {
                    let size_usdt = config.total_capital_usdt * config.position_ratio;
                    let upbit_usdt = spread_calc
                        .upbit_window(&c)
                        .and_then(|w| w.last())
                        .unwrap_or(0.0);
                    let bybit_f64 = spread_calc
                        .bybit_window(&c)
                        .and_then(|w| w.last())
                        .unwrap_or(0.0);
                    let usdt_krw_val = spread_calc.usdt_krw_window().last().unwrap_or(0.0);

                    let mut upbit_price = Decimal::try_from(upbit_usdt).unwrap_or(Decimal::ZERO);
                    let mut bybit_price_dec = Decimal::try_from(bybit_f64).unwrap_or(Decimal::ZERO);
                    let usdt_krw_dec = Decimal::try_from(usdt_krw_val).unwrap_or(Decimal::ZERO);

                    // 볼륨/슬리피지 적용
                    if config.volume_filter_enabled {
                        let mut upbit_slippage_bps = 0.0_f64;
                        let mut bybit_slippage_bps = 0.0_f64;

                        let upbit_vol = cache
                            .upbit_coin_candles
                            .get(&c)
                            .and_then(|candles| find_volume_at(candles, ts));
                        let bybit_vol = cache
                            .bybit_candles
                            .get(&c)
                            .and_then(|candles| find_volume_at(candles, ts));

                        // Upbit 슬리피지 (매수 → 가격 상승)
                        if let Some(uv) = upbit_vol {
                            match slippage::calculate_slippage(
                                size_usdt,
                                uv,
                                upbit_price,
                                true, // Upbit: 코인 수량
                                true, // 매수
                                config.max_participation_rate,
                                config.slippage_base_bps,
                                config.slippage_impact_coeff,
                            ) {
                                Some(r) => {
                                    debug!(
                                        coin = c.as_str(),
                                        exchange = "upbit",
                                        participation_rate = r.participation_rate,
                                        slippage_bps = r.slippage_bps,
                                        "진입 슬리피지 적용"
                                    );
                                    upbit_slippage_bps = r.slippage_bps;
                                    upbit_price = r.adjusted_price;
                                }
                                None => {
                                    debug!(
                                        coin = c.as_str(),
                                        exchange = "upbit",
                                        "진입 거부: Upbit 볼륨 부족"
                                    );
                                    continue;
                                }
                            }
                        }

                        // Bybit 슬리피지 (short 진입 → 매도 → 가격 하락)
                        if let Some(bv) = bybit_vol {
                            match slippage::calculate_slippage(
                                size_usdt,
                                bv,
                                bybit_price_dec,
                                true,  // Bybit: 코인 수량 (volume=base coin, turnover=USDT)
                                false, // short = 매도
                                config.max_participation_rate,
                                config.slippage_base_bps,
                                config.slippage_impact_coeff,
                            ) {
                                Some(r) => {
                                    debug!(
                                        coin = c.as_str(),
                                        exchange = "bybit",
                                        participation_rate = r.participation_rate,
                                        slippage_bps = r.slippage_bps,
                                        "진입 슬리피지 적용"
                                    );
                                    bybit_slippage_bps = r.slippage_bps;
                                    bybit_price_dec = r.adjusted_price;
                                }
                                None => {
                                    debug!(
                                        coin = c.as_str(),
                                        exchange = "bybit",
                                        "진입 거부: Bybit 볼륨 부족"
                                    );
                                    continue;
                                }
                            }
                        }

                        // 슬리피지 포함 수익성 재검증
                        let (profitable, adj_profit) = slippage::is_entry_profitable(
                            upbit_price,
                            bybit_price_dec,
                            mean_pct,
                            upbit_slippage_bps,
                            bybit_slippage_bps,
                            config.upbit_taker_fee,
                            config.bybit_taker_fee,
                        );

                        if !profitable {
                            debug!(
                                coin = c.as_str(),
                                adjusted_profit_pct = adj_profit,
                                original_expected_profit = expected_profit_pct,
                                upbit_slippage_bps,
                                bybit_slippage_bps,
                                "진입 거부: 슬리피지 포함 시 수익성 부족"
                            );
                            continue;
                        }

                        debug!(
                            coin = c.as_str(),
                            adjusted_profit_pct = adj_profit,
                            "슬리피지 포함 수익성 확인됨"
                        );
                    }

                    let liq_price = position::calculate_liquidation_price(
                        bybit_price_dec,
                        config.leverage,
                        config.bybit_mmr,
                        config.bybit_taker_fee,
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
                        size_usdt = %size_usdt,
                        volume_filter = config.volume_filter_enabled,
                        "진입 시그널 실행"
                    );

                    position_mgr.open_position(pos).ok();
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

                    let mut exit_upbit = Decimal::try_from(upbit_usdt).unwrap_or(Decimal::ZERO);
                    let mut exit_bybit = Decimal::try_from(bybit_f64).unwrap_or(Decimal::ZERO);

                    // 청산 슬리피지 적용 (청산은 거부하지 않고 항상 실행)
                    if config.volume_filter_enabled {
                        let size_usdt = position_mgr
                            .open_positions
                            .get(c.as_str())
                            .map(|p| p.size_usdt)
                            .unwrap_or(config.total_capital_usdt * config.position_ratio);

                        let upbit_vol = cache
                            .upbit_coin_candles
                            .get(&c)
                            .and_then(|candles| find_volume_at(candles, ts));
                        let bybit_vol = cache
                            .bybit_candles
                            .get(&c)
                            .and_then(|candles| find_volume_at(candles, ts));

                        // Upbit 청산: 매도 → 가격 하락
                        if let Some(uv) = upbit_vol
                            && let Some(r) = slippage::calculate_slippage(
                                size_usdt,
                                uv,
                                exit_upbit,
                                true,  // Upbit: 코인 수량
                                false, // 매도
                                1.0,   // 청산은 참여율 무시 (항상 실행)
                                config.slippage_base_bps,
                                config.slippage_impact_coeff,
                            )
                        {
                            debug!(
                                coin = c.as_str(),
                                exchange = "upbit",
                                slippage_bps = r.slippage_bps,
                                "청산 슬리피지 적용"
                            );
                            exit_upbit = r.adjusted_price;
                        }

                        // Bybit 청산: short close = 매수 → 가격 상승
                        if let Some(bv) = bybit_vol
                            && let Some(r) = slippage::calculate_slippage(
                                size_usdt,
                                bv,
                                exit_bybit,
                                true, // Bybit: 코인 수량 (volume=base coin, turnover=USDT)
                                true, // 매수 (short close)
                                1.0,  // 청산은 참여율 무시
                                config.slippage_base_bps,
                                config.slippage_impact_coeff,
                            )
                        {
                            debug!(
                                coin = c.as_str(),
                                exchange = "bybit",
                                slippage_bps = r.slippage_bps,
                                "청산 슬리피지 적용"
                            );
                            exit_bybit = r.adjusted_price;
                        }
                    }

                    info!(
                        coin = c.as_str(),
                        z_score = z_score,
                        spread_pct = spread_pct,
                        "청산 시그널 실행"
                    );

                    position_mgr
                        .close_position(
                            &c,
                            ts,
                            exit_upbit,
                            exit_bybit,
                            Decimal::try_from(usdt_krw_val).unwrap_or(Decimal::ZERO),
                            spread_pct,
                            z_score,
                            config.upbit_taker_fee,
                            config.bybit_taker_fee,
                            false,
                        )
                        .ok();
                }
                None => {}
            }
        }
    }

    // 결과 집계
    let trades = position_mgr.closed_positions.clone();
    let total_trades = trades.len();
    let winning_trades = trades.iter().filter(|t| t.net_pnl > Decimal::ZERO).count();
    let losing_trades = trades.iter().filter(|t| t.net_pnl < Decimal::ZERO).count();
    let liquidated_trades = trades.iter().filter(|t| t.is_liquidated).count();
    let win_rate = if total_trades > 0 {
        winning_trades as f64 / total_trades as f64
    } else {
        0.0
    };

    let total_pnl_gross: Decimal = trades.iter().map(|t| t.upbit_pnl + t.bybit_pnl).sum();
    let total_fees: Decimal = trades.iter().map(|t| t.total_fees).sum();
    let net_pnl: Decimal = trades.iter().map(|t| t.net_pnl).sum();
    let max_drawdown = pnl::calculate_max_drawdown(&trades);

    let avg_holding = if total_trades > 0 {
        trades.iter().map(|t| t.holding_minutes as f64).sum::<f64>() / total_trades as f64
    } else {
        0.0
    };

    // 미청산 포지션 unrealized PnL 계산
    let open_positions: Vec<VirtualPosition> =
        position_mgr.open_positions.values().cloned().collect();
    let mut unrealized_pnl = Decimal::ZERO;
    for pos in &open_positions {
        let current_upbit = spread_calc
            .upbit_window(&pos.coin)
            .and_then(|w| w.last())
            .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
            .unwrap_or(Decimal::ZERO);
        let current_bybit = spread_calc
            .bybit_window(&pos.coin)
            .and_then(|w| w.last())
            .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
            .unwrap_or(Decimal::ZERO);

        let upbit_qty = pos.size_usdt / pos.upbit_entry_price;
        let bybit_qty = pos.size_usdt / pos.bybit_entry_price;
        let upbit_pnl = (current_upbit - pos.upbit_entry_price) * upbit_qty;
        let bybit_pnl = (pos.bybit_entry_price - current_bybit) * bybit_qty;
        let fees =
            pos.size_usdt * (config.upbit_taker_fee + config.bybit_taker_fee) * Decimal::from(2u64);
        unrealized_pnl += upbit_pnl + bybit_pnl - fees;
    }

    let daily = pnl::daily_pnl(&trades);

    let now = Utc::now();
    let warmup_end = if timestamps.len() > config.window_size {
        timestamps[config.window_size]
    } else {
        *timestamps.last().unwrap_or(&now)
    };
    let test_period_end = *timestamps.last().unwrap_or(&now);

    if total_trades < 30 {
        warn!(
            "거래 횟수 {}회: 통계적 유의성이 부족합니다. \
             최소 30회 이상의 거래가 필요합니다.",
            total_trades
        );
    }

    let result = BacktestResult {
        config: config.clone(),
        test_period_start: warmup_end,
        test_period_end,
        total_trades,
        winning_trades,
        losing_trades,
        liquidated_trades,
        win_rate,
        total_pnl: total_pnl_gross,
        total_fees,
        net_pnl,
        max_drawdown,
        avg_holding_minutes: avg_holding,
        trades,
        open_positions,
        unrealized_pnl,
        daily_pnl: daily,
        stationarity_metrics: None,
        estimated_half_life: None,
    };

    info!(
        total_trades = result.total_trades,
        winning = result.winning_trades,
        losing = result.losing_trades,
        liquidated = result.liquidated_trades,
        win_rate = format!("{:.1}%", result.win_rate * 100.0),
        net_pnl = %result.net_pnl,
        max_drawdown = %result.max_drawdown,
        "백테스트 완료"
    );

    Ok(result)
}

/// 백테스트 시뮬레이터.
pub struct BacktestSimulator<U: MarketData, B: MarketData> {
    upbit: U,
    bybit: B,
    config: ZScoreConfig,
}

impl<U: MarketData, B: MarketData> BacktestSimulator<U, B> {
    /// 새 BacktestSimulator를 생성합니다.
    pub fn new(upbit: U, bybit: B, config: ZScoreConfig) -> Self {
        Self {
            upbit,
            bybit,
            config,
        }
    }

    /// 캔들 데이터를 수집합니다.
    pub async fn fetch_data(&self) -> Result<CandleDataCache, StrategyError> {
        fetch_candle_data(&self.upbit, &self.bybit, &self.config).await
    }

    /// 백테스트를 실행합니다.
    pub async fn run(&self) -> Result<BacktestResult, StrategyError> {
        let cache = self.fetch_data().await?;
        simulate_with_cache(&self.config, &cache)
    }

    /// 설정에 접근합니다 (테스트용).
    pub fn config(&self) -> &ZScoreConfig {
        &self.config
    }
}

/// 시계열 기록 (CSV 출력용).
#[derive(Debug, Clone)]
pub struct TimeseriesRecord {
    /// 타임스탬프.
    pub timestamp: DateTime<Utc>,
    /// 코인 심볼.
    pub coin: String,
    /// Upbit USDT 환산 가격.
    pub upbit_usdt_price: f64,
    /// Bybit 가격.
    pub bybit_price: f64,
    /// 스프레드 (%).
    pub spread_pct: f64,
    /// 평균 스프레드 (%).
    pub mean_spread_pct: f64,
    /// 표준편차.
    pub stddev: f64,
    /// Z-Score.
    pub z_score: f64,
    /// 시그널 ("ENTER", "EXIT", "NONE").
    pub signal: String,
    /// 포지션 상태 ("OPEN", "NONE").
    pub position: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_close_at() {
        let ts1 = Utc::now();
        let ts2 = ts1 + chrono::Duration::minutes(1);
        let candles = vec![
            Candle {
                market: "KRW-BTC".to_string(),
                timestamp: ts1,
                open: Decimal::new(100, 0),
                high: Decimal::new(101, 0),
                low: Decimal::new(99, 0),
                close: Decimal::new(100_500, 0),
                volume: Decimal::new(10, 0),
            },
            Candle {
                market: "KRW-BTC".to_string(),
                timestamp: ts2,
                open: Decimal::new(100_500, 0),
                high: Decimal::new(101_000, 0),
                low: Decimal::new(100_000, 0),
                close: Decimal::new(100_800, 0),
                volume: Decimal::new(5, 0),
            },
        ];

        assert_eq!(find_close_at(&candles, ts1), Some(Decimal::new(100_500, 0)));
        assert_eq!(find_close_at(&candles, ts2), Some(Decimal::new(100_800, 0)));
        assert_eq!(
            find_close_at(&candles, ts2 + chrono::Duration::minutes(1)),
            None
        );
    }

    #[test]
    fn test_stationarity_metrics_struct() {
        let metrics = StationarityMetrics {
            adf_p_value: Some(0.01),
            hurst_exponent: Some(0.3),
            ou_half_life: Some(300.0),
        };
        assert_eq!(metrics.adf_p_value, Some(0.01));
    }

    #[test]
    fn test_backtest_result_struct() {
        let config = ZScoreConfig::default();
        let result = BacktestResult {
            config,
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
        assert_eq!(result.total_trades, 0);
    }

    #[test]
    fn test_build_aligned_timestamps() {
        let ts1 = Utc::now();
        let ts2 = ts1 + chrono::Duration::minutes(1);
        let ts3 = ts1 + chrono::Duration::minutes(2);

        let mut upbit_coin: HashMap<String, Vec<Candle>> = HashMap::new();
        upbit_coin.insert(
            "BTC".to_string(),
            vec![Candle {
                market: "KRW-BTC".to_string(),
                timestamp: ts1,
                open: Decimal::ONE,
                high: Decimal::ONE,
                low: Decimal::ONE,
                close: Decimal::ONE,
                volume: Decimal::ONE,
            }],
        );

        let usdt_krw = vec![Candle {
            market: "KRW-USDT".to_string(),
            timestamp: ts2,
            open: Decimal::ONE,
            high: Decimal::ONE,
            low: Decimal::ONE,
            close: Decimal::ONE,
            volume: Decimal::ONE,
        }];

        let mut bybit: HashMap<String, Vec<Candle>> = HashMap::new();
        bybit.insert(
            "BTC".to_string(),
            vec![Candle {
                market: "BTCUSDT".to_string(),
                timestamp: ts3,
                open: Decimal::ONE,
                high: Decimal::ONE,
                low: Decimal::ONE,
                close: Decimal::ONE,
                volume: Decimal::ONE,
            }],
        );

        let result = build_aligned_timestamps(&upbit_coin, &usdt_krw, &bybit);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], ts1);
        assert_eq!(result[1], ts2);
        assert_eq!(result[2], ts3);
    }

    #[test]
    fn test_candle_data_cache_struct() {
        let cache = CandleDataCache {
            upbit_coin_candles: HashMap::new(),
            upbit_usdt_candles: vec![],
            bybit_candles: HashMap::new(),
            timestamps: vec![],
        };
        assert!(cache.timestamps.is_empty());
    }
}
