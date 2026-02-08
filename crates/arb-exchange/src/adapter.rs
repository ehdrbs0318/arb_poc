//! 객체 안전(object-safe) 거래소 어댑터 trait.
//!
//! 이 모듈은 거래소 trait의 객체 안전 버전을 제공하여,
//! 동적 디스패치와 런타임 거래소 선택을 가능하게 합니다.

use crate::error::ExchangeResult;
use crate::types::{Balance, Candle, CandleInterval, Order, OrderBook, OrderRequest, Ticker};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
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
    /// 반환값은 timestamp 오름차순 정렬을 보장합니다.
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

    /// 특정 시점 이전의 캔들 데이터를 조회합니다 (페이지네이션용).
    ///
    /// `before`는 exclusive (해당 timestamp 미포함).
    /// 반환값은 timestamp 오름차순 정렬을 보장합니다.
    ///
    /// # 인자
    ///
    /// * `market` - 마켓 코드 (예: "KRW-BTC")
    /// * `interval` - 캔들 간격
    /// * `count` - 조회할 캔들 수
    /// * `before` - 이 시점 이전의 캔들만 조회 (exclusive)
    async fn get_candles_before(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
        before: DateTime<Utc>,
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

#[cfg(test)]
mod tests {
    use super::*;

    // ExchangeAdapter가 객체 안전한지 확인하는 컴파일 타임 테스트
    fn _assert_object_safe(_: &dyn ExchangeAdapter) {}

    // ExchangeAdapter가 Arc와 함께 사용 가능한지 확인하는 컴파일 타임 테스트
    fn _assert_arc_compatible(_: std::sync::Arc<dyn ExchangeAdapter>) {}
}
