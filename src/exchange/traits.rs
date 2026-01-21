//! Exchange trait definitions.
//!
//! This module defines the traits that all exchange implementations must implement.

use crate::exchange::error::ExchangeResult;
use crate::exchange::types::{
    Balance, Candle, CandleInterval, Order, OrderBook, OrderRequest, Ticker,
};
use std::future::Future;

/// Trait for market data operations (public API).
///
/// This trait defines operations that don't require authentication,
/// such as fetching tickers, order books, and candles.
pub trait MarketData: Send + Sync {
    /// Returns the exchange name.
    fn name(&self) -> &str;

    /// Fetches the current ticker for one or more markets.
    ///
    /// # Arguments
    ///
    /// * `markets` - A slice of market codes (e.g., ["KRW-BTC", "KRW-ETH"])
    fn get_ticker(
        &self,
        markets: &[&str],
    ) -> impl Future<Output = ExchangeResult<Vec<Ticker>>> + Send;

    /// Fetches the order book for a market.
    ///
    /// # Arguments
    ///
    /// * `market` - Market code (e.g., "KRW-BTC")
    /// * `depth` - Number of levels to fetch (optional)
    fn get_orderbook(
        &self,
        market: &str,
        depth: Option<u32>,
    ) -> impl Future<Output = ExchangeResult<OrderBook>> + Send;

    /// Fetches candle data for a market.
    ///
    /// # Arguments
    ///
    /// * `market` - Market code (e.g., "KRW-BTC")
    /// * `interval` - Candle interval
    /// * `count` - Number of candles to fetch
    fn get_candles(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
    ) -> impl Future<Output = ExchangeResult<Vec<Candle>>> + Send;
}

/// Trait for order management operations (private API).
///
/// This trait defines operations that require authentication,
/// such as placing orders and checking account balances.
pub trait OrderManagement: Send + Sync {
    /// Places a new order.
    ///
    /// # Arguments
    ///
    /// * `request` - Order request parameters
    fn place_order(
        &self,
        request: &OrderRequest,
    ) -> impl Future<Output = ExchangeResult<Order>> + Send;

    /// Cancels an existing order.
    ///
    /// # Arguments
    ///
    /// * `order_id` - Exchange-assigned order ID
    fn cancel_order(&self, order_id: &str) -> impl Future<Output = ExchangeResult<Order>> + Send;

    /// Fetches an order by ID.
    ///
    /// # Arguments
    ///
    /// * `order_id` - Exchange-assigned order ID
    fn get_order(&self, order_id: &str) -> impl Future<Output = ExchangeResult<Order>> + Send;

    /// Fetches open orders for a market.
    ///
    /// # Arguments
    ///
    /// * `market` - Market code (optional, None for all markets)
    fn get_open_orders(
        &self,
        market: Option<&str>,
    ) -> impl Future<Output = ExchangeResult<Vec<Order>>> + Send;

    /// Fetches account balances.
    fn get_balances(&self) -> impl Future<Output = ExchangeResult<Vec<Balance>>> + Send;

    /// Fetches balance for a specific currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency code (e.g., "BTC", "KRW")
    fn get_balance(&self, currency: &str) -> impl Future<Output = ExchangeResult<Balance>> + Send;
}

/// Combined trait for full exchange functionality.
///
/// This trait combines market data and order management capabilities.
pub trait Exchange: MarketData + OrderManagement {
    /// Returns whether the exchange client is authenticated.
    fn is_authenticated(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time test to ensure traits are object-safe enough for our use case
    fn _assert_send_sync<T: MarketData + Send + Sync>() {}
    fn _assert_order_mgmt<T: OrderManagement + Send + Sync>() {}
}
