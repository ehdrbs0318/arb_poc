//! 실행 정책 trait (시뮬레이션/라이브 컴파일 타임 분기).
//!
//! `ExecutionPolicy`는 Z-Score 시그널 발생 시 실제 체결을 수행하는 방법을 추상화합니다.
//! - `SimPolicy`: 가상 체결 (기존 VirtualPosition 즉시 생성)
//! - `LivePolicy`: 실주문 (LiveExecutor를 통한 IOC 지정가 + 비상 청산)
//!
//! RPITIT 패턴 사용 — tokio::spawn 내 호출을 위해 Send 필수.

use std::future::Future;
use std::sync::Arc;

use rust_decimal::Decimal;

use crate::error::StrategyError;
use crate::output::summary::MonitoringCounters;
use crate::output::writer::{MinuteRecord, SessionWriter};
use crate::zscore::config::ZScoreConfig;
use crate::zscore::instrument::InstrumentInfo;
use crate::zscore::pnl::ClosedPosition;
use crate::zscore::position::PositionManager;

/// 진입 시그널 컨텍스트 (owned 스냅샷, Send + 'static).
///
/// monitor_core에서 시그널 평가 + 9단계 검증 완료 후 생성됩니다.
/// 모든 필드는 Copy 가능하거나 owned 타입입니다.
#[derive(Debug, Clone)]
pub struct EntryContext {
    /// 코인 심볼.
    pub coin: String,
    /// 진입 Z-Score.
    pub z_score: f64,
    /// 진입 시 스프레드 (%).
    pub spread_pct: f64,
    /// 수수료 기반 기대 수익률 (%, 라운딩 전).
    pub expected_profit_pct: f64,
    /// 라운딩 후 조정된 기대 수익률 (%).
    pub adjusted_profit_pct: f64,
    /// Upbit 진입가 KRW (라운딩 전).
    pub upbit_price_krw: Decimal,
    /// Upbit 진입가 USD (라운딩 후, ceil).
    pub upbit_entry_usd: Decimal,
    /// Bybit 진입가 USDT (라운딩 후, floor).
    pub bybit_entry: Decimal,
    /// 진입 수량 (코인 단위, qty_step 라운딩 완료).
    pub qty: Decimal,
    /// USD/KRW 환율.
    pub usd_krw: f64,
    /// Rolling mean.
    pub mean: f64,
    /// Rolling stddev.
    pub stddev: f64,
    /// InstrumentInfo (tick_size, qty_step 등).
    pub instrument_info: InstrumentInfo,
    /// 오더북 안전 볼륨 (USDT).
    pub safe_volume_usdt: f64,
    /// 오더북 볼륨 비율.
    pub volume_ratio: f64,
}

/// 청산 시그널 컨텍스트 (owned 스냅샷, Send + 'static).
#[derive(Debug, Clone)]
pub struct ExitContext {
    /// 코인 심볼.
    pub coin: String,
    /// 청산 Z-Score.
    pub z_score: f64,
    /// 청산 시 스프레드 (%).
    pub spread_pct: f64,
    /// Upbit 매도가 USD (라운딩 후, floor).
    pub exit_upbit_usd: Decimal,
    /// Bybit close (매수)가 USDT (라운딩 후, ceil).
    pub exit_bybit: Decimal,
    /// USD/KRW 환율.
    pub usd_krw: f64,
    /// 청산 안전 볼륨 (USDT, None이면 오더북 미확보).
    pub exit_safe_volume_usdt: Option<f64>,
    /// Rolling mean.
    pub mean: f64,
    /// InstrumentInfo.
    pub instrument_info: Option<InstrumentInfo>,
    /// Bybit 현재가 (USDT, 부분 청산 qty 변환용).
    pub bybit_price: Decimal,
}

/// TTL 만료 청산 컨텍스트 (owned 스냅샷, Send + 'static).
#[derive(Debug, Clone)]
pub struct TtlExpiryContext {
    /// 코인 심볼.
    pub coin: String,
    /// 만료된 포지션들 (id, size_usdt).
    pub positions: Vec<TtlPosition>,
    /// USD/KRW 환율.
    pub usd_krw: f64,
    /// 현재 스프레드 (%).
    pub current_spread_pct: f64,
    /// 현재 Z-Score.
    pub z_score: f64,
    /// InstrumentInfo.
    pub instrument_info: Option<InstrumentInfo>,
    /// Upbit 현재가 USD (라운딩 후).
    pub exit_upbit_usd: Decimal,
    /// Bybit 현재가 (라운딩 후).
    pub exit_bybit: Decimal,
    /// grace period 초과 여부 (강제 청산 모드).
    pub force_close: bool,
}

/// TTL 만료 포지션 정보.
#[derive(Debug, Clone)]
pub struct TtlPosition {
    /// 포지션 ID.
    pub id: u64,
    /// 포지션 크기 (USDT).
    pub size_usdt: Decimal,
    /// 포지션 수량 (코인 단위).
    pub qty: Decimal,
}

/// run() 내부에서 생성되는 공유 리소스.
///
/// 시뮬레이션 정책(SimPolicy)은 이 리소스를 통해
/// PositionManager, trades, counters, session_writer에 접근합니다.
/// 라이브 정책(LivePolicy)은 자체 인프라를 사용하므로 이 리소스를 무시합니다.
pub struct SharedResources {
    /// Z-Score 설정.
    pub config: Arc<ZScoreConfig>,
    /// 포지션 매니저 (가상 포지션 관리).
    pub position_mgr: Arc<tokio::sync::Mutex<PositionManager>>,
    /// 체결 완료된 거래 목록.
    pub trades: Arc<tokio::sync::Mutex<Vec<ClosedPosition>>>,
    /// 모니터링 카운터 (통계).
    pub counters: Arc<parking_lot::Mutex<MonitoringCounters>>,
    /// 세션 CSV 기록기.
    pub session_writer: Arc<tokio::sync::Mutex<Option<SessionWriter>>>,
}

