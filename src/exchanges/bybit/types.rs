//! Bybit 전용 타입 및 API 응답 구조체.
//!
//! 이 타입들은 Bybit V5 API 응답을 역직렬화하는 데 사용되며,
//! 이후 공통 거래소 타입으로 변환됩니다.

use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

/// Bybit V5 API 응답 래퍼.
///
/// 모든 Bybit V5 API 응답은 다음 구조를 따릅니다:
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
    /// 반환 코드 (0 = 성공).
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    /// 반환 메시지.
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    /// 결과 데이터.
    pub result: T,
    /// 확장 정보.
    #[serde(rename = "retExtInfo", default)]
    pub ret_ext_info: serde_json::Value,
    /// 서버 타임스탬프 (밀리초).
    pub time: i64,
}

impl<T> BybitResponse<T> {
    /// 요청이 성공했으면 true를 반환합니다.
    #[inline]
    pub fn is_success(&self) -> bool {
        self.ret_code == 0
    }
}

/// Bybit 티커 목록 결과.
#[derive(Debug, Deserialize)]
pub struct BybitTickerList {
    pub category: String,
    pub list: Vec<BybitTicker>,
}

/// Bybit 티커 응답 (현물).
#[derive(Debug, Deserialize)]
pub struct BybitTicker {
    /// 심볼 (예: "BTCUSDT").
    pub symbol: String,
    /// 최근 체결가.
    #[serde(rename = "lastPrice", deserialize_with = "deserialize_decimal_string")]
    pub last_price: Decimal,
    /// 지수 가격 (현물에서는 비어있을 수 있음).
    #[serde(
        rename = "indexPrice",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub index_price: Option<Decimal>,
    /// 마크 가격 (현물에서는 비어있을 수 있음).
    #[serde(
        rename = "markPrice",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub mark_price: Option<Decimal>,
    /// 이전 24시간 종가.
    #[serde(
        rename = "prevPrice24h",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub prev_price_24h: Decimal,
    /// 24시간 가격 변동률.
    #[serde(
        rename = "price24hPcnt",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub price_24h_pcnt: Decimal,
    /// 24시간 최고가.
    #[serde(
        rename = "highPrice24h",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub high_price_24h: Decimal,
    /// 24시간 최저가.
    #[serde(
        rename = "lowPrice24h",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub low_price_24h: Decimal,
    /// 24시간 거래대금 (호가 통화 기준).
    #[serde(
        rename = "turnover24h",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub turnover_24h: Decimal,
    /// 24시간 거래량 (기준 통화 기준).
    #[serde(rename = "volume24h", deserialize_with = "deserialize_decimal_string")]
    pub volume_24h: Decimal,
    /// 최우선 매수 호가.
    #[serde(rename = "bid1Price", deserialize_with = "deserialize_decimal_string")]
    pub bid1_price: Decimal,
    /// 최우선 매수 수량.
    #[serde(rename = "bid1Size", deserialize_with = "deserialize_decimal_string")]
    pub bid1_size: Decimal,
    /// 최우선 매도 호가.
    #[serde(rename = "ask1Price", deserialize_with = "deserialize_decimal_string")]
    pub ask1_price: Decimal,
    /// 최우선 매도 수량.
    #[serde(rename = "ask1Size", deserialize_with = "deserialize_decimal_string")]
    pub ask1_size: Decimal,
}

/// Bybit 호가창 결과.
#[derive(Debug, Deserialize)]
pub struct BybitOrderbookResult {
    /// 심볼.
    #[serde(rename = "s")]
    pub symbol: String,
    /// 매도 호가 및 수량.
    #[serde(rename = "a")]
    pub asks: Vec<BybitOrderbookLevel>,
    /// 매수 호가 및 수량.
    #[serde(rename = "b")]
    pub bids: Vec<BybitOrderbookLevel>,
    /// 타임스탬프 (밀리초).
    #[serde(rename = "ts")]
    pub timestamp: i64,
    /// 업데이트 ID.
    #[serde(rename = "u")]
    pub update_id: i64,
}

