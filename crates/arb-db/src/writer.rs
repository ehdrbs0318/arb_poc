//! Background DB Writer.
//!
//! mpsc 채널을 통해 DB 쓰기 요청을 수신하고 단일 consumer task에서 처리.
//! 전체 직렬 순서 보장 (단일 consumer).

use crate::alerts::{AlertRecord, AlertRepository};
use crate::balance_snapshots::{BalanceSnapshotRepository, BalanceSnapshotRow};
use crate::error::DbError;
use crate::funding::{FundingRepository, FundingScheduleRecord};
use crate::minutes::{MinuteRecord, MinuteRepository};
use crate::positions::{DbPositionStore, PositionRecord, PositionStore, UpdateFields};
use crate::sessions::SessionRepository;
use crate::trades::{TradeRecord, TradeRepository};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::mpsc;
use tracing::{debug, error, warn};

/// DB 쓰기 요청 타입.
#[derive(Debug)]
pub enum DbWriteRequest {
    /// 포지션 INSERT.
    InsertPosition(PositionRecord),
    /// 포지션 상태 전이.
    UpdatePositionState {
        id: i64,
        from: String,
        to: String,
        fields: UpdateFields,
    },
    /// 거래 기록 INSERT.
    InsertTrade(TradeRecord),
    /// 분봉 데이터 INSERT.
    InsertMinute(MinuteRecord),
    /// 알림 INSERT.
    InsertAlert(AlertRecord),
    /// 펀딩 스케줄 UPSERT.
    UpsertFunding(FundingScheduleRecord),
    /// 잔고 스냅샷 INSERT.
    InsertBalanceSnapshot(BalanceSnapshotRow),
    /// 세션 상태 UPDATE.
    UpdateSession { id: i64, status: String },
    /// 세션 heartbeat UPDATE.
    Heartbeat { session_id: i64 },
}

/// Background DB Writer 통계.
#[derive(Debug)]
pub struct DbWriterStats {
    /// 오버플로우 (드랍된 요청 수).
    pub overflow_count: AtomicU64,
    /// 최종 실패 (재시도 초과) 수.
    pub final_failure_count: AtomicU64,
    /// 처리 완료 수.
    pub processed_count: AtomicU64,
}

impl Default for DbWriterStats {
    fn default() -> Self {
        Self {
            overflow_count: AtomicU64::new(0),
            processed_count: AtomicU64::new(0),
            final_failure_count: AtomicU64::new(0),
        }
    }
}

/// Background DB Writer.
///
/// mpsc::channel(256) bounded 채널로 요청 수신.
/// 채널 full 시 요청 중요도에 따라 drop 또는 재대기 전송.
#[derive(Debug, Clone)]
pub struct DbWriter {
    tx: mpsc::Sender<DbWriteRequest>,
    stats: Arc<DbWriterStats>,
}