/// 실행 정책 trait.
///
/// RPITIT 패턴 사용 — tokio::spawn 내 호출을 위해 Send 필수.
/// 현재 codebase의 MarketData trait과 동일한 패턴입니다.
///
/// ## 시뮬레이션 (SimPolicy)
/// - `on_entry_signal`: VirtualPosition 생성 → PositionManager.open_position()
/// - `on_exit_signal`: PositionManager.close_position() / close_partial()
/// - `on_ttl_expiry`: 가상 TTL 청산
/// - `is_entry_allowed`: 항상 true
///
/// ## 라이브 (LivePolicy)
/// - `on_entry_signal`: BalanceTracker.reserve → pm.register_opening → LiveExecutor.execute_entry
/// - `on_exit_signal`: pm.transition_to_closing → LiveExecutor.execute_exit
/// - `on_ttl_expiry`: 실제 TTL 청산 (LiveExecutor 사용)
/// - `is_entry_allowed`: RiskManager.is_entry_allowed() + 잔고 + reconciliation 상태
pub trait ExecutionPolicy: Send + Sync + 'static {
    /// 진입 시그널 처리.
    ///
    /// 9단계 검증 통과 후 호출됩니다.
    /// 시뮬: VirtualPosition 즉시 생성.
    /// 라이브: BalanceTracker.reserve → LiveExecutor.execute_entry → commit/release.
    fn on_entry_signal(
        &self,
        ctx: EntryContext,
    ) -> impl Future<Output = Result<(), StrategyError>> + Send;

    /// 청산 시그널 처리.
    ///
    /// Z-Score 청산 조건 충족 시 호출됩니다.
    /// 시뮬: PositionManager.close_position() 가상 체결.
    /// 라이브: LiveExecutor.execute_exit() 실주문.
    fn on_exit_signal(
        &self,
        ctx: ExitContext,
    ) -> impl Future<Output = Result<(), StrategyError>> + Send;

    /// TTL 만료 포지션 청산.
    ///
    /// minute_timer에서 만료된 포지션 감지 시 호출됩니다.
    /// 시뮬: 가상 TTL 청산.
    /// 라이브: LiveExecutor.execute_exit() + tokio::spawn으로 분리.
    fn on_ttl_expiry(
        &self,
        ctx: TtlExpiryContext,
    ) -> impl Future<Output = Result<(), StrategyError>> + Send;

    /// 진입 가능 여부 확인 (lock-free, 빠른 체크).
    ///
    /// select! 루프 또는 spawned_check_tick_signal 초반에 호출됩니다.
    /// 시뮬: 항상 true.
    /// 라이브: RiskManager.is_entry_allowed() + reconciliation 상태 등.
    fn is_entry_allowed(&self) -> bool;

    /// 분봉 완결 레코드를 후처리합니다.
    ///
    /// 기본 구현은 no-op이며, LivePolicy에서 DB INSERT producer를 연결합니다.
    fn on_minute_closed(&self, _record: &MinuteRecord) -> impl Future<Output = ()> + Send {
        async {}
    }

    /// 청산 완료 거래를 후처리합니다.
    ///
    /// 기본 구현은 no-op이며, LivePolicy에서 DB INSERT producer를 연결합니다.
    /// `position_db_id`는 DB `positions.id`이며, 없으면 `None`입니다.
    fn on_trade_closed(
        &self,
        _closed: &ClosedPosition,
        _position_db_id: Option<i64>,
    ) -> impl Future<Output = ()> + Send {
        async {}
    }

    /// 공유 리소스를 바인딩합니다.
    ///
    /// `ZScoreMonitor::run()` 내부에서 공유 상태 생성 후 호출됩니다.
    /// SimPolicy: OnceLock으로 내부 상태 설정.
    /// LivePolicy: 자체 인프라를 사용하므로 기본 구현 (no-op).
    fn bind_shared_resources(&self, _resources: SharedResources) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_entry_context_clone() {
        let ctx = EntryContext {
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
        };
        let cloned = ctx.clone();
        assert_eq!(cloned.coin, "BTC");
        assert_eq!(cloned.z_score, 2.5);
    }

    #[test]
    fn test_exit_context_clone() {
        let ctx = ExitContext {
            coin: "ETH".to_string(),
            z_score: 0.5,
            spread_pct: 0.1,
            exit_upbit_usd: Decimal::new(3000, 0),
            exit_bybit: Decimal::new(3005, 0),
            usd_krw: 1380.0,
            exit_safe_volume_usdt: Some(500.0),
            mean: 0.1,
            instrument_info: None,
            bybit_price: Decimal::new(3005, 0),
        };
        let cloned = ctx.clone();
        assert_eq!(cloned.coin, "ETH");
    }

    #[test]
    fn test_ttl_context_force_close() {
        let ctx = TtlExpiryContext {
            coin: "XRP".to_string(),
            positions: vec![TtlPosition {
                id: 1,
                size_usdt: Decimal::new(100, 0),
                qty: Decimal::new(500, 0),
            }],
            usd_krw: 1380.0,
            current_spread_pct: 0.15,
            z_score: 1.2,
            instrument_info: None,
            exit_upbit_usd: Decimal::new(1, 0),
            exit_bybit: Decimal::new(1, 0),
            force_close: true,
        };
        assert!(ctx.force_close);
        assert_eq!(ctx.positions.len(), 1);
    }
}
