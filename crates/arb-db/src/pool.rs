//! MySQL 커넥션 풀 관리.

use crate::error::DbError;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::time::Duration;
use tracing::{debug, info};

/// DB 커넥션 풀 설정.
#[derive(Debug, Clone)]
pub struct DbPoolConfig {
    /// 최대 커넥션 수.
    pub max_connections: u32,
    /// 최소 커넥션 수.
    pub min_connections: u32,
    /// 커넥션 획득 타임아웃 (초).
    pub acquire_timeout_secs: u64,
}

impl Default for DbPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            acquire_timeout_secs: 5,
        }
    }
}

/// MySQL 커넥션 풀 래퍼.
#[derive(Debug, Clone)]
pub struct DbPool {
    pool: MySqlPool,
}

impl DbPool {
    /// MySQL 커넥션 풀 생성.
    ///
    /// # 인자
    ///
    /// * `url` - MySQL 연결 URL (예: `mysql://user:pass@localhost/dbname`)
    /// * `config` - 풀 설정
    pub async fn connect(url: &str, config: &DbPoolConfig) -> Result<Self, DbError> {
        debug!(
            max_connections = config.max_connections,
            min_connections = config.min_connections,
            acquire_timeout_secs = config.acquire_timeout_secs,
            "DB 풀 연결 시도"
        );

        let pool = MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs))
            .connect(url)
            .await
            .map_err(|e| {
                DbError::ConnectionFailed(format!("failed to connect to {}: {}", url, e))
            })?;

        info!("DB 풀 연결 성공");
        Ok(Self { pool })
    }

    /// 커넥션 풀 상태 확인.
    pub async fn health_check(&self) -> Result<(), DbError> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| DbError::ConnectionFailed(format!("health check failed: {}", e)))?;
        Ok(())
    }

    /// 내부 MySqlPool 참조 반환.
    pub fn inner(&self) -> &MySqlPool {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_pool_config_default() {
        let config = DbPoolConfig::default();
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 2);
        assert_eq!(config.acquire_timeout_secs, 5);
    }

    #[test]
    fn test_db_pool_config_custom() {
        let config = DbPoolConfig {
            max_connections: 20,
            min_connections: 5,
            acquire_timeout_secs: 10,
        };
        assert_eq!(config.max_connections, 20);
        assert_eq!(config.min_connections, 5);
        assert_eq!(config.acquire_timeout_secs, 10);
    }
}
