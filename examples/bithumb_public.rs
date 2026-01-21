//! Bithumb public API example.
//!
//! This example demonstrates how to use the Bithumb client to fetch
//! public market data without authentication.
//!
//! # Run
//!
//! ```bash
//! cargo run --example bithumb_public
//! ```

use arb_poc::exchange::{CandleInterval, MarketData};
use arb_poc::exchanges::BithumbClient;
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Create an unauthenticated client
    let client = BithumbClient::new()?;
    println!("Created Bithumb client: {}", client.name());

    // Fetch ticker for BTC
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

    // Fetch orderbook
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

    // Fetch candles
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

    // Fetch multiple tickers
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
