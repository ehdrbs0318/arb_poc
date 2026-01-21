//! 거래소 trait 정의.
//!
//! 이 모듈은 모든 거래소 구현체가 구현해야 하는 trait을 정의합니다.

use crate::exchange::error::ExchangeResult;
use crate::exchange::types::{
    Balance, Candle, CandleInterval, Order, OrderBook, OrderRequest, Ticker,
};
use std::future::Future;

/// 시장 데이터 조회 trait (공개 API).
///
/// 이 trait은 인증이 필요 없는 작업을 정의합니다.
/// 티커, 호가창, 캔들 데이터 조회 등이 포함됩니다.
pub trait MarketData: Send + Sync {
    /// 거래소 이름을 반환합니다.
    fn name(&self) -> &str;

    /// 하나 이상의 마켓에 대한 현재 티커를 조회합니다.
    ///
    /// # 인자
    ///
    /// * `markets` - 마켓 코드 슬라이스 (예: ["KRW-BTC", "KRW-ETH"])
    fn get_ticker(
        &self,
        markets: &[&str],
    ) -> impl Future<Output = ExchangeResult<Vec<Ticker>>> + Send;

    /// 특정 마켓의 호가창을 조회합니다.
    ///
    /// # 인자
    ///
    /// * `market` - 마켓 코드 (예: "KRW-BTC")
    /// * `depth` - 조회할 호가 단계 수 (선택사항)
    fn get_orderbook(
        &self,
        market: &str,
        depth: Option<u32>,
    ) -> impl Future<Output = ExchangeResult<OrderBook>> + Send;

    /// 특정 마켓의 캔들 데이터를 조회합니다.
    ///
    /// # 인자
    ///
    /// * `market` - 마켓 코드 (예: "KRW-BTC")
    /// * `interval` - 캔들 간격
    /// * `count` - 조회할 캔들 개수
    fn get_candles(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
    ) -> impl Future<Output = ExchangeResult<Vec<Candle>>> + Send;
}

/// 주문 관리 trait (비공개 API).
///
/// 이 trait은 인증이 필요한 작업을 정의합니다.
/// 주문 생성, 계좌 잔고 조회 등이 포함됩니다.
pub trait OrderManagement: Send + Sync {
    /// 새 주문을 생성합니다.
    ///
    /// # 인자
    ///
    /// * `request` - 주문 요청 매개변수
    fn place_order(
        &self,
        request: &OrderRequest,
    ) -> impl Future<Output = ExchangeResult<Order>> + Send;

    /// 기존 주문을 취소합니다.
    ///
    /// # 인자
    ///
    /// * `order_id` - 거래소에서 부여한 주문 ID
    fn cancel_order(&self, order_id: &str) -> impl Future<Output = ExchangeResult<Order>> + Send;

    /// 주문 ID로 주문을 조회합니다.
    ///
    /// # 인자
    ///
    /// * `order_id` - 거래소에서 부여한 주문 ID
    fn get_order(&self, order_id: &str) -> impl Future<Output = ExchangeResult<Order>> + Send;

    /// 특정 마켓의 미체결 주문을 조회합니다.
    ///
    /// # 인자
    ///
    /// * `market` - 마켓 코드 (선택사항, None이면 전체 마켓)
    fn get_open_orders(
        &self,
        market: Option<&str>,
    ) -> impl Future<Output = ExchangeResult<Vec<Order>>> + Send;

    /// 계좌 잔고를 조회합니다.
    fn get_balances(&self) -> impl Future<Output = ExchangeResult<Vec<Balance>>> + Send;

    /// 특정 통화의 잔고를 조회합니다.
    ///
    /// # 인자
    ///
    /// * `currency` - 통화 코드 (예: "BTC", "KRW")
    fn get_balance(&self, currency: &str) -> impl Future<Output = ExchangeResult<Balance>> + Send;
}

/// 거래소 전체 기능을 위한 통합 trait.
///
/// 이 trait은 시장 데이터와 주문 관리 기능을 결합합니다.
pub trait Exchange: MarketData + OrderManagement {
    /// 거래소 클라이언트가 인증되었는지 여부를 반환합니다.
    fn is_authenticated(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    // trait이 사용 사례에 충분히 object-safe한지 확인하는 컴파일 타임 테스트
    fn _assert_send_sync<T: MarketData + Send + Sync>() {}
    fn _assert_order_mgmt<T: OrderManagement + Send + Sync>() {}
}
