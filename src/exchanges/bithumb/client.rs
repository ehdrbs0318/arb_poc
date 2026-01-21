//! Bithumb REST API client implementation.
//!
//! This module provides the main client for interacting with the Bithumb API.

use crate::exchange::{
    Balance, Candle, CandleInterval, ExchangeError, ExchangeResult, MarketData, Order, OrderBook,
    OrderBookLevel, OrderManagement, OrderRequest, OrderSide, OrderStatus, OrderType, PriceChange,
    Ticker, TimeInForce,
};
use crate::exchanges::bithumb::auth::{BithumbCredentials, build_query_string};
use crate::exchanges::bithumb::types::{
    BithumbBalance, BithumbCandle, BithumbError, BithumbOrder, BithumbOrderRequest,
    BithumbOrderbook, BithumbTicker,
};
use chrono::{NaiveDateTime, Utc};
use reqwest::Client;
use std::collections::HashMap;

/// Base URL for Bithumb REST API.
const BASE_URL: &str = "https://api.bithumb.com";

/// Bithumb API client.
///
/// This client supports both public and private APIs.
/// For private APIs, credentials must be provided.
#[derive(Debug)]
pub struct BithumbClient {
    client: Client,
    pub(crate) credentials: Option<BithumbCredentials>,
}

impl BithumbClient {
    /// Creates a new unauthenticated Bithumb client.
    ///
    /// This client can only access public APIs.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn new() -> ExchangeResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(ExchangeError::HttpError)?;

