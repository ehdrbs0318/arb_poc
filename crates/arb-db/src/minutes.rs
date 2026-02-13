//! minutes 테이블 Repository.
//!
//! 분봉 스프레드 데이터 저장 및 조회 (기존 minutes.csv 대체).

use crate::error::DbError;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::MySqlPool;
use tracing::debug;

/// 분봉 레코드.
#[derive(Debug, Clone)]
pub struct MinuteRecord {
    pub id: Option<i64>,
    pub session_id: i64,
    pub coin: String,
    pub ts: DateTime<Utc>,
    pub upbit_close: Option<Decimal>,
    pub bybit_close: Option<Decimal>,
    pub spread_pct: Option<f64>,
    pub z_score: Option<f64>,
    pub mean: Option<f64>,
    pub stddev: Option<f64>,
}

/// minutes 테이블 Repository.
#[derive(Debug, Clone)]
pub struct MinuteRepository {
    pool: MySqlPool,
}

impl MinuteRepository {
    /// 새 Repository 생성.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 분봉 데이터 INSERT. 생성된 ID 반환.
    pub async fn insert_minute(&self, minute: &MinuteRecord) -> Result<i64, DbError> {
        debug!(
            session_id = minute.session_id,
            coin = %minute.coin,
            ts = %minute.ts,
            spread_pct = ?minute.spread_pct,
            "분봉 INSERT"
        );

        let result = sqlx::query(
            r#"
            INSERT INTO minutes (
                session_id, coin, ts,
                upbit_close, bybit_close,
                spread_pct, z_score, mean, stddev
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(minute.session_id)
        .bind(&minute.coin)
        .bind(minute.ts)
        .bind(minute.upbit_close)
        .bind(minute.bybit_close)
        .bind(minute.spread_pct)
        .bind(minute.z_score)
        .bind(minute.mean)
        .bind(minute.stddev)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        debug!(minute_id = id, "분봉 INSERT 완료");
        Ok(id)
    }

    /// 특정 세션의 분봉 데이터 조회.
    pub async fn get_minutes_by_session(
        &self,
        session_id: i64,
    ) -> Result<Vec<MinuteRecord>, DbError> {
        debug!(session_id = session_id, "세션별 분봉 조회");

        let rows = sqlx::query_as::<
            _,
            (
                i64,
                i64,
                String,
                DateTime<Utc>,
                Option<Decimal>,
                Option<Decimal>,
                Option<f64>,
                Option<f64>,
                Option<f64>,
                Option<f64>,
            ),
        >(
            r#"
            SELECT
                id, session_id, coin, ts,
                upbit_close, bybit_close,
                spread_pct, z_score, mean, stddev
            FROM minutes
            WHERE session_id = ?
            ORDER BY ts
            "#,
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await?;

        let minutes: Vec<MinuteRecord> = rows
            .into_iter()
            .map(|r| MinuteRecord {
                id: Some(r.0),
                session_id: r.1,
                coin: r.2,
                ts: r.3,
                upbit_close: r.4,
                bybit_close: r.5,
                spread_pct: r.6,
                z_score: r.7,
                mean: r.8,
                stddev: r.9,
            })
            .collect();

        debug!(
            session_id = session_id,
            count = minutes.len(),
            "세션별 분봉 조회 완료"
        );
        Ok(minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minute_record_creation() {
        let record = MinuteRecord {
            id: None,
            session_id: 1,
            coin: "BTC".to_string(),
            ts: Utc::now(),
            upbit_close: Some(Decimal::new(50_000_000, 0)),
            bybit_close: Some(Decimal::new(35000, 0)),
            spread_pct: Some(0.15),
            z_score: Some(1.5),
            mean: Some(0.10),
            stddev: Some(0.05),
        };
        assert!(record.id.is_none());
        assert_eq!(record.coin, "BTC");
    }

    #[test]
    fn test_minute_record_with_none_fields() {
        let record = MinuteRecord {
            id: None,
            session_id: 1,
            coin: "ETH".to_string(),
            ts: Utc::now(),
            upbit_close: None,
            bybit_close: None,
            spread_pct: None,
            z_score: None,
            mean: None,
            stddev: None,
        };
        assert!(record.upbit_close.is_none());
        assert!(record.z_score.is_none());
    }
}
