//! # Telegram Notification Module
//!
//! Telegram Bot API를 사용한 알림 전송 모듈입니다.
//!
//! ## 기능
//!
//! - Markdown V2 포맷 지원
//! - 비동기 메시지 전송
//! - 에러 처리 및 재시도 로직
//!
//! ## 사용 예시
//!
//! ```rust,no_run
//! use arb_telegram::{TelegramClient, SendMessageOptions};
//! use arb_config::TelegramConfig;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = TelegramConfig {
//!         bot_token: "YOUR_BOT_TOKEN".to_string(),
//!         chat_id: "YOUR_CHAT_ID".to_string(),
//!     };
//!
//!     let client = TelegramClient::new(&config)?;
//!
//!     // 간단한 메시지 전송
//!     client.send_message("Hello, World!").await?;
//!
//!     // Markdown V2 포맷 메시지 전송
//!     let options = SendMessageOptions::default().parse_mode("MarkdownV2");
//!     client.send_message_with_options("*Bold* _Italic_", options).await?;
//!
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod types;

pub use client::TelegramClient;
pub use error::TelegramError;
pub use types::SendMessageOptions;

/// Telegram Markdown V2 특수 문자를 이스케이프합니다.
///
/// Telegram Markdown V2 포맷에서는 다음 문자들을 백슬래시로 이스케이프해야 합니다:
/// `_`, `*`, `[`, `]`, `(`, `)`, `~`, `` ` ``, `>`, `#`, `+`, `-`, `=`, `|`, `{`, `}`, `.`, `!`
///
/// # 인자
///
/// * `text` - 이스케이프할 텍스트
///
/// # 반환값
///
/// 특수 문자가 이스케이프된 문자열
///
/// # 예제
///
/// ```
/// use arb_telegram::escape_markdown_v2;
///
/// let escaped = escape_markdown_v2("Hello *World*!");
/// assert_eq!(escaped, r"Hello \*World\*\!");
/// ```
pub fn escape_markdown_v2(text: &str) -> String {
    let special_chars = [
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];

    let mut result = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        if special_chars.contains(&c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

/// 가격을 포맷팅하고 Markdown V2용으로 이스케이프합니다.
///
/// # 인자
///
/// * `price` - 가격 값
/// * `currency` - 통화 단위 (예: "KRW", "USDT")
///
/// # 반환값
///
/// 포맷팅되고 이스케이프된 가격 문자열
///
/// # 예제
///
/// ```
/// use arb_telegram::format_price_markdown;
/// use rust_decimal::Decimal;
///
/// let formatted = format_price_markdown(Decimal::from(50000000), "KRW");
/// // 결과: "50,000,000 KRW" (쉼표와 공백이 이스케이프됨)
/// ```
pub fn format_price_markdown(price: rust_decimal::Decimal, currency: &str) -> String {
    // 천 단위 구분자 추가
    let price_str = price.to_string();
    let parts: Vec<&str> = price_str.split('.').collect();
    let integer_part = parts[0];

    // 음수 처리: 마이너스 기호 분리
    let (is_negative, digits) = if let Some(stripped) = integer_part.strip_prefix('-') {
        (true, stripped)
    } else {
        (false, integer_part)
    };

    // 역순으로 처리하여 3자리마다 쉼표 추가
    let chars: Vec<char> = digits.chars().collect();
    let mut formatted_chars = Vec::with_capacity(chars.len() + chars.len() / 3);

    for (i, &c) in chars.iter().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted_chars.push(',');
        }
        formatted_chars.push(c);
    }

    // 다시 역순으로 뒤집어서 정상 순서로 변환
    formatted_chars.reverse();
    let formatted_integer: String = formatted_chars.into_iter().collect();

    // 음수면 마이너스 기호를 앞에 추가
    let formatted_integer = if is_negative {
        format!("-{}", formatted_integer)
    } else {
        formatted_integer
    };

    let formatted = if parts.len() > 1 {
        format!("{}.{} {}", formatted_integer, parts[1], currency)
    } else {
        format!("{} {}", formatted_integer, currency)
    };

    escape_markdown_v2(&formatted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_escape_markdown_v2() {
        assert_eq!(escape_markdown_v2("Hello World"), "Hello World");
        assert_eq!(escape_markdown_v2("Hello *World*"), r"Hello \*World\*");
        assert_eq!(escape_markdown_v2("Price: 100.50"), r"Price: 100\.50");
        assert_eq!(escape_markdown_v2("Test_underscore"), r"Test\_underscore");
        assert_eq!(escape_markdown_v2("[link](url)"), r"\[link\]\(url\)");
    }

    #[test]
    fn test_escape_markdown_v2_special_chars() {
        let text = "_*[]()~`>#+-=|{}.!";
        let escaped = escape_markdown_v2(text);
        assert_eq!(
            escaped,
            r"\_\*\[\]\(\)\~\`\>\#\+\-\=\|\{\}\.\!"
        );
    }

    #[test]
    fn test_format_price_markdown() {
        let price = Decimal::from(50000000);
        let formatted = format_price_markdown(price, "KRW");
        // 이스케이프 전: "50,000,000 KRW"
        // 이스케이프 후: 쉼표는 특수문자가 아니므로 그대로 유지
        assert!(formatted.contains("50"));
        assert!(formatted.contains("000"));
        assert!(formatted.contains("KRW"));
    }

    #[test]
    fn test_format_price_markdown_with_decimal() {
        let price = Decimal::new(123456789, 2); // 1234567.89
        let formatted = format_price_markdown(price, "USDT");
        assert!(formatted.contains("1"));
        assert!(formatted.contains("USDT"));
        // 소수점은 이스케이프됨
        assert!(formatted.contains(r"\."));
    }

    #[test]
    fn test_format_price_markdown_exact_output() {
        // 정확한 출력값 테스트
        let price = Decimal::from(50000000);
        let formatted = format_price_markdown(price, "KRW");
        assert_eq!(formatted, "50,000,000 KRW");

        let price = Decimal::from(1000);
        let formatted = format_price_markdown(price, "USDT");
        assert_eq!(formatted, "1,000 USDT");

        let price = Decimal::from(100);
        let formatted = format_price_markdown(price, "USDT");
        assert_eq!(formatted, "100 USDT");

        let price = Decimal::from(1);
        let formatted = format_price_markdown(price, "KRW");
        assert_eq!(formatted, "1 KRW");
    }

    #[test]
    fn test_format_price_markdown_with_decimals_exact() {
        // 소수점 포함 정확한 출력값 테스트
        let price = Decimal::new(123456789, 2); // 1234567.89
        let formatted = format_price_markdown(price, "USDT");
        // 소수점이 이스케이프되어 \. 로 변환
        assert_eq!(formatted, r"1,234,567\.89 USDT");

        let price = Decimal::new(1050, 2); // 10.50
        let formatted = format_price_markdown(price, "USDT");
        assert_eq!(formatted, r"10\.50 USDT");
    }

    #[test]
    fn test_format_price_markdown_edge_cases() {
        // 엣지 케이스: 0
        let price = Decimal::from(0);
        let formatted = format_price_markdown(price, "KRW");
        assert_eq!(formatted, "0 KRW");

        // 엣지 케이스: 음수 - 정확한 출력값 검증
        let price = Decimal::from(-1000);
        let formatted = format_price_markdown(price, "KRW");
        // 마이너스 기호가 앞에 오고, 쉼표가 올바른 위치에 삽입됨
        assert_eq!(formatted, r"\-1,000 KRW");

        // 큰 음수
        let price = Decimal::from(-1234567);
        let formatted = format_price_markdown(price, "KRW");
        assert_eq!(formatted, r"\-1,234,567 KRW");

        // 음수 소수점
        let price = Decimal::new(-123456, 2); // -1234.56
        let formatted = format_price_markdown(price, "USDT");
        assert_eq!(formatted, r"\-1,234\.56 USDT");
    }
}
