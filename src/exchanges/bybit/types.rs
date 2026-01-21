//! Bybit-specific types and API response structures.
//!
//! These types are used for deserializing Bybit V5 API responses
//! and are then converted to the common exchange types.

use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

/// Bybit V5 API response wrapper.
///
/// All Bybit V5 API responses follow this structure:
/// ```json
/// {
///   "retCode": 0,
///   "retMsg": "OK",
///   "result": { ... },
///   "retExtInfo": {},
///   "time": 1671017382656
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct BybitResponse<T> {
    /// Return code (0 = success).
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    /// Return message.
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    /// Result data.
    pub result: T,
    /// Extended info.
    #[serde(rename = "retExtInfo", default)]
    pub ret_ext_info: serde_json::Value,
    /// Server timestamp in milliseconds.
    pub time: i64,
}

impl<T> BybitResponse<T> {
    /// Returns true if the request was successful.
    #[inline]
    pub fn is_success(&self) -> bool {
        self.ret_code == 0
    }
}

/// Bybit ticker list result.
#[derive(Debug, Deserialize)]
pub struct BybitTickerList {
    pub category: String,
    pub list: Vec<BybitTicker>,
}

/// Bybit ticker response (spot).
#[derive(Debug, Deserialize)]
pub struct BybitTicker {
    /// Symbol (e.g., "BTCUSDT").
    pub symbol: String,
    /// Last traded price.
    #[serde(rename = "lastPrice", deserialize_with = "deserialize_decimal_string")]
    pub last_price: Decimal,
    /// Index price (may be empty for spot).
    #[serde(
        rename = "indexPrice",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub index_price: Option<Decimal>,
    /// Mark price (may be empty for spot).
    #[serde(
        rename = "markPrice",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub mark_price: Option<Decimal>,
    /// Previous 24h close price.
    #[serde(
        rename = "prevPrice24h",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub prev_price_24h: Decimal,
    /// 24h price change percentage.
    #[serde(
        rename = "price24hPcnt",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub price_24h_pcnt: Decimal,
    /// 24h high price.
    #[serde(
        rename = "highPrice24h",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub high_price_24h: Decimal,
    /// 24h low price.
    #[serde(
        rename = "lowPrice24h",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub low_price_24h: Decimal,
    /// 24h turnover in quote currency.
    #[serde(
        rename = "turnover24h",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub turnover_24h: Decimal,
    /// 24h volume in base currency.
    #[serde(rename = "volume24h", deserialize_with = "deserialize_decimal_string")]
    pub volume_24h: Decimal,
    /// Best bid price.
    #[serde(rename = "bid1Price", deserialize_with = "deserialize_decimal_string")]
    pub bid1_price: Decimal,
    /// Best bid size.
    #[serde(rename = "bid1Size", deserialize_with = "deserialize_decimal_string")]
    pub bid1_size: Decimal,
    /// Best ask price.
    #[serde(rename = "ask1Price", deserialize_with = "deserialize_decimal_string")]
    pub ask1_price: Decimal,
    /// Best ask size.
    #[serde(rename = "ask1Size", deserialize_with = "deserialize_decimal_string")]
    pub ask1_size: Decimal,
}

/// Bybit orderbook result.
#[derive(Debug, Deserialize)]
pub struct BybitOrderbookResult {
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Ask prices and sizes.
    #[serde(rename = "a")]
    pub asks: Vec<BybitOrderbookLevel>,
    /// Bid prices and sizes.
    #[serde(rename = "b")]
    pub bids: Vec<BybitOrderbookLevel>,
    /// Timestamp in milliseconds.
    #[serde(rename = "ts")]
    pub timestamp: i64,
    /// Update ID.
    #[serde(rename = "u")]
    pub update_id: i64,
}

/// Bybit orderbook level [price, size].
#[derive(Debug, Deserialize)]
pub struct BybitOrderbookLevel(
    #[serde(deserialize_with = "deserialize_decimal_string")] pub Decimal,
    #[serde(deserialize_with = "deserialize_decimal_string")] pub Decimal,
);

