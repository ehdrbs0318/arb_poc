//! 여러 거래소를 조율하기 위한 거래소 관리자.
//!
//! 이 모듈은 여러 거래소 인스턴스를 관리하고 통합된 접근을 제공하는
//! `ExchangeManager` 구조체를 제공합니다.

use crate::adapter::ExchangeAdapter;
use crate::error::{ExchangeError, ExchangeResult};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// 여러 거래소 인스턴스를 위한 관리자.
///
/// `ExchangeManager`는 런타임에 여러 거래소를 관리하고 접근하는 중앙화된 방법을
/// 제공합니다. 거래소 어댑터의 동적 등록과 조회를 지원합니다.
///
/// # 예제
///
/// ```ignore
/// use arb_exchange::ExchangeManager;
///
/// let mut manager = ExchangeManager::new();
///
/// // 거래소 등록
/// manager.register("upbit", upbit_adapter);
/// manager.register("bithumb", bithumb_adapter);
///
/// // 이름으로 거래소 접근
/// let upbit = manager.get("upbit").unwrap();
/// let ticker = upbit.get_ticker(&["KRW-BTC"]).await?;
/// ```
#[derive(Default)]
pub struct ExchangeManager {
    exchanges: HashMap<String, Arc<dyn ExchangeAdapter>>,
}

impl ExchangeManager {
    /// 비어있는 새 거래소 관리자를 생성합니다.
    #[must_use]
    pub fn new() -> Self {
        Self {
            exchanges: HashMap::new(),
        }
    }

