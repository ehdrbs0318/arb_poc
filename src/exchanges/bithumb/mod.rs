//! Bithumb exchange SDK implementation.
//!
//! This module provides a client for interacting with the Bithumb cryptocurrency exchange.
//!
//! # Features
//!
//! - Public API: Market data, tickers, order books, candles
//! - Private API: Orders, account balances (requires authentication)
//! - JWT authentication with SHA512 query hash
//!
//! # Example
//!
//! ```no_run
//! use arb_poc::exchanges::BithumbClient;
//! use arb_poc::exchange::MarketData;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create an unauthenticated client for public API
//!     let client = BithumbClient::new()?;
//!
//!     // Fetch ticker
//!     let tickers = client.get_ticker(&["KRW-BTC"]).await?;
//!     println!("BTC Price: {}", tickers[0].trade_price);
//!
//!     Ok(())
//! }
//! ```

mod auth;
mod client;
mod types;

pub use client::BithumbClient;
pub use types::*;
