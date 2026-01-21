//! Bybit V5 REST API client implementation.
//!
//! This module provides the main client for interacting with the Bybit V5 API.

use crate::exchange::{
    Balance, Candle, CandleInterval, ExchangeError, ExchangeResult, MarketData, Order, OrderBook,
    OrderBookLevel, OrderManagement, OrderRequest, OrderSide, OrderStatus, OrderType, PriceChange,
    Ticker, TimeInForce,
};
use crate::exchanges::bybit::auth::{AuthHeaders, BybitCredentials, build_query_string};
use crate::exchanges::bybit::types::{
    BybitCancelOrderRequest, BybitCancelOrderResult, BybitCreateOrderResult, BybitKlineList,
    BybitOrder, BybitOrderList, BybitOrderRequest, BybitOrderbookResult, BybitResponse,
    BybitTickerList, BybitWalletBalanceResult,
};
use chrono::{TimeZone, Utc};
use reqwest::Client;
use rust_decimal::Decimal;

/// Base URL for Bybit V5 REST API (mainnet).
const BASE_URL_MAINNET: &str = "https://api.bybit.com";

/// Base URL for Bybit V5 REST API (testnet).
const BASE_URL_TESTNET: &str = "https://api-testnet.bybit.com";

/// Default category for spot trading.
const DEFAULT_CATEGORY: &str = "spot";

/// Bybit V5 API client.
///
/// This client supports both public (market data) and private (trading) APIs.
/// For private APIs, credentials must be provided.
#[derive(Debug)]
pub struct BybitClient {
    client: Client,
    pub(crate) credentials: Option<BybitCredentials>,
    base_url: String,
    category: String,
}

