//! Upbit 거래소 SDK 구현 모듈.
//!
//! 이 모듈은 Upbit 암호화폐 거래소와 상호작용하기 위한 클라이언트를 제공합니다.
//!
//! # 기능
//!
//! - Quotation API: 시장 데이터, 시세, 호가창, 캔들 조회
//! - Exchange API: 주문, 계좌 잔고 조회 (인증 필요)
//! - SHA512 쿼리 해시를 사용한 JWT 인증
//!
//! # 예제
//!
//! ```no_run
//! use arb_exchanges::UpbitClient;
//! use arb_exchange::MarketData;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 공개 API용 인증 없는 클라이언트 생성
//!     let client = UpbitClient::new()?;
//!
//!     // 시세 조회
//!     let tickers = client.get_ticker(&["KRW-BTC"]).await?;
//!     println!("BTC Price: {}", tickers[0].trade_price);
//!
//!     Ok(())
//! }
//! ```

mod auth;
mod client;
mod stream;
mod types;

pub use client::UpbitClient;
pub use types::*;
