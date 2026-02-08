//! 토큰 버킷 기반 API 레이트 리밋터.
//!
//! 각 거래소 API의 초당 요청 제한을 준수하기 위한 비동기 레이트 리밋터입니다.
//! 토큰 버킷 알고리즘을 사용하되, 버스트 용량을 제한하여
//! 거래소 API의 슬라이딩 윈도우 방식 레이트 리밋에 대응합니다.

use std::sync::Mutex;
use tokio::time::Instant;
use tracing::trace;

/// 토큰 버킷 내부 상태.
struct BucketState {
    /// 현재 사용 가능한 토큰 수.
    tokens: f64,
    /// 마지막 토큰 리필 시각.
    last_refill: Instant,
}

/// 토큰 버킷 기반 레이트 리밋터.
///
/// 초당 최대 요청 수를 제한합니다. 토큰이 부족하면
/// 필요한 시간만큼 비동기로 대기합니다.
///
/// `burst`를 작게 설정하면 요청이 균일하게 분산되어
/// 거래소 API의 슬라이딩 윈도우 레이트 리밋에 안전합니다.
///
/// # 사용 예시
///
/// ```ignore
/// let limiter = RateLimiter::new("upbit", 8, 2); // 초당 8회, 버스트 2
/// limiter.acquire().await; // 토큰 획득 (필요 시 대기)
/// // API 호출 실행
/// ```
pub struct RateLimiter {
    state: Mutex<BucketState>,
    /// 버킷 최대 용량 (버스트 허용량).
    capacity: f64,
    /// 초당 리필 속도.
    refill_rate: f64,
    /// 디버그용 이름.
    name: String,
}

impl RateLimiter {
    /// 새 레이트 리밋터를 생성합니다.
    ///
    /// # 인자
    ///
    /// * `name` - 디버그 로그에 표시할 이름
    /// * `max_per_sec` - 초당 최대 요청 수 (리필 속도)
    /// * `burst` - 최대 버스트 용량 (동시에 보낼 수 있는 최대 요청 수)
    pub fn new(name: impl Into<String>, max_per_sec: u32, burst: u32) -> Self {
        let refill_rate = f64::from(max_per_sec);
        let capacity = f64::from(burst);
        Self {
            state: Mutex::new(BucketState {
                // 초기 토큰은 1개만: 콜드 스타트 시 버스트 방지
                tokens: 1.0_f64.min(capacity),
                last_refill: Instant::now(),
            }),
            capacity,
            refill_rate,
            name: name.into(),
        }
    }

    /// 토큰 하나를 획득합니다. 토큰이 부족하면 비동기로 대기합니다.
    pub async fn acquire(&self) {
        loop {
            let wait_duration = {
                let mut state = self.state.lock().unwrap();
                let now = Instant::now();

                // 경과 시간에 비례하여 토큰 리필
                let elapsed = now.duration_since(state.last_refill).as_secs_f64();
                state.tokens = (state.tokens + elapsed * self.refill_rate).min(self.capacity);
                state.last_refill = now;

                if state.tokens >= 1.0 {
                    // 토큰 소비
                    state.tokens -= 1.0;
                    trace!(
                        limiter = self.name,
                        remaining = format!("{:.1}", state.tokens),
                        "토큰 획득"
                    );
                    None
                } else {
                    // 토큰 부족: 1개 토큰이 리필될 때까지 대기 시간 계산
                    let deficit = 1.0 - state.tokens;
                    let wait_secs = deficit / self.refill_rate;
                    Some(std::time::Duration::from_secs_f64(wait_secs))
                }
            };

            match wait_duration {
                None => return,
                Some(duration) => {
                    trace!(
                        limiter = self.name,
                        wait_ms = duration.as_millis(),
                        "토큰 부족, 대기 중"
                    );
                    tokio::time::sleep(duration).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_burst_limit() {
        // burst=2: 최대 2개만 즉시 처리
        let limiter = RateLimiter::new("test", 10, 2);

        let start = Instant::now();
        // 초기 토큰 1개 → 1번째 즉시, 2번째는 대기
        limiter.acquire().await;
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 50, "첫 요청은 즉시 처리: {}ms", elapsed.as_millis());

        // 2번째 요청은 ~100ms 대기 (10 req/sec → 100ms/token)
        let start = Instant::now();
        limiter.acquire().await;
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() >= 50,
            "두 번째 요청은 토큰 리필 대기: {}ms",
            elapsed.as_millis()
        );
    }

    #[tokio::test]
    async fn test_rate_limiter_steady_rate() {
        // burst=2, rate=10/sec
        let limiter = RateLimiter::new("test", 10, 2);

        // 5개 요청의 총 시간 측정
        let start = Instant::now();
        for _ in 0..5 {
            limiter.acquire().await;
        }
        let elapsed = start.elapsed();
        // 초기 토큰 1개, 나머지 4개는 100ms 간격 → ~400ms
        assert!(
            elapsed.as_millis() >= 300,
            "5개 요청은 최소 300ms 소요: {}ms",
            elapsed.as_millis()
        );
    }

    #[tokio::test]
    async fn test_rate_limiter_refill_after_idle() {
        let limiter = RateLimiter::new("test", 10, 2);

        // 초기 토큰 소비
        limiter.acquire().await;

        // 300ms 대기 → 약 3개 토큰 리필 (burst=2로 cap)
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;

        // burst=2이므로 2개까지 즉시 가능
        let start = Instant::now();
        limiter.acquire().await;
        limiter.acquire().await;
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 50,
            "리필 후 burst까지 즉시 처리: {}ms",
            elapsed.as_millis()
        );
    }

    #[tokio::test]
    async fn test_rate_limiter_throttle() {
        let limiter = RateLimiter::new("test", 10, 2);

        // 초기 토큰 소비
        limiter.acquire().await;

        // 즉시 다음 요청 → 대기 필요
        let start = Instant::now();
        limiter.acquire().await;
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() >= 80,
            "토큰 부족 시 대기해야 합니다: {}ms",
            elapsed.as_millis()
        );
    }
}
