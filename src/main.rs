//! arb_poc - 암호화폐 차익거래 개념 증명(PoC)
//!
//! 차익거래 시스템의 메인 진입점입니다.

use arb_poc::config::Config;
use arb_poc::exchange::MarketData;
use arb_poc::exchanges::UpbitClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tracing::info!("Starting arb_poc...");

    // 설정 로드
    let config = Config::load_or_default();

    // Upbit 클라이언트 생성
    let client = if config.upbit.has_credentials() {
        tracing::info!("Creating authenticated Upbit client");
        UpbitClient::with_credentials(&config.upbit.api_key, &config.upbit.secret_key)?
    } else {
        tracing::info!("Creating unauthenticated Upbit client (public API only)");
        UpbitClient::new()?
    };

    // 시세 조회 및 표시
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
