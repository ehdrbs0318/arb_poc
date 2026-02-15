//! 오더북 캐시 및 슬리피지 안전 볼륨 계산.
//!
//! 오더북 호가를 two-pointer 방식으로 소비하며 슬리피지+수수료 포함
//! 수익성을 검증하여 최대 안전 진입/청산 볼륨을 계산합니다.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use arb_exchange::OrderBook;
use rust_decimal::prelude::ToPrimitive;
use tracing::{debug, trace, warn};

/// 캐시된 오더북.
#[derive(Debug, Clone)]
pub struct CachedOrderBook {
    /// 오더북 스냅샷.
    pub orderbook: OrderBook,
    /// 조회 시각.
    pub fetched_at: Instant,
}

/// 거래소 식별자.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Exchange {
    /// Upbit (한국/KRW).
    Upbit,
    /// Bybit (글로벌/USDT).
    Bybit,
}

/// 오더북 캐시.
///
/// 거래소별, 코인별 오더북 스냅샷과 computing flag를 관리합니다.
#[derive(Debug)]
pub struct OrderBookCache {
    /// Upbit 오더북 캐시.
    upbit: HashMap<String, CachedOrderBook>,
    /// Bybit 오더북 캐시.
    bybit: HashMap<String, CachedOrderBook>,
    /// 오더북 조회 중 플래그 (거래소, 코인).
    computing_flags: HashMap<(Exchange, String), bool>,
}

impl OrderBookCache {
    /// 새 OrderBookCache를 생성합니다.
    pub fn new() -> Self {
        Self {
            upbit: HashMap::new(),
            bybit: HashMap::new(),
            computing_flags: HashMap::new(),
        }
    }

    /// 오더북 캐시를 갱신합니다.
    pub fn update(&mut self, exchange: Exchange, coin: &str, ob: OrderBook) {
        let cached = CachedOrderBook {
            orderbook: ob,
            fetched_at: Instant::now(),
        };
        let map = match exchange {
            Exchange::Upbit => &mut self.upbit,
            Exchange::Bybit => &mut self.bybit,
        };
        map.insert(coin.to_string(), cached);
        trace!(exchange = ?exchange, coin = %coin, "오더북 캐시 갱신");
    }

    /// 캐시된 오더북을 조회합니다.
    pub fn get(&self, exchange: Exchange, coin: &str) -> Option<&CachedOrderBook> {
        let map = match exchange {
            Exchange::Upbit => &self.upbit,
            Exchange::Bybit => &self.bybit,
        };
        map.get(coin)
    }

    /// 캐시가 `max_age_sec` 이내인지 확인합니다.
    pub fn is_fresh(&self, exchange: Exchange, coin: &str, max_age_sec: u64) -> bool {
        self.get(exchange, coin)
            .map(|cached| cached.fetched_at.elapsed().as_secs() < max_age_sec)
            .unwrap_or(false)
    }

    /// computing flag를 확인합니다.
    pub fn is_computing(&self, exchange: Exchange, coin: &str) -> bool {
        self.computing_flags
            .get(&(exchange, coin.to_string()))
            .copied()
            .unwrap_or(false)
    }

    /// computing flag를 설정합니다.
    pub fn set_computing(&mut self, exchange: Exchange, coin: &str, value: bool) {
        self.computing_flags
            .insert((exchange, coin.to_string()), value);
        debug!(exchange = ?exchange, coin = %coin, computing = value, "computing flag 설정");
    }
}

impl Default for OrderBookCache {
    fn default() -> Self {
        Self::new()
    }
}

/// 오더북 데이터 캐시 (데이터 전용).
///
/// 거래소별, 코인별 오더북 스냅샷을 보관합니다.
/// `SharedObCache`에서 `tokio::sync::RwLock`으로 감싸 사용합니다.
#[derive(Debug)]
pub struct ObCacheData {
    /// Upbit 오더북 캐시.
    upbit: HashMap<String, CachedOrderBook>,
    /// Bybit 오더북 캐시.
    bybit: HashMap<String, CachedOrderBook>,
}

impl ObCacheData {
    /// 새 ObCacheData를 생성합니다.
    pub fn new() -> Self {
        Self {
            upbit: HashMap::new(),
            bybit: HashMap::new(),
        }
    }

    /// 오더북 캐시를 갱신합니다.
    pub fn update(&mut self, exchange: Exchange, coin: &str, ob: OrderBook) {
        let cached = CachedOrderBook {
            orderbook: ob,
            fetched_at: Instant::now(),
        };
        let map = match exchange {
            Exchange::Upbit => &mut self.upbit,
            Exchange::Bybit => &mut self.bybit,
        };
        map.insert(coin.to_string(), cached);
        trace!(exchange = ?exchange, coin = %coin, "오더북 캐시 갱신");
    }

