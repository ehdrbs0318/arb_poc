//! 거래소 추상화를 위한 공통 타입.
//!
//! 이 모듈은 모든 거래소 구현에서 사용되는 데이터 구조를 정의합니다.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 거래 쌍/마켓을 나타냅니다 (예: "KRW-BTC", "BTC-USDT").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Market {
    /// 마켓 코드 (예: "KRW-BTC").
    pub code: String,
    /// 기준 통화 (예: "BTC").
    pub base: String,
    /// 견적 통화 (예: "KRW").
    pub quote: String,
}

impl Market {
    /// 코드 문자열로부터 새 Market을 생성합니다.
    ///
    /// # 매개변수
    ///
    /// * `code` - "QUOTE-BASE" 형식의 마켓 코드 (예: "KRW-BTC")
    ///
    /// # 반환값
    ///
    /// 코드가 유효하면 `Some(Market)`을, 그렇지 않으면 `None`을 반환합니다.
    #[must_use]
    pub fn from_code(code: &str) -> Option<Self> {
        let parts: Vec<&str> = code.split('-').collect();
        if parts.len() == 2 {
            Some(Self {
                code: code.to_string(),
                base: parts[1].to_string(),
                quote: parts[0].to_string(),
            })
        } else {
            None
        }
    }
}

/// 마켓의 현재 가격 정보.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    /// 마켓 코드.
    pub market: String,
    /// 현재 거래 가격.
    pub trade_price: Decimal,
    /// 시가.
    pub opening_price: Decimal,
    /// 24시간 최고가.
    pub high_price: Decimal,
    /// 24시간 최저가.
    pub low_price: Decimal,
    /// 전일 종가.
    pub prev_closing_price: Decimal,
    /// 가격 변동 상태.
    pub change: PriceChange,
    /// 변동률 (백분율).
    pub change_rate: Decimal,
    /// 변동 가격 (절대값).
    pub change_price: Decimal,
    /// 24시간 누적 거래량.
    pub acc_trade_volume_24h: Decimal,
    /// 24시간 누적 거래 금액.
    pub acc_trade_price_24h: Decimal,
    /// 타임스탬프.
    pub timestamp: DateTime<Utc>,
}

/// 가격 변동 방향.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PriceChange {
    /// 가격 상승.
    Rise,
    /// 가격 하락.
    Fall,
    /// 가격 변동 없음.
    #[default]
    Even,
}

/// 호가창의 단일 호가 레벨.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    /// 해당 레벨의 가격.
    pub price: Decimal,
    /// 해당 레벨의 수량.
    pub size: Decimal,
}

/// 마켓의 호가창 스냅샷.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// 마켓 코드.
    pub market: String,
    /// 매도(ask) 호가, 가격 오름차순 정렬.
    pub asks: Vec<OrderBookLevel>,
    /// 매수(bid) 호가, 가격 내림차순 정렬.
    pub bids: Vec<OrderBookLevel>,
    /// 총 매도 잔량.
    pub total_ask_size: Decimal,
    /// 총 매수 잔량.
    pub total_bid_size: Decimal,
    /// 타임스탬프.
    pub timestamp: DateTime<Utc>,
}

impl OrderBook {
    /// 최우선 매도호가(가장 낮은 매도 가격)를 반환합니다.
    #[must_use]
    pub fn best_ask(&self) -> Option<&OrderBookLevel> {
        self.asks.first()
    }

    /// 최우선 매수호가(가장 높은 매수 가격)를 반환합니다.
    #[must_use]
    pub fn best_bid(&self) -> Option<&OrderBookLevel> {
        self.bids.first()
    }

    /// 최우선 매도호가와 매수호가 사이의 스프레드를 계산합니다.
    #[must_use]
    pub fn spread(&self) -> Option<Decimal> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some(ask.price - bid.price),
            _ => None,
        }
    }

    /// 스프레드 백분율을 계산합니다.
    #[must_use]
    pub fn spread_percentage(&self) -> Option<Decimal> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) if bid.price > Decimal::ZERO => {
                Some((ask.price - bid.price) / bid.price * Decimal::from(100))
            }
            _ => None,
        }
    }
}

/// 캔들 (OHLCV) 데이터.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// 마켓 코드.
    pub market: String,
    /// 캔들 타임스탬프 (해당 기간의 시작).
    pub timestamp: DateTime<Utc>,
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
}