impl BybitClient {
    /// Creates a new unauthenticated Bybit client for mainnet.
    ///
    /// This client can only access public (market data) APIs.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn new() -> ExchangeResult<Self> {
        Self::new_internal(None, false)
    }

    /// Creates a new unauthenticated Bybit client for testnet.
    ///
    /// This client can only access public (market data) APIs on testnet.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn new_testnet() -> ExchangeResult<Self> {
        Self::new_internal(None, true)
    }

    /// Creates a new authenticated Bybit client for mainnet.
    ///
    /// This client can access both public and private APIs.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Bybit API key
    /// * `secret_key` - Bybit API secret key
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn with_credentials(
        api_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> ExchangeResult<Self> {
        let creds = BybitCredentials::new(api_key, secret_key);
        Self::new_internal(Some(creds), false)
    }

    /// Creates a new authenticated Bybit client for testnet.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Bybit API key
    /// * `secret_key` - Bybit API secret key
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn with_credentials_testnet(
        api_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> ExchangeResult<Self> {
        let creds = BybitCredentials::new(api_key, secret_key);
        Self::new_internal(Some(creds), true)
    }

    /// Internal constructor.
    fn new_internal(credentials: Option<BybitCredentials>, testnet: bool) -> ExchangeResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(ExchangeError::HttpError)?;

        let base_url = if testnet {
            BASE_URL_TESTNET.to_string()
        } else {
            BASE_URL_MAINNET.to_string()
        };

        Ok(Self {
            client,
            credentials,
            base_url,
            category: DEFAULT_CATEGORY.to_string(),
        })
    }

    /// Sets the trading category (spot, linear, inverse, option).
    ///
    /// # Arguments
    ///
    /// * `category` - Trading category
    #[must_use]
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    /// Returns the credentials if available.
    fn credentials(&self) -> ExchangeResult<&BybitCredentials> {
        self.credentials
            .as_ref()
            .ok_or_else(|| ExchangeError::AuthError("Credentials not provided".to_string()))
    }

    /// Makes a GET request to a public endpoint.
    async fn get_public<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> ExchangeResult<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .client
            .get(&url)
            .query(params)
            .send()
            .await
            .map_err(ExchangeError::HttpError)?;

        self.handle_response(response).await
    }

    /// Makes a GET request to a private endpoint.
    async fn get_private<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> ExchangeResult<T> {
        let creds = self.credentials()?;
        let url = format!("{}{}", self.base_url, endpoint);

        let query_string = build_query_string(params.iter().map(|(k, v)| (*k, *v)));
        let auth = creds.auth_headers_get(&query_string)?;

        let response = self
            .client
            .get(&url)
            .query(params)
            .header(AuthHeaders::HEADER_API_KEY, &auth.api_key)
            .header(AuthHeaders::HEADER_TIMESTAMP, auth.timestamp.to_string())
            .header(AuthHeaders::HEADER_SIGN, &auth.signature)
            .header(
                AuthHeaders::HEADER_RECV_WINDOW,
                auth.recv_window.to_string(),
            )
            .send()
            .await
            .map_err(ExchangeError::HttpError)?;

        self.handle_response(response).await
    }

    /// Makes a POST request to a private endpoint.
    async fn post_private<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &impl serde::Serialize,
    ) -> ExchangeResult<T> {
        let creds = self.credentials()?;
        let url = format!("{}{}", self.base_url, endpoint);

        let body_json = serde_json::to_string(body).map_err(ExchangeError::JsonError)?;
        let auth = creds.auth_headers_post(&body_json)?;

        let response = self
            .client
            .post(&url)
            .header(AuthHeaders::HEADER_API_KEY, &auth.api_key)
            .header(AuthHeaders::HEADER_TIMESTAMP, auth.timestamp.to_string())
            .header(AuthHeaders::HEADER_SIGN, &auth.signature)
            .header(
                AuthHeaders::HEADER_RECV_WINDOW,
                auth.recv_window.to_string(),
            )
            .header("Content-Type", "application/json")
            .body(body_json)
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
        let body = response.text().await.map_err(ExchangeError::HttpError)?;

        if !status.is_success() {
            return Err(self.parse_error(&body, status.as_u16()));
        }

        // Parse Bybit response wrapper
        let bybit_resp: BybitResponse<T> =
            serde_json::from_str(&body).map_err(ExchangeError::JsonError)?;

        if !bybit_resp.is_success() {
            return Err(self.convert_bybit_error(bybit_resp.ret_code, &bybit_resp.ret_msg));
        }

        Ok(bybit_resp.result)
    }

    /// Parses error from response body.
    fn parse_error(&self, body: &str, status: u16) -> ExchangeError {
        if let Ok(resp) = serde_json::from_str::<BybitResponse<serde_json::Value>>(body) {
            return self.convert_bybit_error(resp.ret_code, &resp.ret_msg);
        }

        ExchangeError::UnknownError {
            code: status.to_string(),
            message: body.to_string(),
        }
    }

    /// Converts Bybit error code to ExchangeError.
    fn convert_bybit_error(&self, ret_code: i32, message: &str) -> ExchangeError {
        match ret_code {
            // Authentication errors
            10003 | 10004 | 10005 | 33004 => ExchangeError::AuthError(message.to_string()),
            // Invalid parameter (10001 is also used for market not found, handled in order)
            10002 | 10016 => ExchangeError::InvalidParameter(message.to_string()),
            // Insufficient funds
            110007 | 110011 | 110012 => ExchangeError::InsufficientFunds(message.to_string()),
            // Order not found
            110001 | 20001 => ExchangeError::OrderNotFound(message.to_string()),
            // Market not found / Invalid parameter (ambiguous error code)
            10001 => ExchangeError::InvalidParameter(message.to_string()),
            // Rate limit
            10006 | 10018 => ExchangeError::RateLimitExceeded(message.to_string()),
            // System error
            10000 | 10010 => ExchangeError::InternalError(message.to_string()),
            // Unknown
            _ => ExchangeError::UnknownError {
                code: ret_code.to_string(),
                message: message.to_string(),
            },
        }
    }

    /// Converts Bybit symbol format to common market format.
    ///
    /// Bybit uses "BTCUSDT" format, we need to convert to "USDT-BTC" format.
    fn to_market_code(symbol: &str) -> String {
        // Common quote currencies in order of preference
        let quotes = ["USDT", "USDC", "BTC", "ETH", "EUR", "DAI"];

        for quote in quotes {
            if let Some(base) = symbol.strip_suffix(quote) {
                return format!("{}-{}", quote, base);
            }
        }

        // Fallback: return as-is
        symbol.to_string()
    }

    /// Converts common market format to Bybit symbol format.
    ///
    /// Common format "USDT-BTC" -> Bybit "BTCUSDT"
    fn to_bybit_symbol(market: &str) -> String {
        if let Some((quote, base)) = market.split_once('-') {
            format!("{}{}", base, quote)
        } else {
            market.to_string()
        }
    }
}

impl MarketData for BybitClient {
    fn name(&self) -> &str {
        "Bybit"
    }

