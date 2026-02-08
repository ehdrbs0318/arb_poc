//! 슬리피지/볼륨 모델.
//!
//! 캔들 거래량 대비 주문 비율(participation rate)을 기반으로
//! 슬리피지를 추정하는 Square-Root Impact 모델을 구현합니다.
//!
//! ## 모델
//!
//! ```text
//! participation_rate = order_size / candle_volume  (양 거래소 모두 USDT 기준으로 환산)
//! slippage_bps = base_bps + impact_coeff × √participation_rate × 10000
//! ```
//!
//! - **Upbit**: 캔들 volume이 코인 수량이므로 `volume_usdt = volume × close`로 환산
//! - **Bybit**: 캔들 volume이 코인 수량(base coin)이므로 동일하게 `volume × close`로 환산
//!   (Bybit API의 `turnover` 필드가 USDT 금액이지만, Candle 구조체에는 `volume`만 저장됨)

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use tracing::debug;

use crate::common::fee::roundtrip_fee_pct;

/// 슬리피지 계산 결과.
#[derive(Debug, Clone)]
pub struct SlippageResult {
    /// 참여율 (주문 USDT / 캔들 거래량 USDT).
    pub participation_rate: f64,
    /// 슬리피지 (bps).
    pub slippage_bps: f64,
    /// 슬리피지 적용 후 조정 가격.
    pub adjusted_price: Decimal,
}

/// 슬리피지를 계산합니다.
///
/// # 인자
///
/// - `order_usdt`: 주문 크기 (USDT, 단일 leg)
/// - `candle_volume`: 캔들 거래량 (코인 수량 또는 USDT, `is_coin_volume` 참조)
/// - `close_price`: 캔들 종가
/// - `is_coin_volume`: true면 볼륨 단위가 코인 수량 (Upbit/Bybit 모두 해당), false면 USDT
/// - `is_buy`: true면 매수 (가격 불리: 상승), false면 매도 (가격 불리: 하락)
/// - `max_participation_rate`: 최대 참여율 한도 (초과 시 None)
/// - `base_bps`: 기본 슬리피지 (bps)
/// - `impact_coeff`: 충격 계수
///
/// # 반환
///
/// 참여율이 `max_participation_rate`를 초과하면 `None`.
/// 캔들 거래량이 0이면 `None` (유동성 없음).
#[allow(clippy::too_many_arguments)]
pub fn calculate_slippage(
    order_usdt: Decimal,
    candle_volume: Decimal,
    close_price: Decimal,
    is_coin_volume: bool,
    is_buy: bool,
    max_participation_rate: f64,
    base_bps: f64,
    impact_coeff: f64,
) -> Option<SlippageResult> {
    // 거래량 0 → 유동성 없음
    if candle_volume <= Decimal::ZERO || close_price <= Decimal::ZERO {
        return None;
    }

    // USDT 기준 거래량 통일
    let volume_usdt = if is_coin_volume {
        // Upbit: coin 수량 × close 가격 = USDT 환산
        candle_volume * close_price
    } else {
        // Bybit: 이미 USDT
        candle_volume
    };

    let volume_usdt_f64 = volume_usdt.to_f64().unwrap_or(0.0);
    let order_usdt_f64 = order_usdt.to_f64().unwrap_or(0.0);

    if volume_usdt_f64 <= 0.0 {
        return None;
    }

    // 참여율
    let participation_rate = order_usdt_f64 / volume_usdt_f64;

    // 참여율 초과 체크
    if participation_rate > max_participation_rate {
        debug!(
            participation_rate = participation_rate,
            max = max_participation_rate,
            order_usdt = order_usdt_f64,
            volume_usdt = volume_usdt_f64,
            "참여율 초과: 주문 거부"
        );
        return None;
    }

    // Square-Root Impact: slippage_bps = base + coeff × √(participation_rate) × 10000
    let slippage_bps = base_bps + impact_coeff * participation_rate.sqrt() * 10000.0;

    // 가격 조정: 매수 시 불리 = 가격 ↑, 매도 시 불리 = 가격 ↓
    let slip_factor = slippage_bps / 10000.0;
    let factor = if is_buy {
        1.0 + slip_factor
    } else {
        1.0 - slip_factor
    };

    let factor_dec = Decimal::try_from(factor).unwrap_or(Decimal::ONE);
    let adjusted_price = close_price * factor_dec;

    Some(SlippageResult {
        participation_rate,
        slippage_bps,
        adjusted_price,
    })
}