/// 캔들 간격/타임프레임.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandleInterval {
    /// 1분.
    Minute1,
    /// 3분.
    Minute3,
    /// 5분.
    Minute5,
    /// 10분.
    Minute10,
    /// 15분.
    Minute15,
    /// 30분.
    Minute30,
    /// 60분 (1시간).
    Minute60,
    /// 240분 (4시간).
    Minute240,
    /// 1일.
    Day,
    /// 1주.
    Week,
    /// 1개월.
    Month,
}

impl CandleInterval {
    /// 간격을 분 단위로 반환합니다.
    #[must_use]
    pub const fn as_minutes(&self) -> u32 {
        match self {
            Self::Minute1 => 1,
            Self::Minute3 => 3,
            Self::Minute5 => 5,
            Self::Minute10 => 10,
            Self::Minute15 => 15,
            Self::Minute30 => 30,
            Self::Minute60 => 60,
            Self::Minute240 => 240,
            Self::Day => 1440,
            Self::Week => 10080,
            Self::Month => 43200,
        }
    }
}

/// 주문 방향 (매수 또는 매도).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    /// 매수 주문 (bid).
    #[serde(alias = "bid")]
    Buy,
    /// 매도 주문 (ask).
    #[serde(alias = "ask")]
    Sell,
}

/// 주문 유형.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    /// 지정가 주문.
    Limit,
    /// 시장가 주문.
    Market,
    /// 시장가 매수 주문 (총액 기준).
    Price,
    /// 최유리 지정가 주문.
    Best,
}

/// 주문의 유효 기간 조건.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    /// 취소될 때까지 유효 (Good Till Cancelled).
    Gtc,
    /// 즉시 체결 또는 취소 (Immediate Or Cancel).
    Ioc,
    /// 전량 체결 또는 취소 (Fill Or Kill).
    Fok,
    /// 메이커 주문만 허용 (Post Only).
    PostOnly,
}

/// 주문 상태.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// 주문 대기 중.
    Wait,
    /// 주문 처리 중.
    Watch,
    /// 부분 체결됨.
    PartiallyFilled,
    /// 전량 체결됨.
    Filled,
    /// 취소됨.
    Cancelled,
    /// 거부됨.
    Rejected,
}

/// 주문 요청 매개변수.
#[derive(Debug, Clone)]
pub struct OrderRequest {
    /// 마켓 코드.
    pub market: String,
    /// 주문 방향 (매수/매도).
    pub side: OrderSide,
    /// 주문 유형.
    pub order_type: OrderType,
    /// 주문 수량. 지정가 주문 및 시장가 매도 주문에 필수.
    pub volume: Option<Decimal>,
    /// 주문 가격. 지정가 주문 및 시장가 매수 주문(총액)에 필수.
    pub price: Option<Decimal>,
    /// 유효 기간 조건.
    pub time_in_force: Option<TimeInForce>,
    /// 클라이언트 정의 식별자.
    pub identifier: Option<String>,
}