    async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
        let mut tickers = Vec::with_capacity(markets.len());

        for market in markets {
            let symbol = Self::to_bybit_symbol(market);
            let params = [("category", self.category.as_str()), ("symbol", &symbol)];

            let result: BybitTickerList = self.get_public("/v5/market/tickers", &params).await?;

            for t in result.list {
                tickers.push(convert_ticker(t, market));
            }
        }

        Ok(tickers)
    }

    async fn get_orderbook(&self, market: &str, depth: Option<u32>) -> ExchangeResult<OrderBook> {
        let symbol = Self::to_bybit_symbol(market);
        let depth_str = depth.unwrap_or(25).min(500).to_string();
        let params = [
            ("category", self.category.as_str()),
            ("symbol", &symbol),
            ("limit", &depth_str),
        ];

        let result: BybitOrderbookResult = self.get_public("/v5/market/orderbook", &params).await?;

        Ok(convert_orderbook(result, market))
    }

    async fn get_candles(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
    ) -> ExchangeResult<Vec<Candle>> {
        let symbol = Self::to_bybit_symbol(market);
        let interval_str = interval_to_bybit(interval);
        let limit_str = count.min(1000).to_string();

        let params = [
            ("category", self.category.as_str()),
            ("symbol", &symbol),
            ("interval", interval_str),
            ("limit", &limit_str),
        ];

        let result: BybitKlineList = self.get_public("/v5/market/kline", &params).await?;

        Ok(result
            .list
            .into_iter()
            .map(|k| convert_candle(k, market))
            .collect())
    }
}

impl OrderManagement for BybitClient {
    async fn place_order(&self, request: &OrderRequest) -> ExchangeResult<Order> {
        let symbol = Self::to_bybit_symbol(&request.market);

        let side = match request.side {
            OrderSide::Buy => "Buy",
            OrderSide::Sell => "Sell",
        };

        let order_type = match request.order_type {
            OrderType::Limit => "Limit",
            OrderType::Market | OrderType::Price => "Market",
            OrderType::Best => "Limit", // Bybit doesn't have "best" order type
        };

        let time_in_force = request.time_in_force.map(|tif| match tif {
            TimeInForce::Gtc => "GTC",
            TimeInForce::Ioc => "IOC",
            TimeInForce::Fok => "FOK",
            TimeInForce::PostOnly => "PostOnly",
        });

        // For market buy orders with price (total amount), use quoteCoin as market_unit
        let (qty, market_unit) = if request.order_type == OrderType::Price {
            // Market buy by total quote amount
            (
                request.price.unwrap_or(Decimal::ZERO).to_string(),
                Some("quoteCoin".to_string()),
            )
        } else {
            (request.volume.unwrap_or(Decimal::ZERO).to_string(), None)
        };

        let body = BybitOrderRequest {
            category: self.category.clone(),
            symbol,
            side: side.to_string(),
            order_type: order_type.to_string(),
            qty,
            price: request.price.map(|p| p.to_string()),
            time_in_force: time_in_force.map(|s| s.to_string()),
            order_link_id: request.identifier.clone(),
            market_unit,
        };

        let result: BybitCreateOrderResult = self.post_private("/v5/order/create", &body).await?;

        // Fetch the full order details
        self.get_order(&result.order_id).await
    }

    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<Order> {
        // First, get the order to find its symbol
        let order = self.get_order(order_id).await?;
        let symbol = Self::to_bybit_symbol(&order.market);

        let body = BybitCancelOrderRequest {
            category: self.category.clone(),
            symbol,
            order_id: Some(order_id.to_string()),
            order_link_id: None,
        };

        let _result: BybitCancelOrderResult = self.post_private("/v5/order/cancel", &body).await?;

        // Return the updated order
        self.get_order(order_id).await
    }

    async fn get_order(&self, order_id: &str) -> ExchangeResult<Order> {
        let params = [("category", self.category.as_str()), ("orderId", order_id)];

        let result: BybitOrderList = self.get_private("/v5/order/realtime", &params).await?;

        result
            .list
            .into_iter()
            .next()
            .map(convert_order)
            .ok_or_else(|| ExchangeError::OrderNotFound(order_id.to_string()))
    }