    /// 캐시된 오더북을 조회합니다.
    pub fn get(&self, exchange: Exchange, coin: &str) -> Option<&CachedOrderBook> {
        let map = match exchange {
            Exchange::Upbit => &self.upbit,
            Exchange::Bybit => &self.bybit,
        };
        map.get(coin)
    }

    /// 캐시가 `max_age_sec` 이내인지 확인합니다.
    pub fn is_fresh(&self, exchange: Exchange, coin: &str, max_age_sec: u64) -> bool {
        self.get(exchange, coin)
            .map(|cached| cached.fetched_at.elapsed().as_secs() < max_age_sec)
            .unwrap_or(false)
    }

    /// 코인 관련 캐시를 양쪽 거래소에서 제거합니다.
    pub fn remove_coin(&mut self, coin: &str) {
        self.upbit.remove(coin);
        self.bybit.remove(coin);
    }
}

impl Default for ObCacheData {
    fn default() -> Self {
        Self::new()
    }
}

/// Computing flag 관리자 (동시성 안전).
///
/// `parking_lot::Mutex`로 보호되며, `try_set_computing`으로 원자적
/// check-and-set을 제공합니다. lock hold 시간이 극히 짧아
/// async context에서도 안전하게 사용할 수 있습니다.
#[derive(Debug)]
pub struct ComputingFlags {
    /// 거래소/코인별 computing flag.
    inner: parking_lot::Mutex<HashMap<(Exchange, String), bool>>,
}

impl ComputingFlags {
    /// 새 ComputingFlags를 생성합니다.
    pub fn new() -> Self {
        Self {
            inner: parking_lot::Mutex::new(HashMap::new()),
        }
    }

    /// 원자적 check-and-set. 이미 computing 중이면 `true` 반환 (스킵해야 함).
    ///
    /// 단일 lock 내에서 확인과 설정을 동시에 수행하여
    /// 코인당 최대 1개 task만 REST 호출하도록 보장합니다.
    ///
    /// # 반환값
    ///
    /// - `true`: 이미 다른 task가 computing 중 (이 task는 스킵)
    /// - `false`: 설정 성공 (이 task가 REST 수행)
    pub fn try_set_computing(&self, exchange: Exchange, coin: &str) -> bool {
        let mut flags = self.inner.lock();
        let entry = flags.entry((exchange, coin.to_string())).or_insert(false);
        if *entry {
            true // 이미 computing 중
        } else {
            *entry = true;
            debug!(exchange = ?exchange, coin = %coin, "computing flag 설정 (CAS)");
            false // 설정 성공
        }
    }

    /// computing flag를 해제합니다.
    ///
    /// task 완료 시 반드시 호출하여 다음 task가 실행될 수 있도록 합니다.
    /// 에러 발생 시에도 반드시 호출해야 합니다.
    pub fn clear_computing(&self, exchange: Exchange, coin: &str) {
        let mut flags = self.inner.lock();
        flags.insert((exchange, coin.to_string()), false);
        debug!(exchange = ?exchange, coin = %coin, "computing flag 해제");
    }

    /// 특정 코인의 computing flag를 양쪽 거래소에서 제거합니다.
    pub fn remove_coin(&self, coin: &str) {
        let mut flags = self.inner.lock();
        flags.retain(|k, _| k.1 != coin);
    }

    /// computing flag 상태를 조회합니다 (테스트/디버깅용).
    pub fn is_computing(&self, exchange: Exchange, coin: &str) -> bool {
        let flags = self.inner.lock();
        flags
            .get(&(exchange, coin.to_string()))
            .copied()
            .unwrap_or(false)
    }
}

impl Default for ComputingFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// 공유 오더북 캐시.
///
/// 데이터 캐시(`ObCacheData`)와 computing flag(`ComputingFlags`)를
/// 분리하여 lock 경합을 최소화합니다.
///
/// - `data`: `tokio::sync::RwLock`으로 보호 (읽기 빈번, 쓰기는 REST 완료 시만)
/// - `computing`: `parking_lot::Mutex` 기반 atomic CAS (lock hold 극히 짧음)
#[derive(Debug, Clone)]
pub struct SharedObCache {
    /// 오더북 데이터 캐시 (RwLock 보호).
    pub data: Arc<tokio::sync::RwLock<ObCacheData>>,
    /// Computing flag (Mutex 보호, atomic CAS).
    pub computing: Arc<ComputingFlags>,
}

impl SharedObCache {
    /// 새 SharedObCache를 생성합니다.
    pub fn new() -> Self {
        Self {
            data: Arc::new(tokio::sync::RwLock::new(ObCacheData::new())),
            computing: Arc::new(ComputingFlags::new()),
        }
    }
}

impl Default for SharedObCache {
    fn default() -> Self {
        Self::new()
    }
}

