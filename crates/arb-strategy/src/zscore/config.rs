//! Z-Score 전략 설정.
//!
//! TOML 파일에서 전략 설정을 로드합니다.
//! `arb-config`의 간이 파서는 중첩 섹션을 지원하지 않으므로,
//! 전략 설정은 별도 파일(`strategy.toml`)로 분리하여 자체 로딩합니다.

use std::path::{Path, PathBuf};

use arb_exchange::CandleInterval;
use rust_decimal::Decimal;
use serde::Deserialize;
use tracing::{debug, info, warn};

use crate::error::StrategyError;

/// Z-Score 기반 차익거래 전략 설정.
#[derive(Clone, Debug)]
pub struct ZScoreConfig {
    /// 대상 코인 목록 (e.g., ["BTC", "ETH", "XRP"]).
    pub coins: Vec<String>,
    /// 캔들 윈도우 크기 (기본값: 1440 = 1일치 1분봉).
    /// 가이드라인: 최적 윈도우 = 추정 half-life의 3~5배.
    pub window_size: usize,
    /// 캔들 간격 (기본값: 1분).
    pub candle_interval: CandleInterval,
    /// Z-Score 진입 임계값 (기본값: 2.0).
    pub entry_z_threshold: f64,
    /// Z-Score 청산 임계값 (기본값: 0.5).
    pub exit_z_threshold: f64,
    /// 총 자본금 (USDT 기준, 양 거래소 합산).
    pub total_capital_usdt: Decimal,
    /// 포지션당 자본금 비율 (e.g., 0.1 = 총 자본의 10%).
    /// 실제 필요 자본 = total_capital × position_ratio × 2
    pub position_ratio: Decimal,
    /// Upbit taker 수수료율 (기본값: 0.0005 = 0.05%).
    pub upbit_taker_fee: Decimal,
    /// Bybit linear taker 수수료율 (기본값: 0.00055 = 0.055%).
    pub bybit_taker_fee: Decimal,
    /// Bybit 레버리지 (기본값: 1).
    pub leverage: u32,
    /// Bybit maintenance margin rate (기본값: 0.005 = 0.5%).
    pub bybit_mmr: Decimal,
    /// 백테스트 기간 (분 단위, 워밍업 제외, 기본값: 8640 = 6일).
    pub backtest_period_minutes: usize,
    /// Z-Score 계산 시 최소 stddev 임계값 (기본값: 0.01).
    pub min_stddev_threshold: f64,
    /// 결과 CSV 출력 디렉토리 (기본값: "./output/").
    pub output_dir: PathBuf,
    /// 최대 동시 포지션 수 (None이면 coins.len()까지 허용).
    pub max_concurrent_positions: Option<usize>,
    /// 볼륨/슬리피지 모델 활성화 (기본값: false).
    pub volume_filter_enabled: bool,
    /// 최대 참여율 (캔들 거래량 대비 주문 비율, 기본값: 0.1 = 10%).
    /// 초과 시 진입을 거부합니다.
    pub max_participation_rate: f64,
    /// 기본 슬리피지 (bps, 기본값: 1.0 = 0.01%).
    pub slippage_base_bps: f64,
    /// 슬리피지 충격 계수 (기본값: 0.001).
    /// slippage_bps = base_bps + impact_coeff × √(participation_rate) × 10000
    pub slippage_impact_coeff: f64,
}

impl Default for ZScoreConfig {
    fn default() -> Self {
        Self {
            coins: vec!["BTC".to_string()],
            window_size: 1440,
            candle_interval: CandleInterval::Minute1,
            entry_z_threshold: 2.0,
            exit_z_threshold: 0.5,
            total_capital_usdt: Decimal::new(10000, 0),
            position_ratio: Decimal::new(1, 1),   // 0.1
            upbit_taker_fee: Decimal::new(5, 4),  // 0.0005
            bybit_taker_fee: Decimal::new(55, 5), // 0.00055
            leverage: 1,
            bybit_mmr: Decimal::new(5, 3), // 0.005
            backtest_period_minutes: 8640,
            min_stddev_threshold: 0.01,
            output_dir: PathBuf::from("./output/"),
            max_concurrent_positions: None,
            volume_filter_enabled: false,
            max_participation_rate: 0.1,
            slippage_base_bps: 1.0,
            slippage_impact_coeff: 0.001,
        }
    }
}

