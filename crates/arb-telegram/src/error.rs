//! Telegram 관련 에러 타입.

use thiserror::Error;

/// Telegram API 에러.
#[derive(Debug, Error)]
pub enum TelegramError {
    /// 설정 에러 (토큰 또는 chat_id 누락).
    #[error("Telegram configuration error: {0}")]
    ConfigError(String),

    /// HTTP 요청 에러.
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Telegram API 에러 응답.
    #[error("Telegram API error: {description} (code: {error_code})")]
    ApiError {
        /// 에러 코드.
        error_code: i32,
        /// 에러 설명.
        description: String,
    },

    /// JSON 직렬화/역직렬화 에러.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// 요청 제한 초과.
    #[error("Rate limit exceeded, retry after {retry_after} seconds")]
    RateLimited {
        /// 재시도 대기 시간 (초).
        retry_after: i32,
    },
}
