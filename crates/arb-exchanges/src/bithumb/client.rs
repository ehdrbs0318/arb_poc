//! Bithumb REST API 클라이언트 구현.
//!
//! 이 모듈은 Bithumb API와 상호작용하기 위한 메인 클라이언트를 제공합니다.

use arb_exchange::{
    Balance, Candle, CandleInterval, ExchangeError, ExchangeResult, MarketData, Order, OrderBook,
    OrderBookLevel, OrderManagement, OrderRequest, OrderSide, OrderStatus, OrderType, PriceChange,
    Ticker, TimeInForce,
};

use crate::bithumb::auth::{BithumbCredentials, build_query_string};
use crate::bithumb::types::{
    BithumbBalance, BithumbCandle, BithumbError, BithumbOrder, BithumbOrderRequest,
    BithumbOrderbook, BithumbTicker,
};
use crate::rate_limit::RateLimiter;
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, warn};

/// Bithumb REST API 기본 URL.
const BASE_URL: &str = "https://api.bithumb.com";

/// Bithumb API 레이트 리밋 (초당 요청 수).
/// 보수적으로 초당 8회 적용.
const BITHUMB_RATE_LIMIT: u32 = 8;
/// Bithumb API 최대 버스트 용량.
const BITHUMB_BURST: u32 = 2;

/// Bithumb API 클라이언트.
///
/// 이 클라이언트는 Public API와 Private API 모두 지원합니다.
/// Private API를 사용하려면 인증 정보를 제공해야 합니다.
pub struct BithumbClient {
    client: Client,
    pub(crate) credentials: Option<BithumbCredentials>,
    /// API 레이트 리밋터.
    limiter: Arc<RateLimiter>,
}

impl std::fmt::Debug for BithumbClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BithumbClient")
            .field("credentials", &self.credentials.is_some())
            .finish()
    }
}

impl BithumbClient {
    /// 인증되지 않은 새 Bithumb 클라이언트를 생성합니다.
    ///
    /// 이 클라이언트는 Public API만 접근할 수 있습니다.
    ///
    /// # 오류
    ///
    /// HTTP 클라이언트를 생성할 수 없는 경우 오류를 반환합니다.
    pub fn new() -> ExchangeResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(ExchangeError::HttpError)?;

