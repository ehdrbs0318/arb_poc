//! 거래소 에러 타입.
//!
//! 이 모듈은 모든 거래소 구현에서 공통으로 사용되는 에러 타입을 정의합니다.

use thiserror::Error;

/// 거래소 작업 중 발생할 수 있는 에러를 나타냅니다.
#[derive(Error, Debug)]
pub enum ExchangeError {
    /// HTTP 요청 실패.
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON 직렬화/역직렬화 실패.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// 인증 실패.
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// 유효하지 않은 파라미터 제공.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// 작업에 필요한 잔고 부족.
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    /// 주문을 찾을 수 없음.
    #[error("Order not found: {0}")]
    OrderNotFound(String),

    /// 마켓을 찾을 수 없거나 사용 불가.
    #[error("Market not found: {0}")]
    MarketNotFound(String),

    /// 요청 제한 초과.
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// 거래소 오프라인 또는 점검 중.
    #[error("Exchange offline: {0}")]
    ExchangeOffline(String),

    /// WebSocket 연결 에러.
    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    /// 설정 에러.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// 거래소 내부 에러.
    #[error("Internal error: {0}")]
    InternalError(String),

    /// 지원하지 않는 작업.
    #[error("Operation not supported: {0}")]
    Unsupported(String),

    /// 거래소의 알 수 없는 에러.
    #[error("Unknown error: {code} - {message}")]
    UnknownError { code: String, message: String },
}

/// 거래소 작업을 위한 Result 타입 별칭.
pub type ExchangeResult<T> = Result<T, ExchangeError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ExchangeError::AuthError("Invalid API key".to_string());
        assert_eq!(err.to_string(), "Authentication failed: Invalid API key");
    }

    #[test]
    fn test_error_from_json() {
        let json_str = "invalid json";
        let result: Result<serde_json::Value, _> = serde_json::from_str(json_str);
        let err = ExchangeError::from(result.unwrap_err());
        assert!(err.to_string().contains("JSON error"));
    }

    #[test]
    fn test_unknown_error_display() {
        let err = ExchangeError::UnknownError {
            code: "E001".to_string(),
            message: "Something went wrong".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Unknown error: E001 - Something went wrong"
        );
    }

    #[test]
    fn test_unsupported_error_display() {
        let err = ExchangeError::Unsupported("subscribe_markets not implemented".to_string());
        assert_eq!(
            err.to_string(),
            "Operation not supported: subscribe_markets not implemented"
        );
    }
}
