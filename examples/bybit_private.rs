//! Bybit 비공개 API 예제.
//!
//! 이 예제는 Bybit 클라이언트를 사용하여 인증이 필요한 작업을 수행하는 방법을 보여줍니다.
//!
//! 중요: 실행 전 config.toml에 API 인증 정보를 설정하세요:
//! ```toml
//! [bybit]
//! api_key = "YOUR_API_KEY"
//! secret_key = "YOUR_SECRET_KEY"
//! testnet = true  # 테스트에는 testnet을 사용하세요!
//! ```
//!
//! 또는 환경 변수를 설정하세요:
//! - BYBIT_API_KEY
//! - BYBIT_SECRET_KEY
//!
//! 실행 방법: cargo run --example bybit_private

use arb_poc::exchange::OrderManagement;
use arb_poc::exchanges::BybitClient;
use rust_decimal::Decimal;
use std::env;

// OrderRequest는 주석 처리된 주문 예제에서 사용됩니다.
#[allow(unused_imports)]
use arb_poc::exchange::OrderRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 초기화
    tracing_subscriber::fmt().with_env_filter("info").init();

    // 환경 변수 또는 설정에서 API 인증 정보 가져오기
    let api_key = env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY not set");
    let secret_key = env::var("BYBIT_SECRET_KEY").expect("BYBIT_SECRET_KEY not set");
    let use_testnet = env::var("BYBIT_TESTNET")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true); // 안전을 위해 기본값은 testnet

    println!("=== Bybit Private API Example ===\n");
    println!(
        "Using {}\n",
        if use_testnet { "TESTNET" } else { "MAINNET" }
    );

    // 인증된 클라이언트 생성
    let client = if use_testnet {
        BybitClient::with_credentials_testnet(&api_key, &secret_key)?
    } else {
        BybitClient::with_credentials(&api_key, &secret_key)?
    };

    // 1. 계정 잔고 조회
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

    // 2. 특정 통화 잔고 조회
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

    // 3. 미체결 주문 조회
    println!("--- Open Orders ---");
    let open_orders = client.get_open_orders(None).await?;
    if open_orders.is_empty() {
        println!("No open orders");
    } else {
        for order in &open_orders {
            println!(
                "Order {}: {} {:?} {:?} @ {:?}, Status: {:?}, Filled: {}/{}",
                order.id,
                order.market,
                order.side,
                order.order_type,
                order.price,
                order.status,
                order.executed_volume,
                order.volume
            );
        }
    }
    println!();

    // 4. 테스트 지정가 주문 생성 (안전을 위해 주석 처리됨)
    // 경고: 실제 주문을 하려면 주석을 해제하세요!
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

            // 주문 취소
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
