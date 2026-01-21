//! Object-safe exchange adapter trait.
//!
//! This module provides an object-safe version of the exchange traits,
//! enabling dynamic dispatch and runtime exchange selection.

use crate::exchange::error::ExchangeResult;
use crate::exchange::types::{
    Balance, Candle, CandleInterval, Order, OrderBook, OrderRequest, Ticker,
};
use async_trait::async_trait;
use std::fmt::Debug;

/// Object-safe adapter trait for exchange operations.
///
/// This trait uses `async_trait` to enable object safety for async methods,
/// allowing exchanges to be used as `Box<dyn ExchangeAdapter>` or `Arc<dyn ExchangeAdapter>`.
///
/// # Example
///
/// ```ignore
/// use std::sync::Arc;
///
/// let exchange: Arc<dyn ExchangeAdapter> = Arc::new(UpbitAdapter::new(client));
/// let tickers = exchange.get_ticker(&["KRW-BTC"]).await?;
/// ```
#[async_trait]
pub trait ExchangeAdapter: Send + Sync + Debug {
    /// Returns the exchange name (e.g., "Upbit", "Bithumb", "Bybit").
    fn name(&self) -> &str;

    /// Returns whether the exchange client is authenticated.
    fn is_authenticated(&self) -> bool;

    /// Returns the exchange's native quote currency (e.g., "KRW" for Korean exchanges, "USDT" for Bybit).
    fn native_quote_currency(&self) -> &str;

    // ==================== Market Data Operations ====================

    /// Fetches the current ticker for one or more markets.
    ///
    /// # Arguments
    ///
    /// * `markets` - A slice of market codes (e.g., ["KRW-BTC", "KRW-ETH"])
    ///
    /// # Note
    ///
    /// Market codes should be in the internal format `{QUOTE}-{BASE}` (e.g., "KRW-BTC").
    /// The adapter handles conversion to the exchange's native format.
    async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<Ticker>>;

    /// Fetches the order book for a market.
    ///
    /// # Arguments
    ///
    /// * `market` - Market code (e.g., "KRW-BTC")
    /// * `depth` - Number of levels to fetch (optional, default varies by exchange)
    async fn get_orderbook(&self, market: &str, depth: Option<u32>) -> ExchangeResult<OrderBook>;

    /// Fetches candle data for a market.
    ///
    /// # Arguments
    ///
    /// * `market` - Market code (e.g., "KRW-BTC")
    /// * `interval` - Candle interval
    /// * `count` - Number of candles to fetch
    async fn get_candles(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
    ) -> ExchangeResult<Vec<Candle>>;

    // ==================== Order Management Operations ====================

    /// Places a new order.
    ///
    /// # Arguments
    ///
    /// * `request` - Order request parameters
    async fn place_order(&self, request: &OrderRequest) -> ExchangeResult<Order>;

    /// Cancels an existing order.
    ///
    /// # Arguments
    ///
    /// * `order_id` - Exchange-assigned order ID
    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<Order>;

    /// Fetches an order by ID.
    ///
    /// # Arguments
    ///
    /// * `order_id` - Exchange-assigned order ID
    async fn get_order(&self, order_id: &str) -> ExchangeResult<Order>;

    /// Fetches open orders for a market.
    ///
    /// # Arguments
    ///
    /// * `market` - Market code (optional, None for all markets)
    async fn get_open_orders(&self, market: Option<&str>) -> ExchangeResult<Vec<Order>>;

    /// Fetches account balances.
    async fn get_balances(&self) -> ExchangeResult<Vec<Balance>>;

    /// Fetches balance for a specific currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency code (e.g., "BTC", "KRW")
    async fn get_balance(&self, currency: &str) -> ExchangeResult<Balance>;
}

/// Wrapper to adapt existing clients implementing MarketData + OrderManagement to ExchangeAdapter.
///
/// This macro generates an adapter struct that implements ExchangeAdapter for a given client type.
#[macro_export]
macro_rules! impl_exchange_adapter {
    ($adapter_name:ident, $client_type:ty, $exchange_name:expr, $quote_currency:expr) => {
        /// Adapter wrapper for the exchange client.
        #[derive(Debug)]
        pub struct $adapter_name {
            client: $client_type,
        }

        impl $adapter_name {
            /// Creates a new adapter from the client.
            pub fn new(client: $client_type) -> Self {
                Self { client }
            }

            /// Returns a reference to the underlying client.
            pub fn client(&self) -> &$client_type {
                &self.client
            }
        }

        #[async_trait::async_trait]
        impl $crate::exchange::adapter::ExchangeAdapter for $adapter_name {
            fn name(&self) -> &str {
                use $crate::exchange::MarketData;
                self.client.name()
            }

            fn is_authenticated(&self) -> bool {
                self.client.credentials.is_some()
            }

            fn native_quote_currency(&self) -> &str {
                $quote_currency
            }

            async fn get_ticker(
                &self,
                markets: &[&str],
            ) -> $crate::exchange::ExchangeResult<Vec<$crate::exchange::Ticker>> {
                use $crate::exchange::MarketData;
                self.client.get_ticker(markets).await
            }

            async fn get_orderbook(
                &self,
                market: &str,
                depth: Option<u32>,
            ) -> $crate::exchange::ExchangeResult<$crate::exchange::OrderBook> {
                use $crate::exchange::MarketData;
                self.client.get_orderbook(market, depth).await
            }

            async fn get_candles(
                &self,
                market: &str,
                interval: $crate::exchange::CandleInterval,
                count: u32,
            ) -> $crate::exchange::ExchangeResult<Vec<$crate::exchange::Candle>> {
                use $crate::exchange::MarketData;
                self.client.get_candles(market, interval, count).await
            }

            async fn place_order(
                &self,
                request: &$crate::exchange::OrderRequest,
            ) -> $crate::exchange::ExchangeResult<$crate::exchange::Order> {
                use $crate::exchange::OrderManagement;
                self.client.place_order(request).await
            }

            async fn cancel_order(
                &self,
                order_id: &str,
            ) -> $crate::exchange::ExchangeResult<$crate::exchange::Order> {
                use $crate::exchange::OrderManagement;
                self.client.cancel_order(order_id).await
            }

            async fn get_order(
                &self,
                order_id: &str,
            ) -> $crate::exchange::ExchangeResult<$crate::exchange::Order> {
                use $crate::exchange::OrderManagement;
                self.client.get_order(order_id).await
            }

            async fn get_open_orders(
                &self,
                market: Option<&str>,
            ) -> $crate::exchange::ExchangeResult<Vec<$crate::exchange::Order>> {
                use $crate::exchange::OrderManagement;
                self.client.get_open_orders(market).await
            }

            async fn get_balances(
                &self,
            ) -> $crate::exchange::ExchangeResult<Vec<$crate::exchange::Balance>> {
                use $crate::exchange::OrderManagement;
                self.client.get_balances().await
            }

            async fn get_balance(
                &self,
                currency: &str,
            ) -> $crate::exchange::ExchangeResult<$crate::exchange::Balance> {
                use $crate::exchange::OrderManagement;
                self.client.get_balance(currency).await
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time test to ensure ExchangeAdapter is object-safe
    fn _assert_object_safe(_: &dyn ExchangeAdapter) {}

    // Compile-time test to ensure ExchangeAdapter can be used with Arc
    fn _assert_arc_compatible(_: std::sync::Arc<dyn ExchangeAdapter>) {}
}