/// 슬리피지 포함 라운드트립 수익성을 검증합니다.
///
/// 진입 슬리피지가 적용된 가격과 추정 청산 슬리피지를 포함하여
/// 기대 수익이 양수인지 판단합니다.
///
/// # 반환
///
/// `(수익성 여부, 조정된 기대 수익률 %)`
#[allow(clippy::too_many_arguments)]
pub fn is_entry_profitable(
    adjusted_upbit_price: Decimal,
    adjusted_bybit_price: Decimal,
    mean_spread_pct: f64,
    upbit_entry_slippage_bps: f64,
    bybit_entry_slippage_bps: f64,
    upbit_taker_fee: Decimal,
    bybit_taker_fee: Decimal,
) -> (bool, f64) {
    let adj_upbit = adjusted_upbit_price.to_f64().unwrap_or(0.0);
    let adj_bybit = adjusted_bybit_price.to_f64().unwrap_or(0.0);

    // 슬리피지 적용 후 실제 진입 스프레드
    let adjusted_entry_spread = if adj_upbit > 0.0 {
        (adj_bybit - adj_upbit) / adj_upbit * 100.0
    } else {
        0.0
    };

    // 라운드트립 수수료 (%)
    let fee_pct = roundtrip_fee_pct(upbit_taker_fee, bybit_taker_fee)
        .to_f64()
        .unwrap_or(0.0);

    // 청산 슬리피지 추정 (동일 볼륨 가정, bps → %)
    let exit_slippage_cost_pct = (upbit_entry_slippage_bps + bybit_entry_slippage_bps) / 100.0;

    // 조정된 기대 수익
    let adjusted_profit =
        (adjusted_entry_spread - mean_spread_pct) - fee_pct - exit_slippage_cost_pct;

    (adjusted_profit > 0.0, adjusted_profit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_volume_returns_none() {
        let result = calculate_slippage(
            Decimal::new(1000, 0),
            Decimal::ZERO,
            Decimal::new(100_000, 0),
            true,
            true,
            0.1,
            1.0,
            0.001,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_participation_rate_exceeded() {
        // 주문 1000 USDT, 볼륨 100 USDT → 참여율 10배 > 10%
        let result = calculate_slippage(
            Decimal::new(1000, 0),
            Decimal::new(100, 0),
            Decimal::new(1, 0),
            false,
            true,
            0.1,
            1.0,
            0.001,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_slippage_buy_increases_price() {
        // 주문 100 USDT, Bybit 볼륨 10000 USDT → 참여율 1%
        let result = calculate_slippage(
            Decimal::new(100, 0),
            Decimal::new(10000, 0),
            Decimal::new(50000, 0),
            false,
            true, // 매수
            0.1,
            1.0,
            0.001,
        );
        let r = result.unwrap();
        assert!(r.adjusted_price > Decimal::new(50000, 0));
        assert!(r.participation_rate > 0.0 && r.participation_rate < 0.1);
        assert!(r.slippage_bps > 0.0);
    }

    #[test]
    fn test_slippage_sell_decreases_price() {
        // 매도 시 가격 하락
        let result = calculate_slippage(
            Decimal::new(100, 0),
            Decimal::new(10000, 0),
            Decimal::new(50000, 0),
            false,
            false, // 매도
            0.1,
            1.0,
            0.001,
        );
        let r = result.unwrap();
        assert!(r.adjusted_price < Decimal::new(50000, 0));
    }

    #[test]
    fn test_upbit_coin_volume_conversion() {
        // Upbit: 볼륨 1.0 BTC, close 100_000 USDT → volume_usdt = 100_000
        // 주문 1000 USDT → 참여율 = 1000 / 100_000 = 1%
        let result = calculate_slippage(
            Decimal::new(1000, 0),
            Decimal::new(1, 0), // 1 BTC
            Decimal::new(100_000, 0),
            true, // 코인 단위
            true,
            0.1,
            1.0,
            0.001,
        );
        let r = result.unwrap();
        assert!((r.participation_rate - 0.01).abs() < 0.001);
    }

    #[test]
    fn test_bybit_usdt_volume() {
        // Bybit: 볼륨 100_000 USDT, 주문 1000 USDT → 참여율 = 1%
        let result = calculate_slippage(
            Decimal::new(1000, 0),
            Decimal::new(100_000, 0),
            Decimal::new(50000, 0),
            false, // USDT 단위
            true,
            0.1,
            1.0,
            0.001,
        );
        let r = result.unwrap();
        assert!((r.participation_rate - 0.01).abs() < 0.001);
    }

    #[test]
    fn test_very_small_participation() {
        // 참여율이 매우 작으면 슬리피지도 거의 base_bps만
        let result = calculate_slippage(
            Decimal::new(10, 0),        // 10 USDT
            Decimal::new(1_000_000, 0), // 1M USDT
            Decimal::new(50000, 0),
            false,
            true,
            0.1,
            1.0, // base 1 bps
            0.001,
        );
        let r = result.unwrap();
        // participation = 0.00001, sqrt = 0.00316, impact = 0.00316 * 10000 * 0.001 = 0.0316
        // total ≈ 1.03 bps
        assert!(r.slippage_bps < 2.0);
        assert!(r.slippage_bps >= 1.0);
    }

    #[test]
    fn test_negative_close_price_returns_none() {
        let result = calculate_slippage(
            Decimal::new(1000, 0),
            Decimal::new(100, 0),
            Decimal::new(-50000, 0),
            false,
            true,
            0.1,
            1.0,
            0.001,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_exact_max_participation_allowed() {
        // 정확히 max 참여율 = 10%, 주문 1000 USDT, 볼륨 10000 USDT
        let result = calculate_slippage(
            Decimal::new(1000, 0),
            Decimal::new(10000, 0),
            Decimal::new(50000, 0),
            false,
            true,
            0.1,
            1.0,
            0.001,
        );
        let r = result.unwrap();
        assert!((r.participation_rate - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_slippage_disabled_with_zero_coefficients() {
        // base=0, coeff=0 → 슬리피지 0
        let result = calculate_slippage(
            Decimal::new(100, 0),
            Decimal::new(10000, 0),
            Decimal::new(50000, 0),
            false,
            true,
            1.0,
            0.0,
            0.0,
        );
        let r = result.unwrap();
        assert_eq!(r.slippage_bps, 0.0);
        assert_eq!(r.adjusted_price, Decimal::new(50000, 0));
    }

    // --- is_entry_profitable 테스트 ---

    #[test]
    fn test_is_entry_profitable_no_slippage() {
        // Upbit 100000, Bybit 100300 → 스프레드 0.3%, mean=0, slippage=0, fee=0.21%
        // profit = 0.3 - 0 - 0.21 - 0 = 0.09
        let upbit_fee = Decimal::new(5, 4); // 0.0005
        let bybit_fee = Decimal::new(55, 5); // 0.00055
        let (profitable, profit) = is_entry_profitable(
            Decimal::new(100_000, 0),
            Decimal::new(100_300, 0),
            0.0,
            0.0,
            0.0,
            upbit_fee,
            bybit_fee,
        );
        assert!(profitable);
        assert!((profit - 0.09).abs() < 0.01);
    }

    #[test]
    fn test_is_entry_profitable_with_moderate_slippage() {
        // 스프레드 충분히 넓음 (0.5%), slippage_bps=1.5/1.5 → exit slippage=0.03%
        // profit = 0.5 - 0.0 - 0.21 - 0.03 = 0.26
        let upbit_fee = Decimal::new(5, 4);
        let bybit_fee = Decimal::new(55, 5);
        let (profitable, profit) = is_entry_profitable(
            Decimal::new(100_000, 0),
            Decimal::new(100_500, 0),
            0.0,
            1.5,
            1.5,
            upbit_fee,
            bybit_fee,
        );
        assert!(profitable);
        assert!(profit > 0.2);
    }

    #[test]
    fn test_is_entry_profitable_rejected_by_slippage() {
        // 스프레드 좁음 (0.25%), mean=0, slippage_bps=10/10 → exit slippage=0.2%
        // profit = 0.25 - 0 - 0.21 - 0.2 = -0.16
        let upbit_fee = Decimal::new(5, 4);
        let bybit_fee = Decimal::new(55, 5);
        let (profitable, profit) = is_entry_profitable(
            Decimal::new(100_000, 0),
            Decimal::new(100_250, 0),
            0.0,
            10.0,
            10.0,
            upbit_fee,
            bybit_fee,
        );
        assert!(!profitable);
        assert!(profit < 0.0);
    }

    #[test]
    fn test_is_entry_profitable_borderline() {
        // 경계값: profit ≈ 0에 가깝지만 ≤ 0 → 거부
        // 스프레드 0.21%, mean=0, slippage=0 → profit = 0.21 - 0 - 0.21 - 0 = 0.0
        let upbit_fee = Decimal::new(5, 4);
        let bybit_fee = Decimal::new(55, 5);
        let (profitable, profit) = is_entry_profitable(
            Decimal::new(100_000, 0),
            Decimal::new(100_210, 0),
            0.0,
            0.0,
            0.0,
            upbit_fee,
            bybit_fee,
        );
        assert!(!profitable);
        assert!(profit <= 0.001); // 부동소수점 오차 허용
    }

    #[test]
    fn test_is_entry_profitable_negative_mean() {
        // mean=-0.1 (역김프), 스프레드 0.3% → 마진 넓어짐
        // profit = 0.3 - (-0.1) - 0.21 - 0 = 0.19
        let upbit_fee = Decimal::new(5, 4);
        let bybit_fee = Decimal::new(55, 5);
        let (profitable, profit) = is_entry_profitable(
            Decimal::new(100_000, 0),
            Decimal::new(100_300, 0),
            -0.1,
            0.0,
            0.0,
            upbit_fee,
            bybit_fee,
        );
        assert!(profitable);
        assert!((profit - 0.19).abs() < 0.01);
    }
}