impl OrderRequest {
    /// 지정가 매수 주문 요청을 생성합니다.
    #[must_use]
    pub fn limit_buy(market: impl Into<String>, price: Decimal, volume: Decimal) -> Self {
        Self {
            market: market.into(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            volume: Some(volume),
            price: Some(price),
            time_in_force: None,
            identifier: None,
        }
    }

    /// 지정가 매도 주문 요청을 생성합니다.
    #[must_use]
    pub fn limit_sell(market: impl Into<String>, price: Decimal, volume: Decimal) -> Self {
        Self {
            market: market.into(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            volume: Some(volume),
            price: Some(price),
            time_in_force: None,
            identifier: None,
        }
    }

    /// 시장가 매수 주문 요청을 생성합니다 (총액 기준).
    #[must_use]
    pub fn market_buy(market: impl Into<String>, total: Decimal) -> Self {
        Self {
            market: market.into(),
            side: OrderSide::Buy,
            order_type: OrderType::Price,
            volume: None,
            price: Some(total),
            time_in_force: None,
            identifier: None,
        }
    }

    /// 시장가 매도 주문 요청을 생성합니다 (수량 기준).
    #[must_use]
    pub fn market_sell(market: impl Into<String>, volume: Decimal) -> Self {
        Self {
            market: market.into(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            volume: Some(volume),
            price: None,
            time_in_force: None,
            identifier: None,
        }
    }

    /// 유효 기간 조건을 설정합니다.
    #[must_use]
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// 클라이언트 식별자를 설정합니다.
    #[must_use]
    pub fn with_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }
}

/// 거래소로부터의 주문 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// 거래소에서 할당한 주문 ID.
    pub id: String,
    /// 마켓 코드.
    pub market: String,
    /// 주문 방향.
    pub side: OrderSide,
    /// 주문 유형.
    pub order_type: OrderType,
    /// 주문 상태.
    pub status: OrderStatus,
    /// 원래 주문 수량.
    pub volume: Decimal,
    /// 미체결 수량.
    pub remaining_volume: Decimal,
    /// 체결 수량.
    pub executed_volume: Decimal,
    /// 주문 가격 (지정가 주문의 경우).
    pub price: Option<Decimal>,
    /// 평균 체결 가격.
    pub avg_price: Option<Decimal>,
    /// 지불한 수수료.
    pub paid_fee: Decimal,
    /// 주문 생성 타임스탬프.
    pub created_at: DateTime<Utc>,
    /// 클라이언트 식별자 (제공된 경우).
    pub identifier: Option<String>,
}

/// 단일 통화의 계정 잔고.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// 통화 코드 (예: "BTC", "KRW").
    pub currency: String,
    /// 가용 잔고.
    pub balance: Decimal,
    /// 잠긴 잔고 (주문에 사용 중).
    pub locked: Decimal,
    /// 평균 매수 가격.
    pub avg_buy_price: Decimal,
    /// 평균 매수 가격의 단위 통화.
    pub unit_currency: String,
}

impl Balance {
    /// 총 잔고 (가용 + 잠김)를 반환합니다.
    #[must_use]
    pub fn total(&self) -> Decimal {
        self.balance + self.locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_from_code_valid() {
        let market = Market::from_code("KRW-BTC").unwrap();
        assert_eq!(market.code, "KRW-BTC");
        assert_eq!(market.base, "BTC");
        assert_eq!(market.quote, "KRW");
    }

    #[test]
    fn test_market_from_code_invalid() {
        assert!(Market::from_code("INVALID").is_none());
        assert!(Market::from_code("").is_none());
        assert!(Market::from_code("A-B-C").is_none());
    }

    #[test]
    fn test_orderbook_best_prices() {
        let ob = OrderBook {
            market: "KRW-BTC".to_string(),
            asks: vec![
                OrderBookLevel {
                    price: Decimal::from(100),
                    size: Decimal::from(1),
                },
                OrderBookLevel {
                    price: Decimal::from(101),
                    size: Decimal::from(2),
                },
            ],
            bids: vec![
                OrderBookLevel {
                    price: Decimal::from(99),
                    size: Decimal::from(1),
                },
                OrderBookLevel {
                    price: Decimal::from(98),
                    size: Decimal::from(2),
                },
            ],
            total_ask_size: Decimal::from(3),
            total_bid_size: Decimal::from(3),
            timestamp: Utc::now(),
        };

        assert_eq!(ob.best_ask().unwrap().price, Decimal::from(100));
        assert_eq!(ob.best_bid().unwrap().price, Decimal::from(99));
        assert_eq!(ob.spread().unwrap(), Decimal::from(1));
    }

    #[test]
    fn test_order_request_builders() {
        let order = OrderRequest::limit_buy("KRW-BTC", Decimal::from(50000000), Decimal::from(1));
        assert_eq!(order.market, "KRW-BTC");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.price, Some(Decimal::from(50000000)));
        assert_eq!(order.volume, Some(Decimal::from(1)));
    }

    #[test]
    fn test_balance_total() {
        let balance = Balance {
            currency: "BTC".to_string(),
            balance: Decimal::from(10),
            locked: Decimal::from(5),
            avg_buy_price: Decimal::from(50000000),
            unit_currency: "KRW".to_string(),
        };
        assert_eq!(balance.total(), Decimal::from(15));
    }

    #[test]
    fn test_candle_interval_as_minutes() {
        assert_eq!(CandleInterval::Minute1.as_minutes(), 1);
        assert_eq!(CandleInterval::Minute60.as_minutes(), 60);
        assert_eq!(CandleInterval::Day.as_minutes(), 1440);
    }
}