/// Bybit kline/candle list result.
#[derive(Debug, Deserialize)]
pub struct BybitKlineList {
    pub category: String,
    pub symbol: String,
    pub list: Vec<BybitKline>,
}

/// Bybit kline/candle data.
/// Response is an array: [startTime, open, high, low, close, volume, turnover]
#[derive(Debug)]
pub struct BybitKline {
    /// Start time in milliseconds.
    pub start_time: i64,
    /// Open price.
    pub open: Decimal,
    /// High price.
    pub high: Decimal,
    /// Low price.
    pub low: Decimal,
    /// Close price.
    pub close: Decimal,
    /// Volume.
    pub volume: Decimal,
    /// Turnover.
    pub turnover: Decimal,
}

impl<'de> Deserialize<'de> for BybitKline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: Vec<String> = Vec::deserialize(deserializer)?;
        if arr.len() < 7 {
            return Err(serde::de::Error::custom(
                "Expected 7 elements in kline array",
            ));
        }

        let start_time = arr[0].parse::<i64>().map_err(serde::de::Error::custom)?;
        let open = arr[1]
            .parse::<Decimal>()
            .map_err(serde::de::Error::custom)?;
        let high = arr[2]
            .parse::<Decimal>()
            .map_err(serde::de::Error::custom)?;
        let low = arr[3]
            .parse::<Decimal>()
            .map_err(serde::de::Error::custom)?;
        let close = arr[4]
            .parse::<Decimal>()
            .map_err(serde::de::Error::custom)?;
        let volume = arr[5]
            .parse::<Decimal>()
            .map_err(serde::de::Error::custom)?;
        let turnover = arr[6]
            .parse::<Decimal>()
            .map_err(serde::de::Error::custom)?;

        Ok(BybitKline {
            start_time,
            open,
            high,
            low,
            close,
            volume,
            turnover,
        })
    }
}

/// Bybit account wallet balance result.
#[derive(Debug, Deserialize)]
pub struct BybitWalletBalanceResult {
    pub list: Vec<BybitWalletAccount>,
}

/// Bybit wallet account.
#[derive(Debug, Deserialize)]
pub struct BybitWalletAccount {
    /// Account type (UNIFIED, CONTRACT, etc.).
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// Total equity.
    #[serde(
        rename = "totalEquity",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub total_equity: Option<Decimal>,
    /// Account coins.
    pub coin: Vec<BybitCoinBalance>,
}

/// Bybit coin balance.
#[derive(Debug, Deserialize)]
pub struct BybitCoinBalance {
    /// Coin name (e.g., "BTC", "USDT").
    pub coin: String,
    /// Wallet balance.
    #[serde(
        rename = "walletBalance",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub wallet_balance: Decimal,
    /// Available balance.
    #[serde(
        rename = "availableToWithdraw",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub available_to_withdraw: Decimal,
    /// Locked balance (in orders, etc.).
    #[serde(default, deserialize_with = "deserialize_optional_decimal_string")]
    pub locked: Option<Decimal>,
    /// Unrealized PnL.
    #[serde(
        rename = "unrealisedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub unrealised_pnl: Option<Decimal>,
    /// Cumulative realized PnL.
    #[serde(
        rename = "cumRealisedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub cum_realised_pnl: Option<Decimal>,
}

/// Bybit order list result.
#[derive(Debug, Deserialize)]
pub struct BybitOrderList {
    pub category: String,
    pub list: Vec<BybitOrder>,
    #[serde(rename = "nextPageCursor", default)]
    pub next_page_cursor: Option<String>,
}

