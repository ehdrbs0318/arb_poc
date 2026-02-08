//! Upbit WebSocket 실시간 마켓 데이터 스트림 구현.
//!
//! `MarketStream` trait을 구현하여 Upbit의 체결(trade) 데이터를
//! WebSocket으로 실시간 수신합니다.

use arb_exchange::error::{ExchangeError, ExchangeResult};
use arb_exchange::stream::{MarketEvent, MarketStream, StreamCommand, StreamConfig};
use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use futures_util::{SinkExt, StreamExt};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::str::FromStr;
use tokio::sync::{Mutex, mpsc};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, trace, warn};

use crate::upbit::client::UpbitClient;

/// Upbit WebSocket URL.
const UPBIT_WS_URL: &str = "wss://api.upbit.com/websocket/v1";

/// Upbit WebSocket 체결 데이터 응답.
#[derive(Debug, Deserialize)]
struct UpbitWsTrade {
    /// 마켓 코드 (예: "KRW-BTC").
    #[serde(alias = "cd")]
    code: String,
    /// 체결 가격.
    #[serde(alias = "tp")]
    trade_price: f64,
    /// 체결 수량.
    #[serde(alias = "tv")]
    trade_volume: f64,
    /// 체결 타임스탬프 (밀리초).
    #[serde(alias = "ttms")]
    trade_timestamp: i64,
}

/// WebSocket task의 내부 상태.
struct StreamState {
    /// 구독 해제 시그널을 보내는 sender.
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    /// WebSocket task join handle.
    task_handle: Option<tokio::task::JoinHandle<()>>,
    /// 동적 구독 변경 명령을 보내는 sender.
    command_tx: Option<mpsc::Sender<StreamCommand>>,
}

/// Upbit MarketStream 구현을 위한 내부 상태.
pub(crate) struct UpbitStreamInner {
    state: Mutex<Option<StreamState>>,
    config: StreamConfig,
}

impl UpbitStreamInner {
    pub(crate) fn new(config: StreamConfig) -> Self {
        Self {
            state: Mutex::new(None),
            config,
        }
    }
}

#[async_trait]
impl MarketStream for UpbitClient {
    fn stream_name(&self) -> &str {
        "Upbit"
    }

    async fn subscribe(&self, markets: &[&str]) -> ExchangeResult<mpsc::Receiver<MarketEvent>> {
        let inner = self.stream_inner();

        // 기존 구독이 있으면 먼저 해제
        {
            let mut state_guard = inner.state.lock().await;
            if let Some(old_state) = state_guard.take() {
                if let Some(tx) = old_state.shutdown_tx {
                    let _ = tx.send(());
                }
                if let Some(handle) = old_state.task_handle {
                    handle.abort();
                }
                debug!("기존 Upbit WebSocket 구독 해제");
            }
        }

        let buffer_size = inner.config.channel_buffer_size;
        let (event_tx, event_rx) = mpsc::channel(buffer_size);
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
        let (command_tx, command_rx) = mpsc::channel::<StreamCommand>(64);

        let market_codes: Vec<String> = markets.iter().map(|m| m.to_string()).collect();
        let config = inner.config.clone();

        info!(
            markets = ?market_codes,
            "Upbit WebSocket 구독 시작"
        );

        let task_handle = tokio::spawn(async move {
            upbit_ws_loop(market_codes, event_tx, shutdown_rx, command_rx, config).await;
        });

        let mut state_guard = inner.state.lock().await;
        *state_guard = Some(StreamState {
            shutdown_tx: Some(shutdown_tx),
            task_handle: Some(task_handle),
            command_tx: Some(command_tx),
        });

        Ok(event_rx)
    }

    async fn unsubscribe(&self) -> ExchangeResult<()> {
        let inner = self.stream_inner();
        let mut state_guard = inner.state.lock().await;

        if let Some(state) = state_guard.take() {
            info!("Upbit WebSocket 구독 해제");
            if let Some(tx) = state.shutdown_tx {
                let _ = tx.send(());
            }
            if let Some(handle) = state.task_handle {
                handle.abort();
            }
        }

        Ok(())
    }

