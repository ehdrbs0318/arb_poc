//! Telegram Bot API 클라이언트 구현.

use arb_config::TelegramConfig;
use reqwest::Client;
use tracing::{debug, error, warn};

use crate::error::TelegramError;
use crate::types::{Message, SendMessageOptions, SendMessageRequest, TelegramResponse};

/// Telegram Bot API 기본 URL.
const TELEGRAM_API_BASE: &str = "https://api.telegram.org";

/// Telegram Bot API 클라이언트.
///
/// 이 클라이언트는 Telegram Bot API를 통해 메시지를 전송합니다.
///
/// # 예제
///
/// ```rust,no_run
/// use arb_telegram::TelegramClient;
/// use arb_config::TelegramConfig;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = TelegramConfig {
///         bot_token: "YOUR_BOT_TOKEN".to_string(),
///         chat_id: "YOUR_CHAT_ID".to_string(),
///     };
///
///     let client = TelegramClient::new(&config)?;
///     client.send_message("Hello from Rust!").await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TelegramClient {
    client: Client,
    bot_token: String,
    chat_id: String,
}

impl TelegramClient {
    /// 새 Telegram 클라이언트를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `config` - Telegram 설정 (bot_token, chat_id)
    ///
    /// # 에러
    ///
    /// 설정이 유효하지 않거나 HTTP 클라이언트 생성에 실패하면 에러를 반환합니다.
    pub fn new(config: &TelegramConfig) -> Result<Self, TelegramError> {
        if !config.is_configured() {
            return Err(TelegramError::ConfigError(
                "bot_token and chat_id must be provided".to_string(),
            ));
        }

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(TelegramError::HttpError)?;

        Ok(Self {
            client,
            bot_token: config.bot_token.clone(),
            chat_id: config.chat_id.clone(),
        })
    }

    /// 토큰과 chat_id로 직접 클라이언트를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `bot_token` - Telegram Bot 토큰
    /// * `chat_id` - 대상 채팅 ID
    pub fn with_credentials(
        bot_token: impl Into<String>,
        chat_id: impl Into<String>,
    ) -> Result<Self, TelegramError> {
        let config = TelegramConfig {
            bot_token: bot_token.into(),
            chat_id: chat_id.into(),
        };
        Self::new(&config)
    }

    /// API URL을 생성합니다.
    fn api_url(&self, method: &str) -> String {
        format!("{}/bot{}/{}", TELEGRAM_API_BASE, self.bot_token, method)
    }