    async fn get_open_orders(&self, market: Option<&str>) -> ExchangeResult<Vec<Order>> {
        let symbol;
        let mut params = vec![("category", self.category.as_str())];

        if let Some(m) = market {
            symbol = Self::to_bybit_symbol(m);
            params.push(("symbol", symbol.as_str()));
        }

        let result: BybitOrderList = self.get_private("/v5/order/realtime", &params).await?;

        Ok(result.list.into_iter().map(convert_order).collect())
    }

    async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
        let account_type = match self.category.as_str() {
            "spot" => "UNIFIED",
            "linear" | "option" => "UNIFIED",
            "inverse" => "CONTRACT",
            _ => "UNIFIED",
        };

        let params = [("accountType", account_type)];

        let result: BybitWalletBalanceResult = self
            .get_private("/v5/account/wallet-balance", &params)
            .await?;

        let mut balances = Vec::new();
        for account in result.list {
            for coin in account.coin {
                balances.push(convert_balance(coin));
            }
        }

        Ok(balances)
    }

    async fn get_balance(&self, currency: &str) -> ExchangeResult<Balance> {
        let balances = self.get_balances().await?;
        balances
            .into_iter()
            .find(|b| b.currency == currency)
            .ok_or_else(|| {
                ExchangeError::InvalidParameter(format!("Currency not found: {}", currency))
            })
    }
}

// Conversion functions

fn convert_ticker(t: crate::exchanges::bybit::types::BybitTicker, market: &str) -> Ticker {
    let change = if t.price_24h_pcnt > Decimal::ZERO {
        PriceChange::Rise
    } else if t.price_24h_pcnt < Decimal::ZERO {
        PriceChange::Fall
    } else {
        PriceChange::Even
    };

    let change_price = t.last_price - t.prev_price_24h;

    Ticker {
        market: market.to_string(),
        trade_price: t.last_price,
        opening_price: t.prev_price_24h, // Bybit doesn't provide exact opening price
        high_price: t.high_price_24h,
        low_price: t.low_price_24h,
        prev_closing_price: t.prev_price_24h,
        change,
        change_rate: t.price_24h_pcnt,
        change_price,
        acc_trade_volume_24h: t.volume_24h,
        acc_trade_price_24h: t.turnover_24h,
        timestamp: Utc::now(),
    }
}

fn convert_orderbook(ob: BybitOrderbookResult, market: &str) -> OrderBook {
    let asks: Vec<OrderBookLevel> = ob
        .asks
        .iter()
        .map(|level| OrderBookLevel {
            price: level.0,
            size: level.1,
        })
        .collect();

    let bids: Vec<OrderBookLevel> = ob
        .bids
        .iter()
        .map(|level| OrderBookLevel {
            price: level.0,
            size: level.1,
        })
        .collect();

    let total_ask_size = asks.iter().fold(Decimal::ZERO, |acc, l| acc + l.size);
    let total_bid_size = bids.iter().fold(Decimal::ZERO, |acc, l| acc + l.size);

    let timestamp = Utc
        .timestamp_millis_opt(ob.timestamp)
        .single()
        .unwrap_or_else(Utc::now);

    OrderBook {
        market: market.to_string(),
        asks,
        bids,
        total_ask_size,
        total_bid_size,
        timestamp,
    }
}

fn convert_candle(k: crate::exchanges::bybit::types::BybitKline, market: &str) -> Candle {
    let timestamp = Utc
        .timestamp_millis_opt(k.start_time)
        .single()
        .unwrap_or_else(Utc::now);

    Candle {
        market: market.to_string(),
        timestamp,
        open: k.open,
        high: k.high,
        low: k.low,
        close: k.close,
        volume: k.volume,
    }
}

fn convert_order(o: BybitOrder) -> Order {
    let market = BybitClient::to_market_code(&o.symbol);

    let side = match o.side.as_str() {
        "Buy" => OrderSide::Buy,
        _ => OrderSide::Sell,
    };

    let order_type = match o.order_type.as_str() {
        "Limit" => OrderType::Limit,
        "Market" => OrderType::Market,
        _ => OrderType::Limit,
    };

    let status = match o.order_status.as_str() {
        "New" | "Created" => OrderStatus::Wait,
        "PartiallyFilled" | "PartiallyFilledCanceled" => OrderStatus::PartiallyFilled,
        "Filled" => OrderStatus::Filled,
        "Cancelled" | "PendingCancel" => OrderStatus::Cancelled,
        "Rejected" => OrderStatus::Rejected,
        _ => OrderStatus::Wait,
    };

    let executed_volume = o.cum_exec_qty;
    let remaining_volume = o.leaves_qty;

    Order {
        id: o.order_id,
        market,
        side,
        order_type,
        status,
        volume: o.qty,
        remaining_volume,
        executed_volume,
        price: o.price,
        avg_price: o.avg_price,
        paid_fee: o.cum_exec_fee,
        created_at: o.created_time,
        identifier: o.order_link_id,
    }
}

