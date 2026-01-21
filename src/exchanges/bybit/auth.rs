//! Bybit HMAC-SHA256 authentication module.
//!
//! This module handles signature generation for Bybit V5 API authentication.
//!
//! # Signature Generation
//!
//! Bybit uses HMAC-SHA256 for API authentication:
//! - GET requests: `timestamp + api_key + recv_window + queryString`
//! - POST requests: `timestamp + api_key + recv_window + jsonBodyString`
//!
//! The signature is then converted to lowercase hexadecimal.

use crate::exchange::ExchangeError;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// Default receive window in milliseconds.
pub const DEFAULT_RECV_WINDOW: u64 = 5000;

/// Credentials for Bybit API authentication.
#[derive(Debug, Clone)]
pub struct BybitCredentials {
    api_key: String,
    secret_key: String,
    recv_window: u64,
}

impl BybitCredentials {
    /// Creates new credentials with default receive window.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Bybit API key
    /// * `secret_key` - Bybit API secret key
    pub fn new(api_key: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            recv_window: DEFAULT_RECV_WINDOW,
        }
    }

    /// Creates new credentials with custom receive window.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Bybit API key
    /// * `secret_key` - Bybit API secret key
    /// * `recv_window` - Receive window in milliseconds
    #[allow(dead_code)]
    pub fn with_recv_window(
        api_key: impl Into<String>,
        secret_key: impl Into<String>,
        recv_window: u64,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            recv_window,
        }
    }

    /// Returns the API key.
    #[inline]
    #[allow(dead_code)]
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Returns the receive window.
    #[inline]
    #[allow(dead_code)]
    pub fn recv_window(&self) -> u64 {
        self.recv_window
    }

    /// Returns the current UTC timestamp in milliseconds.
    #[inline]
    pub fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }

    /// Generates signature for GET request.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - UTC timestamp in milliseconds
    /// * `query_string` - URL query string (without leading `?`)
    ///
    /// # Returns
    ///
    /// Lowercase hexadecimal HMAC-SHA256 signature
    pub fn sign_get(&self, timestamp: u64, query_string: &str) -> Result<String, ExchangeError> {
        let payload = format!(
            "{}{}{}{}",
            timestamp, self.api_key, self.recv_window, query_string
        );
        self.hmac_sign(&payload)
    }

    /// Generates signature for POST request.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - UTC timestamp in milliseconds
    /// * `body` - JSON body string
    ///
    /// # Returns
    ///
    /// Lowercase hexadecimal HMAC-SHA256 signature
    pub fn sign_post(&self, timestamp: u64, body: &str) -> Result<String, ExchangeError> {
        let payload = format!("{}{}{}{}", timestamp, self.api_key, self.recv_window, body);
        self.hmac_sign(&payload)
    }

    /// Computes HMAC-SHA256 signature.
    fn hmac_sign(&self, payload: &str) -> Result<String, ExchangeError> {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .map_err(|e| ExchangeError::AuthError(format!("HMAC key error: {}", e)))?;

        mac.update(payload.as_bytes());
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
    }

    /// Returns all required authentication headers for a GET request.
    ///
    /// # Arguments
    ///
    /// * `query_string` - URL query string (without leading `?`)
    ///
    /// # Returns
    ///
    /// Tuple of (timestamp, api_key, recv_window, signature)
    pub fn auth_headers_get(&self, query_string: &str) -> Result<AuthHeaders, ExchangeError> {
        let timestamp = Self::timestamp();
        let signature = self.sign_get(timestamp, query_string)?;

        Ok(AuthHeaders {
            timestamp,
            api_key: self.api_key.clone(),
            recv_window: self.recv_window,
            signature,
        })
    }

    /// Returns all required authentication headers for a POST request.
    ///
    /// # Arguments
    ///
    /// * `body` - JSON body string
    ///
    /// # Returns
    ///
    /// Authentication headers struct
    pub fn auth_headers_post(&self, body: &str) -> Result<AuthHeaders, ExchangeError> {
        let timestamp = Self::timestamp();
        let signature = self.sign_post(timestamp, body)?;

        Ok(AuthHeaders {
            timestamp,
            api_key: self.api_key.clone(),
            recv_window: self.recv_window,
            signature,
        })
    }
}

