//! 라이브 실행 정책 (LivePolicy).
//!
//! `ExecutionPolicy` trait의 라이브 구현체입니다.
//! 실주문(LiveExecutor를 통한 IOC 지정가)을 수행합니다.
//!
//! ## 아키텍처
//!
//! - **메모리 authoritative**: PositionManager(pm)가 실시간 의사결정 기준
//! - **DB shadow**: PositionStore로 비동기 영속화 (fire-and-forget)
//! - **Lock order**: balance_tracker → position_mgr → risk_manager.inner
//!
//! ## 진입 흐름 (on_entry_signal)
//!
//! 1. RiskManager.is_killed() 확인 (AtomicBool, lock 불필요)
//! 2. RiskManager.validate_order_size() — 단건 크기 상한
//! 3. BalanceTracker.reserve(upbit_krw, bybit_usdt) — 예약
//! 4. pm.lock() → open_count 확인, open_position (메모리) → DB INSERT
//! 5. pm.unlock()
//! 6. LiveExecutor.execute_entry() (REST, pm 락 밖에서 실행)
//! 7. pm.lock() → 체결 결과 반영 (state: Opening → Open)
//! 8. BalanceTracker.commit() / release()
//! 9. RiskManager.record_trade() (진입 시점에서는 PnL 0)
//!
//! ## 청산 흐름 (on_exit_signal)
//!
//! 1. pm.lock() → state 전이 (Open → Closing), DB UPDATE
//! 2. pm.unlock()
//! 3. LiveExecutor.execute_exit() (REST, pm 락 밖에서 실행)
//! 4. pm.lock() → 체결 결과 반영 (Closing → Closed), DB UPDATE
//! 5. BalanceTracker.on_exit() — 잔고 복원
//! 6. RiskManager.record_trade(pnl)

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, OnceLock};

use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive as _;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use arb_db::minutes::MinuteRecord as DbMinuteRecord;
use arb_db::trades::TradeRecord;
use arb_db::writer::{DbWriteRequest, DbWriter};
use arb_exchange::{InstrumentDataProvider, LinearOrderManagement, MarketData, OrderManagement};

use crate::error::StrategyError;
use crate::output::summary::MonitoringCounters;
use crate::output::writer::{MinuteRecord, SessionWriter};
use crate::zscore::alert::{AlertEvent, AlertService};
use crate::zscore::balance::BalanceTracker;
use crate::zscore::balance_recorder::BalanceSnapshotSender;
use crate::zscore::config::ZScoreConfig;
use crate::zscore::execution_policy::{
    EntryContext, ExecutionPolicy, ExitContext, SharedResources, TtlExpiryContext,
};
use crate::zscore::live_executor::{EntryRequest, ExitRequest, LiveExecutor, OrderExecutionError};
use crate::zscore::pnl::ClosedPosition;
use crate::zscore::position::{self, PositionManager, PositionState, VirtualPosition};
use crate::zscore::position_store::{PositionRecord, PositionStore, UpdateFields};
use crate::zscore::risk::RiskManager;

// ---------------------------------------------------------------------------
// SharedResources 지연 바인딩 (SimPolicy와 동일 패턴)
// ---------------------------------------------------------------------------

/// LivePolicy 내부 공유 상태 (OnceLock으로 지연 초기화).
struct LivePolicyShared {
    config: Arc<ZScoreConfig>,
    position_mgr: Arc<tokio::sync::Mutex<PositionManager>>,
    trades: Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
    counters: Arc<parking_lot::Mutex<MonitoringCounters>>,
    session_writer: Arc<tokio::sync::Mutex<Option<SessionWriter>>>,
}

/// 코인별 Upbit IOC 거부 누적 상태.
#[derive(Debug, Clone)]
struct UpbitIocRejectState {
    consecutive_rejects: u32,
    cooldown_until: Option<chrono::DateTime<Utc>>,
}

// ---------------------------------------------------------------------------
// LivePolicy
// ---------------------------------------------------------------------------

/// 라이브 실행 정책.
///
/// Upbit 현물 매수 + Bybit 선물 short 양 레그 실주문을 수행합니다.
/// `LiveExecutor`, `BalanceTracker`, `RiskManager`, `PositionStore`를
/// 생성자에서 주입받으며, `SharedResources`는 `bind_shared_resources()`로
/// 지연 바인딩됩니다.
pub struct LivePolicy<U, B, S>
where
    U: MarketData + OrderManagement + Send + Sync + 'static,
    B: MarketData
        + OrderManagement
        + LinearOrderManagement
        + InstrumentDataProvider
        + Send
        + Sync
        + 'static,
    S: PositionStore + 'static,
{
    /// 라이브 주문 실행 엔진.
    executor: Arc<LiveExecutor<U, B>>,
    /// 잔고 추적기.
    balance_tracker: Arc<BalanceTracker>,
    /// 리스크 관리자.
    risk_manager: Arc<RiskManager>,
    /// 포지션 영속화 (DB).
    position_store: Arc<S>,
    /// DB 세션 ID (crash recovery용).
    session_id: i64,
    /// 비동기 DB writer (None이면 DB producer 비활성화).
    db_writer: Option<DbWriter>,
    /// 운영 알림 서비스 (None이면 비활성화).
    alert_service: Option<AlertService>,
    /// SharedResources 지연 바인딩.
    shared: OnceLock<LivePolicyShared>,
    /// 잔고 스냅샷 전송 핸들 (None이면 비활성화).
    balance_sender: Option<BalanceSnapshotSender>,
    /// 코인별 Upbit IOC 거부 누적 상태.
    upbit_ioc_reject_states: parking_lot::Mutex<HashMap<String, UpbitIocRejectState>>,
    /// 제네릭 마커.
    _marker: PhantomData<(U, B)>,
}

