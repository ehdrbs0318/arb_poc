//! Bybit 공개 API 예제.
//!
//! 이 예제는 Bybit 클라이언트를 사용하여 공개 시장 데이터를 조회하는 방법을 보여줍니다.
//!
//! 실행 방법: cargo run --example bybit_public

use arb_poc::exchange::{CandleInterval, MarketData};
use arb_poc::exchanges::BybitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 초기화
    tracing_subscriber::fmt().with_env_filter("info").init();

    // 공개 API용 미인증 클라이언트 생성
    let client = BybitClient::new()?;

    println!("=== Bybit Public API Example ===\n");

    // 1. BTC/USDT 티커 조회
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

    // 2. 여러 티커 조회
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

    // 3. 호가창 조회
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

    // 4. 캔들 데이터 조회
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

    // 5. 선형 무기한 선물 테스트
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