/// 안전 볼륨 계산 결과.
#[derive(Debug, Clone)]
pub struct SafeVolumeResult {
    /// 안전하게 진입/청산 가능한 코인 수량.
    pub safe_volume_coins: f64,
    /// USDT 환산.
    pub safe_volume_usdt: f64,
    /// Upbit VWAP (해당 통화 단위).
    pub upbit_vwap: f64,
    /// Bybit VWAP (USDT).
    pub bybit_vwap: f64,
    /// 진입 슬리피지 (%).
    pub entry_slippage_pct: f64,
}

/// 진입 안전 볼륨 계산 단계의 수익성 진단값.
#[derive(Debug, Clone)]
pub struct EntryProfitBreakdown {
    /// 누적 소비 코인 수량.
    pub total_coins: f64,
    /// Upbit VWAP (USD 기준).
    pub upbit_vwap_usd: f64,
    /// Bybit VWAP (USDT).
    pub bybit_vwap: f64,
    /// 유효 스프레드(%): (Bybit - UpbitUSD) / UpbitUSD.
    pub effective_spread_pct: f64,
    /// 롤링 평균 스프레드(%).
    pub mean_spread_pct: f64,
    /// 왕복 수수료(%).
    pub roundtrip_fee_pct: f64,
    /// 진입 슬리피지(%).
    pub entry_slippage_pct: f64,
    /// 추정 청산 슬리피지(%).
    pub estimated_exit_slippage_pct: f64,
    /// 최종 수익성 점수(%).
    pub profit_pct: f64,
}

/// 진입 안전 볼륨 평가 결과.
#[derive(Debug, Clone)]
pub struct EntrySafeVolumeEvaluation {
    /// 안전 볼륨 계산 결과.
    pub safe_volume: Option<SafeVolumeResult>,
    /// 마지막 평가 단계의 수익성 진단값.
    pub last_step: Option<EntryProfitBreakdown>,
}

/// 진입 안전 볼륨을 평가하고 단계별 수익성 진단값을 반환합니다.
#[allow(clippy::too_many_arguments)]
pub fn evaluate_entry_safe_volume(
    upbit_asks: &[(f64, f64)],
    bybit_bids: &[(f64, f64)],
    mean_spread_pct: f64,
    upbit_fee: f64,
    bybit_fee: f64,
    usd_krw: f64,
) -> EntrySafeVolumeEvaluation {
    if upbit_asks.is_empty() || bybit_bids.is_empty() || usd_krw <= 0.0 {
        return EntrySafeVolumeEvaluation {
            safe_volume: None,
            last_step: None,
        };
    }

    let best_ask_usd = upbit_asks[0].0 / usd_krw;
    let best_bid = bybit_bids[0].0;

    let mut upbit_ptr: usize = 0;
    let mut bybit_ptr: usize = 0;
    let mut upbit_remaining = upbit_asks[0].1;
    let mut bybit_remaining = bybit_bids[0].1;

    let mut total_coins: f64 = 0.0;
    let mut upbit_cost_krw: f64 = 0.0;
    let mut bybit_revenue_usdt: f64 = 0.0;

    // 직전 단계의 유효한 결과를 저장
    let mut last_valid: Option<SafeVolumeResult> = None;
    let mut last_step: Option<EntryProfitBreakdown> = None;

    loop {
        // 이번 단계 소비량
        let consume = upbit_remaining.min(bybit_remaining);
        if consume <= 0.0 {
            break;
        }

        upbit_cost_krw += consume * upbit_asks[upbit_ptr].0;
        bybit_revenue_usdt += consume * bybit_bids[bybit_ptr].0;
        total_coins += consume;

        upbit_remaining -= consume;
        bybit_remaining -= consume;

        // 수익성 검증
        let upbit_vwap_usd = (upbit_cost_krw / total_coins) / usd_krw;
        let bybit_vwap = bybit_revenue_usdt / total_coins;
        let effective_spread = (bybit_vwap - upbit_vwap_usd) / upbit_vwap_usd * 100.0;
        let roundtrip_fee = (upbit_fee + bybit_fee) * 2.0 * 100.0;
        let entry_slippage_pct = (upbit_vwap_usd - best_ask_usd) / best_ask_usd * 100.0
            + (best_bid - bybit_vwap) / best_bid * 100.0;
        // estimated_exit_slippage는 진단 전용. effective_spread가 이미 VWAP 기반이므로
        // 진입 슬리피지를 내포하여 profit에서 이중 차감하지 않는다.
        let estimated_exit_slippage = entry_slippage_pct;
        let profit = (effective_spread - mean_spread_pct) - roundtrip_fee;

        let step = EntryProfitBreakdown {
            total_coins,
            upbit_vwap_usd,
            bybit_vwap,
            effective_spread_pct: effective_spread,
            mean_spread_pct,
            roundtrip_fee_pct: roundtrip_fee,
            entry_slippage_pct,
            estimated_exit_slippage_pct: estimated_exit_slippage,
            profit_pct: profit,
        };
        last_step = Some(step);

        trace!(
            total_coins = total_coins,
            effective_spread = effective_spread,
            profit = profit,
            "two-pointer 진입 단계"
        );

        if profit > 0.0 {
            last_valid = Some(SafeVolumeResult {
                safe_volume_coins: total_coins,
                safe_volume_usdt: total_coins * bybit_vwap,
                upbit_vwap: upbit_cost_krw / total_coins,
                bybit_vwap,
                entry_slippage_pct,
            });
        } else {
            // profit <= 0 → 직전 결과 반환
            return EntrySafeVolumeEvaluation {
                safe_volume: last_valid,
                last_step,
            };
        }

        // 잔여 처리: 0인 쪽 다음 호가로 이동
        if upbit_remaining <= 0.0 {
            upbit_ptr += 1;
            if upbit_ptr >= upbit_asks.len() {
                break;
            }
            upbit_remaining = upbit_asks[upbit_ptr].1;
        }
        if bybit_remaining <= 0.0 {
            bybit_ptr += 1;
            if bybit_ptr >= bybit_bids.len() {
                break;
            }
            bybit_remaining = bybit_bids[bybit_ptr].1;
        }
    }

    EntrySafeVolumeEvaluation {
        safe_volume: last_valid,
        last_step,
    }
}