/// Bybit 호가창 레벨 [가격, 수량].
#[derive(Debug, Deserialize)]
pub struct BybitOrderbookLevel(
    #[serde(deserialize_with = "deserialize_decimal_string")] pub Decimal,
    #[serde(deserialize_with = "deserialize_decimal_string")] pub Decimal,
);

/// Bybit K선/캔들 목록 결과.
#[derive(Debug, Deserialize)]
pub struct BybitKlineList {
    pub category: String,
    pub symbol: String,
    pub list: Vec<BybitKline>,
}

/// Bybit K선/캔들 데이터.
/// 응답은 배열 형태: [startTime, open, high, low, close, volume, turnover]
#[derive(Debug)]
pub struct BybitKline {
    /// 시작 시간 (밀리초).
    pub start_time: i64,
    /// 시가.
    pub open: Decimal,
    /// 고가.
    pub high: Decimal,
    /// 저가.
    pub low: Decimal,
    /// 종가.
    pub close: Decimal,
    /// 거래량.
    pub volume: Decimal,
    /// 거래대금.
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

/// Bybit 계정 지갑 잔고 결과.
#[derive(Debug, Deserialize)]
pub struct BybitWalletBalanceResult {
    pub list: Vec<BybitWalletAccount>,
}

/// Bybit 지갑 계정.
#[derive(Debug, Deserialize)]
pub struct BybitWalletAccount {
    /// 계정 유형 (UNIFIED, CONTRACT 등).
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// 총 자산.
    #[serde(
        rename = "totalEquity",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub total_equity: Option<Decimal>,
    /// 계정 코인 목록.
    pub coin: Vec<BybitCoinBalance>,
}

/// Bybit 코인 잔고.
#[derive(Debug, Deserialize)]
pub struct BybitCoinBalance {
    /// 코인 이름 (예: "BTC", "USDT").
    pub coin: String,
    /// 지갑 잔고.
    #[serde(
        rename = "walletBalance",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub wallet_balance: Decimal,
    /// 출금 가능 잔고.
    #[serde(
        rename = "availableToWithdraw",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub available_to_withdraw: Decimal,
    /// 동결 잔고 (주문 등에 사용 중).
    #[serde(default, deserialize_with = "deserialize_optional_decimal_string")]
    pub locked: Option<Decimal>,
    /// 계정 자기자본 (= walletBalance + unrealisedPnl).
    #[serde(default, deserialize_with = "deserialize_optional_decimal_string")]
    pub equity: Option<Decimal>,
    /// 미실현 손익.
    #[serde(
        rename = "unrealisedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub unrealised_pnl: Option<Decimal>,
    /// 누적 실현 손익.
    #[serde(
        rename = "cumRealisedPnl",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub cum_realised_pnl: Option<Decimal>,
}

/// Bybit 주문 목록 결과.
#[derive(Debug, Deserialize)]
pub struct BybitOrderList {
    pub category: String,
    pub list: Vec<BybitOrder>,
    #[serde(rename = "nextPageCursor", default)]
    pub next_page_cursor: Option<String>,
}

/// Bybit 주문 응답.
#[derive(Debug, Deserialize)]
pub struct BybitOrder {
    /// 주문 ID.
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// 클라이언트 주문 ID.
    #[serde(rename = "orderLinkId", default)]
    pub order_link_id: Option<String>,
    /// 심볼.
    pub symbol: String,
    /// 주문 방향: Buy, Sell.
    pub side: String,
    /// 주문 유형: Limit, Market.
    #[serde(rename = "orderType")]
    pub order_type: String,
    /// 주문 가격.
    #[serde(default, deserialize_with = "deserialize_optional_decimal_string")]
    pub price: Option<Decimal>,
    /// 주문 수량.
    #[serde(deserialize_with = "deserialize_decimal_string")]
    pub qty: Decimal,
    /// 주문 유효 기간: GTC, IOC, FOK, PostOnly.
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// 주문 상태.
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    /// 누적 체결 수량.
    #[serde(rename = "cumExecQty", deserialize_with = "deserialize_decimal_string")]
    pub cum_exec_qty: Decimal,
    /// 누적 체결 금액.
    #[serde(
        rename = "cumExecValue",
        deserialize_with = "deserialize_decimal_string"
    )]
    pub cum_exec_value: Decimal,
    /// 누적 체결 수수료.
    #[serde(rename = "cumExecFee", deserialize_with = "deserialize_decimal_string")]
    pub cum_exec_fee: Decimal,
    /// 평균 체결가.
    #[serde(
        rename = "avgPrice",
        default,
        deserialize_with = "deserialize_optional_decimal_string"
    )]
    pub avg_price: Option<Decimal>,
    /// 미체결 수량.
    #[serde(rename = "leavesQty", deserialize_with = "deserialize_decimal_string")]
    pub leaves_qty: Decimal,
    /// 주문 생성 시간 (밀리초).
    #[serde(
        rename = "createdTime",
        deserialize_with = "deserialize_timestamp_string"
    )]
    pub created_time: DateTime<Utc>,
    /// 주문 수정 시간 (밀리초).
    #[serde(
        rename = "updatedTime",
        deserialize_with = "deserialize_timestamp_string"
    )]
    pub updated_time: DateTime<Utc>,
}

