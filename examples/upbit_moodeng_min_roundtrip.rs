//! Upbit MOODENG 최소 실주문 왕복 테스트 예제.
//!
//! 이 예제는 config.toml의 Upbit API 키를 읽어 실제 주문을 발주합니다.
//! - 1) KRW-MOODENG IOC 지정가 매수
//! - 2) 체결 수량만큼 IOC 지정가 매도
//!
//! 실행 전 반드시 소액으로 테스트하세요.
//!
//! 실행:
//!   cargo run --example upbit_moodeng_min_roundtrip
//!
//! 선택 환경변수:
//! - `UPBIT_TEST_MARKET` (기본: KRW-MOODENG)
//! - `UPBIT_TEST_TARGET_KRW` (기본: 7000)

use std::str::FromStr;
use std::time::Duration;

use arb_poc::config::Config;
use arb_poc::exchange::{
    MarketData, Order, OrderManagement, OrderRequest, OrderStatus, TimeInForce,
};
use arb_poc::exchanges::UpbitClient;
use arb_poc::strategy::zscore::instrument::{ceil_to_step, floor_to_step, upbit_tick_size};
use chrono::Utc;
use rust_decimal::Decimal;
use tokio::time::sleep;

fn read_env_decimal(key: &str, default_value: Decimal) -> Decimal {
    std::env::var(key)
        .ok()
        .and_then(|v| Decimal::from_str(v.trim()).ok())
        .unwrap_or(default_value)
}

