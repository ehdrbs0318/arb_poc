//! Bybit private API example.
//!
//! This example demonstrates how to use the Bybit client for authenticated operations.
//!
//! IMPORTANT: Set your API credentials in config.toml before running:
//! ```toml
//! [bybit]
//! api_key = "YOUR_API_KEY"
//! secret_key = "YOUR_SECRET_KEY"
//! testnet = true  # Use testnet for testing!
//! ```
//!
//! Or set environment variables:
//! - BYBIT_API_KEY
//! - BYBIT_SECRET_KEY
//!
//! Run with: cargo run --example bybit_private

use arb_poc::exchange::{OrderManagement, OrderRequest};
use arb_poc::exchanges::BybitClient;
use rust_decimal::Decimal;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    // Get API credentials from environment or config
    let api_key = env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY not set");
    let secret_key = env::var("BYBIT_SECRET_KEY").expect("BYBIT_SECRET_KEY not set");
    let use_testnet = env::var("BYBIT_TESTNET")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true); // Default to testnet for safety

    println!("=== Bybit Private API Example ===\n");
    println!(
        "Using {}\n",
        if use_testnet { "TESTNET" } else { "MAINNET" }
    );

    // Create authenticated client
    let client = if use_testnet {
        BybitClient::with_credentials_testnet(&api_key, &secret_key)?
    } else {
        BybitClient::with_credentials(&api_key, &secret_key)?
    };

    // 1. Get account balances
    println!("--- Account Balances ---");
    let balances = client.get_balances().await?;
    for balance in &balances {
        if balance.balance > Decimal::ZERO || balance.locked > Decimal::ZERO {
            println!(
                "{}: Available={}, Locked={}, Total={}",
                balance.currency,
                balance.balance,
                balance.locked,
                balance.total()
            );
        }
    }
    println!();

    // 2. Get specific balance
    println!("--- USDT Balance ---");
    match client.get_balance("USDT").await {
        Ok(balance) => {
            println!(
                "USDT: Available={}, Locked={}, Total={}",
                balance.balance,
                balance.locked,
                balance.total()
            );
        }
        Err(e) => {
            println!("Could not get USDT balance: {}", e);
        }
    }
    println!();

    // 3. Get open orders
    println!("--- Open Orders ---");
    let open_orders = client.get_open_orders(None).await?;
    if open_orders.is_empty() {
        println!("No open orders");
    } else {
        for order in &open_orders {
            println!(
                "Order {}: {} {} {} @ {:?}, Status: {:?}, Filled: {}/{}",
                order.id,
                order.market,
                format!("{:?}", order.side),
                format!("{:?}", order.order_type),
                order.price,
                order.status,
                order.executed_volume,
                order.volume
            );
        }
    }
    println!();

    // 4. Place a test limit order (commented out for safety)
    // WARNING: Uncomment only if you want to place a real order!
    /*
    println!("--- Place Limit Order ---");
    let order_request = OrderRequest::limit_buy(
        "USDT-BTC",
        Decimal::new(20000, 0),  // Price: 20,000 USDT (intentionally low)
        Decimal::new(1, 4),       // Volume: 0.0001 BTC
    );

    match client.place_order(&order_request).await {
        Ok(order) => {
            println!("Order placed successfully!");
            println!("Order ID: {}", order.id);
            println!("Status: {:?}", order.status);

            // Cancel the order
            println!("\n--- Cancel Order ---");
            match client.cancel_order(&order.id).await {
                Ok(cancelled) => {
                    println!("Order cancelled: {:?}", cancelled.status);
                }
                Err(e) => {
                    println!("Failed to cancel order: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to place order: {}", e);
        }
    }
    */

    println!("\n=== Example Complete ===");
    println!("\nNote: Order placement is commented out for safety.");
    println!("Uncomment the code to test order operations.");

    Ok(())
}