    /// 메시지를 전송합니다.
    ///
    /// # 인자
    ///
    /// * `text` - 전송할 메시지 텍스트
    ///
    /// # 반환값
    ///
    /// 전송된 메시지 정보
    ///
    /// # 예제
    ///
    /// ```rust,no_run
    /// # use arb_telegram::TelegramClient;
    /// # use arb_config::TelegramConfig;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = TelegramConfig::default();
    /// # let client = TelegramClient::new(&config)?;
    /// let message = client.send_message("Hello!").await?;
    /// println!("Message sent with ID: {}", message.message_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message(&self, text: &str) -> Result<Message, TelegramError> {
        self.send_message_with_options(text, SendMessageOptions::default())
            .await
    }

    /// 옵션과 함께 메시지를 전송합니다.
    ///
    /// # 인자
    ///
    /// * `text` - 전송할 메시지 텍스트
    /// * `options` - 메시지 전송 옵션
    ///
    /// # 반환값
    ///
    /// 전송된 메시지 정보
    ///
    /// # 예제
    ///
    /// ```rust,no_run
    /// # use arb_telegram::{TelegramClient, SendMessageOptions};
    /// # use arb_config::TelegramConfig;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = TelegramConfig::default();
    /// # let client = TelegramClient::new(&config)?;
    /// let options = SendMessageOptions::new()
    ///     .markdown_v2()
    ///     .silent();
    ///
    /// client.send_message_with_options("*Bold* message", options).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message_with_options(
        &self,
        text: &str,
        options: SendMessageOptions,
    ) -> Result<Message, TelegramError> {
        let request = SendMessageRequest {
            chat_id: self.chat_id.clone(),
            text: text.to_string(),
            parse_mode: options.parse_mode,
            disable_web_page_preview: options.disable_web_page_preview,
            disable_notification: options.disable_notification,
            reply_to_message_id: options.reply_to_message_id,
        };

        debug!(
            chat_id = %self.chat_id,
            text_length = text.len(),
            "Sending Telegram message"
        );

        let url = self.api_url("sendMessage");
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(TelegramError::HttpError)?;

        let status = response.status();
        let body = response.text().await.map_err(TelegramError::HttpError)?;

        let telegram_response: TelegramResponse<Message> =
            serde_json::from_str(&body).map_err(TelegramError::JsonError)?;

        if telegram_response.ok {
            let message = telegram_response.result.ok_or_else(|| {
                TelegramError::ApiError {
                    error_code: 0,
                    description: "No result in response".to_string(),
                }
            })?;

            debug!(
                message_id = message.message_id,
                "Telegram message sent successfully"
            );

            Ok(message)
        } else {
            let error_code = telegram_response.error_code.unwrap_or(status.as_u16() as i32);
            let description = telegram_response
                .description
                .unwrap_or_else(|| "Unknown error".to_string());

            // Rate limit 처리
            if error_code == 429 {
                let retry_after = telegram_response
                    .parameters
                    .and_then(|p| p.retry_after)
                    .unwrap_or(30);

                warn!(
                    retry_after = retry_after,
                    "Telegram rate limit exceeded"
                );

                return Err(TelegramError::RateLimited { retry_after });
            }

            error!(
                error_code = error_code,
                description = %description,
                "Telegram API error"
            );

            Err(TelegramError::ApiError {
                error_code,
                description,
            })
        }
    }

    /// Markdown V2 포맷 메시지를 전송합니다.
    ///
    /// 이 메서드는 `send_message_with_options`의 편의 래퍼입니다.
    ///
    /// # 인자
    ///
    /// * `text` - Markdown V2 포맷 텍스트
    ///
    /// # 주의
    ///
    /// 텍스트에 Markdown V2 특수 문자가 포함된 경우 `escape_markdown_v2` 함수로
    /// 이스케이프해야 합니다.
    pub async fn send_markdown(&self, text: &str) -> Result<Message, TelegramError> {
        self.send_message_with_options(text, SendMessageOptions::new().markdown_v2())
            .await
    }

    /// HTML 포맷 메시지를 전송합니다.
    ///
    /// # 인자
    ///
    /// * `text` - HTML 포맷 텍스트
    pub async fn send_html(&self, text: &str) -> Result<Message, TelegramError> {
        self.send_message_with_options(text, SendMessageOptions::new().html())
            .await
    }

    /// 무음 메시지를 전송합니다 (알림 없음).
    ///
    /// # 인자
    ///
    /// * `text` - 전송할 메시지 텍스트
    pub async fn send_silent(&self, text: &str) -> Result<Message, TelegramError> {
        self.send_message_with_options(text, SendMessageOptions::new().silent())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation_with_empty_config() {
        let config = TelegramConfig::default();
        let result = TelegramClient::new(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_client_creation_with_valid_config() {
        let config = TelegramConfig {
            bot_token: "test_token".to_string(),
            chat_id: "12345".to_string(),
        };
        let result = TelegramClient::new(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_api_url_generation() {
        let config = TelegramConfig {
            bot_token: "123456:ABC-DEF".to_string(),
            chat_id: "12345".to_string(),
        };
        let client = TelegramClient::new(&config).expect("Failed to create TelegramClient");
        let url = client.api_url("sendMessage");
        assert_eq!(
            url,
            "https://api.telegram.org/bot123456:ABC-DEF/sendMessage"
        );
    }

    #[test]
    fn test_with_credentials() {
        let client = TelegramClient::with_credentials("test_token", "12345");
        assert!(client.is_ok());
    }
}
