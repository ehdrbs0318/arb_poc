//! 시뮬레이션 실행 정책 (SimPolicy).
//!
//! `ExecutionPolicy` trait의 시뮬레이션 구현체입니다.
//! 가상 체결(VirtualPosition 즉시 생성/청산)을 수행합니다.
//! DB, BalanceTracker, RiskManager 등 라이브 인프라에 의존하지 않습니다.
//!
//! ## 라이프사이클
//!
//! 1. `SimPolicy::new()` — 빈 상태로 생성 (ZScoreMonitor::new()에 전달)
//! 2. `bind_shared_resources()` — run() 내부에서 공유 상태 바인딩
//! 3. `on_entry_signal()` / `on_exit_signal()` / `on_ttl_expiry()` — 체결 수행

use std::sync::{Arc, OnceLock};

use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive as _;
use tracing::{info, warn};

use crate::error::StrategyError;
use crate::output::summary::MonitoringCounters;
use crate::output::writer::SessionWriter;
use crate::zscore::config::ZScoreConfig;
use crate::zscore::execution_policy::{
    EntryContext, ExecutionPolicy, ExitContext, SharedResources, TtlExpiryContext,
};
use crate::zscore::pnl::ClosedPosition;
use crate::zscore::position::{self, PositionManager, VirtualPosition};

/// SimPolicy 내부 상태 (OnceLock으로 지연 초기화).
struct SimPolicyInner {
    config: Arc<ZScoreConfig>,
    position_mgr: Arc<tokio::sync::Mutex<PositionManager>>,
    trades: Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
    counters: Arc<parking_lot::Mutex<MonitoringCounters>>,
    session_writer: Arc<tokio::sync::Mutex<Option<SessionWriter>>>,
}

/// 시뮬레이션 실행 정책.
///
/// 가상 포지션을 즉시 생성/청산합니다.
/// 공유 상태는 `bind_shared_resources()`로 지연 주입됩니다.
/// `OnceLock`을 사용하여 한 번만 바인딩 가능합니다.
pub struct SimPolicy {
    inner: OnceLock<SimPolicyInner>,
}

impl SimPolicy {
    /// 빈 SimPolicy를 생성합니다.
    ///
    /// `ZScoreMonitor::new()`에 전달 후, `run()` 내부에서
    /// `bind_shared_resources()`로 공유 상태가 주입됩니다.
    pub fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    /// 테스트용: 공유 상태를 직접 전달하여 SimPolicy를 생성합니다.
    #[cfg(test)]
    pub fn with_resources(
        config: Arc<ZScoreConfig>,
        position_mgr: Arc<tokio::sync::Mutex<PositionManager>>,
        trades: Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
        counters: Arc<parking_lot::Mutex<MonitoringCounters>>,
        session_writer: Arc<tokio::sync::Mutex<Option<SessionWriter>>>,
    ) -> Self {
        let policy = Self::new();
        let _ = policy.inner.set(SimPolicyInner {
            config,
            position_mgr,
            trades,
            counters,
            session_writer,
        });
        policy
    }

    /// 내부 상태에 접근합니다.
    ///
    /// `bind_shared_resources()` 호출 전에 접근하면 패닉합니다.
    fn inner(&self) -> &SimPolicyInner {
        self.inner
            .get()
            .expect("SimPolicy::bind_shared_resources() must be called before use")
    }
}

impl Default for SimPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionPolicy for SimPolicy {
    async fn on_entry_signal(&self, ctx: EntryContext) -> Result<(), StrategyError> {
        let inner = self.inner();

        // Liquidation price 계산
        let liq_price = position::calculate_liquidation_price(
            ctx.bybit_entry,
            inner.config.leverage,
            inner.config.bybit_mmr,
            inner.config.bybit_taker_fee,
        );

        let pos = VirtualPosition {
            id: 0, // PositionManager가 할당
            coin: ctx.coin.clone(),
            entry_time: Utc::now(),
            upbit_entry_price: ctx.upbit_entry_usd,
            bybit_entry_price: ctx.bybit_entry,
            bybit_liquidation_price: liq_price,
            entry_usd_krw: ctx.usd_krw,
            entry_spread_pct: ctx.spread_pct,
            entry_z_score: ctx.z_score,
            qty: ctx.qty,
            ..Default::default()
        };

        info!(
            coin = ctx.coin.as_str(),
            qty = %ctx.qty,
            upbit_entry_usd = %ctx.upbit_entry_usd,
            bybit_entry = %ctx.bybit_entry,
            z_score = ctx.z_score,
            "[SimPolicy] 가상 포지션 생성"
        );

        let mut pm = inner.position_mgr.lock().await;
        if let Err(e) = pm.open_position(pos) {
            warn!(coin = ctx.coin.as_str(), error = %e, "포지션 오픈 실패");
        }

        Ok(())
    }

