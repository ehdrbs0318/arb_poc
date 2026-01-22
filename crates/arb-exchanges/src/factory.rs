//! 설정에서 거래소 인스턴스를 생성하기 위한 거래소 팩토리.
//!
//! 이 모듈은 설정으로부터 거래소 어댑터를 생성하는 팩토리 함수를 제공하여
//! 동적 거래소 인스턴스화를 가능하게 합니다.

use arb_config::{Config, ExchangeConfig};
use arb_exchange::{
    Balance, Candle, CandleInterval, ExchangeAdapter, ExchangeError, ExchangeManager,
    ExchangeResult, MarketData, Order, OrderBook, OrderManagement, OrderRequest, Ticker,
};
use std::sync::Arc;

use crate::{BithumbClient, BybitClient, UpbitClient};

// ==================== 거래소 어댑터 ====================

/// Upbit 거래소 어댑터.
#[derive(Debug)]
pub struct UpbitAdapter {
    client: UpbitClient,
}

impl UpbitAdapter {
    /// 클라이언트로부터 새 Upbit 어댑터를 생성합니다.
    pub fn new(client: UpbitClient) -> Self {
        Self { client }
    }

    /// 인증되지 않은 새 Upbit 어댑터를 생성합니다.
    pub fn public() -> ExchangeResult<Self> {
        Ok(Self {
            client: UpbitClient::new()?,
        })
    }

    /// 인증된 새 Upbit 어댑터를 생성합니다.
    pub fn authenticated(api_key: &str, secret_key: &str) -> ExchangeResult<Self> {
        Ok(Self {
            client: UpbitClient::with_credentials(api_key, secret_key)?,
        })
    }

    /// 거래소 설정으로부터 생성합니다.
    pub fn from_config(config: &ExchangeConfig) -> ExchangeResult<Self> {
        if config.has_credentials() {
            Self::authenticated(&config.api_key, &config.secret_key)
        } else {
            Self::public()
        }
    }
}

#[async_trait::async_trait]
impl ExchangeAdapter for UpbitAdapter {
    fn name(&self) -> &str {
        MarketData::name(&self.client)
    }

    fn is_authenticated(&self) -> bool {
        self.client.credentials.is_some()
    }

    fn native_quote_currency(&self) -> &str {
        "KRW"
    }

    async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
        MarketData::get_ticker(&self.client, markets).await
    }

    async fn get_orderbook(&self, market: &str, depth: Option<u32>) -> ExchangeResult<OrderBook> {
        MarketData::get_orderbook(&self.client, market, depth).await
    }

    async fn get_candles(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
    ) -> ExchangeResult<Vec<Candle>> {
        MarketData::get_candles(&self.client, market, interval, count).await
    }

    async fn place_order(&self, request: &OrderRequest) -> ExchangeResult<Order> {
        OrderManagement::place_order(&self.client, request).await
    }

    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<Order> {
        OrderManagement::cancel_order(&self.client, order_id).await
    }

    async fn get_order(&self, order_id: &str) -> ExchangeResult<Order> {
        OrderManagement::get_order(&self.client, order_id).await
    }

    async fn get_open_orders(&self, market: Option<&str>) -> ExchangeResult<Vec<Order>> {
        OrderManagement::get_open_orders(&self.client, market).await
    }

    async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
        OrderManagement::get_balances(&self.client).await
    }

    async fn get_balance(&self, currency: &str) -> ExchangeResult<Balance> {
        OrderManagement::get_balance(&self.client, currency).await
    }
}

/// Bithumb 거래소 어댑터.
#[derive(Debug)]
pub struct BithumbAdapter {
    client: BithumbClient,
}

impl BithumbAdapter {
    /// 클라이언트로부터 새 Bithumb 어댑터를 생성합니다.
    pub fn new(client: BithumbClient) -> Self {
        Self { client }
    }

    /// 인증되지 않은 새 Bithumb 어댑터를 생성합니다.
    pub fn public() -> ExchangeResult<Self> {
        Ok(Self {
            client: BithumbClient::new()?,
        })
    }

    /// 인증된 새 Bithumb 어댑터를 생성합니다.
    pub fn authenticated(api_key: &str, secret_key: &str) -> ExchangeResult<Self> {
        Ok(Self {
            client: BithumbClient::with_credentials(api_key, secret_key)?,
        })
    }

