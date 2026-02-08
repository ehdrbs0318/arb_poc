//! Bybit V5 REST API 클라이언트 구현.
//!
//! 이 모듈은 Bybit V5 API와 상호작용하기 위한 메인 클라이언트를 제공합니다.

use crate::bybit::auth::{AuthHeaders, BybitCredentials, build_query_string};
use crate::bybit::stream::BybitStreamInner;
use crate::bybit::types::{
    BybitCancelOrderRequest, BybitCancelOrderResult, BybitCreateOrderResult, BybitKlineList,
    BybitOrder, BybitOrderList, BybitOrderRequest, BybitOrderbookResult, BybitResponse,
    BybitTickerList, BybitWalletBalanceResult,
};
use arb_exchange::{
    Balance, Candle, CandleInterval, ExchangeError, ExchangeResult, MarketData, Order, OrderBook,
    OrderBookLevel, OrderManagement, OrderRequest, OrderSide, OrderStatus, OrderType, PriceChange,
    StreamConfig, Ticker, TimeInForce,
};
use chrono::{DateTime, TimeZone, Utc};
use reqwest::Client;
use rust_decimal::Decimal;
use std::sync::Arc;
use tracing::{debug, warn};

/// Bybit V5 REST API 기본 URL (메인넷).
const BASE_URL_MAINNET: &str = "https://api.bybit.com";

/// Bybit V5 REST API 기본 URL (테스트넷).
const BASE_URL_TESTNET: &str = "https://api-testnet.bybit.com";

/// 현물 거래 기본 카테고리.
const DEFAULT_CATEGORY: &str = "spot";

/// Bybit V5 API 클라이언트.
///
/// 이 클라이언트는 공개(시장 데이터) 및 비공개(거래) API를 모두 지원합니다.
/// 비공개 API를 사용하려면 인증 정보를 제공해야 합니다.
pub struct BybitClient {
    client: Client,
    pub(crate) credentials: Option<BybitCredentials>,
    base_url: String,
    category: String,
    /// WebSocket 스트림 내부 상태.
    pub(crate) stream: Arc<BybitStreamInner>,
}

impl std::fmt::Debug for BybitClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BybitClient")
            .field("base_url", &self.base_url)
            .field("category", &self.category)
            .field("credentials", &self.credentials.is_some())
            .finish()
    }
}

impl BybitClient {
    /// 메인넷용 인증되지 않은 새 Bybit 클라이언트를 생성합니다.
    ///
    /// 이 클라이언트는 공개(시장 데이터) API만 접근할 수 있습니다.
    ///
    /// # 에러
    ///
    /// HTTP 클라이언트를 생성할 수 없는 경우 에러를 반환합니다.
    pub fn new() -> ExchangeResult<Self> {
        Self::new_internal(None, false)
    }

    /// 테스트넷용 인증되지 않은 새 Bybit 클라이언트를 생성합니다.
    ///
    /// 이 클라이언트는 테스트넷의 공개(시장 데이터) API만 접근할 수 있습니다.
    ///
    /// # 에러
    ///
    /// HTTP 클라이언트를 생성할 수 없는 경우 에러를 반환합니다.
    pub fn new_testnet() -> ExchangeResult<Self> {
        Self::new_internal(None, true)
    }

