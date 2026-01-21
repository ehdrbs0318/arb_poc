//! Exchange abstraction layer.
//!
//! This module defines the common traits and types that all exchange
//! implementations must adhere to.
//!
//! # Architecture
//!
//! The exchange abstraction layer consists of several components:
//!
//! - **Traits** ([`traits`]): Core traits for exchange operations (`MarketData`, `OrderManagement`, `Exchange`)
//! - **Adapter** ([`adapter`]): Object-safe trait for dynamic dispatch (`ExchangeAdapter`)
//! - **Manager** ([`manager`]): Centralized exchange management (`ExchangeManager`)
//! - **Factory** ([`factory`]): Exchange instantiation from configuration
//! - **Market** ([`market`]): Market code normalization utilities
//! - **Types** ([`types`]): Common data structures (`Ticker`, `OrderBook`, `Order`, etc.)
//! - **Error** ([`error`]): Error types for exchange operations
//!
//! # Example
//!
//! ```ignore
//! use arb_poc::exchange::{ExchangeManager, ExchangeManagerExt, create_exchange};
//! use arb_poc::config::Config;
//!
//! // Create manager and register exchanges
//! let config = Config::load()?;
//! let mut manager = ExchangeManager::new();
//! manager.register_all_from_config(&config)?;
//!
//! // Use exchanges dynamically
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

// Re-export core traits
pub use error::{ExchangeError, ExchangeResult};
pub use traits::{Exchange, MarketData, OrderManagement};
pub use types::*;

// Re-export adapter and manager
pub use adapter::ExchangeAdapter;
pub use manager::ExchangeManager;

// Re-export factory functions and types
pub use factory::{
    create_exchange, create_exchange_boxed, BithumbAdapter, BybitAdapter, ExchangeManagerExt,
    ExchangeName, UpbitAdapter,
};

// Re-export market utilities
pub use market::{
    convert_market_code, create_market_code, get_base_currency, get_quote_currency,
    normalize_currency, parse_market_code, to_exchange_format, to_internal_format,
    MarketCodeBuilder,
};