    /// 거래소 설정으로부터 생성합니다.
    pub fn from_config(config: &ExchangeConfig) -> ExchangeResult<Self> {
        if config.has_credentials() {
            Self::authenticated(&config.api_key, &config.secret_key)
        } else {
            Self::public()
        }
    }
}

#[async_trait::async_trait]
impl ExchangeAdapter for BithumbAdapter {
    fn name(&self) -> &str {
        MarketData::name(&self.client)
    }

    fn is_authenticated(&self) -> bool {
        self.client.credentials.is_some()
    }

    fn native_quote_currency(&self) -> &str {
        "KRW"
    }

    async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
        MarketData::get_ticker(&self.client, markets).await
    }

    async fn get_orderbook(&self, market: &str, depth: Option<u32>) -> ExchangeResult<OrderBook> {
        MarketData::get_orderbook(&self.client, market, depth).await
    }

    async fn get_candles(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
    ) -> ExchangeResult<Vec<Candle>> {
        MarketData::get_candles(&self.client, market, interval, count).await
    }

    async fn place_order(&self, request: &OrderRequest) -> ExchangeResult<Order> {
        OrderManagement::place_order(&self.client, request).await
    }

    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<Order> {
        OrderManagement::cancel_order(&self.client, order_id).await
    }

    async fn get_order(&self, order_id: &str) -> ExchangeResult<Order> {
        OrderManagement::get_order(&self.client, order_id).await
    }

    async fn get_open_orders(&self, market: Option<&str>) -> ExchangeResult<Vec<Order>> {
        OrderManagement::get_open_orders(&self.client, market).await
    }

    async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
        OrderManagement::get_balances(&self.client).await
    }

    async fn get_balance(&self, currency: &str) -> ExchangeResult<Balance> {
        OrderManagement::get_balance(&self.client, currency).await
    }
}

/// Bybit 거래소 어댑터.
#[derive(Debug)]
pub struct BybitAdapter {
    client: BybitClient,
}

impl BybitAdapter {
    /// 클라이언트로부터 새 Bybit 어댑터를 생성합니다.
    pub fn new(client: BybitClient) -> Self {
        Self { client }
    }

    /// 인증되지 않은 새 Bybit 어댑터를 생성합니다 (메인넷).
    pub fn public() -> ExchangeResult<Self> {
        Ok(Self {
            client: BybitClient::new()?,
        })
    }

    /// 인증되지 않은 새 Bybit 어댑터를 생성합니다 (테스트넷).
    pub fn public_testnet() -> ExchangeResult<Self> {
        Ok(Self {
            client: BybitClient::new_testnet()?,
        })
    }

    /// 인증된 새 Bybit 어댑터를 생성합니다 (메인넷).
    pub fn authenticated(api_key: &str, secret_key: &str) -> ExchangeResult<Self> {
        Ok(Self {
            client: BybitClient::with_credentials(api_key, secret_key)?,
        })
    }

    /// 인증된 새 Bybit 어댑터를 생성합니다 (테스트넷).
    pub fn authenticated_testnet(api_key: &str, secret_key: &str) -> ExchangeResult<Self> {
        Ok(Self {
            client: BybitClient::with_credentials_testnet(api_key, secret_key)?,
        })
    }

    /// 거래소 설정으로부터 생성합니다.
    pub fn from_config(config: &ExchangeConfig) -> ExchangeResult<Self> {
        if config.has_credentials() {
            Self::authenticated(&config.api_key, &config.secret_key)
        } else {
            Self::public()
        }
    }

    /// 거래소 설정으로부터 생성합니다 (테스트넷).
    pub fn from_config_testnet(config: &ExchangeConfig) -> ExchangeResult<Self> {
        if config.has_credentials() {
            Self::authenticated_testnet(&config.api_key, &config.secret_key)
        } else {
            Self::public_testnet()
        }
    }
}

#[async_trait::async_trait]
impl ExchangeAdapter for BybitAdapter {
    fn name(&self) -> &str {
        MarketData::name(&self.client)
    }

    fn is_authenticated(&self) -> bool {
        self.client.credentials.is_some()
    }

    fn native_quote_currency(&self) -> &str {
        "USDT"
    }

