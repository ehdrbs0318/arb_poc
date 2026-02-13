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
    /// 최소 기대 수익률 (%, 기본값: 0.10).
    /// 라운딩 후 adjusted_profit가 이 값 미만이면 진입 거부.
    /// 0.0이면 비활성화 (기존 동작과 동일).
    pub min_expected_roi: f64,
    /// 최소 포지션 크기 (USDT, 기본값: 100.0).
    /// qty × bybit_price가 이 값 미만이면 진입 거부.
    /// 0.0이면 비활성화.
    pub min_position_usdt: Decimal,
    /// 세션 출력 설정.
    pub output: crate::output::writer::OutputConfig,

    // === 주문 실행 (라이브 전용) ===
    /// Bybit 선물 카테고리.
    pub bybit_category: String,
    /// 주문 체결 대기 타임아웃 (초).
    pub order_timeout_sec: u64,
    /// 주문 재시도 횟수.
    pub max_retry_count: u32,
    /// 주문 타입: "limit_ioc", "limit_gtc_cancel", "market".
    pub order_type: String,
    /// IOC/GTC 지정가 시 최대 슬리피지 (%).
    pub max_slippage_pct: f64,
    /// Post-execution PnL gate: 수수료의 N% 이상이면 보유 (0.5 = 50%).
    pub post_exec_pnl_gate_ratio: f64,
    /// 비상 청산 IOC 슬리피지 단계 (%).
    pub emergency_wide_ioc_slippage_pct: Vec<f64>,
    /// REST 실패 시 비상 가격 마진 % (매수 +, 매도 -).
    pub emergency_price_fallback_margin_pct: f64,
    /// Dust threshold (USDT): 이하 잔량은 즉시 adjustment_cost로 기록.
    pub max_dust_usdt: f64,

    // === 리스크 관리 ===
    /// Kill switch 활성화 여부.
    pub kill_switch_enabled: bool,
    /// 일일 최대 손실 (자본의 %).
    pub max_daily_loss_pct: f64,
    /// 최대 드로다운 (자본의 %).
    pub max_drawdown_pct: f64,
    /// 단건 최대 손실 (자본의 %).
    pub max_single_loss_pct: f64,
    /// 일일 최대 손실 (USDT).
    pub max_daily_loss_usdt: f64,
    /// 최대 드로다운 (USDT).
    pub max_drawdown_usdt: f64,
    /// 단건 최대 손실 (USDT).
    pub max_single_loss_usdt: f64,
    /// 단일 주문 크기 상한 (USDT, 버그 방어).
    pub max_order_size_usdt: f64,
    /// Rolling 24h 누적 손실 상한 (USDT).
    pub max_rolling_24h_loss_usdt: f64,
    /// HWM drawdown 측정 window (일).
    pub hwm_window_days: u32,
    /// 전체 미실현 손실 한도 (자본의 %).
    pub max_unrealized_loss_pct: f64,

    // === 환율 ===
    /// 환율 캐시 최대 수명 (분).
    pub max_forex_age_min: u64,
    /// 환율 급변 알림 임계치 (%).
    pub forex_change_alert_pct: f64,
    /// 환율 급변 후 안정 대기 시간 (분).
    pub forex_stabilization_minutes: u64,

    // === 펀딩비 ===
    /// 정산 N분 전부터 진입 차단.
    pub funding_block_before_min: u64,
    /// 정산 후 N분까지 진입 차단.
    pub funding_block_after_min: u64,
    /// 정산 전 불리 포지션 강제 청산 활성화.
    pub funding_force_close_enabled: bool,
    /// Major 코인 정산 N분 전 강제 청산.
    pub funding_force_close_minutes_major: u64,
    /// Alt 코인 정산 N분 전 강제 청산.
    pub funding_force_close_minutes_alt: u64,
    /// Major 코인 목록.
    pub funding_major_coins: Vec<String>,
    /// 펀딩비 > 수익의 N% 시 경고.
    pub funding_alert_ratio: f64,
    /// 펀딩비 > 수익의 N% 시 코인 제외.
    pub funding_exclude_ratio: f64,

    // === PendingExchangeRecovery ===
    /// 최대 체류 시간 (시간).
    pub pending_recovery_timeout_hours: u64,

    // === Graceful shutdown ===
    /// 종료 정책: "keep" | "close_all" | "close_if_profitable".
    pub shutdown_policy: String,

    // === 텔레그램 ===
    /// 텔레그램 알림 활성화.
    pub telegram_enabled: bool,
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
            min_expected_roi: 0.10,
            min_position_usdt: Decimal::new(100, 0),
            output: crate::output::writer::OutputConfig::default(),
            // 주문 실행
            bybit_category: "linear".to_string(),
            order_timeout_sec: 5,
            max_retry_count: 2,
            order_type: "limit_ioc".to_string(),
            max_slippage_pct: 0.1,
            post_exec_pnl_gate_ratio: 0.5,
            emergency_wide_ioc_slippage_pct: vec![2.0, 3.0, 5.0],
            emergency_price_fallback_margin_pct: 5.0,
            max_dust_usdt: 5.0,
            // 리스크 관리
            kill_switch_enabled: true,
            max_daily_loss_pct: 10.0,
            max_drawdown_pct: 5.0,
            max_single_loss_pct: 3.0,
            max_daily_loss_usdt: 50.0,
            max_drawdown_usdt: 25.0,
            max_single_loss_usdt: 15.0,
            max_order_size_usdt: 2000.0,
            max_rolling_24h_loss_usdt: 80.0,
            hwm_window_days: 7,
            max_unrealized_loss_pct: 7.0,
            // 환율
            max_forex_age_min: 10,
            forex_change_alert_pct: 0.2,
            forex_stabilization_minutes: 5,
            // 펀딩비
            funding_block_before_min: 60,
            funding_block_after_min: 15,
            funding_force_close_enabled: true,
            funding_force_close_minutes_major: 15,
            funding_force_close_minutes_alt: 30,
            funding_major_coins: vec!["BTC".to_string(), "ETH".to_string()],
            funding_alert_ratio: 0.2,
            funding_exclude_ratio: 0.5,
            // PendingExchangeRecovery
            pending_recovery_timeout_hours: 2,
            // Graceful shutdown
            shutdown_policy: "keep".to_string(),
            // 텔레그램
            telegram_enabled: true,
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
        if self.min_expected_roi < 0.0 {
            return Err(StrategyError::Config(
                "min_expected_roi must be non-negative".to_string(),
            ));
        }
        if self.min_position_usdt < Decimal::ZERO {
            return Err(StrategyError::Config(
                "min_position_usdt must be non-negative".to_string(),
            ));
        }
        if self.grace_period_hours == 0 {
            return Err(StrategyError::Config(
                "grace_period_hours must be greater than 0".to_string(),
            ));
        }

        // --- 라이브 전용 필드 유효성 검증 ---

        // 주문 실행
        if self.order_timeout_sec == 0 {
            return Err(StrategyError::Config(
                "order_timeout_sec must be greater than 0".to_string(),
            ));
        }
        let valid_order_types = ["limit_ioc", "limit_gtc_cancel", "market"];
        if !valid_order_types.contains(&self.order_type.as_str()) {
            return Err(StrategyError::Config(format!(
                "order_type must be one of {:?}, got: {}",
                valid_order_types, self.order_type
            )));
        }
        if self.max_slippage_pct < 0.0 {
            return Err(StrategyError::Config(
                "max_slippage_pct must be non-negative".to_string(),
            ));
        }
        if self.post_exec_pnl_gate_ratio < 0.0 {
            return Err(StrategyError::Config(
                "post_exec_pnl_gate_ratio must be non-negative".to_string(),
            ));
        }
        if self.emergency_wide_ioc_slippage_pct.is_empty() {
            return Err(StrategyError::Config(
                "emergency_wide_ioc_slippage_pct must not be empty".to_string(),
            ));
        }
        if self.emergency_price_fallback_margin_pct <= 0.0 {
            return Err(StrategyError::Config(
                "emergency_price_fallback_margin_pct must be positive".to_string(),
            ));
        }
        if self.max_dust_usdt < 0.0 {
            return Err(StrategyError::Config(
                "max_dust_usdt must be non-negative".to_string(),
            ));
        }

        // 리스크 관리
        if self.max_daily_loss_pct <= 0.0 {
            return Err(StrategyError::Config(
                "max_daily_loss_pct must be positive".to_string(),
            ));
        }
        if self.max_drawdown_pct <= 0.0 {
            return Err(StrategyError::Config(
                "max_drawdown_pct must be positive".to_string(),
            ));
        }
        if self.max_single_loss_pct <= 0.0 {
            return Err(StrategyError::Config(
                "max_single_loss_pct must be positive".to_string(),
            ));
        }
        if self.max_daily_loss_usdt <= 0.0 {
            return Err(StrategyError::Config(
                "max_daily_loss_usdt must be positive".to_string(),
            ));
        }
        if self.max_drawdown_usdt <= 0.0 {
            return Err(StrategyError::Config(
                "max_drawdown_usdt must be positive".to_string(),
            ));
        }
        if self.max_single_loss_usdt <= 0.0 {
            return Err(StrategyError::Config(
                "max_single_loss_usdt must be positive".to_string(),
            ));
        }
        if self.max_order_size_usdt <= 0.0 {
            return Err(StrategyError::Config(
                "max_order_size_usdt must be positive".to_string(),
            ));
        }
        if self.max_rolling_24h_loss_usdt <= 0.0 {
            return Err(StrategyError::Config(
                "max_rolling_24h_loss_usdt must be positive".to_string(),
            ));
        }
        if self.hwm_window_days == 0 {
            return Err(StrategyError::Config(
                "hwm_window_days must be greater than 0".to_string(),
            ));
        }
        if self.max_unrealized_loss_pct <= 0.0 {
            return Err(StrategyError::Config(
                "max_unrealized_loss_pct must be positive".to_string(),
            ));
        }

        // 환율
        if self.max_forex_age_min == 0 {
            return Err(StrategyError::Config(
                "max_forex_age_min must be greater than 0".to_string(),
            ));
        }
        if self.forex_change_alert_pct <= 0.0 {
            return Err(StrategyError::Config(
                "forex_change_alert_pct must be positive".to_string(),
            ));
        }

        // 펀딩비
        if self.funding_block_before_min == 0 {
            return Err(StrategyError::Config(
                "funding_block_before_min must be greater than 0".to_string(),
            ));
        }
        if self.funding_exclude_ratio <= self.funding_alert_ratio {
            return Err(StrategyError::Config(
                "funding_exclude_ratio must be greater than funding_alert_ratio".to_string(),
            ));
        }

        // Graceful shutdown
        let valid_policies = ["keep", "close_all", "close_if_profitable"];
        if !valid_policies.contains(&self.shutdown_policy.as_str()) {
            return Err(StrategyError::Config(format!(
                "shutdown_policy must be one of {:?}, got: {}",
                valid_policies, self.shutdown_policy
            )));
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

// === serde default 함수 ===

fn default_bybit_category() -> String {
    "linear".to_string()
}
fn default_order_timeout_sec() -> u64 {
    5
}
fn default_max_retry_count() -> u32 {
    2
}
fn default_order_type() -> String {
    "limit_ioc".to_string()
}
fn default_max_slippage_pct() -> f64 {
    0.1
}
fn default_post_exec_pnl_gate_ratio() -> f64 {
    0.5
}
fn default_emergency_wide_ioc_slippage_pct() -> Vec<f64> {
    vec![2.0, 3.0, 5.0]
}
fn default_emergency_price_fallback_margin_pct() -> f64 {
    5.0
}
fn default_max_dust_usdt() -> f64 {
    5.0
}
fn default_true() -> bool {
    true
}
fn default_max_daily_loss_pct() -> f64 {
    10.0
}
fn default_max_drawdown_pct() -> f64 {
    5.0
}
fn default_max_single_loss_pct() -> f64 {
    3.0
}
fn default_max_daily_loss_usdt() -> f64 {
    50.0
}
fn default_max_drawdown_usdt() -> f64 {
    25.0
}
fn default_max_single_loss_usdt() -> f64 {
    15.0
}
fn default_max_order_size_usdt() -> f64 {
    2000.0
}
fn default_max_rolling_24h_loss_usdt() -> f64 {
    80.0
}
fn default_hwm_window_days() -> u32 {
    7
}
fn default_max_unrealized_loss_pct() -> f64 {
    7.0
}
fn default_max_forex_age_min() -> u64 {
    10
}
fn default_forex_change_alert_pct() -> f64 {
    0.2
}
fn default_forex_stabilization_minutes() -> u64 {
    5
}
fn default_funding_block_before_min() -> u64 {
    60
}
fn default_funding_block_after_min() -> u64 {
    15
}
fn default_funding_force_close_minutes_major() -> u64 {
    15
}
fn default_funding_force_close_minutes_alt() -> u64 {
    30
}
fn default_funding_major_coins() -> Vec<String> {
    vec!["BTC".to_string(), "ETH".to_string()]
}
fn default_funding_alert_ratio() -> f64 {
    0.2
}
fn default_funding_exclude_ratio() -> f64 {
    0.5
}
fn default_pending_recovery_timeout_hours() -> u64 {
    2
}
fn default_shutdown_policy() -> String {
    "keep".to_string()
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
    min_expected_roi: Option<f64>,
    min_position_usdt: Option<f64>,
    // === 주문 실행 (라이브 전용) ===
    #[serde(default = "default_bybit_category")]
    bybit_category: String,
    #[serde(default = "default_order_timeout_sec")]
    order_timeout_sec: u64,
    #[serde(default = "default_max_retry_count")]
    max_retry_count: u32,
    #[serde(default = "default_order_type")]
    order_type: String,
    #[serde(default = "default_max_slippage_pct")]
    max_slippage_pct: f64,
    #[serde(default = "default_post_exec_pnl_gate_ratio")]
    post_exec_pnl_gate_ratio: f64,
    #[serde(default = "default_emergency_wide_ioc_slippage_pct")]
    emergency_wide_ioc_slippage_pct: Vec<f64>,
    #[serde(default = "default_emergency_price_fallback_margin_pct")]
    emergency_price_fallback_margin_pct: f64,
    #[serde(default = "default_max_dust_usdt")]
    max_dust_usdt: f64,
    // === 리스크 관리 ===
    #[serde(default = "default_true")]
    kill_switch_enabled: bool,
    #[serde(default = "default_max_daily_loss_pct")]
    max_daily_loss_pct: f64,
    #[serde(default = "default_max_drawdown_pct")]
    max_drawdown_pct: f64,
    #[serde(default = "default_max_single_loss_pct")]
    max_single_loss_pct: f64,
    #[serde(default = "default_max_daily_loss_usdt")]
    max_daily_loss_usdt: f64,
    #[serde(default = "default_max_drawdown_usdt")]
    max_drawdown_usdt: f64,
    #[serde(default = "default_max_single_loss_usdt")]
    max_single_loss_usdt: f64,
    #[serde(default = "default_max_order_size_usdt")]
    max_order_size_usdt: f64,
    #[serde(default = "default_max_rolling_24h_loss_usdt")]
    max_rolling_24h_loss_usdt: f64,
    #[serde(default = "default_hwm_window_days")]
    hwm_window_days: u32,
    #[serde(default = "default_max_unrealized_loss_pct")]
    max_unrealized_loss_pct: f64,
    // === 환율 ===
    #[serde(default = "default_max_forex_age_min")]
    max_forex_age_min: u64,
    #[serde(default = "default_forex_change_alert_pct")]
    forex_change_alert_pct: f64,
    #[serde(default = "default_forex_stabilization_minutes")]
    forex_stabilization_minutes: u64,
    // === 펀딩비 ===
    #[serde(default = "default_funding_block_before_min")]
    funding_block_before_min: u64,
    #[serde(default = "default_funding_block_after_min")]
    funding_block_after_min: u64,
    #[serde(default = "default_true")]
    funding_force_close_enabled: bool,
    #[serde(default = "default_funding_force_close_minutes_major")]
    funding_force_close_minutes_major: u64,
    #[serde(default = "default_funding_force_close_minutes_alt")]
    funding_force_close_minutes_alt: u64,
    #[serde(default = "default_funding_major_coins")]
    funding_major_coins: Vec<String>,
    #[serde(default = "default_funding_alert_ratio")]
    funding_alert_ratio: f64,
    #[serde(default = "default_funding_exclude_ratio")]
    funding_exclude_ratio: f64,
    // === PendingExchangeRecovery ===
    #[serde(default = "default_pending_recovery_timeout_hours")]
    pending_recovery_timeout_hours: u64,
    // === Graceful shutdown ===
    #[serde(default = "default_shutdown_policy")]
    shutdown_policy: String,
    // === 텔레그램 ===
    #[serde(default = "default_true")]
    telegram_enabled: bool,
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
            min_expected_roi: None,
            min_position_usdt: None,
            // 주문 실행
            bybit_category: default_bybit_category(),
            order_timeout_sec: default_order_timeout_sec(),
            max_retry_count: default_max_retry_count(),
            order_type: default_order_type(),
            max_slippage_pct: default_max_slippage_pct(),
            post_exec_pnl_gate_ratio: default_post_exec_pnl_gate_ratio(),
            emergency_wide_ioc_slippage_pct: default_emergency_wide_ioc_slippage_pct(),
            emergency_price_fallback_margin_pct: default_emergency_price_fallback_margin_pct(),
            max_dust_usdt: default_max_dust_usdt(),
            // 리스크 관리
            kill_switch_enabled: default_true(),
            max_daily_loss_pct: default_max_daily_loss_pct(),
            max_drawdown_pct: default_max_drawdown_pct(),
            max_single_loss_pct: default_max_single_loss_pct(),
            max_daily_loss_usdt: default_max_daily_loss_usdt(),
            max_drawdown_usdt: default_max_drawdown_usdt(),
            max_single_loss_usdt: default_max_single_loss_usdt(),
            max_order_size_usdt: default_max_order_size_usdt(),
            max_rolling_24h_loss_usdt: default_max_rolling_24h_loss_usdt(),
            hwm_window_days: default_hwm_window_days(),
            max_unrealized_loss_pct: default_max_unrealized_loss_pct(),
            // 환율
            max_forex_age_min: default_max_forex_age_min(),
            forex_change_alert_pct: default_forex_change_alert_pct(),
            forex_stabilization_minutes: default_forex_stabilization_minutes(),
            // 펀딩비
            funding_block_before_min: default_funding_block_before_min(),
            funding_block_after_min: default_funding_block_after_min(),
            funding_force_close_enabled: default_true(),
            funding_force_close_minutes_major: default_funding_force_close_minutes_major(),
            funding_force_close_minutes_alt: default_funding_force_close_minutes_alt(),
            funding_major_coins: default_funding_major_coins(),
            funding_alert_ratio: default_funding_alert_ratio(),
            funding_exclude_ratio: default_funding_exclude_ratio(),
            // PendingExchangeRecovery
            pending_recovery_timeout_hours: default_pending_recovery_timeout_hours(),
            // Graceful shutdown
            shutdown_policy: default_shutdown_policy(),
            // 텔레그램
            telegram_enabled: default_true(),
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
            min_expected_roi: raw.min_expected_roi.unwrap_or(0.10),
            min_position_usdt: raw
                .min_position_usdt
                .and_then(|v| Decimal::try_from(v).ok())
                .unwrap_or(Decimal::new(100, 0)),
            output: crate::output::writer::OutputConfig::default(),
            // 주문 실행
            bybit_category: raw.bybit_category,
            order_timeout_sec: raw.order_timeout_sec,
            max_retry_count: raw.max_retry_count,
            order_type: raw.order_type,
            max_slippage_pct: raw.max_slippage_pct,
            post_exec_pnl_gate_ratio: raw.post_exec_pnl_gate_ratio,
            emergency_wide_ioc_slippage_pct: raw.emergency_wide_ioc_slippage_pct,
            emergency_price_fallback_margin_pct: raw.emergency_price_fallback_margin_pct,
            max_dust_usdt: raw.max_dust_usdt,
            // 리스크 관리
            kill_switch_enabled: raw.kill_switch_enabled,
            max_daily_loss_pct: raw.max_daily_loss_pct,
            max_drawdown_pct: raw.max_drawdown_pct,
            max_single_loss_pct: raw.max_single_loss_pct,
            max_daily_loss_usdt: raw.max_daily_loss_usdt,
            max_drawdown_usdt: raw.max_drawdown_usdt,
            max_single_loss_usdt: raw.max_single_loss_usdt,
            max_order_size_usdt: raw.max_order_size_usdt,
            max_rolling_24h_loss_usdt: raw.max_rolling_24h_loss_usdt,
            hwm_window_days: raw.hwm_window_days,
            max_unrealized_loss_pct: raw.max_unrealized_loss_pct,
            // 환율
            max_forex_age_min: raw.max_forex_age_min,
            forex_change_alert_pct: raw.forex_change_alert_pct,
            forex_stabilization_minutes: raw.forex_stabilization_minutes,
            // 펀딩비
            funding_block_before_min: raw.funding_block_before_min,
            funding_block_after_min: raw.funding_block_after_min,
            funding_force_close_enabled: raw.funding_force_close_enabled,
            funding_force_close_minutes_major: raw.funding_force_close_minutes_major,
            funding_force_close_minutes_alt: raw.funding_force_close_minutes_alt,
            funding_major_coins: raw.funding_major_coins,
            funding_alert_ratio: raw.funding_alert_ratio,
            funding_exclude_ratio: raw.funding_exclude_ratio,
            // PendingExchangeRecovery
            pending_recovery_timeout_hours: raw.pending_recovery_timeout_hours,
            // Graceful shutdown
            shutdown_policy: raw.shutdown_policy,
            // 텔레그램
            telegram_enabled: raw.telegram_enabled,
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

    // --- min_expected_roi / min_position_usdt 테스트 (spec/0010) ---

    #[test]
    fn test_min_expected_roi_default() {
        let config = ZScoreConfig::default();
        assert_eq!(config.min_expected_roi, 0.10);
    }

    #[test]
    fn test_min_position_usdt_default() {
        let config = ZScoreConfig::default();
        assert_eq!(config.min_position_usdt, Decimal::new(100, 0));
    }

    #[test]
    fn test_min_expected_roi_negative_invalid() {
        let config = ZScoreConfig {
            min_expected_roi: -0.01,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_min_position_usdt_negative_invalid() {
        let config = ZScoreConfig {
            min_position_usdt: Decimal::new(-1, 0),
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_min_expected_roi_zero_disables() {
        let config = ZScoreConfig {
            min_expected_roi: 0.0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_min_position_usdt_zero_disables() {
        let config = ZScoreConfig {
            min_position_usdt: Decimal::ZERO,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_min_filters_from_toml() {
        let toml = r#"
[zscore]
coins = ["BTC"]
min_expected_roi = 0.20
min_position_usdt = 500.0
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.min_expected_roi, 0.20);
        assert_eq!(config.min_position_usdt, Decimal::new(500, 0));
    }

    #[test]
    fn test_min_filters_toml_default_when_omitted() {
        let toml = r#"
[zscore]
coins = ["BTC"]
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.min_expected_roi, 0.10);
        assert_eq!(config.min_position_usdt, Decimal::new(100, 0));
    }

    // --- 라이브 전용 필드 테스트 (spec/0011 Phase 4-1) ---

    #[test]
    fn test_live_fields_default() {
        let config = ZScoreConfig::default();
        // 주문 실행
        assert_eq!(config.bybit_category, "linear");
        assert_eq!(config.order_timeout_sec, 5);
        assert_eq!(config.max_retry_count, 2);
        assert_eq!(config.order_type, "limit_ioc");
        assert_eq!(config.max_slippage_pct, 0.1);
        assert_eq!(config.post_exec_pnl_gate_ratio, 0.5);
        assert_eq!(config.emergency_wide_ioc_slippage_pct, vec![2.0, 3.0, 5.0]);
        assert_eq!(config.emergency_price_fallback_margin_pct, 5.0);
        assert_eq!(config.max_dust_usdt, 5.0);
        // 리스크 관리
        assert!(config.kill_switch_enabled);
        assert_eq!(config.max_daily_loss_pct, 10.0);
        assert_eq!(config.max_drawdown_pct, 5.0);
        assert_eq!(config.max_single_loss_pct, 3.0);
        assert_eq!(config.max_daily_loss_usdt, 50.0);
        assert_eq!(config.max_drawdown_usdt, 25.0);
        assert_eq!(config.max_single_loss_usdt, 15.0);
        assert_eq!(config.max_order_size_usdt, 2000.0);
        assert_eq!(config.max_rolling_24h_loss_usdt, 80.0);
        assert_eq!(config.hwm_window_days, 7);
        assert_eq!(config.max_unrealized_loss_pct, 7.0);
        // 환율
        assert_eq!(config.max_forex_age_min, 10);
        assert_eq!(config.forex_change_alert_pct, 0.2);
        assert_eq!(config.forex_stabilization_minutes, 5);
        // 펀딩비
        assert_eq!(config.funding_block_before_min, 60);
        assert_eq!(config.funding_block_after_min, 15);
        assert!(config.funding_force_close_enabled);
        assert_eq!(config.funding_force_close_minutes_major, 15);
        assert_eq!(config.funding_force_close_minutes_alt, 30);
        assert_eq!(
            config.funding_major_coins,
            vec!["BTC".to_string(), "ETH".to_string()]
        );
        assert_eq!(config.funding_alert_ratio, 0.2);
        assert_eq!(config.funding_exclude_ratio, 0.5);
        // PendingExchangeRecovery
        assert_eq!(config.pending_recovery_timeout_hours, 2);
        // Graceful shutdown
        assert_eq!(config.shutdown_policy, "keep");
        // 텔레그램
        assert!(config.telegram_enabled);
    }

    #[test]
    fn test_live_fields_from_toml_all() {
        let toml = r#"
[zscore]
coins = ["BTC"]
bybit_category = "inverse"
order_timeout_sec = 10
max_retry_count = 3
order_type = "market"
max_slippage_pct = 0.2
post_exec_pnl_gate_ratio = 0.8
emergency_wide_ioc_slippage_pct = [1.0, 2.0]
emergency_price_fallback_margin_pct = 3.0
max_dust_usdt = 10.0
kill_switch_enabled = false
max_daily_loss_pct = 5.0
max_drawdown_pct = 3.0
max_single_loss_pct = 1.0
max_daily_loss_usdt = 30.0
max_drawdown_usdt = 15.0
max_single_loss_usdt = 10.0
max_order_size_usdt = 1000.0
max_rolling_24h_loss_usdt = 50.0
hwm_window_days = 14
max_unrealized_loss_pct = 5.0
max_forex_age_min = 5
forex_change_alert_pct = 0.5
forex_stabilization_minutes = 10
funding_block_before_min = 30
funding_block_after_min = 10
funding_force_close_enabled = false
funding_force_close_minutes_major = 10
funding_force_close_minutes_alt = 20
funding_major_coins = ["BTC"]
funding_alert_ratio = 0.1
funding_exclude_ratio = 0.3
pending_recovery_timeout_hours = 4
shutdown_policy = "close_all"
telegram_enabled = false
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.bybit_category, "inverse");
        assert_eq!(config.order_timeout_sec, 10);
        assert_eq!(config.max_retry_count, 3);
        assert_eq!(config.order_type, "market");
        assert_eq!(config.max_slippage_pct, 0.2);
        assert_eq!(config.post_exec_pnl_gate_ratio, 0.8);
        assert_eq!(config.emergency_wide_ioc_slippage_pct, vec![1.0, 2.0]);
        assert_eq!(config.emergency_price_fallback_margin_pct, 3.0);
        assert_eq!(config.max_dust_usdt, 10.0);
        assert!(!config.kill_switch_enabled);
        assert_eq!(config.max_daily_loss_pct, 5.0);
        assert_eq!(config.max_drawdown_pct, 3.0);
        assert_eq!(config.max_single_loss_pct, 1.0);
        assert_eq!(config.max_daily_loss_usdt, 30.0);
        assert_eq!(config.max_drawdown_usdt, 15.0);
        assert_eq!(config.max_single_loss_usdt, 10.0);
        assert_eq!(config.max_order_size_usdt, 1000.0);
        assert_eq!(config.max_rolling_24h_loss_usdt, 50.0);
        assert_eq!(config.hwm_window_days, 14);
        assert_eq!(config.max_unrealized_loss_pct, 5.0);
        assert_eq!(config.max_forex_age_min, 5);
        assert_eq!(config.forex_change_alert_pct, 0.5);
        assert_eq!(config.forex_stabilization_minutes, 10);
        assert_eq!(config.funding_block_before_min, 30);
        assert_eq!(config.funding_block_after_min, 10);
        assert!(!config.funding_force_close_enabled);
        assert_eq!(config.funding_force_close_minutes_major, 10);
        assert_eq!(config.funding_force_close_minutes_alt, 20);
        assert_eq!(config.funding_major_coins, vec!["BTC".to_string()]);
        assert_eq!(config.funding_alert_ratio, 0.1);
        assert_eq!(config.funding_exclude_ratio, 0.3);
        assert_eq!(config.pending_recovery_timeout_hours, 4);
        assert_eq!(config.shutdown_policy, "close_all");
        assert!(!config.telegram_enabled);
    }

    #[test]
    fn test_live_fields_toml_defaults_when_omitted() {
        // 라이브 필드를 TOML에서 생략하면 기본값 사용
        let toml = r#"
[zscore]
coins = ["BTC"]
"#;
        let config = ZScoreConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.bybit_category, "linear");
        assert_eq!(config.order_timeout_sec, 5);
        assert_eq!(config.max_retry_count, 2);
        assert_eq!(config.order_type, "limit_ioc");
        assert!(config.kill_switch_enabled);
        assert_eq!(config.max_daily_loss_pct, 10.0);
        assert_eq!(config.shutdown_policy, "keep");
        assert!(config.telegram_enabled);
    }

    #[test]
    fn test_validate_invalid_order_type() {
        let config = ZScoreConfig {
            order_type: "invalid_type".to_string(),
            ..ZScoreConfig::default()
        };
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("order_type"));
    }

    #[test]
    fn test_validate_invalid_shutdown_policy() {
        let config = ZScoreConfig {
            shutdown_policy: "destroy".to_string(),
            ..ZScoreConfig::default()
        };
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("shutdown_policy"));
    }

    #[test]
    fn test_validate_order_timeout_zero() {
        let config = ZScoreConfig {
            order_timeout_sec: 0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_negative_max_slippage_pct() {
        let config = ZScoreConfig {
            max_slippage_pct: -0.1,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_empty_emergency_slippage() {
        let config = ZScoreConfig {
            emergency_wide_ioc_slippage_pct: vec![],
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_risk_limits_positive() {
        // max_daily_loss_pct = 0.0 이면 에러
        let config = ZScoreConfig {
            max_daily_loss_pct: 0.0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());

        // max_drawdown_pct = 0.0 이면 에러
        let config = ZScoreConfig {
            max_drawdown_pct: 0.0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());

        // max_order_size_usdt = 0.0 이면 에러
        let config = ZScoreConfig {
            max_order_size_usdt: 0.0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_hwm_window_days_zero() {
        let config = ZScoreConfig {
            hwm_window_days: 0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_forex_age_zero() {
        let config = ZScoreConfig {
            max_forex_age_min: 0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_funding_exclude_le_alert() {
        // funding_exclude_ratio <= funding_alert_ratio 이면 에러
        let config = ZScoreConfig {
            funding_alert_ratio: 0.5,
            funding_exclude_ratio: 0.5,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());

        let config = ZScoreConfig {
            funding_alert_ratio: 0.5,
            funding_exclude_ratio: 0.3,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_shutdown_policy_all_valid_values() {
        for policy in &["keep", "close_all", "close_if_profitable"] {
            let config = ZScoreConfig {
                shutdown_policy: policy.to_string(),
                ..ZScoreConfig::default()
            };
            assert!(
                config.validate().is_ok(),
                "policy '{policy}' should be valid"
            );
        }
    }

    #[test]
    fn test_validate_order_type_all_valid_values() {
        for order_type in &["limit_ioc", "limit_gtc_cancel", "market"] {
            let config = ZScoreConfig {
                order_type: order_type.to_string(),
                ..ZScoreConfig::default()
            };
            assert!(
                config.validate().is_ok(),
                "order_type '{order_type}' should be valid"
            );
        }
    }

    #[test]
    fn test_validate_funding_block_before_min_zero() {
        let config = ZScoreConfig {
            funding_block_before_min: 0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_emergency_price_fallback_zero() {
        let config = ZScoreConfig {
            emergency_price_fallback_margin_pct: 0.0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_post_exec_pnl_gate_ratio_negative() {
        let config = ZScoreConfig {
            post_exec_pnl_gate_ratio: -0.1,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_forex_change_alert_pct_zero() {
        let config = ZScoreConfig {
            forex_change_alert_pct: 0.0,
            ..ZScoreConfig::default()
        };
        assert!(config.validate().is_err());
    }
}
