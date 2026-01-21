//! Exchange manager for coordinating multiple exchanges.
//!
//! This module provides the `ExchangeManager` struct for managing multiple
//! exchange instances and providing unified access to them.

use crate::exchange::adapter::ExchangeAdapter;
use crate::exchange::error::{ExchangeError, ExchangeResult};
use std::collections::HashMap;
use std::sync::Arc;

/// Manager for multiple exchange instances.
///
/// The `ExchangeManager` provides a centralized way to manage and access
/// multiple exchanges at runtime. It supports dynamic registration and
/// lookup of exchange adapters.
///
/// # Example
///
/// ```ignore
/// use arb_poc::exchange::ExchangeManager;
/// use arb_poc::exchanges::{UpbitClient, BithumbClient};
///
/// let mut manager = ExchangeManager::new();
///
/// // Register exchanges
/// manager.register("upbit", UpbitAdapter::new(UpbitClient::new()?));
/// manager.register("bithumb", BithumbAdapter::new(BithumbClient::new()?));
///
/// // Access exchanges by name
/// let upbit = manager.get("upbit").unwrap();
/// let ticker = upbit.get_ticker(&["KRW-BTC"]).await?;
/// ```
#[derive(Default)]
pub struct ExchangeManager {
    exchanges: HashMap<String, Arc<dyn ExchangeAdapter>>,
}

impl ExchangeManager {
    /// Creates a new empty exchange manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            exchanges: HashMap::new(),
        }
    }

    /// Creates a new exchange manager with the given capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Initial capacity for the exchange registry
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            exchanges: HashMap::with_capacity(capacity),
        }
    }

    /// Registers an exchange with the given name.
    ///
    /// If an exchange with the same name already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `name` - Name to register the exchange under (case-insensitive)
    /// * `exchange` - The exchange adapter to register
    ///
    /// # Returns
    ///
    /// The previously registered exchange with the same name, if any.
    pub fn register(
        &mut self,
        name: impl Into<String>,
        exchange: impl ExchangeAdapter + 'static,
    ) -> Option<Arc<dyn ExchangeAdapter>> {
        let name = name.into().to_lowercase();
        self.exchanges.insert(name, Arc::new(exchange))
    }

    /// Registers an exchange wrapped in Arc.
    ///
    /// # Arguments
    ///
    /// * `name` - Name to register the exchange under (case-insensitive)
    /// * `exchange` - The exchange adapter wrapped in Arc
    pub fn register_arc(
        &mut self,
        name: impl Into<String>,
        exchange: Arc<dyn ExchangeAdapter>,
    ) -> Option<Arc<dyn ExchangeAdapter>> {
        let name = name.into().to_lowercase();
        self.exchanges.insert(name, exchange)
    }

    /// Removes an exchange from the manager.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the exchange to remove (case-insensitive)
    ///
    /// # Returns
    ///
    /// The removed exchange, if it existed.
    pub fn unregister(&mut self, name: &str) -> Option<Arc<dyn ExchangeAdapter>> {
        self.exchanges.remove(&name.to_lowercase())
    }

    /// Gets a reference to an exchange by name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the exchange (case-insensitive)
    ///
    /// # Returns
    ///
    /// A reference to the exchange adapter, if found.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<Arc<dyn ExchangeAdapter>> {
        self.exchanges.get(&name.to_lowercase()).cloned()
    }

    /// Gets an exchange by name, returning an error if not found.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the exchange (case-insensitive)
    ///
    /// # Errors
    ///
    /// Returns an error if the exchange is not registered.
    pub fn get_or_error(&self, name: &str) -> ExchangeResult<Arc<dyn ExchangeAdapter>> {
        self.get(name).ok_or_else(|| {
            ExchangeError::ConfigError(format!("Exchange not registered: {}", name))
        })
    }

    /// Returns true if an exchange with the given name is registered.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the exchange (case-insensitive)
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.exchanges.contains_key(&name.to_lowercase())
    }

    /// Returns the names of all registered exchanges.
    #[must_use]
    pub fn list_exchanges(&self) -> Vec<&str> {
        self.exchanges.keys().map(String::as_str).collect()
    }

    /// Returns the number of registered exchanges.
    #[must_use]
    pub fn len(&self) -> usize {
        self.exchanges.len()
    }

    /// Returns true if no exchanges are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.exchanges.is_empty()
    }

    /// Returns an iterator over all registered exchanges.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Arc<dyn ExchangeAdapter>)> {
        self.exchanges
            .iter()
            .map(|(name, exchange)| (name.as_str(), exchange))
    }

    /// Returns only authenticated exchanges.
    pub fn authenticated(&self) -> impl Iterator<Item = (&str, &Arc<dyn ExchangeAdapter>)> {
        self.iter().filter(|(_, ex)| ex.is_authenticated())
    }

    /// Returns exchanges filtered by quote currency.
    ///
    /// # Arguments
    ///
    /// * `quote` - The quote currency to filter by (e.g., "KRW", "USDT")
    pub fn by_quote_currency(
        &self,
        quote: &str,
    ) -> impl Iterator<Item = (&str, &Arc<dyn ExchangeAdapter>)> {
        let quote = quote.to_uppercase();
        self.iter()
            .filter(move |(_, ex)| ex.native_quote_currency().to_uppercase() == quote)
    }
}

