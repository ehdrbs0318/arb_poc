//! Yahoo Finance USD/KRW 환율 조회 및 캐싱.
//!
//! 틱 경로(hot path)에서 blocking 없이 환율을 조회할 수 있도록
//! `AtomicU64` 기반 lock-free 캐시를 제공합니다.
//!
//! # 사용 예시
//!
//! ```ignore
//! use arb_forex::ForexCache;
//! use std::sync::Arc;
//! use std::time::Duration;
//!
//! let cache = Arc::new(ForexCache::new(Duration::from_secs(600)));
//! // 초기 환율 조회
//! cache.refresh_if_expired().await?;
//! // 틱 경로에서 즉시 반환
//! let rate = cache.get_cached_rate().unwrap();
//! ```

use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::time::Duration;

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use tracing::{debug, info, warn};

/// Forex 관련 에러.
#[derive(Debug, thiserror::Error)]
pub enum ForexError {
    /// HTTP 요청 실패.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    /// Yahoo Finance API 응답 파싱 실패.
    #[error("Failed to parse Yahoo Finance response: {0}")]
    Parse(String),
    /// 캐시가 비어있음 (초기 조회 전).
    #[error("Forex cache is empty, call refresh_if_expired() first")]
    CacheEmpty,
}

/// Yahoo Finance chart API 응답 구조.
#[derive(Debug, Deserialize)]
struct YahooChartResponse {
    chart: YahooChart,
}