    /// 지정된 용량을 가진 새 거래소 관리자를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `capacity` - 거래소 레지스트리의 초기 용량
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            exchanges: HashMap::with_capacity(capacity),
        }
    }

    /// 주어진 이름으로 거래소를 등록합니다.
    ///
    /// 같은 이름의 거래소가 이미 존재하면 대체됩니다.
    ///
    /// # 인자
    ///
    /// * `name` - 거래소를 등록할 이름 (대소문자 구분 없음)
    /// * `exchange` - 등록할 거래소 어댑터
    ///
    /// # 반환값
    ///
    /// 같은 이름으로 이전에 등록된 거래소가 있었다면 반환합니다.
    pub fn register(
        &mut self,
        name: impl Into<String>,
        exchange: impl ExchangeAdapter + 'static,
    ) -> Option<Arc<dyn ExchangeAdapter>> {
        let name = name.into().to_lowercase();
        let prev = self.exchanges.insert(name.clone(), Arc::new(exchange));
        if prev.is_some() {
            warn!(name = %name, "기존 거래소 교체 등록");
        } else {
            info!(name = %name, "거래소 등록 완료");
        }
        prev
    }

    /// Arc로 래핑된 거래소를 등록합니다.
    ///
    /// # 인자
    ///
    /// * `name` - 거래소를 등록할 이름 (대소문자 구분 없음)
    /// * `exchange` - Arc로 래핑된 거래소 어댑터
    pub fn register_arc(
        &mut self,
        name: impl Into<String>,
        exchange: Arc<dyn ExchangeAdapter>,
    ) -> Option<Arc<dyn ExchangeAdapter>> {
        let name = name.into().to_lowercase();
        let prev = self.exchanges.insert(name.clone(), exchange);
        if prev.is_some() {
            warn!(name = %name, "기존 거래소 교체 등록 (arc)");
        } else {
            info!(name = %name, "거래소 등록 완료 (arc)");
        }
        prev
    }

    /// 관리자에서 거래소를 제거합니다.
    ///
    /// # 인자
    ///
    /// * `name` - 제거할 거래소 이름 (대소문자 구분 없음)
    ///
    /// # 반환값
    ///
    /// 제거된 거래소가 존재했다면 반환합니다.
    pub fn unregister(&mut self, name: &str) -> Option<Arc<dyn ExchangeAdapter>> {
        let removed = self.exchanges.remove(&name.to_lowercase());
        if removed.is_some() {
            info!(name = name, "거래소 등록 해제");
        } else {
            debug!(name = name, "거래소 등록 해제 시도: 미등록 상태");
        }
        removed
    }

    /// 이름으로 거래소 참조를 가져옵니다.
    ///
    /// # 인자
    ///
    /// * `name` - 거래소 이름 (대소문자 구분 없음)
    ///
    /// # 반환값
    ///
    /// 찾은 경우 거래소 어댑터 참조를 반환합니다.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<Arc<dyn ExchangeAdapter>> {
        self.exchanges.get(&name.to_lowercase()).cloned()
    }

    /// 이름으로 거래소를 가져오고, 없으면 에러를 반환합니다.
    ///
    /// # 인자
    ///
    /// * `name` - 거래소 이름 (대소문자 구분 없음)
    ///
    /// # 에러
    ///
    /// 거래소가 등록되지 않은 경우 에러를 반환합니다.
    pub fn get_or_error(&self, name: &str) -> ExchangeResult<Arc<dyn ExchangeAdapter>> {
        self.get(name).ok_or_else(|| {
            warn!(name = name, "미등록 거래소 접근 시도");
            ExchangeError::ConfigError(format!("Exchange not registered: {}", name))
        })
    }

    /// 주어진 이름의 거래소가 등록되어 있으면 true를 반환합니다.
    ///
    /// # 인자
    ///
    /// * `name` - 거래소 이름 (대소문자 구분 없음)
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.exchanges.contains_key(&name.to_lowercase())
    }

    /// 등록된 모든 거래소의 이름을 반환합니다.
    #[must_use]
    pub fn list_exchanges(&self) -> Vec<&str> {
        self.exchanges.keys().map(String::as_str).collect()
    }

    /// 등록된 거래소 개수를 반환합니다.
    #[must_use]
    pub fn len(&self) -> usize {
        self.exchanges.len()
    }

    /// 등록된 거래소가 없으면 true를 반환합니다.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.exchanges.is_empty()
    }

    /// 등록된 모든 거래소에 대한 이터레이터를 반환합니다.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Arc<dyn ExchangeAdapter>)> {
        self.exchanges
            .iter()
            .map(|(name, exchange)| (name.as_str(), exchange))
    }

    /// 인증된 거래소만 반환합니다.
    pub fn authenticated(&self) -> impl Iterator<Item = (&str, &Arc<dyn ExchangeAdapter>)> {
        self.iter().filter(|(_, ex)| ex.is_authenticated())
    }

    /// 기준 통화로 필터링된 거래소를 반환합니다.
    ///
    /// # 인자
    ///
    /// * `quote` - 필터링할 기준 통화 (예: "KRW", "USDT")
    pub fn by_quote_currency(
        &self,
        quote: &str,
    ) -> impl Iterator<Item = (&str, &Arc<dyn ExchangeAdapter>)> {
        let quote = quote.to_uppercase();
        self.iter()
            .filter(move |(_, ex)| ex.native_quote_currency().to_uppercase() == quote)
    }
}

