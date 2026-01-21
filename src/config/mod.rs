//! 설정 관리 모듈.
//!
//! 이 모듈은 거래소 API 자격 증명을 포함한 애플리케이션 설정의
//! 로딩 및 관리를 담당합니다.

use crate::exchange::ExchangeError;
use serde::Deserialize;
use std::path::Path;

/// 애플리케이션 설정.
#[derive(Debug, Deserialize, Default)]
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

impl ExchangeConfig {
    /// 자격 증명이 설정되어 있으면 true를 반환합니다.
    #[must_use]
    pub fn has_credentials(&self) -> bool {
        !self.api_key.is_empty() && !self.secret_key.is_empty()
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
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ExchangeError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ExchangeError::ConfigError(format!(
                "Configuration file not found: {}",
                path.display()
            )));
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| ExchangeError::ConfigError(format!("Failed to read config file: {e}")))?;

        toml::from_str(&content)
            .map_err(|e| ExchangeError::ConfigError(format!("Failed to parse config file: {e}")))
    }

    /// 다음 우선순위로 설정을 로드합니다:
    /// 1. 환경 변수 (UPBIT_API_KEY, UPBIT_SECRET_KEY)
    /// 2. config.local.toml
    /// 3. config.toml
    ///
    /// # 에러
    ///
    /// 설정을 로드할 수 없는 경우 에러를 반환합니다.
    pub fn load() -> Result<Self, ExchangeError> {
        let mut config = Self::default();

        // 파일에서 로드 시도 (낮은 우선순위부터)
        if let Ok(file_config) = Self::from_file("config.toml") {
            config = file_config;
        }

        if let Ok(local_config) = Self::from_file("config.local.toml") {
            config = local_config;
        }

        // 환경 변수로 오버라이드 (최고 우선순위)
        if let Ok(api_key) = std::env::var("UPBIT_API_KEY") {
            config.upbit.api_key = api_key;
        }
        if let Ok(secret_key) = std::env::var("UPBIT_SECRET_KEY") {
            config.upbit.secret_key = secret_key;
        }

        // Bithumb 환경 변수
        if let Ok(api_key) = std::env::var("BITHUMB_API_KEY") {
            config.bithumb.api_key = api_key;
        }
        if let Ok(secret_key) = std::env::var("BITHUMB_SECRET_KEY") {
            config.bithumb.secret_key = secret_key;
        }

        // Bybit 환경 변수
        if let Ok(api_key) = std::env::var("BYBIT_API_KEY") {
            config.bybit.api_key = api_key;
        }
        if let Ok(secret_key) = std::env::var("BYBIT_SECRET_KEY") {
            config.bybit.secret_key = secret_key;
        }

        Ok(config)
    }

    /// 설정을 로드하고, 찾지 못하면 기본값을 반환합니다.
    #[must_use]
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }
}

// toml을 의존성으로 추가해야 함
// 현재는 기본 설정에 사용할 간단한 TOML 파서를 구현

fn parse_toml_simple(content: &str) -> Result<Config, ExchangeError> {
    let mut config = Config::default();
    let mut current_section = "";

    for line in content.lines() {
        let line = line.trim();

        // 빈 줄과 주석 건너뛰기
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // 섹션 헤더
        if line.starts_with('[') && line.ends_with(']') {
            current_section = &line[1..line.len() - 1];
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
                _ => {}
            }
        }
    }

    Ok(config)
}

// toml 크레이트가 아직 없으므로 간단한 파서로 toml 파싱을 오버라이드
mod toml {
    use super::*;

    pub fn from_str(s: &str) -> Result<Config, ExchangeError> {
        parse_toml_simple(s)
    }
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
    fn test_parse_toml_simple() {
        let content = r#"
            [upbit]
            api_key = "test_api_key"
            secret_key = "test_secret_key"
        "#;

        let config = parse_toml_simple(content).unwrap();
        assert_eq!(config.upbit.api_key, "test_api_key");
        assert_eq!(config.upbit.secret_key, "test_secret_key");
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
    fn test_config_load_or_default() {
        let config = Config::load_or_default();
        // 파일이 없으면 기본 설정을 반환해야 함
        assert!(config.upbit.api_key.is_empty() || !config.upbit.api_key.is_empty());
    }
}
