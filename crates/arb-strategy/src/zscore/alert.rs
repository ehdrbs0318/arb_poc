//! 텔레그램 알림 서비스.
//!
//! AlertService는 라이브 트레이딩에서 중요한 이벤트를 텔레그램으로 전송합니다.
//! 일반 알림은 mpsc 비동기 채널로 처리하며, DB alerts 테이블에는 항상 감사로그를 기록합니다.
//! 텔레그램 전송은 best-effort 채널입니다.

use rust_decimal::Decimal;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

/// 알림 이벤트 타입 (13종).
#[derive(Debug, Clone)]
pub enum AlertEvent {
    /// 포지션 진입 체결.
    EntryExecuted {
        coin: String,
        qty: Decimal,
        upbit_price: Decimal,
        bybit_price: Decimal,
        expected_pnl: Decimal,
    },
    /// 포지션 청산 체결.
    ExitExecuted {
        coin: String,
        qty: Decimal,
        realized_pnl: Decimal,
    },
    /// Kill switch 발동.
    KillSwitchTriggered { reason: String, daily_pnl: Decimal },
    /// Kill switch 청산 완료.
    KillSwitchComplete {
        closed_count: usize,
        total_pnl: Decimal,
    },
    /// 한쪽 레그 실패.
    LegFailure {
        coin: String,
        succeeded_leg: String,
        failed_leg: String,
        action_taken: String,
    },
    /// 비상 청산 실패.
    EmergencyCloseFailure {
        coin: String,
        retry_count: u32,
        naked_exposure: Decimal,
    },
    /// Reconciliation 불일치.
    ReconciliationMismatch {
        coin: String,
        internal_qty: Decimal,
        exchange_qty: Decimal,
    },
    /// 거래소 연결 끊김.
    ConnectionLost {
        exchange: String,
        has_open_positions: bool,
    },
    /// 잔고 부족.
    BalanceInsufficient {
        exchange: String,
        required: Decimal,
        available: Decimal,
    },
    /// DB 연결 끊김.
    DbConnectionLost { retry_count: u32 },
    /// 펀딩비 진입 차단.
    FundingBlockEntry {
        coin: String,
        rate: f64,
        direction: String,
    },
    /// 일반 에러.
    Error { message: String },
    /// 일일 요약.
    DailySummary {
        trades: usize,
        pnl: Decimal,
        win_rate: f64,
    },
}

impl AlertEvent {
    /// 알림 심각도 레벨 ("info", "warn", "critical").
    pub fn level(&self) -> &str {
        match self {
            Self::EntryExecuted { .. } | Self::ExitExecuted { .. } | Self::DailySummary { .. } => {
                "info"
            }
            Self::ConnectionLost { .. }
            | Self::BalanceInsufficient { .. }
            | Self::DbConnectionLost { .. }
            | Self::FundingBlockEntry { .. }
            | Self::ReconciliationMismatch { .. }
            | Self::Error { .. } => "warn",
            Self::KillSwitchTriggered { .. }
            | Self::KillSwitchComplete { .. }
            | Self::LegFailure { .. }
            | Self::EmergencyCloseFailure { .. } => "critical",
        }
    }

    /// 알림이 치명적(critical)인지 확인합니다.
    pub fn is_critical(&self) -> bool {
        self.level() == "critical"
    }

    /// 이벤트 타입 문자열.
    pub fn event_type(&self) -> &str {
        match self {
            Self::EntryExecuted { .. } => "entry_executed",
            Self::ExitExecuted { .. } => "exit_executed",
            Self::KillSwitchTriggered { .. } => "kill_switch_triggered",
            Self::KillSwitchComplete { .. } => "kill_switch_complete",
            Self::LegFailure { .. } => "leg_failure",
            Self::EmergencyCloseFailure { .. } => "emergency_close_failure",
            Self::ReconciliationMismatch { .. } => "reconciliation_mismatch",
            Self::ConnectionLost { .. } => "connection_lost",
            Self::BalanceInsufficient { .. } => "balance_insufficient",
            Self::DbConnectionLost { .. } => "db_connection_lost",
            Self::FundingBlockEntry { .. } => "funding_block_entry",
            Self::Error { .. } => "error",
            Self::DailySummary { .. } => "daily_summary",
        }
    }
}