/// Bybit order response.
#[derive(Debug, Deserialize)]
pub struct BybitOrder {
    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Client order ID.
    #[serde(rename = "orderLinkId", default)]
    pub order_link_id: Option<String>,
    /// Symbol.
    pub symbol: String,
    /// Side: Buy, Sell.
    pub side: String,
    /// Order type: Limit, Market.
    #[serde(rename = "orderType")]
    pub order_type: String,
    /// Order price.
    #[serde(default, deserialize_with = "deserialize_optional_decimal_string")]
    pub price: Option<Decimal>,
    /// Order quantity.
    #[serde(deserialize_with = "deserialize_decimal_string")]
    pub qty: Decimal,
    /// Time in force: GTC, IOC, FOK, PostOnly.
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Order status.
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    /// Cumulative executed quantity.
    #[serde(rename = "cumExecQty", deserialize_with = "deserialize_decimal_string")]
    pub cum_exec_qty: Decimal,
    /// Cumulative executed value.
    #[serde(
        rename = "cumExecValue",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub cum_exec_value: Decimal,
    /// Cumulative executed fee.
    #[serde(rename = "cumExecFee", deserialize_with = "deserialize_decimal_string")]
    pub cum_exec_fee: Decimal,
    /// Average executed price.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub avg_price: Option<Decimal>,
    /// Remaining quantity.
    #[serde(rename = "leavesQty", deserialize_with = "deserialize_decimal_string")]
    pub leaves_qty: Decimal,
    /// Created time in milliseconds.
    #[serde(
        rename = "createdTime",
        deserialize_with = "deserialize_timestamp_string"
    )]
    pub created_time: DateTime<Utc>,
    /// Updated time in milliseconds.
    #[serde(
        rename = "updatedTime",
        deserialize_with = "deserialize_timestamp_string"
    )]
    pub updated_time: DateTime<Utc>,
}

/// Bybit create order result.
#[derive(Debug, Deserialize)]
pub struct BybitCreateOrderResult {
    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Client order ID.
    #[serde(rename = "orderLinkId", default)]
    pub order_link_id: Option<String>,
}

/// Bybit cancel order result.
#[derive(Debug, Deserialize)]
pub struct BybitCancelOrderResult {
    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Client order ID.
    #[serde(rename = "orderLinkId", default)]
    pub order_link_id: Option<String>,
}

/// Bybit order request body for creating orders.
#[derive(Debug, Serialize)]
pub struct BybitOrderRequest {
    /// Category: spot, linear, inverse, option.
    pub category: String,
    /// Symbol (e.g., "BTCUSDT").
    pub symbol: String,
    /// Side: Buy, Sell.
    pub side: String,
    /// Order type: Limit, Market.
    #[serde(rename = "orderType")]
    pub order_type: String,
    /// Order quantity.
    pub qty: String,
    /// Order price (required for limit orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Time in force: GTC, IOC, FOK, PostOnly.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    /// Client order ID.
    #[serde(rename = "orderLinkId", skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Market unit for market orders (baseCoin, quoteCoin).
    #[serde(rename = "marketUnit", skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
}

/// Bybit cancel order request body.
#[derive(Debug, Serialize)]
pub struct BybitCancelOrderRequest {
    /// Category: spot, linear, inverse, option.
    pub category: String,
    /// Symbol.
    pub symbol: String,
    /// Order ID (either orderId or orderLinkId is required).
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Client order ID.
    #[serde(rename = "orderLinkId", skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

/// Deserialize decimal from string.
fn deserialize_decimal_string<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(Decimal::ZERO);
    }
    s.parse::<Decimal>().map_err(serde::de::Error::custom)
}

/// Deserialize optional decimal from string.
fn deserialize_optional_decimal_string<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) if !s.is_empty() => s
            .parse::<Decimal>()
            .map(Some)
            .map_err(serde::de::Error::custom),
        _ => Ok(None),
    }
}