    async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
        MarketData::get_ticker(&self.client, markets).await
    }

    async fn get_orderbook(&self, market: &str, depth: Option<u32>) -> ExchangeResult<OrderBook> {
        MarketData::get_orderbook(&self.client, market, depth).await
    }

    async fn get_candles(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
    ) -> ExchangeResult<Vec<Candle>> {
        MarketData::get_candles(&self.client, market, interval, count).await
    }

    async fn place_order(&self, request: &OrderRequest) -> ExchangeResult<Order> {
        OrderManagement::place_order(&self.client, request).await
    }

    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<Order> {
        OrderManagement::cancel_order(&self.client, order_id).await
    }

    async fn get_order(&self, order_id: &str) -> ExchangeResult<Order> {
        OrderManagement::get_order(&self.client, order_id).await
    }

    async fn get_open_orders(&self, market: Option<&str>) -> ExchangeResult<Vec<Order>> {
        OrderManagement::get_open_orders(&self.client, market).await
    }

    async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
        OrderManagement::get_balances(&self.client).await
    }

    async fn get_balance(&self, currency: &str) -> ExchangeResult<Balance> {
        OrderManagement::get_balance(&self.client, currency).await
    }
}

// ==================== 팩토리 함수 ====================

/// 지원되는 거래소 이름.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExchangeName {
    /// Upbit (한국 거래소).
    Upbit,
    /// Bithumb (한국 거래소).
    Bithumb,
    /// Bybit (글로벌 거래소).
    Bybit,
}

impl ExchangeName {
    /// 거래소 이름의 문자열 표현을 반환합니다.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Upbit => "upbit",
            Self::Bithumb => "bithumb",
            Self::Bybit => "bybit",
        }
    }

    /// 지원되는 모든 거래소 이름을 반환합니다.
    pub fn all() -> &'static [Self] {
        &[Self::Upbit, Self::Bithumb, Self::Bybit]
    }

    /// 문자열에서 거래소 이름을 파싱합니다 (편의 메서드).
    ///
    /// `FromStr::from_str`의 편의 래퍼로 `Option<Self>`를 반환합니다.
    pub fn parse(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

impl std::str::FromStr for ExchangeName {
    type Err = ExchangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "upbit" => Ok(Self::Upbit),
            "bithumb" => Ok(Self::Bithumb),
            "bybit" => Ok(Self::Bybit),
            _ => Err(ExchangeError::ConfigError(format!(
                "Unknown exchange: {}. Supported exchanges: {:?}",
                s,
                Self::all()
            ))),
        }
    }
}

impl std::fmt::Display for ExchangeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 이름으로 거래소 어댑터를 생성합니다.
///
/// # 인자
///
/// * `name` - 거래소 이름 (대소문자 구분 없음)
/// * `config` - 거래소 설정 (공개 전용 접근의 경우 선택적)
///
/// # 에러
///
/// 거래소 이름이 지원되지 않거나 클라이언트 생성이 실패하면 에러를 반환합니다.
///
/// # 예제
///
/// ```ignore
/// let upbit = create_exchange("upbit", None)?;
/// let bithumb = create_exchange("bithumb", Some(&config.bithumb))?;
/// ```
pub fn create_exchange(
    name: &str,
    config: Option<&ExchangeConfig>,
) -> ExchangeResult<Arc<dyn ExchangeAdapter>> {
    let exchange_name: ExchangeName = name.parse()?;

    let adapter: Arc<dyn ExchangeAdapter> = match exchange_name {
        ExchangeName::Upbit => {
            if let Some(cfg) = config {
                Arc::new(UpbitAdapter::from_config(cfg)?)
            } else {
                Arc::new(UpbitAdapter::public()?)
            }
        }
        ExchangeName::Bithumb => {
            if let Some(cfg) = config {
                Arc::new(BithumbAdapter::from_config(cfg)?)
            } else {
                Arc::new(BithumbAdapter::public()?)
            }
        }
        ExchangeName::Bybit => {
            if let Some(cfg) = config {
                Arc::new(BybitAdapter::from_config(cfg)?)
            } else {
                Arc::new(BybitAdapter::public()?)
            }
        }
    };

    Ok(adapter)
}