        Ok(Self {
            client,
            credentials: None,
        })
    }

    /// Creates a new authenticated Bithumb client.
    ///
    /// This client can access both public and private APIs.
    ///
    /// # Arguments
    ///
    /// * `access_key` - Bithumb API access key
    /// * `secret_key` - Bithumb API secret key
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn with_credentials(
        access_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> ExchangeResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(ExchangeError::HttpError)?;

        Ok(Self {
            client,
            credentials: Some(BithumbCredentials::new(access_key, secret_key)),
        })
    }

    /// Returns the credentials if available.
    fn credentials(&self) -> ExchangeResult<&BithumbCredentials> {
        self.credentials
            .as_ref()
            .ok_or_else(|| ExchangeError::AuthError("Credentials not provided".to_string()))
    }

    /// Makes a GET request to a public endpoint.
    async fn get_public<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> ExchangeResult<T> {
        let url = format!("{BASE_URL}{endpoint}");
        let mut request = self.client.get(&url);

        if let Some(params) = params {
            request = request.query(params);
        }

        let response = request.send().await.map_err(ExchangeError::HttpError)?;
        self.handle_response(response).await
    }

    /// Makes a GET request to a private endpoint.
    async fn get_private<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> ExchangeResult<T> {
        let creds = self.credentials()?;
        let url = format!("{BASE_URL}{endpoint}");

        let auth_header = if let Some(params) = params {
            let query_string = build_query_string(params.iter().map(|(k, v)| (*k, *v)));
            creds.authorization_header_with_query(&query_string)?
        } else {
            creds.authorization_header()?
        };

        let mut request = self.client.get(&url).header("Authorization", auth_header);

        if let Some(params) = params {
            request = request.query(params);
        }

        let response = request.send().await.map_err(ExchangeError::HttpError)?;
        self.handle_response(response).await
    }

    /// Makes a POST request to a private endpoint.
    async fn post_private<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &impl serde::Serialize,
    ) -> ExchangeResult<T> {
        let creds = self.credentials()?;
        let url = format!("{BASE_URL}{endpoint}");

        // Serialize body to query string format for hash
        let body_map: HashMap<String, serde_json::Value> =
            serde_json::from_str(&serde_json::to_string(body).map_err(ExchangeError::JsonError)?)
                .map_err(ExchangeError::JsonError)?;

        let query_parts: Vec<(String, String)> = body_map
            .iter()
            .filter(|(_, v)| !v.is_null())
            .map(|(k, v)| {
                let value = match v {
                    serde_json::Value::String(s) => s.clone(),
                    _ => v.to_string().trim_matches('"').to_string(),
                };
                (k.clone(), value)
            })
            .collect();

        let query_string = build_query_string(query_parts);
        let auth_header = creds.authorization_header_with_query(&query_string)?;

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(ExchangeError::HttpError)?;

        self.handle_response(response).await
    }

    /// Makes a DELETE request to a private endpoint.
    async fn delete_private<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> ExchangeResult<T> {
        let creds = self.credentials()?;
        let url = format!("{BASE_URL}{endpoint}");

        let query_string = build_query_string(params.iter().map(|(k, v)| (*k, *v)));
        let auth_header = creds.authorization_header_with_query(&query_string)?;

        let response = self
            .client
            .delete(&url)
            .header("Authorization", auth_header)
            .query(params)
            .send()
            .await
            .map_err(ExchangeError::HttpError)?;

        self.handle_response(response).await
    }

    /// Handles API response and converts errors.
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> ExchangeResult<T> {
        let status = response.status();

        if status.is_success() {
            response.json::<T>().await.map_err(ExchangeError::HttpError)
        } else {
            let error_text = response.text().await.unwrap_or_default();

            // Try to parse as Bithumb error
            if let Ok(bithumb_error) = serde_json::from_str::<BithumbError>(&error_text) {
                return Err(self.convert_bithumb_error(status.as_u16(), &bithumb_error));
            }

            Err(ExchangeError::UnknownError {
                code: status.as_u16().to_string(),
                message: error_text,
            })
        }
    }

    /// Converts Bithumb error to ExchangeError.
    fn convert_bithumb_error(&self, status: u16, error: &BithumbError) -> ExchangeError {
        let name = &error.error.name;
        let message = &error.error.message;

        match (status, name.as_str()) {
            (401, _) => ExchangeError::AuthError(message.clone()),
            (_, "invalid_parameter") => ExchangeError::InvalidParameter(message.clone()),
            (_, "insufficient_funds_bid" | "insufficient_funds_ask") => {
                ExchangeError::InsufficientFunds(message.clone())
            }
            (_, "notfoundmarket") => ExchangeError::MarketNotFound(message.clone()),
            (403, "market_offline") => ExchangeError::ExchangeOffline(message.clone()),
            (429, _) | (418, _) => ExchangeError::RateLimitExceeded(message.clone()),
            _ => ExchangeError::UnknownError {
                code: name.clone(),
                message: message.clone(),
            },
        }
    }

    /// Converts internal market format to Bithumb format.
    ///
    /// Internal format uses "{QUOTE}-{BASE}" (e.g., "KRW-BTC").
    /// Bithumb uses "{COIN}_{PAYMENT}" format (e.g., "BTC_KRW").
    #[allow(dead_code)]
    fn to_bithumb_market(market: &str) -> String {
        // "KRW-BTC" -> "BTC_KRW"
        let parts: Vec<&str> = market.split('-').collect();
        if parts.len() == 2 {
            format!("{}_{}", parts[1], parts[0])
        } else {
            market.to_string()
        }
    }

    /// Converts Bithumb market format to internal format.
    ///
    /// Bithumb uses "{COIN}_{PAYMENT}" format (e.g., "BTC_KRW").
    /// Internal format uses "{QUOTE}-{BASE}" (e.g., "KRW-BTC").
    #[allow(dead_code)]
    fn from_bithumb_market(market: &str) -> String {
        // "BTC_KRW" -> "KRW-BTC"
        let parts: Vec<&str> = market.split('_').collect();
        if parts.len() == 2 {
            format!("{}-{}", parts[1], parts[0])
        } else {
            market.to_string()
        }
    }
}

impl MarketData for BithumbClient {
    fn name(&self) -> &str {
        "Bithumb"
    }

    async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
        let markets_str = markets.join(",");
        let params = [("markets", markets_str.as_str())];

        let tickers: Vec<BithumbTicker> = self.get_public("/v1/ticker", Some(&params)).await?;

        Ok(tickers.into_iter().map(convert_ticker).collect())
    }

    async fn get_orderbook(&self, market: &str, depth: Option<u32>) -> ExchangeResult<OrderBook> {
        let depth_str = depth.unwrap_or(15).to_string();
        let params = [("markets", market), ("level", &depth_str)];

        let orderbooks: Vec<BithumbOrderbook> =
            self.get_public("/v1/orderbook", Some(&params)).await?;

        orderbooks
            .into_iter()
            .next()
            .map(convert_orderbook)
            .ok_or_else(|| ExchangeError::MarketNotFound(market.to_string()))
    }

    async fn get_candles(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
    ) -> ExchangeResult<Vec<Candle>> {
        let count_str = count.min(200).to_string();
        let params = [("market", market), ("count", &count_str)];

        let endpoint = match interval {
            CandleInterval::Minute1 => "/v1/candles/minutes/1",
            CandleInterval::Minute3 => "/v1/candles/minutes/3",
            CandleInterval::Minute5 => "/v1/candles/minutes/5",
            CandleInterval::Minute10 => "/v1/candles/minutes/10",
            CandleInterval::Minute15 => "/v1/candles/minutes/15",
            CandleInterval::Minute30 => "/v1/candles/minutes/30",
            CandleInterval::Minute60 => "/v1/candles/minutes/60",
            CandleInterval::Minute240 => "/v1/candles/minutes/240",
            CandleInterval::Day => "/v1/candles/days",
            CandleInterval::Week => "/v1/candles/weeks",
            CandleInterval::Month => "/v1/candles/months",
        };

        let candles: Vec<BithumbCandle> = self.get_public(endpoint, Some(&params)).await?;

        Ok(candles.into_iter().map(convert_candle).collect())
    }
}

