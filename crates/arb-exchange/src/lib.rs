//! 거래소 추상화 계층.
//!
//! 이 모듈은 모든 거래소 구현체가 따라야 하는 공통 trait과 타입을 정의합니다.
//!
//! # 아키텍처
//!
//! 거래소 추상화 계층은 다음 구성요소로 이루어져 있습니다:
//!
//! - **Traits** ([`traits`]): 거래소 운영을 위한 핵심 trait (`MarketData`, `OrderManagement`, `Exchange`)
//! - **Adapter** ([`adapter`]): 동적 디스패치를 위한 object-safe trait (`ExchangeAdapter`)
//! - **Manager** ([`manager`]): 중앙집중식 거래소 관리 (`ExchangeManager`)
//! - **Market** ([`market`]): 마켓 코드 정규화 유틸리티
//! - **Types** ([`types`]): 공통 데이터 구조체 (`Ticker`, `OrderBook`, `Order` 등)
//! - **Error** ([`error`]): 거래소 운영 관련 에러 타입
//!
//! # 예제
//!
//! ```ignore
//! use arb_exchange::{ExchangeManager, ExchangeAdapter};
//!
//! // 매니저 생성 및 거래소 등록
//! let mut manager = ExchangeManager::new();
//!
//! // 거래소 동적 사용
//! for name in manager.list_exchanges() {
//!     let exchange = manager.get(name).unwrap();
//!     let ticker = exchange.get_ticker(&["KRW-BTC"]).await?;
//!     println!("[{}] BTC: {}", name, ticker[0].trade_price);
//! }
//! ```

pub mod adapter;
pub mod error;
pub mod manager;
pub mod market;
pub mod stream;
pub mod traits;
pub mod types;

// chrono 재내보내기 (전략 크레이트에서 DateTime<Utc> 사용 시 호환성 보장)
pub use chrono;

// 핵심 trait 재내보내기
pub use error::{ExchangeError, ExchangeResult};
pub use traits::{Exchange, MarketData, OrderManagement};
pub use types::*;

// adapter와 manager 재내보내기
pub use adapter::ExchangeAdapter;
pub use manager::ExchangeManager;

// stream trait 재내보내기
pub use stream::{MarketEvent, MarketStream, StreamConfig};

// market 유틸리티 재내보내기
pub use market::{
    ExchangeName, MarketCodeBuilder, convert_market_code, create_market_code, get_base_currency,
    get_quote_currency, normalize_currency, parse_market_code, to_exchange_format,
    to_internal_format,
};
