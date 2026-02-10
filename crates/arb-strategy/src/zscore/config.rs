//! Z-Score 전략 설정.
//!
//! TOML 파일에서 전략 설정을 로드합니다.
//! `arb-config`의 간이 파서는 중첩 섹션을 지원하지 않으므로,
//! 전략 설정은 별도 파일(`strategy.toml`)로 분리하여 자체 로딩합니다.

use std::path::Path;

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
    /// 코인 페어당 최대 포지션 크기 비율 = total_capital_usdt × max_position_ratio.
    pub max_position_ratio: Decimal,
    /// Upbit taker 수수료율 (기본값: 0.0005 = 0.05%).
    pub upbit_taker_fee: Decimal,
    /// Bybit linear taker 수수료율 (기본값: 0.00055 = 0.055%).
    pub bybit_taker_fee: Decimal,
    /// Bybit 레버리지 (기본값: 1).
    pub leverage: u32,
    /// Bybit maintenance margin rate (기본값: 0.005 = 0.5%).
    pub bybit_mmr: Decimal,
    /// Z-Score 계산 시 최소 stddev 임계값 (기본값: 0.01).
    pub min_stddev_threshold: f64,
    /// 최대 동시 포지션 수 (None이면 coins.len()까지 허용).
    pub max_concurrent_positions: Option<usize>,
    /// 자동 코인 선택 활성화 (기본값: false).
    /// true이면 거래량 기반으로 대상 코인을 자동 선택합니다.
    pub auto_select: bool,
    /// 자동 선택 시 최대 코인 수 (기본값: 5).
    pub max_coins: usize,
    /// 코인 재선택 주기 (분, 기본값: 10).
    pub reselect_interval_min: u64,
    /// 자동 선택 시 최소 1시간 거래량 (USDT 기준, 기본값: 1,000,000).
    pub min_volume_1h_usdt: Decimal,
    /// 코인 선택 시 최대 스프레드 stddev (기본값: 0.5).
    /// 워밍업 후 계산된 stddev가 이 값을 초과하는 코인은 자동 선택에서 제외.
    /// 0.0이면 필터 비활성화.
    pub max_spread_stddev: f64,
    /// 자동 선택에서 제외할 코인 블랙리스트.
    pub blacklist: Vec<String>,
    /// 포지션 TTL (시간 단위, 기본값: 24).
    /// 해당 시간 이후 자동 청산합니다.
    pub position_ttl_hours: u64,
    /// TTL 초과 후 분할 청산 유예 기간 (시간).
    pub grace_period_hours: u64,
    /// 같은 코인에 대한 재진입 최소 간격 (초).
    pub entry_cooldown_sec: u64,
    /// 오더북 캐시 최대 유효 시간 (초).
    pub max_cache_age_sec: u64,
    /// 세션 출력 설정.
    pub output: crate::output::writer::OutputConfig,
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
            max_position_ratio: Decimal::new(2, 1), // 0.2
            upbit_taker_fee: Decimal::new(5, 4),    // 0.0005
            bybit_taker_fee: Decimal::new(55, 5),   // 0.00055
            leverage: 1,
            bybit_mmr: Decimal::new(5, 3), // 0.005
            min_stddev_threshold: 0.01,
            max_concurrent_positions: None,
            auto_select: false,
            max_coins: 5,
            reselect_interval_min: 10,
            min_volume_1h_usdt: Decimal::new(50_000, 0),
            max_spread_stddev: 0.5,
            blacklist: vec![],
            position_ttl_hours: 24,
            grace_period_hours: 4,
            entry_cooldown_sec: 10,
            max_cache_age_sec: 5,
            output: crate::output::writer::OutputConfig::default(),
        }
    }
}