        Ok(Self {
            client,
            credentials: None,
            limiter: Arc::new(RateLimiter::new("bithumb", BITHUMB_RATE_LIMIT, BITHUMB_BURST)),
        })
    }

    /// 인증된 새 Bithumb 클라이언트를 생성합니다.
    ///
    /// 이 클라이언트는 Public API와 Private API 모두 접근할 수 있습니다.
    ///
    /// # 인자
    ///
    /// * `access_key` - Bithumb API 액세스 키
    /// * `secret_key` - Bithumb API 시크릿 키
    ///
    /// # 오류
    ///
    /// HTTP 클라이언트를 생성할 수 없는 경우 오류를 반환합니다.
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
            limiter: Arc::new(RateLimiter::new("bithumb", BITHUMB_RATE_LIMIT, BITHUMB_BURST)),
        })
    }

    /// 인증 정보가 있으면 반환합니다.
    fn credentials(&self) -> ExchangeResult<&BithumbCredentials> {
        self.credentials
            .as_ref()
            .ok_or_else(|| ExchangeError::AuthError("Credentials not provided".to_string()))
    }

    /// Public 엔드포인트에 GET 요청을 보냅니다.
    async fn get_public<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> ExchangeResult<T> {
        self.limiter.acquire().await;
        let url = format!("{BASE_URL}{endpoint}");
        debug!(endpoint, "Bithumb public GET 요청");
        let mut request = self.client.get(&url);

        if let Some(params) = params {
            request = request.query(params);
        }

        let response = request.send().await.map_err(ExchangeError::HttpError)?;
        self.handle_response(response).await
    }

    /// Private 엔드포인트에 GET 요청을 보냅니다.
    async fn get_private<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> ExchangeResult<T> {
        self.limiter.acquire().await;
        let creds = self.credentials()?;
        let url = format!("{BASE_URL}{endpoint}");
        debug!(endpoint, "Bithumb private GET 요청");

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

    /// Private 엔드포인트에 POST 요청을 보냅니다.
    async fn post_private<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &impl serde::Serialize,
    ) -> ExchangeResult<T> {
        self.limiter.acquire().await;
        let creds = self.credentials()?;
        let url = format!("{BASE_URL}{endpoint}");
        debug!(endpoint, "Bithumb private POST 요청");

        // 해시를 위해 본문을 쿼리 스트링 형식으로 직렬화
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

    /// Private 엔드포인트에 DELETE 요청을 보냅니다.
    async fn delete_private<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> ExchangeResult<T> {
        self.limiter.acquire().await;
        let creds = self.credentials()?;
        let url = format!("{BASE_URL}{endpoint}");
        debug!(endpoint, "Bithumb private DELETE 요청");

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

    /// API 응답을 처리하고 오류를 변환합니다.
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> ExchangeResult<T> {
        let status = response.status();

        if status.is_success() {
            response.json::<T>().await.map_err(ExchangeError::HttpError)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            warn!(status = status.as_u16(), error = %error_text, "Bithumb API 에러 응답");

            // Bithumb 오류로 파싱 시도
            if let Ok(bithumb_error) = serde_json::from_str::<BithumbError>(&error_text) {
                return Err(self.convert_bithumb_error(status.as_u16(), &bithumb_error));
            }

            Err(ExchangeError::UnknownError {
                code: status.as_u16().to_string(),
                message: error_text,
            })
        }
    }

    /// Bithumb 오류를 ExchangeError로 변환합니다.
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

    /// 내부 마켓 형식을 Bithumb 형식으로 변환합니다.
    ///
    /// 내부 형식은 "{QUOTE}-{BASE}" 형식을 사용합니다 (예: "KRW-BTC").
    /// Bithumb은 "{COIN}_{PAYMENT}" 형식을 사용합니다 (예: "BTC_KRW").
    #[allow(dead_code)]
    pub fn to_bithumb_market(market: &str) -> String {
        // "KRW-BTC" -> "BTC_KRW"
        let parts: Vec<&str> = market.split('-').collect();
        if parts.len() == 2 {
            format!("{}_{}", parts[1], parts[0])
        } else {
            market.to_string()
        }
    }

    /// Bithumb 마켓 형식을 내부 형식으로 변환합니다.
    ///
    /// Bithumb은 "{COIN}_{PAYMENT}" 형식을 사용합니다 (예: "BTC_KRW").
    /// 내부 형식은 "{QUOTE}-{BASE}" 형식을 사용합니다 (예: "KRW-BTC").
    #[allow(dead_code)]
    pub fn from_bithumb_market(market: &str) -> String {
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

        let endpoint = bithumb_candle_endpoint(interval);

        let candles: Vec<BithumbCandle> = self.get_public(endpoint, Some(&params)).await?;

        // Bithumb은 최신순으로 반환하므로 오름차순으로 정렬
        let mut result: Vec<Candle> = candles.into_iter().map(convert_candle).collect();
        result.reverse();
        Ok(result)
    }

    async fn get_candles_before(
        &self,
        _market: &str,
        _interval: CandleInterval,
        _count: u32,
        _before: DateTime<Utc>,
    ) -> ExchangeResult<Vec<Candle>> {
        warn!("Bithumb get_candles_before 미구현: API 미지원");
        Err(ExchangeError::InternalError(
            "get_candles_before not implemented for Bithumb".to_string(),
        ))
    }

    async fn get_all_tickers(&self) -> ExchangeResult<Vec<Ticker>> {
        // Bithumb은 자동 코인 선택에 사용되지 않음
        Err(ExchangeError::Unsupported(
            "get_all_tickers not implemented for Bithumb".into(),
        ))
    }

    fn market_code(base: &str, quote: &str) -> String {
        // Bithumb 형식: "{QUOTE}-{BASE}" (예: "KRW-BTC")
        format!("{}-{}", quote.to_uppercase(), base.to_uppercase())
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

/// CandleInterval에 대응하는 Bithumb API 엔드포인트를 반환합니다.
fn bithumb_candle_endpoint(interval: CandleInterval) -> &'static str {
    match interval {
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
    }
}

// 변환 함수들

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

    // 매도호가는 오름차순, 매수호가는 내림차순 정렬
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
        // 내부 형식에서 Bithumb 형식으로
        assert_eq!(BithumbClient::to_bithumb_market("KRW-BTC"), "BTC_KRW");
        assert_eq!(BithumbClient::to_bithumb_market("BTC-ETH"), "ETH_BTC");

        // Bithumb 형식에서 내부 형식으로
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

    #[test]
    fn test_market_code() {
        // Bithumb 형식: "{QUOTE}-{BASE}"
        assert_eq!(BithumbClient::market_code("BTC", "KRW"), "KRW-BTC");
        assert_eq!(BithumbClient::market_code("ETH", "KRW"), "KRW-ETH");
        assert_eq!(BithumbClient::market_code("btc", "krw"), "KRW-BTC");
    }

    #[test]
    fn test_bithumb_candle_endpoint() {
        assert_eq!(
            bithumb_candle_endpoint(CandleInterval::Minute1),
            "/v1/candles/minutes/1"
        );
        assert_eq!(
            bithumb_candle_endpoint(CandleInterval::Minute5),
            "/v1/candles/minutes/5"
        );
        assert_eq!(
            bithumb_candle_endpoint(CandleInterval::Day),
            "/v1/candles/days"
        );
        assert_eq!(
            bithumb_candle_endpoint(CandleInterval::Week),
            "/v1/candles/weeks"
        );
        assert_eq!(
            bithumb_candle_endpoint(CandleInterval::Month),
            "/v1/candles/months"
        );
    }
}
