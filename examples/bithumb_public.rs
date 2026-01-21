//! Bithumb 공개 API 예제.
//!
//! 이 예제는 인증 없이 Bithumb 클라이언트를 사용하여
//! 공개 시장 데이터를 조회하는 방법을 보여줍니다.
//!
//! # 실행
//!
//! ```bash
//! cargo run --example bithumb_public
//! ```

use arb_poc::exchange::{CandleInterval, MarketData};
use arb_poc::exchanges::BithumbClient;
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅을 위한 tracing 초기화
    tracing_subscriber::fmt::init();

    // 인증되지 않은 클라이언트 생성
    let client = BithumbClient::new()?;
    println!("Created Bithumb client: {}", client.name());

    // BTC 시세 조회
    println!("\n--- Ticker (KRW-BTC) ---");
    match client.get_ticker(&["KRW-BTC"]).await {
        Ok(tickers) => {
            for ticker in &tickers {
                println!("Market: {}", ticker.market);
                println!("Trade Price: {} KRW", ticker.trade_price);
                println!(
                    "Change: {:?} ({:.2}%)",
                    ticker.change,
                    ticker.change_rate * Decimal::from(100)
                );
                println!("24h Volume: {} BTC", ticker.acc_trade_volume_24h);
                println!("24h Trade Price: {} KRW", ticker.acc_trade_price_24h);
            }
        }
        Err(e) => println!("Error fetching ticker: {}", e),
    }

    // 호가창 조회
    println!("\n--- Order Book (KRW-BTC) ---");
    match client.get_orderbook("KRW-BTC", Some(5)).await {
        Ok(orderbook) => {
            println!("Market: {}", orderbook.market);
            println!("Total Ask Size: {} BTC", orderbook.total_ask_size);
            println!("Total Bid Size: {} BTC", orderbook.total_bid_size);

            if let Some(spread) = orderbook.spread() {
                println!("Spread: {} KRW", spread);
            }

            println!("\nTop 5 Asks (Sell Orders):");
            for (i, ask) in orderbook.asks.iter().take(5).enumerate() {
                println!("  {}. {} KRW - {} BTC", i + 1, ask.price, ask.size);
            }

            println!("\nTop 5 Bids (Buy Orders):");
            for (i, bid) in orderbook.bids.iter().take(5).enumerate() {
                println!("  {}. {} KRW - {} BTC", i + 1, bid.price, bid.size);
            }
        }
        Err(e) => println!("Error fetching orderbook: {}", e),
    }

    // 캔들 데이터 조회
    println!("\n--- Candles (KRW-BTC, 1 minute) ---");
    match client
        .get_candles("KRW-BTC", CandleInterval::Minute1, 5)
        .await
    {
        Ok(candles) => {
            for candle in &candles {
                println!(
                    "{}: O={} H={} L={} C={} V={}",
                    candle.timestamp.format("%Y-%m-%d %H:%M"),
                    candle.open,
                    candle.high,
                    candle.low,
                    candle.close,
                    candle.volume
                );
            }
        }
        Err(e) => println!("Error fetching candles: {}", e),
    }

    // 다중 시세 조회
    println!("\n--- Multiple Tickers ---");
    match client.get_ticker(&["KRW-BTC", "KRW-ETH", "KRW-XRP"]).await {
        Ok(tickers) => {
            for ticker in &tickers {
                println!(
                    "{}: {} KRW ({:?})",
                    ticker.market, ticker.trade_price, ticker.change
                );
            }
        }
        Err(e) => println!("Error fetching tickers: {}", e),
    }

    Ok(())
}