impl fmt::Display for AlertEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntryExecuted {
                coin,
                qty,
                upbit_price,
                bybit_price,
                expected_pnl,
            } => {
                write!(
                    f,
                    "\u{1f4c8} 진입: {coin} qty={qty} upbit={upbit_price} bybit={bybit_price} expected_pnl={expected_pnl}"
                )
            }
            Self::ExitExecuted {
                coin,
                qty,
                realized_pnl,
            } => {
                write!(f, "\u{1f4c9} 청산: {coin} qty={qty} pnl={realized_pnl}")
            }
            Self::KillSwitchTriggered { reason, daily_pnl } => {
                write!(f, "\u{1f6a8} KILL SWITCH: {reason} (daily_pnl={daily_pnl})")
            }
            Self::KillSwitchComplete {
                closed_count,
                total_pnl,
            } => {
                write!(
                    f,
                    "\u{2705} KILL SWITCH COMPLETE: {closed_count}건 청산, total_pnl={total_pnl}"
                )
            }
            Self::LegFailure {
                coin,
                succeeded_leg,
                failed_leg,
                action_taken,
            } => {
                write!(
                    f,
                    "\u{26a0}\u{fe0f} LEG FAILURE: {coin} succeeded={succeeded_leg} failed={failed_leg} action={action_taken}"
                )
            }
            Self::EmergencyCloseFailure {
                coin,
                retry_count,
                naked_exposure,
            } => {
                write!(
                    f,
                    "\u{1f525} EMERGENCY CLOSE FAILED: {coin} retries={retry_count} naked={naked_exposure}"
                )
            }
            Self::ReconciliationMismatch {
                coin,
                internal_qty,
                exchange_qty,
            } => {
                write!(
                    f,
                    "\u{26a0}\u{fe0f} RECONCILIATION: {coin} internal={internal_qty} exchange={exchange_qty}"
                )
            }
            Self::ConnectionLost {
                exchange,
                has_open_positions,
            } => {
                write!(
                    f,
                    "\u{1f50c} CONNECTION LOST: {exchange} open_positions={has_open_positions}"
                )
            }
            Self::BalanceInsufficient {
                exchange,
                required,
                available,
            } => {
                write!(
                    f,
                    "\u{1f4b0} BALANCE LOW: {exchange} required={required} available={available}"
                )
            }
            Self::DbConnectionLost { retry_count } => {
                write!(
                    f,
                    "\u{1f5c4}\u{fe0f} DB CONNECTION LOST: retries={retry_count}"
                )
            }
            Self::FundingBlockEntry {
                coin,
                rate,
                direction,
            } => {
                write!(
                    f,
                    "\u{1f4ca} FUNDING BLOCK: {coin} rate={rate:.4}% direction={direction}"
                )
            }
            Self::Error { message } => write!(f, "\u{274c} ERROR: {message}"),
            Self::DailySummary {
                trades,
                pnl,
                win_rate,
            } => {
                write!(
                    f,
                    "\u{1f4ca} DAILY: {trades} trades, pnl={pnl}, win_rate={win_rate:.1}%"
                )
            }
        }
    }
}

/// 텔레그램 전송 함수 타입.
pub type TelegramSendFn =
    Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> + Send + Sync>;

