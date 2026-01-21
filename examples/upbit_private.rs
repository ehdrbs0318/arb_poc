//! # Upbit 비공개 API 예제
//!
//! 이 예제는 Upbit 비공개(거래) API를 사용하여
//! 주문을 관리하고 계정 잔고를 확인하는 방법을 보여줍니다.
//!
//! ## 사전 요구사항
//!
//! 1. 프로젝트 루트에 `config.toml` 파일을 생성하세요:
//!
//! ```toml
//! [upbit]
//! api_key = "your-api-key"
//! secret_key = "your-secret-key"
//! ```
//!
//! 또는 환경 변수를 설정하세요:
//! - `UPBIT_API_KEY`
//! - `UPBIT_SECRET_KEY`
//!
//! ## 예제 실행 방법
//!
//! ```bash
//! cargo run --example upbit_private
//! ```
//!
//! **경고**: 이 예제는 실제 계정에 주문을 넣을 수 있습니다.
//! 실행 전에 코드를 검토하고 주의해서 사용하세요.

use arb_poc::config::Config;
use arb_poc::exchange::{MarketData, OrderManagement};
use arb_poc::exchanges::UpbitClient;
use rust_decimal::Decimal;

// OrderRequest는 아래 주석 처리된 주문 예제에서 사용됩니다
#[allow(unused_imports)]
use arb_poc::exchange::OrderRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Upbit Private API Example ===\n");

    // 설정 로드
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

    // 인증된 클라이언트 생성
    let client = UpbitClient::with_credentials(&config.upbit.api_key, &config.upbit.secret_key)?;

    println!("Exchange: {}", client.name());
    println!("Authentication: Enabled\n");

    // 1. 계정 잔고 조회
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

    // 2. 미체결 주문 조회
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

    // 3. 예제: 지정가 주문 넣기 (안전을 위해 주석 처리됨)
    // 주문 기능을 테스트하려면 아래 코드의 주석을 해제하세요.
    // 계정에 적절한 가격과 수량을 사용하세요.
    /*
    println!("--- Place Order Example ---");

    // 먼저 현재 가격 조회
    let tickers = client.get_ticker(&["KRW-BTC"]).await?;
    let current_price = tickers.first().map(|t| t.trade_price).unwrap_or_default();

    // 현재 가격의 90%에 지정가 매수 주문 (체결 가능성 낮음)
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

            // 주문 취소
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