impl DbWriter {
    /// 새 DbWriter 생성 및 consumer task 시작.
    ///
    /// # 인자
    ///
    /// * `session_repo` - 세션 Repository
    /// * `position_store` - 포지션 Store
    /// * `trade_repo` - 거래 Repository
    /// * `minute_repo` - 분봉 Repository
    /// * `alert_repo` - 알림 Repository
    /// * `funding_repo` - 펀딩 Repository
    /// * `balance_snapshot_repo` - 잔고 스냅샷 Repository
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        session_repo: SessionRepository,
        position_store: DbPositionStore,
        trade_repo: TradeRepository,
        minute_repo: MinuteRepository,
        alert_repo: AlertRepository,
        funding_repo: FundingRepository,
        balance_snapshot_repo: BalanceSnapshotRepository,
    ) -> Self {
        let (tx, rx) = mpsc::channel(256);
        let stats = Arc::new(DbWriterStats::default());

        let consumer_stats = Arc::clone(&stats);
        tokio::spawn(async move {
            run_consumer(
                rx,
                session_repo,
                position_store,
                trade_repo,
                minute_repo,
                alert_repo,
                funding_repo,
                balance_snapshot_repo,
                consumer_stats,
            )
            .await;
        });

        Self { tx, stats }
    }

    /// DB 쓰기 요청 전송.
    ///
    /// 채널이 가득 찬 경우:
    /// - 중요 요청(trades/alerts/positions/session): async send로 재대기
    /// - 비중요 요청(minutes/funding/snapshot/heartbeat): newest drop
    pub fn send(&self, request: DbWriteRequest) {
        match self.tx.try_send(request) {
            Ok(()) => {}
            Err(mpsc::error::TrySendError::Full(req)) => {
                let overflow_count = self.stats.overflow_count.fetch_add(1, Ordering::Relaxed) + 1;

                if is_critical_request(&req) {
                    if should_log_overflow(overflow_count) {
                        warn!(
                            overflow_count = overflow_count,
                            request_type = describe_request(&req),
                            "DB writer channel full, queueing critical request with async send"
                        );
                    }
                    let tx = self.tx.clone();
                    let stats = Arc::clone(&self.stats);
                    tokio::spawn(async move {
                        if tx.send(req).await.is_err() {
                            stats.final_failure_count.fetch_add(1, Ordering::Relaxed);
                            error!("DB writer channel closed while sending critical request");
                        }
                    });
                } else if should_log_overflow(overflow_count) {
                    warn!(
                        overflow_count = overflow_count,
                        request_type = describe_request(&req),
                        "DB writer channel full, dropping newest request"
                    );
                }
            }
            Err(mpsc::error::TrySendError::Closed(req)) => {
                self.stats
                    .final_failure_count
                    .fetch_add(1, Ordering::Relaxed);
                error!(
                    request_type = describe_request(&req),
                    "DB writer channel closed, request dropped"
                );
            }
        }
    }

    /// 오버플로우 카운트 조회.
    pub fn overflow_count(&self) -> u64 {
        self.stats.overflow_count.load(Ordering::Relaxed)
    }

    /// 처리 완료 카운트 조회.
    pub fn processed_count(&self) -> u64 {
        self.stats.processed_count.load(Ordering::Relaxed)
    }

    /// 최종 실패 카운트 조회.
    pub fn final_failure_count(&self) -> u64 {
        self.stats.final_failure_count.load(Ordering::Relaxed)
    }
}

/// 요청 타입 문자열 반환 (로깅용).
fn describe_request(req: &DbWriteRequest) -> &'static str {
    match req {
        DbWriteRequest::InsertPosition(_) => "InsertPosition",
        DbWriteRequest::UpdatePositionState { .. } => "UpdatePositionState",
        DbWriteRequest::InsertTrade(_) => "InsertTrade",
        DbWriteRequest::InsertMinute(_) => "InsertMinute",
        DbWriteRequest::InsertAlert(_) => "InsertAlert",
        DbWriteRequest::UpsertFunding(_) => "UpsertFunding",
        DbWriteRequest::InsertBalanceSnapshot(_) => "InsertBalanceSnapshot",
        DbWriteRequest::UpdateSession { .. } => "UpdateSession",
        DbWriteRequest::Heartbeat { .. } => "Heartbeat",
    }
}

/// 채널 full 시 재대기해야 하는 중요 요청 여부.
fn is_critical_request(req: &DbWriteRequest) -> bool {
    matches!(
        req,
        DbWriteRequest::InsertPosition(_)
            | DbWriteRequest::UpdatePositionState { .. }
            | DbWriteRequest::InsertTrade(_)
            | DbWriteRequest::InsertAlert(_)
            | DbWriteRequest::UpdateSession { .. }
    )
}

/// overflow 로그 스팸 방지를 위한 샘플링 정책.
fn should_log_overflow(overflow_count: u64) -> bool {
    overflow_count <= 10 || overflow_count.is_multiple_of(100)
}

/// 최대 재시도 횟수.
const MAX_RETRIES: u32 = 3;

/// Consumer task: 채널에서 요청을 받아 순차 처리.
#[allow(clippy::too_many_arguments)]
async fn run_consumer(
    mut rx: mpsc::Receiver<DbWriteRequest>,
    session_repo: SessionRepository,
    position_store: DbPositionStore,
    trade_repo: TradeRepository,
    minute_repo: MinuteRepository,
    alert_repo: AlertRepository,
    funding_repo: FundingRepository,
    balance_snapshot_repo: BalanceSnapshotRepository,
    stats: Arc<DbWriterStats>,
) {
    debug!("DB writer consumer task 시작");

    while let Some(request) = rx.recv().await {
        let request_type = describe_request(&request);
        let mut last_err: Option<DbError> = None;

        for attempt in 1..=MAX_RETRIES {
            let result = execute_request(
                &request,
                &session_repo,
                &position_store,
                &trade_repo,
                &minute_repo,
                &alert_repo,
                &funding_repo,
                &balance_snapshot_repo,
            )
            .await;

            match result {
                Ok(()) => {
                    stats.processed_count.fetch_add(1, Ordering::Relaxed);
                    last_err = None;
                    break;
                }
                Err(e) => {
                    warn!(
                        request_type = request_type,
                        attempt = attempt,
                        error = %e,
                        "DB write failed, retrying"
                    );
                    last_err = Some(e);
                    // 재시도 전 짧은 대기
                    tokio::time::sleep(std::time::Duration::from_millis(100 * attempt as u64))
                        .await;
                }
            }
        }

        if let Some(e) = last_err {
            stats.final_failure_count.fetch_add(1, Ordering::Relaxed);
            error!(
                request_type = request_type,
                error = %e,
                "DB write failed after {} retries, giving up",
                MAX_RETRIES
            );
        }
    }

    debug!("DB writer consumer task 종료");
}

