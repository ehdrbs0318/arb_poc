//! Z-Score 실시간 모니터링 예제.
//!
//! Upbit/Bybit WebSocket 스트림에서 실시간으로 Z-Score 기반
//! 차익거래 시그널을 감지합니다.
//!
//! ## 실행 방법
//!
//! ```bash
//! # 기본 설정으로 실행
//! cargo run --example zscore_monitor
//!
//! # 커스텀 설정 파일 지정
//! STRATEGY_CONFIG=strategy.toml cargo run --example zscore_monitor
//!
//! # 디버그 로그 활성화
//! RUST_LOG=debug cargo run --example zscore_monitor
//! ```
//!
//! ## 사전 준비
//!
//! - 인터넷 연결 (WebSocket + REST API)
//! - (선택) `strategy.toml` 파일 생성 (`strategy.example.toml` 참조)
//!
//! ## 종료
//!
//! `Ctrl+C`로 graceful shutdown합니다.

use std::sync::Arc;
use std::time::Duration;

use arb_poc::exchange::MarketData;
use arb_poc::exchanges::{BybitClient, UpbitClient};
use arb_poc::forex::ForexCache;
use arb_poc::strategy::zscore::config::ZScoreConfig;
use arb_poc::strategy::zscore::monitor::ZScoreMonitor;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    println!("=== Z-Score 실시간 모니터 ===\n");

    // 설정 로드
    let config_path = std::env::var("STRATEGY_CONFIG").unwrap_or_else(|_| "strategy.toml".into());
    let config = if std::path::Path::new(&config_path).exists() {
        println!("설정 파일 로드: {config_path}");
        let cfg = ZScoreConfig::from_file(&config_path)?;
        cfg.validate()?;
        cfg
    } else {
        println!("설정 파일 없음 — 기본값 사용 (strategy.example.toml 참조)");
        ZScoreConfig::default()
    };

    println!("코인: {:?}", config.coins);
    println!(
        "윈도우: {} | 진입 Z: {} | 청산 Z: {}",
        config.window_size, config.entry_z_threshold, config.exit_z_threshold
    );
    println!(
        "총 자본: {} USDT | Bybit 카테고리: linear\n",
        config.total_capital_usdt
    );

    // 거래소 클라이언트 생성
    let upbit = UpbitClient::new()?;
    let bybit = BybitClient::new()?.with_category("linear");

    println!("Upbit: {} | Bybit: {}", upbit.name(), bybit.name());
    println!("워밍업 데이터 수집 + WebSocket 연결 시작...\n");

    // CancellationToken 생성
    let cancel_token = CancellationToken::new();

    // Ctrl+C 시그널 핸들러
    let cancel_clone = cancel_token.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        println!("\nCtrl+C 감지 — graceful shutdown 시작...");
        cancel_clone.cancel();
    });

    // ForexCache 생성 (TTL 10분)
    let forex_cache = Arc::new(ForexCache::new(Duration::from_secs(600)));

    // 실시간 모니터링 실행
    let monitor = ZScoreMonitor::new(upbit, bybit, config, forex_cache);
    let trades = monitor.run(cancel_token).await?;

    // 결과 출력
    println!("\n=== 모니터링 결과 ===");
    println!("총 거래: {} 건", trades.len());

    if !trades.is_empty() {
        let winning = trades
            .iter()
            .filter(|t| t.net_pnl > rust_decimal::Decimal::ZERO)
            .count();
        let net_pnl: rust_decimal::Decimal = trades.iter().map(|t| t.net_pnl).sum();

        println!("승리: {} 건 | 패배: {} 건", winning, trades.len() - winning);
        println!("순 PnL: {} USDT", net_pnl);
    }

    println!("\n=== 모니터링 종료 ===");
    Ok(())
}