impl ZScoreConfig {
    /// 설정값 유효성 검증.
    pub fn validate(&self) -> Result<(), StrategyError> {
        if self.coins.is_empty() {
            return Err(StrategyError::Config("coins must not be empty".to_string()));
        }
        if self.window_size == 0 {
            return Err(StrategyError::Config(
                "window_size must be greater than 0".to_string(),
            ));
        }
        if self.entry_z_threshold <= 0.0 {
            return Err(StrategyError::Config(
                "entry_z_threshold must be positive".to_string(),
            ));
        }
        if self.exit_z_threshold < 0.0 {
            return Err(StrategyError::Config(
                "exit_z_threshold must be non-negative".to_string(),
            ));
        }
        if self.entry_z_threshold <= self.exit_z_threshold {
            return Err(StrategyError::Config(
                "entry_z_threshold must be greater than exit_z_threshold".to_string(),
            ));
        }
        if self.position_ratio <= Decimal::ZERO || self.position_ratio > Decimal::new(5, 1) {
            return Err(StrategyError::Config(
                "position_ratio must be in (0, 0.5]".to_string(),
            ));
        }
        if self.min_stddev_threshold <= 0.0 {
            return Err(StrategyError::Config(
                "min_stddev_threshold must be positive".to_string(),
            ));
        }

        // 다중 코인 자본 초과 경고
        let total_ratio =
            self.position_ratio * Decimal::from(self.coins.len() as u64) * Decimal::from(2u64);
        if total_ratio > Decimal::ONE {
            warn!(
                "position_ratio({}) × 코인 수({}) × 2 = {}. 모든 코인에 동시 진입하면 자본 초과.",
                self.position_ratio,
                self.coins.len(),
                total_ratio
            );
        }

        // 암호화폐 상관관계 집중 리스크 경고
        if self.coins.len() > 1 {
            warn!(
                "암호화폐 간 상관계수가 높습니다(0.7~0.9). \
                 다중 코인 동시 진입 시 실질적 분산 효과가 제한적이며, \
                 집중 리스크에 노출됩니다. max_concurrent_positions 설정을 권장합니다."
            );
        }

        // 설정 검증 통과 시 주요 파라미터 로깅
        debug!(
            coins = ?self.coins,
            window_size = self.window_size,
            entry_z = self.entry_z_threshold,
            exit_z = self.exit_z_threshold,
            total_capital = %self.total_capital_usdt,
            "설정 검증 통과"
        );

        Ok(())
    }

    /// 총 필요 데이터 수 (워밍업 + 백테스트 기간).
    pub fn total_candles_needed(&self) -> usize {
        self.window_size + self.backtest_period_minutes
    }

    /// TOML 파일에서 설정을 로드합니다.
    ///
    /// 파일 형식은 `[zscore]` 섹션 아래에 설정값을 기술합니다.
    /// 누락된 필드는 기본값이 적용됩니다.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, StrategyError> {
        let content = std::fs::read_to_string(path.as_ref())?;
        let config = Self::from_toml_str(&content)?;
        info!(path = %path.as_ref().display(), "전략 설정 파일 로드 완료");
        Ok(config)
    }

    /// TOML 문자열에서 설정을 파싱합니다.
    pub fn from_toml_str(s: &str) -> Result<Self, StrategyError> {
        let wrapper: TomlWrapper = toml::from_str(s)
            .map_err(|e| StrategyError::Config(format!("TOML parse error: {e}")))?;
        Ok(wrapper.zscore.into())
    }

    /// TOML 문자열에서 `[sweep]` 섹션도 함께 파싱합니다.
    pub fn from_toml_str_with_sweep(
        s: &str,
    ) -> Result<(Self, Option<RawSweepConfig>), StrategyError> {
        let wrapper: TomlWrapper = toml::from_str(s)
            .map_err(|e| StrategyError::Config(format!("TOML parse error: {e}")))?;
        Ok((wrapper.zscore.into(), wrapper.sweep))
    }
}

/// TOML 최상위 래퍼 (`[zscore]` 및 `[sweep]` 섹션).
#[derive(Deserialize)]
struct TomlWrapper {
    #[serde(default)]
    zscore: RawZScoreConfig,
    #[serde(default)]
    sweep: Option<RawSweepConfig>,
}

