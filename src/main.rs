//! arb_poc - Cryptocurrency Arbitrage Proof of Concept
//!
//! This is the main entry point for the arbitrage trading system.

use arb_poc::config::Config;
use arb_poc::exchange::MarketData;
use arb_poc::exchanges::UpbitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tracing::info!("Starting arb_poc...");

    // Load configuration
    let config = Config::load_or_default();

    // Create Upbit client
    let client = if config.upbit.has_credentials() {
        tracing::info!("Creating authenticated Upbit client");
        UpbitClient::with_credentials(&config.upbit.api_key, &config.upbit.secret_key)?
    } else {
        tracing::info!("Creating unauthenticated Upbit client (public API only)");
        UpbitClient::new()?
    };

    // Fetch and display ticker
    tracing::info!("Fetching BTC ticker from Upbit...");
    match client.get_ticker(&["KRW-BTC"]).await {
        Ok(tickers) => {
            if let Some(ticker) = tickers.first() {
                tracing::info!(
                    "KRW-BTC: {} KRW ({})",
                    ticker.trade_price,
                    match ticker.change {
                        arb_poc::exchange::PriceChange::Rise => "RISE",
                        arb_poc::exchange::PriceChange::Fall => "FALL",
                        arb_poc::exchange::PriceChange::Even => "EVEN",
                    }
                );
            }
        }
        Err(e) => {
            tracing::error!("Failed to fetch ticker: {}", e);
        }
    }

    Ok(())
}
