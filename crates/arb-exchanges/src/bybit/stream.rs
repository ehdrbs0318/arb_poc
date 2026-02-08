//! Bybit WebSocket 실시간 마켓 데이터 스트림 구현.
//!
//! `MarketStream` trait을 구현하여 Bybit의 tickers (best bid/ask)
//! 데이터를 WebSocket으로 실시간 수신합니다.

use arb_exchange::error::ExchangeResult;
use arb_exchange::stream::{MarketEvent, MarketStream, StreamConfig};
use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use futures_util::{SinkExt, StreamExt};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::{Mutex, mpsc};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, trace, warn};

use crate::bybit::client::BybitClient;

/// Bybit Linear WebSocket URL (메인넷).
const BYBIT_WS_LINEAR_URL: &str = "wss://stream.bybit.com/v5/public/linear";

/// Bybit heartbeat 간격 (20초).
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(20);

/// Bybit WebSocket 응답 래퍼.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BybitWsResponse {
    /// 토픽 이름 (예: "tickers.BTCUSDT").
    topic: Option<String>,
    /// 데이터 타입 ("snapshot" or "delta").
    #[serde(rename = "type")]
    msg_type: Option<String>,
    /// 타임스탬프 (ms).
    ts: Option<i64>,
    /// 데이터 페이로드.
    data: Option<serde_json::Value>,
    /// 오퍼레이션 응답 (subscribe, pong 등).
    op: Option<String>,
    /// 성공 여부.
    success: Option<bool>,
    /// 응답 메시지.
    ret_msg: Option<String>,
}

/// Bybit Linear Ticker 데이터 (tickers.{symbol}).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct BybitTickerData {
    /// 심볼 이름.
    symbol: Option<String>,
    /// Best bid price.
    bid1_price: Option<String>,
    /// Best ask price.
    ask1_price: Option<String>,
    /// Last price.
    last_price: Option<String>,
}

/// WebSocket task의 내부 상태.
struct StreamState {
    /// 구독 해제 시그널을 보내는 sender.
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    /// WebSocket task join handle.
    task_handle: Option<tokio::task::JoinHandle<()>>,
}

/// Bybit MarketStream 구현을 위한 내부 상태.
pub(crate) struct BybitStreamInner {
    state: Mutex<Option<StreamState>>,
    config: StreamConfig,
}

impl BybitStreamInner {
    pub(crate) fn new(config: StreamConfig) -> Self {
        Self {
            state: Mutex::new(None),
            config,
        }
    }
}

#[async_trait]
impl MarketStream for BybitClient {
    fn stream_name(&self) -> &str {
        "Bybit"
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
                debug!("기존 Bybit WebSocket 구독 해제");
            }
        }

        let buffer_size = inner.config.channel_buffer_size;
        let (event_tx, event_rx) = mpsc::channel(buffer_size);
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

        // tickers.{symbol} 형식으로 구독 토픽 생성
        let topics: Vec<String> = markets.iter().map(|m| format!("tickers.{m}")).collect();
        let config = inner.config.clone();

        info!(
            topics = ?topics,
            "Bybit WebSocket 구독 시작"
        );

        let task_handle = tokio::spawn(async move {
            bybit_ws_loop(topics, event_tx, shutdown_rx, config).await;
        });

        let mut state_guard = inner.state.lock().await;
        *state_guard = Some(StreamState {
            shutdown_tx: Some(shutdown_tx),
            task_handle: Some(task_handle),
        });

        Ok(event_rx)
    }

    async fn unsubscribe(&self) -> ExchangeResult<()> {
        let inner = self.stream_inner();
        let mut state_guard = inner.state.lock().await;

        if let Some(state) = state_guard.take() {
            info!("Bybit WebSocket 구독 해제");
            if let Some(tx) = state.shutdown_tx {
                let _ = tx.send(());
            }
            if let Some(handle) = state.task_handle {
                handle.abort();
            }
        }

        Ok(())
    }
}