/// Authentication headers for Bybit API requests.
#[derive(Debug, Clone)]
pub struct AuthHeaders {
    /// UTC timestamp in milliseconds.
    pub timestamp: u64,
    /// API key.
    pub api_key: String,
    /// Receive window in milliseconds.
    pub recv_window: u64,
    /// HMAC-SHA256 signature.
    pub signature: String,
}

impl AuthHeaders {
    /// Header name for API key.
    pub const HEADER_API_KEY: &'static str = "X-BAPI-API-KEY";
    /// Header name for timestamp.
    pub const HEADER_TIMESTAMP: &'static str = "X-BAPI-TIMESTAMP";
    /// Header name for signature.
    pub const HEADER_SIGN: &'static str = "X-BAPI-SIGN";
    /// Header name for receive window.
    pub const HEADER_RECV_WINDOW: &'static str = "X-BAPI-RECV-WINDOW";
}

/// Builds a query string from parameters.
///
/// # Arguments
///
/// * `params` - Iterator of key-value pairs
///
/// # Returns
///
/// URL-encoded query string
pub fn build_query_string<I, K, V>(params: I) -> String
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    let pairs: Vec<String> = params
        .into_iter()
        .map(|(k, v)| format!("{}={}", k.as_ref(), v.as_ref()))
        .collect();
    pairs.join("&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_new() {
        let creds = BybitCredentials::new("api_key", "secret_key");
        assert_eq!(creds.api_key(), "api_key");
        assert_eq!(creds.recv_window(), DEFAULT_RECV_WINDOW);
    }

    #[test]
    fn test_credentials_with_recv_window() {
        let creds = BybitCredentials::with_recv_window("api_key", "secret_key", 10000);
        assert_eq!(creds.recv_window(), 10000);
    }

    #[test]
    fn test_timestamp() {
        let ts = BybitCredentials::timestamp();
        assert!(ts > 0);
        // Should be a reasonable timestamp (after 2020)
        assert!(ts > 1577836800000);
    }

    #[test]
    fn test_sign_get() {
        let creds = BybitCredentials::new("my_api_key", "my_secret_key");
        let timestamp = 1672531200000u64;
        let query = "category=spot&symbol=BTCUSDT";

        let signature = creds.sign_get(timestamp, query).unwrap();

        // Signature should be 64 hex characters (256 bits)
        assert_eq!(signature.len(), 64);
        // Should be lowercase hex
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(signature, signature.to_lowercase());
    }

    #[test]
    fn test_sign_post() {
        let creds = BybitCredentials::new("my_api_key", "my_secret_key");
        let timestamp = 1672531200000u64;
        let body = r#"{"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"0.1","price":"30000"}"#;

        let signature = creds.sign_post(timestamp, body).unwrap();

        assert_eq!(signature.len(), 64);
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_signature_consistency() {
        let creds = BybitCredentials::new("test_key", "test_secret");
        let timestamp = 1672531200000u64;
        let query = "symbol=BTCUSDT";

        let sig1 = creds.sign_get(timestamp, query).unwrap();
        let sig2 = creds.sign_get(timestamp, query).unwrap();

        // Same input should produce same signature
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_auth_headers_get() {
        let creds = BybitCredentials::new("api_key", "secret_key");
        let headers = creds.auth_headers_get("symbol=BTCUSDT").unwrap();

        assert_eq!(headers.api_key, "api_key");
        assert_eq!(headers.recv_window, DEFAULT_RECV_WINDOW);
        assert!(headers.timestamp > 0);
        assert_eq!(headers.signature.len(), 64);
    }

    #[test]
    fn test_auth_headers_post() {
        let creds = BybitCredentials::new("api_key", "secret_key");
        let body = r#"{"symbol":"BTCUSDT"}"#;
        let headers = creds.auth_headers_post(body).unwrap();

        assert_eq!(headers.api_key, "api_key");
        assert_eq!(headers.recv_window, DEFAULT_RECV_WINDOW);
        assert!(headers.timestamp > 0);
        assert_eq!(headers.signature.len(), 64);
    }

    #[test]
    fn test_build_query_string() {
        let params = vec![("category", "spot"), ("symbol", "BTCUSDT"), ("limit", "10")];
        let query = build_query_string(params);
        assert_eq!(query, "category=spot&symbol=BTCUSDT&limit=10");
    }

    #[test]
    fn test_build_query_string_empty() {
        let params: Vec<(&str, &str)> = vec![];
        let query = build_query_string(params);
        assert_eq!(query, "");
    }
}