/// TOML 역직렬화 전용 중간 구조체.
///
/// `Decimal`은 TOML float에서 직접 역직렬화가 어려우므로
/// `f64`로 먼저 받은 뒤 `Decimal::try_from()`으로 변환합니다.
#[derive(Deserialize)]
#[serde(default)]
struct RawZScoreConfig {
    coins: Vec<String>,
    window_size: usize,
    entry_z_threshold: f64,
    exit_z_threshold: f64,
    total_capital_usdt: f64,
    position_ratio: f64,
    upbit_taker_fee: f64,
    bybit_taker_fee: f64,
    leverage: u32,
    bybit_mmr: f64,
    backtest_period_minutes: usize,
    min_stddev_threshold: f64,
    output_dir: String,
    max_concurrent_positions: Option<usize>,
    volume_filter_enabled: bool,
    max_participation_rate: f64,
    slippage_base_bps: f64,
    slippage_impact_coeff: f64,
}

impl Default for RawZScoreConfig {
    fn default() -> Self {
        let defaults = ZScoreConfig::default();
        Self {
            coins: defaults.coins,
            window_size: defaults.window_size,
            entry_z_threshold: defaults.entry_z_threshold,
            exit_z_threshold: defaults.exit_z_threshold,
            total_capital_usdt: 10000.0,
            position_ratio: 0.1,
            upbit_taker_fee: 0.0005,
            bybit_taker_fee: 0.00055,
            leverage: defaults.leverage,
            bybit_mmr: 0.005,
            backtest_period_minutes: defaults.backtest_period_minutes,
            min_stddev_threshold: defaults.min_stddev_threshold,
            output_dir: "./output/".to_string(),
            max_concurrent_positions: defaults.max_concurrent_positions,
            volume_filter_enabled: defaults.volume_filter_enabled,
            max_participation_rate: defaults.max_participation_rate,
            slippage_base_bps: defaults.slippage_base_bps,
            slippage_impact_coeff: defaults.slippage_impact_coeff,
        }
    }
}

impl From<RawZScoreConfig> for ZScoreConfig {
    fn from(raw: RawZScoreConfig) -> Self {
        Self {
            coins: raw.coins,
            window_size: raw.window_size,
            candle_interval: CandleInterval::Minute1,
            entry_z_threshold: raw.entry_z_threshold,
            exit_z_threshold: raw.exit_z_threshold,
            total_capital_usdt: Decimal::try_from(raw.total_capital_usdt)
                .unwrap_or(Decimal::new(10000, 0)),
            position_ratio: Decimal::try_from(raw.position_ratio).unwrap_or(Decimal::new(1, 1)),
            upbit_taker_fee: Decimal::try_from(raw.upbit_taker_fee).unwrap_or(Decimal::new(5, 4)),
            bybit_taker_fee: Decimal::try_from(raw.bybit_taker_fee).unwrap_or(Decimal::new(55, 5)),
            leverage: raw.leverage,
            bybit_mmr: Decimal::try_from(raw.bybit_mmr).unwrap_or(Decimal::new(5, 3)),
            backtest_period_minutes: raw.backtest_period_minutes,
            min_stddev_threshold: raw.min_stddev_threshold,
            output_dir: PathBuf::from(raw.output_dir),
            max_concurrent_positions: raw.max_concurrent_positions,
            volume_filter_enabled: raw.volume_filter_enabled,
            max_participation_rate: raw.max_participation_rate,
            slippage_base_bps: raw.slippage_base_bps,
            slippage_impact_coeff: raw.slippage_impact_coeff,
        }
    }
}

/// TOML 역직렬화 전용 sweep 설정.
#[derive(Debug, Deserialize)]
pub struct RawSweepConfig {
    /// 테스트할 entry_z 값 목록.
    pub entry_z_values: Vec<f64>,
    /// 테스트할 exit_z 값 목록 (없으면 base_config.exit_z 사용).
    pub exit_z_values: Option<Vec<f64>>,
    /// 최대 조합 수 (기본 50).
    pub max_combinations: Option<usize>,
}