    /// 메인넷용 인증된 새 Bybit 클라이언트를 생성합니다.
    ///
    /// 이 클라이언트는 공개 및 비공개 API 모두 접근할 수 있습니다.
    ///
    /// # 인자
    ///
    /// * `api_key` - Bybit API 키
    /// * `secret_key` - Bybit API 시크릿 키
    ///
    /// # 에러
    ///
    /// HTTP 클라이언트를 생성할 수 없는 경우 에러를 반환합니다.
    pub fn with_credentials(
        api_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> ExchangeResult<Self> {
        let creds = BybitCredentials::new(api_key, secret_key);
        Self::new_internal(Some(creds), false)
    }

    /// 테스트넷용 인증된 새 Bybit 클라이언트를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `api_key` - Bybit API 키
    /// * `secret_key` - Bybit API 시크릿 키
    ///
    /// # 에러
    ///
    /// HTTP 클라이언트를 생성할 수 없는 경우 에러를 반환합니다.
    pub fn with_credentials_testnet(
        api_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> ExchangeResult<Self> {
        let creds = BybitCredentials::new(api_key, secret_key);
        Self::new_internal(Some(creds), true)
    }

    /// 내부 생성자.
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
            stream: Arc::new(BybitStreamInner::new(StreamConfig::default())),
        })
    }

    /// WebSocket 스트림 내부 상태에 접근합니다.
    pub(crate) fn stream_inner(&self) -> &BybitStreamInner {
        &self.stream
    }

    /// 거래 카테고리를 설정합니다 (spot, linear, inverse, option).
    ///
    /// # 인자
    ///
    /// * `category` - 거래 카테고리
    #[must_use]
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    /// 사용 가능한 경우 인증 정보를 반환합니다.
    fn credentials(&self) -> ExchangeResult<&BybitCredentials> {
        self.credentials
            .as_ref()
            .ok_or_else(|| ExchangeError::AuthError("Credentials not provided".to_string()))
    }

    /// 공개 엔드포인트에 GET 요청을 보냅니다.
    async fn get_public<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> ExchangeResult<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!(endpoint, "Bybit public GET 요청");
        let response = self
            .client
            .get(&url)
            .query(params)
            .send()
            .await
            .map_err(ExchangeError::HttpError)?;

        self.handle_response(response).await
    }

    /// 비공개 엔드포인트에 GET 요청을 보냅니다.
    async fn get_private<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> ExchangeResult<T> {
        let creds = self.credentials()?;
        let url = format!("{}{}", self.base_url, endpoint);
        debug!(endpoint, "Bybit private GET 요청");

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

    /// 비공개 엔드포인트에 POST 요청을 보냅니다.
    async fn post_private<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &impl serde::Serialize,
    ) -> ExchangeResult<T> {
        let creds = self.credentials()?;
        let url = format!("{}{}", self.base_url, endpoint);
        debug!(endpoint, "Bybit private POST 요청");

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

    /// API 응답을 처리하고 에러를 변환합니다.
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> ExchangeResult<T> {
        let status = response.status();
        let body = response.text().await.map_err(ExchangeError::HttpError)?;

        if !status.is_success() {
            warn!(status = status.as_u16(), "Bybit API HTTP 에러");
            return Err(self.parse_error(&body, status.as_u16()));
        }

        // Bybit 응답 래퍼 파싱
        let bybit_resp: BybitResponse<T> =
            serde_json::from_str(&body).map_err(ExchangeError::JsonError)?;

        if !bybit_resp.is_success() {
            warn!(ret_code = bybit_resp.ret_code, ret_msg = %bybit_resp.ret_msg, "Bybit API 비즈니스 에러");
            return Err(self.convert_bybit_error(bybit_resp.ret_code, &bybit_resp.ret_msg));
        }

        Ok(bybit_resp.result)
    }

    /// 응답 본문에서 에러를 파싱합니다.
    fn parse_error(&self, body: &str, status: u16) -> ExchangeError {
        if let Ok(resp) = serde_json::from_str::<BybitResponse<serde_json::Value>>(body) {
            return self.convert_bybit_error(resp.ret_code, &resp.ret_msg);
        }

        ExchangeError::UnknownError {
            code: status.to_string(),
            message: body.to_string(),
        }
    }

    /// Bybit 에러 코드를 ExchangeError로 변환합니다.
    fn convert_bybit_error(&self, ret_code: i32, message: &str) -> ExchangeError {
        match ret_code {
            // 인증 에러
            10003 | 10004 | 10005 | 33004 => ExchangeError::AuthError(message.to_string()),
            // 잘못된 파라미터 (10001은 마켓을 찾을 수 없는 경우에도 사용되며, 주문에서 처리됨)
            10002 | 10016 => ExchangeError::InvalidParameter(message.to_string()),
            // 잔고 부족
            110007 | 110011 | 110012 => ExchangeError::InsufficientFunds(message.to_string()),
            // 주문을 찾을 수 없음
            110001 | 20001 => ExchangeError::OrderNotFound(message.to_string()),
            // 마켓을 찾을 수 없음 / 잘못된 파라미터 (모호한 에러 코드)
            10001 => ExchangeError::InvalidParameter(message.to_string()),
            // 요청 제한 초과
            10006 | 10018 => ExchangeError::RateLimitExceeded(message.to_string()),
            // 시스템 에러
            10000 | 10010 => ExchangeError::InternalError(message.to_string()),
            // 알 수 없음
            _ => ExchangeError::UnknownError {
                code: ret_code.to_string(),
                message: message.to_string(),
            },
        }
    }

    /// Bybit 심볼 형식을 공통 마켓 형식으로 변환합니다.
    ///
    /// Bybit은 "BTCUSDT" 형식을 사용하며, "USDT-BTC" 형식으로 변환해야 합니다.
    fn to_market_code(symbol: &str) -> String {
        // 일반적인 quote 통화 (우선순위 순)
        let quotes = ["USDT", "USDC", "BTC", "ETH", "EUR", "DAI"];

        for quote in quotes {
            if let Some(base) = symbol.strip_suffix(quote) {
                return format!("{}-{}", quote, base);
            }
        }

        // 폴백: 그대로 반환
        symbol.to_string()
    }

    /// 공통 마켓 형식을 Bybit 심볼 형식으로 변환합니다.
    ///
    /// 공통 형식 "USDT-BTC" -> Bybit "BTCUSDT"
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

        // Bybit는 최신순으로 반환하므로 오름차순으로 정렬
        let mut candles: Vec<Candle> = result
            .list
            .into_iter()
            .map(|k| convert_candle(k, market))
            .collect();
        candles.reverse();
        Ok(candles)
    }

    async fn get_candles_before(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
        before: DateTime<Utc>,
    ) -> ExchangeResult<Vec<Candle>> {
        let symbol = Self::to_bybit_symbol(market);
        let interval_str = interval_to_bybit(interval);
        let limit_str = count.min(1000).to_string();
        // Bybit의 `end`는 inclusive (밀리초)이므로 1ms를 빼서 exclusive 시맨틱을 구현
        let end_ms = (before.timestamp_millis() - 1).to_string();

        let params = [
            ("category", self.category.as_str()),
            ("symbol", &symbol),
            ("interval", interval_str),
            ("limit", &limit_str),
            ("end", &end_ms),
        ];

        let result: BybitKlineList = self.get_public("/v5/market/kline", &params).await?;

        // Bybit는 최신순으로 반환하므로 오름차순으로 정렬
        let mut candles: Vec<Candle> = result
            .list
            .into_iter()
            .map(|k| convert_candle(k, market))
            .collect();
        candles.reverse();
        Ok(candles)
    }

    fn market_code(base: &str, quote: &str) -> String {
        // Bybit 형식: "{BASE}{QUOTE}" (예: "BTCUSDT")
        format!("{}{}", base.to_uppercase(), quote.to_uppercase())
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
            OrderType::Best => "Limit", // Bybit에는 "best" 주문 유형이 없음
        };

        let time_in_force = request.time_in_force.map(|tif| match tif {
            TimeInForce::Gtc => "GTC",
            TimeInForce::Ioc => "IOC",
            TimeInForce::Fok => "FOK",
            TimeInForce::PostOnly => "PostOnly",
        });

        // 가격이 있는 시장가 매수 주문의 경우 (총 금액), market_unit으로 quoteCoin 사용
        let (qty, market_unit) = if request.order_type == OrderType::Price {
            // 총 quote 금액으로 시장가 매수
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

        // 전체 주문 상세 정보 조회
        self.get_order(&result.order_id).await
    }

    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<Order> {
        // 먼저 주문을 조회하여 심볼 확인
        let order = self.get_order(order_id).await?;
        let symbol = Self::to_bybit_symbol(&order.market);

        let body = BybitCancelOrderRequest {
            category: self.category.clone(),
            symbol,
            order_id: Some(order_id.to_string()),
            order_link_id: None,
        };

        let _result: BybitCancelOrderResult = self.post_private("/v5/order/cancel", &body).await?;

        // 업데이트된 주문 반환
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

// 변환 함수들

fn convert_ticker(t: crate::bybit::types::BybitTicker, market: &str) -> Ticker {
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
        opening_price: t.prev_price_24h, // Bybit은 정확한 시가를 제공하지 않음
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

fn convert_candle(k: crate::bybit::types::BybitKline, market: &str) -> Candle {
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

fn convert_balance(b: crate::bybit::types::BybitCoinBalance) -> Balance {
    let locked = b.wallet_balance - b.available_to_withdraw;

    Balance {
        currency: b.coin,
        balance: b.available_to_withdraw,
        locked,
        avg_buy_price: Decimal::ZERO, // Bybit은 이 정보를 제공하지 않음
        unit_currency: "USDT".to_string(), // 기본값 USDT
    }
}

/// CandleInterval을 Bybit 간격 문자열로 변환합니다.
fn interval_to_bybit(interval: CandleInterval) -> &'static str {
    match interval {
        CandleInterval::Minute1 => "1",
        CandleInterval::Minute3 => "3",
        CandleInterval::Minute5 => "5",
        CandleInterval::Minute10 => "15", // Bybit에는 10분 간격이 없어 15분 사용
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
    fn test_market_code() {
        // Bybit 형식: "{BASE}{QUOTE}"
        assert_eq!(BybitClient::market_code("BTC", "USDT"), "BTCUSDT");
        assert_eq!(BybitClient::market_code("ETH", "USDC"), "ETHUSDC");
        assert_eq!(BybitClient::market_code("btc", "usdt"), "BTCUSDT");
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