fn convert_balance(b: crate::exchanges::bybit::types::BybitCoinBalance) -> Balance {
    let locked = b.wallet_balance - b.available_to_withdraw;

    Balance {
        currency: b.coin,
        balance: b.available_to_withdraw,
        locked,
        avg_buy_price: Decimal::ZERO,      // Bybit doesn't provide this
        unit_currency: "USDT".to_string(), // Default to USDT
    }
}

/// Converts CandleInterval to Bybit interval string.
fn interval_to_bybit(interval: CandleInterval) -> &'static str {
    match interval {
        CandleInterval::Minute1 => "1",
        CandleInterval::Minute3 => "3",
        CandleInterval::Minute5 => "5",
        CandleInterval::Minute10 => "15", // Bybit doesn't have 10m, use 15m
        CandleInterval::Minute15 => "15",
        CandleInterval::Minute30 => "30",
        CandleInterval::Minute60 => "60",
        CandleInterval::Minute240 => "240",
        CandleInterval::Day => "D",
        CandleInterval::Week => "W",
        CandleInterval::Month => "M",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bybit_client_new() {
        let client = BybitClient::new();
        assert!(client.is_ok());
        let client = client.unwrap();
        assert!(client.credentials.is_none());
        assert_eq!(client.base_url, BASE_URL_MAINNET);
    }

    #[test]
    fn test_bybit_client_testnet() {
        let client = BybitClient::new_testnet();
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.base_url, BASE_URL_TESTNET);
    }

    #[test]
    fn test_bybit_client_with_credentials() {
        let client = BybitClient::with_credentials("api_key", "secret_key");
        assert!(client.is_ok());
        let client = client.unwrap();
        assert!(client.credentials.is_some());
    }

    #[test]
    fn test_bybit_client_with_category() {
        let client = BybitClient::new().unwrap().with_category("linear");
        assert_eq!(client.category, "linear");
    }

    #[test]
    fn test_to_bybit_symbol() {
        assert_eq!(BybitClient::to_bybit_symbol("USDT-BTC"), "BTCUSDT");
        assert_eq!(BybitClient::to_bybit_symbol("USDC-ETH"), "ETHUSDC");
        assert_eq!(BybitClient::to_bybit_symbol("BTC-ETH"), "ETHBTC");
        assert_eq!(BybitClient::to_bybit_symbol("BTCUSDT"), "BTCUSDT");
    }

    #[test]
    fn test_to_market_code() {
        assert_eq!(BybitClient::to_market_code("BTCUSDT"), "USDT-BTC");
        assert_eq!(BybitClient::to_market_code("ETHUSDC"), "USDC-ETH");
        assert_eq!(BybitClient::to_market_code("ETHBTC"), "BTC-ETH");
    }

    #[test]
    fn test_interval_to_bybit() {
        assert_eq!(interval_to_bybit(CandleInterval::Minute1), "1");
        assert_eq!(interval_to_bybit(CandleInterval::Minute5), "5");
        assert_eq!(interval_to_bybit(CandleInterval::Minute60), "60");
        assert_eq!(interval_to_bybit(CandleInterval::Day), "D");
        assert_eq!(interval_to_bybit(CandleInterval::Week), "W");
    }

    #[test]
    fn test_market_data_name() {
        let client = BybitClient::new().unwrap();
        assert_eq!(client.name(), "Bybit");
    }

    #[test]
    fn test_convert_bybit_error() {
        let client = BybitClient::new().unwrap();

        let err = client.convert_bybit_error(10003, "Invalid API key");
        assert!(matches!(err, ExchangeError::AuthError(_)));

        let err = client.convert_bybit_error(110007, "Insufficient balance");
        assert!(matches!(err, ExchangeError::InsufficientFunds(_)));

        let err = client.convert_bybit_error(10006, "Too many requests");
        assert!(matches!(err, ExchangeError::RateLimitExceeded(_)));
    }
}
