//! DB 에러 타입 정의.

use thiserror::Error;

/// DB 레이어에서 발생할 수 있는 에러 타입.
#[derive(Debug, Error)]
pub enum DbError {
    /// SQL 쿼리 실행 실패.
    #[error("query failed: {0}")]
    QueryFailed(#[from] sqlx::Error),

    /// 커넥션 풀 연결 실패.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    /// 마이그레이션 실행 실패.
    #[error("migration failed: {0}")]
    MigrationFailed(String),

    /// 커넥션 풀 리소스 소진.
    #[error("pool exhausted")]
    PoolExhausted,

    /// 직렬화/역직렬화 오류.
    #[error("serialization error: {0}")]
    SerializationError(String),
}