impl std::fmt::Debug for ExchangeManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExchangeManager")
            .field("exchanges", &self.list_exchanges())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock adapter for testing
    #[derive(Debug)]
    struct MockAdapter {
        name: String,
        authenticated: bool,
        quote_currency: String,
    }

    impl MockAdapter {
        fn new(name: &str, authenticated: bool, quote_currency: &str) -> Self {
            Self {
                name: name.to_string(),
                authenticated,
                quote_currency: quote_currency.to_string(),
            }
        }
    }

    #[async_trait::async_trait]
    impl ExchangeAdapter for MockAdapter {
        fn name(&self) -> &str {
            &self.name
        }

        fn is_authenticated(&self) -> bool {
            self.authenticated
        }

        fn native_quote_currency(&self) -> &str {
            &self.quote_currency
        }

        async fn get_ticker(
            &self,
            _markets: &[&str],
        ) -> ExchangeResult<Vec<crate::exchange::Ticker>> {
            Ok(vec![])
        }

        async fn get_orderbook(
            &self,
            _market: &str,
            _depth: Option<u32>,
        ) -> ExchangeResult<crate::exchange::OrderBook> {
            Err(ExchangeError::InternalError("Not implemented".to_string()))
        }

        async fn get_candles(
            &self,
            _market: &str,
            _interval: crate::exchange::CandleInterval,
            _count: u32,
        ) -> ExchangeResult<Vec<crate::exchange::Candle>> {
            Ok(vec![])
        }

        async fn place_order(
            &self,
            _request: &crate::exchange::OrderRequest,
        ) -> ExchangeResult<crate::exchange::Order> {
            Err(ExchangeError::AuthError("Not authenticated".to_string()))
        }

        async fn cancel_order(&self, _order_id: &str) -> ExchangeResult<crate::exchange::Order> {
            Err(ExchangeError::AuthError("Not authenticated".to_string()))
        }

        async fn get_order(&self, _order_id: &str) -> ExchangeResult<crate::exchange::Order> {
            Err(ExchangeError::AuthError("Not authenticated".to_string()))
        }

        async fn get_open_orders(
            &self,
            _market: Option<&str>,
        ) -> ExchangeResult<Vec<crate::exchange::Order>> {
            Ok(vec![])
        }

        async fn get_balances(&self) -> ExchangeResult<Vec<crate::exchange::Balance>> {
            Ok(vec![])
        }

        async fn get_balance(&self, _currency: &str) -> ExchangeResult<crate::exchange::Balance> {
            Err(ExchangeError::InvalidParameter("Not found".to_string()))
        }
    }

    #[test]
    fn test_manager_new() {
        let manager = ExchangeManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_manager_register_and_get() {
        let mut manager = ExchangeManager::new();

        manager.register("Upbit", MockAdapter::new("Upbit", false, "KRW"));
        manager.register("bithumb", MockAdapter::new("Bithumb", true, "KRW"));

        assert_eq!(manager.len(), 2);
        assert!(manager.contains("upbit"));
        assert!(manager.contains("UPBIT"));
        assert!(manager.contains("Bithumb"));

        let upbit = manager.get("upbit").unwrap();
        assert_eq!(upbit.name(), "Upbit");

        let bithumb = manager.get("BITHUMB").unwrap();
        assert_eq!(bithumb.name(), "Bithumb");
    }

    #[test]
    fn test_manager_unregister() {
        let mut manager = ExchangeManager::new();

        manager.register("upbit", MockAdapter::new("Upbit", false, "KRW"));
        assert!(manager.contains("upbit"));

        let removed = manager.unregister("upbit");
        assert!(removed.is_some());
        assert!(!manager.contains("upbit"));
    }

    #[test]
    fn test_manager_list_exchanges() {
        let mut manager = ExchangeManager::new();

        manager.register("upbit", MockAdapter::new("Upbit", false, "KRW"));
        manager.register("bithumb", MockAdapter::new("Bithumb", false, "KRW"));
        manager.register("bybit", MockAdapter::new("Bybit", false, "USDT"));

        let names = manager.list_exchanges();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"upbit"));
        assert!(names.contains(&"bithumb"));
        assert!(names.contains(&"bybit"));
    }

    #[test]
    fn test_manager_authenticated() {
        let mut manager = ExchangeManager::new();

        manager.register("upbit", MockAdapter::new("Upbit", false, "KRW"));
        manager.register("bithumb", MockAdapter::new("Bithumb", true, "KRW"));
        manager.register("bybit", MockAdapter::new("Bybit", true, "USDT"));

        let authenticated: Vec<_> = manager.authenticated().collect();
        assert_eq!(authenticated.len(), 2);
    }

    #[test]
    fn test_manager_by_quote_currency() {
        let mut manager = ExchangeManager::new();

        manager.register("upbit", MockAdapter::new("Upbit", false, "KRW"));
        manager.register("bithumb", MockAdapter::new("Bithumb", false, "KRW"));
        manager.register("bybit", MockAdapter::new("Bybit", false, "USDT"));

        let krw_exchanges: Vec<_> = manager.by_quote_currency("KRW").collect();
        assert_eq!(krw_exchanges.len(), 2);

        let usdt_exchanges: Vec<_> = manager.by_quote_currency("usdt").collect();
        assert_eq!(usdt_exchanges.len(), 1);
    }

    #[test]
    fn test_manager_get_or_error() {
        let mut manager = ExchangeManager::new();
        manager.register("upbit", MockAdapter::new("Upbit", false, "KRW"));

        assert!(manager.get_or_error("upbit").is_ok());
        assert!(manager.get_or_error("nonexistent").is_err());
    }
}