impl<U, B, S> LivePolicy<U, B, S>
where
    U: MarketData + OrderManagement + Send + Sync + 'static,
    B: MarketData
        + OrderManagement
        + LinearOrderManagement
        + InstrumentDataProvider
        + Send
        + Sync
        + 'static,
    S: PositionStore + 'static,
{
    /// 새 LivePolicy를 생성합니다.
    ///
    /// `SharedResources`는 `bind_shared_resources()`로 나중에 바인딩됩니다.
    ///
    /// # 인자
    ///
    /// * `executor` - 라이브 주문 실행 엔진
    /// * `balance_tracker` - 잔고 추적기
    /// * `risk_manager` - 리스크 관리자
    /// * `position_store` - 포지션 영속화 (DB)
    /// * `session_id` - DB 세션 ID
    /// * `db_writer` - DB writer (minutes/trades producer)
    /// * `alert_service` - 운영 알림 서비스
    /// * `balance_sender` - 잔고 스냅샷 전송 핸들 (None이면 비활성화)
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        executor: Arc<LiveExecutor<U, B>>,
        balance_tracker: Arc<BalanceTracker>,
        risk_manager: Arc<RiskManager>,
        position_store: Arc<S>,
        session_id: i64,
        db_writer: Option<DbWriter>,
        alert_service: Option<AlertService>,
        balance_sender: Option<BalanceSnapshotSender>,
    ) -> Self {
        info!(session_id = session_id, "LivePolicy 초기화");
        Self {
            executor,
            balance_tracker,
            risk_manager,
            position_store,
            session_id,
            db_writer,
            alert_service,
            shared: OnceLock::new(),
            balance_sender,
            upbit_ioc_reject_states: parking_lot::Mutex::new(HashMap::new()),
            _marker: PhantomData,
        }
    }

    /// SharedResources에 접근합니다.
    fn shared(&self) -> &LivePolicyShared {
        self.shared
            .get()
            .expect("LivePolicy::bind_shared_resources() must be called before use")
    }

    /// UUID v7 형식의 client order ID를 생성합니다.
    fn new_client_order_id() -> String {
        Uuid::now_v7().to_string()
    }

    /// PositionRecord를 생성합니다 (DB INSERT용).
    fn make_position_record(&self, pos: &VirtualPosition) -> PositionRecord {
        PositionRecord {
            id: None,
            session_id: self.session_id,
            coin: pos.coin.clone(),
            state: pos.state.to_string(),
            upbit_qty: pos.qty,
            bybit_qty: pos.qty,
            upbit_entry_price: Some(pos.upbit_entry_price),
            bybit_entry_price: Some(pos.bybit_entry_price),
            upbit_order_id: pos.upbit_order_id.clone(),
            bybit_order_id: pos.bybit_order_id.clone(),
            entry_spread_pct: Some(pos.entry_spread_pct),
            entry_z_score: Some(pos.entry_z_score),
            entry_usd_krw: Some(pos.entry_usd_krw),
            opened_at: Some(pos.entry_time),
            closed_at: None,
            realized_pnl: None,
            exit_upbit_order_id: None,
            exit_bybit_order_id: None,
            client_order_id: pos.client_order_id.clone(),
            exit_client_order_id: None,
            in_flight: true,
            succeeded_leg: None,
            emergency_attempts: 0,
        }
    }

    /// DB 상태 전이를 비동기로 수행합니다 (fire-and-forget).
    ///
    /// 실패 시 warn 로그를 남기되, 메모리 상태에는 영향을 주지 않습니다.
    async fn db_update_state(&self, db_id: i64, from: &str, to: &str, fields: UpdateFields) {
        debug!(db_id = db_id, from = from, to = to, "DB 상태 전이 요청");

        match self
            .position_store
            .update_state(db_id, from, to, fields)
            .await
        {
            Ok(result) => {
                debug!(db_id = db_id, result = ?result, "DB 상태 전이 완료");
            }
            Err(e) => {
                error!(
                    db_id = db_id,
                    from = from,
                    to = to,
                    error = %e,
                    "DB 상태 전이 실패 (메모리 상태는 이미 전이됨)"
                );
            }
        }
    }

    /// ClosedPosition을 trades 목록 + CSV에 기록합니다.
    async fn record_trade(&self, closed: &ClosedPosition) {
        let shared = self.shared();

        shared.trades.lock().await.push(closed.clone());

        let mut sw = shared.session_writer.lock().await;
        if let Some(ref mut w) = *sw
            && let Err(e) = w.append_trade(closed)
        {
            warn!(error = %e, "CSV 기록 실패");
        }
    }

    /// 운영 알림 전송 (서비스 비활성화 시 no-op).
    fn emit_alert(&self, event: AlertEvent) {
        if let Some(service) = &self.alert_service {
            service.send(event);
        }
    }

    /// 치명적 운영 알림 전송 (채널 여유 대기).
    async fn emit_alert_critical(&self, event: AlertEvent) {
        if let Some(service) = &self.alert_service {
            service.send_critical(event).await;
        }
    }

    /// f64를 DB 저장용 Decimal로 변환합니다.
    fn decimal_from_f64(value: f64) -> Option<Decimal> {
        if !value.is_finite() {
            return None;
        }
        Decimal::try_from(value).ok()
    }

    /// Upbit IOC 주문 거부 메시지 여부를 판별합니다.
    fn is_upbit_ioc_rejection_message(message: &str) -> bool {
        let lower = message.to_ascii_lowercase();
        lower.contains("time_in_force")
            || lower.contains("ioc")
            || lower.contains("not supported")
            || lower.contains("validation")
            || lower.contains("ord_type")
    }

    /// 코인별 Upbit IOC 거부 상태를 성공 기준으로 초기화합니다.
    fn clear_upbit_ioc_reject_state(&self, coin: &str) {
        let mut states = self.upbit_ioc_reject_states.lock();
        states.remove(coin);
    }

    /// 코인이 IOC 거부 cooldown 상태인지 확인합니다.
    fn upbit_ioc_cooldown_until(&self, coin: &str) -> Option<chrono::DateTime<Utc>> {
        let now = Utc::now();
        let mut states = self.upbit_ioc_reject_states.lock();
        if let Some(state) = states.get_mut(coin)
            && let Some(until) = state.cooldown_until
        {
            if until > now {
                return Some(until);
            }
            // cooldown 만료 시 상태 초기화
            state.cooldown_until = None;
            state.consecutive_rejects = 0;
        }
        None
    }

    /// Upbit IOC 거부를 기록하고 필요 시 cooldown 차단을 설정합니다.
    ///
    /// # 반환값
    /// cooldown이 새로 설정되었으면 Some(until), 아니면 None
    fn record_upbit_ioc_reject(&self, coin: &str) -> Option<chrono::DateTime<Utc>> {
        let (block_count, cooldown_minutes) = if let Some(shared) = self.shared.get() {
            (
                shared.config.upbit_ioc_reject_block_count,
                shared.config.upbit_ioc_reject_cooldown_minutes,
            )
        } else {
            (3, 30)
        };

        let mut states = self.upbit_ioc_reject_states.lock();
        let state = states
            .entry(coin.to_string())
            .or_insert_with(|| UpbitIocRejectState {
                consecutive_rejects: 0,
                cooldown_until: None,
            });

        state.consecutive_rejects = state.consecutive_rejects.saturating_add(1);
        if state.consecutive_rejects >= block_count {
            let until = Utc::now() + chrono::Duration::minutes(cooldown_minutes as i64);
            state.cooldown_until = Some(until);
            return Some(until);
        }
        None
    }

    /// 분봉 1건을 DB writer로 전송합니다.
    async fn enqueue_minute_record(&self, record: &MinuteRecord) {
        let Some(db_writer) = &self.db_writer else {
            return;
        };

        let ts = match chrono::DateTime::parse_from_rfc3339(&record.timestamp) {
            Ok(ts) => ts.with_timezone(&Utc),
            Err(e) => {
                warn!(
                    timestamp = record.timestamp.as_str(),
                    error = %e,
                    "분봉 timestamp 파싱 실패, DB INSERT 스킵"
                );
                self.emit_alert(AlertEvent::Error {
                    message: format!("minute timestamp parse failed: {}", e),
                });
                return;
            }
        };

        let minute = DbMinuteRecord {
            id: None,
            session_id: self.session_id,
            coin: record.coin.clone(),
            ts,
            upbit_close: Self::decimal_from_f64(record.upbit_close),
            bybit_close: Self::decimal_from_f64(record.bybit_close),
            spread_pct: record.spread_pct.is_finite().then_some(record.spread_pct),
            z_score: record.z_score.is_finite().then_some(record.z_score),
            mean: record.mean.is_finite().then_some(record.mean),
            stddev: record.stddev.is_finite().then_some(record.stddev),
        };

        db_writer.send(DbWriteRequest::InsertMinute(minute));
    }

    /// 청산 거래 1건을 DB writer로 전송합니다.
    async fn enqueue_trade_record(&self, closed: &ClosedPosition, position_db_id: Option<i64>) {
        let Some(db_writer) = &self.db_writer else {
            return;
        };
        let Some(position_id) = position_db_id else {
            warn!(
                coin = closed.coin.as_str(),
                local_position_id = closed.id,
                "position db_id 없음, trades INSERT 스킵"
            );
            self.emit_alert(AlertEvent::Error {
                message: format!(
                    "trade insert skipped: coin={} local_pos_id={} has_no_db_id",
                    closed.coin, closed.id
                ),
            });
            return;
        };

        let upbit_price_krw =
            Self::decimal_from_f64(closed.exit_usd_krw).map(|rate| closed.upbit_exit_price * rate);

        let trade = TradeRecord {
            id: None,
            session_id: self.session_id,
            position_id,
            coin: closed.coin.clone(),
            side: "exit".to_string(),
            qty: closed.qty,
            upbit_price_krw,
            bybit_price_usdt: Some(closed.bybit_exit_price),
            upbit_fee: closed.actual_upbit_fee.or(Some(closed.upbit_fees)),
            bybit_fee: closed.actual_bybit_fee.or(Some(closed.bybit_fees)),
            spread_pct: closed
                .exit_spread_pct
                .is_finite()
                .then_some(closed.exit_spread_pct),
            z_score: closed
                .exit_z_score
                .is_finite()
                .then_some(closed.exit_z_score),
            realized_pnl: Some(closed.net_pnl),
            adjustment_cost: closed.adjustment_cost,
            exit_usd_krw: closed
                .exit_usd_krw
                .is_finite()
                .then_some(closed.exit_usd_krw),
            executed_at: closed.exit_time,
        };

        db_writer.send(DbWriteRequest::InsertTrade(trade));
    }
}

