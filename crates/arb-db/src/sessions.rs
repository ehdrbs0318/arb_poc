//! sessions 테이블 Repository.
//!
//! 세션 생성, 종료, heartbeat 업데이트, crash 복구 처리.

use crate::error::DbError;
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use tracing::debug;

/// 세션 레코드.
#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub id: i64,
    pub parent_session_id: Option<i64>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub config_json: String,
    pub status: String,
}

/// sessions 테이블 Repository.
#[derive(Debug, Clone)]
pub struct SessionRepository {
    pool: MySqlPool,
}

impl SessionRepository {
    /// 새 Repository 생성.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 새 세션 생성. 생성된 세션 ID 반환.
    ///
    /// # 인자
    ///
    /// * `config_json` - 세션 설정 JSON (민감 필드 redact 완료 상태)
    /// * `parent_session_id` - crash recovery 시 이전 세션 ID
    pub async fn create_session(
        &self,
        config_json: &str,
        parent_session_id: Option<i64>,
    ) -> Result<i64, DbError> {
        debug!(
            parent_session_id = ?parent_session_id,
            config_len = config_json.len(),
            "세션 INSERT"
        );

        let result = sqlx::query(
            r#"
            INSERT INTO sessions (parent_session_id, started_at, config_json, status)
            VALUES (?, NOW(3), ?, 'Running')
            "#,
        )
        .bind(parent_session_id)
        .bind(config_json)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        debug!(session_id = id, "세션 생성 완료");
        Ok(id)
    }

    /// 세션 종료 상태 업데이트.
    ///
    /// # 인자
    ///
    /// * `id` - 세션 ID
    /// * `status` - 종료 상태 ("Completed", "GracefulStop" 등)
    pub async fn end_session(&self, id: i64, status: &str) -> Result<(), DbError> {
        debug!(session_id = id, status = status, "세션 종료 UPDATE");

        sqlx::query(
            r#"
            UPDATE sessions SET ended_at = NOW(3), status = ?
            WHERE id = ?
            "#,
        )
        .bind(status)
        .bind(id)
        .execute(&self.pool)
        .await?;

        debug!(session_id = id, "세션 종료 완료");
        Ok(())
    }

    /// 세션 heartbeat 갱신 (status 유지).
    pub async fn update_heartbeat(&self, id: i64) -> Result<(), DbError> {
        debug!(session_id = id, "heartbeat UPDATE");

        sqlx::query(
            r#"
            UPDATE sessions SET ended_at = NOW(3)
            WHERE id = ? AND status = 'Running'
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 이전 세션을 Crashed로 마킹.
    ///
    /// # 인자
    ///
    /// * `id` - 크래시된 세션 ID
    /// * `new_session_id` - 새로 시작한 복구 세션 ID
    pub async fn mark_crashed(&self, id: i64, new_session_id: i64) -> Result<(), DbError> {
        debug!(
            crashed_session_id = id,
            new_session_id = new_session_id,
            "세션 Crashed 마킹"
        );

        sqlx::query(
            r#"
            UPDATE sessions SET status = 'Crashed', ended_at = NOW(3)
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        debug!(session_id = id, "Crashed 마킹 완료");
        Ok(())
    }

    /// 가장 최근 Running 세션 조회 (crash recovery용).
    pub async fn find_last_running(&self) -> Result<Option<SessionRecord>, DbError> {
        debug!("마지막 Running 세션 조회");

        let row = sqlx::query_as::<
            _,
            (
                i64,
                Option<i64>,
                DateTime<Utc>,
                Option<DateTime<Utc>>,
                String,
                String,
            ),
        >(
            r#"
            SELECT id, parent_session_id, started_at, ended_at, config_json, status
            FROM sessions
            WHERE status = 'Running'
            ORDER BY id DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;

        let record = row.map(
            |(id, parent_session_id, started_at, ended_at, config_json, status)| SessionRecord {
                id,
                parent_session_id,
                started_at,
                ended_at,
                config_json,
                status,
            },
        );

        debug!(found = record.is_some(), "Running 세션 조회 완료");
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_record_default_fields() {
        let record = SessionRecord {
            id: 1,
            parent_session_id: None,
            started_at: Utc::now(),
            ended_at: None,
            config_json: "{}".to_string(),
            status: "Running".to_string(),
        };
        assert_eq!(record.id, 1);
        assert!(record.parent_session_id.is_none());
        assert_eq!(record.status, "Running");
    }

    #[test]
    fn test_session_record_with_parent() {
        let record = SessionRecord {
            id: 2,
            parent_session_id: Some(1),
            started_at: Utc::now(),
            ended_at: None,
            config_json: r#"{"key": "value"}"#.to_string(),
            status: "Running".to_string(),
        };
        assert_eq!(record.parent_session_id, Some(1));
    }
}