/// DB alert fallback 함수 타입.
pub type DbAlertFn = Box<
    dyn Fn(
            i64,
            &str,
            &str,
            &str,
            Option<String>,
        ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>
        + Send
        + Sync,
>;

/// AlertService: 텔레그램 알림 + DB 감사로그.
///
/// 일반 알림은 mpsc 비동기 채널(64 bounded)로 전송하여 이벤트 루프를 블로킹하지 않습니다.
/// 치명적 알림(kill switch, 비상 청산 실패)은 청산 완료 후 동기적으로 전송합니다.
#[derive(Clone)]
pub struct AlertService {
    /// 비동기 알림 채널 sender.
    tx: tokio::sync::mpsc::Sender<AlertEvent>,
    /// 현재 세션 ID.
    session_id: i64,
}

/// AlertService consumer handle (background task).
pub struct AlertConsumer {
    handle: tokio::task::JoinHandle<()>,
}

impl AlertConsumer {
    /// Consumer task 완료를 대기합니다.
    pub async fn shutdown(self) {
        let _ = self.handle.await;
    }
}

impl AlertService {
    /// AlertService를 생성하고 백그라운드 consumer task를 시작합니다.
    ///
    /// # 인자
    /// - `session_id`: 현재 트레이딩 세션 ID
    /// - `telegram_send_fn`: 텔레그램 전송 함수 (테스트에서 mock 가능)
    /// - `db_alert_fn`: DB alerts 테이블 기록 함수 (항상 호출)
    ///
    /// # 반환값
    /// `(AlertService, AlertConsumer)` 튜플. Consumer는 shutdown 대기용.
    pub fn new(
        session_id: i64,
        telegram_send_fn: impl Fn(String) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>
        + Send
        + Sync
        + 'static,
        db_alert_fn: impl Fn(
            i64,
            &str,
            &str,
            &str,
            Option<String>,
        ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>
        + Send
        + Sync
        + 'static,
    ) -> (Self, AlertConsumer) {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<AlertEvent>(64);

        let handle = tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                let message = event.to_string();
                let level = event.level().to_string();
                let event_type = event.event_type().to_string();
                let payload = format!("{:?}", event);

                let mut db_ok = false;
                let mut tg_ok = false;

                // DB 감사로그는 항상 기록
                match (db_alert_fn)(session_id, &level, &event_type, &message, Some(payload)).await
                {
                    Ok(()) => {
                        db_ok = true;
                    }
                    Err(db_err) => {
                        tracing::error!(
                            error = db_err.as_str(),
                            event_type = event_type.as_str(),
                            "alerts DB 기록 실패"
                        );
                    }
                }

                // 텔레그램은 best-effort 전송
                match (telegram_send_fn)(message.clone()).await {
                    Ok(()) => {
                        tg_ok = true;
                        tracing::debug!(
                            event_type = event_type.as_str(),
                            "텔레그램 알림 전송 완료"
                        );
                    }
                    Err(tg_err) => {
                        tracing::warn!(
                            error = tg_err.as_str(),
                            event_type = event_type.as_str(),
                            "텔레그램 전송 실패"
                        );
                    }
                }

                if !db_ok && !tg_ok {
                    tracing::error!(
                        event_type = event_type.as_str(),
                        "알림 DB + 텔레그램 모두 실패"
                    );
                }
            }
            tracing::info!("AlertService consumer 종료");
        });

        (Self { tx, session_id }, AlertConsumer { handle })
    }

    /// 비동기 알림을 전송합니다 (try_send, 이벤트 루프 비블로킹).
    pub fn send(&self, event: AlertEvent) {
        if let Err(e) = self.tx.try_send(event) {
            tracing::warn!(error = %e, "알림 채널 가득 참, 이벤트 드랍");
        }
    }

    /// 세션 ID를 반환합니다.
    pub fn session_id(&self) -> i64 {
        self.session_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    };

    fn noop_telegram() -> impl Fn(String) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>
    + Send
    + Sync
    + 'static {
        |_msg: String| Box::pin(async { Ok(()) })
    }

    #[allow(clippy::type_complexity)]
    fn noop_db_alert() -> impl Fn(
        i64,
        &str,
        &str,
        &str,
        Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>
    + Send
    + Sync
    + 'static {
        |_sid, _level, _evt, _msg, _payload| Box::pin(async { Ok(()) })
    }

    #[tokio::test]
    async fn test_alert_event_levels() {
        // info 레벨
        assert_eq!(
            AlertEvent::EntryExecuted {
                coin: "BTC".into(),
                qty: Decimal::ONE,
                upbit_price: Decimal::ONE,
                bybit_price: Decimal::ONE,
                expected_pnl: Decimal::ONE,
            }
            .level(),
            "info"
        );
        assert_eq!(
            AlertEvent::ExitExecuted {
                coin: "BTC".into(),
                qty: Decimal::ONE,
                realized_pnl: Decimal::ONE,
            }
            .level(),
            "info"
        );
        assert_eq!(
            AlertEvent::DailySummary {
                trades: 10,
                pnl: Decimal::ONE,
                win_rate: 50.0,
            }
            .level(),
            "info"
        );

        // warn 레벨
        assert_eq!(
            AlertEvent::ConnectionLost {
                exchange: "upbit".into(),
                has_open_positions: true,
            }
            .level(),
            "warn"
        );
        assert_eq!(
            AlertEvent::BalanceInsufficient {
                exchange: "bybit".into(),
                required: Decimal::from(100u64),
                available: Decimal::from(50u64),
            }
            .level(),
            "warn"
        );
        assert_eq!(
            AlertEvent::DbConnectionLost { retry_count: 3 }.level(),
            "warn"
        );
        assert_eq!(
            AlertEvent::FundingBlockEntry {
                coin: "BTC".into(),
                rate: 0.01,
                direction: "short_pays".into(),
            }
            .level(),
            "warn"
        );
        assert_eq!(
            AlertEvent::ReconciliationMismatch {
                coin: "BTC".into(),
                internal_qty: Decimal::ONE,
                exchange_qty: Decimal::from(2u64),
            }
            .level(),
            "warn"
        );
        assert_eq!(
            AlertEvent::Error {
                message: "test".into(),
            }
            .level(),
            "warn"
        );

        // critical 레벨
        assert_eq!(
            AlertEvent::KillSwitchTriggered {
                reason: "test".into(),
                daily_pnl: Decimal::ZERO,
            }
            .level(),
            "critical"
        );
        assert!(
            AlertEvent::KillSwitchTriggered {
                reason: "test".into(),
                daily_pnl: Decimal::ZERO,
            }
            .is_critical()
        );
        assert_eq!(
            AlertEvent::KillSwitchComplete {
                closed_count: 3,
                total_pnl: Decimal::ZERO,
            }
            .level(),
            "critical"
        );
        assert_eq!(
            AlertEvent::LegFailure {
                coin: "BTC".into(),
                succeeded_leg: "upbit".into(),
                failed_leg: "bybit".into(),
                action_taken: "reverse".into(),
            }
            .level(),
            "critical"
        );
        assert_eq!(
            AlertEvent::EmergencyCloseFailure {
                coin: "BTC".into(),
                retry_count: 5,
                naked_exposure: Decimal::from(1000u64),
            }
            .level(),
            "critical"
        );
    }

    #[tokio::test]
    async fn test_alert_event_is_critical() {
        // info는 critical이 아님
        assert!(
            !AlertEvent::EntryExecuted {
                coin: "BTC".into(),
                qty: Decimal::ONE,
                upbit_price: Decimal::ONE,
                bybit_price: Decimal::ONE,
                expected_pnl: Decimal::ONE,
            }
            .is_critical()
        );

        // warn은 critical이 아님
        assert!(
            !AlertEvent::ConnectionLost {
                exchange: "upbit".into(),
                has_open_positions: false,
            }
            .is_critical()
        );

        // critical은 critical
        assert!(
            AlertEvent::EmergencyCloseFailure {
                coin: "ETH".into(),
                retry_count: 3,
                naked_exposure: Decimal::from(500u64),
            }
            .is_critical()
        );
    }

    #[tokio::test]
    async fn test_alert_event_display() {
        let event = AlertEvent::EntryExecuted {
            coin: "BTC".into(),
            qty: Decimal::from(1u64),
            upbit_price: Decimal::from(50000u64),
            bybit_price: Decimal::from(49000u64),
            expected_pnl: Decimal::from(10u64),
        };
        let s = event.to_string();
        assert!(s.contains("BTC"));
        assert!(s.contains("50000"));
        assert!(s.contains("49000"));

        let exit = AlertEvent::ExitExecuted {
            coin: "ETH".into(),
            qty: Decimal::from(5u64),
            realized_pnl: Decimal::from(25u64),
        };
        assert!(exit.to_string().contains("ETH"));
        assert!(exit.to_string().contains("25"));

        let kill = AlertEvent::KillSwitchTriggered {
            reason: "daily loss limit".into(),
            daily_pnl: Decimal::from(-100i64),
        };
        assert!(kill.to_string().contains("KILL SWITCH"));
        assert!(kill.to_string().contains("daily loss limit"));
    }

    #[tokio::test]
    async fn test_alert_event_display_all_variants() {
        // 모든 variant의 Display가 panic 없이 동작하는지 확인
        let events: Vec<AlertEvent> = vec![
            AlertEvent::EntryExecuted {
                coin: "BTC".into(),
                qty: Decimal::ONE,
                upbit_price: Decimal::ONE,
                bybit_price: Decimal::ONE,
                expected_pnl: Decimal::ONE,
            },
            AlertEvent::ExitExecuted {
                coin: "BTC".into(),
                qty: Decimal::ONE,
                realized_pnl: Decimal::ONE,
            },
            AlertEvent::KillSwitchTriggered {
                reason: "test".into(),
                daily_pnl: Decimal::ZERO,
            },
            AlertEvent::KillSwitchComplete {
                closed_count: 1,
                total_pnl: Decimal::ZERO,
            },
            AlertEvent::LegFailure {
                coin: "BTC".into(),
                succeeded_leg: "upbit".into(),
                failed_leg: "bybit".into(),
                action_taken: "reverse".into(),
            },
            AlertEvent::EmergencyCloseFailure {
                coin: "BTC".into(),
                retry_count: 3,
                naked_exposure: Decimal::from(1000u64),
            },
            AlertEvent::ReconciliationMismatch {
                coin: "BTC".into(),
                internal_qty: Decimal::ONE,
                exchange_qty: Decimal::from(2u64),
            },
            AlertEvent::ConnectionLost {
                exchange: "upbit".into(),
                has_open_positions: true,
            },
            AlertEvent::BalanceInsufficient {
                exchange: "bybit".into(),
                required: Decimal::from(100u64),
                available: Decimal::from(50u64),
            },
            AlertEvent::DbConnectionLost { retry_count: 5 },
            AlertEvent::FundingBlockEntry {
                coin: "BTC".into(),
                rate: 0.0123,
                direction: "short_pays".into(),
            },
            AlertEvent::Error {
                message: "test error".into(),
            },
            AlertEvent::DailySummary {
                trades: 42,
                pnl: Decimal::from(100u64),
                win_rate: 65.5,
            },
        ];

        for event in &events {
            let s = event.to_string();
            assert!(!s.is_empty(), "Display for {:?} should not be empty", event);
        }
    }

    #[tokio::test]
    async fn test_alert_event_types() {
        assert_eq!(
            AlertEvent::EntryExecuted {
                coin: "BTC".into(),
                qty: Decimal::ONE,
                upbit_price: Decimal::ONE,
                bybit_price: Decimal::ONE,
                expected_pnl: Decimal::ONE,
            }
            .event_type(),
            "entry_executed"
        );
        assert_eq!(
            AlertEvent::ExitExecuted {
                coin: "BTC".into(),
                qty: Decimal::ONE,
                realized_pnl: Decimal::ONE,
            }
            .event_type(),
            "exit_executed"
        );
        assert_eq!(
            AlertEvent::KillSwitchTriggered {
                reason: "".into(),
                daily_pnl: Decimal::ZERO,
            }
            .event_type(),
            "kill_switch_triggered"
        );
        assert_eq!(
            AlertEvent::KillSwitchComplete {
                closed_count: 0,
                total_pnl: Decimal::ZERO,
            }
            .event_type(),
            "kill_switch_complete"
        );
        assert_eq!(
            AlertEvent::LegFailure {
                coin: "".into(),
                succeeded_leg: "".into(),
                failed_leg: "".into(),
                action_taken: "".into(),
            }
            .event_type(),
            "leg_failure"
        );
        assert_eq!(
            AlertEvent::EmergencyCloseFailure {
                coin: "".into(),
                retry_count: 0,
                naked_exposure: Decimal::ZERO,
            }
            .event_type(),
            "emergency_close_failure"
        );
        assert_eq!(
            AlertEvent::ReconciliationMismatch {
                coin: "".into(),
                internal_qty: Decimal::ZERO,
                exchange_qty: Decimal::ZERO,
            }
            .event_type(),
            "reconciliation_mismatch"
        );
        assert_eq!(
            AlertEvent::ConnectionLost {
                exchange: "".into(),
                has_open_positions: false,
            }
            .event_type(),
            "connection_lost"
        );
        assert_eq!(
            AlertEvent::BalanceInsufficient {
                exchange: "".into(),
                required: Decimal::ZERO,
                available: Decimal::ZERO,
            }
            .event_type(),
            "balance_insufficient"
        );
        assert_eq!(
            AlertEvent::DbConnectionLost { retry_count: 0 }.event_type(),
            "db_connection_lost"
        );
        assert_eq!(
            AlertEvent::FundingBlockEntry {
                coin: "BTC".into(),
                rate: 0.01,
                direction: "short_pays".into(),
            }
            .event_type(),
            "funding_block_entry"
        );
        assert_eq!(
            AlertEvent::Error { message: "".into() }.event_type(),
            "error"
        );
        assert_eq!(
            AlertEvent::DailySummary {
                trades: 0,
                pnl: Decimal::ZERO,
                win_rate: 0.0,
            }
            .event_type(),
            "daily_summary"
        );
    }

    #[tokio::test]
    async fn test_alert_service_send() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let telegram_fn =
            move |_msg: String| -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
                let c = counter_clone.clone();
                Box::pin(async move {
                    c.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                })
            };

        let (service, consumer) = AlertService::new(1, telegram_fn, noop_db_alert());

        service.send(AlertEvent::EntryExecuted {
            coin: "BTC".into(),
            qty: Decimal::ONE,
            upbit_price: Decimal::ONE,
            bybit_price: Decimal::ONE,
            expected_pnl: Decimal::ONE,
        });

        // tx 드랍 → consumer 종료
        drop(service);
        consumer.shutdown().await;

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_alert_service_db_always_written() {
        let db_counter = Arc::new(AtomicU32::new(0));
        let db_counter_clone = db_counter.clone();

        let db_fn = move |_sid: i64,
                          _level: &str,
                          _evt: &str,
                          _msg: &str,
                          _payload: Option<String>|
              -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
            let c = db_counter_clone.clone();
            Box::pin(async move {
                c.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
        };

        let (service, consumer) = AlertService::new(1, noop_telegram(), db_fn);

        service.send(AlertEvent::Error {
            message: "test".into(),
        });

        drop(service);
        consumer.shutdown().await;

        assert_eq!(db_counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_alert_service_multiple_sends() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let telegram_fn =
            move |_msg: String| -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
                let c = counter_clone.clone();
                Box::pin(async move {
                    c.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                })
            };

        let (service, consumer) = AlertService::new(1, telegram_fn, noop_db_alert());

        for _ in 0..5 {
            service.send(AlertEvent::Error {
                message: "test".into(),
            });
        }

        drop(service);
        consumer.shutdown().await;

        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }

    #[tokio::test]
    async fn test_alert_service_telegram_failure_db_fallback() {
        let db_counter = Arc::new(AtomicU32::new(0));
        let db_counter_clone = db_counter.clone();

        let fail_telegram =
            |_msg: String| -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
                Box::pin(async { Err("telegram offline".to_string()) })
            };

        let db_fn = move |_sid: i64,
                          _level: &str,
                          _evt: &str,
                          _msg: &str,
                          _payload: Option<String>|
              -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
            let c = db_counter_clone.clone();
            Box::pin(async move {
                c.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
        };

        let (service, consumer) = AlertService::new(1, fail_telegram, db_fn);

        service.send(AlertEvent::Error {
            message: "test error".into(),
        });

        drop(service);
        consumer.shutdown().await;

        assert_eq!(db_counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_alert_service_both_telegram_and_db_failure() {
        // 텔레그램 + DB 모두 실패 시 triple failure (로그만 남김, panic 없음)
        let fail_telegram =
            |_msg: String| -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
                Box::pin(async { Err("telegram offline".to_string()) })
            };

        let fail_db = |_sid: i64,
                       _level: &str,
                       _evt: &str,
                       _msg: &str,
                       _payload: Option<String>|
         -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
            Box::pin(async { Err("db offline".to_string()) })
        };

        let (service, consumer) = AlertService::new(1, fail_telegram, fail_db);

        service.send(AlertEvent::KillSwitchTriggered {
            reason: "daily loss".into(),
            daily_pnl: Decimal::from(-500i64),
        });

        drop(service);
        // triple failure 시에도 panic 없이 정상 종료되어야 함
        consumer.shutdown().await;
    }

    #[tokio::test]
    async fn test_alert_service_session_id() {
        let (service, _consumer) = AlertService::new(42, noop_telegram(), noop_db_alert());
        assert_eq!(service.session_id(), 42);
    }
}
