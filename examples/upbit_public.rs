//! # Upbit 공개 API 예제
//!
//! 이 예제는 Upbit 공개(시세 조회) API를 사용하여
//! 인증 없이 시장 데이터를 조회하는 방법을 보여줍니다.
//!
//! ## 예제 실행 방법
//!
//! ```bash
//! cargo run --example upbit_public
//! ```
//!
//! 이 예제는 API 인증 정보가 필요하지 않습니다.

use arb_poc::exchange::{CandleInterval, MarketData};
use arb_poc::exchanges::UpbitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Upbit Public API Example ===\n");

    // 인증 없이 클라이언트 생성
    let client = UpbitClient::new()?;
    println!("Exchange: {}\n", client.name());

    // 1. 여러 마켓의 티커 조회
    println!("--- Tickers ---");
    let markets = ["KRW-BTC", "KRW-ETH", "KRW-XRP"];
    let tickers = client.get_ticker(&markets).await?;

    for ticker in &tickers {
        let change_str = match ticker.change {
            arb_poc::exchange::PriceChange::Rise => "+",
            arb_poc::exchange::PriceChange::Fall => "-",
            arb_poc::exchange::PriceChange::Even => " ",
        };
        println!(
            "{}: {:>15} KRW ({}{:.2}%)",
            ticker.market,
            ticker.trade_price,
            change_str,
            ticker.change_rate * rust_decimal::Decimal::from(100)
        );
    }
    println!();

    // 2. 호가창 조회
    println!("--- Order Book (KRW-BTC) ---");
    let orderbook = client.get_orderbook("KRW-BTC", Some(5)).await?;

    println!(
        "Best Ask: {} KRW (size: {})",
        orderbook
            .best_ask()
            .map(|a| a.price.to_string())
            .unwrap_or_default(),
        orderbook
            .best_ask()
            .map(|a| a.size.to_string())
            .unwrap_or_default()
    );
    println!(
        "Best Bid: {} KRW (size: {})",
        orderbook
            .best_bid()
            .map(|b| b.price.to_string())
            .unwrap_or_default(),
        orderbook
            .best_bid()
            .map(|b| b.size.to_string())
            .unwrap_or_default()
    );

    if let Some(spread) = orderbook.spread() {
        println!(
            "Spread: {} KRW ({:.4}%)",
            spread,
            orderbook.spread_percentage().unwrap_or_default()
        );
    }
    println!("Total Ask Size: {}", orderbook.total_ask_size);
    println!("Total Bid Size: {}", orderbook.total_bid_size);
    println!();

    // 3. 캔들 데이터 조회
    println!("--- Recent Candles (KRW-BTC, 15min) ---");
    let candles = client
        .get_candles("KRW-BTC", CandleInterval::Minute15, 5)
        .await?;

    for candle in &candles {
        println!(
            "{}: O:{} H:{} L:{} C:{} V:{:.4}",
            candle.timestamp.format("%Y-%m-%d %H:%M"),
            candle.open,
            candle.high,
            candle.low,
            candle.close,
            candle.volume
        );
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
