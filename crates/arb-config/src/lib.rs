//! 설정 관리 모듈.
//!
//! 이 모듈은 거래소 API 자격 증명 및 Telegram 설정을 포함한
//! 애플리케이션 설정의 로딩 및 관리를 담당합니다.

mod error;

pub use error::ConfigError;

use serde::Deserialize;
use std::path::Path;
use tracing::{debug, info, warn};

/// 애플리케이션 설정.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    /// Upbit 거래소 설정.
    #[serde(default)]
    pub upbit: ExchangeConfig,
    /// Bithumb 거래소 설정.
    #[serde(default)]
    pub bithumb: ExchangeConfig,
    /// Bybit 거래소 설정.
    #[serde(default)]
    pub bybit: ExchangeConfig,
    /// Telegram 설정.
    #[serde(default)]
    pub telegram: TelegramConfig,
    /// 데이터베이스 설정.
    #[serde(default)]
    pub database: DatabaseConfig,
}

/// 단일 거래소 설정.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct ExchangeConfig {
    /// API 액세스 키.
    #[serde(default)]
    pub api_key: String,
    /// API 시크릿 키.
    #[serde(default)]
    pub secret_key: String,
}

/// Telegram 설정.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct TelegramConfig {
    /// Telegram Bot 토큰.
    #[serde(default)]
    pub bot_token: String,
    /// Telegram Chat ID.
    #[serde(default)]
    pub chat_id: String,
}

/// 데이터베이스 설정.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct DatabaseConfig {
    /// 데이터베이스 접속 URL (e.g., "mysql://user:pass@localhost:3306/arb").
    #[serde(default)]
    pub url: String,
}

impl DatabaseConfig {
    /// DB URL이 설정되어 있으면 true를 반환합니다.
    #[must_use]
    pub fn is_configured(&self) -> bool {
        !self.url.is_empty()
    }
}

impl ExchangeConfig {
    /// 자격 증명이 설정되어 있으면 true를 반환합니다.
    #[must_use]
    pub fn has_credentials(&self) -> bool {
        !self.api_key.is_empty() && !self.secret_key.is_empty()
    }
}

impl TelegramConfig {
    /// Telegram 설정이 유효하면 true를 반환합니다.
    #[must_use]
    pub fn is_configured(&self) -> bool {
        !self.bot_token.is_empty() && !self.chat_id.is_empty()
    }
}

impl Config {
    /// TOML 파일에서 설정을 로드합니다.
    ///
    /// # 인자
    ///
    /// * `path` - 설정 파일 경로
    ///
    /// # 에러
    ///
    /// 파일을 읽거나 파싱할 수 없는 경우 에러를 반환합니다.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();

        if !path.exists() {
            warn!(path = %path.display(), "설정 파일 미발견");
            return Err(ConfigError::FileNotFound(path.display().to_string()));
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::ReadError(format!("Failed to read config file: {e}")))?;

        let config = parse_toml_simple(&content)?;
        info!(path = %path.display(), "설정 파일 로드 완료");
        Ok(config)
    }

    /// 다음 우선순위로 설정을 로드합니다:
    /// 1. 환경 변수 (UPBIT_API_KEY, UPBIT_SECRET_KEY, TELEGRAM_BOT_TOKEN 등)
    /// 2. config.local.toml
    /// 3. config.toml
    ///
    /// # 에러
    ///
    /// 설정을 로드할 수 없는 경우 에러를 반환합니다.
    pub fn load() -> Result<Self, ConfigError> {
        let mut config = Self::default();

        // 파일에서 로드 시도 (낮은 우선순위부터)
        if let Ok(file_config) = Self::from_file("config.toml") {
            debug!("config.toml 로드 성공");
            config = file_config;
        }

        if let Ok(local_config) = Self::from_file("config.local.toml") {
            debug!("config.local.toml 로드 성공 (우선 적용)");
            config = local_config;
        }

        // 환경 변수로 오버라이드 (최고 우선순위)
        // Upbit
        if let Ok(api_key) = std::env::var("UPBIT_API_KEY") {
            config.upbit.api_key = api_key;
        }
        if let Ok(secret_key) = std::env::var("UPBIT_SECRET_KEY") {
            config.upbit.secret_key = secret_key;
        }

        // Bithumb
        if let Ok(api_key) = std::env::var("BITHUMB_API_KEY") {
            config.bithumb.api_key = api_key;
        }
        if let Ok(secret_key) = std::env::var("BITHUMB_SECRET_KEY") {
            config.bithumb.secret_key = secret_key;
        }

        // Bybit
        if let Ok(api_key) = std::env::var("BYBIT_API_KEY") {
            config.bybit.api_key = api_key;
        }
        if let Ok(secret_key) = std::env::var("BYBIT_SECRET_KEY") {
            config.bybit.secret_key = secret_key;
        }

        // Telegram
        if let Ok(bot_token) = std::env::var("TELEGRAM_BOT_TOKEN") {
            config.telegram.bot_token = bot_token;
        }
        if let Ok(chat_id) = std::env::var("TELEGRAM_CHAT_ID") {
            config.telegram.chat_id = chat_id;
        }

        // Database
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            config.database.url = database_url;
        }

        debug!(
            upbit_configured = config.upbit.has_credentials(),
            bithumb_configured = config.bithumb.has_credentials(),
            bybit_configured = config.bybit.has_credentials(),
            telegram_configured = config.telegram.is_configured(),
            database_configured = config.database.is_configured(),
            "설정 로드 완료: 자격 증명 상태"
        );

        Ok(config)
    }

    /// 설정을 로드하고, 찾지 못하면 기본값을 반환합니다.
    #[must_use]
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }
}

