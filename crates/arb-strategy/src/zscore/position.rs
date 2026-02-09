//! 가상 포지션 관리.
//!
//! 시뮬레이션용 가상 포지션 생성, 관리, 청산을 담당합니다.
//! Bybit Isolated Margin 기반 liquidation price 계산을 포함합니다.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use tracing::{info, warn};

use crate::error::PositionError;
use crate::zscore::pnl::ClosedPosition;

/// 가상 포지션.
///
/// Upbit 현물 매수 + Bybit 선물 short 한 쌍을 나타냅니다.
#[derive(Debug, Clone)]
pub struct VirtualPosition {
    /// 코인 심볼.
    pub coin: String,
    /// 진입 시간.
    pub entry_time: DateTime<Utc>,
    /// Upbit 현물 진입가 (USDT 환산).
    pub upbit_entry_price: Decimal,
    /// Bybit short 진입가 (USDT).
    pub bybit_entry_price: Decimal,
    /// Bybit liquidation price (Isolated Margin 기준).
    pub bybit_liquidation_price: Decimal,
    /// 진입 시 USD/KRW 환율 (사후 분석용).
    pub entry_usd_krw: f64,
    /// 진입 시 스프레드 (%).
    pub entry_spread_pct: f64,
    /// 진입 시 Z-Score.
    pub entry_z_score: f64,
    /// 포지션 크기 (USDT, 단일 leg 기준).
    pub size_usdt: Decimal,
}

/// 포지션 매니저.
///
/// 활성 포지션과 청산된 포지션 이력을 관리합니다.
#[derive(Debug, Clone)]
pub struct PositionManager {
    /// 활성 포지션 (코인별 최대 1개).
    pub open_positions: HashMap<String, VirtualPosition>,
    /// 청산된 포지션 이력.
    pub closed_positions: Vec<ClosedPosition>,
}

impl PositionManager {
    /// 새 PositionManager를 생성합니다.
    pub fn new() -> Self {
        Self {
            open_positions: HashMap::new(),
            closed_positions: Vec::new(),
        }
    }

    /// 현재 사용 중인 자본 합계 (양 leg 합산).
    pub fn used_capital(&self) -> Decimal {
        self.open_positions
            .values()
            .map(|p| p.size_usdt * Decimal::from(2u64))
            .sum()
    }

    /// 가용 자본 확인.
    pub fn available_capital(&self, total_capital: Decimal) -> Decimal {
        total_capital - self.used_capital()
    }

    /// 해당 코인에 대한 포지션이 있는지 확인합니다.
    pub fn has_position(&self, coin: &str) -> bool {
        self.open_positions.contains_key(coin)
    }

    /// 현재 열린 포지션 수를 반환합니다.
    pub fn open_count(&self) -> usize {
        self.open_positions.len()
    }

    /// 새 포지션을 추가합니다.
    pub fn open_position(&mut self, position: VirtualPosition) -> Result<(), PositionError> {
        if self.open_positions.contains_key(&position.coin) {
            warn!(
                coin = %position.coin,
                "포지션이 이미 존재하여 진입 불가"
            );
            return Err(PositionError::AlreadyExists {
                coin: position.coin.clone(),
            });
        }
        info!(
            coin = %position.coin,
            size_usdt = %position.size_usdt,
            upbit_entry_price = %position.upbit_entry_price,
            bybit_entry_price = %position.bybit_entry_price,
            liquidation_price = %position.bybit_liquidation_price,
            "포지션 진입 완료"
        );
        self.open_positions.insert(position.coin.clone(), position);
        Ok(())
    }

