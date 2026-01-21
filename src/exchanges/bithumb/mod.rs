//! Bithumb 거래소 SDK 구현.
//!
//! 이 모듈은 Bithumb 암호화폐 거래소와 상호작용하기 위한 클라이언트를 제공합니다.
//!
//! # 기능
//!
//! - Public API: 시세, 티커, 호가창, 캔들 데이터
//! - Private API: 주문, 계좌 잔고 (인증 필요)
//! - SHA512 쿼리 해시를 사용한 JWT 인증
//!
//! # 예제
//!
//! ```no_run
//! use arb_poc::exchanges::BithumbClient;
//! use arb_poc::exchange::MarketData;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Public API용 인증되지 않은 클라이언트 생성
//!     let client = BithumbClient::new()?;
//!
//!     // 티커 조회
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