/// Deserialize timestamp from string milliseconds.
fn deserialize_timestamp_string<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let millis: i64 = s.parse().map_err(serde::de::Error::custom)?;
    Utc.timestamp_millis_opt(millis)
        .single()
        .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_bybit_response() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {"test": "value"},
            "retExtInfo": {},
            "time": 1671017382656
        }"#;

        let resp: BybitResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(resp.is_success());
        assert_eq!(resp.ret_msg, "OK");
    }

    #[test]
    fn test_deserialize_bybit_ticker() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "lastPrice": "42000.50",
            "prevPrice24h": "41000.00",
            "price24hPcnt": "0.0244",
            "highPrice24h": "43000.00",
            "lowPrice24h": "40500.00",
            "turnover24h": "1000000000",
            "volume24h": "25000",
            "bid1Price": "42000.00",
            "bid1Size": "1.5",
            "ask1Price": "42001.00",
            "ask1Size": "2.0"
        }"#;

        let ticker: BybitTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.last_price, Decimal::new(4200050, 2));
    }

    #[test]
    fn test_deserialize_bybit_orderbook() {
        let json = r#"{
            "s": "BTCUSDT",
            "a": [["42001.00", "1.5"], ["42002.00", "2.0"]],
            "b": [["42000.00", "1.0"], ["41999.00", "0.5"]],
            "ts": 1671017382656,
            "u": 12345
        }"#;

        let ob: BybitOrderbookResult = serde_json::from_str(json).unwrap();
        assert_eq!(ob.symbol, "BTCUSDT");
        assert_eq!(ob.asks.len(), 2);
        assert_eq!(ob.bids.len(), 2);
        assert_eq!(ob.asks[0].0, Decimal::new(4200100, 2));
    }

    #[test]
    fn test_deserialize_bybit_kline() {
        let json = r#"["1671017382656", "42000.00", "43000.00", "41500.00", "42500.00", "100.5", "4225000.00"]"#;

        let kline: BybitKline = serde_json::from_str(json).unwrap();
        assert_eq!(kline.start_time, 1671017382656);
        assert_eq!(kline.open, Decimal::new(4200000, 2));
        assert_eq!(kline.high, Decimal::new(4300000, 2));
        assert_eq!(kline.low, Decimal::new(4150000, 2));
        assert_eq!(kline.close, Decimal::new(4250000, 2));
    }

    #[test]
    fn test_deserialize_bybit_order() {
        let json = r#"{
            "orderId": "1234567890",
            "orderLinkId": "client123",
            "symbol": "BTCUSDT",
            "side": "Buy",
            "orderType": "Limit",
            "price": "42000.00",
            "qty": "0.1",
            "timeInForce": "GTC",
            "orderStatus": "New",
            "cumExecQty": "0",
            "cumExecValue": "0",
            "cumExecFee": "0",
            "avgPrice": "",
            "leavesQty": "0.1",
            "createdTime": "1671017382656",
            "updatedTime": "1671017382656"
        }"#;

        let order: BybitOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.order_id, "1234567890");
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.side, "Buy");
        assert_eq!(order.price, Some(Decimal::new(4200000, 2)));
    }

    #[test]
    fn test_deserialize_bybit_wallet_balance() {
        let json = r#"{
            "accountType": "UNIFIED",
            "totalEquity": "100000.00",
            "coin": [
                {
                    "coin": "USDT",
                    "walletBalance": "50000.00",
                    "availableToWithdraw": "45000.00"
                },
                {
                    "coin": "BTC",
                    "walletBalance": "1.5",
                    "availableToWithdraw": "1.0"
                }
            ]
        }"#;

        let account: BybitWalletAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.account_type, "UNIFIED");
        assert_eq!(account.coin.len(), 2);
        assert_eq!(account.coin[0].coin, "USDT");
        assert_eq!(account.coin[0].wallet_balance, Decimal::new(5000000, 2));
    }

    #[test]
    fn test_bybit_order_request_serialization() {
        let req = BybitOrderRequest {
            category: "spot".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "Buy".to_string(),
            order_type: "Limit".to_string(),
            qty: "0.1".to_string(),
            price: Some("42000.00".to_string()),
            time_in_force: Some("GTC".to_string()),
            order_link_id: None,
            market_unit: None,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"category\":\"spot\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(!json.contains("orderLinkId"));
    }
}
