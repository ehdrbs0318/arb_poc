//! # Upbit Private API Example
//!
//! This example demonstrates how to use the Upbit private (exchange) API
//! to manage orders and check account balances.
//!
//! ## Prerequisites
//!
//! 1. Create a `config.toml` file in the project root:
//!
//! ```toml
//! [upbit]
//! api_key = "your-api-key"
//! secret_key = "your-secret-key"
//! ```
//!
//! Or set environment variables:
//! - `UPBIT_API_KEY`
//! - `UPBIT_SECRET_KEY`
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example upbit_private
//! ```
//!
//! **WARNING**: This example may place real orders on your account.
//! Use with caution and review the code before running.

use arb_poc::config::Config;
use arb_poc::exchange::{MarketData, OrderManagement};
use arb_poc::exchanges::UpbitClient;
use rust_decimal::Decimal;

// OrderRequest is used in the commented-out order placement example
#[allow(unused_imports)]
use arb_poc::exchange::OrderRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Upbit Private API Example ===\n");

    // Load configuration
    let config = Config::load()?;

    if !config.upbit.has_credentials() {
        eprintln!("Error: Upbit credentials not configured.");
        eprintln!("Please set up config.toml or environment variables.");
        eprintln!("\nExample config.toml:");
        eprintln!("[upbit]");
        eprintln!("api_key = \"your-api-key\"");
        eprintln!("secret_key = \"your-secret-key\"");
        return Ok(());
    }

    // Create an authenticated client
    let client = UpbitClient::with_credentials(&config.upbit.api_key, &config.upbit.secret_key)?;

    println!("Exchange: {}", client.name());
    println!("Authentication: Enabled\n");

    // 1. Fetch account balances
    println!("--- Account Balances ---");
    let balances = client.get_balances().await?;

    for balance in &balances {
        let total = balance.total();
        if total > Decimal::ZERO {
            println!(
                "{}: {} (available: {}, locked: {})",
                balance.currency, total, balance.balance, balance.locked
            );
            if balance.avg_buy_price > Decimal::ZERO {
                println!(
                    "  Avg Buy Price: {} {}",
                    balance.avg_buy_price, balance.unit_currency
                );
            }
        }
    }
    println!();

    // 2. Fetch open orders
    println!("--- Open Orders ---");
    let open_orders = client.get_open_orders(None).await?;

    if open_orders.is_empty() {
        println!("No open orders.");
    } else {
        for order in &open_orders {
            println!(
                "[{}] {} {:?} {} @ {:?}",
                order.id, order.market, order.side, order.volume, order.price
            );
        }
    }
    println!();

    // 3. Example: Place a limit order (COMMENTED OUT FOR SAFETY)
    // Uncomment the following code to test order placement.
    // Make sure to use appropriate price and volume for your account.
    /*
    println!("--- Place Order Example ---");

    // Get current price first
    let tickers = client.get_ticker(&["KRW-BTC"]).await?;
    let current_price = tickers.first().map(|t| t.trade_price).unwrap_or_default();

    // Place a limit buy order at 90% of current price (unlikely to fill)
    let order_price = current_price * Decimal::new(9, 1); // 90%
    let order_volume = Decimal::new(1, 4); // 0.0001 BTC

    let order_request = OrderRequest::limit_buy(
        "KRW-BTC",
        order_price,
        order_volume,
    );

    println!("Placing limit buy order:");
    println!("  Market: KRW-BTC");
    println!("  Price: {} KRW", order_price);
    println!("  Volume: {} BTC", order_volume);

    match client.place_order(&order_request).await {
        Ok(order) => {
            println!("Order placed successfully!");
            println!("  Order ID: {}", order.id);
            println!("  Status: {:?}", order.status);

            // Cancel the order
            println!("\nCancelling order...");
            match client.cancel_order(&order.id).await {
                Ok(cancelled) => {
                    println!("Order cancelled: {:?}", cancelled.status);
                }
                Err(e) => {
                    eprintln!("Failed to cancel order: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to place order: {}", e);
        }
    }
    */

    println!("\n=== Example Complete ===");
    println!("\nNote: Order placement code is commented out for safety.");
    println!("Review and uncomment the code to test order operations.");

    Ok(())
}
