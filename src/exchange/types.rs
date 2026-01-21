//! Common types for exchange abstraction.
//!
//! This module defines the data structures used across all exchange implementations.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a trading pair/market (e.g., "KRW-BTC", "BTC-USDT").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Market {
    /// The market code (e.g., "KRW-BTC").
    pub code: String,
    /// Base currency (e.g., "BTC").
    pub base: String,
    /// Quote currency (e.g., "KRW").
    pub quote: String,
}

impl Market {
    /// Creates a new Market from a code string.
    ///
    /// # Arguments
    ///
    /// * `code` - Market code in "QUOTE-BASE" format (e.g., "KRW-BTC")
    ///
    /// # Returns
    ///
    /// Returns `Some(Market)` if the code is valid, `None` otherwise.
    #[must_use]
    pub fn from_code(code: &str) -> Option<Self> {
        let parts: Vec<&str> = code.split('-').collect();
        if parts.len() == 2 {
            Some(Self {
                code: code.to_string(),
                base: parts[1].to_string(),
                quote: parts[0].to_string(),
            })
        } else {
            None
        }
    }
}

/// Current price information for a market.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    /// Market code.
    pub market: String,
    /// Current trade price.
    pub trade_price: Decimal,
    /// Opening price.
    pub opening_price: Decimal,
    /// Highest price in 24 hours.
    pub high_price: Decimal,
    /// Lowest price in 24 hours.
    pub low_price: Decimal,
    /// Previous closing price.
    pub prev_closing_price: Decimal,
    /// Price change status.
    pub change: PriceChange,
    /// Change rate (percentage).
    pub change_rate: Decimal,
    /// Change price (absolute).
    pub change_price: Decimal,
    /// 24-hour accumulated trade volume.
    pub acc_trade_volume_24h: Decimal,
    /// 24-hour accumulated trade price.
    pub acc_trade_price_24h: Decimal,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

/// Price change direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PriceChange {
    /// Price increased.
    Rise,
    /// Price decreased.
    Fall,
    /// Price unchanged.
    #[default]
    Even,
}

/// A single level in the order book.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    /// Price at this level.
    pub price: Decimal,
    /// Size/quantity at this level.
    pub size: Decimal,
}

/// Order book snapshot for a market.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Market code.
    pub market: String,
    /// Ask (sell) orders, sorted by price ascending.
    pub asks: Vec<OrderBookLevel>,
    /// Bid (buy) orders, sorted by price descending.
    pub bids: Vec<OrderBookLevel>,
    /// Total ask size.
    pub total_ask_size: Decimal,
    /// Total bid size.
    pub total_bid_size: Decimal,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
}

impl OrderBook {
    /// Returns the best ask (lowest sell price).
    #[must_use]
    pub fn best_ask(&self) -> Option<&OrderBookLevel> {
        self.asks.first()
    }

    /// Returns the best bid (highest buy price).
    #[must_use]
    pub fn best_bid(&self) -> Option<&OrderBookLevel> {
        self.bids.first()
    }

    /// Calculates the spread between best ask and best bid.
    #[must_use]
    pub fn spread(&self) -> Option<Decimal> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some(ask.price - bid.price),
            _ => None,
        }
    }

    /// Calculates the spread percentage.
    #[must_use]
    pub fn spread_percentage(&self) -> Option<Decimal> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) if bid.price > Decimal::ZERO => {
                Some((ask.price - bid.price) / bid.price * Decimal::from(100))
            }
            _ => None,
        }
    }
}

/// Candle (OHLCV) data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// Market code.
    pub market: String,
    /// Candle timestamp (start of the period).
    pub timestamp: DateTime<Utc>,
    /// Opening price.
    pub open: Decimal,
    /// Highest price.
    pub high: Decimal,
    /// Lowest price.
    pub low: Decimal,
    /// Closing price.
    pub close: Decimal,
    /// Volume.
    pub volume: Decimal,
}

