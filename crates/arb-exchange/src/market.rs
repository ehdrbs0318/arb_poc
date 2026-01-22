//! 마켓 코드 정규화 유틸리티.
//!
//! 이 모듈은 서로 다른 거래소의 마켓 코드 형식 간 변환을 위한
//! 유틸리티를 제공합니다.
//!
//! # 마켓 코드 형식
//!
//! 각 거래소마다 다른 마켓 코드 형식을 사용합니다:
//!
//! - **내부 형식**: `{QUOTE}-{BASE}` (예: "KRW-BTC", "USDT-ETH")
//! - **Upbit**: `{QUOTE}-{BASE}` (예: "KRW-BTC") - 내부 형식과 동일
//! - **Bithumb**: `{QUOTE}-{BASE}` (예: "KRW-BTC") - 내부 형식과 동일
//! - **Bybit**: `{BASE}{QUOTE}` (예: "BTCUSDT")
//!
//! 내부 형식은 `{QUOTE}-{BASE}` 규칙을 따릅니다:
//! - QUOTE: 지불에 사용하는 통화 (KRW, USDT 등)
//! - BASE: 구매하려는 통화 (BTC, ETH 등)

/// 지원되는 거래소 이름.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExchangeName {
    /// Upbit (한국 거래소).
    Upbit,
    /// Bithumb (한국 거래소).
    Bithumb,
    /// Bybit (글로벌 거래소).
    Bybit,
}

impl ExchangeName {
    /// 거래소 이름의 문자열 표현을 반환합니다.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Upbit => "upbit",
            Self::Bithumb => "bithumb",
            Self::Bybit => "bybit",
        }
    }

    /// 지원되는 모든 거래소 이름을 반환합니다.
    pub fn all() -> &'static [Self] {
        &[Self::Upbit, Self::Bithumb, Self::Bybit]
    }

    /// 문자열에서 거래소 이름을 파싱합니다 (편의 메서드).
    ///
    /// `FromStr::from_str`의 편의 래퍼로 `Option<Self>`를 반환합니다.
    pub fn parse(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

impl std::str::FromStr for ExchangeName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "upbit" => Ok(Self::Upbit),
            "bithumb" => Ok(Self::Bithumb),
            "bybit" => Ok(Self::Bybit),
            _ => Err(format!(
                "Unknown exchange: {}. Supported exchanges: {:?}",
                s,
                Self::all()
            )),
        }
    }
}

impl std::fmt::Display for ExchangeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 마켓 코드를 내부 형식으로 변환합니다.
///
/// # 인자
///
/// * `exchange` - 원본 거래소
/// * `market` - 거래소 형식의 마켓 코드
///
/// # 반환값
///
/// `{QUOTE}-{BASE}` 내부 형식의 마켓 코드.
///
/// # 예제
///
/// ```
/// use arb_exchange::market::{to_internal_format, ExchangeName};
///
/// assert_eq!(to_internal_format(ExchangeName::Bybit, "BTCUSDT"), "USDT-BTC");
/// assert_eq!(to_internal_format(ExchangeName::Upbit, "KRW-BTC"), "KRW-BTC");
/// ```
pub fn to_internal_format(exchange: ExchangeName, market: &str) -> String {
    match exchange {
        ExchangeName::Upbit | ExchangeName::Bithumb => {
            // 이미 내부 형식임
            market.to_uppercase()
        }
        ExchangeName::Bybit => {
            // Bybit는 "BTCUSDT" 형식을 사용, "USDT-BTC"로 변환
            bybit_to_internal(market)
        }
    }
}

/// 내부 마켓 코드를 거래소 고유 형식으로 변환합니다.
///
/// # 인자
///
/// * `exchange` - 대상 거래소
/// * `market` - 내부 형식의 마켓 코드
///
/// # 반환값
///
/// 거래소 고유 형식의 마켓 코드.
///
/// # 예제
///
/// ```
/// use arb_exchange::market::{to_exchange_format, ExchangeName};
///
/// assert_eq!(to_exchange_format(ExchangeName::Bybit, "USDT-BTC"), "BTCUSDT");
/// assert_eq!(to_exchange_format(ExchangeName::Upbit, "KRW-BTC"), "KRW-BTC");
/// ```
pub fn to_exchange_format(exchange: ExchangeName, market: &str) -> String {
    match exchange {
        ExchangeName::Upbit | ExchangeName::Bithumb => {
            // 이미 내부 형식임
            market.to_uppercase()
        }
        ExchangeName::Bybit => {
            // "USDT-BTC"를 "BTCUSDT"로 변환
            internal_to_bybit(market)
        }
    }
}

/// 거래소 형식 간 직접 변환합니다.
///
/// # 인자
///
/// * `from_exchange` - 원본 거래소
/// * `to_exchange` - 대상 거래소
/// * `market` - 원본 거래소 형식의 마켓 코드
///
/// # 반환값
///
/// 대상 거래소 형식의 마켓 코드.
pub fn convert_market_code(
    from_exchange: ExchangeName,
    to_exchange: ExchangeName,
    market: &str,
) -> String {
    let internal = to_internal_format(from_exchange, market);
    to_exchange_format(to_exchange, &internal)
}

