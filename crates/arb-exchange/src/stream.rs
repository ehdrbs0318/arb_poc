//! 실시간 마켓 데이터 스트림 trait 정의.
//!
//! WebSocket 기반 실시간 데이터를 위한 추상화 계층입니다.

use crate::error::{ExchangeError, ExchangeResult};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::time::Duration;

/// 실시간 시세 데이터 이벤트.
#[derive(Debug, Clone)]
pub enum MarketEvent {
    /// 체결 이벤트 (개별 체결 데이터).
    Trade {
        /// 마켓 코드 (예: "KRW-BTC", "BTCUSDT").
        market: String,
        /// 체결 가격.
        price: Decimal,
        /// 체결 수량.
        volume: Decimal,
        /// 체결 시각.
        timestamp: DateTime<Utc>,
    },
    /// 호가 업데이트 (best bid/ask).
    BestQuote {
        /// 마켓 코드 (예: "KRW-BTC", "BTCUSDT").
        market: String,
        /// 최우선 매수 호가.
        bid: Decimal,
        /// 최우선 매도 호가.
        ask: Decimal,
        /// 호가 갱신 시각.
        timestamp: DateTime<Utc>,
    },
}

/// WebSocket 재연결 정책 설정.
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// 초기 백오프 딜레이 (기본값: 1초).
    pub initial_backoff: Duration,
    /// 최대 백오프 딜레이 (기본값: 30초).
    pub max_backoff: Duration,
    /// 최대 재시도 횟수 (기본값: 10, 0이면 무제한).
    pub max_retries: u32,
    /// REST fallback 폴링 간격 (기본값: 5초).
    pub rest_fallback_interval: Duration,
    /// 채널 버퍼 크기 (기본값: 10000).
    pub channel_buffer_size: usize,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            initial_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(30),
            max_retries: 10,
            rest_fallback_interval: Duration::from_secs(5),
            channel_buffer_size: 10000,
        }
    }
}

/// WebSocket 루프에 전달하는 명령.
#[derive(Debug, Clone)]
pub enum StreamCommand {
    /// 마켓 추가 (Upbit: 전체 재구독, Bybit: 개별 subscribe).
    Subscribe(Vec<String>),
    /// 마켓 제거 (Upbit: 전체 재구독, Bybit: 개별 unsubscribe).
    Unsubscribe(Vec<String>),
}

/// 실시간 마켓 데이터 스트림 trait.
///
/// WebSocket 기반 실시간 데이터 구독을 위한 인터페이스입니다.
/// 내부적으로 `Arc<Mutex<InnerState>>`를 사용하여 `&self`로 상태를 관리합니다.
/// bounded channel을 사용하여 backpressure를 적용합니다.
#[async_trait]
pub trait MarketStream: Send + Sync {
    /// 거래소 이름을 반환합니다.
    ///
    /// `MarketData::name()`과 별도 -- 동일 구조체에 두 trait 구현 시 충돌 방지.
    fn stream_name(&self) -> &str;

    /// 지정한 마켓들에 대한 실시간 스트림을 시작합니다.
    ///
    /// 반환되는 Receiver에서 `MarketEvent`를 수신합니다.
    /// 이미 구독 중인 상태에서 재호출하면 기존 구독을 종료하고 새로운 구독으로 대체합니다.
    async fn subscribe(
        &self,
        markets: &[&str],
    ) -> ExchangeResult<tokio::sync::mpsc::Receiver<MarketEvent>>;

    /// 모든 구독을 종료합니다.
    async fn unsubscribe(&self) -> ExchangeResult<()>;

    /// 기존 연결을 유지하면서 마켓을 추가 구독합니다.
    ///
    /// 기본 구현은 `Unsupported` 에러를 반환합니다.
    /// 거래소별 구현체에서 필요에 따라 오버라이드하세요.
    async fn subscribe_markets(&self, _markets: &[&str]) -> ExchangeResult<()> {
        Err(ExchangeError::Unsupported(
            "subscribe_markets not implemented".into(),
        ))
    }