/// Candle interval/timeframe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandleInterval {
    /// 1 minute.
    Minute1,
    /// 3 minutes.
    Minute3,
    /// 5 minutes.
    Minute5,
    /// 10 minutes.
    Minute10,
    /// 15 minutes.
    Minute15,
    /// 30 minutes.
    Minute30,
    /// 60 minutes (1 hour).
    Minute60,
    /// 240 minutes (4 hours).
    Minute240,
    /// 1 day.
    Day,
    /// 1 week.
    Week,
    /// 1 month.
    Month,
}

impl CandleInterval {
    /// Returns the interval in minutes.
    #[must_use]
    pub const fn as_minutes(&self) -> u32 {
        match self {
            Self::Minute1 => 1,
            Self::Minute3 => 3,
            Self::Minute5 => 5,
            Self::Minute10 => 10,
            Self::Minute15 => 15,
            Self::Minute30 => 30,
            Self::Minute60 => 60,
            Self::Minute240 => 240,
            Self::Day => 1440,
            Self::Week => 10080,
            Self::Month => 43200,
        }
    }
}

/// Order side (buy or sell).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    /// Buy order (bid).
    #[serde(alias = "bid")]
    Buy,
    /// Sell order (ask).
    #[serde(alias = "ask")]
    Sell,
}

/// Order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    /// Limit order.
    Limit,
    /// Market order.
    Market,
    /// Market buy order (by price/total).
    Price,
    /// Best price order.
    Best,
}

/// Time in force for orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    /// Good till cancelled.
    Gtc,
    /// Immediate or cancel.
    Ioc,
    /// Fill or kill.
    Fok,
    /// Post only (maker only).
    PostOnly,
}

/// Order status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// Order is waiting to be processed.
    Wait,
    /// Order is being processed.
    Watch,
    /// Order is partially filled.
    PartiallyFilled,
    /// Order is completely filled.
    Filled,
    /// Order is cancelled.
    Cancelled,
    /// Order is rejected.
    Rejected,
}

/// Order request parameters.
#[derive(Debug, Clone)]
pub struct OrderRequest {
    /// Market code.
    pub market: String,
    /// Order side (buy/sell).
    pub side: OrderSide,
    /// Order type.
    pub order_type: OrderType,
    /// Order volume (quantity). Required for limit and market sell orders.
    pub volume: Option<Decimal>,
    /// Order price. Required for limit orders and market buy orders (as total).
    pub price: Option<Decimal>,
    /// Time in force.
    pub time_in_force: Option<TimeInForce>,
    /// Client-defined identifier.
    pub identifier: Option<String>,
}

impl OrderRequest {
    /// Creates a limit buy order request.
    #[must_use]
    pub fn limit_buy(market: impl Into<String>, price: Decimal, volume: Decimal) -> Self {
        Self {
            market: market.into(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            volume: Some(volume),
            price: Some(price),
            time_in_force: None,
            identifier: None,
        }
    }

    /// Creates a limit sell order request.
    #[must_use]
    pub fn limit_sell(market: impl Into<String>, price: Decimal, volume: Decimal) -> Self {
        Self {
            market: market.into(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            volume: Some(volume),
            price: Some(price),
            time_in_force: None,
            identifier: None,
        }
    }

    /// Creates a market buy order request (by total price).
    #[must_use]
    pub fn market_buy(market: impl Into<String>, total: Decimal) -> Self {
        Self {
            market: market.into(),
            side: OrderSide::Buy,
            order_type: OrderType::Price,
            volume: None,
            price: Some(total),
            time_in_force: None,
            identifier: None,
        }
    }

    /// Creates a market sell order request (by volume).
    #[must_use]
    pub fn market_sell(market: impl Into<String>, volume: Decimal) -> Self {
        Self {
            market: market.into(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            volume: Some(volume),
            price: None,
            time_in_force: None,
            identifier: None,
        }
    }

    /// Sets the time in force.
    #[must_use]
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Sets the client identifier.
    #[must_use]
    pub fn with_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }
}

/// Order response from the exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Exchange-assigned order ID.
    pub id: String,
    /// Market code.
    pub market: String,
    /// Order side.
    pub side: OrderSide,
    /// Order type.
    pub order_type: OrderType,
    /// Order status.
    pub status: OrderStatus,
    /// Original order volume.
    pub volume: Decimal,
    /// Remaining volume.
    pub remaining_volume: Decimal,
    /// Executed volume.
    pub executed_volume: Decimal,
    /// Order price (for limit orders).
    pub price: Option<Decimal>,
    /// Average executed price.
    pub avg_price: Option<Decimal>,
    /// Paid fee.
    pub paid_fee: Decimal,
    /// Order creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Client identifier (if provided).
    pub identifier: Option<String>,
}

/// Account balance for a single currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// Currency code (e.g., "BTC", "KRW").
    pub currency: String,
    /// Available balance.
    pub balance: Decimal,
    /// Locked balance (in orders).
    pub locked: Decimal,
    /// Average buy price.
    pub avg_buy_price: Decimal,
    /// Unit currency for avg_buy_price.
    pub unit_currency: String,
}