/// 간단한 TOML 파서.
///
/// toml 크레이트를 추가하지 않고 기본적인 TOML 파싱을 지원합니다.
fn parse_toml_simple(content: &str) -> Result<Config, ConfigError> {
    let mut config = Config::default();
    let mut current_section = "";

    for line in content.lines() {
        let line = line.trim();

        // 빈 줄과 주석 건너뛰기
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // 섹션 헤더 - UTF-8 안전한 파싱
        if let Some(section_name) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            current_section = section_name;
            continue;
        }

        // 키-값 쌍
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');

            match (current_section, key) {
                ("upbit", "api_key") => config.upbit.api_key = value.to_string(),
                ("upbit", "secret_key") => config.upbit.secret_key = value.to_string(),
                ("bithumb", "api_key") => config.bithumb.api_key = value.to_string(),
                ("bithumb", "secret_key") => config.bithumb.secret_key = value.to_string(),
                ("bybit", "api_key") => config.bybit.api_key = value.to_string(),
                ("bybit", "secret_key") => config.bybit.secret_key = value.to_string(),
                ("telegram", "bot_token") => config.telegram.bot_token = value.to_string(),
                ("telegram", "chat_id") => config.telegram.chat_id = value.to_string(),
                ("database", "url") => config.database.url = value.to_string(),
                _ => {}
            }
        }
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_config_has_credentials() {
        let empty = ExchangeConfig::default();
        assert!(!empty.has_credentials());

        let partial = ExchangeConfig {
            api_key: "key".to_string(),
            secret_key: String::new(),
        };
        assert!(!partial.has_credentials());

        let full = ExchangeConfig {
            api_key: "key".to_string(),
            secret_key: "secret".to_string(),
        };
        assert!(full.has_credentials());
    }

    #[test]
    fn test_telegram_config_is_configured() {
        let empty = TelegramConfig::default();
        assert!(!empty.is_configured());

        let partial = TelegramConfig {
            bot_token: "token".to_string(),
            chat_id: String::new(),
        };
        assert!(!partial.is_configured());

        let full = TelegramConfig {
            bot_token: "token".to_string(),
            chat_id: "12345".to_string(),
        };
        assert!(full.is_configured());
    }

    #[test]
    fn test_parse_toml_simple() {
        let content = r#"
            [upbit]
            api_key = "test_api_key"
            secret_key = "test_secret_key"

            [telegram]
            bot_token = "123456:ABC-DEF"
            chat_id = "-100123456789"
        "#;

        let config = parse_toml_simple(content).unwrap();
        assert_eq!(config.upbit.api_key, "test_api_key");
        assert_eq!(config.upbit.secret_key, "test_secret_key");
        assert_eq!(config.telegram.bot_token, "123456:ABC-DEF");
        assert_eq!(config.telegram.chat_id, "-100123456789");
    }

    #[test]
    fn test_parse_toml_with_comments() {
        let content = r#"
            # 이것은 주석입니다
            [upbit]
            # API 자격 증명
            api_key = "key"
            secret_key = "secret"
        "#;

        let config = parse_toml_simple(content).unwrap();
        assert_eq!(config.upbit.api_key, "key");
    }

    #[test]
    fn test_database_config_is_configured() {
        let empty = DatabaseConfig::default();
        assert!(!empty.is_configured());

        let configured = DatabaseConfig {
            url: "mysql://user:pass@localhost/arb".to_string(),
        };
        assert!(configured.is_configured());
    }

    #[test]
    fn test_parse_toml_database() {
        let content = r#"
            [database]
            url = "mysql://user:pass@localhost:3306/arb"
        "#;

        let config = parse_toml_simple(content).unwrap();
        assert_eq!(config.database.url, "mysql://user:pass@localhost:3306/arb");
        assert!(config.database.is_configured());
    }

    #[test]
    fn test_parse_toml_database_missing() {
        let content = r#"
            [upbit]
            api_key = "key"
            secret_key = "secret"
        "#;

        let config = parse_toml_simple(content).unwrap();
        assert!(!config.database.is_configured());
    }

    #[test]
    fn test_config_load_or_default() {
        let config = Config::load_or_default();
        // 파일이 없어도 패닉 없이 Config를 반환해야 함
        // 기본 설정의 경우 빈 문자열이거나 파일에서 로드한 값이 있을 수 있음
        // 반환된 Config가 유효한 구조인지 확인
        assert!(
            config.upbit.api_key.len() < 1000,
            "API key should be reasonably sized"
        );
        assert!(
            config.telegram.bot_token.len() < 1000,
            "Bot token should be reasonably sized"
        );
    }
}
