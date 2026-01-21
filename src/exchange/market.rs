//! Market code normalization utilities.
//!
//! This module provides utilities for converting between different exchange's
//! market code formats.
//!
//! # Market Code Formats
//!
//! Different exchanges use different formats for market codes:
//!
//! - **Internal format**: `{QUOTE}-{BASE}` (e.g., "KRW-BTC", "USDT-ETH")
//! - **Upbit**: `{QUOTE}-{BASE}` (e.g., "KRW-BTC") - same as internal
//! - **Bithumb**: `{QUOTE}-{BASE}` (e.g., "KRW-BTC") - same as internal
//! - **Bybit**: `{BASE}{QUOTE}` (e.g., "BTCUSDT")
//!
//! The internal format follows the convention: `{QUOTE}-{BASE}` where:
//! - QUOTE: The currency you're paying with (KRW, USDT, etc.)
//! - BASE: The currency you're buying (BTC, ETH, etc.)

use crate::exchange::factory::ExchangeName;

/// Converts a market code to the internal format.
///
/// # Arguments
///
/// * `exchange` - The source exchange
/// * `market` - The market code in the exchange's format
///
/// # Returns
///
/// The market code in internal format `{QUOTE}-{BASE}`.
///
/// # Example
///
/// ```ignore
/// use arb_poc::exchange::market::to_internal_format;
/// use arb_poc::exchange::factory::ExchangeName;
///
/// assert_eq!(to_internal_format(ExchangeName::Bybit, "BTCUSDT"), "USDT-BTC");
/// assert_eq!(to_internal_format(ExchangeName::Upbit, "KRW-BTC"), "KRW-BTC");
/// ```
pub fn to_internal_format(exchange: ExchangeName, market: &str) -> String {
    match exchange {
        ExchangeName::Upbit | ExchangeName::Bithumb => {
            // Already in internal format
            market.to_uppercase()
        }
        ExchangeName::Bybit => {
            // Bybit uses "BTCUSDT" format, convert to "USDT-BTC"
            bybit_to_internal(market)
        }
    }
}

/// Converts an internal market code to the exchange's native format.
///
/// # Arguments
///
/// * `exchange` - The target exchange
/// * `market` - The market code in internal format
///
/// # Returns
///
/// The market code in the exchange's native format.
///
/// # Example
///
/// ```ignore
/// use arb_poc::exchange::market::to_exchange_format;
/// use arb_poc::exchange::factory::ExchangeName;
///
/// assert_eq!(to_exchange_format(ExchangeName::Bybit, "USDT-BTC"), "BTCUSDT");
/// assert_eq!(to_exchange_format(ExchangeName::Upbit, "KRW-BTC"), "KRW-BTC");
/// ```
pub fn to_exchange_format(exchange: ExchangeName, market: &str) -> String {
    match exchange {
        ExchangeName::Upbit | ExchangeName::Bithumb => {
            // Already in internal format
            market.to_uppercase()
        }
        ExchangeName::Bybit => {
            // Convert "USDT-BTC" to "BTCUSDT"
            internal_to_bybit(market)
        }
    }
}

/// Converts between exchange formats directly.
///
/// # Arguments
///
/// * `from_exchange` - The source exchange
/// * `to_exchange` - The target exchange
/// * `market` - The market code in the source exchange's format
///
/// # Returns
///
/// The market code in the target exchange's format.
pub fn convert_market_code(
    from_exchange: ExchangeName,
    to_exchange: ExchangeName,
    market: &str,
) -> String {
    let internal = to_internal_format(from_exchange, market);
    to_exchange_format(to_exchange, &internal)
}

/// Parses an internal market code into its components.
///
/// # Arguments
///
/// * `market` - Market code in internal format (e.g., "KRW-BTC")
///
/// # Returns
///
/// A tuple of (quote_currency, base_currency), or None if parsing fails.
///
/// # Example
///
/// ```ignore
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

