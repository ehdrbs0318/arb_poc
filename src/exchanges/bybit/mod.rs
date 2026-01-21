//! Bybit V5 거래소 SDK 구현.
//!
//! 이 모듈은 Bybit V5 암호화폐 거래소와 상호작용하기 위한 클라이언트를 제공합니다.
//!
//! # 기능
//!
//! - 시장 데이터 API: 시세(Ticker), 호가창, 캔들(klines)
//! - 거래 API: 주문 생성, 주문 취소, 주문 조회 (인증 필요)
//! - 계정 API: 지갑 잔고 (인증 필요)
//! - HMAC-SHA256 인증
//! - spot, linear, inverse, option 거래 지원
//!
//! # 지원 카테고리
//!
//! - `spot`: 현물 거래
//! - `linear`: USDT/USDC 무기한 계약
//! - `inverse`: inverse 무기한 계약
//! - `option`: 옵션 거래
//!
//! # 예제
//!
//! ```no_run
//! use arb_poc::exchanges::BybitClient;
//! use arb_poc::exchange::MarketData;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 공개 API용 인증되지 않은 클라이언트 생성
//!     let client = BybitClient::new()?;
//!
//!     // 시세 조회 (공통 마켓 형식 QUOTE-BASE 사용)
//!     let tickers = client.get_ticker(&["USDT-BTC"]).await?;
//!     println!("BTC Price: {}", tickers[0].trade_price);
//!
//!     // linear 무기한용 클라이언트 생성
//!     let perp_client = BybitClient::new()?.with_category("linear");
//!
//!     Ok(())
//! }
//! ```
//!
//! # 인증 예제
//!
//! ```no_run
//! use arb_poc::exchanges::BybitClient;
//! use arb_poc::exchange::{OrderManagement, OrderRequest};
//! use rust_decimal::Decimal;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 인증된 클라이언트 생성
//!     let client = BybitClient::with_credentials("your_api_key", "your_secret_key")?;
//!
//!     // 계정 잔고 조회
//!     let balances = client.get_balances().await?;
//!     for balance in balances {
//!         println!("{}: {}", balance.currency, balance.balance);
//!     }
//!
//!     // 지정가 매수 주문
//!     let order = OrderRequest::limit_buy(
//!         "USDT-BTC",
//!         Decimal::new(40000, 0),  // 가격
//!         Decimal::new(1, 3),      // 수량 (0.001 BTC)
//!     );
//!     let result = client.place_order(&order).await?;
//!     println!("Order placed: {}", result.id);
//!
//!     Ok(())
//! }
//! ```
//!
//! # 테스트넷 지원
//!
//! ```no_run
//! use arb_poc::exchanges::BybitClient;
//!
//! // 테스트넷 공개 API
//! let client = BybitClient::new_testnet()?;
//!
//! // 테스트넷 인증 API
//! let client = BybitClient::with_credentials_testnet("test_key", "test_secret")?;
//! # Ok::<(), arb_poc::exchange::ExchangeError>(())
//! ```

mod auth;
mod client;
mod types;

pub use client::BybitClient;
pub use types::*;
