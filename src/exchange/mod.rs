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
//! - **Factory** ([`factory`]): 설정에서 거래소 인스턴스 생성
//! - **Market** ([`market`]): 마켓 코드 정규화 유틸리티
//! - **Types** ([`types`]): 공통 데이터 구조체 (`Ticker`, `OrderBook`, `Order` 등)
//! - **Error** ([`error`]): 거래소 운영 관련 에러 타입
//!
//! # 예제
//!
//! ```ignore
//! use arb_poc::exchange::{ExchangeManager, ExchangeManagerExt, create_exchange};
//! use arb_poc::config::Config;
//!
//! // 매니저 생성 및 거래소 등록
//! let config = Config::load()?;
//! let mut manager = ExchangeManager::new();
//! manager.register_all_from_config(&config)?;
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
pub mod factory;
pub mod manager;
pub mod market;
pub mod traits;
pub mod types;

// 핵심 trait 재내보내기
pub use error::{ExchangeError, ExchangeResult};
pub use traits::{Exchange, MarketData, OrderManagement};
pub use types::*;

// adapter와 manager 재내보내기
pub use adapter::ExchangeAdapter;
pub use manager::ExchangeManager;

// factory 함수 및 타입 재내보내기
pub use factory::{
    create_exchange, create_exchange_boxed, BithumbAdapter, BybitAdapter, ExchangeManagerExt,
    ExchangeName, UpbitAdapter,
};

// market 유틸리티 재내보내기
pub use market::{
    convert_market_code, create_market_code, get_base_currency, get_quote_currency,
    normalize_currency, parse_market_code, to_exchange_format, to_internal_format,
    MarketCodeBuilder,
};
