//! Z-Score 파라미터 sweep 백테스트.
//!
//! entry_z 임계값을 여러 값으로 비교하여 최적 파라미터를 탐색합니다.
//! 캔들 데이터는 1회만 수집하여 모든 파라미터 조합에 재활용합니다.
//!
//! ## 실행 방법
//!
//! ```bash
//! cargo run --example zscore_parameter_sweep
//! STRATEGY_CONFIG=strategy.toml cargo run --example zscore_parameter_sweep
//! ```
//!
//! ## 사전 준비
//!
//! - 인터넷 연결 (거래소 공개 API 호출)
//! - (선택) `strategy.toml` 파일 생성 (`strategy.example.toml` 참조)
//! - (선택) `[sweep]` 섹션에 entry_z_values, exit_z_values 설정

use arb_poc::exchange::MarketData;
use arb_poc::exchanges::{BybitClient, UpbitClient};
use arb_poc::strategy::output::{console, csv};
use arb_poc::strategy::zscore::config::ZScoreConfig;
use arb_poc::strategy::zscore::sweep::{SweepConfig, run_sweep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    println!("=== Z-Score 파라미터 Sweep ===\n");

    // 설정 로드
    let config_path = std::env::var("STRATEGY_CONFIG").unwrap_or_else(|_| "strategy.toml".into());

    let (config, raw_sweep) = if std::path::Path::new(&config_path).exists() {
        println!("설정 파일 로드: {config_path}");
        let content = std::fs::read_to_string(&config_path)?;
        ZScoreConfig::from_toml_str_with_sweep(&content)?
    } else {
        println!("설정 파일 없음 — 기본값 사용");
        (ZScoreConfig::default(), None)
    };

    // SweepConfig 구성
    let sweep_config = if let Some(raw) = raw_sweep {
        raw.into_sweep_config(config)
    } else {
        // 기본 sweep 값
        SweepConfig {
            base_config: config,
            entry_z_values: vec![1.0, 1.25, 1.5, 1.75, 2.0, 2.5],
            exit_z_values: vec![], // base_config.exit_z 사용
            max_combinations: 50,
        }
    };

    println!("코인: {:?}", sweep_config.base_config.coins);
    println!("entry_z 값: {:?}", sweep_config.entry_z_values);
    println!(
        "exit_z 값: {:?}",
        if sweep_config.exit_z_values.is_empty() {
            vec![sweep_config.base_config.exit_z_threshold]
        } else {
            sweep_config.exit_z_values.clone()
        }
    );
    println!(
        "총 자본: {} USDT\n",
        sweep_config.base_config.total_capital_usdt
    );

    // 거래소 클라이언트 생성 (공개 API만 사용)
    let upbit = UpbitClient::new()?;
    let bybit = BybitClient::new()?;

    println!("Upbit: {} | Bybit: {}", upbit.name(), bybit.name());
    println!("데이터 수집 시작...\n");

    // Sweep 실행
    let result = run_sweep(&upbit, &bybit, &sweep_config).await?;

    // 콘솔 비교 테이블 출력
    console::print_sweep_summary(&result);

    // Sweep CSV 저장
    let csv_path = csv::write_sweep_csv(&sweep_config.base_config.output_dir, &result)?;
    println!("\nSweep CSV: {csv_path}");

    println!("\n=== Sweep 완료 ===");
    Ok(())
}
