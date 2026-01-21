//! Example demonstrating the unified ExchangeManager API.
//!
//! This example shows how to:
//! - Create an ExchangeManager
//! - Register multiple exchanges
//! - Use exchanges dynamically at runtime
//! - Convert market codes between exchanges
//!
//! Run with: `cargo run --example exchange_manager`

use arb_poc::config::Config;
use arb_poc::exchange::{
    create_exchange, ExchangeManager, ExchangeManagerExt, ExchangeName,
    MarketCodeBuilder,
};
use arb_poc::exchange::market::convert_market_code;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== Exchange Manager Example ===\n");

    // ==================== Method 1: Manual Registration ====================
    println!("--- Method 1: Manual Registration ---");

    let mut manager = ExchangeManager::new();

    // Register exchanges without authentication (public API only)
    // Using the factory directly with Arc
    manager.register_arc("upbit", create_exchange("upbit", None)?);
    manager.register_arc("bithumb", create_exchange("bithumb", None)?);
    manager.register_arc("bybit", create_exchange("bybit", None)?);

    println!("Registered exchanges: {:?}", manager.list_exchanges());
    println!("Total: {} exchanges\n", manager.len());

    // ==================== Method 2: From Config ====================
    println!("--- Method 2: From Config ---");

    let config = Config::load_or_default();
    let mut config_manager = ExchangeManager::with_capacity(3);

    // This will use credentials from config if available
    if let Err(e) = config_manager.register_all_from_config(&config) {
        println!("Warning: Could not register all exchanges: {}", e);
    }
    println!("Registered from config: {:?}", config_manager.list_exchanges());

    // ==================== Using Exchanges Dynamically ====================
    println!("\n--- Fetching Tickers from All Exchanges ---");

    // Korean exchanges use KRW-BTC format
    let krw_market = "KRW-BTC";
    // Bybit uses USDT-BTC format internally (converted to BTCUSDT for API)
    let usdt_market = "USDT-BTC";

    for name in manager.list_exchanges() {
        let exchange = manager.get(name).unwrap();

        // Select appropriate market based on exchange's native currency
        let market = if exchange.native_quote_currency() == "KRW" {
            krw_market
        } else {
            usdt_market
        };

        match exchange.get_ticker(&[market]).await {
            Ok(tickers) => {
                if let Some(ticker) = tickers.first() {
                    println!(
                        "[{}] {}: {} {} (24h change: {:.2}%)",
                        name,
                        ticker.market,
                        ticker.trade_price,
                        exchange.native_quote_currency(),
                        ticker.change_rate * rust_decimal::Decimal::from(100)
                    );
                }
            }
            Err(e) => {
                println!("[{}] Error fetching ticker: {}", name, e);
            }
        }
    }

    // ==================== Filtering Exchanges ====================
    println!("\n--- Filtering by Quote Currency ---");

    let krw_exchanges: Vec<_> = manager.by_quote_currency("KRW").collect();
    println!("KRW exchanges: {:?}", krw_exchanges.iter().map(|(n, _)| *n).collect::<Vec<_>>());

    let usdt_exchanges: Vec<_> = manager.by_quote_currency("USDT").collect();
    println!("USDT exchanges: {:?}", usdt_exchanges.iter().map(|(n, _)| *n).collect::<Vec<_>>());

    // ==================== Market Code Conversion ====================
    println!("\n--- Market Code Conversion ---");

    // Convert from Upbit format to Bybit format
    let upbit_market = "KRW-BTC";
    let bybit_market = convert_market_code(ExchangeName::Upbit, ExchangeName::Bybit, upbit_market);
    println!("Upbit '{}' -> Bybit '{}'", upbit_market, bybit_market);

    // Convert from Bybit format to internal format
    let bybit_symbol = "BTCUSDT";
    let internal = arb_poc::exchange::market::to_internal_format(ExchangeName::Bybit, bybit_symbol);
    println!("Bybit '{}' -> Internal '{}'", bybit_symbol, internal);

    // Using MarketCodeBuilder
    let market = MarketCodeBuilder::new("ETH")
        .quote("USDT")
        .build_for(ExchangeName::Bybit);
    println!("MarketCodeBuilder for ETH/USDT on Bybit: {}", market);

    // ==================== Checking Authentication Status ====================
    println!("\n--- Authentication Status ---");

    for (name, exchange) in manager.iter() {
        let auth_status = if exchange.is_authenticated() {
            "authenticated"
        } else {
            "public only"
        };
        println!("[{}] {} - {}", name, exchange.name(), auth_status);
    }

    // ==================== Fetching Order Book ====================
    println!("\n--- Order Book Example (Upbit) ---");

    if let Some(upbit) = manager.get("upbit") {
        match upbit.get_orderbook("KRW-BTC", Some(5)).await {
            Ok(orderbook) => {
                println!("Market: {}", orderbook.market);
                if let Some(best_ask) = orderbook.best_ask() {
                    println!("Best Ask: {} @ {}", best_ask.size, best_ask.price);
                }
                if let Some(best_bid) = orderbook.best_bid() {
                    println!("Best Bid: {} @ {}", best_bid.size, best_bid.price);
                }
                if let Some(spread) = orderbook.spread() {
                    println!("Spread: {}", spread);
                }
            }
            Err(e) => {
                println!("Error fetching orderbook: {}", e);
            }
        }
    }

    println!("\n=== Example Complete ===");

    Ok(())
}