impl<U, B, S> ExecutionPolicy for LivePolicy<U, B, S>
where
    U: MarketData + OrderManagement + Send + Sync + 'static,
    B: MarketData
        + OrderManagement
        + LinearOrderManagement
        + InstrumentDataProvider
        + Send
        + Sync
        + 'static,
    S: PositionStore + 'static,
{
    async fn on_entry_signal(&self, ctx: EntryContext) -> Result<(), StrategyError> {
        let shared = self.shared();
        let coin = &ctx.coin;

        // ① Kill switch 체크 (AtomicBool, lock 불필요)
        if self.risk_manager.is_killed() {
            info!(
                coin = coin.as_str(),
                z_score = ctx.z_score,
                spread_pct = ctx.spread_pct,
                expected_profit = ctx.expected_profit_pct,
                filter = "kill_switch",
                "진입 거부: z-score 통과 후 kill switch 발동"
            );
            return Ok(());
        }

        if let Some(until) = self.upbit_ioc_cooldown_until(coin) {
            info!(
                coin = coin.as_str(),
                cooldown_until = %until,
                "진입 거부: Upbit IOC 거부 cooldown 적용 중"
            );
            shared.counters.lock().entry_rejected_order_constraint_count += 1;
            return Ok(());
        }

        // ② 주문 크기 상한 체크
        let order_size_usdt = ctx.qty * ctx.bybit_entry;
        if !self.risk_manager.validate_order_size(order_size_usdt) {
            info!(
                coin = coin.as_str(),
                z_score = ctx.z_score,
                spread_pct = ctx.spread_pct,
                expected_profit = ctx.expected_profit_pct,
                order_size_usdt = %order_size_usdt,
                filter = "max_order_size_usdt",
                "진입 거부: z-score 통과 후 주문 크기 상한 초과"
            );
            shared.counters.lock().entry_rejected_order_constraint_count += 1;
            return Ok(());
        }

        // ③ 잔고 예약 (Upbit KRW + Bybit USDT)
        let upbit_krw_needed = ctx.upbit_price_krw * ctx.qty;
        let bybit_usdt_needed = order_size_usdt;

        debug!(
            coin = coin.as_str(),
            upbit_krw = %upbit_krw_needed,
            bybit_usdt = %bybit_usdt_needed,
            "잔고 예약 시도"
        );

        let Some(mut reservation) = self
            .balance_tracker
            .reserve(upbit_krw_needed, bybit_usdt_needed)
        else {
            let (_upbit_available, bybit_available) = self.balance_tracker.available();
            info!(
                coin = coin.as_str(),
                upbit_krw = %upbit_krw_needed,
                bybit_usdt = %bybit_usdt_needed,
                "진입 거부: 잔고 부족"
            );
            self.emit_alert(AlertEvent::BalanceInsufficient {
                exchange: "bybit".to_string(),
                required: bybit_usdt_needed,
                available: bybit_available,
            });
            shared.counters.lock().entry_rejected_order_constraint_count += 1;
            return Ok(());
        };

        let client_order_id = Self::new_client_order_id();

        // ④ pm.lock() → max_concurrent 확인 + VirtualPosition 등록 (Opening)
        let (pos_id, db_id) = {
            let mut pm = shared.position_mgr.lock().await;

            // Kill switch 이중 체크 (TOCTOU 방지)
            if self.risk_manager.is_killed() {
                info!(
                    coin = coin.as_str(),
                    z_score = ctx.z_score,
                    spread_pct = ctx.spread_pct,
                    expected_profit = ctx.expected_profit_pct,
                    filter = "kill_switch_recheck",
                    "진입 거부: z-score 통과 후 kill switch 이중 체크"
                );
                drop(pm);
                self.balance_tracker.release(&mut reservation);
                return Ok(());
            }

            let max_pos = shared
                .config
                .max_concurrent_positions
                .unwrap_or(shared.config.coins.len());
            if pm.open_count() >= max_pos {
                info!(
                    coin = coin.as_str(),
                    z_score = ctx.z_score,
                    spread_pct = ctx.spread_pct,
                    expected_profit = ctx.expected_profit_pct,
                    open_count = pm.open_count(),
                    max_pos = max_pos,
                    filter = "max_concurrent_positions",
                    "진입 거부: z-score 통과 후 최대 동시 포지션 수 초과"
                );
                drop(pm);
                self.balance_tracker.release(&mut reservation);
                shared.counters.lock().entry_rejected_order_constraint_count += 1;
                return Ok(());
            }

            // Liquidation price 계산
            let liq_price = position::calculate_liquidation_price(
                ctx.bybit_entry,
                shared.config.leverage,
                shared.config.bybit_mmr,
                shared.config.bybit_taker_fee,
            );

            let pos = VirtualPosition {
                id: 0, // PositionManager가 할당
                coin: coin.clone(),
                entry_time: Utc::now(),
                upbit_entry_price: ctx.upbit_entry_usd,
                bybit_entry_price: ctx.bybit_entry,
                bybit_liquidation_price: liq_price,
                entry_usd_krw: ctx.usd_krw,
                entry_spread_pct: ctx.spread_pct,
                entry_z_score: ctx.z_score,
                qty: ctx.qty,
                state: PositionState::Opening,
                in_flight: true,
                client_order_id: Some(client_order_id.clone()),
                ..Default::default()
            };

            pm.open_position(pos)?;

            // 방금 추가한 포지션의 ID를 가져옴
            let pos_id = pm
                .open_positions
                .get(coin.as_str())
                .and_then(|ps| ps.last())
                .map(|p| p.id)
                .unwrap_or(0);

            // DB INSERT (Opening)
            let record = self.make_position_record(
                pm.open_positions
                    .get(coin.as_str())
                    .and_then(|ps| ps.last())
                    .unwrap(),
            );
            let db_id = match self.position_store.save(&record).await {
                Ok(id) => {
                    debug!(db_id = id, "DB 포지션 INSERT 완료 (Opening)");
                    // db_id를 메모리 포지션에 설정
                    if let Some(ps) = pm.open_positions.get_mut(coin.as_str())
                        && let Some(p) = ps.iter_mut().find(|p| p.id == pos_id)
                    {
                        p.db_id = Some(id);
                    }
                    id
                }
                Err(e) => {
                    error!(error = %e, "DB 포지션 INSERT 실패 (메모리는 등록됨)");
                    -1 // DB 실패해도 진행 (메모리가 authoritative)
                }
            };

            (pos_id, db_id)
        };
        // pm 락 해제 완료

        info!(
            coin = coin.as_str(),
            pos_id = pos_id,
            db_id = db_id,
            qty = %ctx.qty,
            upbit_krw = %ctx.upbit_price_krw,
            bybit_usdt = %ctx.bybit_entry,
            client_order_id = client_order_id.as_str(),
            "진입 주문 발주 시작"
        );

        // ⑤ LiveExecutor.execute_entry() — REST 호출 (pm 락 밖)
        let entry_request = EntryRequest {
            coin: coin.clone(),
            qty: ctx.qty,
            upbit_krw_price: ctx.upbit_price_krw,
            bybit_usdt_price: ctx.bybit_entry,
            usd_krw: ctx.usd_krw,
            instrument_info: ctx.instrument_info.clone(),
            client_order_id: client_order_id.clone(),
        };

        let exec_result = self.executor.execute_entry(&entry_request).await;

        // ⑥ pm.lock() → 체결 결과 반영
        match exec_result {
            Ok(executed) => {
                info!(
                    coin = coin.as_str(),
                    upbit_order_id = executed.upbit_order_id.as_str(),
                    bybit_order_id = executed.bybit_order_id.as_str(),
                    effective_qty = %executed.effective_qty,
                    upbit_avg_krw = %executed.upbit_avg_price_krw,
                    bybit_avg_usdt = %executed.bybit_avg_price,
                    adjustment_cost = %executed.adjustment_cost,
                    "진입 양 레그 체결 성공"
                );

                // pm 락 → state 전이 (Opening → Open)
                {
                    let mut pm = shared.position_mgr.lock().await;
                    if let Some(ps) = pm.open_positions.get_mut(coin.as_str())
                        && let Some(p) = ps.iter_mut().find(|p| p.id == pos_id)
                    {
                        p.state = PositionState::Open;
                        p.in_flight = false;
                        p.qty = executed.effective_qty;
                        p.upbit_order_id = Some(executed.upbit_order_id.clone());
                        p.bybit_order_id = Some(executed.bybit_order_id.clone());
                        // 실제 체결가 반영
                        let usd_krw_dec = Decimal::try_from(ctx.usd_krw).unwrap_or(Decimal::ONE);
                        if usd_krw_dec > Decimal::ZERO {
                            p.upbit_entry_price = executed.upbit_avg_price_krw / usd_krw_dec;
                        }
                        p.bybit_entry_price = executed.bybit_avg_price;
                    }
                }

                // DB 상태 전이 (Opening → Open)
                if db_id >= 0 {
                    self.db_update_state(
                        db_id,
                        "Opening",
                        "Open",
                        UpdateFields {
                            upbit_order_id: Some(executed.upbit_order_id),
                            bybit_order_id: Some(executed.bybit_order_id),
                            upbit_qty: Some(executed.upbit_filled_qty),
                            bybit_qty: Some(executed.bybit_filled_qty),
                            upbit_entry_price: Some(executed.upbit_avg_price_krw),
                            bybit_entry_price: Some(executed.bybit_avg_price),
                            in_flight: Some(false),
                            ..Default::default()
                        },
                    )
                    .await;
                }

                // 잔고 확정 (실 체결 금액 기준)
                let actual_upbit_krw = executed.upbit_avg_price_krw * executed.upbit_filled_qty;
                let actual_bybit_usdt = executed.bybit_avg_price * executed.bybit_filled_qty;
                self.balance_tracker
                    .commit(&mut reservation, actual_upbit_krw, actual_bybit_usdt);

                let expected_pnl = Decimal::try_from(ctx.adjusted_profit_pct)
                    .ok()
                    .map(|pct| actual_bybit_usdt * pct / Decimal::from(100u64))
                    .unwrap_or(Decimal::ZERO);
                self.emit_alert(AlertEvent::EntryExecuted {
                    coin: coin.clone(),
                    qty: executed.effective_qty,
                    upbit_price: executed.upbit_avg_price_krw,
                    bybit_price: executed.bybit_avg_price,
                    expected_pnl,
                });

                info!(
                    coin = coin.as_str(),
                    pos_id = pos_id,
                    effective_qty = %executed.effective_qty,
                    "진입 완료 (메모리 + DB)"
                );
                self.clear_upbit_ioc_reject_state(coin);

                // 잔고 스냅샷 — 진입 직후
                if let Some(sender) = &self.balance_sender
                    && sender.on_position_entry(db_id)
                {
                    shared.counters.lock().balance_snapshot_dropped += 1;
                }
            }
            Err(exec_err) => {
                // 주문 실패/부분 체결
                warn!(
                    coin = coin.as_str(),
                    error = %exec_err,
                    "진입 주문 실행 실패"
                );

                match &exec_err {
                    OrderExecutionError::BothUnfilled => {
                        // 양쪽 미체결 → 포지션 제거 + 예약 해제
                        {
                            let mut pm = shared.position_mgr.lock().await;
                            if let Some(ps) = pm.open_positions.get_mut(coin.as_str()) {
                                ps.retain(|p| p.id != pos_id);
                                if ps.is_empty() {
                                    pm.open_positions.remove(coin.as_str());
                                }
                            }
                        }

                        // DB 삭제
                        if db_id >= 0
                            && let Err(e) = self.position_store.remove(db_id).await
                        {
                            warn!(db_id = db_id, error = %e, "DB 포지션 삭제 실패");
                        }

                        self.balance_tracker.release(&mut reservation);
                        info!(coin = coin.as_str(), "양쪽 미체결, 포지션 제거 + 예약 해제");
                    }
                    OrderExecutionError::SingleLegFilled {
                        leg,
                        emergency_closed,
                        failed_leg_error,
                    } => {
                        if *leg == crate::zscore::live_executor::Leg::Bybit
                            && let Some(err) = failed_leg_error
                            && Self::is_upbit_ioc_rejection_message(err)
                        {
                            let blocked_until = self.record_upbit_ioc_reject(coin);
                            warn!(
                                coin = coin.as_str(),
                                error = err.as_str(),
                                blocked_until = ?blocked_until,
                                "Upbit IOC 주문 거부 감지"
                            );
                        }

                        self.emit_alert(AlertEvent::LegFailure {
                            coin: coin.clone(),
                            succeeded_leg: leg.to_string(),
                            failed_leg: match leg {
                                crate::zscore::live_executor::Leg::Upbit => "bybit".to_string(),
                                crate::zscore::live_executor::Leg::Bybit => "upbit".to_string(),
                            },
                            action_taken: if *emergency_closed {
                                "emergency_close_succeeded".to_string()
                            } else {
                                "kill_switch_triggered".to_string()
                            },
                        });

                        // 한쪽만 체결 → 비상 청산 결과에 따라 처리
                        if *emergency_closed {
                            // 비상 청산 성공 → 포지션 제거 + 예약 해제
                            {
                                let mut pm = shared.position_mgr.lock().await;
                                if let Some(ps) = pm.open_positions.get_mut(coin.as_str()) {
                                    ps.retain(|p| p.id != pos_id);
                                    if ps.is_empty() {
                                        pm.open_positions.remove(coin.as_str());
                                    }
                                }
                            }

                            if db_id >= 0 {
                                self.db_update_state(
                                    db_id,
                                    "Opening",
                                    "Closed",
                                    UpdateFields {
                                        succeeded_leg: Some(leg.to_string()),
                                        in_flight: Some(false),
                                        ..Default::default()
                                    },
                                )
                                .await;
                            }

                            self.balance_tracker.release(&mut reservation);
                            warn!(
                                coin = coin.as_str(),
                                leg = %leg,
                                "한쪽 체결 + 비상 청산 성공 → 포지션 제거"
                            );
                        } else {
                            // 비상 청산 실패 → PartiallyClosedOneLeg 상태로 전환
                            {
                                let mut pm = shared.position_mgr.lock().await;
                                if let Some(ps) = pm.open_positions.get_mut(coin.as_str())
                                    && let Some(p) = ps.iter_mut().find(|p| p.id == pos_id)
                                {
                                    p.state = PositionState::PartiallyClosedOneLeg;
                                    p.in_flight = false;
                                    p.succeeded_leg = Some(leg.to_string());
                                }
                            }

                            if db_id >= 0 {
                                self.db_update_state(
                                    db_id,
                                    "Opening",
                                    "PartiallyClosedOneLeg",
                                    UpdateFields {
                                        succeeded_leg: Some(leg.to_string()),
                                        in_flight: Some(false),
                                        ..Default::default()
                                    },
                                )
                                .await;
                            }

                            // 예약 유지 (비상 청산 재시도 필요)
                            error!(
                                coin = coin.as_str(),
                                leg = %leg,
                                "비상 청산 실패 — NAKED EXPOSURE — kill switch 검토 필요"
                            );

                            // kill switch 발동
                            self.risk_manager.trigger_kill_switch(&format!(
                                "emergency close failed: {} leg={}",
                                coin, leg
                            ));
                            self.emit_alert_critical(AlertEvent::EmergencyCloseFailure {
                                coin: coin.clone(),
                                retry_count: 1,
                                naked_exposure: bybit_usdt_needed,
                            })
                            .await;
                        }
                    }
                    OrderExecutionError::ExchangeError(_) | OrderExecutionError::Timeout { .. } => {
                        // 거래소 에러/타임아웃 → 포지션 제거 + 예약 해제
                        {
                            let mut pm = shared.position_mgr.lock().await;
                            if let Some(ps) = pm.open_positions.get_mut(coin.as_str()) {
                                ps.retain(|p| p.id != pos_id);
                                if ps.is_empty() {
                                    pm.open_positions.remove(coin.as_str());
                                }
                            }
                        }

                        if db_id >= 0
                            && let Err(e) = self.position_store.remove(db_id).await
                        {
                            warn!(db_id = db_id, error = %e, "DB 포지션 삭제 실패");
                        }

                        self.balance_tracker.release(&mut reservation);
                        warn!(
                            coin = coin.as_str(),
                            error = %exec_err,
                            "거래소 에러/타임아웃, 포지션 제거 + 예약 해제"
                        );
                    }
                    OrderExecutionError::EmergencyCloseFailed { leg, order_id } => {
                        error!(
                            coin = coin.as_str(),
                            leg = %leg,
                            order_id = order_id.as_str(),
                            "EmergencyCloseFailed — kill switch 발동"
                        );
                        self.risk_manager.trigger_kill_switch(&format!(
                            "emergency close failed: {} order_id={}",
                            leg, order_id
                        ));
                        self.emit_alert_critical(AlertEvent::EmergencyCloseFailure {
                            coin: coin.clone(),
                            retry_count: 1,
                            naked_exposure: bybit_usdt_needed,
                        })
                        .await;
                    }
                    OrderExecutionError::BothUnfilledWithErrors {
                        upbit_error,
                        bybit_error: _,
                    } => {
                        if let Some(err) = upbit_error
                            && Self::is_upbit_ioc_rejection_message(err)
                        {
                            let blocked_until = self.record_upbit_ioc_reject(coin);
                            warn!(
                                coin = coin.as_str(),
                                error = err.as_str(),
                                blocked_until = ?blocked_until,
                                "Upbit IOC 주문 거부 감지 (양쪽 미체결)"
                            );
                        }

                        {
                            let mut pm = shared.position_mgr.lock().await;
                            if let Some(ps) = pm.open_positions.get_mut(coin.as_str()) {
                                ps.retain(|p| p.id != pos_id);
                                if ps.is_empty() {
                                    pm.open_positions.remove(coin.as_str());
                                }
                            }
                        }

                        if db_id >= 0
                            && let Err(e) = self.position_store.remove(db_id).await
                        {
                            warn!(db_id = db_id, error = %e, "DB 포지션 삭제 실패");
                        }

                        self.balance_tracker.release(&mut reservation);
                        warn!(
                            coin = coin.as_str(),
                            "거래소 에러 동반 양쪽 미체결, 포지션 제거 + 예약 해제"
                        );
                    }
                }
            }
        }

        Ok(())
    }

    async fn on_exit_signal(&self, ctx: ExitContext) -> Result<(), StrategyError> {
        let shared = self.shared();
        let coin = &ctx.coin;
        let exit_safe_volume_usdt = ctx.exit_safe_volume_usdt.unwrap_or(0.0);

        if exit_safe_volume_usdt <= 0.0 {
            debug!(coin = coin.as_str(), "청산 거부: 오더북 안전 볼륨 0");
            return Ok(());
        }

        // pm 락 → 청산 대상 포지션 수집 + Closing 전이
        let positions_to_close: Vec<(u64, Option<i64>, Decimal, Decimal, f64)> = {
            let mut pm = shared.position_mgr.lock().await;
            let positions: Vec<(u64, Option<i64>, Decimal, Decimal, f64)> = pm
                .open_positions
                .get(coin.as_str())
                .map(|ps| {
                    ps.iter()
                        .filter(|p| p.state == PositionState::Open && !p.in_flight)
                        .map(|p| {
                            let profit_rate = (ctx.spread_pct - p.entry_spread_pct)
                                / p.size_usdt().to_f64().unwrap_or(1.0);
                            (p.id, p.db_id, p.qty, p.size_usdt(), profit_rate)
                        })
                        .collect()
                })
                .unwrap_or_default();

            // 수익률 높은 순으로 정렬
            let mut sorted = positions;
            sorted.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal));

            // 안전 볼륨 내에서 청산 대상 선택 + Closing 전이
            let mut remaining_safe_usdt = exit_safe_volume_usdt;
            let mut targets = Vec::new();

            for (pid, db_id, qty, size, _rate) in sorted {
                if remaining_safe_usdt <= 0.0 {
                    break;
                }
                let size_f64 = size.to_f64().unwrap_or(0.0);

                if remaining_safe_usdt >= size_f64 {
                    remaining_safe_usdt -= size_f64;
                } else {
                    // 부분 청산은 safe 볼륨 내에서만
                    remaining_safe_usdt = 0.0;
                }

                // Closing 전이 (메모리)
                if let Some(ps) = pm.open_positions.get_mut(coin.as_str())
                    && let Some(p) = ps.iter_mut().find(|p| p.id == pid)
                {
                    p.state = PositionState::Closing;
                    p.in_flight = true;
                    p.closing_started_at = Some(Utc::now());
                }

                targets.push((pid, db_id, qty, size, 0.0));
            }

            targets
        };
        // pm 락 해제

        if positions_to_close.is_empty() {
            debug!(coin = coin.as_str(), "청산 대상 포지션 없음");
            return Ok(());
        }

        // 각 포지션에 대해 청산 수행
        for (pid, db_id, qty, _size, _) in &positions_to_close {
            let exit_client_order_id = Self::new_client_order_id();

            // DB Closing 전이
            if let Some(db_id) = db_id {
                self.db_update_state(
                    *db_id,
                    "Open",
                    "Closing",
                    UpdateFields {
                        exit_client_order_id: Some(exit_client_order_id.clone()),
                        in_flight: Some(true),
                        ..Default::default()
                    },
                )
                .await;
            }

            info!(
                coin = coin.as_str(),
                pos_id = pid,
                qty = %qty,
                exit_client_order_id = exit_client_order_id.as_str(),
                "청산 주문 발주"
            );

            let exit_request = ExitRequest {
                coin: coin.clone(),
                qty: *qty,
                instrument_info: ctx.instrument_info.clone().unwrap_or_default(),
                exit_client_order_id: exit_client_order_id.clone(),
            };

            let exec_result = self.executor.execute_exit(&exit_request).await;

            match exec_result {
                Ok(executed) => {
                    info!(
                        coin = coin.as_str(),
                        pos_id = pid,
                        upbit_filled = %executed.upbit_filled_qty,
                        bybit_filled = %executed.bybit_filled_qty,
                        "청산 양 레그 체결 성공"
                    );

                    // pm 락 → close_position (메모리)
                    let closed_opt = {
                        let mut pm = shared.position_mgr.lock().await;
                        match pm.close_position(
                            coin,
                            *pid,
                            Utc::now(),
                            ctx.exit_upbit_usd,
                            ctx.exit_bybit,
                            ctx.usd_krw,
                            ctx.spread_pct,
                            ctx.z_score,
                            shared.config.upbit_taker_fee,
                            shared.config.bybit_taker_fee,
                            false,
                        ) {
                            Ok(closed) => Some(closed),
                            Err(e) => {
                                warn!(pos_id = pid, error = %e, "메모리 포지션 청산 실패");
                                None
                            }
                        }
                    };

                    if let Some(closed) = closed_opt {
                        // DB Closed 전이
                        if let Some(db_id) = db_id {
                            self.db_update_state(
                                *db_id,
                                "Closing",
                                "Closed",
                                UpdateFields {
                                    exit_upbit_order_id: Some(executed.upbit_order_id.clone()),
                                    exit_bybit_order_id: Some(executed.bybit_order_id.clone()),
                                    realized_pnl: Some(closed.net_pnl),
                                    in_flight: Some(false),
                                    ..Default::default()
                                },
                            )
                            .await;
                        }

                        // 잔고 복원
                        let received_upbit_krw =
                            executed.upbit_avg_price_krw * executed.upbit_filled_qty;
                        let received_bybit_usdt =
                            executed.bybit_avg_price * executed.bybit_filled_qty;
                        self.balance_tracker
                            .on_exit(received_upbit_krw, received_bybit_usdt);

                        // 리스크 기록
                        if let Some(reason) = self.risk_manager.record_trade(closed.net_pnl) {
                            error!(
                                reason = %reason,
                                pnl = %closed.net_pnl,
                                "청산 후 kill switch 발동"
                            );
                            self.emit_alert_critical(AlertEvent::KillSwitchTriggered {
                                reason: reason.to_string(),
                                daily_pnl: closed.net_pnl,
                            })
                            .await;
                        }

                        // trades + CSV 기록
                        self.record_trade(&closed).await;
                        self.on_trade_closed(&closed, *db_id).await;

                        // 잔고 스냅샷 — 청산 직후
                        if let Some(sender) = &self.balance_sender
                            && let Some(id) = db_id
                            && sender.on_position_exit(*id)
                        {
                            shared.counters.lock().balance_snapshot_dropped += 1;
                        }

                        info!(
                            coin = coin.as_str(),
                            pos_id = pid,
                            pnl = %closed.net_pnl,
                            "청산 완료"
                        );
                    }
                }
                Err(exec_err) => {
                    warn!(
                        coin = coin.as_str(),
                        pos_id = pid,
                        error = %exec_err,
                        "청산 주문 실행 실패"
                    );

                    // Closing → PendingExchangeRecovery (실패 시)
                    {
                        let mut pm = shared.position_mgr.lock().await;
                        if let Some(ps) = pm.open_positions.get_mut(coin.as_str())
                            && let Some(p) = ps.iter_mut().find(|p| p.id == *pid)
                        {
                            p.state = PositionState::PendingExchangeRecovery;
                            p.in_flight = false;
                        }
                    }

                    if let Some(db_id) = db_id {
                        self.db_update_state(
                            *db_id,
                            "Closing",
                            "PendingExchangeRecovery",
                            UpdateFields {
                                in_flight: Some(false),
                                ..Default::default()
                            },
                        )
                        .await;
                    }
                }
            }
        }

        Ok(())
    }

    async fn on_ttl_expiry(&self, ctx: TtlExpiryContext) -> Result<(), StrategyError> {
        let shared = self.shared();
        let coin = &ctx.coin;

        info!(
            coin = coin.as_str(),
            positions_count = ctx.positions.len(),
            force_close = ctx.force_close,
            "TTL 만료 청산 시작"
        );

        for ttl_pos in &ctx.positions {
            let exit_client_order_id = Self::new_client_order_id();

            // pm 락 → Closing 전이
            let db_id = {
                let mut pm = shared.position_mgr.lock().await;
                let mut found_db_id = None;
                if let Some(ps) = pm.open_positions.get_mut(coin.as_str())
                    && let Some(p) = ps.iter_mut().find(|p| p.id == ttl_pos.id)
                {
                    if p.state == PositionState::Open && !p.in_flight {
                        p.state = PositionState::Closing;
                        p.in_flight = true;
                        p.closing_started_at = Some(Utc::now());
                        p.exit_client_order_id = Some(exit_client_order_id.clone());
                        found_db_id = p.db_id;
                    } else {
                        debug!(
                            pos_id = ttl_pos.id,
                            state = %p.state,
                            in_flight = p.in_flight,
                            "TTL 청산 스킵: 적합하지 않은 상태"
                        );
                        continue;
                    }
                }
                found_db_id
            };

            // DB Closing 전이
            if let Some(db_id) = db_id {
                self.db_update_state(
                    db_id,
                    "Open",
                    "Closing",
                    UpdateFields {
                        exit_client_order_id: Some(exit_client_order_id.clone()),
                        in_flight: Some(true),
                        ..Default::default()
                    },
                )
                .await;
            }

            let stage = if ctx.force_close {
                "2단계 강제"
            } else {
                "1단계 TTL"
            };

            info!(
                coin = coin.as_str(),
                pos_id = ttl_pos.id,
                qty = %ttl_pos.qty,
                stage = stage,
                exit_client_order_id = exit_client_order_id.as_str(),
                "{} 청산 주문 발주", stage
            );

            let exit_request = ExitRequest {
                coin: coin.clone(),
                qty: ttl_pos.qty,
                instrument_info: ctx.instrument_info.clone().unwrap_or_default(),
                exit_client_order_id,
            };

            let exec_result = self.executor.execute_exit(&exit_request).await;

            match exec_result {
                Ok(executed) => {
                    // pm 락 → close_position
                    let closed_opt = {
                        let mut pm = shared.position_mgr.lock().await;
                        match pm.close_position(
                            coin,
                            ttl_pos.id,
                            Utc::now(),
                            ctx.exit_upbit_usd,
                            ctx.exit_bybit,
                            ctx.usd_krw,
                            ctx.current_spread_pct,
                            f64::NAN,
                            shared.config.upbit_taker_fee,
                            shared.config.bybit_taker_fee,
                            ctx.force_close,
                        ) {
                            Ok(closed) => Some(closed),
                            Err(e) => {
                                warn!(pos_id = ttl_pos.id, error = %e, "TTL 청산 메모리 실패");
                                None
                            }
                        }
                    };

                    if let Some(closed) = closed_opt {
                        // DB Closed 전이
                        if let Some(db_id) = db_id {
                            self.db_update_state(
                                db_id,
                                "Closing",
                                "Closed",
                                UpdateFields {
                                    exit_upbit_order_id: Some(executed.upbit_order_id.clone()),
                                    exit_bybit_order_id: Some(executed.bybit_order_id.clone()),
                                    realized_pnl: Some(closed.net_pnl),
                                    in_flight: Some(false),
                                    ..Default::default()
                                },
                            )
                            .await;
                        }

                        // 잔고 복원
                        let received_upbit_krw =
                            executed.upbit_avg_price_krw * executed.upbit_filled_qty;
                        let received_bybit_usdt =
                            executed.bybit_avg_price * executed.bybit_filled_qty;
                        self.balance_tracker
                            .on_exit(received_upbit_krw, received_bybit_usdt);

                        // 리스크 기록
                        if let Some(reason) = self.risk_manager.record_trade(closed.net_pnl) {
                            error!(
                                reason = %reason,
                                pnl = %closed.net_pnl,
                                "TTL 청산 후 kill switch 발동"
                            );
                            self.emit_alert_critical(AlertEvent::KillSwitchTriggered {
                                reason: reason.to_string(),
                                daily_pnl: closed.net_pnl,
                            })
                            .await;
                        }

                        // trades + CSV 기록
                        self.record_trade(&closed).await;
                        self.on_trade_closed(&closed, db_id).await;

                        // 잔고 스냅샷 — TTL 청산 직후
                        if let Some(sender) = &self.balance_sender
                            && let Some(id) = db_id
                            && sender.on_position_exit(id)
                        {
                            shared.counters.lock().balance_snapshot_dropped += 1;
                        }

                        if ctx.force_close {
                            shared.counters.lock().forced_liquidation_count += 1;
                        }

                        info!(
                            coin = coin.as_str(),
                            pos_id = ttl_pos.id,
                            pnl = %closed.net_pnl,
                            stage = stage,
                            "{} 청산 완료", stage
                        );
                    }
                }
                Err(exec_err) => {
                    warn!(
                        coin = coin.as_str(),
                        pos_id = ttl_pos.id,
                        error = %exec_err,
                        stage = stage,
                        "{} 청산 실행 실패", stage
                    );

                    // PendingExchangeRecovery 전이
                    {
                        let mut pm = shared.position_mgr.lock().await;
                        if let Some(ps) = pm.open_positions.get_mut(coin.as_str())
                            && let Some(p) = ps.iter_mut().find(|p| p.id == ttl_pos.id)
                        {
                            p.state = PositionState::PendingExchangeRecovery;
                            p.in_flight = false;
                        }
                    }

                    if let Some(db_id) = db_id {
                        self.db_update_state(
                            db_id,
                            "Closing",
                            "PendingExchangeRecovery",
                            UpdateFields {
                                in_flight: Some(false),
                                ..Default::default()
                            },
                        )
                        .await;
                    }

                    // 강제 청산 실패 시 kill switch 고려
                    if ctx.force_close {
                        error!(
                            coin = coin.as_str(),
                            pos_id = ttl_pos.id,
                            "강제 청산 실패 — kill switch 발동"
                        );
                        self.risk_manager.trigger_kill_switch(&format!(
                            "TTL force close failed: {} pos_id={}",
                            coin, ttl_pos.id
                        ));
                        self.emit_alert_critical(AlertEvent::EmergencyCloseFailure {
                            coin: coin.clone(),
                            retry_count: 1,
                            naked_exposure: ttl_pos.size_usdt,
                        })
                        .await;
                    }
                }
            }
        }

        Ok(())
    }

    fn is_entry_allowed(&self) -> bool {
        // RiskManager에 위임 (AtomicBool + Mutex 내부 확인)
        self.risk_manager.is_entry_allowed()
    }

    async fn on_minute_closed(&self, record: &MinuteRecord) {
        self.enqueue_minute_record(record).await;
    }

    async fn on_trade_closed(&self, closed: &ClosedPosition, position_db_id: Option<i64>) {
        self.enqueue_trade_record(closed, position_db_id).await;
        self.emit_alert(AlertEvent::ExitExecuted {
            coin: closed.coin.clone(),
            qty: closed.qty,
            realized_pnl: closed.net_pnl,
        });
    }

    fn bind_shared_resources(&self, resources: SharedResources) {
        let result = self.shared.set(LivePolicyShared {
            config: resources.config,
            position_mgr: resources.position_mgr,
            trades: resources.trades,
            counters: resources.counters,
            session_writer: resources.session_writer,
        });
        if result.is_err() {
            warn!("LivePolicy::bind_shared_resources() 중복 호출 무시");
        }
    }
}