/// 개별 요청 실행.
#[allow(clippy::too_many_arguments)]
async fn execute_request(
    request: &DbWriteRequest,
    session_repo: &SessionRepository,
    position_store: &DbPositionStore,
    trade_repo: &TradeRepository,
    minute_repo: &MinuteRepository,
    alert_repo: &AlertRepository,
    funding_repo: &FundingRepository,
    balance_snapshot_repo: &BalanceSnapshotRepository,
) -> Result<(), DbError> {
    match request {
        DbWriteRequest::InsertPosition(pos) => {
            position_store.save(pos).await?;
        }
        DbWriteRequest::UpdatePositionState {
            id,
            from,
            to,
            fields,
        } => {
            position_store
                .update_state(*id, from, to, fields.clone())
                .await?;
        }
        DbWriteRequest::InsertTrade(trade) => {
            trade_repo.insert_trade(trade).await?;
        }
        DbWriteRequest::InsertMinute(minute) => {
            minute_repo.insert_minute(minute).await?;
        }
        DbWriteRequest::InsertAlert(alert) => {
            alert_repo.insert_alert(alert).await?;
        }
        DbWriteRequest::UpsertFunding(funding) => {
            funding_repo.upsert_funding(funding).await?;
        }
        DbWriteRequest::InsertBalanceSnapshot(row) => {
            balance_snapshot_repo.insert_snapshot(row).await?;
        }
        DbWriteRequest::UpdateSession { id, status } => {
            session_repo.end_session(*id, status).await?;
        }
        DbWriteRequest::Heartbeat { session_id } => {
            session_repo.update_heartbeat(*session_id).await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_request() {
        let req = DbWriteRequest::Heartbeat { session_id: 1 };
        assert_eq!(describe_request(&req), "Heartbeat");

        let req = DbWriteRequest::UpdateSession {
            id: 1,
            status: "Completed".to_string(),
        };
        assert_eq!(describe_request(&req), "UpdateSession");
    }

    #[test]
    fn test_describe_request_insert_balance_snapshot() {
        use crate::balance_snapshots::BalanceSnapshotRow;
        use chrono::Utc;
        use rust_decimal::Decimal;

        let row = BalanceSnapshotRow {
            created_at: Utc::now(),
            snapshot_group_id: 1,
            session_id: 1,
            record_type: "PERIODIC".to_string(),
            cex: "UPBIT".to_string(),
            currency: "KRW".to_string(),
            available: Decimal::ZERO,
            locked: Decimal::ZERO,
            coin_value: Decimal::ZERO,
            total: Decimal::ZERO,
            position_id: None,
            usd_krw: 1350.0,
            usdt_krw: 1380.0,
            total_usd: Decimal::ZERO,
            total_usdt: Decimal::ZERO,
        };
        let req = DbWriteRequest::InsertBalanceSnapshot(row);
        assert_eq!(describe_request(&req), "InsertBalanceSnapshot");
    }

    #[test]
    fn test_db_writer_stats_default() {
        let stats = DbWriterStats::default();
        assert_eq!(stats.overflow_count.load(Ordering::Relaxed), 0);
        assert_eq!(stats.processed_count.load(Ordering::Relaxed), 0);
        assert_eq!(stats.final_failure_count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_is_critical_request() {
        let critical = DbWriteRequest::UpdateSession {
            id: 1,
            status: "Completed".to_string(),
        };
        assert!(is_critical_request(&critical));

        let non_critical = DbWriteRequest::Heartbeat { session_id: 1 };
        assert!(!is_critical_request(&non_critical));
    }
}
