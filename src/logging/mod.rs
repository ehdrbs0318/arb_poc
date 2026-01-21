//! # Logging Module
//!
//! 차익거래 시스템을 위한 구조화된 로깅 모듈입니다.
//!
//! `tracing` 크레이트를 기반으로 하며, 다음 기능을 제공합니다:
//! - 환경 변수 또는 설정 파일 기반 로그 레벨 설정
//! - 거래소별 span 생성 유틸리티
//! - 주문 및 거래 이벤트 로깅 헬퍼
//! - 파일/콘솔 출력 설정
//!
//! ## 사용 예시
//!
//! ```rust,no_run
//! use arb_poc::logging::{init_logging, LogConfig, exchange_span};
//!
//! // 로깅 초기화
//! let config = LogConfig::default();
//! init_logging(&config).expect("로깅 초기화 실패");
//!
//! // 거래소별 span 사용
//! let _guard = exchange_span("upbit", Some("KRW-BTC"));
//! tracing::info!("시세 조회 중");
//! ```

use std::env;
use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;

use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan, time::ChronoLocal},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use crate::exchange::types::Order;

/// 로깅 초기화 상태를 추적하는 전역 변수
static LOGGING_INITIALIZED: OnceLock<bool> = OnceLock::new();

/// 로깅 설정 구조체
///
/// 환경 변수 또는 설정 파일에서 로깅 관련 설정을 로드합니다.
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// 로그 레벨 (trace, debug, info, warn, error)
    pub level: String,

    /// 콘솔 출력 활성화 여부
    pub console_enabled: bool,

    /// 파일 출력 활성화 여부
    pub file_enabled: bool,

    /// 로그 파일 경로 (file_enabled가 true일 때 사용)
    pub file_path: Option<PathBuf>,

    /// 타임스탬프 포맷
    pub timestamp_format: String,

    /// span 이벤트 표시 여부 (NEW, CLOSE 등)
    pub show_span_events: bool,

    /// 대상(target) 표시 여부
    pub show_target: bool,

    /// 파일/라인 정보 표시 여부
    pub show_file_line: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            console_enabled: true,
            file_enabled: false,
            file_path: None,
            timestamp_format: "%Y-%m-%d %H:%M:%S%.3f".to_string(),
            show_span_events: false,
            show_target: true,
            show_file_line: false,
        }
    }
}

impl LogConfig {
    /// 환경 변수에서 로깅 설정을 로드합니다.
    ///
    /// 지원하는 환경 변수:
    /// - `RUST_LOG`: 로그 레벨 (기본값: info)
    /// - `LOG_FILE`: 로그 파일 경로 (설정 시 파일 로깅 활성화)
    /// - `LOG_CONSOLE`: 콘솔 출력 활성화 ("true" 또는 "false", 기본값: true)
    /// - `LOG_SHOW_TARGET`: 대상 표시 ("true" 또는 "false", 기본값: true)
    /// - `LOG_SHOW_FILE_LINE`: 파일/라인 표시 ("true" 또는 "false", 기본값: false)
    pub fn from_env() -> Self {
        let file_path = env::var("LOG_FILE").ok().map(PathBuf::from);
        let file_enabled = file_path.is_some();

        Self {
            level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            console_enabled: env::var("LOG_CONSOLE")
                .map(|v| v.to_lowercase() != "false")
                .unwrap_or(true),
            file_enabled,
            file_path,
            timestamp_format: "%Y-%m-%d %H:%M:%S%.3f".to_string(),
            show_span_events: env::var("LOG_SHOW_SPAN_EVENTS")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
            show_target: env::var("LOG_SHOW_TARGET")
                .map(|v| v.to_lowercase() != "false")
                .unwrap_or(true),
            show_file_line: env::var("LOG_SHOW_FILE_LINE")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
        }
    }

    /// 개발 환경용 설정을 반환합니다.
    ///
    /// - 로그 레벨: debug
    /// - 콘솔 출력: 활성화
    /// - span 이벤트: 표시
    /// - 파일/라인 정보: 표시
    pub fn development() -> Self {
        Self {
            level: "debug".to_string(),
            console_enabled: true,
            file_enabled: false,
            file_path: None,
            timestamp_format: "%Y-%m-%d %H:%M:%S%.3f".to_string(),
            show_span_events: true,
            show_target: true,
            show_file_line: true,
        }
    }