impl RawSweepConfig {
    /// `SweepConfig`로 변환합니다.
    pub fn into_sweep_config(self, base_config: ZScoreConfig) -> crate::zscore::sweep::SweepConfig {
        let exit_z_values = self
            .exit_z_values
            .unwrap_or_else(|| vec![base_config.exit_z_threshold]);
        crate::zscore::sweep::SweepConfig {
            base_config,
            entry_z_values: self.entry_z_values,
            exit_z_values,
            max_combinations: self.max_combinations.unwrap_or(50),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ZScoreConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_coins() {
        let config = ZScoreConfig {
            coins: vec![],
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_entry_le_exit() {
        let config = ZScoreConfig {
            entry_z_threshold: 0.5,
            exit_z_threshold: 2.0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_position_ratio_too_high() {
        let config = ZScoreConfig {
            position_ratio: Decimal::new(6, 1), // 0.6
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_total_candles_needed() {
        let config = ZScoreConfig::default();
        assert_eq!(config.total_candles_needed(), 1440 + 8640);
    }

    #[test]
    fn test_from_toml_str_full() {
        let toml = r#"
[zscore]
coins = ["BTC", "ETH", "XRP"]
window_size = 720
entry_z_threshold = 2.5
exit_z_threshold = 0.3
total_capital_usdt = 50000.0
position_ratio = 0.05
leverage = 2
bybit_mmr = 0.005
backtest_period_minutes = 4320
min_stddev_threshold = 0.02
output_dir = "./results/"
max_concurrent_positions = 2
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.coins, vec!["BTC", "ETH", "XRP"]);
        assert_eq!(config.window_size, 720);
        assert_eq!(config.entry_z_threshold, 2.5);
        assert_eq!(config.exit_z_threshold, 0.3);
        assert_eq!(config.total_capital_usdt, Decimal::new(50000, 0));
        assert_eq!(config.leverage, 2);
        assert_eq!(config.backtest_period_minutes, 4320);
        assert_eq!(config.min_stddev_threshold, 0.02);
        assert_eq!(config.output_dir, PathBuf::from("./results/"));
        assert_eq!(config.max_concurrent_positions, Some(2));
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_from_toml_str_defaults() {
        let toml = "[zscore]\n";
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        let defaults = ZScoreConfig::default();
        assert_eq!(config.coins, defaults.coins);
        assert_eq!(config.window_size, defaults.window_size);
        assert_eq!(config.entry_z_threshold, defaults.entry_z_threshold);
    }

    #[test]
    fn test_from_toml_str_partial() {
        let toml = r#"
[zscore]
coins = ["SOL"]
entry_z_threshold = 3.0
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.coins, vec!["SOL"]);
        assert_eq!(config.entry_z_threshold, 3.0);
        // 나머지는 기본값
        assert_eq!(config.window_size, 1440);
    }

    #[test]
    fn test_from_toml_str_invalid() {
        let toml = "invalid toml {{{}}}";
        assert!(ZScoreConfig::from_toml_str(toml).is_err());
    }

    #[test]
    fn test_from_toml_str_with_sweep() {
        let toml = r#"
[zscore]
coins = ["BTC"]
entry_z_threshold = 2.0

[sweep]
entry_z_values = [1.5, 2.0, 2.5, 3.0]
exit_z_values = [0.3, 0.5, 0.7]
max_combinations = 100
"#;
        let (config, raw_sweep) = ZScoreConfig::from_toml_str_with_sweep(toml).unwrap();
        assert_eq!(config.coins, vec!["BTC"]);
        assert_eq!(config.entry_z_threshold, 2.0);

        let raw_sweep = raw_sweep.expect("sweep 섹션이 파싱되어야 함");
        assert_eq!(raw_sweep.entry_z_values, vec![1.5, 2.0, 2.5, 3.0]);
        assert_eq!(raw_sweep.exit_z_values, Some(vec![0.3, 0.5, 0.7]));
        assert_eq!(raw_sweep.max_combinations, Some(100));

        let sweep_config = raw_sweep.into_sweep_config(config);
        assert_eq!(sweep_config.entry_z_values.len(), 4);
        assert_eq!(sweep_config.exit_z_values.len(), 3);
        assert_eq!(sweep_config.max_combinations, 100);
    }

    #[test]
    fn test_from_toml_str_with_sweep_defaults() {
        let toml = r#"
[zscore]
coins = ["ETH"]
exit_z_threshold = 0.4

[sweep]
entry_z_values = [2.0, 2.5]
"#;
        let (config, raw_sweep) = ZScoreConfig::from_toml_str_with_sweep(toml).unwrap();
        let raw_sweep = raw_sweep.expect("sweep 섹션이 파싱되어야 함");
        assert!(raw_sweep.exit_z_values.is_none());
        assert!(raw_sweep.max_combinations.is_none());

        let sweep_config = raw_sweep.into_sweep_config(config);
        // exit_z_values가 없으면 base_config.exit_z_threshold 사용
        assert_eq!(sweep_config.exit_z_values, vec![0.4]);
        assert_eq!(sweep_config.max_combinations, 50);
    }

    #[test]
    fn test_from_toml_str_without_sweep() {
        let toml = r#"
[zscore]
coins = ["BTC"]
"#;
        let (_config, raw_sweep) = ZScoreConfig::from_toml_str_with_sweep(toml).unwrap();
        assert!(raw_sweep.is_none());
    }
}