    async fn on_exit_signal(&self, ctx: ExitContext) -> Result<(), StrategyError> {
        let inner = self.inner();
        let exit_safe_volume_usdt = ctx.exit_safe_volume_usdt.unwrap_or(0.0);

        // pm 락 내에서 청산 수행 → 결과만 수집 후 락 해제
        let (closed_positions, partial_count) = {
            let mut pm = inner.position_mgr.lock().await;
            let positions: Vec<(u64, Decimal, f64)> = pm
                .open_positions
                .get(ctx.coin.as_str())
                .map(|ps| {
                    ps.iter()
                        .map(|p| {
                            let profit_rate = (ctx.spread_pct - p.entry_spread_pct)
                                / p.size_usdt().to_f64().unwrap_or(1.0);
                            (p.id, p.size_usdt(), profit_rate)
                        })
                        .collect()
                })
                .unwrap_or_default();
            let mut sorted = positions;
            sorted.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

            let mut remaining_safe_usdt = exit_safe_volume_usdt;
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
                        &ctx.coin,
                        *pid,
                        Utc::now(),
                        ctx.exit_upbit_usd,
                        ctx.exit_bybit,
                        ctx.usd_krw,
                        ctx.spread_pct,
                        ctx.z_score,
                        inner.config.upbit_taker_fee,
                        inner.config.bybit_taker_fee,
                        false,
                    ) {
                        Ok(closed) => closed_results.push(closed),
                        Err(e) => warn!(error = %e, "청산 실패"),
                    }
                } else {
                    // 부분 청산: USDT → qty 변환
                    let partial_qty = Decimal::try_from(
                        remaining_safe_usdt / ctx.bybit_price.to_f64().unwrap_or(1.0),
                    )
                    .unwrap_or(Decimal::ZERO);
                    remaining_safe_usdt = 0.0;
                    match pm.close_partial(
                        &ctx.coin,
                        *pid,
                        partial_qty,
                        ctx.instrument_info.as_ref(),
                        ctx.exit_upbit_usd,
                        ctx.exit_bybit,
                        ctx.usd_krw,
                        ctx.spread_pct,
                        ctx.z_score,
                        inner.config.upbit_taker_fee,
                        inner.config.bybit_taker_fee,
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
            inner.trades.lock().await.push(closed.clone());
            let mut sw = inner.session_writer.lock().await;
            if let Some(ref mut w) = *sw
                && let Err(e) = w.append_trade(closed)
            {
                warn!(error = %e, "CSV 기록 실패");
            }
        }
        if partial_count > 0 {
            inner.counters.lock().partial_close_count += partial_count;
        }

        Ok(())
    }

    async fn on_ttl_expiry(&self, ctx: TtlExpiryContext) -> Result<(), StrategyError> {
        let inner = self.inner();
        let now = Utc::now();
        let is_liquidated = ctx.force_close;

        let closed_positions: Vec<ClosedPosition> = {
            let mut pm = inner.position_mgr.lock().await;
            let mut result = Vec::new();
            for ttl_pos in &ctx.positions {
                match pm.close_position(
                    &ctx.coin,
                    ttl_pos.id,
                    now,
                    ctx.exit_upbit_usd,
                    ctx.exit_bybit,
                    ctx.usd_krw,
                    ctx.current_spread_pct,
                    f64::NAN,
                    inner.config.upbit_taker_fee,
                    inner.config.bybit_taker_fee,
                    is_liquidated,
                ) {
                    Ok(closed) => result.push(closed),
                    Err(e) => {
                        let stage = if ctx.force_close {
                            "2단계 강제"
                        } else {
                            "1단계 TTL"
                        };
                        warn!(error = %e, "{} 청산 실패", stage);
                    }
                }
            }
            result
        };

        // pm 락 해제 후 trades → session_writer → counters 순서로 기록
        for closed in &closed_positions {
            inner.trades.lock().await.push(closed.clone());
            let mut sw = inner.session_writer.lock().await;
            if let Some(ref mut w) = *sw
                && let Err(e) = w.append_trade(closed)
            {
                let stage = if ctx.force_close { "강제" } else { "TTL" };
                warn!(error = %e, "{} 청산 CSV 기록 실패", stage);
            }
        }

        if ctx.force_close {
            inner.counters.lock().forced_liquidation_count += closed_positions.len() as u64;
        }

        Ok(())
    }

    fn is_entry_allowed(&self) -> bool {
        // 시뮬레이션에서는 항상 진입 허용
        true
    }

