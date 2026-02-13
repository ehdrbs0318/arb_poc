//! # arb_poc
//!
//! 암호화폐 거래소를 위한 차익거래 시스템 개념 증명(PoC)입니다.
//!
//! 이 크레이트는 여러 암호화폐 거래소와 상호작용하고
//! 차익거래 전략을 구현하기 위한 추상화를 제공합니다.
//!
//! ## 아키텍처
//!
//! 이 크레이트는 workspace crates를 re-export합니다:
//!
//! - [`config`]: 설정 관리 (from `arb-config`)
//! - [`logging`]: 구조화된 로깅 시스템 (from `arb-logging`)
//! - [`telegram`]: Telegram 알림 시스템 (from `arb-telegram`)
//! - [`exchange`]: 거래소 추상화를 위한 공통 trait 및 타입 (from `arb-exchange`)
//! - [`exchanges`]: 특정 거래소 구현체 (from `arb-exchanges`)
//! - [`forex`]: USD/KRW 환율 캐시 (from `arb-forex`)
//! - [`strategy`]: 차익거래 전략 구현 (from `arb-strategy`)
//! - [`db`]: MySQL 영속화 레이어 (from `arb-db`)
//!
//! ## 예제
//!
//! ```no_run
//! use arb_poc::config::Config;
//! use arb_poc::exchange::MarketData;
//! use arb_poc::exchanges::UpbitClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::load_or_default();
//!     let client = UpbitClient::new()?;
//!     let tickers = client.get_ticker(&["KRW-BTC"]).await?;
//!     println!("BTC: {}", tickers[0].trade_price);
//!     Ok(())
//! }
//! ```

// arb-db ↔ arb-strategy 어댑터
pub mod adapter;

// Re-export workspace crates
pub use arb_config as config;
pub use arb_db as db;
pub use arb_exchange as exchange;
pub use arb_exchanges as exchanges;
pub use arb_forex as forex;
pub use arb_logging as logging;
pub use arb_strategy as strategy;
pub use arb_telegram as telegram;
