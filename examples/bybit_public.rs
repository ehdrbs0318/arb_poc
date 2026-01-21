//! Bybit public API example.
//!
//! This example demonstrates how to use the Bybit client for public market data.
//!
//! Run with: cargo run --example bybit_public

use arb_poc::exchange::{CandleInterval, MarketData};
use arb_poc::exchanges::BybitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    // Create an unauthenticated client for public API
    let client = BybitClient::new()?;

    println!("=== Bybit Public API Example ===\n");

    // 1. Fetch ticker for BTC/USDT
    println!("--- Ticker ---");
    let tickers = client.get_ticker(&["USDT-BTC"]).await?;
    for ticker in &tickers {
        println!("Market: {}", ticker.market);
        println!("Price: {}", ticker.trade_price);
        println!("24h High: {}", ticker.high_price);
        println!("24h Low: {}", ticker.low_price);
        println!("24h Volume: {}", ticker.acc_trade_volume_24h);
        println!(
            "24h Change: {:.2}%",
            ticker.change_rate * rust_decimal::Decimal::from(100)
        );
        println!();
    }

    // 2. Fetch multiple tickers
    println!("--- Multiple Tickers ---");
    let markets = ["USDT-BTC", "USDT-ETH", "USDT-SOL"];
    let tickers = client.get_ticker(&markets).await?;
    for ticker in &tickers {
        println!(
            "{}: {} ({:+.2}%)",
            ticker.market,
            ticker.trade_price,
            ticker.change_rate * rust_decimal::Decimal::from(100)
        );
    }
    println!();

    // 3. Fetch order book
    println!("--- Order Book (BTC/USDT, depth=10) ---");
    let orderbook = client.get_orderbook("USDT-BTC", Some(10)).await?;
    println!("Market: {}", orderbook.market);
    println!("\nAsks (Sell orders):");
    for (i, ask) in orderbook.asks.iter().take(5).enumerate() {
        println!("  {}: {} @ {}", i + 1, ask.size, ask.price);
    }
    println!("\nBids (Buy orders):");
    for (i, bid) in orderbook.bids.iter().take(5).enumerate() {
        println!("  {}: {} @ {}", i + 1, bid.size, bid.price);
    }
    if let (Some(ask), Some(bid)) = (orderbook.best_ask(), orderbook.best_bid()) {
        println!("\nBest Ask: {}", ask.price);
        println!("Best Bid: {}", bid.price);
        println!(
            "Spread: {} ({:.4}%)",
            orderbook.spread().unwrap_or_default(),
            orderbook.spread_percentage().unwrap_or_default()
        );
    }
    println!();

    // 4. Fetch candles
    println!("--- Candles (BTC/USDT, 1h, last 5) ---");
    let candles = client
        .get_candles("USDT-BTC", CandleInterval::Minute60, 5)
        .await?;
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
    println!();

    // 5. Test with linear perpetuals
    println!("--- Linear Perpetual Ticker (BTCUSDT) ---");
    let perp_client = BybitClient::new()?.with_category("linear");
    let tickers = perp_client.get_ticker(&["USDT-BTC"]).await?;
    for ticker in &tickers {
        println!(
            "{}: {} ({:+.2}%)",
            ticker.market,
            ticker.trade_price,
            ticker.change_rate * rust_decimal::Decimal::from(100)
        );
    }

    println!("\n=== Example Complete ===");

    Ok(())
}