/// 진입 시 안전 볼륨 계산 (Upbit 매수 + Bybit 숏).
///
/// two-pointer로 Upbit asks + Bybit bids를 동시 소비하며
/// VWAP 기반 수익성 > 0인 최대 볼륨을 계산합니다.
///
/// # 인자
///
/// * `upbit_asks` - Upbit 매도호가 (price_krw, size_coins), 가격 오름차순
/// * `bybit_bids` - Bybit 매수호가 (price_usdt, size_coins), 가격 내림차순
/// * `mean_spread_pct` - 현재 rolling mean spread (%)
/// * `upbit_fee` - Upbit taker 수수료율 (예: 0.0005)
/// * `bybit_fee` - Bybit taker 수수료율 (예: 0.00055)
/// * `usd_krw` - 현재 USD/KRW 환율
///
/// # 반환값
///
/// 수익성이 양수인 최대 안전 볼륨. 오더북이 비어있거나 첫 단계부터 수익성이 없으면 `None`.
#[allow(clippy::too_many_arguments)]
pub fn calculate_entry_safe_volume(
    upbit_asks: &[(f64, f64)],
    bybit_bids: &[(f64, f64)],
    mean_spread_pct: f64,
    upbit_fee: f64,
    bybit_fee: f64,
    usd_krw: f64,
) -> Option<SafeVolumeResult> {
    evaluate_entry_safe_volume(
        upbit_asks,
        bybit_bids,
        mean_spread_pct,
        upbit_fee,
        bybit_fee,
        usd_krw,
    )
    .safe_volume
}