/// 박스 타입을 사용하여 거래소 어댑터를 생성합니다.
///
/// `create_exchange`와 동일하지만 `Box<dyn ExchangeAdapter>`를 반환합니다.
pub fn create_exchange_boxed(
    name: &str,
    config: Option<&ExchangeConfig>,
) -> ExchangeResult<Box<dyn ExchangeAdapter>> {
    let exchange_name: ExchangeName = name.parse()?;

    let adapter: Box<dyn ExchangeAdapter> = match exchange_name {
        ExchangeName::Upbit => {
            if let Some(cfg) = config {
                Box::new(UpbitAdapter::from_config(cfg)?)
            } else {
                Box::new(UpbitAdapter::public()?)
            }
        }
        ExchangeName::Bithumb => {
            if let Some(cfg) = config {
                Box::new(BithumbAdapter::from_config(cfg)?)
            } else {
                Box::new(BithumbAdapter::public()?)
            }
        }
        ExchangeName::Bybit => {
            if let Some(cfg) = config {
                Box::new(BybitAdapter::from_config(cfg)?)
            } else {
                Box::new(BybitAdapter::public()?)
            }
        }
    };

    Ok(adapter)
}

/// 설정에서 간편하게 등록하기 위한 ExchangeManager 확장 trait.
pub trait ExchangeManagerExt {
    /// 설정에서 거래소를 등록합니다.
    ///
    /// # 인자
    ///
    /// * `name` - 거래소 이름
    /// * `config` - 거래소 설정
    fn register_from_config(
        &mut self,
        name: &str,
        config: Option<&ExchangeConfig>,
    ) -> ExchangeResult<()>;

    /// 애플리케이션 설정에서 지원되는 모든 거래소를 등록합니다.
    ///
    /// # 인자
    ///
    /// * `config` - 애플리케이션 설정
    fn register_all_from_config(&mut self, config: &Config) -> ExchangeResult<()>;
}

impl ExchangeManagerExt for ExchangeManager {
    fn register_from_config(
        &mut self,
        name: &str,
        config: Option<&ExchangeConfig>,
    ) -> ExchangeResult<()> {
        let adapter = create_exchange(name, config)?;
        self.register_arc(name, adapter);
        Ok(())
    }

    fn register_all_from_config(&mut self, config: &Config) -> ExchangeResult<()> {
        // Upbit 등록
        self.register_from_config("upbit", Some(&config.upbit))?;

        // Bithumb 등록
        self.register_from_config("bithumb", Some(&config.bithumb))?;

        // Bybit 등록
        self.register_from_config("bybit", Some(&config.bybit))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_name_from_str() {
        assert_eq!(
            "upbit".parse::<ExchangeName>().ok(),
            Some(ExchangeName::Upbit)
        );
        assert_eq!(
            "UPBIT".parse::<ExchangeName>().ok(),
            Some(ExchangeName::Upbit)
        );
        assert_eq!(
            "bithumb".parse::<ExchangeName>().ok(),
            Some(ExchangeName::Bithumb)
        );
        assert_eq!(
            "bybit".parse::<ExchangeName>().ok(),
            Some(ExchangeName::Bybit)
        );
        assert!("unknown".parse::<ExchangeName>().is_err());
    }

    #[test]
    fn test_exchange_name_as_str() {
        assert_eq!(ExchangeName::Upbit.as_str(), "upbit");
        assert_eq!(ExchangeName::Bithumb.as_str(), "bithumb");
        assert_eq!(ExchangeName::Bybit.as_str(), "bybit");
    }

    #[test]
    fn test_create_exchange_unknown() {
        let result = create_exchange("unknown", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_exchange_upbit() {
        let result = create_exchange("upbit", None);
        assert!(result.is_ok());
        let adapter = result.unwrap();
        assert_eq!(adapter.name(), "Upbit");
        assert!(!adapter.is_authenticated());
    }

    #[test]
    fn test_create_exchange_bithumb() {
        let result = create_exchange("bithumb", None);
        assert!(result.is_ok());
        let adapter = result.unwrap();
        assert_eq!(adapter.name(), "Bithumb");
    }

    #[test]
    fn test_create_exchange_bybit() {
        let result = create_exchange("bybit", None);
        assert!(result.is_ok());
        let adapter = result.unwrap();
        assert_eq!(adapter.name(), "Bybit");
    }

    #[test]
    fn test_create_exchange_with_credentials() {
        let config = ExchangeConfig {
            api_key: "test_key".to_string(),
            secret_key: "test_secret".to_string(),
        };

        let result = create_exchange("upbit", Some(&config));
        assert!(result.is_ok());
        let adapter = result.unwrap();
        assert!(adapter.is_authenticated());
    }
}
