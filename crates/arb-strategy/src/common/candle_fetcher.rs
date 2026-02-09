//! 페이지네이션 기반 캔들 데이터 수집.
//!
//! `get_candles_before` API를 반복 호출하여 대량의 캔들 데이터를 수집합니다.
//! 워밍업 및 기타 용도에서 공통으로 사용합니다.

use std::time::Duration;

use chrono::{DateTime, Utc};
use tracing::debug;

use arb_exchange::MarketData;
use arb_exchange::types::{Candle, CandleInterval};

use crate::error::StrategyError;

/// 페이지네이션으로 전체 기간 캔들을 수집합니다.
///
/// `get_candles_before`는 `before` exclusive이고 결과를 timestamp 오름차순 반환.
pub async fn fetch_all_candles<M: MarketData>(
    client: &M,
    market: &str,
    interval: CandleInterval,
    total_count: usize,
    end_time: DateTime<Utc>,
    page_size: u32,
    delay: Duration,
) -> Result<Vec<Candle>, StrategyError> {
    let mut collected: Vec<Candle> = Vec::new();
    let mut cursor = end_time;

    while collected.len() < total_count {
        tokio::time::sleep(delay).await;

        let batch = client
            .get_candles_before(market, interval, page_size, cursor)
            .await?;

        if batch.is_empty() {
            break;
        }

        // batch는 오름차순 (trait 규약)
        // 중복 제거: collected에 이미 있는 데이터보다 이전인 캔들만 유지
        let filtered: Vec<Candle> = if collected.is_empty() {
            batch
        } else {
            let earliest = collected.first().unwrap().timestamp;
            batch
                .into_iter()
                .filter(|c| c.timestamp < earliest)
                .collect()
        };

        if filtered.is_empty() {
            break;
        }

        // 커서를 가장 오래된 캔들 시간으로 이동
        cursor = filtered.first().unwrap().timestamp;

        // prepend: 역순 수집 후 나중에 정렬
        let mut new_collected = filtered;
        new_collected.append(&mut collected);
        collected = new_collected;
    }

    // 필요 개수 초과분 제거 (앞쪽에서 자름)
    if collected.len() > total_count {
        collected = collected.split_off(collected.len() - total_count);
    }

    debug!(
        market = market,
        collected = collected.len(),
        total_needed = total_count,
        "캔들 데이터 수집 완료"
    );

    Ok(collected)
}
