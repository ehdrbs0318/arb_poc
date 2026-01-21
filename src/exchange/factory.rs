//! Exchange factory for creating exchange instances from configuration.
//!
//! This module provides factory functions for creating exchange adapters
//! from configuration, enabling dynamic exchange instantiation.

use crate::config::ExchangeConfig;
use crate::exchange::adapter::ExchangeAdapter;
use crate::exchange::error::{ExchangeError, ExchangeResult};
use crate::exchange::manager::ExchangeManager;
use crate::exchanges::{BithumbClient, BybitClient, UpbitClient};
use std::sync::Arc;

// ==================== Exchange Adapters ====================

/// Upbit exchange adapter.
#[derive(Debug)]
pub struct UpbitAdapter {
    client: UpbitClient,
}

impl UpbitAdapter {
    /// Creates a new Upbit adapter from a client.
    pub fn new(client: UpbitClient) -> Self {
        Self { client }
    }

    /// Creates a new unauthenticated Upbit adapter.
    pub fn public() -> ExchangeResult<Self> {
        Ok(Self {
            client: UpbitClient::new()?,
        })
    }

    /// Creates a new authenticated Upbit adapter.
    pub fn authenticated(api_key: &str, secret_key: &str) -> ExchangeResult<Self> {
        Ok(Self {
            client: UpbitClient::with_credentials(api_key, secret_key)?,
        })
    }

    /// Creates from exchange config.
    pub fn from_config(config: &ExchangeConfig) -> ExchangeResult<Self> {
        if config.has_credentials() {
            Self::authenticated(&config.api_key, &config.secret_key)
        } else {
            Self::public()
        }
    }
}

#[async_trait::async_trait]
impl ExchangeAdapter for UpbitAdapter {
    fn name(&self) -> &str {
        use crate::exchange::MarketData;
        self.client.name()
    }

    fn is_authenticated(&self) -> bool {
        self.client.credentials.is_some()
    }

    fn native_quote_currency(&self) -> &str {
        "KRW"
    }

    async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<crate::exchange::Ticker>> {
        use crate::exchange::MarketData;
        self.client.get_ticker(markets).await
    }

    async fn get_orderbook(
        &self,
        market: &str,
        depth: Option<u32>,
    ) -> ExchangeResult<crate::exchange::OrderBook> {
        use crate::exchange::MarketData;
        self.client.get_orderbook(market, depth).await
    }

    async fn get_candles(
        &self,
        market: &str,
        interval: crate::exchange::CandleInterval,
        count: u32,
    ) -> ExchangeResult<Vec<crate::exchange::Candle>> {
        use crate::exchange::MarketData;
        self.client.get_candles(market, interval, count).await
    }

    async fn place_order(
        &self,
        request: &crate::exchange::OrderRequest,
    ) -> ExchangeResult<crate::exchange::Order> {
        use crate::exchange::OrderManagement;
        self.client.place_order(request).await
    }

    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<crate::exchange::Order> {
        use crate::exchange::OrderManagement;
        self.client.cancel_order(order_id).await
    }

    async fn get_order(&self, order_id: &str) -> ExchangeResult<crate::exchange::Order> {
        use crate::exchange::OrderManagement;
        self.client.get_order(order_id).await
    }

    async fn get_open_orders(
        &self,
        market: Option<&str>,
    ) -> ExchangeResult<Vec<crate::exchange::Order>> {
        use crate::exchange::OrderManagement;
        self.client.get_open_orders(market).await
    }

    async fn get_balances(&self) -> ExchangeResult<Vec<crate::exchange::Balance>> {
        use crate::exchange::OrderManagement;
        self.client.get_balances().await
    }

    async fn get_balance(&self, currency: &str) -> ExchangeResult<crate::exchange::Balance> {
        use crate::exchange::OrderManagement;
        self.client.get_balance(currency).await
    }
}

/// Bithumb exchange adapter.
#[derive(Debug)]
pub struct BithumbAdapter {
    client: BithumbClient,
}

impl BithumbAdapter {
    /// Creates a new Bithumb adapter from a client.
    pub fn new(client: BithumbClient) -> Self {
        Self { client }
    }

    /// Creates a new unauthenticated Bithumb adapter.
    pub fn public() -> ExchangeResult<Self> {
        Ok(Self {
            client: BithumbClient::new()?,
        })
    }

    /// Creates a new authenticated Bithumb adapter.
    pub fn authenticated(api_key: &str, secret_key: &str) -> ExchangeResult<Self> {
        Ok(Self {
            client: BithumbClient::with_credentials(api_key, secret_key)?,
        })
    }

