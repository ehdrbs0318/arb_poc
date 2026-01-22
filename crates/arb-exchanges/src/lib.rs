//! 거래소 SDK 구현 모듈.
//!
//! 이 모듈은 각 암호화폐 거래소별 구체적인 구현체를 포함합니다.
//!
//! # 지원 거래소
//!
//! - [upbit] - Upbit (한국 거래소)
//! - [bithumb] - Bithumb (한국 거래소)
//! - [bybit] - Bybit V5 (글로벌 거래소)
//!
//! # 예제
//!
//! ```no_run
//! use arb_exchanges::{UpbitClient, BithumbClient, BybitClient};
//! use arb_exchange::MarketData;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Upbit 클라이언트 생성
//!     let upbit = UpbitClient::new()?;
//!     let tickers = upbit.get_ticker(&["KRW-BTC"]).await?;
//!     println!("Upbit BTC: {}", tickers[0].trade_price);
//!
//!     // Bithumb 클라이언트 생성
//!     let bithumb = BithumbClient::new()?;
//!     let tickers = bithumb.get_ticker(&["KRW-BTC"]).await?;
//!     println!("Bithumb BTC: {}", tickers[0].trade_price);
//!
//!     // Bybit 클라이언트 생성
//!     let bybit = BybitClient::new()?;
//!     let tickers = bybit.get_ticker(&["USDT-BTC"]).await?;
//!     println!("Bybit BTC: {}", tickers[0].trade_price);
//!
//!     Ok(())
//! }
//! ```

pub mod bithumb;
pub mod bybit;
pub mod factory;
pub mod upbit;

pub use bithumb::BithumbClient;
pub use bybit::BybitClient;
pub use factory::{
    create_exchange, create_exchange_boxed, BithumbAdapter, BybitAdapter, ExchangeManagerExt,
    ExchangeName, UpbitAdapter,
};
pub use upbit::UpbitClient;