/// Creates an internal market code from components.
///
/// # Arguments
///
/// * `quote` - Quote currency (e.g., "KRW", "USDT")
/// * `base` - Base currency (e.g., "BTC", "ETH")
///
/// # Returns
///
/// The market code in internal format.
pub fn create_market_code(quote: &str, base: &str) -> String {
    format!("{}-{}", quote.to_uppercase(), base.to_uppercase())
}

/// Extracts the base currency from a market code.
///
/// # Arguments
///
/// * `market` - Market code in internal format
///
/// # Returns
///
/// The base currency, or None if parsing fails.
pub fn get_base_currency(market: &str) -> Option<String> {
    parse_market_code(market).map(|(_, base)| base)
}

/// Extracts the quote currency from a market code.
///
/// # Arguments
///
/// * `market` - Market code in internal format
///
/// # Returns
///
/// The quote currency, or None if parsing fails.
pub fn get_quote_currency(market: &str) -> Option<String> {
    parse_market_code(market).map(|(quote, _)| quote)
}

/// Normalizes a base currency across exchanges.
///
/// Some exchanges use different symbols for the same asset
/// (e.g., "XBT" vs "BTC" for Bitcoin).
///
/// # Arguments
///
/// * `currency` - The currency symbol to normalize
///
/// # Returns
///
/// The normalized currency symbol.
pub fn normalize_currency(currency: &str) -> &str {
    match currency.to_uppercase().as_str() {
        "XBT" => "BTC",
        // Add more mappings as needed
        _ => currency,
    }
}

// ==================== Bybit Conversion Functions ====================

/// Common quote currencies for Bybit, in order of preference for detection.
const BYBIT_QUOTE_CURRENCIES: &[&str] = &["USDT", "USDC", "BTC", "ETH", "EUR", "DAI"];

/// Converts Bybit symbol format to internal format.
///
/// Bybit uses "BTCUSDT" format, we need "USDT-BTC".
fn bybit_to_internal(symbol: &str) -> String {
    let symbol = symbol.to_uppercase();

    for quote in BYBIT_QUOTE_CURRENCIES {
        if let Some(base) = symbol.strip_suffix(quote) {
            return format!("{}-{}", quote, base);
        }
    }

    // Fallback: return as-is (might already be in internal format)
    symbol
}

/// Converts internal format to Bybit symbol format.
///
/// Internal "USDT-BTC" -> Bybit "BTCUSDT"
fn internal_to_bybit(market: &str) -> String {
    if let Some((quote, base)) = market.split_once('-') {
        format!("{}{}", base.to_uppercase(), quote.to_uppercase())
    } else {
        market.to_uppercase()
    }
}

/// Market code builder for convenient market code creation.
#[derive(Debug, Clone)]
pub struct MarketCodeBuilder {
    base: String,
    quote: String,
}

impl MarketCodeBuilder {
    /// Creates a new market code builder.
    ///
    /// # Arguments
    ///
    /// * `base` - Base currency (the asset being traded)
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into().to_uppercase(),
            quote: String::new(),
        }
    }

    /// Sets the quote currency.
    ///
    /// # Arguments
    ///
    /// * `quote` - Quote currency (the currency used to price the base)
    pub fn quote(mut self, quote: impl Into<String>) -> Self {
        self.quote = quote.into().to_uppercase();
        self
    }

    /// Builds the market code for the specified exchange.
    ///
    /// # Arguments
    ///
    /// * `exchange` - Target exchange
    pub fn build_for(&self, exchange: ExchangeName) -> String {
        let internal = format!("{}-{}", self.quote, self.base);
        to_exchange_format(exchange, &internal)
    }

    /// Builds the market code in internal format.
    pub fn build(&self) -> String {
        format!("{}-{}", self.quote, self.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // Upbit to Bybit
        assert_eq!(
            convert_market_code(ExchangeName::Upbit, ExchangeName::Bybit, "USDT-BTC"),
            "BTCUSDT"
        );

        // Bybit to Upbit
        assert_eq!(
            convert_market_code(ExchangeName::Bybit, ExchangeName::Upbit, "BTCUSDT"),
            "USDT-BTC"
        );

        // Same exchange
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