    /// Creates from exchange config.
    pub fn from_config(config: &ExchangeConfig) -> ExchangeResult<Self> {
        if config.has_credentials() {
            Self::authenticated(&config.api_key, &config.secret_key)
        } else {
            Self::public()
        }
    }
}

#[async_trait::async_trait]
impl ExchangeAdapter for BithumbAdapter {
    fn name(&self) -> &str {
        use crate::exchange::MarketData;
        self.client.name()
    }

    fn is_authenticated(&self) -> bool {
        self.client.credentials.is_some()
    }

    fn native_quote_currency(&self) -> &str {
        "KRW"
    }

    async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<crate::exchange::Ticker>> {
        use crate::exchange::MarketData;
        self.client.get_ticker(markets).await
    }

    async fn get_orderbook(
        &self,
        market: &str,
        depth: Option<u32>,
    ) -> ExchangeResult<crate::exchange::OrderBook> {
        use crate::exchange::MarketData;
        self.client.get_orderbook(market, depth).await
    }

    async fn get_candles(
        &self,
        market: &str,
        interval: crate::exchange::CandleInterval,
        count: u32,
    ) -> ExchangeResult<Vec<crate::exchange::Candle>> {
        use crate::exchange::MarketData;
        self.client.get_candles(market, interval, count).await
    }

    async fn place_order(
        &self,
        request: &crate::exchange::OrderRequest,
    ) -> ExchangeResult<crate::exchange::Order> {
        use crate::exchange::OrderManagement;
        self.client.place_order(request).await
    }

    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<crate::exchange::Order> {
        use crate::exchange::OrderManagement;
        self.client.cancel_order(order_id).await
    }

    async fn get_order(&self, order_id: &str) -> ExchangeResult<crate::exchange::Order> {
        use crate::exchange::OrderManagement;
        self.client.get_order(order_id).await
    }

    async fn get_open_orders(
        &self,
        market: Option<&str>,
    ) -> ExchangeResult<Vec<crate::exchange::Order>> {
        use crate::exchange::OrderManagement;
        self.client.get_open_orders(market).await
    }

    async fn get_balances(&self) -> ExchangeResult<Vec<crate::exchange::Balance>> {
        use crate::exchange::OrderManagement;
        self.client.get_balances().await
    }

    async fn get_balance(&self, currency: &str) -> ExchangeResult<crate::exchange::Balance> {
        use crate::exchange::OrderManagement;
        self.client.get_balance(currency).await
    }
}

/// Bybit exchange adapter.
#[derive(Debug)]
pub struct BybitAdapter {
    client: BybitClient,
}

impl BybitAdapter {
    /// Creates a new Bybit adapter from a client.
    pub fn new(client: BybitClient) -> Self {
        Self { client }
    }

    /// Creates a new unauthenticated Bybit adapter (mainnet).
    pub fn public() -> ExchangeResult<Self> {
        Ok(Self {
            client: BybitClient::new()?,
        })
    }

    /// Creates a new unauthenticated Bybit adapter (testnet).
    pub fn public_testnet() -> ExchangeResult<Self> {
        Ok(Self {
            client: BybitClient::new_testnet()?,
        })
    }

    /// Creates a new authenticated Bybit adapter (mainnet).
    pub fn authenticated(api_key: &str, secret_key: &str) -> ExchangeResult<Self> {
        Ok(Self {
            client: BybitClient::with_credentials(api_key, secret_key)?,
        })
    }

    /// Creates a new authenticated Bybit adapter (testnet).
    pub fn authenticated_testnet(api_key: &str, secret_key: &str) -> ExchangeResult<Self> {
        Ok(Self {
            client: BybitClient::with_credentials_testnet(api_key, secret_key)?,
        })
    }

    /// Creates from exchange config.
    pub fn from_config(config: &ExchangeConfig) -> ExchangeResult<Self> {
        if config.has_credentials() {
            Self::authenticated(&config.api_key, &config.secret_key)
        } else {
            Self::public()
        }
    }

    /// Creates from exchange config (testnet).
    pub fn from_config_testnet(config: &ExchangeConfig) -> ExchangeResult<Self> {
        if config.has_credentials() {
            Self::authenticated_testnet(&config.api_key, &config.secret_key)
        } else {
            Self::public_testnet()
        }
    }
}

#[async_trait::async_trait]
impl ExchangeAdapter for BybitAdapter {
    fn name(&self) -> &str {
        use crate::exchange::MarketData;
        self.client.name()
    }

    fn is_authenticated(&self) -> bool {
        self.client.credentials.is_some()
    }

