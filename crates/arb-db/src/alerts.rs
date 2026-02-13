//! alerts 테이블 Repository.
//!
//! 알림 기록 저장 (기존 alerts.log 대체).

use crate::error::DbError;
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use tracing::debug;

/// 알림 레코드.
#[derive(Debug, Clone)]
pub struct AlertRecord {
    pub id: Option<i64>,
    pub session_id: i64,
    pub level: String,
    pub event_type: String,
    pub message: String,
    pub payload_json: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// alerts 테이블 Repository.
#[derive(Debug, Clone)]
pub struct AlertRepository {
    pool: MySqlPool,
}

impl AlertRepository {
    /// 새 Repository 생성.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 알림 기록 INSERT. 생성된 ID 반환.
    pub async fn insert_alert(&self, alert: &AlertRecord) -> Result<i64, DbError> {
        debug!(
            session_id = alert.session_id,
            level = %alert.level,
            event_type = %alert.event_type,
            "알림 INSERT"
        );

        let result = sqlx::query(
            r#"
            INSERT INTO alerts (
                session_id, level, event_type, message, payload_json, created_at
            ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(alert.session_id)
        .bind(&alert.level)
        .bind(&alert.event_type)
        .bind(&alert.message)
        .bind(&alert.payload_json)
        .bind(alert.created_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        debug!(alert_id = id, "알림 INSERT 완료");
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_record_creation() {
        let record = AlertRecord {
            id: None,
            session_id: 1,
            level: "warn".to_string(),
            event_type: "one_leg_fail".to_string(),
            message: "Upbit order failed".to_string(),
            payload_json: Some(r#"{"order_id": "123"}"#.to_string()),
            created_at: Utc::now(),
        };
        assert!(record.id.is_none());
        assert_eq!(record.level, "warn");
    }

    #[test]
    fn test_alert_record_without_payload() {
        let record = AlertRecord {
            id: Some(42),
            session_id: 1,
            level: "critical".to_string(),
            event_type: "kill_switch".to_string(),
            message: "Kill switch activated".to_string(),
            payload_json: None,
            created_at: Utc::now(),
        };
        assert_eq!(record.id, Some(42));
        assert!(record.payload_json.is_none());
    }
}