    async fn subscribe_markets(&self, markets: &[&str]) -> ExchangeResult<()> {
        let inner = self.stream_inner();
        let state_guard = inner.state.lock().await;
        if let Some(ref state) = *state_guard {
            if let Some(ref tx) = state.command_tx {
                tx.send(StreamCommand::Subscribe(
                    markets.iter().map(|m| m.to_string()).collect(),
                ))
                .await
                .map_err(|_| ExchangeError::WebSocketError("command channel closed".into()))?;
                Ok(())
            } else {
                Err(ExchangeError::WebSocketError("not subscribed".into()))
            }
        } else {
            Err(ExchangeError::WebSocketError("not subscribed".into()))
        }
    }

    async fn unsubscribe_markets(&self, markets: &[&str]) -> ExchangeResult<()> {
        let inner = self.stream_inner();
        let state_guard = inner.state.lock().await;
        if let Some(ref state) = *state_guard {
            if let Some(ref tx) = state.command_tx {
                tx.send(StreamCommand::Unsubscribe(
                    markets.iter().map(|m| m.to_string()).collect(),
                ))
                .await
                .map_err(|_| ExchangeError::WebSocketError("command channel closed".into()))?;
                Ok(())
            } else {
                Err(ExchangeError::WebSocketError("not subscribed".into()))
            }
        } else {
            Err(ExchangeError::WebSocketError("not subscribed".into()))
        }
    }
}

