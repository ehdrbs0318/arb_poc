//! Bithumb private API example.
//!
//! This example demonstrates how to use the Bithumb client to access
//! private API endpoints with authentication.
//!
//! # Configuration
//!
//! Set the following environment variables before running:
//! - BITHUMB_API_KEY: Your Bithumb API access key
//! - BITHUMB_SECRET_KEY: Your Bithumb API secret key
//!
//! Or create a config.toml file with your credentials.
//!
//! # Run
//!
//! ```bash
//! BITHUMB_API_KEY=your_key BITHUMB_SECRET_KEY=your_secret cargo run --example bithumb_private
//! ```

use arb_poc::config::Config;
use arb_poc::exchange::OrderManagement;
use arb_poc::exchanges::BithumbClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::load_or_default();

    if !config.bithumb.has_credentials() {
        println!("Error: Bithumb credentials not configured.");
        println!("Please set BITHUMB_API_KEY and BITHUMB_SECRET_KEY environment variables,");
        println!("or create a config.toml file with your credentials.");
        return Ok(());
    }

    // Create an authenticated client
    let client =
        BithumbClient::with_credentials(&config.bithumb.api_key, &config.bithumb.secret_key)?;
    println!("Created authenticated Bithumb client");

    // Fetch account balances
    println!("\n--- Account Balances ---");
    match client.get_balances().await {
        Ok(balances) => {
            for balance in &balances {
                if balance.total() > rust_decimal::Decimal::ZERO {
                    println!(
                        "{}: Available={}, Locked={}, Total={}",
                        balance.currency,
                        balance.balance,
                        balance.locked,
                        balance.total()
                    );
                }
            }
        }
        Err(e) => println!("Error fetching balances: {}", e),
    }

    // Fetch specific currency balance
    println!("\n--- KRW Balance ---");
    match client.get_balance("KRW").await {
        Ok(balance) => {
            println!(
                "KRW: Available={}, Locked={}, Total={}",
                balance.balance,
                balance.locked,
                balance.total()
            );
        }
        Err(e) => println!("Error fetching KRW balance: {}", e),
    }

    // Fetch open orders
    println!("\n--- Open Orders (KRW-BTC) ---");
    match client.get_open_orders(Some("KRW-BTC")).await {
        Ok(orders) => {
            if orders.is_empty() {
                println!("No open orders");
            } else {
                for order in &orders {
                    println!(
                        "Order {}: {:?} {:?} {} @ {:?}",
                        order.id, order.side, order.order_type, order.volume, order.price
                    );
                }
            }
        }
        Err(e) => println!("Error fetching open orders: {}", e),
    }

    // Example: Place and cancel an order (commented out for safety)
    /*
    use arb_poc::exchange::OrderRequest;
    use rust_decimal::Decimal;

    // Place a limit buy order
    println!("\n--- Place Order ---");
    let order_request = OrderRequest::limit_buy(
        "KRW-BTC",
        Decimal::from(10000000),  // Very low price to avoid execution
        Decimal::new(1, 4),       // 0.0001 BTC
    );

    match client.place_order(&order_request).await {
        Ok(order) => {
            println!("Order placed: {}", order.id);
            println!("Status: {:?}", order.status);

            // Cancel the order
            println!("\n--- Cancel Order ---");
            match client.cancel_order(&order.id).await {
                Ok(cancelled) => {
                    println!("Order cancelled: {}", cancelled.id);
                    println!("Status: {:?}", cancelled.status);
                }
                Err(e) => println!("Error cancelling order: {}", e),
            }
        }
        Err(e) => println!("Error placing order: {}", e),
    }
    */

    Ok(())
}
