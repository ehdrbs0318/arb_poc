//! funding_schedules 테이블 Repository.
//!
//! 종목별 펀딩 스케줄 관리 (UPSERT).

use crate::error::DbError;
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use tracing::debug;

/// 펀딩 스케줄 레코드.
#[derive(Debug, Clone)]
pub struct FundingScheduleRecord {
    pub id: Option<i64>,
    pub coin: String,
    pub interval_hours: i32,
    pub next_funding_time: DateTime<Utc>,
    pub current_rate: f64,
}

/// funding_schedules 테이블 Repository.
#[derive(Debug, Clone)]
pub struct FundingRepository {
    pool: MySqlPool,
}

impl FundingRepository {
    /// 새 Repository 생성.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 펀딩 스케줄 UPSERT (coin 기준 유니크).
    pub async fn upsert_funding(&self, record: &FundingScheduleRecord) -> Result<(), DbError> {
        debug!(
            coin = %record.coin,
            interval_hours = record.interval_hours,
            next_funding_time = %record.next_funding_time,
            current_rate = record.current_rate,
            "펀딩 스케줄 UPSERT"
        );

        sqlx::query(
            r#"
            INSERT INTO funding_schedules (coin, interval_hours, next_funding_time, current_rate)
            VALUES (?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE
                interval_hours = VALUES(interval_hours),
                next_funding_time = VALUES(next_funding_time),
                current_rate = VALUES(current_rate)
            "#,
        )
        .bind(&record.coin)
        .bind(record.interval_hours)
        .bind(record.next_funding_time)
        .bind(record.current_rate)
        .execute(&self.pool)
        .await?;

        debug!(coin = %record.coin, "펀딩 스케줄 UPSERT 완료");
        Ok(())
    }

    /// 전체 펀딩 스케줄 조회.
    pub async fn get_all_funding(&self) -> Result<Vec<FundingScheduleRecord>, DbError> {
        debug!("전체 펀딩 스케줄 조회");

        let rows = sqlx::query_as::<_, (i64, String, i32, DateTime<Utc>, f64)>(
            r#"
            SELECT id, coin, interval_hours, next_funding_time, current_rate
            FROM funding_schedules
            ORDER BY coin
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let records: Vec<FundingScheduleRecord> = rows
            .into_iter()
            .map(
                |(id, coin, interval_hours, next_funding_time, current_rate)| {
                    FundingScheduleRecord {
                        id: Some(id),
                        coin,
                        interval_hours,
                        next_funding_time,
                        current_rate,
                    }
                },
            )
            .collect();

        debug!(count = records.len(), "전체 펀딩 스케줄 조회 완료");
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funding_schedule_record() {
        let record = FundingScheduleRecord {
            id: None,
            coin: "BTC".to_string(),
            interval_hours: 8,
            next_funding_time: Utc::now(),
            current_rate: 0.0001,
        };
        assert!(record.id.is_none());
        assert_eq!(record.interval_hours, 8);
    }

    #[test]
    fn test_funding_schedule_with_id() {
        let record = FundingScheduleRecord {
            id: Some(1),
            coin: "ETH".to_string(),
            interval_hours: 4,
            next_funding_time: Utc::now(),
            current_rate: -0.0005,
        };
        assert_eq!(record.id, Some(1));
        assert!(record.current_rate < 0.0);
    }
}
