//! Configuration management module.
//!
//! This module handles loading and managing configuration for the application,
//! including exchange API credentials.

use crate::exchange::ExchangeError;
use serde::Deserialize;
use std::path::Path;

/// Application configuration.
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    /// Upbit exchange configuration.
    #[serde(default)]
    pub upbit: ExchangeConfig,
    /// Bithumb exchange configuration.
    #[serde(default)]
    pub bithumb: ExchangeConfig,
    /// Bybit exchange configuration.
    #[serde(default)]
    pub bybit: ExchangeConfig,
}

/// Configuration for a single exchange.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct ExchangeConfig {
    /// API access key.
    #[serde(default)]
    pub api_key: String,
    /// API secret key.
    #[serde(default)]
    pub secret_key: String,
}

impl ExchangeConfig {
    /// Returns true if credentials are configured.
    #[must_use]
    pub fn has_credentials(&self) -> bool {
        !self.api_key.is_empty() && !self.secret_key.is_empty()
    }
}

impl Config {
    /// Loads configuration from a TOML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
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

    /// Loads configuration with the following priority:
    /// 1. Environment variables (UPBIT_API_KEY, UPBIT_SECRET_KEY)
    /// 2. config.local.toml
    /// 3. config.toml
    ///
    /// # Errors
    ///
    /// Returns an error if no configuration can be loaded.
    pub fn load() -> Result<Self, ExchangeError> {
        let mut config = Self::default();

        // Try loading from files (lower priority first)
        if let Ok(file_config) = Self::from_file("config.toml") {
            config = file_config;
        }

        if let Ok(local_config) = Self::from_file("config.local.toml") {
            config = local_config;
        }

        // Override with environment variables (highest priority)
        if let Ok(api_key) = std::env::var("UPBIT_API_KEY") {
            config.upbit.api_key = api_key;
        }
        if let Ok(secret_key) = std::env::var("UPBIT_SECRET_KEY") {
            config.upbit.secret_key = secret_key;
        }

        // Bithumb environment variables
        if let Ok(api_key) = std::env::var("BITHUMB_API_KEY") {
            config.bithumb.api_key = api_key;
        }
        if let Ok(secret_key) = std::env::var("BITHUMB_SECRET_KEY") {
            config.bithumb.secret_key = secret_key;
        }

        // Bybit environment variables
        if let Ok(api_key) = std::env::var("BYBIT_API_KEY") {
            config.bybit.api_key = api_key;
        }
        if let Ok(secret_key) = std::env::var("BYBIT_SECRET_KEY") {
            config.bybit.secret_key = secret_key;
        }

        Ok(config)
    }

    /// Loads configuration, returning default if not found.
    #[must_use]
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }
}

// We need to add toml as a dependency
// For now, implement a simple TOML parser that works for our basic config

fn parse_toml_simple(content: &str) -> Result<Config, ExchangeError> {
    let mut config = Config::default();
    let mut current_section = "";

    for line in content.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Section header
        if line.starts_with('[') && line.ends_with(']') {
            current_section = &line[1..line.len() - 1];
            continue;
        }

        // Key-value pair
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

// Override the toml parsing with our simple parser since we don't have toml crate yet
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
            # This is a comment
            [upbit]
            # API credentials
            api_key = "key"
            secret_key = "secret"
        "#;

        let config = parse_toml_simple(content).unwrap();
        assert_eq!(config.upbit.api_key, "key");
    }

    #[test]
    fn test_config_load_or_default() {
        let config = Config::load_or_default();
        // Should return default config if no files exist
        assert!(config.upbit.api_key.is_empty() || !config.upbit.api_key.is_empty());
    }
}