// ---------------------------------------------------------------------------
// 테스트
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{Arc, Mutex as StdMutex};

    use arb_exchange::*;
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;
    use tokio::sync::Mutex;

    use crate::output::summary::MonitoringCounters;
    use crate::output::writer::SessionWriter;
    use crate::zscore::balance::BalanceTracker;
    use crate::zscore::config::ZScoreConfig;
    use crate::zscore::execution_policy::{SharedResources, TtlPosition};
    use crate::zscore::instrument::InstrumentInfo;
    use crate::zscore::pnl::ClosedPosition;
    use crate::zscore::position::PositionManager;
    use crate::zscore::position_store::{
        PositionRecord, PositionStore, TransitionResult, UpdateFields,
    };
    use crate::zscore::risk::{RiskConfig, RiskManager};

    // ===================================================================
    // Mock 거래소
    // ===================================================================

    #[derive(Debug, Clone)]
    struct MockOrderResponse {
        id: String,
        executed_volume: Decimal,
        avg_price: Option<Decimal>,
        paid_fee: Decimal,
        should_fail: bool,
    }

    impl Default for MockOrderResponse {
        fn default() -> Self {
            Self {
                id: "mock-001".to_string(),
                executed_volume: Decimal::new(1, 2),
                avg_price: Some(Decimal::new(42000, 0)),
                paid_fee: Decimal::ZERO,
                should_fail: false,
            }
        }
    }

    struct MockUpbit {
        response: Mutex<MockOrderResponse>,
    }

    impl MockUpbit {
        fn new(resp: MockOrderResponse) -> Self {
            Self {
                response: Mutex::new(resp),
            }
        }
    }

    impl MarketData for MockUpbit {
        fn name(&self) -> &str {
            "mock_upbit"
        }
        async fn get_ticker(&self, _: &[&str]) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }
        async fn get_orderbook(&self, _: &str, _: Option<u32>) -> ExchangeResult<OrderBook> {
            Err(ExchangeError::Unsupported("mock".into()))
        }
        async fn get_candles(
            &self,
            _: &str,
            _: CandleInterval,
            _: u32,
        ) -> ExchangeResult<Vec<Candle>> {
            Ok(vec![])
        }
        async fn get_candles_before(
            &self,
            _: &str,
            _: CandleInterval,
            _: u32,
            _: DateTime<Utc>,
        ) -> ExchangeResult<Vec<Candle>> {
            Ok(vec![])
        }
        async fn get_all_tickers(&self) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }
        fn market_code(base: &str, quote: &str) -> String {
            format!("{}-{}", quote, base)
        }
    }

    impl OrderManagement for MockUpbit {
        async fn place_order(&self, req: &OrderRequest) -> ExchangeResult<Order> {
            let resp = self.response.lock().await.clone();
            if resp.should_fail {
                return Err(ExchangeError::ApiError("mock upbit fail".into()));
            }
            Ok(Order {
                id: resp.id,
                market: req.market.clone(),
                side: req.side,
                order_type: req.order_type,
                status: OrderStatus::Filled,
                volume: req.volume.unwrap_or(Decimal::ZERO),
                remaining_volume: Decimal::ZERO,
                executed_volume: resp.executed_volume,
                price: req.price,
                avg_price: resp.avg_price,
                paid_fee: resp.paid_fee,
                created_at: Utc::now(),
                identifier: req.identifier.clone(),
            })
        }
        async fn cancel_order(&self, _: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".into()))
        }
        async fn get_order(&self, _: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".into()))
        }
        async fn get_open_orders(&self, _: Option<&str>) -> ExchangeResult<Vec<Order>> {
            Ok(vec![])
        }
        async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
            Ok(vec![])
        }
        async fn get_balance(&self, _: &str) -> ExchangeResult<Balance> {
            Err(ExchangeError::Unsupported("mock".into()))
        }
    }

    struct MockBybit {
        response: Mutex<MockOrderResponse>,
    }

    impl MockBybit {
        fn new(resp: MockOrderResponse) -> Self {
            Self {
                response: Mutex::new(resp),
            }
        }
    }

    impl MarketData for MockBybit {
        fn name(&self) -> &str {
            "mock_bybit"
        }
        async fn get_ticker(&self, _: &[&str]) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }
        async fn get_orderbook(&self, _: &str, _: Option<u32>) -> ExchangeResult<OrderBook> {
            Err(ExchangeError::Unsupported("mock".into()))
        }
        async fn get_candles(
            &self,
            _: &str,
            _: CandleInterval,
            _: u32,
        ) -> ExchangeResult<Vec<Candle>> {
            Ok(vec![])
        }
        async fn get_candles_before(
            &self,
            _: &str,
            _: CandleInterval,
            _: u32,
            _: DateTime<Utc>,
        ) -> ExchangeResult<Vec<Candle>> {
            Ok(vec![])
        }
        async fn get_all_tickers(&self) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }
        fn market_code(base: &str, _: &str) -> String {
            format!("{}USDT", base)
        }
    }

    impl OrderManagement for MockBybit {
        async fn place_order(&self, req: &OrderRequest) -> ExchangeResult<Order> {
            let resp = self.response.lock().await.clone();
            if resp.should_fail {
                return Err(ExchangeError::ApiError("mock bybit fail".into()));
            }
            Ok(Order {
                id: resp.id,
                market: req.market.clone(),
                side: req.side,
                order_type: req.order_type,
                status: OrderStatus::Filled,
                volume: req.volume.unwrap_or(Decimal::ZERO),
                remaining_volume: Decimal::ZERO,
                executed_volume: resp.executed_volume,
                price: req.price,
                avg_price: resp.avg_price,
                paid_fee: resp.paid_fee,
                created_at: Utc::now(),
                identifier: req.identifier.clone(),
            })
        }
        async fn cancel_order(&self, _: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".into()))
        }
        async fn get_order(&self, _: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::Unsupported("mock".into()))
        }
        async fn get_open_orders(&self, _: Option<&str>) -> ExchangeResult<Vec<Order>> {
            Ok(vec![])
        }
        async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
            Ok(vec![])
        }
        async fn get_balance(&self, _: &str) -> ExchangeResult<Balance> {
            Err(ExchangeError::Unsupported("mock".into()))
        }
    }

    impl InstrumentDataProvider for MockBybit {
        async fn get_instrument_info(&self, _: &str) -> ExchangeResult<InstrumentInfoResponse> {
            Ok(InstrumentInfoResponse {
                tick_size: Decimal::new(1, 2),
                qty_step: Decimal::new(1, 3),
                min_order_qty: Decimal::new(1, 3),
                max_order_qty: Decimal::new(100, 0),
                min_notional: Decimal::new(5, 0),
            })
        }
    }

    impl LinearOrderManagement for MockBybit {
        async fn place_order_linear(
            &self,
            request: &OrderRequest,
            _reduce_only: bool,
        ) -> ExchangeResult<Order> {
            self.place_order(request).await
        }

        async fn get_order_linear(&self, order_id: &str) -> ExchangeResult<Order> {
            self.get_order(order_id).await
        }

        async fn cancel_order_linear(
            &self,
            order_id: &str,
            _symbol: Option<&str>,
        ) -> ExchangeResult<Order> {
            self.cancel_order(order_id).await
        }
    }

    // ===================================================================
    // Mock PositionStore
    // ===================================================================

    struct MockPositionStore {
        records: StdMutex<Vec<PositionRecord>>,
        next_id: StdMutex<i64>,
    }

    impl MockPositionStore {
        fn new() -> Self {
            Self {
                records: StdMutex::new(Vec::new()),
                next_id: StdMutex::new(1),
            }
        }
    }

    impl PositionStore for MockPositionStore {
        async fn save(&self, pos: &PositionRecord) -> Result<i64, String> {
            let mut id_guard = self.next_id.lock().unwrap();
            let id = *id_guard;
            *id_guard += 1;
            let mut record = pos.clone();
            record.id = Some(id);
            self.records.lock().unwrap().push(record);
            Ok(id)
        }

        async fn update_state(
            &self,
            id: i64,
            from: &str,
            to: &str,
            _fields: UpdateFields,
        ) -> Result<TransitionResult, String> {
            let mut records = self.records.lock().unwrap();
            if let Some(rec) = records
                .iter_mut()
                .find(|r| r.id == Some(id) && r.state == from)
            {
                rec.state = to.to_string();
                Ok(TransitionResult::Applied)
            } else {
                Ok(TransitionResult::AlreadyTransitioned)
            }
        }

        async fn load_open(&self, session_id: i64) -> Result<Vec<PositionRecord>, String> {
            let records = self.records.lock().unwrap();
            Ok(records
                .iter()
                .filter(|r| r.session_id == session_id && r.state != "Closed")
                .cloned()
                .collect())
        }

        async fn remove(&self, id: i64) -> Result<(), String> {
            self.records.lock().unwrap().retain(|r| r.id != Some(id));
            Ok(())
        }
    }

    // ===================================================================
    // 헬퍼 함수
    // ===================================================================

    fn make_config() -> Arc<ZScoreConfig> {
        Arc::new(ZScoreConfig {
            max_slippage_pct: 0.1,
            order_timeout_sec: 5,
            max_dust_usdt: 5.0,
            upbit_taker_fee: Decimal::new(5, 4),
            bybit_taker_fee: Decimal::new(55, 5),
            emergency_wide_ioc_slippage_pct: vec![2.0, 3.0, 5.0],
            total_capital_usdt: Decimal::from(10000),
            max_position_ratio: Decimal::new(2, 1),
            max_concurrent_positions: Some(5),
            ..ZScoreConfig::default()
        })
    }

    fn make_risk_config() -> RiskConfig {
        RiskConfig {
            max_single_loss_usdt: Decimal::from(1000),
            max_single_loss_pct: 100.0,
            max_daily_loss_usdt: Decimal::from(1000),
            max_daily_loss_pct: 100.0,
            max_drawdown_usdt: Decimal::from(1000),
            max_drawdown_pct: 100.0,
            max_rolling_24h_loss_usdt: Decimal::from(1000),
            max_order_size_usdt: Decimal::from(5000),
            max_concurrent_positions: 5,
            total_capital_usdt: Decimal::from(10000),
            kill_switch_enabled: true,
            ..RiskConfig::default()
        }
    }

    fn make_entry_ctx() -> EntryContext {
        EntryContext {
            coin: "BTC".to_string(),
            z_score: 2.5,
            spread_pct: 0.3,
            expected_profit_pct: 0.09,
            adjusted_profit_pct: 0.08,
            upbit_price_krw: Decimal::new(60_000_000, 0),
            upbit_entry_usd: Decimal::new(42000, 0),
            bybit_entry: Decimal::new(42000, 0),
            qty: Decimal::new(1, 2), // 0.01 BTC
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

    #[allow(clippy::type_complexity)]
    fn make_live_policy(
        upbit_resp: MockOrderResponse,
        bybit_resp: MockOrderResponse,
    ) -> (
        LivePolicy<MockUpbit, MockBybit, MockPositionStore>,
        Arc<ZScoreConfig>,
        Arc<tokio::sync::Mutex<PositionManager>>,
        Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
        Arc<parking_lot::Mutex<MonitoringCounters>>,
        Arc<BalanceTracker>,
        Arc<RiskManager>,
        Arc<MockPositionStore>,
    ) {
        let upbit = Arc::new(MockUpbit::new(upbit_resp));
        let bybit = Arc::new(MockBybit::new(bybit_resp));
        let config = make_config();
        let executor = Arc::new(LiveExecutor::new(upbit, bybit, Arc::clone(&config)));

        let balance_tracker = Arc::new(BalanceTracker::new(
            Decimal::from(100_000_000), // 1억 KRW
            Decimal::from(10_000),      // 10,000 USDT
        ));
        let risk_manager = Arc::new(RiskManager::new(make_risk_config()));
        let position_store = Arc::new(MockPositionStore::new());

        let policy = LivePolicy::new(
            executor,
            Arc::clone(&balance_tracker),
            Arc::clone(&risk_manager),
            Arc::clone(&position_store),
            1, // session_id
            None::<DbWriter>,
            None::<AlertService>,
            None::<BalanceSnapshotSender>,
        );

        let pm = Arc::new(tokio::sync::Mutex::new(PositionManager::new()));
        let trades = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let counters = Arc::new(parking_lot::Mutex::new(MonitoringCounters::default()));
        let session_writer = Arc::new(tokio::sync::Mutex::new(None::<SessionWriter>));

        policy.bind_shared_resources(SharedResources {
            config: Arc::clone(&config),
            position_mgr: Arc::clone(&pm),
            trades: Arc::clone(&trades),
            counters: Arc::clone(&counters),
            session_writer,
        });

        (
            policy,
            config,
            pm,
            trades,
            counters,
            balance_tracker,
            risk_manager,
            position_store,
        )
    }

    // ===================================================================
    // is_entry_allowed 테스트
    // ===================================================================

    #[test]
    fn test_is_entry_allowed_normal() {
        let (policy, ..) =
            make_live_policy(MockOrderResponse::default(), MockOrderResponse::default());
        assert!(policy.is_entry_allowed());
    }

    #[test]
    fn test_is_entry_allowed_kill_switch() {
        let (policy, _, _, _, _, _, risk_manager, _) =
            make_live_policy(MockOrderResponse::default(), MockOrderResponse::default());
        risk_manager.trigger_kill_switch("test kill");
        assert!(!policy.is_entry_allowed());
    }

    // ===================================================================
    // on_entry_signal 테스트
    // ===================================================================

    #[tokio::test]
    async fn test_entry_both_filled() {
        let (policy, _, pm, _trades, _, balance_tracker, _, position_store) = make_live_policy(
            MockOrderResponse {
                id: "upbit-001".into(),
                executed_volume: Decimal::new(1, 2),
                avg_price: Some(Decimal::new(60_000_000, 0)),
                paid_fee: Decimal::new(30_000, 0),
                ..Default::default()
            },
            MockOrderResponse {
                id: "bybit-001".into(),
                executed_volume: Decimal::new(1, 2),
                avg_price: Some(Decimal::new(42000, 0)),
                paid_fee: Decimal::new(231, 3),
                ..Default::default()
            },
        );

        let result = policy.on_entry_signal(make_entry_ctx()).await;
        assert!(result.is_ok());

        // 메모리 포지션 확인
        let pm = pm.lock().await;
        let btc_positions = pm.open_positions.get("BTC");
        assert!(btc_positions.is_some());
        let positions = btc_positions.unwrap();
        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0].state, PositionState::Open);
        assert!(!positions[0].in_flight);
        assert!(positions[0].upbit_order_id.is_some());
        assert!(positions[0].bybit_order_id.is_some());
        assert!(positions[0].db_id.is_some());

        // DB 레코드 확인
        let records = position_store.records.lock().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].state, "Open");

        // 잔고 확인 (예약 확정 후 차감됨)
        let (upbit_avail, bybit_avail) = balance_tracker.available();
        assert!(upbit_avail < Decimal::from(100_000_000));
        assert!(bybit_avail < Decimal::from(10_000));
    }

    #[tokio::test]
    async fn test_entry_both_failed() {
        let (policy, _, pm, _, _, balance_tracker, _, position_store) = make_live_policy(
            MockOrderResponse {
                should_fail: true,
                ..Default::default()
            },
            MockOrderResponse {
                should_fail: true,
                ..Default::default()
            },
        );

        let result = policy.on_entry_signal(make_entry_ctx()).await;
        assert!(result.is_ok());

        // 포지션 없어야 함 (제거됨)
        let pm = pm.lock().await;
        assert!(!pm.open_positions.contains_key("BTC"));

        // DB 레코드도 삭제됨
        let records = position_store.records.lock().unwrap();
        assert!(records.is_empty());

        // 잔고 복원 확인
        let (upbit_avail, bybit_avail) = balance_tracker.available();
        assert_eq!(upbit_avail, Decimal::from(100_000_000));
        assert_eq!(bybit_avail, Decimal::from(10_000));
    }

    #[tokio::test]
    async fn test_entry_kill_switch_blocks() {
        let (policy, _, pm, _, _, _, risk_manager, _) =
            make_live_policy(MockOrderResponse::default(), MockOrderResponse::default());

        risk_manager.trigger_kill_switch("test");

        let result = policy.on_entry_signal(make_entry_ctx()).await;
        assert!(result.is_ok());

        // 포지션 없어야 함
        let pm = pm.lock().await;
        assert!(!pm.open_positions.contains_key("BTC"));
    }

    #[tokio::test]
    async fn test_entry_insufficient_balance() {
        let upbit = Arc::new(MockUpbit::new(MockOrderResponse::default()));
        let bybit = Arc::new(MockBybit::new(MockOrderResponse::default()));
        let config = make_config();
        let executor = Arc::new(LiveExecutor::new(upbit, bybit, Arc::clone(&config)));

        // 잔고를 매우 적게 설정
        let balance_tracker = Arc::new(BalanceTracker::new(
            Decimal::from(100), // 100 KRW (부족)
            Decimal::from(1),   // 1 USDT (부족)
        ));
        let risk_manager = Arc::new(RiskManager::new(make_risk_config()));
        let position_store = Arc::new(MockPositionStore::new());

        let policy = LivePolicy::new(
            executor,
            Arc::clone(&balance_tracker),
            Arc::clone(&risk_manager),
            Arc::clone(&position_store),
            1,
            None::<DbWriter>,
            None::<AlertService>,
            None::<BalanceSnapshotSender>,
        );

        let pm = Arc::new(tokio::sync::Mutex::new(PositionManager::new()));
        let trades = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let counters = Arc::new(parking_lot::Mutex::new(MonitoringCounters::default()));
        let session_writer = Arc::new(tokio::sync::Mutex::new(None::<SessionWriter>));

        policy.bind_shared_resources(SharedResources {
            config: Arc::clone(&config),
            position_mgr: Arc::clone(&pm),
            trades: Arc::clone(&trades),
            counters: Arc::clone(&counters),
            session_writer,
        });

        let result = policy.on_entry_signal(make_entry_ctx()).await;
        assert!(result.is_ok());

        // 포지션 없어야 함
        let pm_guard = pm.lock().await;
        assert!(!pm_guard.open_positions.contains_key("BTC"));

        // 카운터 확인
        assert!(counters.lock().entry_rejected_order_constraint_count > 0);
    }

    // ===================================================================
    // on_exit_signal 테스트
    // ===================================================================

    #[tokio::test]
    async fn test_exit_success() {
        let (policy, _config, pm, trades, _, _balance_tracker, _, position_store) =
            make_live_policy(
                MockOrderResponse {
                    id: "upbit-exit-001".into(),
                    executed_volume: Decimal::new(1, 2),
                    avg_price: Some(Decimal::new(61_000_000, 0)),
                    paid_fee: Decimal::ZERO,
                    ..Default::default()
                },
                MockOrderResponse {
                    id: "bybit-exit-001".into(),
                    executed_volume: Decimal::new(1, 2),
                    avg_price: Some(Decimal::new(41500, 0)),
                    paid_fee: Decimal::ZERO,
                    ..Default::default()
                },
            );

        // 먼저 Open 포지션 생성 (직접 삽입)
        {
            let mut pm_guard = pm.lock().await;
            let pos = VirtualPosition {
                id: 0,
                coin: "BTC".to_string(),
                entry_time: Utc::now(),
                upbit_entry_price: Decimal::new(42000, 0),
                bybit_entry_price: Decimal::new(42050, 0),
                bybit_liquidation_price: Decimal::new(50000, 0),
                entry_usd_krw: 1380.0,
                entry_spread_pct: 0.2,
                entry_z_score: 2.0,
                qty: Decimal::new(1, 2),
                state: PositionState::Open,
                db_id: Some(1),
                ..Default::default()
            };
            pm_guard.open_position(pos).unwrap();
        }

        // DB에도 레코드 삽입
        {
            let mut records = position_store.records.lock().unwrap();
            records.push(PositionRecord {
                id: Some(1),
                session_id: 1,
                coin: "BTC".to_string(),
                state: "Open".to_string(),
                upbit_qty: Decimal::new(1, 2),
                bybit_qty: Decimal::new(1, 2),
                upbit_entry_price: Some(Decimal::new(60_000_000, 0)),
                bybit_entry_price: Some(Decimal::new(42050, 0)),
                upbit_order_id: None,
                bybit_order_id: None,
                entry_spread_pct: Some(0.2),
                entry_z_score: Some(2.0),
                entry_usd_krw: Some(1380.0),
                opened_at: Some(Utc::now()),
                closed_at: None,
                realized_pnl: None,
                exit_upbit_order_id: None,
                exit_bybit_order_id: None,
                client_order_id: None,
                exit_client_order_id: None,
                in_flight: false,
                succeeded_leg: None,
                emergency_attempts: 0,
            });
        }

        let exit_ctx = ExitContext {
            coin: "BTC".to_string(),
            z_score: 0.5,
            spread_pct: 0.1,
            exit_upbit_usd: Decimal::new(42500, 0),
            exit_bybit: Decimal::new(41500, 0),
            usd_krw: 1380.0,
            exit_safe_volume_usdt: Some(1000.0),
            mean: 0.1,
            instrument_info: Some(InstrumentInfo {
                tick_size: Decimal::new(1, 1),
                qty_step: Decimal::new(1, 3),
                min_order_qty: Decimal::new(1, 3),
                min_notional: Decimal::new(5, 0),
                max_order_qty: Decimal::new(100, 0),
            }),
            bybit_price: Decimal::new(41500, 0),
        };

        let result = policy.on_exit_signal(exit_ctx).await;
        assert!(result.is_ok());

        // trades에 기록 확인
        let trades_guard = trades.lock().await;
        assert_eq!(trades_guard.len(), 1);

        // DB 상태 Closed 확인
        let records = position_store.records.lock().unwrap();
        assert_eq!(records[0].state, "Closed");
    }

    #[tokio::test]
    async fn test_exit_no_positions() {
        let (policy, ..) =
            make_live_policy(MockOrderResponse::default(), MockOrderResponse::default());

        let exit_ctx = ExitContext {
            coin: "BTC".to_string(),
            z_score: 0.5,
            spread_pct: 0.1,
            exit_upbit_usd: Decimal::new(42500, 0),
            exit_bybit: Decimal::new(41500, 0),
            usd_krw: 1380.0,
            exit_safe_volume_usdt: Some(1000.0),
            mean: 0.1,
            instrument_info: None,
            bybit_price: Decimal::new(41500, 0),
        };

        let result = policy.on_exit_signal(exit_ctx).await;
        assert!(result.is_ok());
    }

    // ===================================================================
    // on_ttl_expiry 테스트
    // ===================================================================

    #[tokio::test]
    async fn test_ttl_expiry_success() {
        let (policy, _, pm, trades, counters, _, _, position_store) = make_live_policy(
            MockOrderResponse {
                id: "upbit-ttl".into(),
                executed_volume: Decimal::new(1, 2),
                avg_price: Some(Decimal::new(61_000_000, 0)),
                paid_fee: Decimal::ZERO,
                ..Default::default()
            },
            MockOrderResponse {
                id: "bybit-ttl".into(),
                executed_volume: Decimal::new(1, 2),
                avg_price: Some(Decimal::new(41500, 0)),
                paid_fee: Decimal::ZERO,
                ..Default::default()
            },
        );

        // Open 포지션 생성
        let pos_id = {
            let mut pm_guard = pm.lock().await;
            let pos = VirtualPosition {
                id: 0,
                coin: "XRP".to_string(),
                entry_time: Utc::now() - chrono::Duration::hours(25),
                upbit_entry_price: Decimal::new(1, 0),
                bybit_entry_price: Decimal::new(1, 0),
                bybit_liquidation_price: Decimal::new(2, 0),
                entry_usd_krw: 1380.0,
                entry_spread_pct: 0.2,
                entry_z_score: 2.0,
                qty: Decimal::new(1, 2),
                state: PositionState::Open,
                db_id: Some(1),
                ..Default::default()
            };
            pm_guard.open_position(pos).unwrap();
            pm_guard.open_positions.get("XRP").unwrap()[0].id
        };

        // DB 레코드
        {
            let mut records = position_store.records.lock().unwrap();
            records.push(PositionRecord {
                id: Some(1),
                session_id: 1,
                coin: "XRP".to_string(),
                state: "Open".to_string(),
                upbit_qty: Decimal::new(1, 2),
                bybit_qty: Decimal::new(1, 2),
                upbit_entry_price: None,
                bybit_entry_price: None,
                upbit_order_id: None,
                bybit_order_id: None,
                entry_spread_pct: None,
                entry_z_score: None,
                entry_usd_krw: None,
                opened_at: None,
                closed_at: None,
                realized_pnl: None,
                exit_upbit_order_id: None,
                exit_bybit_order_id: None,
                client_order_id: None,
                exit_client_order_id: None,
                in_flight: false,
                succeeded_leg: None,
                emergency_attempts: 0,
            });
        }

        let ttl_ctx = TtlExpiryContext {
            coin: "XRP".to_string(),
            positions: vec![TtlPosition {
                id: pos_id,
                size_usdt: Decimal::new(1, 2),
                qty: Decimal::new(1, 2),
            }],
            usd_krw: 1380.0,
            current_spread_pct: 0.15,
            z_score: 1.2,
            instrument_info: Some(InstrumentInfo {
                tick_size: Decimal::new(1, 1),
                qty_step: Decimal::new(1, 3),
                min_order_qty: Decimal::new(1, 3),
                min_notional: Decimal::new(5, 0),
                max_order_qty: Decimal::new(100, 0),
            }),
            exit_upbit_usd: Decimal::new(1, 0),
            exit_bybit: Decimal::new(1, 0),
            force_close: true,
        };

        let result = policy.on_ttl_expiry(ttl_ctx).await;
        assert!(result.is_ok());

        // trades 기록 확인
        let trades_guard = trades.lock().await;
        assert_eq!(trades_guard.len(), 1);

        // forced_liquidation 카운터 확인
        assert_eq!(counters.lock().forced_liquidation_count, 1);

        // DB Closed 확인
        let records = position_store.records.lock().unwrap();
        assert_eq!(records[0].state, "Closed");
    }

    #[tokio::test]
    async fn test_ttl_expiry_skips_in_flight() {
        let (policy, _, pm, trades, ..) =
            make_live_policy(MockOrderResponse::default(), MockOrderResponse::default());

        // in_flight 포지션 생성
        let pos_id = {
            let mut pm_guard = pm.lock().await;
            let pos = VirtualPosition {
                id: 0,
                coin: "ETH".to_string(),
                entry_time: Utc::now(),
                upbit_entry_price: Decimal::ONE,
                bybit_entry_price: Decimal::ONE,
                bybit_liquidation_price: Decimal::new(2, 0),
                entry_usd_krw: 1380.0,
                entry_spread_pct: 0.2,
                entry_z_score: 2.0,
                qty: Decimal::new(1, 2),
                state: PositionState::Open,
                in_flight: true, // in_flight
                ..Default::default()
            };
            pm_guard.open_position(pos).unwrap();
            pm_guard.open_positions.get("ETH").unwrap()[0].id
        };

        let ttl_ctx = TtlExpiryContext {
            coin: "ETH".to_string(),
            positions: vec![TtlPosition {
                id: pos_id,
                size_usdt: Decimal::new(100, 0),
                qty: Decimal::new(1, 2),
            }],
            usd_krw: 1380.0,
            current_spread_pct: 0.15,
            z_score: 1.2,
            instrument_info: None,
            exit_upbit_usd: Decimal::ONE,
            exit_bybit: Decimal::ONE,
            force_close: false,
        };

        let result = policy.on_ttl_expiry(ttl_ctx).await;
        assert!(result.is_ok());

        // trades 비어있어야 함 (in_flight 스킵)
        let trades_guard = trades.lock().await;
        assert!(trades_guard.is_empty());
    }

    // ===================================================================
    // bind_shared_resources 테스트
    // ===================================================================

    #[test]
    fn test_bind_shared_resources_once() {
        let (policy, ..) =
            make_live_policy(MockOrderResponse::default(), MockOrderResponse::default());

        // 이미 바인딩된 상태에서 다시 바인딩 시도 → warn 로그만 (패닉 아님)
        let config = make_config();
        let pm = Arc::new(tokio::sync::Mutex::new(PositionManager::new()));
        let trades = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let counters = Arc::new(parking_lot::Mutex::new(MonitoringCounters::default()));
        let sw = Arc::new(tokio::sync::Mutex::new(None::<SessionWriter>));

        policy.bind_shared_resources(SharedResources {
            config,
            position_mgr: pm,
            trades,
            counters,
            session_writer: sw,
        });
        // 두 번째 호출은 무시됨 (패닉하지 않음)
    }

    #[test]
    fn test_is_upbit_ioc_rejection_message() {
        assert!(
            LivePolicy::<MockUpbit, MockBybit, MockPositionStore>::is_upbit_ioc_rejection_message(
                "Invalid time_in_force: IOC is not supported"
            )
        );
        assert!(
            LivePolicy::<MockUpbit, MockBybit, MockPositionStore>::is_upbit_ioc_rejection_message(
                "validation error: ord_type"
            )
        );
        assert!(
            !LivePolicy::<MockUpbit, MockBybit, MockPositionStore>::is_upbit_ioc_rejection_message(
                "network timeout"
            )
        );
    }
}