    fn native_quote_currency(&self) -> &str {
        "USDT"
    }

    async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<crate::exchange::Ticker>> {
        use crate::exchange::MarketData;
        self.client.get_ticker(markets).await
    }

    async fn get_orderbook(
        &self,
        market: &str,
        depth: Option<u32>,
    ) -> ExchangeResult<crate::exchange::OrderBook> {
        use crate::exchange::MarketData;
        self.client.get_orderbook(market, depth).await
    }

    async fn get_candles(
        &self,
        market: &str,
        interval: crate::exchange::CandleInterval,
        count: u32,
    ) -> ExchangeResult<Vec<crate::exchange::Candle>> {
        use crate::exchange::MarketData;
        self.client.get_candles(market, interval, count).await
    }

    async fn place_order(
        &self,
        request: &crate::exchange::OrderRequest,
    ) -> ExchangeResult<crate::exchange::Order> {
        use crate::exchange::OrderManagement;
        self.client.place_order(request).await
    }

    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<crate::exchange::Order> {
        use crate::exchange::OrderManagement;
        self.client.cancel_order(order_id).await
    }

    async fn get_order(&self, order_id: &str) -> ExchangeResult<crate::exchange::Order> {
        use crate::exchange::OrderManagement;
        self.client.get_order(order_id).await
    }

    async fn get_open_orders(
        &self,
        market: Option<&str>,
    ) -> ExchangeResult<Vec<crate::exchange::Order>> {
        use crate::exchange::OrderManagement;
        self.client.get_open_orders(market).await
    }

    async fn get_balances(&self) -> ExchangeResult<Vec<crate::exchange::Balance>> {
        use crate::exchange::OrderManagement;
        self.client.get_balances().await
    }

    async fn get_balance(&self, currency: &str) -> ExchangeResult<crate::exchange::Balance> {
        use crate::exchange::OrderManagement;
        self.client.get_balance(currency).await
    }
}

// ==================== Factory Functions ====================

/// Supported exchange names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExchangeName {
    /// Upbit (Korean exchange).
    Upbit,
    /// Bithumb (Korean exchange).
    Bithumb,
    /// Bybit (Global exchange).
    Bybit,
}

impl ExchangeName {
    /// Returns the string representation of the exchange name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Upbit => "upbit",
            Self::Bithumb => "bithumb",
            Self::Bybit => "bybit",
        }
    }

    /// Returns all supported exchange names.
    pub fn all() -> &'static [Self] {
        &[Self::Upbit, Self::Bithumb, Self::Bybit]
    }

    /// Parses an exchange name from a string (convenience method).
    ///
    /// This is a convenience wrapper around `FromStr::from_str` that returns `Option<Self>`.
    pub fn parse(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

impl std::str::FromStr for ExchangeName {
    type Err = ExchangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "upbit" => Ok(Self::Upbit),
            "bithumb" => Ok(Self::Bithumb),
            "bybit" => Ok(Self::Bybit),
            _ => Err(ExchangeError::ConfigError(format!(
                "Unknown exchange: {}. Supported exchanges: {:?}",
                s,
                Self::all()
            ))),
        }
    }
}

impl std::fmt::Display for ExchangeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Creates an exchange adapter by name.
///
/// # Arguments
///
/// * `name` - Exchange name (case-insensitive)
/// * `config` - Exchange configuration (optional for public-only access)
///
/// # Errors
///
/// Returns an error if the exchange name is not supported or client creation fails.
///
/// # Example
///
/// ```ignore
/// let upbit = create_exchange("upbit", None)?;
/// let bithumb = create_exchange("bithumb", Some(&config.bithumb))?;
/// ```
pub fn create_exchange(
    name: &str,
    config: Option<&ExchangeConfig>,
) -> ExchangeResult<Arc<dyn ExchangeAdapter>> {
    let exchange_name: ExchangeName = name.parse()?;

    let adapter: Arc<dyn ExchangeAdapter> = match exchange_name {
        ExchangeName::Upbit => {
            if let Some(cfg) = config {
                Arc::new(UpbitAdapter::from_config(cfg)?)
            } else {
                Arc::new(UpbitAdapter::public()?)
            }
        }
        ExchangeName::Bithumb => {
            if let Some(cfg) = config {
                Arc::new(BithumbAdapter::from_config(cfg)?)
            } else {
                Arc::new(BithumbAdapter::public()?)
            }
        }
        ExchangeName::Bybit => {
            if let Some(cfg) = config {
                Arc::new(BybitAdapter::from_config(cfg)?)
            } else {
                Arc::new(BybitAdapter::public()?)
            }
        }
    };

    Ok(adapter)
}