impl Balance {
    /// Returns the total balance (available + locked).
    #[must_use]
    pub fn total(&self) -> Decimal {
        self.balance + self.locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_from_code_valid() {
        let market = Market::from_code("KRW-BTC").unwrap();
        assert_eq!(market.code, "KRW-BTC");
        assert_eq!(market.base, "BTC");
        assert_eq!(market.quote, "KRW");
    }

    #[test]
    fn test_market_from_code_invalid() {
        assert!(Market::from_code("INVALID").is_none());
        assert!(Market::from_code("").is_none());
        assert!(Market::from_code("A-B-C").is_none());
    }

    #[test]
    fn test_orderbook_best_prices() {
        let ob = OrderBook {
            market: "KRW-BTC".to_string(),
            asks: vec![
                OrderBookLevel {
                    price: Decimal::from(100),
                    size: Decimal::from(1),
                },
                OrderBookLevel {
                    price: Decimal::from(101),
                    size: Decimal::from(2),
                },
            ],
            bids: vec![
                OrderBookLevel {
                    price: Decimal::from(99),
                    size: Decimal::from(1),
                },
                OrderBookLevel {
                    price: Decimal::from(98),
                    size: Decimal::from(2),
                },
            ],
            total_ask_size: Decimal::from(3),
            total_bid_size: Decimal::from(3),
            timestamp: Utc::now(),
        };

        assert_eq!(ob.best_ask().unwrap().price, Decimal::from(100));
        assert_eq!(ob.best_bid().unwrap().price, Decimal::from(99));
        assert_eq!(ob.spread().unwrap(), Decimal::from(1));
    }

    #[test]
    fn test_order_request_builders() {
        let order = OrderRequest::limit_buy("KRW-BTC", Decimal::from(50000000), Decimal::from(1));
        assert_eq!(order.market, "KRW-BTC");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.price, Some(Decimal::from(50000000)));
        assert_eq!(order.volume, Some(Decimal::from(1)));
    }

    #[test]
    fn test_balance_total() {
        let balance = Balance {
            currency: "BTC".to_string(),
            balance: Decimal::from(10),
            locked: Decimal::from(5),
            avg_buy_price: Decimal::from(50000000),
            unit_currency: "KRW".to_string(),
        };
        assert_eq!(balance.total(), Decimal::from(15));
    }

    #[test]
    fn test_candle_interval_as_minutes() {
        assert_eq!(CandleInterval::Minute1.as_minutes(), 1);
        assert_eq!(CandleInterval::Minute60.as_minutes(), 60);
        assert_eq!(CandleInterval::Day.as_minutes(), 1440);
    }
}