    fn bind_shared_resources(&self, resources: SharedResources) {
        let result = self.inner.set(SimPolicyInner {
            config: resources.config,
            position_mgr: resources.position_mgr,
            trades: resources.trades,
            counters: resources.counters,
            session_writer: resources.session_writer,
        });
        if result.is_err() {
            warn!("SimPolicy::bind_shared_resources() 중복 호출 무시");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zscore::execution_policy::TtlPosition;
    use crate::zscore::instrument::InstrumentInfo;

    fn make_config() -> Arc<ZScoreConfig> {
        Arc::new(ZScoreConfig::default())
    }

    #[allow(clippy::type_complexity)]
    fn make_sim_policy() -> (
        SimPolicy,
        Arc<tokio::sync::Mutex<PositionManager>>,
        Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
        Arc<parking_lot::Mutex<MonitoringCounters>>,
    ) {
        let config = make_config();
        let pm = Arc::new(tokio::sync::Mutex::new(PositionManager::new()));
        let trades = Arc::new(tokio::sync::Mutex::new(Vec::<ClosedPosition>::new()));
        let counters = Arc::new(parking_lot::Mutex::new(MonitoringCounters::default()));
        let sw = Arc::new(tokio::sync::Mutex::new(None::<SessionWriter>));

        let policy = SimPolicy::with_resources(
            config,
            Arc::clone(&pm),
            Arc::clone(&trades),
            Arc::clone(&counters),
            sw,
        );

        (policy, pm, trades, counters)
    }

    fn make_entry_ctx() -> EntryContext {
        EntryContext {
            coin: "BTC".to_string(),
            z_score: 2.5,
            spread_pct: 0.3,
            expected_profit_pct: 0.09,
            adjusted_profit_pct: 0.08,
            upbit_price_krw: Decimal::new(138_000_000, 0),
            upbit_entry_usd: Decimal::new(100_000, 0),
            bybit_entry: Decimal::new(100_050, 0),
            qty: Decimal::new(10, 3),
            usd_krw: 1380.0,
            mean: 0.1,
            stddev: 0.05,
            instrument_info: InstrumentInfo {
                tick_size: Decimal::new(1, 1),
                qty_step: Decimal::new(1, 3),
                min_order_qty: Decimal::new(1, 3),
                min_notional: Decimal::new(5, 0),
                max_order_qty: Decimal::new(100, 0),
            },
            safe_volume_usdt: 1000.0,
            volume_ratio: 0.7,
        }
    }

    #[test]
    fn test_sim_policy_default() {
        let _policy = SimPolicy::default();
        // 빈 상태로 생성됨 (bind 전)
    }

    #[test]
    fn test_sim_policy_bind_shared_resources() {
        let policy = SimPolicy::new();
        let config = make_config();
        let pm = Arc::new(tokio::sync::Mutex::new(PositionManager::new()));
        let trades = Arc::new(tokio::sync::Mutex::new(Vec::<ClosedPosition>::new()));
        let counters = Arc::new(parking_lot::Mutex::new(MonitoringCounters::default()));
        let sw = Arc::new(tokio::sync::Mutex::new(None::<SessionWriter>));

        policy.bind_shared_resources(SharedResources {
            config,
            position_mgr: pm,
            trades,
            counters,
            session_writer: sw,
        });

        assert!(policy.is_entry_allowed());
    }

    #[tokio::test]
    async fn test_sim_policy_entry_creates_position() {
        let (policy, pm, _trades, _counters) = make_sim_policy();
        let ctx = make_entry_ctx();

        policy.on_entry_signal(ctx).await.unwrap();

        let pm = pm.lock().await;
        assert_eq!(pm.open_count(), 1);
        assert!(pm.has_position("BTC"));
    }

    #[tokio::test]
    async fn test_sim_policy_entry_then_exit() {
        let (policy, pm, trades, _counters) = make_sim_policy();

        // 진입
        let entry_ctx = make_entry_ctx();
        policy.on_entry_signal(entry_ctx).await.unwrap();

        // 청산
        let exit_ctx = ExitContext {
            coin: "BTC".to_string(),
            z_score: 0.5,
            spread_pct: 0.1,
            exit_upbit_usd: Decimal::new(100_100, 0),
            exit_bybit: Decimal::new(100_000, 0),
            usd_krw: 1380.0,
            exit_safe_volume_usdt: Some(10000.0),
            mean: 0.1,
            instrument_info: None,
            bybit_price: Decimal::new(100_000, 0),
        };
        policy.on_exit_signal(exit_ctx).await.unwrap();

        let pm = pm.lock().await;
        assert_eq!(pm.open_count(), 0);

        let trades = trades.lock().await;
        assert_eq!(trades.len(), 1);
    }

    #[tokio::test]
    async fn test_sim_policy_is_entry_allowed() {
        let (policy, _pm, _trades, _counters) = make_sim_policy();
        assert!(policy.is_entry_allowed());
    }

    #[tokio::test]
    async fn test_sim_policy_ttl_expiry() {
        let (policy, pm, trades, counters) = make_sim_policy();

        // 먼저 포지션 생성
        let entry_ctx = make_entry_ctx();
        policy.on_entry_signal(entry_ctx).await.unwrap();

        let position_id = {
            let pm = pm.lock().await;
            pm.open_positions.get("BTC").unwrap().first().unwrap().id
        };

        // TTL 만료 (force_close=false)
        let ttl_ctx = TtlExpiryContext {
            coin: "BTC".to_string(),
            positions: vec![TtlPosition {
                id: position_id,
                size_usdt: Decimal::new(1000, 0),
                qty: Decimal::new(10, 3),
            }],
            usd_krw: 1380.0,
            current_spread_pct: 0.15,
            z_score: 1.2,
            instrument_info: None,
            exit_upbit_usd: Decimal::new(100_100, 0),
            exit_bybit: Decimal::new(100_000, 0),
            force_close: false,
        };
        policy.on_ttl_expiry(ttl_ctx).await.unwrap();

        let pm = pm.lock().await;
        assert_eq!(pm.open_count(), 0);

        let trades = trades.lock().await;
        assert_eq!(trades.len(), 1);

        // force_close=false이면 forced_liquidation_count 증가 안 함
        let c = counters.lock();
        assert_eq!(c.forced_liquidation_count, 0);
    }

    #[tokio::test]
    async fn test_sim_policy_ttl_force_close() {
        let (policy, pm, _trades, counters) = make_sim_policy();

        // 포지션 생성
        let entry_ctx = make_entry_ctx();
        policy.on_entry_signal(entry_ctx).await.unwrap();

        let position_id = {
            let pm = pm.lock().await;
            pm.open_positions.get("BTC").unwrap().first().unwrap().id
        };

        // 강제 청산
        let ttl_ctx = TtlExpiryContext {
            coin: "BTC".to_string(),
            positions: vec![TtlPosition {
                id: position_id,
                size_usdt: Decimal::new(1000, 0),
                qty: Decimal::new(10, 3),
            }],
            usd_krw: 1380.0,
            current_spread_pct: 0.15,
            z_score: 1.2,
            instrument_info: None,
            exit_upbit_usd: Decimal::new(100_100, 0),
            exit_bybit: Decimal::new(100_000, 0),
            force_close: true,
        };
        policy.on_ttl_expiry(ttl_ctx).await.unwrap();

        let pm = pm.lock().await;
        assert_eq!(pm.open_count(), 0);

        let c = counters.lock();
        assert_eq!(c.forced_liquidation_count, 1);
    }

    #[tokio::test]
    async fn test_sim_policy_exit_no_positions() {
        let (policy, _pm, trades, _counters) = make_sim_policy();

        // 포지션 없이 청산 시도 → 아무 일도 안 일어남
        let exit_ctx = ExitContext {
            coin: "BTC".to_string(),
            z_score: 0.5,
            spread_pct: 0.1,
            exit_upbit_usd: Decimal::new(100_100, 0),
            exit_bybit: Decimal::new(100_000, 0),
            usd_krw: 1380.0,
            exit_safe_volume_usdt: Some(10000.0),
            mean: 0.1,
            instrument_info: None,
            bybit_price: Decimal::new(100_000, 0),
        };
        policy.on_exit_signal(exit_ctx).await.unwrap();

        let trades = trades.lock().await;
        assert!(trades.is_empty());
    }

    #[tokio::test]
    async fn test_sim_policy_partial_close() {
        let (policy, pm, trades, counters) = make_sim_policy();

        // 큰 포지션 진입
        let mut entry_ctx = make_entry_ctx();
        entry_ctx.qty = Decimal::new(10, 0); // 10 BTC
        policy.on_entry_signal(entry_ctx).await.unwrap();

        // 안전 볼륨이 포지션 크기보다 작아 부분 청산 발생
        let exit_ctx = ExitContext {
            coin: "BTC".to_string(),
            z_score: 0.5,
            spread_pct: 0.1,
            exit_upbit_usd: Decimal::new(100_100, 0),
            exit_bybit: Decimal::new(100_000, 0),
            usd_krw: 1380.0,
            exit_safe_volume_usdt: Some(500.0), // 포지션보다 작음
            mean: 0.1,
            instrument_info: None,
            bybit_price: Decimal::new(100_050, 0),
        };
        policy.on_exit_signal(exit_ctx).await.unwrap();

        // 부분 청산이 발생했으므로 partial_close_count > 0
        {
            let c = counters.lock();
            assert!(c.partial_close_count > 0);
        }

        // 포지션이 아직 남아있음 (부분 청산)
        let pm = pm.lock().await;
        assert!(pm.has_position("BTC"));

        // 하나의 closed trade가 기록됨
        let trades = trades.lock().await;
        assert_eq!(trades.len(), 1);
    }
}