    /// 포지션을 청산하고 ClosedPosition을 반환합니다.
    #[allow(clippy::too_many_arguments)]
    pub fn close_position(
        &mut self,
        coin: &str,
        exit_time: DateTime<Utc>,
        exit_upbit_usdt_price: Decimal,
        exit_bybit_price: Decimal,
        exit_usd_krw: f64,
        exit_spread_pct: f64,
        exit_z_score: f64,
        upbit_taker_fee: Decimal,
        bybit_taker_fee: Decimal,
        is_liquidated: bool,
    ) -> Result<ClosedPosition, PositionError> {
        let pos = self.open_positions.remove(coin).ok_or_else(|| {
            warn!(coin = %coin, "청산할 포지션이 존재하지 않음");
            PositionError::NotFound {
                coin: coin.to_string(),
            }
        })?;

        let holding_minutes = (exit_time - pos.entry_time).num_minutes().unsigned_abs();

        // USDT notional matching
        let upbit_qty = pos.size_usdt / pos.upbit_entry_price;
        let bybit_qty = pos.size_usdt / pos.bybit_entry_price;

        // Upbit 현물 PnL
        let upbit_pnl = (exit_upbit_usdt_price - pos.upbit_entry_price) * upbit_qty;

        // Bybit short PnL
        let bybit_pnl = (pos.bybit_entry_price - exit_bybit_price) * bybit_qty;

        // 수수료 (진입 + 청산)
        let upbit_fees = pos.size_usdt * upbit_taker_fee * Decimal::from(2u64);
        let bybit_fees = pos.size_usdt * bybit_taker_fee * Decimal::from(2u64);
        let total_fees = upbit_fees + bybit_fees;

        // 순 PnL
        let net_pnl = upbit_pnl + bybit_pnl - total_fees;

        let closed = ClosedPosition {
            coin: pos.coin,
            entry_time: pos.entry_time,
            exit_time,
            holding_minutes,
            size_usdt: pos.size_usdt,
            upbit_pnl,
            bybit_pnl,
            upbit_fees,
            bybit_fees,
            total_fees,
            net_pnl,
            entry_z_score: pos.entry_z_score,
            exit_z_score,
            entry_spread_pct: pos.entry_spread_pct,
            exit_spread_pct,
            entry_usd_krw: pos.entry_usd_krw,
            exit_usd_krw,
            is_liquidated,
        };

        info!(
            coin = %closed.coin,
            holding_minutes = %closed.holding_minutes,
            upbit_pnl = %closed.upbit_pnl,
            bybit_pnl = %closed.bybit_pnl,
            total_fees = %closed.total_fees,
            net_pnl = %closed.net_pnl,
            is_liquidated = %closed.is_liquidated,
            "포지션 청산 완료"
        );

        self.closed_positions.push(closed.clone());
        Ok(closed)
    }

    /// Bybit liquidation 체크: 현재 가격이 liquidation price 이상이면 강제 청산.
    pub fn check_liquidation(&self, coin: &str, current_bybit_price: Decimal) -> bool {
        let Some(p) = self.open_positions.get(coin) else {
            return false;
        };

        let liquidated = current_bybit_price >= p.bybit_liquidation_price;

        if liquidated {
            // 강제 청산 조건 도달 경고
            warn!(
                coin = %coin,
                current_bybit_price = %current_bybit_price,
                liquidation_price = %p.bybit_liquidation_price,
                "Bybit liquidation 조건 도달"
            );
        }

        liquidated
    }
}

