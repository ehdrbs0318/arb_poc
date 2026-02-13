//! positions 테이블 Repository 및 PositionStore trait.
//!
//! 포지션 상태 머신의 영속화를 담당. 낙관적 잠금 기반 상태 전이.

use crate::error::DbError;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{MySqlPool, Row};
use std::future::Future;
use tracing::debug;

/// 포지션 상태 전이 결과.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionResult {
    /// 전이 성공.
    Applied,
    /// 이미 다른 상태로 전이됨 (현재 상태 반환).
    AlreadyTransitioned(String),
}

/// 포지션 DB 레코드.
#[derive(Debug, Clone)]
pub struct PositionRecord {
    pub id: Option<i64>,
    pub session_id: i64,
    pub coin: String,
    pub state: String,
    pub upbit_qty: Decimal,
    pub bybit_qty: Decimal,
    pub upbit_entry_price: Option<Decimal>,
    pub bybit_entry_price: Option<Decimal>,
    pub upbit_order_id: Option<String>,
    pub bybit_order_id: Option<String>,
    pub entry_spread_pct: Option<f64>,
    pub entry_z_score: Option<f64>,
    pub entry_usd_krw: Option<f64>,
    pub opened_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub realized_pnl: Option<Decimal>,
    pub exit_upbit_order_id: Option<String>,
    pub exit_bybit_order_id: Option<String>,
    pub client_order_id: Option<String>,
    pub exit_client_order_id: Option<String>,
    pub in_flight: bool,
    pub succeeded_leg: Option<String>,
    pub emergency_attempts: i32,
}

/// 포지션 업데이트 필드 (부분 업데이트용).
#[derive(Debug, Clone, Default)]
pub struct UpdateFields {
    pub upbit_order_id: Option<String>,
    pub bybit_order_id: Option<String>,
    pub upbit_qty: Option<Decimal>,
    pub bybit_qty: Option<Decimal>,
    pub upbit_entry_price: Option<Decimal>,
    pub bybit_entry_price: Option<Decimal>,
    pub entry_spread_pct: Option<f64>,
    pub entry_z_score: Option<f64>,
    pub entry_usd_krw: Option<f64>,
    pub opened_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub realized_pnl: Option<Decimal>,
    pub exit_upbit_order_id: Option<String>,
    pub exit_bybit_order_id: Option<String>,
    pub exit_client_order_id: Option<String>,
    pub in_flight: Option<bool>,
    pub succeeded_leg: Option<String>,
    pub emergency_attempts: Option<i32>,
}

/// 포지션 영속화 trait.
///
/// DB 구현체와 테스트용 mock 구현체를 분리하기 위한 추상화.
/// RPITIT 패턴 사용 (프로젝트 관례).
pub trait PositionStore: Send + Sync {
    /// 포지션 INSERT (Opening 상태). 생성된 ID 반환.
    fn save(&self, pos: &PositionRecord) -> impl Future<Output = Result<i64, DbError>> + Send;

    /// 포지션 상태 전이 (낙관적 잠금: WHERE state = from).
    fn update_state(
        &self,
        id: i64,
        from: &str,
        to: &str,
        fields: UpdateFields,
    ) -> impl Future<Output = Result<TransitionResult, DbError>> + Send;

    /// 특정 세션의 non-Closed 포지션 조회 (crash recovery용).
    fn load_open(
        &self,
        session_id: i64,
    ) -> impl Future<Output = Result<Vec<PositionRecord>, DbError>> + Send;

    /// 포지션 삭제 (Opening 미발주 건).
    fn remove(&self, id: i64) -> impl Future<Output = Result<(), DbError>> + Send;
}

/// MySQL 기반 PositionStore 구현.
#[derive(Debug, Clone)]
pub struct DbPositionStore {
    pool: MySqlPool,
}