/// 청산 시 안전 볼륨 계산 (Upbit 매도 + Bybit 롱 커버).
///
/// 진입의 역방향: Upbit bids + Bybit asks를 소비합니다.
///
/// # 인자
///
/// * `upbit_bids` - Upbit 매수호가 (price_krw, size_coins), 가격 내림차순
/// * `bybit_asks` - Bybit 매도호가 (price_usdt, size_coins), 가격 오름차순
/// * `mean_spread_pct` - 현재 rolling mean spread (%)
/// * `upbit_fee` - Upbit taker 수수료율
/// * `bybit_fee` - Bybit taker 수수료율
/// * `usd_krw` - 현재 USD/KRW 환율
///
/// # 반환값
///
/// 수익성이 양수인 최대 안전 볼륨. 오더북이 비어있거나 수익성이 없으면 `None`.
#[allow(clippy::too_many_arguments)]
pub fn calculate_exit_safe_volume(
    upbit_bids: &[(f64, f64)],
    bybit_asks: &[(f64, f64)],
    mean_spread_pct: f64,
    upbit_fee: f64,
    bybit_fee: f64,
    usd_krw: f64,
) -> Option<SafeVolumeResult> {
    if upbit_bids.is_empty() || bybit_asks.is_empty() || usd_krw <= 0.0 {
        return None;
    }

    let best_bid_usd = upbit_bids[0].0 / usd_krw;
    let best_ask = bybit_asks[0].0;

    let mut upbit_ptr: usize = 0;
    let mut bybit_ptr: usize = 0;
    let mut upbit_remaining = upbit_bids[0].1;
    let mut bybit_remaining = bybit_asks[0].1;

    let mut total_coins: f64 = 0.0;
    let mut upbit_revenue_krw: f64 = 0.0;
    let mut bybit_cost_usdt: f64 = 0.0;

    let mut last_valid: Option<SafeVolumeResult> = None;

    loop {
        let consume = upbit_remaining.min(bybit_remaining);
        if consume <= 0.0 {
            break;
        }

        upbit_revenue_krw += consume * upbit_bids[upbit_ptr].0;
        bybit_cost_usdt += consume * bybit_asks[bybit_ptr].0;
        total_coins += consume;

        upbit_remaining -= consume;
        bybit_remaining -= consume;

        // 청산 수익성 검증
        // Upbit 매도: revenue, Bybit 매수(숏 커버): cost
        let upbit_vwap_usd = (upbit_revenue_krw / total_coins) / usd_krw;
        let bybit_vwap = bybit_cost_usdt / total_coins;
        // 청산 시 스프레드: Upbit 매도 수익 - Bybit 매수 비용
        let effective_spread = (upbit_vwap_usd - bybit_vwap) / bybit_vwap * 100.0;
        let roundtrip_fee = (upbit_fee + bybit_fee) * 2.0 * 100.0;
        // exit_slippage_pct는 진단 전용. effective_spread가 VWAP 기반이므로
        // 슬리피지를 이미 내포하여 profit에서 이중 차감하지 않는다.
        let exit_slippage_pct = (best_bid_usd - upbit_vwap_usd) / best_bid_usd * 100.0
            + (bybit_vwap - best_ask) / best_ask * 100.0;
        let profit = (effective_spread - mean_spread_pct) - roundtrip_fee;

        trace!(
            total_coins = total_coins,
            effective_spread = effective_spread,
            profit = profit,
            "two-pointer 청산 단계"
        );

        if profit > 0.0 {
            last_valid = Some(SafeVolumeResult {
                safe_volume_coins: total_coins,
                safe_volume_usdt: total_coins * bybit_vwap,
                upbit_vwap: upbit_revenue_krw / total_coins,
                bybit_vwap,
                entry_slippage_pct: exit_slippage_pct,
            });
        } else {
            return last_valid;
        }

        if upbit_remaining <= 0.0 {
            upbit_ptr += 1;
            if upbit_ptr >= upbit_bids.len() {
                break;
            }
            upbit_remaining = upbit_bids[upbit_ptr].1;
        }
        if bybit_remaining <= 0.0 {
            bybit_ptr += 1;
            if bybit_ptr >= bybit_asks.len() {
                break;
            }
            bybit_remaining = bybit_asks[bybit_ptr].1;
        }
    }

    last_valid
}

/// OrderBook의 호가를 `(f64, f64)` 튜플 슬라이스로 변환합니다.
///
/// `OrderBookLevel`의 `Decimal` 필드를 `f64`로 변환하여
/// 슬리피지 계산 함수에서 사용할 수 있는 형태로 만듭니다.
pub fn levels_to_f64(ob: &OrderBook, is_asks: bool) -> Vec<(f64, f64)> {
    let levels = if is_asks { &ob.asks } else { &ob.bids };
    levels
        .iter()
        .filter_map(|level| {
            let price = level.price.to_f64()?;
            let size = level.size.to_f64()?;
            if price > 0.0 && size > 0.0 {
                Some((price, size))
            } else {
                warn!(price = %level.price, size = %level.size, "유효하지 않은 호가 레벨 무시");
                None
            }
        })
        .collect()
}

