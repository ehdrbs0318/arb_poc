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
//! use arb_exchanges::BybitClient;
//! use arb_exchange::MarketData;
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

mod auth;
mod client;
mod stream;
mod types;

pub use client::BybitClient;
pub use types::*;