#[derive(Debug, Deserialize)]
struct YahooChart {
    result: Option<Vec<YahooChartResult>>,
    error: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct YahooChartResult {
    timestamp: Option<Vec<i64>>,
    indicators: YahooIndicators,
}

#[derive(Debug, Deserialize)]
struct YahooIndicators {
    quote: Vec<YahooQuote>,
}

#[derive(Debug, Deserialize)]
struct YahooQuote {
    close: Vec<Option<f64>>,
}

/// USD/KRW 환율 캐시.
///
/// Yahoo Finance API에서 현재 USD/KRW 환율을 조회하고
/// TTL 기반으로 캐싱합니다.
///
/// 설계 원칙:
/// - 틱 경로(hot path)에서는 캐시 값만 동기적으로 반환 (blocking I/O 없음)
/// - 환율 갱신은 별도 `tokio::spawn` task에서 비동기로 수행
/// - `AtomicU64`로 read contention 제거
pub struct ForexCache {
    /// HTTP 클라이언트.
    client: reqwest::Client,
    /// 캐시된 환율 (f64를 u64 bits로 저장).
    cached_rate: AtomicU64,
    /// 캐시 갱신 시각 (unix millis).
    cached_at: AtomicI64,
    /// 캐시 TTL.
    ttl: Duration,
}

impl ForexCache {
    /// 새 ForexCache를 생성합니다.
    ///
    /// # 인자
    /// - `ttl`: 캐시 TTL (e.g., 10분)
    pub fn new(ttl: Duration) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .user_agent("Mozilla/5.0 (compatible; arb-forex/0.1)")
                .build()
                .expect("Failed to create HTTP client"),
            cached_rate: AtomicU64::new(0),
            cached_at: AtomicI64::new(0),
            ttl,
        }
    }

    /// 캐시된 USD/KRW 환율을 즉시 반환합니다 (blocking 없음).
    ///
    /// 캐시가 비어있으면 `None` 반환.
    /// 틱 경로(hot path)에서 사용합니다.
    pub fn get_cached_rate(&self) -> Option<f64> {
        let bits = self.cached_rate.load(Ordering::Relaxed);
        if bits == 0 {
            return None;
        }
        Some(f64::from_bits(bits))
    }

    /// 캐시가 만료되었는지 확인합니다.
    fn is_expired(&self) -> bool {
        let cached_at_ms = self.cached_at.load(Ordering::Relaxed);
        if cached_at_ms == 0 {
            return true;
        }
        let now_ms = Utc::now().timestamp_millis();
        let elapsed_ms = now_ms - cached_at_ms;
        elapsed_ms > self.ttl.as_millis() as i64
    }

    /// 테스트용: 캐시에 환율을 직접 설정합니다.
    ///
    /// 프로덕션 코드에서는 `refresh_if_expired()`를 사용하세요.
    pub fn update_cache_for_test(&self, rate: f64) {
        self.update_cache(rate);
    }

    /// 내부 캐시를 갱신합니다.
    fn update_cache(&self, rate: f64) {
        self.cached_rate.store(rate.to_bits(), Ordering::Relaxed);
        self.cached_at
            .store(Utc::now().timestamp_millis(), Ordering::Relaxed);
        debug!(rate = rate, "USD/KRW 환율 캐시 갱신");
    }

    /// USD/KRW 환율을 Yahoo Finance에서 조회하고 캐시를 갱신합니다.
    ///
    /// TTL 만료 시에만 실제 HTTP 요청을 발행합니다.
    /// 별도 갱신 task 또는 초기화 시 호출합니다.
    pub async fn refresh_if_expired(&self) -> Result<f64, ForexError> {
        // TTL 미만료 시 캐시값 반환
        if !self.is_expired()
            && let Some(rate) = self.get_cached_rate()
        {
            return Ok(rate);
        }

        // Yahoo Finance API 호출
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/USDKRW=X?interval=1m&range=1d";
        debug!(url = url, "USD/KRW 환율 조회 요청");

        let resp = self.client.get(url).send().await?;
        let body: YahooChartResponse = resp.json().await?;

        // 에러 응답 확인
        if let Some(err) = body.chart.error {
            return Err(ForexError::Parse(format!("Yahoo Finance API error: {err}")));
        }

        let result = body
            .chart
            .result
            .and_then(|r| r.into_iter().next())
            .ok_or_else(|| ForexError::Parse("Empty result array".to_string()))?;

        // 가장 최근 유효한 close 값을 찾음
        let rate = result
            .indicators
            .quote
            .first()
            .and_then(|q| q.close.iter().rev().find_map(|c| *c))
            .ok_or_else(|| ForexError::Parse("No valid close price found".to_string()))?;

        // 환율 유효성 검증 (USD/KRW는 일반적으로 1000~2000 범위)
        if !(500.0..=3000.0).contains(&rate) {
            return Err(ForexError::Parse(format!(
                "USD/KRW rate {rate} is out of valid range (500~3000)"
            )));
        }

        self.update_cache(rate);
        info!(rate = rate, "USD/KRW 환율 갱신 완료");

        Ok(rate)
    }

    /// 특정 기간의 일봉 USD/KRW 환율을 조회합니다 (워밍업용).
    ///
    /// 각 날짜의 close 환율을 반환합니다.
    pub async fn get_daily_rates(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<(DateTime<Utc>, f64)>, ForexError> {
        let period1 = from.timestamp();
        let period2 = to.timestamp();

        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/USDKRW=X?interval=1d&period1={period1}&period2={period2}"
        );
        debug!(url = %url, "USD/KRW 일봉 환율 조회 요청");

        let resp = self.client.get(&url).send().await?;
        let body: YahooChartResponse = resp.json().await?;

        if let Some(err) = body.chart.error {
            return Err(ForexError::Parse(format!("Yahoo Finance API error: {err}")));
        }

        let result = body
            .chart
            .result
            .and_then(|r| r.into_iter().next())
            .ok_or_else(|| ForexError::Parse("Empty result array".to_string()))?;

        let timestamps = result
            .timestamp
            .ok_or_else(|| ForexError::Parse("No timestamps in response".to_string()))?;

        let closes = result
            .indicators
            .quote
            .first()
            .map(|q| &q.close)
            .ok_or_else(|| ForexError::Parse("No quote data in response".to_string()))?;

        let mut rates = Vec::new();
        for (ts, close) in timestamps.iter().zip(closes.iter()) {
            if let Some(rate) = close {
                // 유효성 검증
                if (500.0..=3000.0).contains(rate) {
                    let dt = Utc.timestamp_opt(*ts, 0).single().unwrap_or(Utc::now());
                    rates.push((dt, *rate));
                } else {
                    warn!(
                        rate = rate,
                        timestamp = ts,
                        "일봉 환율이 유효 범위 밖, 건너뜀"
                    );
                }
            }
        }

        info!(count = rates.len(), from = %from, to = %to, "일봉 환율 조회 완료");

        // 캐시 갱신: 가장 최근 일봉을 캐시에 저장
        if let Some((_, last_rate)) = rates.last() {
            // 현재 캐시가 비어있을 때만 (초기 워밍업 시)
            if self.get_cached_rate().is_none() {
                self.update_cache(*last_rate);
                info!(rate = last_rate, "워밍업 환율로 캐시 초기화");
            }
        }

        Ok(rates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_initially_empty() {
        let cache = ForexCache::new(Duration::from_secs(600));
        assert!(cache.get_cached_rate().is_none());
        assert!(cache.is_expired());
    }

    #[test]
    fn test_cache_update_and_read() {
        let cache = ForexCache::new(Duration::from_secs(600));
        cache.update_cache(1450.5);

        let rate = cache.get_cached_rate();
        assert!(rate.is_some());
        assert!((rate.unwrap() - 1450.5).abs() < 0.001);
    }

    #[test]
    fn test_cache_ttl_not_expired() {
        let cache = ForexCache::new(Duration::from_secs(600));
        cache.update_cache(1450.0);

        // 방금 갱신했으므로 만료되지 않음
        assert!(!cache.is_expired());
    }

    #[test]
    fn test_cache_ttl_expired() {
        let cache = ForexCache::new(Duration::from_secs(1)); // 1초 TTL

        // 과거 시각으로 캐시 설정
        cache
            .cached_rate
            .store((1450.0_f64).to_bits(), Ordering::Relaxed);
        let past_ms = Utc::now().timestamp_millis() - 2000; // 2초 전
        cache.cached_at.store(past_ms, Ordering::Relaxed);

        assert!(cache.is_expired());
    }

    #[test]
    fn test_rate_validation_range() {
        // 유효 범위: 500~3000
        let valid_rates = [500.0, 1000.0, 1450.5, 2000.0, 3000.0];
        for rate in valid_rates {
            assert!(
                (500.0..=3000.0).contains(&rate),
                "Rate {rate} should be valid"
            );
        }

        let invalid_rates = [0.0, 100.0, 499.9, 3000.1, 5000.0];
        for rate in invalid_rates {
            assert!(
                !(500.0..=3000.0).contains(&rate),
                "Rate {rate} should be invalid"
            );
        }
    }
}