impl ZScoreConfig {
    /// 설정값 유효성 검증.
    pub fn validate(&self) -> Result<(), StrategyError> {
        // auto_select=false이면 coins가 비어있으면 에러
        if !self.auto_select && self.coins.is_empty() {
            return Err(StrategyError::Config("coins must not be empty".to_string()));
        }
        // auto_select=true이면 max_coins > 0 필수
        if self.auto_select && self.max_coins == 0 {
            return Err(StrategyError::Config(
                "max_coins must be greater than 0 when auto_select is enabled".to_string(),
            ));
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
        if self.max_position_ratio <= Decimal::ZERO || self.max_position_ratio > Decimal::ONE {
            return Err(StrategyError::Config(
                "max_position_ratio must be in (0, 1.0]".to_string(),
            ));
        }
        if self.min_stddev_threshold <= 0.0 {
            return Err(StrategyError::Config(
                "min_stddev_threshold must be positive".to_string(),
            ));
        }
        if self.max_spread_stddev < 0.0 {
            return Err(StrategyError::Config(
                "max_spread_stddev must be non-negative".to_string(),
            ));
        }
        if !self.auto_select && self.max_spread_stddev > 0.0 {
            warn!("max_spread_stddev는 auto_select=true일 때만 적용됩니다");
        }
        if self.grace_period_hours == 0 {
            return Err(StrategyError::Config(
                "grace_period_hours must be greater than 0".to_string(),
            ));
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
        let mut config: ZScoreConfig = wrapper.zscore.into();

        // [output] 섹션이 있으면 OutputConfig로 변환
        if let Some(raw_output) = wrapper.output {
            config.output = crate::output::writer::OutputConfig {
                enabled: raw_output.enabled.unwrap_or(true),
                dir: raw_output.dir.unwrap_or_else(|| "output".to_string()),
            };
        }

        Ok(config)
    }
}

/// TOML 최상위 래퍼 (`[zscore]`, `[output]` 섹션).
#[derive(Deserialize)]
struct TomlWrapper {
    #[serde(default)]
    zscore: RawZScoreConfig,
    #[serde(default)]
    output: Option<RawOutputConfig>,
}

/// TOML 출력 설정 역직렬화용 중간 구조체.
#[derive(Deserialize, Default)]
struct RawOutputConfig {
    enabled: Option<bool>,
    dir: Option<String>,
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
    max_position_ratio: Option<f64>,
    upbit_taker_fee: f64,
    bybit_taker_fee: f64,
    leverage: u32,
    bybit_mmr: f64,
    min_stddev_threshold: f64,
    max_concurrent_positions: Option<usize>,
    auto_select: Option<bool>,
    max_coins: Option<usize>,
    reselect_interval_min: Option<u64>,
    min_volume_1h_usdt: Option<f64>,
    max_spread_stddev: Option<f64>,
    blacklist: Option<Vec<String>>,
    position_ttl_hours: Option<u64>,
    grace_period_hours: Option<u64>,
    entry_cooldown_sec: Option<u64>,
    max_cache_age_sec: Option<u64>,
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
            max_position_ratio: None,
            upbit_taker_fee: 0.0005,
            bybit_taker_fee: 0.00055,
            leverage: defaults.leverage,
            bybit_mmr: 0.005,
            min_stddev_threshold: defaults.min_stddev_threshold,
            max_concurrent_positions: defaults.max_concurrent_positions,
            auto_select: None,
            max_coins: None,
            reselect_interval_min: None,
            min_volume_1h_usdt: None,
            max_spread_stddev: None,
            blacklist: None,
            position_ttl_hours: None,
            grace_period_hours: None,
            entry_cooldown_sec: None,
            max_cache_age_sec: None,
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
            max_position_ratio: raw
                .max_position_ratio
                .and_then(|v| Decimal::try_from(v).ok())
                .unwrap_or(Decimal::new(2, 1)), // 0.2
            upbit_taker_fee: Decimal::try_from(raw.upbit_taker_fee).unwrap_or(Decimal::new(5, 4)),
            bybit_taker_fee: Decimal::try_from(raw.bybit_taker_fee).unwrap_or(Decimal::new(55, 5)),
            leverage: raw.leverage,
            bybit_mmr: Decimal::try_from(raw.bybit_mmr).unwrap_or(Decimal::new(5, 3)),
            min_stddev_threshold: raw.min_stddev_threshold,
            max_concurrent_positions: raw.max_concurrent_positions,
            auto_select: raw.auto_select.unwrap_or(false),
            max_coins: raw.max_coins.unwrap_or(5),
            reselect_interval_min: raw.reselect_interval_min.unwrap_or(10),
            min_volume_1h_usdt: raw
                .min_volume_1h_usdt
                .and_then(|v| Decimal::try_from(v).ok())
                .unwrap_or(Decimal::new(50_000, 0)),
            max_spread_stddev: raw.max_spread_stddev.unwrap_or(0.5),
            blacklist: raw.blacklist.unwrap_or_default(),
            position_ttl_hours: raw.position_ttl_hours.unwrap_or(24),
            grace_period_hours: raw.grace_period_hours.unwrap_or(4),
            entry_cooldown_sec: raw.entry_cooldown_sec.unwrap_or(10),
            max_cache_age_sec: raw.max_cache_age_sec.unwrap_or(5),
            output: crate::output::writer::OutputConfig::default(),
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
    fn test_validate_max_position_ratio_too_high() {
        let config = ZScoreConfig {
            max_position_ratio: Decimal::new(11, 1), // 1.1
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
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
max_position_ratio = 0.05
leverage = 2
bybit_mmr = 0.005
min_stddev_threshold = 0.02
max_concurrent_positions = 2
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.coins, vec!["BTC", "ETH", "XRP"]);
        assert_eq!(config.window_size, 720);
        assert_eq!(config.entry_z_threshold, 2.5);
        assert_eq!(config.exit_z_threshold, 0.3);
        assert_eq!(config.total_capital_usdt, Decimal::new(50000, 0));
        assert_eq!(config.leverage, 2);
        assert_eq!(config.min_stddev_threshold, 0.02);
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
    fn test_auto_select_defaults() {
        let config = ZScoreConfig::default();
        assert!(!config.auto_select);
        assert_eq!(config.max_coins, 5);
        assert_eq!(config.reselect_interval_min, 10);
        assert_eq!(config.min_volume_1h_usdt, Decimal::new(50_000, 0));
        assert!(config.blacklist.is_empty());
        assert_eq!(config.position_ttl_hours, 24);
    }

    #[test]
    fn test_auto_select_from_toml() {
        let toml = r#"
[zscore]
coins = ["BTC"]
auto_select = true
max_coins = 3
reselect_interval_min = 5
min_volume_1h_usdt = 500000.0
blacklist = ["USDC", "USDT"]
position_ttl_hours = 12
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert!(config.auto_select);
        assert_eq!(config.max_coins, 3);
        assert_eq!(config.reselect_interval_min, 5);
        assert_eq!(config.min_volume_1h_usdt, Decimal::new(500_000, 0));
        assert_eq!(config.blacklist, vec!["USDC", "USDT"]);
        assert_eq!(config.position_ttl_hours, 12);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_auto_select_partial_toml_uses_defaults() {
        let toml = r#"
[zscore]
coins = ["BTC"]
auto_select = true
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert!(config.auto_select);
        // 나머지는 기본값
        assert_eq!(config.max_coins, 5);
        assert_eq!(config.reselect_interval_min, 10);
        assert_eq!(config.min_volume_1h_usdt, Decimal::new(50_000, 0));
        assert!(config.blacklist.is_empty());
        assert_eq!(config.position_ttl_hours, 24);
    }

    #[test]
    fn test_auto_select_true_empty_coins_is_valid() {
        // auto_select=true이면 coins가 비어있어도 유효
        let config = ZScoreConfig {
            coins: vec![],
            auto_select: true,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_auto_select_false_empty_coins_is_invalid() {
        // auto_select=false이면 coins가 비어있으면 에러
        let config = ZScoreConfig {
            coins: vec![],
            auto_select: false,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_auto_select_max_coins_zero_is_invalid() {
        // auto_select=true이고 max_coins=0이면 에러
        let config = ZScoreConfig {
            auto_select: true,
            max_coins: 0,
            ..ZScoreConfig::default()
        };
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("max_coins"));
    }

    // --- max_spread_stddev 테스트 ---

    #[test]
    fn test_max_spread_stddev_default() {
        let config = ZScoreConfig::default();
        assert_eq!(config.max_spread_stddev, 0.5);
    }

    #[test]
    fn test_max_spread_stddev_negative_invalid() {
        let config = ZScoreConfig {
            max_spread_stddev: -0.1,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_max_spread_stddev_zero_disables() {
        let config = ZScoreConfig {
            max_spread_stddev: 0.0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_max_spread_stddev_from_toml() {
        let toml = r#"
[zscore]
coins = ["BTC"]
auto_select = true
max_spread_stddev = 0.3
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.max_spread_stddev, 0.3);
    }

    #[test]
    fn test_max_spread_stddev_toml_default() {
        let toml = r#"
[zscore]
coins = ["BTC"]
auto_select = true
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.max_spread_stddev, 0.5);
    }

    // --- OutputConfig 테스트 ---

    #[test]
    fn test_output_config_default() {
        // 기본 ZScoreConfig의 output 필드가 기본값인지 확인
        let config = ZScoreConfig::default();
        assert!(config.output.enabled);
        assert_eq!(config.output.dir, "output");
    }

    #[test]
    fn test_output_config_from_toml() {
        // TOML에서 [output] 섹션을 파싱하여 OutputConfig로 변환
        let toml = r#"
[zscore]
coins = ["BTC"]

[output]
enabled = false
dir = "my_output"
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert!(!config.output.enabled);
        assert_eq!(config.output.dir, "my_output");
    }

    #[test]
    fn test_output_config_missing_uses_default() {
        // [output] 섹션이 없으면 기본값 사용
        let toml = r#"
[zscore]
coins = ["BTC"]
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert!(config.output.enabled);
        assert_eq!(config.output.dir, "output");
    }
}