async fn wait_for_execution(
    client: &UpbitClient,
    order_id: &str,
    polls: usize,
    poll_interval_ms: u64,
) -> Result<Order, Box<dyn std::error::Error>> {
    let mut latest = client.get_order(order_id).await?;
    for _ in 0..polls {
        if latest.executed_volume > Decimal::ZERO {
            return Ok(latest);
        }
        if matches!(
            latest.status,
            OrderStatus::Cancelled | OrderStatus::Rejected
        ) {
            return Ok(latest);
        }
        sleep(Duration::from_millis(poll_interval_ms)).await;
        latest = client.get_order(order_id).await?;
    }
    Ok(latest)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    if !config.upbit.has_credentials() {
        return Err("Upbit API 키가 설정되지 않았습니다 (config.toml 또는 환경변수)".into());
    }

    let market = std::env::var("UPBIT_TEST_MARKET").unwrap_or_else(|_| "KRW-MOODENG".to_string());
    let target_krw = read_env_decimal("UPBIT_TEST_TARGET_KRW", Decimal::new(7000, 0));
    let min_total_krw = Decimal::new(5000, 0);
    let target_total_krw = if target_krw < min_total_krw {
        min_total_krw
    } else {
        target_krw
    };

    println!("=== Upbit MOODENG 최소 왕복 주문 테스트 ===");
    println!("market={market} target_total_krw={target_total_krw}");

    let client = UpbitClient::with_credentials(&config.upbit.api_key, &config.upbit.secret_key)?;

    let krw_balance = client.get_balance("KRW").await?;
    if krw_balance.balance < target_total_krw {
        return Err(format!(
            "KRW 잔고 부족: available={} required={}",
            krw_balance.balance, target_total_krw
        )
        .into());
    }

    let ob = client.get_orderbook(&market, Some(15)).await?;
    let best_ask = ob.best_ask().ok_or("매수 호가가 없어 주문 불가")?.price;
    let _best_bid = ob.best_bid().ok_or("매도 호가가 없어 주문 불가")?.price;

    // IOC 매수 체결률을 높이기 위해 최우선 매도호가 대비 +0.5%를 사용합니다.
    let buy_price_raw = best_ask * Decimal::new(1005, 3); // 1.005
    let buy_tick = upbit_tick_size(buy_price_raw);
    let buy_price = ceil_to_step(buy_price_raw, buy_tick);
    let qty_step = Decimal::new(1, 8);
    let buy_qty = ceil_to_step(target_total_krw / buy_price, qty_step);
    let buy_notional = buy_qty * buy_price;

    println!(
        "[BUY] best_ask={} buy_price={} qty={} notional_krw={}",
        best_ask, buy_price, buy_qty, buy_notional
    );

    let buy_req = OrderRequest::limit_buy(market.clone(), buy_price, buy_qty)
        .with_time_in_force(TimeInForce::Ioc)
        .with_identifier(format!("moodeng-buy-{}", Utc::now().timestamp_millis()));
    let mut buy_order = client.place_order(&buy_req).await?;

    if buy_order.executed_volume <= Decimal::ZERO {
        buy_order = wait_for_execution(&client, &buy_order.id, 10, 200).await?;
    }

    println!(
        "[BUY RESULT] id={} status={:?} requested={} executed={} avg_price={:?}",
        buy_order.id,
        buy_order.status,
        buy_order.volume,
        buy_order.executed_volume,
        buy_order.avg_price
    );

    if buy_order.executed_volume <= Decimal::ZERO {
        if matches!(buy_order.status, OrderStatus::Wait | OrderStatus::Watch) {
            let cancelled = client.cancel_order(&buy_order.id).await?;
            println!(
                "[BUY CANCEL] id={} status={:?} executed={}",
                cancelled.id, cancelled.status, cancelled.executed_volume
            );
        }

        println!("[BUY FALLBACK] IOC 미체결 -> 시장가 매수 재시도");
        let buy_fallback_req = OrderRequest::market_buy(market.clone(), target_total_krw)
            .with_identifier(format!("moodeng-buy-mkt-{}", Utc::now().timestamp_millis()));
        let mut buy_fallback = client.place_order(&buy_fallback_req).await?;
        buy_fallback = wait_for_execution(&client, &buy_fallback.id, 20, 200).await?;

        println!(
            "[BUY FALLBACK RESULT] id={} status={:?} requested={} executed={} avg_price={:?}",
            buy_fallback.id,
            buy_fallback.status,
            buy_fallback.volume,
            buy_fallback.executed_volume,
            buy_fallback.avg_price
        );
        if buy_fallback.executed_volume <= Decimal::ZERO {
            return Err("시장가 매수도 체결 수량이 0입니다.".into());
        }
        buy_order = buy_fallback;
    }

    let filled_qty = buy_order.executed_volume;
    let ob2 = client.get_orderbook(&market, Some(15)).await?;
    let best_bid2 = ob2.best_bid().ok_or("매도 호가가 없어 청산 불가")?.price;

    // IOC 매도 체결률을 높이기 위해 최우선 매수호가 대비 -0.5%를 사용합니다.
    let mut sell_price_raw = best_bid2 * Decimal::new(995, 3); // 0.995
    let mut sell_tick = upbit_tick_size(sell_price_raw);
    let mut sell_price = floor_to_step(sell_price_raw, sell_tick);
    let mut sell_notional = filled_qty * sell_price;

    // 매도 최소 주문금액(5,000 KRW) 미달 시, 가격을 최우선 매수호가로 올려 한 번 더 보정합니다.
    if sell_notional < min_total_krw {
        sell_price_raw = best_bid2;
        sell_tick = upbit_tick_size(sell_price_raw);
        sell_price = floor_to_step(sell_price_raw, sell_tick);
        sell_notional = filled_qty * sell_price;
    }

    if sell_notional < min_total_krw {
        return Err(format!(
            "매도 최소 주문금액 미달: filled_qty={} sell_price={} notional={}",
            filled_qty, sell_price, sell_notional
        )
        .into());
    }

    println!(
        "[SELL] best_bid={} sell_price={} qty={} notional_krw={}",
        best_bid2, sell_price, filled_qty, sell_notional
    );

    let sell_req = OrderRequest::limit_sell(market.clone(), sell_price, filled_qty)
        .with_time_in_force(TimeInForce::Ioc)
        .with_identifier(format!("moodeng-sell-{}", Utc::now().timestamp_millis()));
    let mut sell_order = client.place_order(&sell_req).await?;
    if sell_order.executed_volume <= Decimal::ZERO {
        sell_order = wait_for_execution(&client, &sell_order.id, 10, 200).await?;
    }

    println!(
        "[SELL RESULT] id={} status={:?} requested={} executed={} avg_price={:?}",
        sell_order.id,
        sell_order.status,
        sell_order.volume,
        sell_order.executed_volume,
        sell_order.avg_price
    );

    if sell_order.executed_volume < filled_qty {
        let remaining = filled_qty - sell_order.executed_volume;
        if remaining > Decimal::ZERO {
            if matches!(sell_order.status, OrderStatus::Wait | OrderStatus::Watch) {
                let cancelled = client.cancel_order(&sell_order.id).await?;
                println!(
                    "[SELL CANCEL] id={} status={:?} executed={} remaining={}",
                    cancelled.id, cancelled.status, cancelled.executed_volume, remaining
                );
            }

            println!("[SELL FALLBACK] 잔량 {} 시장가 매도", remaining);
            let sell_fallback_req = OrderRequest::market_sell(market.clone(), remaining)
                .with_identifier(format!(
                    "moodeng-sell-mkt-{}",
                    Utc::now().timestamp_millis()
                ));
            let mut sell_fallback = client.place_order(&sell_fallback_req).await?;
            sell_fallback = wait_for_execution(&client, &sell_fallback.id, 20, 200).await?;

            println!(
                "[SELL FALLBACK RESULT] id={} status={:?} requested={} executed={} avg_price={:?}",
                sell_fallback.id,
                sell_fallback.status,
                sell_fallback.volume,
                sell_fallback.executed_volume,
                sell_fallback.avg_price
            );
            if sell_fallback.executed_volume <= Decimal::ZERO {
                return Err("시장가 매도도 체결 수량이 0입니다.".into());
            }
        }
    }

    println!("=== 테스트 완료: 매수/매도 주문이 모두 실행되었습니다 ===");
    Ok(())
}
