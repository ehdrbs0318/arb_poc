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

    /// API 응답 에러 (비즈니스 로직 실패).
    #[error("API error: {0}")]
    ApiError(String),

    /// 데이터 파싱 에러 (문자열 → 숫자 변환 등).
    #[error("Parse error: {0}")]
    ParseError(String),

    /// 거래소의 알 수 없는 에러.
    #[error("Unknown error: {code} - {message}")]
    UnknownError { code: String, message: String },
}

impl ExchangeError {
    /// 에러가 재시도 가능한지 판별합니다.
    ///
    /// 일시적 네트워크 문제, rate limit, 거래소 점검 등은 재시도 가능하고,
    /// 인증 실패, 잔고 부족, 잘못된 파라미터 등은 재시도해도 결과가 동일합니다.
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::RateLimitExceeded(_) => true,
            Self::HttpError(e) => e.is_timeout() || e.is_connect(),
            Self::ExchangeOffline(_) => true,
            Self::WebSocketError(_) => true,
            Self::InternalError(_) => true,
            Self::AuthError(_) => false,
            Self::InsufficientFunds(_) => false,
            Self::InvalidParameter(_) => false,
            Self::OrderNotFound(_) => false,
            Self::MarketNotFound(_) => false,
            Self::ConfigError(_) => false,
            Self::Unsupported(_) => false,
            Self::ApiError(_) => false,
            Self::ParseError(_) => false,
            Self::JsonError(_) => false,
            Self::UnknownError { .. } => false,
        }
    }
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

    #[test]
    fn test_api_error_display() {
        let err =
            ExchangeError::ApiError("No instrument info found for symbol: INVALID".to_string());
        assert_eq!(
            err.to_string(),
            "API error: No instrument info found for symbol: INVALID"
        );
    }

    #[test]
    fn test_parse_error_display() {
        let err = ExchangeError::ParseError("tick_size parse: invalid decimal".to_string());
        assert_eq!(
            err.to_string(),
            "Parse error: tick_size parse: invalid decimal"
        );
    }

    #[test]
    fn test_is_retryable_true_cases() {
        // rate limit은 재시도 가능
        let err = ExchangeError::RateLimitExceeded("too many requests".to_string());
        assert!(err.is_retryable());

        // 거래소 오프라인은 재시도 가능
        let err = ExchangeError::ExchangeOffline("maintenance".to_string());
        assert!(err.is_retryable());

        // WebSocket 에러는 재시도 가능
        let err = ExchangeError::WebSocketError("connection lost".to_string());
        assert!(err.is_retryable());

        // 내부 에러는 재시도 가능
        let err = ExchangeError::InternalError("system error".to_string());
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_false_cases() {
        // 인증 실패는 재시도 불가
        let err = ExchangeError::AuthError("invalid key".to_string());
        assert!(!err.is_retryable());

        // 잔고 부족은 재시도 불가
        let err = ExchangeError::InsufficientFunds("not enough".to_string());
        assert!(!err.is_retryable());

        // 잘못된 파라미터는 재시도 불가
        let err = ExchangeError::InvalidParameter("bad param".to_string());
        assert!(!err.is_retryable());

        // 주문 미발견은 재시도 불가
        let err = ExchangeError::OrderNotFound("no order".to_string());
        assert!(!err.is_retryable());

        // 마켓 미발견은 재시도 불가
        let err = ExchangeError::MarketNotFound("no market".to_string());
        assert!(!err.is_retryable());

        // 설정 에러는 재시도 불가
        let err = ExchangeError::ConfigError("bad config".to_string());
        assert!(!err.is_retryable());

        // 미지원 에러는 재시도 불가
        let err = ExchangeError::Unsupported("not supported".to_string());
        assert!(!err.is_retryable());

        // API 에러는 재시도 불가
        let err = ExchangeError::ApiError("business error".to_string());
        assert!(!err.is_retryable());

        // 파싱 에러는 재시도 불가
        let err = ExchangeError::ParseError("parse failed".to_string());
        assert!(!err.is_retryable());

        // JSON 에러는 재시도 불가
        let json_str = "invalid json";
        let result: Result<serde_json::Value, _> = serde_json::from_str(json_str);
        let err = ExchangeError::from(result.unwrap_err());
        assert!(!err.is_retryable());

        // 알 수 없는 에러는 재시도 불가
        let err = ExchangeError::UnknownError {
            code: "999".to_string(),
            message: "unknown".to_string(),
        };
        assert!(!err.is_retryable());
    }
}