    /// 기존 연결을 유지하면서 마켓 구독을 해제합니다.
    ///
    /// 기본 구현은 `Unsupported` 에러를 반환합니다.
    /// 거래소별 구현체에서 필요에 따라 오버라이드하세요.
    async fn unsubscribe_markets(&self, _markets: &[&str]) -> ExchangeResult<()> {
        Err(ExchangeError::Unsupported(
            "unsubscribe_markets not implemented".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_event_trade_debug() {
        let event = MarketEvent::Trade {
            market: "KRW-BTC".to_string(),
            price: Decimal::new(50_000_000, 0),
            volume: Decimal::new(1, 1),
            timestamp: Utc::now(),
        };
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("Trade"));
        assert!(debug_str.contains("KRW-BTC"));
    }

    #[test]
    fn test_market_event_best_quote_debug() {
        let event = MarketEvent::BestQuote {
            market: "BTCUSDT".to_string(),
            bid: Decimal::new(49_999, 0),
            ask: Decimal::new(50_001, 0),
            timestamp: Utc::now(),
        };
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("BestQuote"));
        assert!(debug_str.contains("BTCUSDT"));
    }

    #[test]
    fn test_market_event_clone() {
        let event = MarketEvent::Trade {
            market: "KRW-ETH".to_string(),
            price: Decimal::new(3_000_000, 0),
            volume: Decimal::new(5, 1),
            timestamp: Utc::now(),
        };
        let cloned = event.clone();
        match (&event, &cloned) {
            (
                MarketEvent::Trade {
                    market: m1,
                    price: p1,
                    ..
                },
                MarketEvent::Trade {
                    market: m2,
                    price: p2,
                    ..
                },
            ) => {
                assert_eq!(m1, m2);
                assert_eq!(p1, p2);
            }
            _ => panic!("Clone should produce the same variant"),
        }
    }

    #[test]
    fn test_stream_config_default() {
        let config = StreamConfig::default();
        assert_eq!(config.initial_backoff, Duration::from_secs(1));
        assert_eq!(config.max_backoff, Duration::from_secs(30));
        assert_eq!(config.max_retries, 10);
        assert_eq!(config.rest_fallback_interval, Duration::from_secs(5));
        assert_eq!(config.channel_buffer_size, 10000);
    }

    #[test]
    fn test_stream_config_custom() {
        let config = StreamConfig {
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(60),
            max_retries: 0,
            rest_fallback_interval: Duration::from_secs(10),
            channel_buffer_size: 50000,
        };
        assert_eq!(config.initial_backoff, Duration::from_millis(500));
        assert_eq!(config.max_backoff, Duration::from_secs(60));
        assert_eq!(config.max_retries, 0);
        assert_eq!(config.rest_fallback_interval, Duration::from_secs(10));
        assert_eq!(config.channel_buffer_size, 50000);
    }

    #[test]
    fn test_stream_config_clone() {
        let config = StreamConfig::default();
        let cloned = config.clone();
        assert_eq!(config.initial_backoff, cloned.initial_backoff);
        assert_eq!(config.max_backoff, cloned.max_backoff);
        assert_eq!(config.max_retries, cloned.max_retries);
        assert_eq!(config.rest_fallback_interval, cloned.rest_fallback_interval);
        assert_eq!(config.channel_buffer_size, cloned.channel_buffer_size);
    }

    #[test]
    fn test_stream_config_debug() {
        let config = StreamConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("StreamConfig"));
        assert!(debug_str.contains("initial_backoff"));
    }

    #[test]
    fn test_market_event_trade_fields() {
        let now = Utc::now();
        let price = Decimal::new(100_000_000, 0);
        let volume = Decimal::new(25, 2);
        let event = MarketEvent::Trade {
            market: "KRW-BTC".to_string(),
            price,
            volume,
            timestamp: now,
        };
        match event {
            MarketEvent::Trade {
                market,
                price: p,
                volume: v,
                timestamp: t,
            } => {
                assert_eq!(market, "KRW-BTC");
                assert_eq!(p, price);
                assert_eq!(v, volume);
                assert_eq!(t, now);
            }
            _ => panic!("Expected Trade variant"),
        }
    }

    #[test]
    fn test_market_event_best_quote_fields() {
        let now = Utc::now();
        let bid = Decimal::new(49_999_000, 0);
        let ask = Decimal::new(50_001_000, 0);
        let event = MarketEvent::BestQuote {
            market: "KRW-BTC".to_string(),
            bid,
            ask,
            timestamp: now,
        };
        match event {
            MarketEvent::BestQuote {
                market,
                bid: b,
                ask: a,
                timestamp: t,
            } => {
                assert_eq!(market, "KRW-BTC");
                assert_eq!(b, bid);
                assert_eq!(a, ask);
                assert_eq!(t, now);
            }
            _ => panic!("Expected BestQuote variant"),
        }
    }

    #[test]
    fn test_stream_command_subscribe_debug() {
        let cmd = StreamCommand::Subscribe(vec!["KRW-BTC".to_string(), "KRW-ETH".to_string()]);
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("Subscribe"));
        assert!(debug_str.contains("KRW-BTC"));
    }

    #[test]
    fn test_stream_command_unsubscribe_debug() {
        let cmd = StreamCommand::Unsubscribe(vec!["BTCUSDT".to_string()]);
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("Unsubscribe"));
        assert!(debug_str.contains("BTCUSDT"));
    }

    #[test]
    fn test_stream_command_clone() {
        let cmd = StreamCommand::Subscribe(vec!["KRW-BTC".to_string()]);
        let cloned = cmd.clone();
        match (&cmd, &cloned) {
            (StreamCommand::Subscribe(a), StreamCommand::Subscribe(b)) => {
                assert_eq!(a, b);
            }
            _ => panic!("Clone should produce the same variant"),
        }
    }

    #[test]
    fn test_stream_command_subscribe_empty() {
        let cmd = StreamCommand::Subscribe(vec![]);
        match cmd {
            StreamCommand::Subscribe(markets) => assert!(markets.is_empty()),
            _ => panic!("Expected Subscribe variant"),
        }
    }

    #[test]
    fn test_stream_command_unsubscribe_multiple() {
        let cmd = StreamCommand::Unsubscribe(vec!["KRW-BTC".to_string(), "KRW-ETH".to_string()]);
        match cmd {
            StreamCommand::Unsubscribe(markets) => assert_eq!(markets.len(), 2),
            _ => panic!("Expected Unsubscribe variant"),
        }
    }
}