impl std::fmt::Debug for ExchangeManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExchangeManager")
            .field("exchanges", &self.list_exchanges())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    // 테스트용 모의 어댑터
    #[derive(Debug)]
    struct MockAdapter {
        name: String,
        authenticated: bool,
        quote_currency: String,
    }

    impl MockAdapter {
        fn new(name: &str, authenticated: bool, quote_currency: &str) -> Self {
            Self {
                name: name.to_string(),
                authenticated,
                quote_currency: quote_currency.to_string(),
            }
        }
    }

    #[async_trait::async_trait]
    impl ExchangeAdapter for MockAdapter {
        fn name(&self) -> &str {
            &self.name
        }

        fn is_authenticated(&self) -> bool {
            self.authenticated
        }

        fn native_quote_currency(&self) -> &str {
            &self.quote_currency
        }

        async fn get_ticker(&self, _markets: &[&str]) -> ExchangeResult<Vec<Ticker>> {
            Ok(vec![])
        }

        async fn get_orderbook(
            &self,
            _market: &str,
            _depth: Option<u32>,
        ) -> ExchangeResult<OrderBook> {
            Err(ExchangeError::InternalError("Not implemented".to_string()))
        }

        async fn get_candles(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
        ) -> ExchangeResult<Vec<Candle>> {
            Ok(vec![])
        }

        async fn get_candles_before(
            &self,
            _market: &str,
            _interval: CandleInterval,
            _count: u32,
            _before: chrono::DateTime<chrono::Utc>,
        ) -> ExchangeResult<Vec<Candle>> {
            Ok(vec![])
        }

        async fn place_order(&self, _request: &OrderRequest) -> ExchangeResult<Order> {
            Err(ExchangeError::AuthError("Not authenticated".to_string()))
        }

        async fn cancel_order(&self, _order_id: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::AuthError("Not authenticated".to_string()))
        }

        async fn get_order(&self, _order_id: &str) -> ExchangeResult<Order> {
            Err(ExchangeError::AuthError("Not authenticated".to_string()))
        }

        async fn get_open_orders(&self, _market: Option<&str>) -> ExchangeResult<Vec<Order>> {
            Ok(vec![])
        }

        async fn get_balances(&self) -> ExchangeResult<Vec<Balance>> {
            Ok(vec![])
        }

        async fn get_balance(&self, _currency: &str) -> ExchangeResult<Balance> {
            Err(ExchangeError::InvalidParameter("Not found".to_string()))
        }
    }

    #[test]
    fn test_manager_new() {
        let manager = ExchangeManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_manager_register_and_get() {
        let mut manager = ExchangeManager::new();

        manager.register("Upbit", MockAdapter::new("Upbit", false, "KRW"));
        manager.register("bithumb", MockAdapter::new("Bithumb", true, "KRW"));

        assert_eq!(manager.len(), 2);
        assert!(manager.contains("upbit"));
        assert!(manager.contains("UPBIT"));
        assert!(manager.contains("Bithumb"));

        let upbit = manager.get("upbit").unwrap();
        assert_eq!(upbit.name(), "Upbit");

        let bithumb = manager.get("BITHUMB").unwrap();
        assert_eq!(bithumb.name(), "Bithumb");
    }

    #[test]
    fn test_manager_unregister() {
        let mut manager = ExchangeManager::new();

        manager.register("upbit", MockAdapter::new("Upbit", false, "KRW"));
        assert!(manager.contains("upbit"));

        let removed = manager.unregister("upbit");
        assert!(removed.is_some());
        assert!(!manager.contains("upbit"));
    }

    #[test]
    fn test_manager_list_exchanges() {
        let mut manager = ExchangeManager::new();

        manager.register("upbit", MockAdapter::new("Upbit", false, "KRW"));
        manager.register("bithumb", MockAdapter::new("Bithumb", false, "KRW"));
        manager.register("bybit", MockAdapter::new("Bybit", false, "USDT"));

        let names = manager.list_exchanges();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"upbit"));
        assert!(names.contains(&"bithumb"));
        assert!(names.contains(&"bybit"));
    }

    #[test]
    fn test_manager_authenticated() {
        let mut manager = ExchangeManager::new();

        manager.register("upbit", MockAdapter::new("Upbit", false, "KRW"));
        manager.register("bithumb", MockAdapter::new("Bithumb", true, "KRW"));
        manager.register("bybit", MockAdapter::new("Bybit", true, "USDT"));

        let authenticated: Vec<_> = manager.authenticated().collect();
        assert_eq!(authenticated.len(), 2);
    }

    #[test]
    fn test_manager_by_quote_currency() {
        let mut manager = ExchangeManager::new();

        manager.register("upbit", MockAdapter::new("Upbit", false, "KRW"));
        manager.register("bithumb", MockAdapter::new("Bithumb", false, "KRW"));
        manager.register("bybit", MockAdapter::new("Bybit", false, "USDT"));

        let krw_exchanges: Vec<_> = manager.by_quote_currency("KRW").collect();
        assert_eq!(krw_exchanges.len(), 2);

        let usdt_exchanges: Vec<_> = manager.by_quote_currency("usdt").collect();
        assert_eq!(usdt_exchanges.len(), 1);
    }

    #[test]
    fn test_manager_get_or_error() {
        let mut manager = ExchangeManager::new();
        manager.register("upbit", MockAdapter::new("Upbit", false, "KRW"));

        assert!(manager.get_or_error("upbit").is_ok());
        assert!(manager.get_or_error("nonexistent").is_err());
    }
}