/// Creates an exchange adapter using boxed type.
///
/// Same as `create_exchange` but returns `Box<dyn ExchangeAdapter>`.
pub fn create_exchange_boxed(
    name: &str,
    config: Option<&ExchangeConfig>,
) -> ExchangeResult<Box<dyn ExchangeAdapter>> {
    let exchange_name: ExchangeName = name.parse()?;

    let adapter: Box<dyn ExchangeAdapter> = match exchange_name {
        ExchangeName::Upbit => {
            if let Some(cfg) = config {
                Box::new(UpbitAdapter::from_config(cfg)?)
            } else {
                Box::new(UpbitAdapter::public()?)
            }
        }
        ExchangeName::Bithumb => {
            if let Some(cfg) = config {
                Box::new(BithumbAdapter::from_config(cfg)?)
            } else {
                Box::new(BithumbAdapter::public()?)
            }
        }
        ExchangeName::Bybit => {
            if let Some(cfg) = config {
                Box::new(BybitAdapter::from_config(cfg)?)
            } else {
                Box::new(BybitAdapter::public()?)
            }
        }
    };

    Ok(adapter)
}

/// Extension trait for ExchangeManager to simplify registration from config.
pub trait ExchangeManagerExt {
    /// Registers an exchange from configuration.
    ///
    /// # Arguments
    ///
    /// * `name` - Exchange name
    /// * `config` - Exchange configuration
    fn register_from_config(
        &mut self,
        name: &str,
        config: Option<&ExchangeConfig>,
    ) -> ExchangeResult<()>;

    /// Registers all supported exchanges from the application config.
    ///
    /// # Arguments
    ///
    /// * `config` - Application configuration
    fn register_all_from_config(&mut self, config: &crate::config::Config) -> ExchangeResult<()>;
}

impl ExchangeManagerExt for ExchangeManager {
    fn register_from_config(
        &mut self,
        name: &str,
        config: Option<&ExchangeConfig>,
    ) -> ExchangeResult<()> {
        let adapter = create_exchange(name, config)?;
        self.register_arc(name, adapter);
        Ok(())
    }

    fn register_all_from_config(&mut self, config: &crate::config::Config) -> ExchangeResult<()> {
        // Register Upbit
        self.register_from_config("upbit", Some(&config.upbit))?;

        // Register Bithumb
        self.register_from_config("bithumb", Some(&config.bithumb))?;

        // Register Bybit
        self.register_from_config("bybit", Some(&config.bybit))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_name_from_str() {
        assert_eq!("upbit".parse::<ExchangeName>().ok(), Some(ExchangeName::Upbit));
        assert_eq!("UPBIT".parse::<ExchangeName>().ok(), Some(ExchangeName::Upbit));
        assert_eq!(
            "bithumb".parse::<ExchangeName>().ok(),
            Some(ExchangeName::Bithumb)
        );
        assert_eq!("bybit".parse::<ExchangeName>().ok(), Some(ExchangeName::Bybit));
        assert!("unknown".parse::<ExchangeName>().is_err());
    }

    #[test]
    fn test_exchange_name_as_str() {
        assert_eq!(ExchangeName::Upbit.as_str(), "upbit");
        assert_eq!(ExchangeName::Bithumb.as_str(), "bithumb");
        assert_eq!(ExchangeName::Bybit.as_str(), "bybit");
    }

    #[test]
    fn test_create_exchange_unknown() {
        let result = create_exchange("unknown", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_exchange_upbit() {
        let result = create_exchange("upbit", None);
        assert!(result.is_ok());
        let adapter = result.unwrap();
        assert_eq!(adapter.name(), "Upbit");
        assert!(!adapter.is_authenticated());
    }

    #[test]
    fn test_create_exchange_bithumb() {
        let result = create_exchange("bithumb", None);
        assert!(result.is_ok());
        let adapter = result.unwrap();
        assert_eq!(adapter.name(), "Bithumb");
    }

    #[test]
    fn test_create_exchange_bybit() {
        let result = create_exchange("bybit", None);
        assert!(result.is_ok());
        let adapter = result.unwrap();
        assert_eq!(adapter.name(), "Bybit");
    }

    #[test]
    fn test_create_exchange_with_credentials() {
        let config = ExchangeConfig {
            api_key: "test_key".to_string(),
            secret_key: "test_secret".to_string(),
        };

        let result = create_exchange("upbit", Some(&config));
        assert!(result.is_ok());
        let adapter = result.unwrap();
        assert!(adapter.is_authenticated());
    }
}