/// 내부 마켓 코드를 구성 요소로 파싱합니다.
///
/// # 인자
///
/// * `market` - 내부 형식의 마켓 코드 (예: "KRW-BTC")
///
/// # 반환값
///
/// (quote_currency, base_currency) 튜플, 파싱 실패 시 None.
///
/// # 예제
///
/// ```
/// use arb_exchange::market::parse_market_code;
///
/// let (quote, base) = parse_market_code("KRW-BTC").unwrap();
/// assert_eq!(quote, "KRW");
/// assert_eq!(base, "BTC");
/// ```
pub fn parse_market_code(market: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = market.split('-').collect();
    if parts.len() == 2 {
        Some((parts[0].to_uppercase(), parts[1].to_uppercase()))
    } else {
        None
    }
}

/// 구성 요소로부터 내부 마켓 코드를 생성합니다.
///
/// # 인자
///
/// * `quote` - Quote 통화 (예: "KRW", "USDT")
/// * `base` - Base 통화 (예: "BTC", "ETH")
///
/// # 반환값
///
/// 내부 형식의 마켓 코드.
pub fn create_market_code(quote: &str, base: &str) -> String {
    format!("{}-{}", quote.to_uppercase(), base.to_uppercase())
}

/// 마켓 코드에서 base 통화를 추출합니다.
///
/// # 인자
///
/// * `market` - 내부 형식의 마켓 코드
///
/// # 반환값
///
/// Base 통화, 파싱 실패 시 None.
pub fn get_base_currency(market: &str) -> Option<String> {
    parse_market_code(market).map(|(_, base)| base)
}

/// 마켓 코드에서 quote 통화를 추출합니다.
///
/// # 인자
///
/// * `market` - 내부 형식의 마켓 코드
///
/// # 반환값
///
/// Quote 통화, 파싱 실패 시 None.
pub fn get_quote_currency(market: &str) -> Option<String> {
    parse_market_code(market).map(|(quote, _)| quote)
}

/// 거래소 간 base 통화를 정규화합니다.
///
/// 일부 거래소는 동일한 자산에 대해 다른 심볼을 사용합니다
/// (예: Bitcoin의 경우 "XBT" vs "BTC").
///
/// # 인자
///
/// * `currency` - 정규화할 통화 심볼
///
/// # 반환값
///
/// 정규화된 통화 심볼.
pub fn normalize_currency(currency: &str) -> &str {
    match currency.to_uppercase().as_str() {
        "XBT" => "BTC",
        // 필요에 따라 매핑 추가
        _ => currency,
    }
}

// ==================== Bybit 변환 함수 ====================

/// Bybit의 일반적인 quote 통화 목록, 감지 우선순위 순서.
const BYBIT_QUOTE_CURRENCIES: &[&str] = &["USDT", "USDC", "BTC", "ETH", "EUR", "DAI"];

/// Bybit 심볼 형식을 내부 형식으로 변환합니다.
///
/// Bybit는 "BTCUSDT" 형식을 사용, "USDT-BTC"가 필요함.
fn bybit_to_internal(symbol: &str) -> String {
    let symbol = symbol.to_uppercase();

    for quote in BYBIT_QUOTE_CURRENCIES {
        if let Some(base) = symbol.strip_suffix(quote) {
            return format!("{}-{}", quote, base);
        }
    }

    // 폴백: 그대로 반환 (이미 내부 형식일 수 있음)
    symbol
}

/// 내부 형식을 Bybit 심볼 형식으로 변환합니다.
///
/// 내부 "USDT-BTC" -> Bybit "BTCUSDT"
fn internal_to_bybit(market: &str) -> String {
    if let Some((quote, base)) = market.split_once('-') {
        format!("{}{}", base.to_uppercase(), quote.to_uppercase())
    } else {
        market.to_uppercase()
    }
}

/// 편리한 마켓 코드 생성을 위한 빌더.
#[derive(Debug, Clone)]
pub struct MarketCodeBuilder {
    base: String,
    quote: String,
}

