//! Bithumb 비공개 API 예제.
//!
//! 이 예제는 인증을 사용하여 Bithumb 비공개 API 엔드포인트에
//! 접근하는 방법을 보여줍니다.
//!
//! # 설정
//!
//! 실행 전 다음 환경 변수를 설정하세요:
//! - BITHUMB_API_KEY: Bithumb API 액세스 키
//! - BITHUMB_SECRET_KEY: Bithumb API 시크릿 키
//!
//! 또는 자격 증명이 포함된 config.toml 파일을 생성하세요.
//!
//! # 실행
//!
//! ```bash
//! BITHUMB_API_KEY=your_key BITHUMB_SECRET_KEY=your_secret cargo run --example bithumb_private
//! ```

use arb_poc::config::Config;
use arb_poc::exchange::OrderManagement;
use arb_poc::exchanges::BithumbClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅을 위한 tracing 초기화
    tracing_subscriber::fmt::init();

    // 설정 로드
    let config = Config::load_or_default();

    if !config.bithumb.has_credentials() {
        println!("Error: Bithumb credentials not configured.");
        println!("Please set BITHUMB_API_KEY and BITHUMB_SECRET_KEY environment variables,");
        println!("or create a config.toml file with your credentials.");
        return Ok(());
    }

    // 인증된 클라이언트 생성
    let client =
        BithumbClient::with_credentials(&config.bithumb.api_key, &config.bithumb.secret_key)?;
    println!("Created authenticated Bithumb client");

    // 계정 잔고 조회
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

    // 특정 통화 잔고 조회
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

    // 미체결 주문 조회
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

    // 예제: 주문 생성 및 취소 (안전을 위해 주석 처리됨)
    /*
    use arb_poc::exchange::OrderRequest;
    use rust_decimal::Decimal;

    // 지정가 매수 주문 생성
    println!("\n--- Place Order ---");
    let order_request = OrderRequest::limit_buy(
        "KRW-BTC",
        Decimal::from(10000000),  // 체결 방지를 위한 매우 낮은 가격
        Decimal::new(1, 4),       // 0.0001 BTC
    );

    match client.place_order(&order_request).await {
        Ok(order) => {
            println!("Order placed: {}", order.id);
            println!("Status: {:?}", order.status);

            // 주문 취소
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
