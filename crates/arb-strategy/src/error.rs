//! 전략 에러 타입 정의.

use arb_exchange::ExchangeError;
use rust_decimal::Decimal;
use thiserror::Error;

/// 통계 연산 에러.
#[derive(Error, Debug)]
pub enum StatisticsError {
    /// 표준편차가 0인 경우 z-score를 계산할 수 없습니다.
    #[error("Standard deviation is zero, cannot compute z-score")]
    ZeroDivision,

    /// 계산 결과에 NaN이 검출되었습니다.
    #[error("NaN detected in calculation: {0}")]
    NanDetected(String),

    /// 충분한 데이터가 없습니다.
    #[error("Insufficient data: need {required}, have {actual}")]
    InsufficientData {
        /// 필요한 데이터 수
        required: usize,
        /// 현재 보유한 데이터 수
        actual: usize,
    },

    /// 값이 최소 임계값 미만입니다.
    #[error("Value below minimum threshold: {value} < {threshold}")]
    BelowThreshold {
        /// 현재 값
        value: f64,
        /// 최소 임계값
        threshold: f64,
    },
}

/// 포지션 에러.
#[derive(Error, Debug)]
pub enum PositionError {
    /// 해당 코인에 대한 포지션이 이미 존재합니다.
    #[error("Position already exists for {coin}")]
    AlreadyExists {
        /// 코인 심볼
        coin: String,
    },

    /// 해당 코인에 대한 포지션을 찾을 수 없습니다.
    #[error("Position not found for {coin}")]
    NotFound {
        /// 코인 심볼
        coin: String,
    },

    /// 자본이 부족합니다.
    #[error("Insufficient capital: need {required}, available {available}")]
    InsufficientCapital {
        /// 필요 자본
        required: Decimal,
        /// 가용 자본
        available: Decimal,
    },

    /// 포지션이 청산되었습니다.
    #[error("Position liquidated for {coin} at price {price}")]
    Liquidated {
        /// 코인 심볼
        coin: String,
        /// 청산 가격
        price: Decimal,
    },
}

/// 전략 에러.
#[derive(Error, Debug)]
pub enum StrategyError {
    /// 거래소 에러.
    #[error("Exchange error: {0}")]
    Exchange(#[from] ExchangeError),

    /// 통계 연산 에러.
    #[error("Statistics error: {0}")]
    Statistics(#[from] StatisticsError),

    /// 설정 에러.
    #[error("Configuration error: {0}")]
    Config(String),

    /// 데이터 정렬 에러.
    #[error("Data alignment error: {0}")]
    DataAlignment(String),

    /// 포지션 관련 에러.
    #[error("Position error: {0}")]
    Position(#[from] PositionError),

    /// IO 에러.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
