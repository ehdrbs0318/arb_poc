//! 객체 안전(object-safe) 거래소 어댑터 trait.
//!
//! 이 모듈은 거래소 trait의 객체 안전 버전을 제공하여,
//! 동적 디스패치와 런타임 거래소 선택을 가능하게 합니다.

use crate::error::ExchangeResult;
use crate::types::{Balance, Candle, CandleInterval, Order, OrderBook, OrderRequest, Ticker};
use async_trait::async_trait;
use std::fmt::Debug;

/// 거래소 작업을 위한 객체 안전 어댑터 trait.
///
/// 이 trait은 `async_trait`을 사용하여 비동기 메서드의 객체 안전성을 확보하며,
/// 거래소를 `Box<dyn ExchangeAdapter>` 또는 `Arc<dyn ExchangeAdapter>`로 사용할 수 있게 합니다.
///
/// # 예제
///
/// ```ignore
/// use std::sync::Arc;
///
/// let exchange: Arc<dyn ExchangeAdapter> = Arc::new(UpbitAdapter::new(client));
/// let tickers = exchange.get_ticker(&["KRW-BTC"]).await?;
/// ```
#[async_trait]
pub trait ExchangeAdapter: Send + Sync + Debug {
    /// 거래소 이름을 반환합니다 (예: "Upbit", "Bithumb", "Bybit").
    fn name(&self) -> &str;

    /// 거래소 클라이언트가 인증되었는지 여부를 반환합니다.
    fn is_authenticated(&self) -> bool;

    /// 거래소의 기본 호가 통화를 반환합니다 (예: 한국 거래소는 "KRW", Bybit은 "USDT").
    fn native_quote_currency(&self) -> &str;

    // ==================== 시장 데이터 작업 ====================

    /// 하나 이상의 마켓에 대한 현재 티커를 조회합니다.
    ///
    /// # 인자
    ///
    /// * `markets` - 마켓 코드 슬라이스 (예: ["KRW-BTC", "KRW-ETH"])
    ///
    /// # 참고
    ///
    /// 마켓 코드는 내부 형식 `{QUOTE}-{BASE}`(예: "KRW-BTC")를 사용해야 합니다.
    /// 어댑터가 거래소의 고유 형식으로 변환을 처리합니다.
    async fn get_ticker(&self, markets: &[&str]) -> ExchangeResult<Vec<Ticker>>;

    /// 마켓의 호가창을 조회합니다.
    ///
    /// # 인자
    ///
    /// * `market` - 마켓 코드 (예: "KRW-BTC")
    /// * `depth` - 조회할 호가 레벨 수 (선택사항, 기본값은 거래소마다 다름)
    async fn get_orderbook(&self, market: &str, depth: Option<u32>) -> ExchangeResult<OrderBook>;

    /// 마켓의 캔들 데이터를 조회합니다.
    ///
    /// # 인자
    ///
    /// * `market` - 마켓 코드 (예: "KRW-BTC")
    /// * `interval` - 캔들 간격
    /// * `count` - 조회할 캔들 수
    async fn get_candles(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
    ) -> ExchangeResult<Vec<Candle>>;

    // ==================== 주문 관리 작업 ====================

    /// 새 주문을 생성합니다.
    ///
    /// # 인자
    ///
    /// * `request` - 주문 요청 파라미터
    async fn place_order(&self, request: &OrderRequest) -> ExchangeResult<Order>;

    /// 기존 주문을 취소합니다.
    ///
    /// # 인자
    ///
    /// * `order_id` - 거래소가 할당한 주문 ID
    async fn cancel_order(&self, order_id: &str) -> ExchangeResult<Order>;

    /// ID로 주문을 조회합니다.
    ///
    /// # 인자
    ///
    /// * `order_id` - 거래소가 할당한 주문 ID
    async fn get_order(&self, order_id: &str) -> ExchangeResult<Order>;

    /// 마켓의 미체결 주문을 조회합니다.
    ///
    /// # 인자
    ///
    /// * `market` - 마켓 코드 (선택사항, None이면 모든 마켓)
    async fn get_open_orders(&self, market: Option<&str>) -> ExchangeResult<Vec<Order>>;

