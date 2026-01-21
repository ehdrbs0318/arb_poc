//! Bithumb JWT authentication module.
//!
//! This module handles JWT token generation for Bithumb API authentication.
//! Bithumb uses JWT (HS256) with SHA512 query hash for private API requests.

use crate::exchange::ExchangeError;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;
use sha2::{Digest, Sha512};
use uuid::Uuid;

/// JWT payload for Bithumb authentication.
#[derive(Debug, Serialize)]
struct JwtPayload {
    access_key: String,
    nonce: String,
    timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_hash_alg: Option<String>,
}

/// Credentials for Bithumb API authentication.
#[derive(Debug, Clone)]
pub struct BithumbCredentials {
    access_key: String,
    secret_key: String,
}

impl BithumbCredentials {
    /// Creates new credentials.
    ///
    /// # Arguments
    ///
    /// * `access_key` - Bithumb API access key
    /// * `secret_key` - Bithumb API secret key
    pub fn new(access_key: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            access_key: access_key.into(),
            secret_key: secret_key.into(),
        }
    }

    /// Returns the current timestamp in milliseconds.
    fn current_timestamp_ms() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    }

    /// Generates a JWT token without query parameters.
    ///
    /// Used for endpoints that don't require query parameters.
    pub fn generate_token(&self) -> Result<String, ExchangeError> {
        let payload = JwtPayload {
            access_key: self.access_key.clone(),
            nonce: Uuid::new_v4().to_string(),
            timestamp: Self::current_timestamp_ms(),
            query_hash: None,
            query_hash_alg: None,
        };

        self.encode_token(&payload)
    }

    /// Generates a JWT token with query hash.
    ///
    /// Used for endpoints that require query parameters.
    ///
    /// # Arguments
    ///
    /// * `query_string` - URL-encoded query string (e.g., "market=KRW-BTC&side=bid")
    pub fn generate_token_with_query(&self, query_string: &str) -> Result<String, ExchangeError> {
        let query_hash = self.hash_query(query_string);

        let payload = JwtPayload {
            access_key: self.access_key.clone(),
            nonce: Uuid::new_v4().to_string(),
            timestamp: Self::current_timestamp_ms(),
            query_hash: Some(query_hash),
            query_hash_alg: Some("SHA512".to_string()),
        };

        self.encode_token(&payload)
    }

    /// Hashes the query string using SHA512.
    fn hash_query(&self, query_string: &str) -> String {
        let mut hasher = Sha512::new();
        hasher.update(query_string.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Encodes the JWT payload.
    fn encode_token(&self, payload: &JwtPayload) -> Result<String, ExchangeError> {
        let header = Header::default(); // Uses HS256 by default
        let key = EncodingKey::from_secret(self.secret_key.as_bytes());

        encode(&header, payload, &key).map_err(|e| ExchangeError::AuthError(e.to_string()))
    }

    /// Returns the authorization header value.
    pub fn authorization_header(&self) -> Result<String, ExchangeError> {
        let token = self.generate_token()?;
        Ok(format!("Bearer {token}"))
    }

    /// Returns the authorization header value with query hash.
    pub fn authorization_header_with_query(
        &self,
        query_string: &str,
    ) -> Result<String, ExchangeError> {
        let token = self.generate_token_with_query(query_string)?;
        Ok(format!("Bearer {token}"))
    }
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
        .map(|(k, v)| {
            format!(
                "{}={}",
                url::form_urlencoded::byte_serialize(k.as_ref().as_bytes()).collect::<String>(),
                url::form_urlencoded::byte_serialize(v.as_ref().as_bytes()).collect::<String>()
            )
        })
        .collect();
    pairs.join("&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let creds = BithumbCredentials::new("test_access_key", "test_secret_key");
        let token = creds.generate_token();
        assert!(token.is_ok());

        let token = token.unwrap();
        // JWT tokens have 3 parts separated by dots
        assert_eq!(token.split('.').count(), 3);
    }

    #[test]
    fn test_generate_token_with_query() {
        let creds = BithumbCredentials::new("test_access_key", "test_secret_key");
        let query = "market=KRW-BTC&side=bid";
        let token = creds.generate_token_with_query(query);
        assert!(token.is_ok());
    }

    #[test]
    fn test_hash_query() {
        let creds = BithumbCredentials::new("test", "test");
        let hash = creds.hash_query("market=KRW-BTC");

        // SHA512 produces 128 hex characters
        assert_eq!(hash.len(), 128);
        // Hash should be consistent
        assert_eq!(hash, creds.hash_query("market=KRW-BTC"));
    }

    #[test]
    fn test_build_query_string() {
        let params = vec![
            ("market", "KRW-BTC"),
            ("side", "bid"),
            ("price", "50000000"),
        ];
        let query = build_query_string(params);
        assert_eq!(query, "market=KRW-BTC&side=bid&price=50000000");
    }

    #[test]
    fn test_build_query_string_with_special_chars() {
        let params = vec![("key", "value with spaces")];
        let query = build_query_string(params);
        // URL encoding uses + for spaces in form-urlencoded format
        assert_eq!(query, "key=value+with+spaces");
    }

    #[test]
    fn test_authorization_header() {
        let creds = BithumbCredentials::new("test_access_key", "test_secret_key");
        let header = creds.authorization_header().unwrap();
        assert!(header.starts_with("Bearer "));
    }
}
