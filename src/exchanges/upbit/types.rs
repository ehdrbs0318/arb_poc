//! Upbit-specific types and API response structures.
//!
//! These types are used for deserializing Upbit API responses
//! and are then converted to the common exchange types.

use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

/// Upbit API error response.
#[derive(Debug, Deserialize)]
pub struct UpbitError {
    pub error: UpbitErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct UpbitErrorDetail {
    pub name: String,
    pub message: String,
}

/// Upbit ticker response.
#[derive(Debug, Deserialize)]
pub struct UpbitTicker {
    pub market: String,
    pub trade_price: Decimal,
    pub opening_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub prev_closing_price: Decimal,
    pub change: String,
    #[serde(default)]
    pub change_rate: Decimal,
    #[serde(default)]
    pub change_price: Decimal,
    pub acc_trade_volume_24h: Decimal,
    pub acc_trade_price_24h: Decimal,
    #[serde(deserialize_with = "deserialize_timestamp_millis")]
    pub timestamp: DateTime<Utc>,
}

/// Upbit orderbook response.
#[derive(Debug, Deserialize)]
pub struct UpbitOrderbook {
    pub market: String,
    #[serde(deserialize_with = "deserialize_timestamp_millis")]
    pub timestamp: DateTime<Utc>,
    pub total_ask_size: Decimal,
    pub total_bid_size: Decimal,
    pub orderbook_units: Vec<UpbitOrderbookUnit>,
}

#[derive(Debug, Deserialize)]
pub struct UpbitOrderbookUnit {
    pub ask_price: Decimal,
    pub bid_price: Decimal,
    pub ask_size: Decimal,
    pub bid_size: Decimal,
}

/// Upbit candle response (minutes).
#[derive(Debug, Deserialize)]
pub struct UpbitCandle {
    pub market: String,
    pub candle_date_time_utc: String,
    pub opening_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub trade_price: Decimal,
    pub candle_acc_trade_volume: Decimal,
    #[serde(default)]
    pub candle_acc_trade_price: Decimal,
}

/// Upbit account balance response.
#[derive(Debug, Deserialize)]
pub struct UpbitBalance {
    pub currency: String,
    #[serde(deserialize_with = "deserialize_decimal_string")]
    pub balance: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_string")]
    pub locked: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_string")]
    pub avg_buy_price: Decimal,
    #[serde(default)]
    pub avg_buy_price_modified: bool,
    pub unit_currency: String,
}

/// Upbit order response.
#[derive(Debug, Deserialize)]
pub struct UpbitOrder {
    pub uuid: String,
    pub market: String,
    pub side: String,
    pub ord_type: String,
    pub state: String,
    #[serde(deserialize_with = "deserialize_decimal_string")]
    pub volume: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_string")]
    pub remaining_volume: Decimal,
    #[serde(default, deserialize_with = "deserialize_optional_decimal_string")]
    pub executed_volume: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_optional_decimal_string")]
    pub price: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_optional_decimal_string")]
    pub avg_price: Option<Decimal>,
    #[serde(deserialize_with = "deserialize_decimal_string")]
    pub paid_fee: Decimal,
    pub created_at: String,
    #[serde(default)]
    pub identifier: Option<String>,
}

/// Upbit order request body.
#[derive(Debug, Serialize)]
pub struct UpbitOrderRequest {
    pub market: String,
    pub side: String,
    pub ord_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

/// Deserialize timestamp from milliseconds.
fn deserialize_timestamp_millis<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = i64::deserialize(deserializer)?;
    Utc.timestamp_millis_opt(millis)
        .single()
        .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))
}

/// Deserialize decimal from string.
fn deserialize_decimal_string<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_upbit_ticker() {
        let json = r#"{
            "market": "KRW-BTC",
            "trade_price": 148500000,
            "opening_price": 148000000,
            "high_price": 149000000,
            "low_price": 147500000,
            "prev_closing_price": 148000000,
            "change": "RISE",
            "change_rate": 0.003,
            "change_price": 500000,
            "acc_trade_volume_24h": 1234.5678,
            "acc_trade_price_24h": 183456789012,
            "timestamp": 1676965262177
        }"#;

        let ticker: UpbitTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.market, "KRW-BTC");
        assert_eq!(ticker.trade_price, Decimal::from(148500000));
        assert_eq!(ticker.change, "RISE");
    }

    #[test]
    fn test_deserialize_upbit_orderbook() {
        let json = r#"{
            "market": "KRW-BTC",
            "timestamp": 1676965262177,
            "total_ask_size": 10.5,
            "total_bid_size": 9.5,
            "orderbook_units": [
                {"ask_price": 148520000, "bid_price": 148490000, "ask_size": 0.01, "bid_size": 0.04}
            ]
        }"#;

        let ob: UpbitOrderbook = serde_json::from_str(json).unwrap();
        assert_eq!(ob.market, "KRW-BTC");
        assert_eq!(ob.orderbook_units.len(), 1);
        assert_eq!(ob.orderbook_units[0].ask_price, Decimal::from(148520000));
    }

    #[test]
    fn test_deserialize_upbit_balance() {
        let json = r#"{
            "currency": "BTC",
            "balance": "0.5",
            "locked": "0.1",
            "avg_buy_price": "50000000",
            "avg_buy_price_modified": false,
            "unit_currency": "KRW"
        }"#;

        let balance: UpbitBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.currency, "BTC");
        assert_eq!(balance.balance, Decimal::new(5, 1));
    }
}
