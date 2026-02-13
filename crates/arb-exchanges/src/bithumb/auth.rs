//! Bithumb JWT 인증 모듈.
//!
//! 이 모듈은 Bithumb API 인증을 위한 JWT 토큰 생성을 처리합니다.
//! Bithumb은 비공개 API 요청에 SHA512 쿼리 해시와 함께 JWT (HS256)를 사용합니다.

use arb_exchange::ExchangeError;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;
use sha2::{Digest, Sha512};
use tracing::warn;
use uuid::Uuid;

/// Bithumb 인증용 JWT 페이로드.
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

/// Bithumb API 인증용 자격 증명.
#[derive(Clone)]
pub struct BithumbCredentials {
    access_key: String,
    secret_key: String,
}

impl std::fmt::Debug for BithumbCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let masked_access_key = if self.access_key.len() >= 4 {
            format!("{}****", &self.access_key[..4])
        } else {
            "****".to_string()
        };
        f.debug_struct("BithumbCredentials")
            .field("access_key", &masked_access_key)
            .field("secret_key", &"****")
            .finish()
    }
}

impl BithumbCredentials {
    /// 새 자격 증명을 생성합니다.
    ///
    /// # 인자
    ///
    /// * `access_key` - Bithumb API 액세스 키
    /// * `secret_key` - Bithumb API 시크릿 키
    pub fn new(access_key: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            access_key: access_key.into(),
            secret_key: secret_key.into(),
        }
    }

    /// 현재 타임스탬프를 밀리초 단위로 반환합니다.
    ///
    /// `SystemTime`이 `UNIX_EPOCH` 이전인 경우 0을 반환하고 warn 로그를 남깁니다.
    fn current_timestamp_ms() -> i64 {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        if ts == 0 {
            warn!("SystemTime duration_since failed, using default");
        }
        ts
    }

    /// 쿼리 파라미터 없이 JWT 토큰을 생성합니다.
    ///
    /// 쿼리 파라미터가 필요 없는 엔드포인트에 사용됩니다.
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

    /// 쿼리 해시와 함께 JWT 토큰을 생성합니다.
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
            timestamp: Self::current_timestamp_ms(),
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
        let header = Header::default(); // 기본적으로 HS256 사용
        let key = EncodingKey::from_secret(self.secret_key.as_bytes());

        encode(&header, payload, &key).map_err(|e| ExchangeError::AuthError(e.to_string()))
    }

    /// Authorization 헤더 값을 반환합니다.
    pub fn authorization_header(&self) -> Result<String, ExchangeError> {
        let token = self.generate_token()?;
        Ok(format!("Bearer {token}"))
    }

    /// 쿼리 해시와 함께 Authorization 헤더 값을 반환합니다.
    pub fn authorization_header_with_query(
        &self,
        query_string: &str,
    ) -> Result<String, ExchangeError> {
        let token = self.generate_token_with_query(query_string)?;
        Ok(format!("Bearer {token}"))
    }
}

/// 파라미터들로부터 쿼리 문자열을 생성합니다.
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
        let creds = BithumbCredentials::new("test_access_key", "test_secret_key");
        let token = creds.generate_token();
        assert!(token.is_ok());

        let token = token.unwrap();
        // JWT 토큰은 점으로 구분된 3개의 부분으로 구성됨
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

        // SHA512는 128자의 16진수 문자열을 생성함
        assert_eq!(hash.len(), 128);
        // 해시는 일관성이 있어야 함
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
    fn test_authorization_header() {
        let creds = BithumbCredentials::new("test_access_key", "test_secret_key");
        let header = creds.authorization_header().unwrap();
        assert!(header.starts_with("Bearer "));
    }

    #[test]
    fn test_debug_masks_credentials() {
        let creds = BithumbCredentials::new("abcd1234secret", "my_super_secret_key");
        let debug_str = format!("{:?}", creds);
        // access_key 앞 4자만 노출
        assert!(debug_str.contains("abcd****"));
        // secret_key 완전 마스킹
        assert!(!debug_str.contains("my_super_secret_key"));
        assert!(debug_str.contains("\"****\""));
    }

    #[test]
    fn test_debug_masks_short_access_key() {
        let creds = BithumbCredentials::new("ab", "short_secret");
        let debug_str = format!("{:?}", creds);
        // 4자 미만이면 전체 마스킹
        assert!(debug_str.contains("\"****\""));
        assert!(!debug_str.contains("short_secret"));
    }
}