impl OrderManagement for BithumbClient {
    async fn place_order(&self, request: &OrderRequest) -> ExchangeResult<Order> {
        let side = match request.side {
            OrderSide::Buy => "bid",
            OrderSide::Sell => "ask",
        };

        let ord_type = match request.order_type {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            OrderType::Price => "price",
            OrderType::Best => "best",
        };

        let time_in_force = request.time_in_force.map(|tif| match tif {
            TimeInForce::Gtc => "gtc",
            TimeInForce::Ioc => "ioc",
            TimeInForce::Fok => "fok",
            TimeInForce::PostOnly => "post_only",
        });

        let body = BithumbOrderRequest {
            market: request.market.clone(),
            side: side.to_string(),
            ord_type: ord_type.to_string(),
            volume: request.volume.map(|v| v.to_string()),
            price: request.price.map(|p| p.to_string()),
            time_in_force: time_in_force.map(|s| s.to_string()),
            identifier: request.identifier.clone(),
        };

        let bithumb_order: BithumbOrder = self.post_private("/v1/orders", &body).await?;
        Ok(convert_order(bithumb_order))
    }

    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<Order> {
        let params = [("uuid", order_id)];
        let bithumb_order: BithumbOrder = self.delete_private("/v1/order", &params).await?;
        Ok(convert_order(bithumb_order))
    }

    async fn get_order(&self, order_id: &str) -> ExchangeResult<Order> {
        let params = [("uuid", order_id)];
        let bithumb_order: BithumbOrder = self.get_private("/v1/order", Some(&params)).await?;
        Ok(convert_order(bithumb_order))
    }

    async fn get_open_orders(&self, market: Option<&str>) -> ExchangeResult<Vec<Order>> {
        let mut params = vec![("state", "wait")];
        if let Some(m) = market {
            params.push(("market", m));
        }

        let bithumb_orders: Vec<BithumbOrder> =
            self.get_private("/v1/orders", Some(&params)).await?;
        Ok(bithumb_orders.into_iter().map(convert_order).collect())
    }

    async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
        let bithumb_balances: Vec<BithumbBalance> = self.get_private("/v1/accounts", None).await?;
        Ok(bithumb_balances.into_iter().map(convert_balance).collect())
    }

    async fn get_balance(&self, currency: &str) -> ExchangeResult<Balance> {
        let balances = self.get_balances().await?;
        balances
            .into_iter()
            .find(|b| b.currency == currency)
            .ok_or_else(|| {
                ExchangeError::InvalidParameter(format!("Currency not found: {currency}"))
            })
    }
}

// Conversion functions

fn convert_ticker(t: BithumbTicker) -> Ticker {
    let change = match t.change.as_str() {
        "RISE" => PriceChange::Rise,
        "FALL" => PriceChange::Fall,
        _ => PriceChange::Even,
    };

    Ticker {
        market: t.market,
        trade_price: t.trade_price,
        opening_price: t.opening_price,
        high_price: t.high_price,
        low_price: t.low_price,
        prev_closing_price: t.prev_closing_price,
        change,
        change_rate: t.change_rate,
        change_price: t.change_price,
        acc_trade_volume_24h: t.acc_trade_volume_24h,
        acc_trade_price_24h: t.acc_trade_price_24h,
        timestamp: t.timestamp,
    }
}

fn convert_orderbook(ob: BithumbOrderbook) -> OrderBook {
    let mut asks: Vec<OrderBookLevel> = ob
        .orderbook_units
        .iter()
        .map(|u| OrderBookLevel {
            price: u.ask_price,
            size: u.ask_size,
        })
        .collect();

    let mut bids: Vec<OrderBookLevel> = ob
        .orderbook_units
        .iter()
        .map(|u| OrderBookLevel {
            price: u.bid_price,
            size: u.bid_size,
        })
        .collect();

    // Sort asks ascending, bids descending
    asks.sort_by(|a, b| a.price.cmp(&b.price));
    bids.sort_by(|a, b| b.price.cmp(&a.price));

    OrderBook {
        market: ob.market,
        asks,
        bids,
        total_ask_size: ob.total_ask_size,
        total_bid_size: ob.total_bid_size,
        timestamp: ob.timestamp,
    }
}