impl Default for PositionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Bybit short Isolated Margin liquidation price를 계산합니다.
///
/// `liq_price = entry_price × (1 + 1/leverage - MMR - bybit_taker_fee)`
pub fn calculate_liquidation_price(
    entry_price: Decimal,
    leverage: u32,
    mmr: Decimal,
    bybit_taker_fee: Decimal,
) -> Decimal {
    let leverage_dec = Decimal::from(leverage);
    entry_price * (Decimal::ONE + Decimal::ONE / leverage_dec - mmr - bybit_taker_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_position(coin: &str, size: i64, entry_price: i64) -> VirtualPosition {
        VirtualPosition {
            coin: coin.to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(entry_price, 0),
            bybit_entry_price: Decimal::new(entry_price + 50, 0),
            bybit_liquidation_price: Decimal::new(entry_price * 2, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            size_usdt: Decimal::new(size, 0),
        }
    }

    #[test]
    fn test_new_position_manager() {
        let pm = PositionManager::new();
        assert_eq!(pm.open_count(), 0);
        assert_eq!(pm.used_capital(), Decimal::ZERO);
        assert!(!pm.has_position("BTC"));
    }

    #[test]
    fn test_open_position() {
        let mut pm = PositionManager::new();
        let pos = make_position("BTC", 1000, 100_000);
        pm.open_position(pos).unwrap();
        assert!(pm.has_position("BTC"));
        assert_eq!(pm.open_count(), 1);
        assert_eq!(pm.used_capital(), Decimal::new(2000, 0));
    }

    #[test]
    fn test_open_duplicate_position() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1000, 100_000))
            .unwrap();
        let result = pm.open_position(make_position("BTC", 500, 100_000));
        assert!(result.is_err());
    }

    #[test]
    fn test_available_capital() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1000, 100_000))
            .unwrap();
        let total = Decimal::new(10_000, 0);
        assert_eq!(pm.available_capital(total), Decimal::new(8000, 0));
    }

    #[test]
    fn test_close_position() {
        let mut pm = PositionManager::new();
        let entry_time = Utc::now();
        pm.open_positions.insert(
            "BTC".to_string(),
            VirtualPosition {
                coin: "BTC".to_string(),
                entry_time,
                upbit_entry_price: Decimal::new(100_000, 0),
                bybit_entry_price: Decimal::new(100_050, 0),
                bybit_liquidation_price: Decimal::new(199_445, 0),
                entry_usd_krw: 1380.0,
                entry_spread_pct: 0.05,
                entry_z_score: 2.5,
                size_usdt: Decimal::new(1000, 0),
            },
        );

        let exit_time = entry_time + chrono::Duration::minutes(30);
        let closed = pm
            .close_position(
                "BTC",
                exit_time,
                Decimal::new(100_020, 0), // upbit 청산가
                Decimal::new(100_020, 0), // bybit 청산가
                1381.0,                   // exit usd_krw
                0.0,                      // exit spread
                0.3,                      // exit z_score
                Decimal::new(5, 4),       // upbit fee 0.0005
                Decimal::new(55, 5),      // bybit fee 0.00055
                false,
            )
            .unwrap();

        assert_eq!(closed.coin, "BTC");
        assert_eq!(closed.holding_minutes, 30);
        assert!(!closed.is_liquidated);
        assert!(!pm.has_position("BTC"));
        assert_eq!(pm.closed_positions.len(), 1);
    }

    #[test]
    fn test_close_nonexistent_position() {
        let mut pm = PositionManager::new();
        let result = pm.close_position(
            "BTC",
            Utc::now(),
            Decimal::ZERO,
            Decimal::ZERO,
            0.0,
            0.0,
            0.0,
            Decimal::ZERO,
            Decimal::ZERO,
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_check_liquidation() {
        let mut pm = PositionManager::new();
        pm.open_positions.insert(
            "BTC".to_string(),
            VirtualPosition {
                coin: "BTC".to_string(),
                entry_time: Utc::now(),
                upbit_entry_price: Decimal::new(100_000, 0),
                bybit_entry_price: Decimal::new(100_050, 0),
                bybit_liquidation_price: Decimal::new(199_445, 0),
                entry_usd_krw: 1380.0,
                entry_spread_pct: 0.05,
                entry_z_score: 2.5,
                size_usdt: Decimal::new(1000, 0),
            },
        );

        // 가격이 liquidation 아래
        assert!(!pm.check_liquidation("BTC", Decimal::new(150_000, 0)));
        // 가격이 liquidation 이상
        assert!(pm.check_liquidation("BTC", Decimal::new(199_445, 0)));
        assert!(pm.check_liquidation("BTC", Decimal::new(200_000, 0)));
        // 존재하지 않는 코인
        assert!(!pm.check_liquidation("ETH", Decimal::new(200_000, 0)));
    }

    #[test]
    fn test_calculate_liquidation_price() {
        // entry_price = 100,000, leverage = 1, MMR = 0.005, fee = 0.00055
        // liq = 100,000 * (1 + 1 - 0.005 - 0.00055) = 100,000 * 1.99445 = 199,445
        let liq = calculate_liquidation_price(
            Decimal::new(100_000, 0),
            1,
            Decimal::new(5, 3),
            Decimal::new(55, 5),
        );
        assert_eq!(liq, Decimal::new(199_445, 0));
    }

    #[test]
    fn test_calculate_liquidation_price_higher_leverage() {
        // entry_price = 100,000, leverage = 2, MMR = 0.005, fee = 0.00055
        // liq = 100,000 * (1 + 0.5 - 0.005 - 0.00055) = 100,000 * 1.49445 = 149,445
        let liq = calculate_liquidation_price(
            Decimal::new(100_000, 0),
            2,
            Decimal::new(5, 3),
            Decimal::new(55, 5),
        );
        assert_eq!(liq, Decimal::new(149_445, 0));
    }
}