/// Bybit 주문 생성 결과.
#[derive(Debug, Deserialize)]
pub struct BybitCreateOrderResult {
    /// 주문 ID.
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// 클라이언트 주문 ID.
    #[serde(rename = "orderLinkId", default)]
    pub order_link_id: Option<String>,
}

/// Bybit 주문 취소 결과.
#[derive(Debug, Deserialize)]
pub struct BybitCancelOrderResult {
    /// 주문 ID.
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// 클라이언트 주문 ID.
    #[serde(rename = "orderLinkId", default)]
    pub order_link_id: Option<String>,
}

/// Bybit 주문 생성 요청 본문.
#[derive(Debug, Serialize)]
pub struct BybitOrderRequest {
    /// 카테고리: spot, linear, inverse, option.
    pub category: String,
    /// 심볼 (예: "BTCUSDT").
    pub symbol: String,
    /// 주문 방향: Buy, Sell.
    pub side: String,
    /// 주문 유형: Limit, Market.
    #[serde(rename = "orderType")]
    pub order_type: String,
    /// 주문 수량.
    pub qty: String,
    /// 주문 가격 (지정가 주문 시 필수).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// 주문 유효 기간: GTC, IOC, FOK, PostOnly.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    /// 클라이언트 주문 ID.
    #[serde(rename = "orderLinkId", skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// 시장가 주문 시 단위 (baseCoin, quoteCoin).
    #[serde(rename = "marketUnit", skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
}

/// Bybit 주문 취소 요청 본문.
#[derive(Debug, Serialize)]
pub struct BybitCancelOrderRequest {
    /// 카테고리: spot, linear, inverse, option.
    pub category: String,
    /// 심볼.
    pub symbol: String,
    /// 주문 ID (orderId 또는 orderLinkId 중 하나 필수).
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 클라이언트 주문 ID.
    #[serde(rename = "orderLinkId", skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

/// 문자열에서 Decimal로 역직렬화.
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

/// 문자열에서 Optional Decimal로 역직렬화.
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

/// 문자열 밀리초에서 타임스탬프로 역직렬화.
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
        // equity 필드가 없으면 None
        assert!(account.coin[0].equity.is_none());
    }

    #[test]
    fn test_deserialize_bybit_wallet_balance_with_equity() {
        // equity, unrealisedPnl이 포함된 Bybit Unified 계정 응답
        let json = r#"{
            "accountType": "UNIFIED",
            "totalEquity": "51000.50",
            "coin": [
                {
                    "coin": "USDT",
                    "walletBalance": "50000.00",
                    "availableToWithdraw": "45000.00",
                    "equity": "51000.50",
                    "unrealisedPnl": "1000.50"
                }
            ]
        }"#;

        let account: BybitWalletAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.coin.len(), 1);
        let coin = &account.coin[0];
        assert_eq!(coin.coin, "USDT");
        assert_eq!(coin.wallet_balance, Decimal::new(5000000, 2));
        assert_eq!(coin.available_to_withdraw, Decimal::new(4500000, 2));
        assert_eq!(coin.equity, Some(Decimal::new(5100050, 2)));
        assert_eq!(coin.unrealised_pnl, Some(Decimal::new(100050, 2)));
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