/// Bybit WebSocket 이벤트 루프 (재연결 + heartbeat 포함).
async fn bybit_ws_loop(
    topics: Vec<String>,
    event_tx: mpsc::Sender<MarketEvent>,
    mut shutdown_rx: tokio::sync::oneshot::Receiver<()>,
    config: StreamConfig,
) {
    let mut retry_count: u32 = 0;
    let mut backoff = config.initial_backoff;

    loop {
        // 종료 확인
        if shutdown_rx.try_recv().is_ok() {
            info!("Bybit WebSocket 종료 요청");
            break;
        }

        match connect_and_subscribe(&topics).await {
            Ok(ws_stream) => {
                info!("Bybit WebSocket 연결 성공");
                retry_count = 0;
                backoff = config.initial_backoff;

                let (mut write, mut read) = ws_stream.split();
                let mut heartbeat = tokio::time::interval(HEARTBEAT_INTERVAL);

                loop {
                    tokio::select! {
                        _ = &mut shutdown_rx => {
                            info!("Bybit WebSocket 종료 요청 (루프 내)");
                            let _ = write.close().await;
                            return;
                        }
                        _ = heartbeat.tick() => {
                            // Bybit heartbeat ping
                            let ping = serde_json::json!({"op": "ping"});
                            if let Err(e) = write.send(Message::Text(ping.to_string().into())).await {
                                error!(error = %e, "Bybit heartbeat 전송 실패");
                                break;
                            }
                            trace!("Bybit heartbeat ping 전송");
                        }
                        msg = read.next() => {
                            match msg {
                                Some(Ok(Message::Text(text))) => {
                                    if let Some(event) = parse_bybit_ticker(&text) {
                                        match event_tx.try_send(event) {
                                            Ok(()) => {
                                                trace!("Bybit 이벤트 전송 성공");
                                            }
                                            Err(mpsc::error::TrySendError::Full(_)) => {
                                                warn!("Bybit 이벤트 채널 가득 참 — 이벤트 드롭");
                                            }
                                            Err(mpsc::error::TrySendError::Closed(_)) => {
                                                debug!("Bybit 이벤트 채널 닫힘 — 종료");
                                                return;
                                            }
                                        }
                                    }
                                    // pong/subscribe 응답은 무시 (파싱 실패로 None 반환)
                                }
                                Some(Ok(Message::Ping(data))) => {
                                    let _ = write.send(Message::Pong(data)).await;
                                }
                                Some(Ok(Message::Close(_))) => {
                                    warn!("Bybit WebSocket 서버에서 Close 수신");
                                    break;
                                }
                                Some(Err(e)) => {
                                    error!(error = %e, "Bybit WebSocket 수신 에러");
                                    break;
                                }
                                None => {
                                    warn!("Bybit WebSocket 스트림 종료");
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!(error = %e, "Bybit WebSocket 연결 실패");
            }
        }

        // 재연결 로직
        retry_count += 1;
        if config.max_retries > 0 && retry_count > config.max_retries {
            error!(
                retries = retry_count,
                "Bybit WebSocket 최대 재시도 초과 — 스트림 종료"
            );
            break;
        }

        warn!(
            retry = retry_count,
            backoff_ms = backoff.as_millis(),
            "Bybit WebSocket 재연결 대기"
        );

        tokio::select! {
            _ = &mut shutdown_rx => {
                info!("Bybit WebSocket 재연결 대기 중 종료 요청");
                return;
            }
            _ = tokio::time::sleep(backoff) => {}
        }

        // exponential backoff (최대값 제한)
        backoff = std::cmp::min(backoff * 2, config.max_backoff);
    }
}

/// Bybit WebSocket에 연결하고 구독 메시지를 보냅니다.
async fn connect_and_subscribe(
    topics: &[String],
) -> Result<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    let (ws_stream, response) = connect_async(BYBIT_WS_LINEAR_URL).await?;

    debug!(status = ?response.status(), "Bybit WebSocket 핸드셰이크 완료");

    let (mut write, read) = ws_stream.split();

    // 구독 메시지: {"op": "subscribe", "args": ["tickers.BTCUSDT", ...]}
    let args: Vec<serde_json::Value> = topics
        .iter()
        .map(|t| serde_json::Value::String(t.clone()))
        .collect();

    let subscribe_msg = serde_json::json!({
        "op": "subscribe",
        "args": args
    });

    debug!(msg = %subscribe_msg, "Bybit 구독 메시지 전송");

    write
        .send(Message::Text(subscribe_msg.to_string().into()))
        .await?;

    // write/read를 다시 결합
    Ok(read.reunite(write)?)
}

/// Bybit WebSocket 메시지를 MarketEvent::BestQuote로 파싱합니다.
fn parse_bybit_ticker(text: &str) -> Option<MarketEvent> {
    let resp: BybitWsResponse = serde_json::from_str(text).ok()?;

    // op 응답 (subscribe, pong)은 무시
    if resp.op.is_some() {
        if let Some(success) = resp.success
            && !success
        {
            warn!(
                ret_msg = resp.ret_msg.as_deref().unwrap_or(""),
                "Bybit WebSocket 오퍼레이션 실패"
            );
        }
        return None;
    }

    // tickers 토픽만 처리
    let topic = resp.topic.as_deref()?;
    if !topic.starts_with("tickers.") {
        return None;
    }

    let data = resp.data?;
    let ticker: BybitTickerData = serde_json::from_value(data).ok()?;

    let symbol = ticker.symbol?;
    let bid_str = ticker.bid1_price?;
    let ask_str = ticker.ask1_price?;

    let bid = Decimal::from_str(&bid_str).ok()?;
    let ask = Decimal::from_str(&ask_str).ok()?;

    let timestamp = resp
        .ts
        .and_then(|ts| Utc.timestamp_millis_opt(ts).single())
        .unwrap_or_else(Utc::now);

    Some(MarketEvent::BestQuote {
        market: symbol,
        bid,
        ask,
        timestamp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bybit_ticker_snapshot() {
        let json = r#"{
            "topic": "tickers.BTCUSDT",
            "type": "snapshot",
            "data": {
                "symbol": "BTCUSDT",
                "bid1Price": "99500.50",
                "bid1Size": "10.5",
                "ask1Price": "99501.00",
                "ask1Size": "8.3",
                "lastPrice": "99500.75",
                "tickDirection": "PlusTick"
            },
            "cs": 12345,
            "ts": 1707177600000
        }"#;

        let event = parse_bybit_ticker(json);
        assert!(event.is_some());
        if let Some(MarketEvent::BestQuote {
            market, bid, ask, ..
        }) = event
        {
            assert_eq!(market, "BTCUSDT");
            assert_eq!(bid, Decimal::from_str("99500.50").unwrap());
            assert_eq!(ask, Decimal::from_str("99501.00").unwrap());
        } else {
            panic!("Expected BestQuote event");
        }
    }

    #[test]
    fn test_parse_bybit_ticker_delta() {
        // delta 메시지에서 bid1Price/ask1Price가 포함된 경우
        let json = r#"{
            "topic": "tickers.ETHUSDT",
            "type": "delta",
            "data": {
                "symbol": "ETHUSDT",
                "bid1Price": "3200.50",
                "ask1Price": "3201.00"
            },
            "cs": 12346,
            "ts": 1707177601000
        }"#;

        let event = parse_bybit_ticker(json);
        assert!(event.is_some());
    }

    #[test]
    fn test_parse_bybit_pong() {
        let json = r#"{
            "success": true,
            "ret_msg": "pong",
            "conn_id": "some-conn-id",
            "op": "ping"
        }"#;

        // pong 응답은 None 반환
        assert!(parse_bybit_ticker(json).is_none());
    }

    #[test]
    fn test_parse_bybit_subscribe_response() {
        let json = r#"{
            "success": true,
            "ret_msg": "",
            "conn_id": "some-conn-id",
            "req_id": "",
            "op": "subscribe"
        }"#;

        assert!(parse_bybit_ticker(json).is_none());
    }

    #[test]
    fn test_parse_bybit_delta_without_bid_ask() {
        // delta 메시지에서 bid1Price/ask1Price가 없는 경우
        let json = r#"{
            "topic": "tickers.BTCUSDT",
            "type": "delta",
            "data": {
                "symbol": "BTCUSDT",
                "lastPrice": "99500.75"
            },
            "cs": 12347,
            "ts": 1707177602000
        }"#;

        // bid/ask 없으면 None
        assert!(parse_bybit_ticker(json).is_none());
    }
}
