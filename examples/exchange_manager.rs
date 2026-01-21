//! 통합 ExchangeManager API를 보여주는 예제.
//!
//! 이 예제에서 다루는 내용:
//! - ExchangeManager 생성
//! - 여러 거래소 등록
//! - 런타임에 거래소를 동적으로 사용
//! - 거래소 간 마켓 코드 변환
//!
//! 실행 방법: `cargo run --example exchange_manager`

use arb_poc::config::Config;
use arb_poc::exchange::{
    create_exchange, ExchangeManager, ExchangeManagerExt, ExchangeName,
    MarketCodeBuilder,
};
use arb_poc::exchange::market::convert_market_code;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 초기화
    tracing_subscriber::fmt::init();

    println!("=== Exchange Manager Example ===\n");

    // ==================== 방법 1: 수동 등록 ====================
    println!("--- Method 1: Manual Registration ---");

    let mut manager = ExchangeManager::new();

    // 인증 없이 거래소 등록 (공개 API만 사용)
    // Arc와 함께 팩토리 직접 사용
    manager.register_arc("upbit", create_exchange("upbit", None)?);
    manager.register_arc("bithumb", create_exchange("bithumb", None)?);
    manager.register_arc("bybit", create_exchange("bybit", None)?);

    println!("Registered exchanges: {:?}", manager.list_exchanges());
    println!("Total: {} exchanges\n", manager.len());

    // ==================== 방법 2: 설정 파일에서 로드 ====================
    println!("--- Method 2: From Config ---");

    let config = Config::load_or_default();
    let mut config_manager = ExchangeManager::with_capacity(3);

    // 설정에 인증 정보가 있으면 사용
    if let Err(e) = config_manager.register_all_from_config(&config) {
        println!("Warning: Could not register all exchanges: {}", e);
    }
    println!("Registered from config: {:?}", config_manager.list_exchanges());

    // ==================== 거래소 동적 사용 ====================
    println!("\n--- Fetching Tickers from All Exchanges ---");

    // 한국 거래소는 KRW-BTC 형식 사용
    let krw_market = "KRW-BTC";
    // Bybit은 내부적으로 USDT-BTC 형식 사용 (API에서는 BTCUSDT로 변환됨)
    let usdt_market = "USDT-BTC";

    for name in manager.list_exchanges() {
        let exchange = manager.get(name).unwrap();

        // 거래소의 기본 통화에 따라 적절한 마켓 선택
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

    // ==================== 거래소 필터링 ====================
    println!("\n--- Filtering by Quote Currency ---");

    let krw_exchanges: Vec<_> = manager.by_quote_currency("KRW").collect();
    println!("KRW exchanges: {:?}", krw_exchanges.iter().map(|(n, _)| *n).collect::<Vec<_>>());

    let usdt_exchanges: Vec<_> = manager.by_quote_currency("USDT").collect();
    println!("USDT exchanges: {:?}", usdt_exchanges.iter().map(|(n, _)| *n).collect::<Vec<_>>());

    // ==================== 마켓 코드 변환 ====================
    println!("\n--- Market Code Conversion ---");

    // Upbit 형식에서 Bybit 형식으로 변환
    let upbit_market = "KRW-BTC";
    let bybit_market = convert_market_code(ExchangeName::Upbit, ExchangeName::Bybit, upbit_market);
    println!("Upbit '{}' -> Bybit '{}'", upbit_market, bybit_market);

    // Bybit 형식에서 내부 형식으로 변환
    let bybit_symbol = "BTCUSDT";
    let internal = arb_poc::exchange::market::to_internal_format(ExchangeName::Bybit, bybit_symbol);
    println!("Bybit '{}' -> Internal '{}'", bybit_symbol, internal);

    // MarketCodeBuilder 사용
    let market = MarketCodeBuilder::new("ETH")
        .quote("USDT")
        .build_for(ExchangeName::Bybit);
    println!("MarketCodeBuilder for ETH/USDT on Bybit: {}", market);

    // ==================== 인증 상태 확인 ====================
    println!("\n--- Authentication Status ---");

    for (name, exchange) in manager.iter() {
        let auth_status = if exchange.is_authenticated() {
            "authenticated"
        } else {
            "public only"
        };
        println!("[{}] {} - {}", name, exchange.name(), auth_status);
    }

    // ==================== 호가창 조회 ====================
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