fn convert_candle(c: BithumbCandle) -> Candle {
    let timestamp = NaiveDateTime::parse_from_str(&c.candle_date_time_utc, "%Y-%m-%dT%H:%M:%S")
        .map(|dt| dt.and_utc())
        .unwrap_or_else(|_| Utc::now());

    Candle {
        market: c.market,
        timestamp,
        open: c.opening_price,
        high: c.high_price,
        low: c.low_price,
        close: c.trade_price,
        volume: c.candle_acc_trade_volume,
    }
}

fn convert_order(o: BithumbOrder) -> Order {
    let side = match o.side.as_str() {
        "bid" => OrderSide::Buy,
        _ => OrderSide::Sell,
    };

    let order_type = match o.ord_type.as_str() {
        "limit" => OrderType::Limit,
        "market" => OrderType::Market,
        "price" => OrderType::Price,
        _ => OrderType::Best,
    };

    let status = match o.state.as_str() {
        "wait" => OrderStatus::Wait,
        "watch" => OrderStatus::Watch,
        "done" => OrderStatus::Filled,
        "cancel" => OrderStatus::Cancelled,
        _ => OrderStatus::Wait,
    };

    let created_at = NaiveDateTime::parse_from_str(&o.created_at, "%Y-%m-%dT%H:%M:%S%z")
        .or_else(|_| NaiveDateTime::parse_from_str(&o.created_at, "%Y-%m-%dT%H:%M:%S"))
        .map(|dt| dt.and_utc())
        .unwrap_or_else(|_| Utc::now());

    Order {
        id: o.uuid,
        market: o.market,
        side,
        order_type,
        status,
        volume: o.volume,
        remaining_volume: o.remaining_volume,
        executed_volume: o.executed_volume.unwrap_or(o.volume - o.remaining_volume),
        price: o.price,
        avg_price: o.avg_price,
        paid_fee: o.paid_fee,
        created_at,
        identifier: o.identifier,
    }
}

fn convert_balance(b: BithumbBalance) -> Balance {
    Balance {
        currency: b.currency,
        balance: b.balance,
        locked: b.locked,
        avg_buy_price: b.avg_buy_price,
        unit_currency: b.unit_currency,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_bithumb_client_new() {
        let client = BithumbClient::new();
        assert!(client.is_ok());
        let client = client.unwrap();
        assert!(client.credentials.is_none());
    }

    #[test]
    fn test_bithumb_client_with_credentials() {
        let client = BithumbClient::with_credentials("access_key", "secret_key");
        assert!(client.is_ok());
        let client = client.unwrap();
        assert!(client.credentials.is_some());
    }

    #[test]
    fn test_market_format_conversion() {
        // Internal to Bithumb
        assert_eq!(BithumbClient::to_bithumb_market("KRW-BTC"), "BTC_KRW");
        assert_eq!(BithumbClient::to_bithumb_market("BTC-ETH"), "ETH_BTC");

        // Bithumb to Internal
        assert_eq!(BithumbClient::from_bithumb_market("BTC_KRW"), "KRW-BTC");
        assert_eq!(BithumbClient::from_bithumb_market("ETH_BTC"), "BTC-ETH");
    }

    #[test]
    fn test_convert_ticker() {
        let bithumb_ticker = BithumbTicker {
            market: "KRW-BTC".to_string(),
            trade_price: Decimal::from(50000000),
            opening_price: Decimal::from(49000000),
            high_price: Decimal::from(51000000),
            low_price: Decimal::from(48000000),
            prev_closing_price: Decimal::from(49500000),
            change: "RISE".to_string(),
            change_rate: Decimal::new(1, 2),
            change_price: Decimal::from(500000),
            acc_trade_volume_24h: Decimal::from(1000),
            acc_trade_price_24h: Decimal::from(50000000000i64),
            timestamp: Utc::now(),
        };

        let ticker = convert_ticker(bithumb_ticker);
        assert_eq!(ticker.market, "KRW-BTC");
        assert_eq!(ticker.trade_price, Decimal::from(50000000));
        assert_eq!(ticker.change, PriceChange::Rise);
    }

    #[test]
    fn test_market_data_name() {
        let client = BithumbClient::new().unwrap();
        assert_eq!(client.name(), "Bithumb");
    }
}
