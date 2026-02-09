//! 차익거래 전략 구현 크레이트.
//!
//! 이 크레이트는 거래소 추상화 계층(`arb-exchange`)의 trait에만 의존하며,
//! 구체적인 거래소 SDK(`arb-exchanges`)에는 의존하지 않습니다 (DI 패턴).

pub mod common;
pub mod error;
pub mod zscore;

pub use error::{PositionError, StatisticsError, StrategyError};
