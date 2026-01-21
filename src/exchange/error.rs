//! Exchange error types.
//!
//! This module defines the common error types used across all exchange implementations.

use thiserror::Error;

/// Represents errors that can occur during exchange operations.
#[derive(Error, Debug)]
pub enum ExchangeError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Authentication failed.
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// Invalid parameter provided.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Insufficient funds for the operation.
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    /// Order not found.
    #[error("Order not found: {0}")]
    OrderNotFound(String),

    /// Market not found or unavailable.
    #[error("Market not found: {0}")]
    MarketNotFound(String),

    /// Rate limit exceeded.
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// Exchange is offline or under maintenance.
    #[error("Exchange offline: {0}")]
    ExchangeOffline(String),

    /// WebSocket connection error.
    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Internal exchange error.
    #[error("Internal error: {0}")]
    InternalError(String),

    /// Unknown error from the exchange.
    #[error("Unknown error: {code} - {message}")]
    UnknownError { code: String, message: String },
}

/// Result type alias for exchange operations.
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
}
