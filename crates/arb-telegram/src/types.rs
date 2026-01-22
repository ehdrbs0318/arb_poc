//! Telegram API 타입 정의.

use serde::{Deserialize, Serialize};

/// 메시지 전송 옵션.
#[derive(Debug, Clone, Default)]
pub struct SendMessageOptions {
    /// 메시지 파싱 모드 (HTML, Markdown, MarkdownV2).
    pub parse_mode: Option<String>,
    /// 링크 미리보기 비활성화 여부.
    pub disable_web_page_preview: bool,
    /// 알림 비활성화 여부.
    pub disable_notification: bool,
    /// 답장 대상 메시지 ID.
    pub reply_to_message_id: Option<i64>,
}

impl SendMessageOptions {
    /// 새 옵션을 생성합니다.
    pub fn new() -> Self {
        Self::default()
    }

    /// 파싱 모드를 설정합니다.
    ///
    /// # 인자
    ///
    /// * `mode` - 파싱 모드 ("HTML", "Markdown", "MarkdownV2")
    #[must_use]
    pub fn parse_mode(mut self, mode: impl Into<String>) -> Self {
        self.parse_mode = Some(mode.into());
        self
    }

    /// Markdown V2 파싱 모드를 설정합니다.
    #[must_use]
    pub fn markdown_v2(self) -> Self {
        self.parse_mode("MarkdownV2")
    }

    /// HTML 파싱 모드를 설정합니다.
    #[must_use]
    pub fn html(self) -> Self {
        self.parse_mode("HTML")
    }

    /// 링크 미리보기를 비활성화합니다.
    #[must_use]
    pub fn disable_preview(mut self) -> Self {
        self.disable_web_page_preview = true;
        self
    }

    /// 알림을 비활성화합니다 (무음 메시지).
    #[must_use]
    pub fn silent(mut self) -> Self {
        self.disable_notification = true;
        self
    }

    /// 특정 메시지에 답장합니다.
    #[must_use]
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.reply_to_message_id = Some(message_id);
        self
    }
}

/// Telegram sendMessage API 요청 본문.
#[derive(Debug, Serialize)]
pub(crate) struct SendMessageRequest {
    pub chat_id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub disable_web_page_preview: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
}

/// Telegram API 응답 래퍼.
#[derive(Debug, Deserialize)]
pub(crate) struct TelegramResponse<T> {
    pub ok: bool,
    #[serde(default)]
    pub result: Option<T>,
    #[serde(default)]
    pub error_code: Option<i32>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parameters: Option<ResponseParameters>,
}

/// Telegram API 응답 파라미터.
#[derive(Debug, Deserialize)]
pub(crate) struct ResponseParameters {
    #[serde(default)]
    pub retry_after: Option<i32>,
}

/// Telegram Message 객체.
#[derive(Debug, Default, Deserialize)]
pub struct Message {
    /// 메시지 ID.
    pub message_id: i64,
    /// 메시지 텍스트.
    pub text: Option<String>,
    /// 전송 일시 (Unix timestamp).
    pub date: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_message_options_builder() {
        let options = SendMessageOptions::new()
            .markdown_v2()
            .disable_preview()
            .silent()
            .reply_to(123);

        assert_eq!(options.parse_mode, Some("MarkdownV2".to_string()));
        assert!(options.disable_web_page_preview);
        assert!(options.disable_notification);
        assert_eq!(options.reply_to_message_id, Some(123));
    }

    #[test]
    fn test_send_message_options_html() {
        let options = SendMessageOptions::new().html();
        assert_eq!(options.parse_mode, Some("HTML".to_string()));
    }

    #[test]
    fn test_send_message_request_serialization() {
        let request = SendMessageRequest {
            chat_id: "12345".to_string(),
            text: "Hello".to_string(),
            parse_mode: Some("MarkdownV2".to_string()),
            disable_web_page_preview: false,
            disable_notification: false,
            reply_to_message_id: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("chat_id"));
        assert!(json.contains("12345"));
        assert!(json.contains("Hello"));
        assert!(json.contains("MarkdownV2"));
        // disable_web_page_preview는 false이므로 포함되지 않아야 함
        assert!(!json.contains("disable_web_page_preview"));
    }
}
