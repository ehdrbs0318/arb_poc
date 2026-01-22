//! Bybit HMAC-SHA256 인증 모듈.
//!
//! 이 모듈은 Bybit V5 API 인증을 위한 서명 생성을 처리합니다.
//!
//! # 서명 생성
//!
//! Bybit은 API 인증에 HMAC-SHA256을 사용합니다:
//! - GET 요청: `timestamp + api_key + recv_window + queryString`
//! - POST 요청: `timestamp + api_key + recv_window + jsonBodyString`
//!
//! 서명은 소문자 16진수로 변환됩니다.

use arb_exchange::ExchangeError;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// 기본 수신 윈도우 (밀리초 단위).
pub const DEFAULT_RECV_WINDOW: u64 = 5000;

/// Bybit API 인증을 위한 자격 증명.
#[derive(Debug, Clone)]
pub struct BybitCredentials {
    api_key: String,
    secret_key: String,
    recv_window: u64,
}

impl BybitCredentials {
    /// 기본 수신 윈도우로 새 자격 증명을 생성합니다.
    ///
    /// # 인자
    ///
    /// * `api_key` - Bybit API 키
    /// * `secret_key` - Bybit API 시크릿 키
    pub fn new(api_key: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            recv_window: DEFAULT_RECV_WINDOW,
        }
    }

    /// 사용자 정의 수신 윈도우로 새 자격 증명을 생성합니다.
    ///
    /// # 인자
    ///
    /// * `api_key` - Bybit API 키
    /// * `secret_key` - Bybit API 시크릿 키
    /// * `recv_window` - 수신 윈도우 (밀리초 단위)
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

    /// API 키를 반환합니다.
    #[inline]
    #[allow(dead_code)]
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// 수신 윈도우를 반환합니다.
    #[inline]
    #[allow(dead_code)]
    pub fn recv_window(&self) -> u64 {
        self.recv_window
    }

    /// 현재 UTC 타임스탬프를 밀리초 단위로 반환합니다.
    #[inline]
    pub fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }

    /// GET 요청을 위한 서명을 생성합니다.
    ///
    /// # 인자
    ///
    /// * `timestamp` - UTC 타임스탬프 (밀리초 단위)
    /// * `query_string` - URL 쿼리 문자열 (앞의 `?` 제외)
    ///
    /// # 반환값
    ///
    /// 소문자 16진수 HMAC-SHA256 서명
    pub fn sign_get(&self, timestamp: u64, query_string: &str) -> Result<String, ExchangeError> {
        let payload = format!(
            "{}{}{}{}",
            timestamp, self.api_key, self.recv_window, query_string
        );
        self.hmac_sign(&payload)
    }

    /// POST 요청을 위한 서명을 생성합니다.
    ///
    /// # 인자
    ///
    /// * `timestamp` - UTC 타임스탬프 (밀리초 단위)
    /// * `body` - JSON 본문 문자열
    ///
    /// # 반환값
    ///
    /// 소문자 16진수 HMAC-SHA256 서명
    pub fn sign_post(&self, timestamp: u64, body: &str) -> Result<String, ExchangeError> {
        let payload = format!("{}{}{}{}", timestamp, self.api_key, self.recv_window, body);
        self.hmac_sign(&payload)
    }

    /// HMAC-SHA256 서명을 계산합니다.
    fn hmac_sign(&self, payload: &str) -> Result<String, ExchangeError> {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .map_err(|e| ExchangeError::AuthError(format!("HMAC key error: {}", e)))?;

        mac.update(payload.as_bytes());
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
    }

    /// GET 요청에 필요한 모든 인증 헤더를 반환합니다.
    ///
    /// # 인자
    ///
    /// * `query_string` - URL 쿼리 문자열 (앞의 `?` 제외)
    ///
    /// # 반환값
    ///
    /// (timestamp, api_key, recv_window, signature) 튜플
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

    /// POST 요청에 필요한 모든 인증 헤더를 반환합니다.
    ///
    /// # 인자
    ///
    /// * `body` - JSON 본문 문자열
    ///
    /// # 반환값
    ///
    /// 인증 헤더 구조체
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

/// Bybit API 요청을 위한 인증 헤더.
#[derive(Debug, Clone)]
pub struct AuthHeaders {
    /// UTC 타임스탬프 (밀리초 단위).
    pub timestamp: u64,
    /// API 키.
    pub api_key: String,
    /// 수신 윈도우 (밀리초 단위).
    pub recv_window: u64,
    /// HMAC-SHA256 서명.
    pub signature: String,
}

impl AuthHeaders {
    /// API 키 헤더 이름.
    pub const HEADER_API_KEY: &'static str = "X-BAPI-API-KEY";
    /// 타임스탬프 헤더 이름.
    pub const HEADER_TIMESTAMP: &'static str = "X-BAPI-TIMESTAMP";
    /// 서명 헤더 이름.
    pub const HEADER_SIGN: &'static str = "X-BAPI-SIGN";
    /// 수신 윈도우 헤더 이름.
    pub const HEADER_RECV_WINDOW: &'static str = "X-BAPI-RECV-WINDOW";
}

/// 매개변수로부터 쿼리 문자열을 생성합니다.
///
/// # 인자
///
/// * `params` - 키-값 쌍의 반복자
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
        // 합리적인 타임스탬프여야 함 (2020년 이후)
        assert!(ts > 1577836800000);
    }

    #[test]
    fn test_sign_get() {
        let creds = BybitCredentials::new("my_api_key", "my_secret_key");
        let timestamp = 1672531200000u64;
        let query = "category=spot&symbol=BTCUSDT";

        let signature = creds.sign_get(timestamp, query).unwrap();

        // 서명은 64개의 16진수 문자여야 함 (256 bits)
        assert_eq!(signature.len(), 64);
        // 소문자 16진수여야 함
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

        // 동일한 입력은 동일한 서명을 생성해야 함
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
