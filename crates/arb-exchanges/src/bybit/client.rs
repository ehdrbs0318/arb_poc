//! Bybit V5 REST API 클라이언트 구현.
//!
//! 이 모듈은 Bybit V5 API와 상호작용하기 위한 메인 클라이언트를 제공합니다.

use crate::bybit::auth::{AuthHeaders, BybitCredentials, build_query_string};
use crate::bybit::stream::BybitStreamInner;
use crate::bybit::types::{
    BybitCancelOrderRequest, BybitCancelOrderResult, BybitCreateOrderResult,
    BybitInstrumentInfoList, BybitKlineList, BybitLinearTickerList, BybitOrder, BybitOrderList,
    BybitOrderRequest, BybitOrderbookResult, BybitPositionList, BybitResponse,
    BybitSetLeverageRequest, BybitSwitchIsolatedRequest, BybitTickerList, BybitWalletBalanceResult,
    LinearTickerInfo,
};
use crate::rate_limit::RateLimiter;
use arb_exchange::{
    Balance, Candle, CandleInterval, ExchangeError, ExchangeResult, InstrumentDataProvider,
    InstrumentInfoResponse, MarketData, Order, OrderBook, OrderBookLevel, OrderManagement,
    OrderRequest, OrderSide, OrderStatus, OrderType, PositionInfo, PriceChange, StreamConfig,
    Ticker, TimeInForce,
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

/// Bybit 공개 API 레이트 리밋 (초당 요청 수).
/// 시세, 오더북, 캔들 등 공개 엔드포인트 전용.
const BYBIT_PUBLIC_RATE_LIMIT: u32 = 10;
/// 공개 API 최대 버스트 용량.
const BYBIT_PUBLIC_BURST: u32 = 3;

/// Bybit 비공개 API 레이트 리밋 (초당 요청 수).
/// 주문, 잔고, 포지션 등 인증 필요 엔드포인트 전용.
const BYBIT_PRIVATE_RATE_LIMIT: u32 = 10;
/// 비공개 API 최대 버스트 용량.
const BYBIT_PRIVATE_BURST: u32 = 3;

/// Bybit 긴급 API 레이트 리밋 (초당 요청 수).
/// kill switch 청산 등 긴급 상황에서만 사용.
const BYBIT_EMERGENCY_RATE_LIMIT: u32 = 20;
/// 긴급 API 최대 버스트 용량.
const BYBIT_EMERGENCY_BURST: u32 = 5;

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
    /// 공개 API (시세, 오더북) 레이트 리밋터.
    public_limiter: Arc<RateLimiter>,
    /// 비공개 API (주문, 잔고, 포지션) 레이트 리밋터.
    private_limiter: Arc<RateLimiter>,
    /// 긴급 API (kill switch 청산) 레이트 리밋터.
    /// 향후 LiveExecutor의 kill switch에서 사용됩니다.
    #[allow(dead_code)]
    emergency_limiter: Arc<RateLimiter>,
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

impl Clone for BybitClient {
    /// 클라이언트를 복제합니다.
    ///
    /// reqwest::Client와 모든 Arc 필드는 내부적으로 참조 카운팅이므로
    /// 실제 리소스(커넥션 풀, rate limiter 상태 등)를 공유합니다.
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            credentials: self.credentials.clone(),
            base_url: self.base_url.clone(),
            category: self.category.clone(),
            stream: Arc::clone(&self.stream),
            public_limiter: Arc::clone(&self.public_limiter),
            private_limiter: Arc::clone(&self.private_limiter),
            emergency_limiter: Arc::clone(&self.emergency_limiter),
        }
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
            public_limiter: Arc::new(RateLimiter::new(
                "bybit-public",
                BYBIT_PUBLIC_RATE_LIMIT,
                BYBIT_PUBLIC_BURST,
            )),
            private_limiter: Arc::new(RateLimiter::new(
                "bybit-private",
                BYBIT_PRIVATE_RATE_LIMIT,
                BYBIT_PRIVATE_BURST,
            )),
            emergency_limiter: Arc::new(RateLimiter::new(
                "bybit-emergency",
                BYBIT_EMERGENCY_RATE_LIMIT,
                BYBIT_EMERGENCY_BURST,
            )),
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
        self.public_limiter.acquire().await;
        let url = format!("{}{}", self.base_url, endpoint);
        debug!(endpoint, ?params, "Bybit public GET 요청");
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
        self.private_limiter.acquire().await;
        let creds = self.credentials()?;
        let url = format!("{}{}", self.base_url, endpoint);
        debug!(endpoint, ?params, "Bybit private GET 요청");

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
        self.private_limiter.acquire().await;
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
        let url = response.url().to_string();
        let body = response.text().await.map_err(ExchangeError::HttpError)?;

        debug!(
            status = status.as_u16(),
            url = %url,
            body_len = body.len(),
            body_preview = %if body.len() > 200 { &body[..200] } else { &body },
            "Bybit API 응답 수신"
        );

        if !status.is_success() {
            warn!(status = status.as_u16(), body = %body, "Bybit API HTTP 에러");
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

    async fn get_all_tickers(&self) -> ExchangeResult<Vec<Ticker>> {
        // 단일 호출로 전종목 티커 조회 (category만 지정, symbol 생략)
        let params = [("category", self.category.as_str())];

        let result: BybitTickerList = self.get_public("/v5/market/tickers", &params).await?;

        // USDT 페어만 필터하여 변환
        let tickers = result
            .list
            .into_iter()
            .filter(|t| t.symbol.ends_with("USDT"))
            .map(|t| {
                let market = Self::to_market_code(&t.symbol);
                convert_ticker(t, &market)
            })
            .collect();

        Ok(tickers)
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
            reduce_only: None,
            position_idx: None,
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

impl InstrumentDataProvider for BybitClient {
    async fn get_instrument_info(&self, symbol: &str) -> ExchangeResult<InstrumentInfoResponse> {
        // category="linear" 하드코딩 (선물 계약의 instrument info).
        // self.category는 "spot"이므로 사용하지 않음.
        let result: BybitInstrumentInfoList = self
            .get_public(
                "/v5/market/instruments-info",
                &[("category", "linear"), ("symbol", symbol)],
            )
            .await?;

        let item = result.list.into_iter().next().ok_or_else(|| {
            ExchangeError::ApiError(format!("No instrument info found for symbol: {}", symbol))
        })?;

        // 문자열 -> Decimal 변환
        let tick_size = item
            .price_filter
            .tick_size
            .parse::<Decimal>()
            .map_err(|e| ExchangeError::ParseError(format!("tick_size parse: {}", e)))?;
        let qty_step = item
            .lot_size_filter
            .qty_step
            .parse::<Decimal>()
            .map_err(|e| ExchangeError::ParseError(format!("qty_step parse: {}", e)))?;
        let min_order_qty = item
            .lot_size_filter
            .min_order_qty
            .parse::<Decimal>()
            .map_err(|e| ExchangeError::ParseError(format!("min_order_qty parse: {}", e)))?;
        let max_order_qty = item
            .lot_size_filter
            .max_order_qty
            .parse::<Decimal>()
            .map_err(|e| ExchangeError::ParseError(format!("max_order_qty parse: {}", e)))?;
        let min_notional = item
            .lot_size_filter
            .min_notional_value
            .unwrap_or_else(|| "5".to_string())
            .parse::<Decimal>()
            .map_err(|e| ExchangeError::ParseError(format!("min_notional parse: {}", e)))?;

        Ok(InstrumentInfoResponse {
            tick_size,
            qty_step,
            min_order_qty,
            max_order_qty,
            min_notional,
        })
    }
}

impl arb_exchange::LinearOrderManagement for BybitClient {
    async fn place_order_linear(
        &self,
        request: &OrderRequest,
        reduce_only: bool,
    ) -> ExchangeResult<Order> {
        self.place_order_linear_impl(request, reduce_only).await
    }

    async fn get_order_linear(&self, order_id: &str) -> ExchangeResult<Order> {
        self.get_order_linear_impl(order_id).await
    }

    async fn cancel_order_linear(
        &self,
        order_id: &str,
        symbol: Option<&str>,
    ) -> ExchangeResult<Order> {
        self.cancel_order_linear_impl(order_id, symbol).await
    }
}

/// Bybit 선물(linear) 전용 API 메서드.
impl BybitClient {
    /// 선물(linear) 주문을 생성합니다.
    ///
    /// # 인자
    ///
    /// * `request` - 주문 요청 파라미터
    /// * `reduce_only` - 포지션 감소 전용 주문 여부 (true이면 기존 포지션을 줄이는 방향으로만 체결)
    ///
    /// # 반환값
    ///
    /// 생성된 주문 정보를 반환합니다.
    pub async fn place_order_linear_impl(
        &self,
        request: &OrderRequest,
        reduce_only: bool,
    ) -> ExchangeResult<Order> {
        let symbol = Self::to_bybit_symbol(&request.market);

        let side = match request.side {
            OrderSide::Buy => "Buy",
            OrderSide::Sell => "Sell",
        };

        let order_type = match request.order_type {
            OrderType::Limit => "Limit",
            OrderType::Market | OrderType::Price => "Market",
            OrderType::Best => "Limit",
        };

        let time_in_force = request.time_in_force.map(|tif| match tif {
            TimeInForce::Gtc => "GTC",
            TimeInForce::Ioc => "IOC",
            TimeInForce::Fok => "FOK",
            TimeInForce::PostOnly => "PostOnly",
        });

        let qty = request.volume.unwrap_or(Decimal::ZERO).to_string();

        let body = BybitOrderRequest {
            category: "linear".to_string(),
            symbol: symbol.clone(),
            side: side.to_string(),
            order_type: order_type.to_string(),
            qty,
            price: request.price.map(|p| p.to_string()),
            time_in_force: time_in_force.map(|s| s.to_string()),
            order_link_id: request.identifier.clone(),
            market_unit: None,
            reduce_only: Some(reduce_only),
            position_idx: Some(0), // one-way mode
        };

        debug!(
            symbol = %symbol,
            side = %side,
            qty = %body.qty,
            price = ?request.price,
            order_type = %order_type,
            time_in_force = ?time_in_force,
            reduce_only,
            "Bybit linear 주문 생성 요청"
        );

        let result: BybitCreateOrderResult = self.post_private("/v5/order/create", &body).await?;

        debug!(
            order_id = %result.order_id,
            order_link_id = ?result.order_link_id,
            "Bybit linear 주문 생성 완료"
        );

        self.get_order_linear_impl(&result.order_id).await
    }

    /// 선물(linear) 주문을 조회합니다.
    ///
    /// # 인자
    ///
    /// * `order_id` - 조회할 주문 ID
    pub async fn get_order_linear_impl(&self, order_id: &str) -> ExchangeResult<Order> {
        let params = [("category", "linear"), ("orderId", order_id)];

        debug!(order_id, "Bybit linear 주문 조회 요청");

        let result: BybitOrderList = self.get_private("/v5/order/realtime", &params).await?;

        result
            .list
            .into_iter()
            .next()
            .map(convert_order)
            .ok_or_else(|| ExchangeError::OrderNotFound(order_id.to_string()))
    }

    /// 선물(linear) 주문을 취소합니다.
    ///
    /// # 인자
    ///
    /// * `order_id` - 취소할 주문 ID
    /// * `symbol` - 심볼 (예: "BTCUSDT"). 지정하면 주문 조회를 생략하여 API 호출 1회 절약.
    pub async fn cancel_order_linear_impl(
        &self,
        order_id: &str,
        symbol: Option<&str>,
    ) -> ExchangeResult<Order> {
        let bybit_symbol = if let Some(s) = symbol {
            s.to_string()
        } else {
            // 심볼이 없으면 주문 조회하여 심볼 확인
            let order = self.get_order_linear_impl(order_id).await?;
            Self::to_bybit_symbol(&order.market)
        };

        debug!(order_id, symbol = %bybit_symbol, "Bybit linear 주문 취소 요청");

        let body = BybitCancelOrderRequest {
            category: "linear".to_string(),
            symbol: bybit_symbol,
            order_id: Some(order_id.to_string()),
            order_link_id: None,
        };

        let _result: BybitCancelOrderResult = self.post_private("/v5/order/cancel", &body).await?;

        debug!(order_id, "Bybit linear 주문 취소 완료");

        self.get_order_linear_impl(order_id).await
    }

    /// 선물 심볼의 레버리지를 설정합니다.
    ///
    /// buyLeverage와 sellLeverage를 동일하게 설정합니다.
    /// 이미 동일한 레버리지가 설정되어 있으면 에러를 무시합니다.
    ///
    /// # 인자
    ///
    /// * `symbol` - 심볼 (예: "BTCUSDT")
    /// * `leverage` - 설정할 레버리지 배수
    pub async fn set_leverage(&self, symbol: &str, leverage: u32) -> ExchangeResult<()> {
        let lev_str = leverage.to_string();
        debug!(symbol, leverage, "Bybit 레버리지 설정 요청");

        let body = BybitSetLeverageRequest {
            category: "linear".to_string(),
            symbol: symbol.to_string(),
            buy_leverage: lev_str.clone(),
            sell_leverage: lev_str,
        };

        match self
            .post_private::<serde_json::Value>("/v5/position/set-leverage", &body)
            .await
        {
            Ok(_) => {
                debug!(symbol, leverage, "Bybit 레버리지 설정 완료");
                Ok(())
            }
            Err(ExchangeError::ApiError(msg)) if msg.contains("leverage not modified") => {
                debug!(symbol, leverage, "레버리지 이미 동일 — 무시");
                Ok(())
            }
            Err(ExchangeError::UnknownError { code, message }) if code == "110043" => {
                // 110043: Set leverage not modified
                debug!(
                    symbol,
                    leverage, "레버리지 이미 동일 (110043) — 무시: {}", message
                );
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// 선물 심볼의 마진 모드를 전환합니다 (cross/isolated).
    ///
    /// 이미 동일한 모드가 설정되어 있으면 에러를 무시합니다.
    ///
    /// # 인자
    ///
    /// * `symbol` - 심볼 (예: "BTCUSDT")
    /// * `trade_mode` - 0=cross, 1=isolated
    /// * `leverage` - 전환 시 적용할 레버리지
    pub async fn switch_margin_mode(
        &self,
        symbol: &str,
        trade_mode: i32,
        leverage: u32,
    ) -> ExchangeResult<()> {
        let lev_str = leverage.to_string();
        let mode_name = if trade_mode == 0 { "cross" } else { "isolated" };
        debug!(
            symbol,
            mode = mode_name,
            leverage,
            "Bybit 마진 모드 전환 요청"
        );

        let body = BybitSwitchIsolatedRequest {
            category: "linear".to_string(),
            symbol: symbol.to_string(),
            trade_mode,
            buy_leverage: lev_str.clone(),
            sell_leverage: lev_str,
        };

        match self
            .post_private::<serde_json::Value>("/v5/position/switch-isolated", &body)
            .await
        {
            Ok(_) => {
                debug!(symbol, mode = mode_name, "Bybit 마진 모드 전환 완료");
                Ok(())
            }
            Err(ExchangeError::UnknownError { code, message }) if code == "110026" => {
                // 110026: Cross/isolated margin mode is not modified
                debug!(
                    symbol,
                    mode = mode_name,
                    "마진 모드 이미 동일 (110026) — 무시: {}",
                    message
                );
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// 선물 포지션 정보를 조회합니다.
    ///
    /// # 인자
    ///
    /// * `symbol` - 심볼 (예: "BTCUSDT")
    pub async fn get_positions(&self, symbol: &str) -> ExchangeResult<Vec<PositionInfo>> {
        let params = [("category", "linear"), ("symbol", symbol)];

        debug!(symbol, "Bybit 포지션 조회 요청");

        let result: BybitPositionList = self.get_private("/v5/position/list", &params).await?;

        let positions: Vec<PositionInfo> = result
            .list
            .into_iter()
            .map(|p| PositionInfo {
                symbol: p.symbol,
                side: p.side,
                size: p.size,
                entry_price: p.avg_price,
                leverage: p.leverage,
                unrealised_pnl: p.unrealised_pnl,
                liq_price: p.liq_price.unwrap_or(Decimal::ZERO),
            })
            .collect();

        debug!(symbol, count = positions.len(), "Bybit 포지션 조회 완료");
        Ok(positions)
    }

    /// 선물(linear) 티커 정보를 조회합니다 (펀딩레이트 포함).
    ///
    /// # 인자
    ///
    /// * `symbol` - 조회할 심볼 (None이면 전체 조회)
    pub async fn get_tickers_linear(
        &self,
        symbol: Option<&str>,
    ) -> ExchangeResult<Vec<LinearTickerInfo>> {
        let symbol_owned;
        let mut params = vec![("category", "linear")];
        if let Some(s) = symbol {
            symbol_owned = s.to_string();
            params.push(("symbol", &symbol_owned));
        }

        debug!(symbol = ?symbol, "Bybit linear 티커 조회 요청");

        let result: BybitLinearTickerList = self.get_public("/v5/market/tickers", &params).await?;

        let tickers: Vec<LinearTickerInfo> = result
            .list
            .into_iter()
            .map(|t| {
                let funding_rate = t
                    .funding_rate
                    .map(|d| d.to_string().parse::<f64>().unwrap_or(0.0))
                    .unwrap_or(0.0);
                let next_funding_time = t
                    .next_funding_time
                    .as_deref()
                    .unwrap_or("0")
                    .parse::<i64>()
                    .unwrap_or(0);

                LinearTickerInfo {
                    symbol: t.symbol,
                    funding_rate,
                    next_funding_time,
                }
            })
            .collect();

        debug!(count = tickers.len(), "Bybit linear 티커 조회 완료");
        Ok(tickers)
    }

    /// 긴급 주문용 POST 메서드 (emergency_limiter 사용).
    ///
    /// kill switch 등 긴급 청산 시에만 사용합니다.
    /// 향후 LiveExecutor에서 호출됩니다.
    #[allow(dead_code)]
    pub(crate) async fn post_emergency<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &impl serde::Serialize,
    ) -> ExchangeResult<T> {
        self.emergency_limiter.acquire().await;
        let creds = self.credentials()?;
        let url = format!("{}{}", self.base_url, endpoint);
        debug!(endpoint, "Bybit emergency POST 요청");

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