impl DbPositionStore {
    /// 새 DbPositionStore 생성.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl PositionStore for DbPositionStore {
    async fn save(&self, pos: &PositionRecord) -> Result<i64, DbError> {
        debug!(
            session_id = pos.session_id,
            coin = %pos.coin,
            state = %pos.state,
            upbit_qty = %pos.upbit_qty,
            bybit_qty = %pos.bybit_qty,
            client_order_id = ?pos.client_order_id,
            "포지션 INSERT"
        );

        let result = sqlx::query(
            r#"
            INSERT INTO positions (
                session_id, coin, state, upbit_qty, bybit_qty,
                upbit_entry_price, bybit_entry_price,
                upbit_order_id, bybit_order_id,
                entry_spread_pct, entry_z_score, entry_usd_krw,
                opened_at, closed_at, realized_pnl,
                exit_upbit_order_id, exit_bybit_order_id,
                client_order_id, exit_client_order_id,
                in_flight, succeeded_leg, emergency_attempts
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(pos.session_id)
        .bind(&pos.coin)
        .bind(&pos.state)
        .bind(pos.upbit_qty)
        .bind(pos.bybit_qty)
        .bind(pos.upbit_entry_price)
        .bind(pos.bybit_entry_price)
        .bind(&pos.upbit_order_id)
        .bind(&pos.bybit_order_id)
        .bind(pos.entry_spread_pct)
        .bind(pos.entry_z_score)
        .bind(pos.entry_usd_krw)
        .bind(pos.opened_at)
        .bind(pos.closed_at)
        .bind(pos.realized_pnl)
        .bind(&pos.exit_upbit_order_id)
        .bind(&pos.exit_bybit_order_id)
        .bind(&pos.client_order_id)
        .bind(&pos.exit_client_order_id)
        .bind(pos.in_flight)
        .bind(&pos.succeeded_leg)
        .bind(pos.emergency_attempts)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        debug!(position_id = id, "포지션 INSERT 완료");
        Ok(id)
    }

    async fn update_state(
        &self,
        id: i64,
        from: &str,
        to: &str,
        fields: UpdateFields,
    ) -> Result<TransitionResult, DbError> {
        debug!(
            position_id = id,
            from = from,
            to = to,
            "포지션 상태 전이 시도"
        );

        // 동적 SET 절 구성
        let mut set_parts = vec!["state = ?".to_string()];
        if fields.upbit_order_id.is_some() {
            set_parts.push("upbit_order_id = ?".to_string());
        }
        if fields.bybit_order_id.is_some() {
            set_parts.push("bybit_order_id = ?".to_string());
        }
        if fields.upbit_qty.is_some() {
            set_parts.push("upbit_qty = ?".to_string());
        }
        if fields.bybit_qty.is_some() {
            set_parts.push("bybit_qty = ?".to_string());
        }
        if fields.upbit_entry_price.is_some() {
            set_parts.push("upbit_entry_price = ?".to_string());
        }
        if fields.bybit_entry_price.is_some() {
            set_parts.push("bybit_entry_price = ?".to_string());
        }
        if fields.entry_spread_pct.is_some() {
            set_parts.push("entry_spread_pct = ?".to_string());
        }
        if fields.entry_z_score.is_some() {
            set_parts.push("entry_z_score = ?".to_string());
        }
        if fields.entry_usd_krw.is_some() {
            set_parts.push("entry_usd_krw = ?".to_string());
        }
        if fields.opened_at.is_some() {
            set_parts.push("opened_at = ?".to_string());
        }
        if fields.closed_at.is_some() {
            set_parts.push("closed_at = ?".to_string());
        }
        if fields.realized_pnl.is_some() {
            set_parts.push("realized_pnl = ?".to_string());
        }
        if fields.exit_upbit_order_id.is_some() {
            set_parts.push("exit_upbit_order_id = ?".to_string());
        }
        if fields.exit_bybit_order_id.is_some() {
            set_parts.push("exit_bybit_order_id = ?".to_string());
        }
        if fields.exit_client_order_id.is_some() {
            set_parts.push("exit_client_order_id = ?".to_string());
        }
        if fields.in_flight.is_some() {
            set_parts.push("in_flight = ?".to_string());
        }
        if fields.succeeded_leg.is_some() {
            set_parts.push("succeeded_leg = ?".to_string());
        }
        if fields.emergency_attempts.is_some() {
            set_parts.push("emergency_attempts = ?".to_string());
        }

        let sql = format!(
            "UPDATE positions SET {} WHERE id = ? AND state = ?",
            set_parts.join(", ")
        );

        let mut query = sqlx::query(&sql);

        // state (to) 바인딩
        query = query.bind(to);

        // 동적 필드 바인딩 (SET 절 추가 순서와 동일하게)
        if let Some(ref v) = fields.upbit_order_id {
            query = query.bind(v);
        }
        if let Some(ref v) = fields.bybit_order_id {
            query = query.bind(v);
        }
        if let Some(v) = fields.upbit_qty {
            query = query.bind(v);
        }
        if let Some(v) = fields.bybit_qty {
            query = query.bind(v);
        }
        if let Some(v) = fields.upbit_entry_price {
            query = query.bind(v);
        }
        if let Some(v) = fields.bybit_entry_price {
            query = query.bind(v);
        }
        if let Some(v) = fields.entry_spread_pct {
            query = query.bind(v);
        }
        if let Some(v) = fields.entry_z_score {
            query = query.bind(v);
        }
        if let Some(v) = fields.entry_usd_krw {
            query = query.bind(v);
        }
        if let Some(v) = fields.opened_at {
            query = query.bind(v);
        }
        if let Some(v) = fields.closed_at {
            query = query.bind(v);
        }
        if let Some(v) = fields.realized_pnl {
            query = query.bind(v);
        }
        if let Some(ref v) = fields.exit_upbit_order_id {
            query = query.bind(v);
        }
        if let Some(ref v) = fields.exit_bybit_order_id {
            query = query.bind(v);
        }
        if let Some(ref v) = fields.exit_client_order_id {
            query = query.bind(v);
        }
        if let Some(v) = fields.in_flight {
            query = query.bind(v);
        }
        if let Some(ref v) = fields.succeeded_leg {
            query = query.bind(v);
        }
        if let Some(v) = fields.emergency_attempts {
            query = query.bind(v);
        }

        // WHERE 절 바인딩
        query = query.bind(id);
        query = query.bind(from);

        let result = query.execute(&self.pool).await?;

        if result.rows_affected() == 0 {
            // 낙관적 잠금 실패: 현재 상태 조회
            let current_state: Option<(String,)> =
                sqlx::query_as("SELECT state FROM positions WHERE id = ?")
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await?;

            let state = current_state
                .map(|(s,)| s)
                .unwrap_or_else(|| "DELETED".to_string());

            debug!(
                position_id = id,
                expected = from,
                actual = %state,
                "상태 전이 실패: 이미 다른 상태"
            );
            return Ok(TransitionResult::AlreadyTransitioned(state));
        }

        debug!(
            position_id = id,
            from = from,
            to = to,
            "포지션 상태 전이 완료"
        );
        Ok(TransitionResult::Applied)
    }

    async fn load_open(&self, session_id: i64) -> Result<Vec<PositionRecord>, DbError> {
        debug!(session_id = session_id, "오픈 포지션 조회");

        let rows = sqlx::query(
            r#"
            SELECT
                id, session_id, coin, state,
                upbit_qty, bybit_qty,
                upbit_entry_price, bybit_entry_price,
                upbit_order_id, bybit_order_id,
                entry_spread_pct, entry_z_score, entry_usd_krw,
                opened_at, closed_at,
                realized_pnl,
                exit_upbit_order_id, exit_bybit_order_id,
                client_order_id, exit_client_order_id,
                in_flight, succeeded_leg, emergency_attempts
            FROM positions
            WHERE session_id = ? AND state != 'Closed'
            ORDER BY id
            "#,
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await?;

        let positions: Vec<PositionRecord> = rows
            .into_iter()
            .map(|r| PositionRecord {
                id: Some(r.get("id")),
                session_id: r.get("session_id"),
                coin: r.get("coin"),
                state: r.get("state"),
                upbit_qty: r.get("upbit_qty"),
                bybit_qty: r.get("bybit_qty"),
                upbit_entry_price: r.get("upbit_entry_price"),
                bybit_entry_price: r.get("bybit_entry_price"),
                upbit_order_id: r.get("upbit_order_id"),
                bybit_order_id: r.get("bybit_order_id"),
                entry_spread_pct: r.get("entry_spread_pct"),
                entry_z_score: r.get("entry_z_score"),
                entry_usd_krw: r.get("entry_usd_krw"),
                opened_at: r.get("opened_at"),
                closed_at: r.get("closed_at"),
                realized_pnl: r.get("realized_pnl"),
                exit_upbit_order_id: r.get("exit_upbit_order_id"),
                exit_bybit_order_id: r.get("exit_bybit_order_id"),
                client_order_id: r.get("client_order_id"),
                exit_client_order_id: r.get("exit_client_order_id"),
                in_flight: r.get("in_flight"),
                succeeded_leg: r.get("succeeded_leg"),
                emergency_attempts: r.get("emergency_attempts"),
            })
            .collect();

        debug!(
            session_id = session_id,
            count = positions.len(),
            "오픈 포지션 조회 완료"
        );
        Ok(positions)
    }

    async fn remove(&self, id: i64) -> Result<(), DbError> {
        debug!(position_id = id, "포지션 DELETE");

        sqlx::query("DELETE FROM positions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        debug!(position_id = id, "포지션 DELETE 완료");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_record_creation() {
        let record = PositionRecord {
            id: None,
            session_id: 1,
            coin: "BTC".to_string(),
            state: "Opening".to_string(),
            upbit_qty: Decimal::new(100, 8),
            bybit_qty: Decimal::new(100, 8),
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
            client_order_id: Some("test-order-1".to_string()),
            exit_client_order_id: None,
            in_flight: false,
            succeeded_leg: None,
            emergency_attempts: 0,
        };
        assert!(record.id.is_none());
        assert_eq!(record.state, "Opening");
        assert!(!record.in_flight);
    }

    #[test]
    fn test_update_fields_default() {
        let fields = UpdateFields::default();
        assert!(fields.upbit_order_id.is_none());
        assert!(fields.bybit_order_id.is_none());
        assert!(fields.in_flight.is_none());
        assert!(fields.emergency_attempts.is_none());
    }

    #[test]
    fn test_transition_result_variants() {
        let applied = TransitionResult::Applied;
        let already = TransitionResult::AlreadyTransitioned("Closed".to_string());

        assert_eq!(applied, TransitionResult::Applied);
        assert_eq!(
            already,
            TransitionResult::AlreadyTransitioned("Closed".to_string())
        );
    }

    #[test]
    fn test_update_fields_partial() {
        let fields = UpdateFields {
            upbit_order_id: Some("upbit-123".to_string()),
            in_flight: Some(true),
            ..Default::default()
        };
        assert_eq!(fields.upbit_order_id, Some("upbit-123".to_string()));
        assert_eq!(fields.in_flight, Some(true));
        assert!(fields.bybit_order_id.is_none());
    }
}