/// 1시간 거래대금 기반 동적 safe_volume_ratio.
///
/// 유동성이 낮을수록 오더북 변동이 크므로 더 보수적으로 진입합니다.
///
/// | 1시간 거래대금 (USDT) | safe_volume_ratio |
/// |----------------------|-------------------|
/// | < 100,000            | 0.5               |
/// | 100,000 ~ 300,000    | 0.6               |
/// | 300,000 ~ 500,000    | 0.7               |
/// | >= 500,000           | 0.8               |
pub fn safe_volume_ratio_from_volume(volume_1h_usdt: f64) -> f64 {
    if volume_1h_usdt < 100_000.0 {
        0.5
    } else if volume_1h_usdt < 300_000.0 {
        0.6
    } else if volume_1h_usdt < 500_000.0 {
        0.7
    } else {
        0.8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;

    use arb_exchange::OrderBookLevel;

    /// 테스트용 OrderBook을 생성합니다.
    fn make_orderbook(asks: Vec<(i64, i64)>, bids: Vec<(i64, i64)>) -> OrderBook {
        OrderBook {
            market: "TEST".to_string(),
            asks: asks
                .into_iter()
                .map(|(p, s)| OrderBookLevel {
                    price: Decimal::new(p, 0),
                    size: Decimal::new(s, 0),
                })
                .collect(),
            bids: bids
                .into_iter()
                .map(|(p, s)| OrderBookLevel {
                    price: Decimal::new(p, 0),
                    size: Decimal::new(s, 0),
                })
                .collect(),
            total_ask_size: Decimal::ZERO,
            total_bid_size: Decimal::ZERO,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_cache_basic_operations() {
        let mut cache = OrderBookCache::new();
        let ob = make_orderbook(vec![(100, 10)], vec![(99, 10)]);

        // 초기에는 비어있음
        assert!(cache.get(Exchange::Upbit, "BTC").is_none());
        assert!(!cache.is_fresh(Exchange::Upbit, "BTC", 5));

        // 캐시 갱신
        cache.update(Exchange::Upbit, "BTC", ob);
        assert!(cache.get(Exchange::Upbit, "BTC").is_some());
        assert!(cache.is_fresh(Exchange::Upbit, "BTC", 5));

        // 다른 거래소는 비어있음
        assert!(cache.get(Exchange::Bybit, "BTC").is_none());
    }

    #[test]
    fn test_computing_flag() {
        let mut cache = OrderBookCache::new();

        // 초기에는 false
        assert!(!cache.is_computing(Exchange::Upbit, "BTC"));

        // 설정 후 true
        cache.set_computing(Exchange::Upbit, "BTC", true);
        assert!(cache.is_computing(Exchange::Upbit, "BTC"));

        // 다른 거래소/코인은 독립적
        assert!(!cache.is_computing(Exchange::Bybit, "BTC"));
        assert!(!cache.is_computing(Exchange::Upbit, "ETH"));

        // 해제
        cache.set_computing(Exchange::Upbit, "BTC", false);
        assert!(!cache.is_computing(Exchange::Upbit, "BTC"));
    }

    #[test]
    fn test_entry_safe_volume_basic() {
        // Upbit asks (KRW): 가격 오름차순, 2단계
        // 1,400,000 KRW / 1,400 usd_krw = 1000 USD
        let upbit_asks = vec![
            (1_400_000.0, 1.0), // 140만원에 1개
            (1_401_000.0, 1.0), // 140.1만원에 1개
        ];
        // Bybit bids (USDT): 가격 내림차순, 2단계
        // 1050 USDT → 스프레드 약 5%
        let bybit_bids = vec![
            (1050.0, 1.0), // 1050 USDT에 1개
            (1049.0, 1.0), // 1049 USDT에 1개
        ];

        let usd_krw = 1400.0;
        let mean_spread_pct = 0.0;
        let upbit_fee = 0.0005;
        let bybit_fee = 0.00055;

        let result = calculate_entry_safe_volume(
            &upbit_asks,
            &bybit_bids,
            mean_spread_pct,
            upbit_fee,
            bybit_fee,
            usd_krw,
        );

        assert!(result.is_some());
        let sv = result.unwrap();
        assert!(sv.safe_volume_coins > 0.0);
        assert!(sv.safe_volume_usdt > 0.0);
        assert!(sv.upbit_vwap > 0.0);
        assert!(sv.bybit_vwap > 0.0);
    }

    #[test]
    fn test_entry_safe_volume_profit_boundary() {
        // 매우 좁은 스프레드 → 수수료가 이기므로 safe volume = None
        let upbit_asks = vec![(1_400_000.0, 1.0)];
        let bybit_bids = vec![(1000.0, 1.0)];

        let usd_krw = 1400.0;
        // 스프레드가 거의 0이고 mean도 0이면 수수료가 이김
        let mean_spread_pct = 0.0;
        let upbit_fee = 0.0005;
        let bybit_fee = 0.00055;

        let result = calculate_entry_safe_volume(
            &upbit_asks,
            &bybit_bids,
            mean_spread_pct,
            upbit_fee,
            bybit_fee,
            usd_krw,
        );

        // 수익성이 없으므로 None이어야 함
        assert!(result.is_none());
    }

    #[test]
    fn test_entry_safe_volume_empty_orderbook() {
        // 빈 오더북 → None
        assert!(
            calculate_entry_safe_volume(&[], &[(1000.0, 1.0)], 0.0, 0.0005, 0.00055, 1400.0)
                .is_none()
        );
        assert!(
            calculate_entry_safe_volume(&[(1_400_000.0, 1.0)], &[], 0.0, 0.0005, 0.00055, 1400.0)
                .is_none()
        );
        assert!(calculate_entry_safe_volume(&[], &[], 0.0, 0.0005, 0.00055, 1400.0).is_none());
        // usd_krw = 0 → None
        assert!(
            calculate_entry_safe_volume(
                &[(1_400_000.0, 1.0)],
                &[(1000.0, 1.0)],
                0.0,
                0.0005,
                0.00055,
                0.0
            )
            .is_none()
        );
    }

    #[test]
    fn test_exit_safe_volume_basic() {
        // 청산: Upbit bids (KRW 내림차순) + Bybit asks (USDT 오름차순)
        let upbit_bids = vec![(1_460_000.0, 1.0), (1_459_000.0, 1.0)];
        let bybit_asks = vec![(1000.0, 1.0), (1001.0, 1.0)];

        let usd_krw = 1400.0;
        let mean_spread_pct = -4.0; // 큰 음의 mean → 청산 수익성 확보
        let upbit_fee = 0.0005;
        let bybit_fee = 0.00055;

        let result = calculate_exit_safe_volume(
            &upbit_bids,
            &bybit_asks,
            mean_spread_pct,
            upbit_fee,
            bybit_fee,
            usd_krw,
        );

        assert!(result.is_some());
        let sv = result.unwrap();
        assert!(sv.safe_volume_coins > 0.0);
        assert!(sv.safe_volume_usdt > 0.0);
    }

    #[test]
    fn test_safe_volume_ratio() {
        // < 100,000 → 0.5
        assert_eq!(safe_volume_ratio_from_volume(50_000.0), 0.5);
        assert_eq!(safe_volume_ratio_from_volume(99_999.0), 0.5);
        // 100,000 ~ 300,000 → 0.6
        assert_eq!(safe_volume_ratio_from_volume(100_000.0), 0.6);
        assert_eq!(safe_volume_ratio_from_volume(200_000.0), 0.6);
        assert_eq!(safe_volume_ratio_from_volume(299_999.0), 0.6);
        // 300,000 ~ 500,000 → 0.7
        assert_eq!(safe_volume_ratio_from_volume(300_000.0), 0.7);
        assert_eq!(safe_volume_ratio_from_volume(400_000.0), 0.7);
        assert_eq!(safe_volume_ratio_from_volume(499_999.0), 0.7);
        // >= 500,000 → 0.8
        assert_eq!(safe_volume_ratio_from_volume(500_000.0), 0.8);
        assert_eq!(safe_volume_ratio_from_volume(1_000_000.0), 0.8);
    }

    #[test]
    fn test_levels_to_f64() {
        let ob = make_orderbook(vec![(100, 10), (101, 20)], vec![(99, 15), (98, 25)]);

        let asks = levels_to_f64(&ob, true);
        assert_eq!(asks.len(), 2);
        assert_eq!(asks[0], (100.0, 10.0));
        assert_eq!(asks[1], (101.0, 20.0));

        let bids = levels_to_f64(&ob, false);
        assert_eq!(bids.len(), 2);
        assert_eq!(bids[0], (99.0, 15.0));
        assert_eq!(bids[1], (98.0, 25.0));
    }

    #[test]
    fn test_entry_safe_volume_multi_level() {
        // 큰 스프레드를 가진 다단계 오더북
        // Upbit: 1,400,000 KRW / 1,400 usd_krw = 1000 USD
        // Bybit: 1050 USDT → 스프레드 약 5%
        let upbit_asks = vec![(1_400_000.0, 0.5), (1_401_000.0, 0.5), (1_402_000.0, 0.5)];
        let bybit_bids = vec![(1050.0, 0.5), (1049.0, 0.5), (1048.0, 0.5)];

        let usd_krw = 1400.0;
        let mean_spread_pct = 0.0;
        let upbit_fee = 0.0005;
        let bybit_fee = 0.00055;

        let result = calculate_entry_safe_volume(
            &upbit_asks,
            &bybit_bids,
            mean_spread_pct,
            upbit_fee,
            bybit_fee,
            usd_krw,
        );

        assert!(result.is_some());
        let sv = result.unwrap();
        // 스프레드가 약 5%이므로 충분히 수익성 있음
        assert!(sv.safe_volume_coins > 0.0);
    }

    #[test]
    fn test_cache_default() {
        let cache = OrderBookCache::default();
        assert!(cache.get(Exchange::Upbit, "BTC").is_none());
        assert!(!cache.is_computing(Exchange::Upbit, "BTC"));
    }

    // --- ObCacheData 테스트 ---

    #[test]
    fn test_ob_cache_data_basic_operations() {
        let mut data = ObCacheData::new();
        let ob = make_orderbook(vec![(100, 10)], vec![(99, 10)]);

        // 초기에는 비어있음
        assert!(data.get(Exchange::Upbit, "BTC").is_none());
        assert!(!data.is_fresh(Exchange::Upbit, "BTC", 5));

        // 갱신
        data.update(Exchange::Upbit, "BTC", ob);
        assert!(data.get(Exchange::Upbit, "BTC").is_some());
        assert!(data.is_fresh(Exchange::Upbit, "BTC", 5));

        // 다른 거래소는 비어있음
        assert!(data.get(Exchange::Bybit, "BTC").is_none());
    }

    #[test]
    fn test_ob_cache_data_default() {
        let data = ObCacheData::default();
        assert!(data.get(Exchange::Upbit, "BTC").is_none());
        assert!(data.get(Exchange::Bybit, "ETH").is_none());
    }

    #[test]
    fn test_ob_cache_data_multiple_coins() {
        let mut data = ObCacheData::new();
        let ob1 = make_orderbook(vec![(100, 10)], vec![(99, 10)]);
        let ob2 = make_orderbook(vec![(200, 20)], vec![(199, 20)]);

        data.update(Exchange::Upbit, "BTC", ob1);
        data.update(Exchange::Bybit, "ETH", ob2);

        assert!(data.get(Exchange::Upbit, "BTC").is_some());
        assert!(data.get(Exchange::Bybit, "ETH").is_some());
        assert!(data.get(Exchange::Upbit, "ETH").is_none());
        assert!(data.get(Exchange::Bybit, "BTC").is_none());
    }

    // --- ComputingFlags 테스트 ---

    #[test]
    fn test_computing_flag_try_set_atomic() {
        let flags = ComputingFlags::new();

        // 초기에는 computing 아님
        assert!(!flags.is_computing(Exchange::Upbit, "BTC"));

        // 첫 번째 try_set: false 반환 (설정 성공)
        assert!(!flags.try_set_computing(Exchange::Upbit, "BTC"));
        assert!(flags.is_computing(Exchange::Upbit, "BTC"));

        // 두 번째 try_set: true 반환 (이미 computing 중 → 스킵)
        assert!(flags.try_set_computing(Exchange::Upbit, "BTC"));

        // 다른 거래소/코인은 독립적
        assert!(!flags.try_set_computing(Exchange::Bybit, "BTC"));
        assert!(!flags.try_set_computing(Exchange::Upbit, "ETH"));
    }

    #[test]
    fn test_computing_flag_clear() {
        let flags = ComputingFlags::new();

        // 설정
        assert!(!flags.try_set_computing(Exchange::Upbit, "BTC"));
        assert!(flags.is_computing(Exchange::Upbit, "BTC"));

        // 해제
        flags.clear_computing(Exchange::Upbit, "BTC");
        assert!(!flags.is_computing(Exchange::Upbit, "BTC"));

        // 해제 후 다시 설정 가능
        assert!(!flags.try_set_computing(Exchange::Upbit, "BTC"));
        assert!(flags.is_computing(Exchange::Upbit, "BTC"));
    }

    #[test]
    fn test_computing_flag_clear_without_set() {
        // 설정하지 않은 상태에서 해제해도 패닉 없음
        let flags = ComputingFlags::new();
        flags.clear_computing(Exchange::Upbit, "BTC");
        assert!(!flags.is_computing(Exchange::Upbit, "BTC"));
    }

    #[test]
    fn test_computing_flags_default() {
        let flags = ComputingFlags::default();
        assert!(!flags.is_computing(Exchange::Upbit, "BTC"));
    }

    // --- SharedObCache 테스트 ---

    #[tokio::test]
    async fn test_shared_ob_cache_data_update_and_get() {
        let cache = SharedObCache::new();
        let ob = make_orderbook(vec![(100, 10)], vec![(99, 10)]);

        // 초기에는 비어있음
        {
            let data = cache.data.read().await;
            assert!(data.get(Exchange::Upbit, "BTC").is_none());
        }

        // write lock으로 갱신
        {
            let mut data = cache.data.write().await;
            data.update(Exchange::Upbit, "BTC", ob);
        }

        // read lock으로 조회
        {
            let data = cache.data.read().await;
            assert!(data.get(Exchange::Upbit, "BTC").is_some());
            assert!(data.is_fresh(Exchange::Upbit, "BTC", 5));
        }
    }

    #[tokio::test]
    async fn test_shared_ob_cache_computing_and_data_independent() {
        let cache = SharedObCache::new();
        let ob = make_orderbook(vec![(100, 10)], vec![(99, 10)]);

        // computing flag 설정은 data lock 없이 가능
        assert!(!cache.computing.try_set_computing(Exchange::Upbit, "BTC"));

        // data lock 획득은 computing flag와 독립적
        {
            let mut data = cache.data.write().await;
            data.update(Exchange::Upbit, "BTC", ob);
        }

        // 양쪽 모두 독립적으로 동작
        assert!(cache.computing.is_computing(Exchange::Upbit, "BTC"));
        {
            let data = cache.data.read().await;
            assert!(data.get(Exchange::Upbit, "BTC").is_some());
        }

        cache.computing.clear_computing(Exchange::Upbit, "BTC");
        assert!(!cache.computing.is_computing(Exchange::Upbit, "BTC"));
    }

    #[test]
    fn test_shared_ob_cache_default() {
        let cache = SharedObCache::default();
        assert!(!cache.computing.is_computing(Exchange::Upbit, "BTC"));
    }

    #[test]
    fn test_shared_ob_cache_clone() {
        let cache = SharedObCache::new();

        // clone은 Arc clone이므로 같은 데이터를 공유
        let cache2 = cache.clone();

        // 한쪽에서 computing flag 설정
        assert!(!cache.computing.try_set_computing(Exchange::Upbit, "BTC"));

        // 다른 쪽에서도 반영됨
        assert!(cache2.computing.is_computing(Exchange::Upbit, "BTC"));
        assert!(cache2.computing.try_set_computing(Exchange::Upbit, "BTC"));
    }
}
