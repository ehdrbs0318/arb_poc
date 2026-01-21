//! Bybit V5 exchange SDK implementation.
//!
//! This module provides a client for interacting with the Bybit V5 cryptocurrency exchange.
//!
//! # Features
//!
//! - Market Data API: Tickers, order books, candles (klines)
//! - Trading API: Place orders, cancel orders, query orders (requires authentication)
//! - Account API: Wallet balances (requires authentication)
//! - HMAC-SHA256 authentication
//! - Support for spot, linear, inverse, and option trading
//!
//! # Supported Categories
//!
//! - `spot`: Spot trading
//! - `linear`: USDT/USDC perpetual contracts
//! - `inverse`: Inverse perpetual contracts
//! - `option`: Options trading
//!
//! # Example
//!
//! ```no_run
//! use arb_poc::exchanges::BybitClient;
//! use arb_poc::exchange::MarketData;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create an unauthenticated client for public API
//!     let client = BybitClient::new()?;
//!
//!     // Fetch ticker (using common market format QUOTE-BASE)
//!     let tickers = client.get_ticker(&["USDT-BTC"]).await?;
//!     println!("BTC Price: {}", tickers[0].trade_price);
//!
//!     // Create a client for linear perpetuals
//!     let perp_client = BybitClient::new()?.with_category("linear");
//!
//!     Ok(())
//! }
//! ```
//!
//! # Authentication Example
//!
//! ```no_run
//! use arb_poc::exchanges::BybitClient;
//! use arb_poc::exchange::{OrderManagement, OrderRequest};
//! use rust_decimal::Decimal;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create an authenticated client
//!     let client = BybitClient::with_credentials("your_api_key", "your_secret_key")?;
//!
//!     // Get account balances
//!     let balances = client.get_balances().await?;
//!     for balance in balances {
//!         println!("{}: {}", balance.currency, balance.balance);
//!     }
//!
//!     // Place a limit order
//!     let order = OrderRequest::limit_buy(
//!         "USDT-BTC",
//!         Decimal::new(40000, 0),  // price
//!         Decimal::new(1, 3),      // volume (0.001 BTC)
//!     );
//!     let result = client.place_order(&order).await?;
//!     println!("Order placed: {}", result.id);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Testnet Support
//!
//! ```no_run
//! use arb_poc::exchanges::BybitClient;
//!
//! // Public API on testnet
//! let client = BybitClient::new_testnet()?;
//!
//! // Authenticated API on testnet
//! let client = BybitClient::with_credentials_testnet("test_key", "test_secret")?;
//! # Ok::<(), arb_poc::exchange::ExchangeError>(())
//! ```

mod auth;
mod client;
mod types;

pub use client::BybitClient;
pub use types::*;
