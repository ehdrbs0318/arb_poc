//! Upbit JWT 인증 모듈.
//!
//! 이 모듈은 Upbit API 인증을 위한 JWT 토큰 생성을 처리합니다.

use arb_exchange::ExchangeError;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;
use sha2::{Digest, Sha512};
use uuid::Uuid;

/// Upbit 인증을 위한 JWT 페이로드.
#[derive(Debug, Serialize)]
struct JwtPayload {
    access_key: String,
    nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_hash_alg: Option<String>,
}

/// Upbit API 인증을 위한 자격 증명.
#[derive(Debug, Clone)]
pub struct UpbitCredentials {
    access_key: String,
    secret_key: String,
}

impl UpbitCredentials {
    /// 새로운 자격 증명을 생성합니다.
    ///
    /// # 인자
    ///
    /// * `access_key` - Upbit API access key
    /// * `secret_key` - Upbit API secret key
    pub fn new(access_key: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            access_key: access_key.into(),
            secret_key: secret_key.into(),
        }
    }

    /// 쿼리 파라미터 없이 JWT 토큰을 생성합니다.
    ///
    /// 쿼리 파라미터가 필요하지 않은 엔드포인트에 사용됩니다.
    pub fn generate_token(&self) -> Result<String, ExchangeError> {
        let payload = JwtPayload {
            access_key: self.access_key.clone(),
            nonce: Uuid::new_v4().to_string(),
            query_hash: None,
            query_hash_alg: None,
        };

        self.encode_token(&payload)
    }

    /// 쿼리 해시를 포함한 JWT 토큰을 생성합니다.
    ///
    /// 쿼리 파라미터가 필요한 엔드포인트에 사용됩니다.
    ///
    /// # 인자
    ///
    /// * `query_string` - URL 인코딩된 쿼리 문자열 (예: "market=KRW-BTC&side=bid")
    pub fn generate_token_with_query(&self, query_string: &str) -> Result<String, ExchangeError> {
        let query_hash = self.hash_query(query_string);

        let payload = JwtPayload {
            access_key: self.access_key.clone(),
            nonce: Uuid::new_v4().to_string(),
            query_hash: Some(query_hash),
            query_hash_alg: Some("SHA512".to_string()),
        };

        self.encode_token(&payload)
    }

    /// SHA512를 사용하여 쿼리 문자열을 해시합니다.
    fn hash_query(&self, query_string: &str) -> String {
        let mut hasher = Sha512::new();
        hasher.update(query_string.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// JWT 페이로드를 인코딩합니다.
    fn encode_token(&self, payload: &JwtPayload) -> Result<String, ExchangeError> {
        let header = Header::default(); // 기본값으로 HS256 사용
        let key = EncodingKey::from_secret(self.secret_key.as_bytes());

        encode(&header, payload, &key).map_err(|e| ExchangeError::AuthError(e.to_string()))
    }

    /// Authorization 헤더 값을 반환합니다.
    pub fn authorization_header(&self) -> Result<String, ExchangeError> {
        let token = self.generate_token()?;
        Ok(format!("Bearer {token}"))
    }

    /// 쿼리 해시를 포함한 Authorization 헤더 값을 반환합니다.
    pub fn authorization_header_with_query(
        &self,
        query_string: &str,
    ) -> Result<String, ExchangeError> {
        let token = self.generate_token_with_query(query_string)?;
        Ok(format!("Bearer {token}"))
    }
}

/// 파라미터로부터 쿼리 문자열을 생성합니다.
///
/// # 인자
///
/// * `params` - 키-값 쌍의 이터레이터
///
/// # 반환값
///
/// URL 인코딩된 쿼리 문자열
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
        let creds = UpbitCredentials::new("test_access_key", "test_secret_key");
        let token = creds.generate_token();
        assert!(token.is_ok());

        let token = token.unwrap();
        // JWT 토큰은 점(.)으로 구분된 3개의 파트로 구성됨
        assert_eq!(token.split('.').count(), 3);
    }

    #[test]
    fn test_generate_token_with_query() {
        let creds = UpbitCredentials::new("test_access_key", "test_secret_key");
        let query = "market=KRW-BTC&side=bid";
        let token = creds.generate_token_with_query(query);
        assert!(token.is_ok());
    }

    #[test]
    fn test_hash_query() {
        let creds = UpbitCredentials::new("test", "test");
        let hash = creds.hash_query("market=KRW-BTC");

        // SHA512는 128개의 16진수 문자를 생성함
        assert_eq!(hash.len(), 128);
        // 해시 값은 일관성이 있어야 함
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
        // URL 인코딩은 form-urlencoded 형식에서 공백을 +로 변환함
        assert_eq!(query, "key=value+with+spaces");
    }

    #[test]
    fn test_authorization_header() {
        let creds = UpbitCredentials::new("test_access_key", "test_secret_key");
        let header = creds.authorization_header().unwrap();
        assert!(header.starts_with("Bearer "));
    }
}
