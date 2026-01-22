//! Bithumb 전용 타입 및 API 응답 구조체.
//!
//! 이 타입들은 Bithumb API 응답을 역직렬화하는 데 사용되며,
//! 이후 공통 거래소 타입으로 변환됩니다.

use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

/// Bithumb API 에러 응답.
#[derive(Debug, Deserialize)]
pub struct BithumbError {
    pub error: BithumbErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct BithumbErrorDetail {
    pub name: String,
    pub message: String,
}

/// Bithumb 시세 응답.
#[derive(Debug, Deserialize)]
pub struct BithumbTicker {
    pub market: String,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub trade_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub opening_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub high_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub low_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub prev_closing_price: Decimal,
    pub change: String,
    #[serde(default, deserialize_with = "deserialize_decimal_from_number_opt")]
    pub change_rate: Decimal,
    #[serde(default, deserialize_with = "deserialize_decimal_from_number_opt")]
    pub change_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub acc_trade_volume_24h: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub acc_trade_price_24h: Decimal,
    #[serde(deserialize_with = "deserialize_timestamp_millis")]
    pub timestamp: DateTime<Utc>,
}

/// Bithumb 호가창 응답.
#[derive(Debug, Deserialize)]
pub struct BithumbOrderbook {
    pub market: String,
    #[serde(deserialize_with = "deserialize_timestamp_millis")]
    pub timestamp: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub total_ask_size: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub total_bid_size: Decimal,
    pub orderbook_units: Vec<BithumbOrderbookUnit>,
}

#[derive(Debug, Deserialize)]
pub struct BithumbOrderbookUnit {
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub ask_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub bid_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub ask_size: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub bid_size: Decimal,
}

/// Bithumb 캔들 응답 (분봉).
#[derive(Debug, Deserialize)]
pub struct BithumbCandle {
    pub market: String,
    pub candle_date_time_utc: String,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub opening_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub high_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub low_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub trade_price: Decimal,
    #[serde(deserialize_with = "deserialize_decimal_from_number")]
    pub candle_acc_trade_volume: Decimal,
    #[serde(default, deserialize_with = "deserialize_decimal_from_number_opt")]
    pub candle_acc_trade_price: Decimal,
}

/// Bithumb 계정 잔고 응답.
#[derive(Debug, Deserialize)]
pub struct BithumbBalance {
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

/// Bithumb 주문 응답.
#[derive(Debug, Deserialize)]
pub struct BithumbOrder {
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

/// Bithumb 주문 요청 본문.
#[derive(Debug, Serialize)]
pub struct BithumbOrderRequest {
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

/// 밀리초 타임스탬프를 역직렬화.
fn deserialize_timestamp_millis<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = i64::deserialize(deserializer)?;
    Utc.timestamp_millis_opt(millis)
        .single()
        .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))
}

/// 숫자에서 Decimal로 역직렬화.
fn deserialize_decimal_from_number<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    // 먼저 f64로 역직렬화를 시도한 후 Decimal로 변환
    let value = serde_json::Value::deserialize(deserializer)?;
    match &value {
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Decimal::from(i))
            } else if let Some(f) = n.as_f64() {
                Decimal::try_from(f).map_err(serde::de::Error::custom)
            } else {
                Err(serde::de::Error::custom("invalid number"))
            }
        }
        serde_json::Value::String(s) => s.parse::<Decimal>().map_err(serde::de::Error::custom),
        _ => Err(serde::de::Error::custom("expected number or string")),
    }
}

/// 기본값을 사용하여 숫자에서 Decimal로 역직렬화.
fn deserialize_decimal_from_number_opt<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match &value {
        serde_json::Value::Null => Ok(Decimal::ZERO),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Decimal::from(i))
            } else if let Some(f) = n.as_f64() {
                Decimal::try_from(f).map_err(serde::de::Error::custom)
            } else {
                Err(serde::de::Error::custom("invalid number"))
            }
        }
        serde_json::Value::String(s) if s.is_empty() => Ok(Decimal::ZERO),
        serde_json::Value::String(s) => s.parse::<Decimal>().map_err(serde::de::Error::custom),
        _ => Err(serde::de::Error::custom("expected number or string")),
    }
}

/// 문자열에서 Decimal로 역직렬화.
fn deserialize_decimal_string<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<Decimal>().map_err(serde::de::Error::custom)
}

/// 문자열에서 Option<Decimal>로 역직렬화.
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
    fn test_deserialize_bithumb_ticker() {
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

        let ticker: BithumbTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.market, "KRW-BTC");
        assert_eq!(ticker.trade_price, Decimal::from(148500000));
        assert_eq!(ticker.change, "RISE");
    }

    #[test]
    fn test_deserialize_bithumb_orderbook() {
        let json = r#"{
            "market": "KRW-BTC",
            "timestamp": 1676965262177,
            "total_ask_size": 10.5,
            "total_bid_size": 9.5,
            "orderbook_units": [
                {"ask_price": 148520000, "bid_price": 148490000, "ask_size": 0.01, "bid_size": 0.04}
            ]
        }"#;

        let ob: BithumbOrderbook = serde_json::from_str(json).unwrap();
        assert_eq!(ob.market, "KRW-BTC");
        assert_eq!(ob.orderbook_units.len(), 1);
        assert_eq!(ob.orderbook_units[0].ask_price, Decimal::from(148520000));
    }

    #[test]
    fn test_deserialize_bithumb_balance() {
        let json = r#"{
            "currency": "BTC",
            "balance": "0.5",
            "locked": "0.1",
            "avg_buy_price": "50000000",
            "avg_buy_price_modified": false,
            "unit_currency": "KRW"
        }"#;

        let balance: BithumbBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.currency, "BTC");
        assert_eq!(balance.balance, Decimal::new(5, 1));
    }
}
