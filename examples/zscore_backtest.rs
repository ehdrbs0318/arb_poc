//! Z-Score 차익거래 전략 히스토리컬 백테스트 예제.
//!
//! Upbit(KRW 현물) + Bybit(USDT 무기한 선물) 간
//! Z-Score 기반 mean-reversion 차익거래 전략을 과거 데이터로 시뮬레이션합니다.
//!
//! ## 실행 방법
//!
//! ```bash
//! # 기본 설정으로 실행
//! cargo run --example zscore_backtest
//!
//! # 커스텀 설정 파일 지정
//! STRATEGY_CONFIG=strategy.toml cargo run --example zscore_backtest
//! ```
//!
//! ## 사전 준비
//!
//! - 인터넷 연결 (거래소 공개 API 호출)
//! - (선택) `strategy.toml` 파일 생성 (`strategy.example.toml` 참조)

use arb_poc::exchange::MarketData;
use arb_poc::exchanges::{BybitClient, UpbitClient};
use arb_poc::strategy::output::{console, csv};
use arb_poc::strategy::zscore::config::ZScoreConfig;
use arb_poc::strategy::zscore::simulator::BacktestSimulator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    println!("=== Z-Score 백테스트 ===\n");

    // 설정 로드: 환경변수 STRATEGY_CONFIG 또는 기본 strategy.toml
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
        "총 자본: {} USDT | 백테스트 기간: {}분\n",
        config.total_capital_usdt, config.backtest_period_minutes
    );

    // 거래소 클라이언트 생성 (공개 API만 사용)
    let upbit = UpbitClient::new()?;
    let bybit = BybitClient::new()?;

    println!("Upbit: {} | Bybit: {}", upbit.name(), bybit.name());
    println!("데이터 수집 시작...\n");

    // 백테스트 실행
    let simulator = BacktestSimulator::new(upbit, bybit, config);
    let result = simulator.run().await?;

    // 결과 출력
    console::print_backtest_summary(&result);

    // CSV 저장
    if !result.trades.is_empty() {
        let trades_path = csv::write_trades_csv(&result.config.output_dir, &result.trades)?;
        println!("\n거래 내역 CSV: {trades_path}");
    }

    console::print_trade_detail(&result);

    println!("\n=== 백테스트 완료 ===");
    Ok(())
}
