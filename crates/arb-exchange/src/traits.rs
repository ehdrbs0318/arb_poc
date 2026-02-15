//! 거래소 trait 정의.
//!
//! 이 모듈은 모든 거래소 구현체가 구현해야 하는 trait을 정의합니다.

use crate::error::ExchangeResult;
use crate::types::{
    Balance, Candle, CandleInterval, InstrumentInfoResponse, Order, OrderBook, OrderRequest,
    PositionInfo, Ticker,
};
use chrono::{DateTime, Utc};
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
    /// 반환값은 timestamp 오름차순 정렬을 보장합니다.
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

    /// 특정 시점 이전의 캔들 데이터를 조회합니다 (페이지네이션용).
    ///
    /// `before`는 exclusive (해당 timestamp 미포함, 직전까지 반환).
    /// 반환값은 timestamp 오름차순 정렬을 보장합니다.
    ///
    /// # 인자
    ///
    /// * `market` - 마켓 코드 (예: "KRW-BTC")
    /// * `interval` - 캔들 간격
    /// * `count` - 조회할 캔들 개수
    /// * `before` - 이 시점 이전의 캔들만 조회 (exclusive)
    fn get_candles_before(
        &self,
        market: &str,
        interval: CandleInterval,
        count: u32,
        before: DateTime<Utc>,
    ) -> impl Future<Output = ExchangeResult<Vec<Candle>>> + Send;

    /// 전종목 Ticker를 조회합니다.
    ///
    /// 거래소가 지원하는 모든 마켓의 현재 티커 정보를 반환합니다.
    /// 코인 자동 선택 등에서 활용됩니다.
    fn get_all_tickers(&self) -> impl Future<Output = ExchangeResult<Vec<Ticker>>> + Send;

    /// 거래소에 맞는 마켓 코드를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `base` - Base 통화 (예: "BTC")
    /// * `quote` - Quote 통화 (예: "KRW", "USDT")
    ///
    /// # 예제
    ///
    /// Upbit: `market_code("BTC", "KRW")` → `"KRW-BTC"`
    /// Bybit: `market_code("BTC", "USDT")` → `"BTCUSDT"`
    fn market_code(base: &str, quote: &str) -> String;
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

/// 선물(linear) 주문 관리 trait.
///
/// Bybit linear 등 선물 거래소에서만 구현합니다.
/// `LiveExecutor`에서 short 진입 및 청산에 사용됩니다.
///
/// 현물(`OrderManagement`)과 분리하여, 카테고리(spot/linear) 혼동을
/// 컴파일 타임에 방지합니다.
pub trait LinearOrderManagement: Send + Sync {
    /// 선물 주문을 생성합니다.
    ///
    /// # 인자
    ///
    /// * `request` - 주문 요청 (market, side, order_type, volume, price 등)
    /// * `reduce_only` - true이면 포지션 축소만 허용 (청산 전용)
    fn place_order_linear(
        &self,
        request: &OrderRequest,
        reduce_only: bool,
    ) -> impl Future<Output = ExchangeResult<Order>> + Send;

    /// 선물 주문을 조회합니다.
    ///
    /// # 인자
    ///
    /// * `order_id` - 거래소에서 부여한 주문 ID
    fn get_order_linear(
        &self,
        order_id: &str,
    ) -> impl Future<Output = ExchangeResult<Order>> + Send;

    /// 선물 주문을 취소합니다.
    ///
    /// # 인자
    ///
    /// * `order_id` - 취소할 주문 ID
    /// * `symbol` - 심볼 (예: "BTCUSDT"). None이면 주문 조회로 확인.
    fn cancel_order_linear(
        &self,
        order_id: &str,
        symbol: Option<&str>,
    ) -> impl Future<Output = ExchangeResult<Order>> + Send;

    /// 선물 포지션을 조회합니다.
    ///
    /// # 인자
    ///
    /// * `symbol` - 심볼 (예: "BTCUSDT")
    fn get_positions_linear(
        &self,
        symbol: &str,
    ) -> impl Future<Output = ExchangeResult<Vec<PositionInfo>>> + Send;
}

/// 거래 규격(instrument info) 조회 trait.
///
/// Bybit 등 instrument info API를 제공하는 거래소만 구현합니다.
pub trait InstrumentDataProvider: Send + Sync {
    /// 심볼의 거래 규격을 조회합니다.
    ///
    /// # 인자
    ///
    /// * `symbol` - 거래소 형식의 심볼 (예: "BTCUSDT")
    ///
    /// # 반환값
    ///
    /// 해당 심볼의 가격/수량 규격 정보를 반환합니다.
    fn get_instrument_info(
        &self,
        symbol: &str,
    ) -> impl Future<Output = ExchangeResult<InstrumentInfoResponse>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    // trait이 사용 사례에 충분히 object-safe한지 확인하는 컴파일 타임 테스트
    fn _assert_send_sync<T: MarketData + Send + Sync>() {}
    fn _assert_order_mgmt<T: OrderManagement + Send + Sync>() {}
    fn _assert_instrument_data<T: InstrumentDataProvider + Send + Sync>() {}
}