/// Upbit WebSocket 이벤트 루프 (재연결 + 동적 구독 포함).
async fn upbit_ws_loop(
    initial_markets: Vec<String>,
    event_tx: mpsc::Sender<MarketEvent>,
    mut shutdown_rx: tokio::sync::oneshot::Receiver<()>,
    mut command_rx: mpsc::Receiver<StreamCommand>,
    config: StreamConfig,
) {
    let mut retry_count: u32 = 0;
    let mut backoff = config.initial_backoff;
    // 현재 구독 중인 마켓 목록 (재구독 시 사용)
    let mut current_markets = initial_markets;

    loop {
        // 종료 확인
        if shutdown_rx.try_recv().is_ok() {
            info!("Upbit WebSocket 종료 요청");
            break;
        }

        match connect_and_subscribe(&current_markets).await {
            Ok(ws_stream) => {
                info!("Upbit WebSocket 연결 성공");
                retry_count = 0;
                backoff = config.initial_backoff;

                let (mut write, mut read) = ws_stream.split();

                loop {
                    tokio::select! {
                        _ = &mut shutdown_rx => {
                            info!("Upbit WebSocket 종료 요청 (루프 내)");
                            let _ = write.close().await;
                            return;
                        }
                        cmd = command_rx.recv() => {
                            match cmd {
                                Some(StreamCommand::Subscribe(new_markets)) => {
                                    // 새 마켓을 현재 목록에 추가 (중복 제거)
                                    for m in &new_markets {
                                        if !current_markets.contains(m) {
                                            current_markets.push(m.clone());
                                        }
                                    }
                                    info!(
                                        markets = ?current_markets,
                                        added = ?new_markets,
                                        "Upbit 동적 구독 추가 — 전체 재구독"
                                    );
                                    // Upbit은 전체 목록으로 재구독
                                    let msg = build_subscribe_message(&current_markets);
                                    if let Err(e) = write.send(Message::Text(msg.into())).await {
                                        error!(error = %e, "Upbit 재구독 메시지 전송 실패");
                                        break;
                                    }
                                }
                                Some(StreamCommand::Unsubscribe(remove_markets)) => {
                                    // 마켓 제거
                                    current_markets.retain(|m| !remove_markets.contains(m));
                                    info!(
                                        markets = ?current_markets,
                                        removed = ?remove_markets,
                                        "Upbit 동적 구독 해제 — 전체 재구독"
                                    );
                                    if current_markets.is_empty() {
                                        warn!("Upbit 구독 마켓이 0개 — 빈 구독 유지");
                                    }
                                    let msg = build_subscribe_message(&current_markets);
                                    if let Err(e) = write.send(Message::Text(msg.into())).await {
                                        error!(error = %e, "Upbit 재구독 메시지 전송 실패");
                                        break;
                                    }
                                }
                                None => {
                                    debug!("Upbit command 채널 닫힘");
                                }
                            }
                        }
                        msg = read.next() => {
                            match msg {
                                Some(Ok(Message::Text(text))) => {
                                    if let Some(event) = parse_upbit_trade(&text) {
                                        // backpressure: try_send로 버퍼 가득 찬 경우 드롭
                                        match event_tx.try_send(event) {
                                            Ok(()) => {
                                                trace!("Upbit 이벤트 전송 성공");
                                            }
                                            Err(mpsc::error::TrySendError::Full(_)) => {
                                                warn!("Upbit 이벤트 채널 가득 참 — 이벤트 드롭");
                                            }
                                            Err(mpsc::error::TrySendError::Closed(_)) => {
                                                debug!("Upbit 이벤트 채널 닫힘 — 종료");
                                                return;
                                            }
                                        }
                                    }
                                }
                                Some(Ok(Message::Binary(data))) => {
                                    // Upbit는 바이너리 프레임도 보낼 수 있음
                                    if let Ok(text) = String::from_utf8(data.to_vec())
                                        && let Some(event) = parse_upbit_trade(&text)
                                    {
                                        match event_tx.try_send(event) {
                                            Ok(()) => {}
                                            Err(mpsc::error::TrySendError::Full(_)) => {
                                                warn!("Upbit 이벤트 채널 가득 참 — 이벤트 드롭");
                                            }
                                            Err(mpsc::error::TrySendError::Closed(_)) => {
                                                return;
                                            }
                                        }
                                    }
                                }
                                Some(Ok(Message::Ping(data))) => {
                                    let _ = write.send(Message::Pong(data)).await;
                                }
                                Some(Ok(Message::Close(_))) => {
                                    warn!("Upbit WebSocket 서버에서 Close 수신");
                                    break;
                                }
                                Some(Err(e)) => {
                                    error!(error = %e, "Upbit WebSocket 수신 에러");
                                    break;
                                }
                                None => {
                                    warn!("Upbit WebSocket 스트림 종료");
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!(error = %e, "Upbit WebSocket 연결 실패");
            }
        }

        // 재연결 로직
        retry_count += 1;
        if config.max_retries > 0 && retry_count > config.max_retries {
            error!(
                retries = retry_count,
                "Upbit WebSocket 최대 재시도 초과 — 스트림 종료"
            );
            break;
        }

        warn!(
            retry = retry_count,
            backoff_ms = backoff.as_millis(),
            "Upbit WebSocket 재연결 대기"
        );

        tokio::select! {
            _ = &mut shutdown_rx => {
                info!("Upbit WebSocket 재연결 대기 중 종료 요청");
                return;
            }
            _ = tokio::time::sleep(backoff) => {}
        }

        // exponential backoff (최대값 제한)
        backoff = std::cmp::min(backoff * 2, config.max_backoff);
    }
}

/// Upbit 구독 메시지를 생성합니다.
///
/// Upbit은 개별 종목 추가/제거를 지원하지 않으므로,
/// 항상 전체 마켓 목록으로 재구독 메시지를 만들어야 합니다.
fn build_subscribe_message(markets: &[String]) -> String {
    let ticket = uuid::Uuid::new_v4().to_string();
    let codes: Vec<serde_json::Value> = markets
        .iter()
        .map(|m| serde_json::Value::String(m.clone()))
        .collect();

    let msg = serde_json::json!([
        {"ticket": ticket},
        {"type": "trade", "codes": codes, "isOnlyRealtime": true},
        {"format": "DEFAULT"}
    ]);
    msg.to_string()
}

/// Upbit WebSocket에 연결하고 구독 메시지를 보냅니다.
async fn connect_and_subscribe(
    markets: &[String],
) -> Result<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    let (ws_stream, response) = connect_async(UPBIT_WS_URL).await?;

    debug!(status = ?response.status(), "Upbit WebSocket 핸드셰이크 완료");

    let (mut write, read) = ws_stream.split();

    // 구독 메시지: [{"ticket": "..."}, {"type": "trade", "codes": [...]}, {"format": "DEFAULT"}]
    let ticket = uuid::Uuid::new_v4().to_string();
    let codes: Vec<serde_json::Value> = markets
        .iter()
        .map(|m| serde_json::Value::String(m.clone()))
        .collect();

    let subscribe_msg = serde_json::json!([
        {"ticket": ticket},
        {"type": "trade", "codes": codes, "isOnlyRealtime": true},
        {"format": "DEFAULT"}
    ]);

    debug!(msg = %subscribe_msg, "Upbit 구독 메시지 전송");

    write
        .send(Message::Text(subscribe_msg.to_string().into()))
        .await?;

    // write/read를 다시 결합
    Ok(read.reunite(write)?)
}

/// Upbit WebSocket 메시지를 MarketEvent로 파싱합니다.
fn parse_upbit_trade(text: &str) -> Option<MarketEvent> {
    let trade: UpbitWsTrade = serde_json::from_str(text).ok()?;

    let price = Decimal::from_str(&trade.trade_price.to_string()).ok()?;
    let volume = Decimal::from_str(&trade.trade_volume.to_string()).ok()?;
    let timestamp = Utc
        .timestamp_millis_opt(trade.trade_timestamp)
        .single()
        .unwrap_or_else(Utc::now);

    Some(MarketEvent::Trade {
        market: trade.code,
        price,
        volume,
        timestamp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_upbit_trade_default_format() {
        let json = r#"{
            "type": "trade",
            "code": "KRW-BTC",
            "trade_price": 138500000.0,
            "trade_volume": 0.001,
            "trade_timestamp": 1707177600000,
            "ask_bid": "BID",
            "sequential_id": 1234567890,
            "stream_type": "REALTIME"
        }"#;

        let event = parse_upbit_trade(json);
        assert!(event.is_some());
        if let Some(MarketEvent::Trade {
            market,
            price,
            volume,
            ..
        }) = event
        {
            assert_eq!(market, "KRW-BTC");
            assert_eq!(price, Decimal::new(138_500_000, 0));
            assert_eq!(volume, Decimal::new(1, 3));
        } else {
            panic!("Expected Trade event");
        }
    }

    #[test]
    fn test_parse_upbit_trade_simple_format() {
        let json = r#"{
            "cd": "KRW-ETH",
            "tp": 3500000.0,
            "tv": 0.5,
            "ttms": 1707177600000
        }"#;

        let event = parse_upbit_trade(json);
        assert!(event.is_some());
        if let Some(MarketEvent::Trade { market, price, .. }) = event {
            assert_eq!(market, "KRW-ETH");
            assert_eq!(price, Decimal::new(3_500_000, 0));
        }
    }

    #[test]
    fn test_parse_upbit_trade_invalid() {
        let json = r#"{"type": "orderbook", "data": {}}"#;
        assert!(parse_upbit_trade(json).is_none());
    }

    #[test]
    fn test_build_subscribe_message_contains_markets() {
        let markets = vec!["KRW-BTC".to_string(), "KRW-ETH".to_string()];
        let msg = build_subscribe_message(&markets);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();

        // 배열 형태인지 확인
        assert!(parsed.is_array());
        let arr = parsed.as_array().unwrap();
        assert_eq!(arr.len(), 3);

        // ticket 필드 확인
        assert!(arr[0]["ticket"].is_string());

        // trade 타입과 codes 확인
        assert_eq!(arr[1]["type"], "trade");
        let codes = arr[1]["codes"].as_array().unwrap();
        assert_eq!(codes.len(), 2);
        assert_eq!(codes[0], "KRW-BTC");
        assert_eq!(codes[1], "KRW-ETH");
        assert_eq!(arr[1]["isOnlyRealtime"], true);

        // format 확인
        assert_eq!(arr[2]["format"], "DEFAULT");
    }

    #[test]
    fn test_build_subscribe_message_empty() {
        let markets: Vec<String> = vec![];
        let msg = build_subscribe_message(&markets);
        let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
        let arr = parsed.as_array().unwrap();
        let codes = arr[1]["codes"].as_array().unwrap();
        assert!(codes.is_empty());
    }
}
