//! # Telegram 알림 API 예제
//!
//! 이 예제는 Telegram Bot API를 사용하여 알림 메시지를 보내는 방법을 보여줍니다.
//!
//! ## 사전 요구사항
//!
//! 1. Telegram Bot 생성:
//!    - @BotFather에게 `/newbot` 명령으로 새 봇 생성
//!    - Bot Token을 받아 `config.toml`에 설정
//!
//! 2. Chat ID 확인:
//!    - 봇에게 메시지를 보낸 후 `https://api.telegram.org/bot<TOKEN>/getUpdates`로
//!      chat_id 확인
//!
//! 3. 프로젝트 루트에 `config.toml` 파일을 생성하세요:
//!
//! ```toml
//! [telegram]
//! bot_token = "your-bot-token"
//! chat_id = "your-chat-id"
//! ```
//!
//! 또는 환경 변수를 설정하세요:
//! - `TELEGRAM_BOT_TOKEN`
//! - `TELEGRAM_CHAT_ID`
//!
//! ## 예제 실행 방법
//!
//! ```bash
//! cargo run --example telegram
//! ```

use arb_poc::config::Config;
use arb_poc::telegram::{SendMessageOptions, TelegramClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Telegram API Example ===\n");

    // 설정 로드
    let config = Config::load()?;

    if !config.telegram.is_configured() {
        eprintln!("Error: Telegram credentials not configured.");
        eprintln!("Please set up config.toml or environment variables.");
        eprintln!("\nExample config.toml:");
        eprintln!("[telegram]");
        eprintln!("bot_token = \"your-bot-token\"");
        eprintln!("chat_id = \"your-chat-id\"");
        return Ok(());
    }

    // Telegram 클라이언트 생성
    let client = TelegramClient::new(&config.telegram)?;

    println!("Chat ID: {}", config.telegram.chat_id);
    println!();

    // 1. 간단한 텍스트 메시지 보내기
    println!("--- Sending Plain Text Message ---");
    match client
        .send_message("Hello from arb_poc! This is a test message.")
        .await
    {
        Ok(msg) => println!(
            "Plain text message sent successfully! (message_id: {})",
            msg.message_id
        ),
        Err(e) => println!("Failed to send message: {}", e),
    }

    // 약간의 지연을 두어 Telegram API rate limit 방지
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 2. Markdown V2 형식의 메시지 보내기
    println!("\n--- Sending Markdown V2 Message ---");

    // 특수 문자가 포함된 메시지 (자동 이스케이프됨)
    let symbol = "BTC";
    let price = "148,500,000 KRW";
    let change = "+2.5%";

    // 수동으로 Markdown 형식 지정 (특수 문자 이스케이프 필요)
    let markdown_message = format!(
        "*{} 가격 알림*\n\n\
        현재가: `{}`\n\
        변동률: `{}`\n\n\
        _arb\\_poc 알림 시스템_",
        symbol, price, change
    );

    let options = SendMessageOptions::new().markdown_v2();

    match client
        .send_message_with_options(&markdown_message, options)
        .await
    {
        Ok(_) => println!("Markdown message sent successfully!"),
        Err(e) => println!("Failed to send markdown message: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 3. HTML 형식의 메시지 보내기
    println!("\n--- Sending HTML Message ---");

    let html_message = format!(
        "<b>{} 가격 알림</b>\n\n\
        현재가: <code>{}</code>\n\
        변동률: <code>{}</code>\n\n\
        <i>arb_poc 알림 시스템</i>",
        symbol, price, change
    );

    let html_options = SendMessageOptions::new().html();

    match client
        .send_message_with_options(&html_message, html_options)
        .await
    {
        Ok(_) => println!("HTML message sent successfully!"),
        Err(e) => println!("Failed to send HTML message: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 4. 무음 알림 보내기
    println!("\n--- Sending Silent Notification ---");

    let silent_options = SendMessageOptions::new().silent();

    match client
        .send_message_with_options("This is a silent notification.", silent_options)
        .await
    {
        Ok(_) => println!("Silent notification sent successfully!"),
        Err(e) => println!("Failed to send silent notification: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 5. 거래 알림 예제
    println!("\n--- Sending Trade Alert Example ---");

    let alert_message = "<b>Trade Opportunity Found</b>\n\n\
        <b>Buy:</b> Upbit KRW-BTC @ 148,500,000 KRW\n\
        <b>Sell:</b> Bithumb KRW-BTC @ 148,800,000 KRW\n\n\
        <b>Expected Profit:</b> <code>0.20%</code>\n\
        <b>Expected Amount:</b> <code>300,000 KRW</code>\n\n\
        <i>Auto trading disabled - Manual confirmation required</i>";

    let alert_options = SendMessageOptions::new().html();

    match client
        .send_message_with_options(alert_message, alert_options)
        .await
    {
        Ok(_) => println!("Trade alert sent successfully!"),
        Err(e) => println!("Failed to send trade alert: {}", e),
    }

    println!("\n=== Example Complete ===");
    println!("\nCheck your Telegram chat for the messages!");

    Ok(())
}