    /// 프로덕션 환경용 설정을 반환합니다.
    ///
    /// - 로그 레벨: info
    /// - 콘솔 출력: 활성화
    /// - 파일 출력: 활성화 (지정된 경로)
    /// - span 이벤트: 비표시
    /// - 파일/라인 정보: 비표시
    pub fn production(log_file_path: PathBuf) -> Self {
        Self {
            level: "info".to_string(),
            console_enabled: true,
            file_enabled: true,
            file_path: Some(log_file_path),
            timestamp_format: "%Y-%m-%d %H:%M:%S%.3f".to_string(),
            show_span_events: false,
            show_target: true,
            show_file_line: false,
        }
    }
}

/// 로깅 초기화 에러 타입
#[derive(Debug, thiserror::Error)]
pub enum LogError {
    /// 로그 파일 생성 실패
    #[error("로그 파일 생성 실패: {0}")]
    FileCreation(#[from] io::Error),

    /// EnvFilter 파싱 실패
    #[error("로그 필터 파싱 실패: {0}")]
    FilterParse(String),

    /// 이미 초기화됨
    #[error("로깅이 이미 초기화되었습니다")]
    AlreadyInitialized,

    /// 구독자 설정 실패
    #[error("로깅 구독자 설정 실패: {0}")]
    SubscriberSetup(String),
}

/// 로깅 시스템을 초기화합니다.
///
/// 이 함수는 애플리케이션 시작 시 한 번만 호출해야 합니다.
/// 중복 호출 시 `LogError::AlreadyInitialized` 에러를 반환합니다.
///
/// # Arguments
///
/// * `config` - 로깅 설정
///
/// # Returns
///
/// 성공 시 `Ok(())`, 실패 시 `Err(LogError)`
///
/// # Examples
///
/// ```rust,no_run
/// use arb_poc::logging::{init_logging, LogConfig};
///
/// let config = LogConfig::from_env();
/// init_logging(&config).expect("로깅 초기화 실패");
/// ```
pub fn init_logging(config: &LogConfig) -> Result<(), LogError> {
    // 이미 초기화되었는지 확인
    if LOGGING_INITIALIZED.get().is_some() {
        return Err(LogError::AlreadyInitialized);
    }

    // EnvFilter 생성
    let filter = EnvFilter::try_new(&config.level)
        .map_err(|e| LogError::FilterParse(e.to_string()))?;

    // span 이벤트 설정
    let span_events = if config.show_span_events {
        FmtSpan::NEW | FmtSpan::CLOSE
    } else {
        FmtSpan::NONE
    };

    // 타임스탬프 포맷터 생성
    let timer = ChronoLocal::new(config.timestamp_format.clone());

    // 콘솔 레이어 생성
    if config.console_enabled && config.file_enabled {
        // 콘솔 + 파일 출력
        let file_path = config.file_path.as_ref()
            .ok_or_else(|| LogError::FilterParse("파일 경로가 지정되지 않았습니다".to_string()))?;

        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        let file_layer = fmt::layer()
            .with_timer(timer.clone())
            .with_target(config.show_target)
            .with_file(config.show_file_line)
            .with_line_number(config.show_file_line)
            .with_span_events(span_events.clone())
            .with_writer(file)
            .with_ansi(false);

        let console_layer = fmt::layer()
            .with_timer(timer)
            .with_target(config.show_target)
            .with_file(config.show_file_line)
            .with_line_number(config.show_file_line)
            .with_span_events(span_events)
            .with_writer(io::stdout);

        tracing_subscriber::registry()
            .with(filter)
            .with(console_layer)
            .with(file_layer)
            .try_init()
            .map_err(|e| LogError::SubscriberSetup(e.to_string()))?;
    } else if config.file_enabled {
        // 파일 출력만
        let file_path = config.file_path.as_ref()
            .ok_or_else(|| LogError::FilterParse("파일 경로가 지정되지 않았습니다".to_string()))?;

        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        let file_layer = fmt::layer()
            .with_timer(timer)
            .with_target(config.show_target)
            .with_file(config.show_file_line)
            .with_line_number(config.show_file_line)
            .with_span_events(span_events)
            .with_writer(file)
            .with_ansi(false);

        tracing_subscriber::registry()
            .with(filter)
            .with(file_layer)
            .try_init()
            .map_err(|e| LogError::SubscriberSetup(e.to_string()))?;
    } else {
        // 콘솔 출력만 (기본)
        let console_layer = fmt::layer()
            .with_timer(timer)
            .with_target(config.show_target)
            .with_file(config.show_file_line)
            .with_line_number(config.show_file_line)
            .with_span_events(span_events)
            .with_writer(io::stdout);

        tracing_subscriber::registry()
            .with(filter)
            .with(console_layer)
            .try_init()
            .map_err(|e| LogError::SubscriberSetup(e.to_string()))?;
    }

    // 초기화 완료 표시
    let _ = LOGGING_INITIALIZED.set(true);

    info!(
        level = %config.level,
        console = config.console_enabled,
        file = config.file_enabled,
        "로깅 시스템 초기화 완료"
    );

    Ok(())
}

/// 거래소별 span을 생성합니다.
///
/// 이 함수는 특정 거래소에 대한 작업을 추적하기 위한 span을 생성합니다.
/// 반환된 `EnteredSpan`은 스코프를 벗어날 때 자동으로 종료됩니다.
///
/// # Arguments
///
/// * `exchange` - 거래소 이름 (예: "upbit", "binance", "bithumb")
/// * `symbol` - 거래 심볼 (선택적, 예: "KRW-BTC", "BTCUSDT")
///
/// # Returns
///
/// 진입된 span guard
///
/// # Examples
///
/// ```rust,no_run
/// use arb_poc::logging::exchange_span;
///
/// {
///     let _guard = exchange_span("upbit", Some("KRW-BTC"));
///     // 이 스코프 내의 모든 로그는 exchange=upbit, symbol=KRW-BTC를 포함
///     tracing::info!("시세 조회 중");
/// }
/// // span이 자동으로 종료됨
/// ```
#[must_use = "span guard는 스코프 내에서 유지되어야 합니다"]
pub fn exchange_span(exchange: &str, symbol: Option<&str>) -> tracing::span::EnteredSpan {
    match symbol {
        Some(sym) => {
            tracing::info_span!("exchange", exchange = %exchange, symbol = %sym).entered()
        }
        None => {
            tracing::info_span!("exchange", exchange = %exchange).entered()
        }
    }
}

/// 특정 작업에 대한 span을 생성합니다.
///
/// # Arguments
///
/// * `operation` - 작업 이름 (예: "fetch_orderbook", "place_order")
/// * `exchange` - 거래소 이름
/// * `symbol` - 거래 심볼 (선택적)
///
/// # Returns
///
/// 진입된 span guard
#[must_use = "span guard는 스코프 내에서 유지되어야 합니다"]
pub fn operation_span(
    operation: &str,
    exchange: &str,
    symbol: Option<&str>,
) -> tracing::span::EnteredSpan {
    match symbol {
        Some(sym) => {
            tracing::info_span!(
                "operation",
                op = %operation,
                exchange = %exchange,
                symbol = %sym
            ).entered()
        }
        None => {
            tracing::info_span!(
                "operation",
                op = %operation,
                exchange = %exchange
            ).entered()
        }
    }
}

/// 주문 이벤트를 로깅합니다.
///
/// 주문 생성, 체결, 취소 등의 이벤트를 구조화된 형식으로 로깅합니다.
///
/// # Arguments
///
/// * `event` - 이벤트 설명 (예: "주문 생성", "주문 체결", "주문 취소")
/// * `order` - 주문 정보
///
/// # Examples
///
/// ```rust,no_run
/// use arb_poc::logging::log_order_event;
/// use arb_poc::exchange::types::{Order, OrderSide, OrderType, OrderStatus};
/// use rust_decimal::Decimal;
/// use chrono::Utc;
///
/// let order = Order {
///     id: "order123".to_string(),
///     market: "KRW-BTC".to_string(),
///     side: OrderSide::Buy,
///     order_type: OrderType::Limit,
///     status: OrderStatus::Wait,
///     volume: Decimal::from(1),
///     remaining_volume: Decimal::from(1),
///     executed_volume: Decimal::ZERO,
///     price: Some(Decimal::from(50000000)),
///     avg_price: None,
///     paid_fee: Decimal::ZERO,
///     created_at: Utc::now(),
///     identifier: None,
/// };
///
/// log_order_event("주문 생성", &order);
/// ```
pub fn log_order_event(event: &str, order: &Order) {
    info!(
        event = %event,
        order_id = %order.id,
        market = %order.market,
        side = ?order.side,
        order_type = ?order.order_type,
        price = ?order.price,
        volume = %order.volume,
        executed = %order.executed_volume,
        remaining = %order.remaining_volume,
        status = ?order.status,
        "주문 이벤트"
    );
}

/// 거래 체결 이벤트를 로깅합니다.
///
/// # Arguments
///
/// * `exchange` - 거래소 이름
/// * `symbol` - 거래 심볼
/// * `side` - 매수/매도
/// * `price` - 체결 가격
/// * `quantity` - 체결 수량
/// * `order_id` - 주문 ID
pub fn log_trade_execution(
    exchange: &str,
    symbol: &str,
    side: &str,
    price: rust_decimal::Decimal,
    quantity: rust_decimal::Decimal,
    order_id: &str,
) {
    info!(
        exchange = %exchange,
        symbol = %symbol,
        side = %side,
        price = %price,
        quantity = %quantity,
        order_id = %order_id,
        "거래 체결"
    );
}

/// 가격 정보를 로깅합니다 (debug 레벨).
///
/// # Arguments
///
/// * `exchange` - 거래소 이름
/// * `symbol` - 거래 심볼
/// * `bid` - 매수 호가
/// * `ask` - 매도 호가
/// * `last` - 최근 체결가 (선택적)
pub fn log_price_update(
    exchange: &str,
    symbol: &str,
    bid: rust_decimal::Decimal,
    ask: rust_decimal::Decimal,
    last: Option<rust_decimal::Decimal>,
) {
    debug!(
        exchange = %exchange,
        symbol = %symbol,
        bid = %bid,
        ask = %ask,
        last = ?last,
        spread_bps = %((ask - bid) / bid * rust_decimal::Decimal::from(10000)),
        "가격 업데이트"
    );
}

/// 차익거래 기회를 로깅합니다.
///
/// # Arguments
///
/// * `buy_exchange` - 매수 거래소
/// * `sell_exchange` - 매도 거래소
/// * `symbol` - 거래 심볼
/// * `buy_price` - 매수 가격
/// * `sell_price` - 매도 가격
/// * `profit_bps` - 예상 수익률 (basis points)
pub fn log_arbitrage_opportunity(
    buy_exchange: &str,
    sell_exchange: &str,
    symbol: &str,
    buy_price: rust_decimal::Decimal,
    sell_price: rust_decimal::Decimal,
    profit_bps: rust_decimal::Decimal,
) {
    if profit_bps > rust_decimal::Decimal::ZERO {
        info!(
            buy_exchange = %buy_exchange,
            sell_exchange = %sell_exchange,
            symbol = %symbol,
            buy_price = %buy_price,
            sell_price = %sell_price,
            profit_bps = %profit_bps,
            "차익거래 기회 발견"
        );
    } else {
        trace!(
            buy_exchange = %buy_exchange,
            sell_exchange = %sell_exchange,
            symbol = %symbol,
            buy_price = %buy_price,
            sell_price = %sell_price,
            profit_bps = %profit_bps,
            "차익거래 기회 없음"
        );
    }
}

/// API 요청을 로깅합니다 (debug 레벨).
///
/// # Arguments
///
/// * `exchange` - 거래소 이름
/// * `endpoint` - API 엔드포인트
/// * `method` - HTTP 메서드
pub fn log_api_request(exchange: &str, endpoint: &str, method: &str) {
    debug!(
        exchange = %exchange,
        endpoint = %endpoint,
        method = %method,
        "API 요청"
    );
}

/// API 응답을 로깅합니다 (debug 레벨).
///
/// # Arguments
///
/// * `exchange` - 거래소 이름
/// * `endpoint` - API 엔드포인트
/// * `status` - HTTP 상태 코드
/// * `latency_ms` - 응답 지연 시간 (밀리초)
pub fn log_api_response(exchange: &str, endpoint: &str, status: u16, latency_ms: u64) {
    debug!(
        exchange = %exchange,
        endpoint = %endpoint,
        status = status,
        latency_ms = latency_ms,
        "API 응답"
    );
}

/// API 에러를 로깅합니다 (error 레벨).
///
/// # Arguments
///
/// * `exchange` - 거래소 이름
/// * `endpoint` - API 엔드포인트
/// * `error` - 에러 메시지
pub fn log_api_error(exchange: &str, endpoint: &str, error: &str) {
    error!(
        exchange = %exchange,
        endpoint = %endpoint,
        error = %error,
        "API 에러"
    );
}

/// WebSocket 연결 상태를 로깅합니다.
///
/// # Arguments
///
/// * `exchange` - 거래소 이름
/// * `status` - 연결 상태 ("connected", "disconnected", "reconnecting")
/// * `url` - WebSocket URL (선택적)
pub fn log_websocket_status(exchange: &str, status: &str, url: Option<&str>) {
    match status {
        "connected" => info!(
            exchange = %exchange,
            status = %status,
            url = ?url,
            "WebSocket 연결됨"
        ),
        "disconnected" => warn!(
            exchange = %exchange,
            status = %status,
            url = ?url,
            "WebSocket 연결 해제"
        ),
        "reconnecting" => info!(
            exchange = %exchange,
            status = %status,
            url = ?url,
            "WebSocket 재연결 중"
        ),
        _ => debug!(
            exchange = %exchange,
            status = %status,
            url = ?url,
            "WebSocket 상태 변경"
        ),
    }
}

/// 시스템 시작을 로깅합니다.
///
/// # Arguments
///
/// * `version` - 애플리케이션 버전
/// * `exchanges` - 활성화된 거래소 목록
pub fn log_system_start(version: &str, exchanges: &[&str]) {
    info!(
        version = %version,
        exchanges = ?exchanges,
        "차익거래 시스템 시작"
    );
}

/// 시스템 종료를 로깅합니다.
///
/// # Arguments
///
/// * `reason` - 종료 사유
pub fn log_system_shutdown(reason: &str) {
    info!(
        reason = %reason,
        "차익거래 시스템 종료"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();

        assert!(config.console_enabled);
        assert!(!config.file_enabled);
        assert!(config.file_path.is_none());
        assert!(config.show_target);
        assert!(!config.show_file_line);
    }

    #[test]
    fn test_log_config_development() {
        let config = LogConfig::development();

        assert_eq!(config.level, "debug");
        assert!(config.console_enabled);
        assert!(!config.file_enabled);
        assert!(config.show_span_events);
        assert!(config.show_file_line);
    }

    #[test]
    fn test_log_config_production() {
        let path = PathBuf::from("/tmp/test.log");
        let config = LogConfig::production(path.clone());

        assert_eq!(config.level, "info");
        assert!(config.console_enabled);
        assert!(config.file_enabled);
        assert_eq!(config.file_path, Some(path));
        assert!(!config.show_span_events);
        assert!(!config.show_file_line);
    }

    #[test]
    fn test_exchange_span_with_symbol() {
        // span 생성 테스트 (실제 로깅은 하지 않음)
        let _guard = exchange_span("upbit", Some("KRW-BTC"));
        // span이 정상적으로 생성되고 drop되는지 확인
    }

    #[test]
    fn test_exchange_span_without_symbol() {
        let _guard = exchange_span("binance", None);
    }

    #[test]
    fn test_operation_span() {
        let _guard = operation_span("fetch_orderbook", "upbit", Some("KRW-BTC"));
    }

    // 참고: log_* 함수들은 실제 로깅 시스템이 초기화되어야 동작하므로
    // 여기서는 함수 시그니처와 컴파일이 되는지만 확인합니다.
    // 통합 테스트에서 실제 로깅 동작을 검증할 수 있습니다.
}