impl MarketCodeBuilder {
    /// 새로운 마켓 코드 빌더를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `base` - Base 통화 (거래되는 자산)
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into().to_uppercase(),
            quote: String::new(),
        }
    }

    /// Quote 통화를 설정합니다.
    ///
    /// # 인자
    ///
    /// * `quote` - Quote 통화 (base의 가격을 표시하는 통화)
    pub fn quote(mut self, quote: impl Into<String>) -> Self {
        self.quote = quote.into().to_uppercase();
        self
    }

    /// 지정된 거래소용 마켓 코드를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `exchange` - 대상 거래소
    pub fn build_for(&self, exchange: ExchangeName) -> String {
        let internal = format!("{}-{}", self.quote, self.base);
        to_exchange_format(exchange, &internal)
    }

    /// 내부 형식의 마켓 코드를 생성합니다.
    pub fn build(&self) -> String {
        format!("{}-{}", self.quote, self.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_name_from_str() {
        assert_eq!("upbit".parse::<ExchangeName>().ok(), Some(ExchangeName::Upbit));
        assert_eq!("UPBIT".parse::<ExchangeName>().ok(), Some(ExchangeName::Upbit));
        assert_eq!(
            "bithumb".parse::<ExchangeName>().ok(),
            Some(ExchangeName::Bithumb)
        );
        assert_eq!("bybit".parse::<ExchangeName>().ok(), Some(ExchangeName::Bybit));
        assert!("unknown".parse::<ExchangeName>().is_err());
    }

    #[test]
    fn test_exchange_name_as_str() {
        assert_eq!(ExchangeName::Upbit.as_str(), "upbit");
        assert_eq!(ExchangeName::Bithumb.as_str(), "bithumb");
        assert_eq!(ExchangeName::Bybit.as_str(), "bybit");
    }

    #[test]
    fn test_to_internal_format_upbit() {
        assert_eq!(to_internal_format(ExchangeName::Upbit, "KRW-BTC"), "KRW-BTC");
        assert_eq!(to_internal_format(ExchangeName::Upbit, "krw-btc"), "KRW-BTC");
    }

    #[test]
    fn test_to_internal_format_bithumb() {
        assert_eq!(
            to_internal_format(ExchangeName::Bithumb, "KRW-BTC"),
            "KRW-BTC"
        );
    }

    #[test]
    fn test_to_internal_format_bybit() {
        assert_eq!(
            to_internal_format(ExchangeName::Bybit, "BTCUSDT"),
            "USDT-BTC"
        );
        assert_eq!(
            to_internal_format(ExchangeName::Bybit, "ETHUSDC"),
            "USDC-ETH"
        );
        assert_eq!(
            to_internal_format(ExchangeName::Bybit, "ETHBTC"),
            "BTC-ETH"
        );
    }

    #[test]
    fn test_to_exchange_format_upbit() {
        assert_eq!(
            to_exchange_format(ExchangeName::Upbit, "KRW-BTC"),
            "KRW-BTC"
        );
    }

    #[test]
    fn test_to_exchange_format_bybit() {
        assert_eq!(
            to_exchange_format(ExchangeName::Bybit, "USDT-BTC"),
            "BTCUSDT"
        );
        assert_eq!(
            to_exchange_format(ExchangeName::Bybit, "USDC-ETH"),
            "ETHUSDC"
        );
    }

    #[test]
    fn test_convert_market_code() {
        // Upbit에서 Bybit로
        assert_eq!(
            convert_market_code(ExchangeName::Upbit, ExchangeName::Bybit, "USDT-BTC"),
            "BTCUSDT"
        );

        // Bybit에서 Upbit로
        assert_eq!(
            convert_market_code(ExchangeName::Bybit, ExchangeName::Upbit, "BTCUSDT"),
            "USDT-BTC"
        );

        // 동일한 거래소
        assert_eq!(
            convert_market_code(ExchangeName::Upbit, ExchangeName::Upbit, "KRW-BTC"),
            "KRW-BTC"
        );
    }

    #[test]
    fn test_parse_market_code() {
        let (quote, base) = parse_market_code("KRW-BTC").unwrap();
        assert_eq!(quote, "KRW");
        assert_eq!(base, "BTC");

        assert!(parse_market_code("INVALID").is_none());
    }

    #[test]
    fn test_create_market_code() {
        assert_eq!(create_market_code("KRW", "BTC"), "KRW-BTC");
        assert_eq!(create_market_code("usdt", "eth"), "USDT-ETH");
    }

    #[test]
    fn test_get_base_currency() {
        assert_eq!(get_base_currency("KRW-BTC"), Some("BTC".to_string()));
        assert_eq!(get_base_currency("USDT-ETH"), Some("ETH".to_string()));
    }

    #[test]
    fn test_get_quote_currency() {
        assert_eq!(get_quote_currency("KRW-BTC"), Some("KRW".to_string()));
        assert_eq!(get_quote_currency("USDT-ETH"), Some("USDT".to_string()));
    }

    #[test]
    fn test_normalize_currency() {
        assert_eq!(normalize_currency("XBT"), "BTC");
        assert_eq!(normalize_currency("BTC"), "BTC");
        assert_eq!(normalize_currency("ETH"), "ETH");
    }

    #[test]
    fn test_market_code_builder() {
        let market = MarketCodeBuilder::new("BTC").quote("USDT").build();
        assert_eq!(market, "USDT-BTC");

        let bybit_market = MarketCodeBuilder::new("BTC")
            .quote("USDT")
            .build_for(ExchangeName::Bybit);
        assert_eq!(bybit_market, "BTCUSDT");
    }
}