    /// 계정 잔고를 조회합니다.
    async fn get_balances(&self) -> ExchangeResult<Vec<Balance>>;

    /// 특정 통화의 잔고를 조회합니다.
    ///
    /// # 인자
    ///
    /// * `currency` - 통화 코드 (예: "BTC", "KRW")
    async fn get_balance(&self, currency: &str) -> ExchangeResult<Balance>;
}

/// MarketData + OrderManagement를 구현하는 기존 클라이언트를 ExchangeAdapter로 적응시키는 래퍼.
///
/// 이 macro는 주어진 클라이언트 타입에 대해 ExchangeAdapter를 구현하는 어댑터 구조체를 생성합니다.
#[macro_export]
macro_rules! impl_exchange_adapter {
    ($adapter_name:ident, $client_type:ty, $exchange_name:expr, $quote_currency:expr) => {
        /// 거래소 클라이언트를 위한 어댑터 래퍼.
        #[derive(Debug)]
        pub struct $adapter_name {
            client: $client_type,
        }

        impl $adapter_name {
            /// 클라이언트로부터 새 어댑터를 생성합니다.
            pub fn new(client: $client_type) -> Self {
                Self { client }
            }

            /// 기본 클라이언트에 대한 참조를 반환합니다.
            pub fn client(&self) -> &$client_type {
                &self.client
            }
        }

        #[async_trait::async_trait]
        impl $crate::adapter::ExchangeAdapter for $adapter_name {
            fn name(&self) -> &str {
                use $crate::traits::MarketData;
                self.client.name()
            }

            fn is_authenticated(&self) -> bool {
                self.client.credentials.is_some()
            }

            fn native_quote_currency(&self) -> &str {
                $quote_currency
            }

            async fn get_ticker(
                &self,
                markets: &[&str],
            ) -> $crate::ExchangeResult<Vec<$crate::Ticker>> {
                use $crate::traits::MarketData;
                self.client.get_ticker(markets).await
            }

            async fn get_orderbook(
                &self,
                market: &str,
                depth: Option<u32>,
            ) -> $crate::ExchangeResult<$crate::OrderBook> {
                use $crate::traits::MarketData;
                self.client.get_orderbook(market, depth).await
            }

            async fn get_candles(
                &self,
                market: &str,
                interval: $crate::CandleInterval,
                count: u32,
            ) -> $crate::ExchangeResult<Vec<$crate::Candle>> {
                use $crate::traits::MarketData;
                self.client.get_candles(market, interval, count).await
            }

            async fn place_order(
                &self,
                request: &$crate::OrderRequest,
            ) -> $crate::ExchangeResult<$crate::Order> {
                use $crate::traits::OrderManagement;
                self.client.place_order(request).await
            }

            async fn cancel_order(
                &self,
                order_id: &str,
            ) -> $crate::ExchangeResult<$crate::Order> {
                use $crate::traits::OrderManagement;
                self.client.cancel_order(order_id).await
            }

            async fn get_order(
                &self,
                order_id: &str,
            ) -> $crate::ExchangeResult<$crate::Order> {
                use $crate::traits::OrderManagement;
                self.client.get_order(order_id).await
            }

            async fn get_open_orders(
                &self,
                market: Option<&str>,
            ) -> $crate::ExchangeResult<Vec<$crate::Order>> {
                use $crate::traits::OrderManagement;
                self.client.get_open_orders(market).await
            }

            async fn get_balances(
                &self,
            ) -> $crate::ExchangeResult<Vec<$crate::Balance>> {
                use $crate::traits::OrderManagement;
                self.client.get_balances().await
            }

            async fn get_balance(
                &self,
                currency: &str,
            ) -> $crate::ExchangeResult<$crate::Balance> {
                use $crate::traits::OrderManagement;
                self.client.get_balance(currency).await
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // ExchangeAdapter가 객체 안전한지 확인하는 컴파일 타임 테스트
    fn _assert_object_safe(_: &dyn ExchangeAdapter) {}

    // ExchangeAdapter가 Arc와 함께 사용 가능한지 확인하는 컴파일 타임 테스트
    fn _assert_arc_compatible(_: std::sync::Arc<dyn ExchangeAdapter>) {}
}
