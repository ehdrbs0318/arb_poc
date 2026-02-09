//! 가상 포지션 관리.
//!
//! 시뮬레이션용 가상 포지션 생성, 관리, 청산을 담당합니다.
//! 코인당 복수 독립 포지션을 지원하며, 부분 청산 기능을 제공합니다.
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
/// 코인당 복수 포지션이 존재할 수 있으며, 고유 ID로 식별합니다.
#[derive(Debug, Clone)]
pub struct VirtualPosition {
    /// 포지션 고유 ID (시퀀스 번호).
    pub id: u64,
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
/// 코인당 복수 독립 포지션을 Vec으로 관리합니다.
#[derive(Debug, Clone)]
pub struct PositionManager {
    /// 활성 포지션 (코인당 복수 가능).
    pub open_positions: HashMap<String, Vec<VirtualPosition>>,
    /// 청산된 포지션 이력.
    pub closed_positions: Vec<ClosedPosition>,
    /// 다음 포지션 ID.
    next_id: u64,
    /// 코인별 마지막 진입 시각 (재진입 cooldown용).
    last_entry_time: HashMap<String, DateTime<Utc>>,
}

impl PositionManager {
    /// 새 PositionManager를 생성합니다.
    pub fn new() -> Self {
        Self {
            open_positions: HashMap::new(),
            closed_positions: Vec::new(),
            next_id: 0,
            last_entry_time: HashMap::new(),
        }
    }

    /// 현재 사용 중인 자본 합계 (양 leg 합산).
    pub fn used_capital(&self) -> Decimal {
        self.open_positions
            .values()
            .flat_map(|positions| positions.iter())
            .map(|p| p.size_usdt * Decimal::from(2u64))
            .sum()
    }

    /// 특정 코인의 사용 중인 자본 합계 (단일 leg 기준).
    pub fn coin_used_capital(&self, coin: &str) -> Decimal {
        self.open_positions
            .get(coin)
            .map(|positions| positions.iter().map(|p| p.size_usdt).sum())
            .unwrap_or(Decimal::ZERO)
    }

    /// 해당 코인에 대한 포지션이 있는지 확인합니다.
    pub fn has_position(&self, coin: &str) -> bool {
        self.open_positions
            .get(coin)
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    /// 현재 열린 포지션 수를 반환합니다.
    pub fn open_count(&self) -> usize {
        self.open_positions.values().map(|v| v.len()).sum()
    }

    /// 새 포지션을 추가합니다.
    ///
    /// 포지션에 고유 ID를 할당하고 Vec에 push합니다.
    /// 자본 한도 검증은 호출측(monitor.rs)에서 수행합니다.
    pub fn open_position(&mut self, mut position: VirtualPosition) -> Result<(), PositionError> {
        position.id = self.next_id;
        self.next_id += 1;
        let coin = position.coin.clone();
        self.last_entry_time
            .insert(coin.clone(), position.entry_time);
        info!(
            coin = %position.coin,
            id = position.id,
            size_usdt = %position.size_usdt,
            upbit_entry_price = %position.upbit_entry_price,
            bybit_entry_price = %position.bybit_entry_price,
            liquidation_price = %position.bybit_liquidation_price,
            "포지션 진입 완료"
        );
        self.open_positions.entry(coin).or_default().push(position);
        Ok(())
    }

    /// 포지션을 전량 청산하고 ClosedPosition을 반환합니다.
    ///
    /// Vec에서 `id == position_id`인 포지션을 찾아 제거합니다.
    #[allow(clippy::too_many_arguments)]
    pub fn close_position(
        &mut self,
        coin: &str,
        position_id: u64,
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
        let positions = self.open_positions.get_mut(coin).ok_or_else(|| {
            warn!(coin = %coin, position_id = position_id, "청산할 포지션이 존재하지 않음");
            PositionError::NotFound {
                coin: coin.to_string(),
            }
        })?;

        let idx = positions
            .iter()
            .position(|p| p.id == position_id)
            .ok_or_else(|| {
                warn!(coin = %coin, position_id = position_id, "해당 ID의 포지션을 찾을 수 없음");
                PositionError::NotFound {
                    coin: coin.to_string(),
                }
            })?;

        let pos = positions.remove(idx);

        // 빈 Vec 정리
        if positions.is_empty() {
            self.open_positions.remove(coin);
        }

        let closed = Self::build_closed_position(
            &pos,
            pos.size_usdt,
            exit_time,
            exit_upbit_usdt_price,
            exit_bybit_price,
            exit_usd_krw,
            exit_spread_pct,
            exit_z_score,
            upbit_taker_fee,
            bybit_taker_fee,
            is_liquidated,
        );

        info!(
            coin = %closed.coin,
            id = closed.id,
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

    /// 포지션을 부분 청산합니다.
    ///
    /// `partial_size_usdt`만큼 ClosedPosition을 생성하고,
    /// 잔여 size_usdt로 VirtualPosition을 축소합니다 (ID 동일 유지).
    ///
    /// # 반환값
    ///
    /// `(ClosedPosition, Option<VirtualPosition>)` — 잔여가 0이면 None.
    #[allow(clippy::too_many_arguments)]
    pub fn close_partial(
        &mut self,
        coin: &str,
        position_id: u64,
        partial_size_usdt: Decimal,
        exit_upbit_price: Decimal,
        exit_bybit_price: Decimal,
        exit_usd_krw: f64,
        exit_spread_pct: f64,
        exit_z_score: f64,
        upbit_taker_fee: Decimal,
        bybit_taker_fee: Decimal,
        is_liquidated: bool,
    ) -> Result<(ClosedPosition, Option<VirtualPosition>), PositionError> {
        let positions = self.open_positions.get_mut(coin).ok_or_else(|| {
            warn!(coin = %coin, position_id = position_id, "부분 청산할 포지션이 존재하지 않음");
            PositionError::NotFound {
                coin: coin.to_string(),
            }
        })?;

        let idx = positions
            .iter()
            .position(|p| p.id == position_id)
            .ok_or_else(|| {
                warn!(coin = %coin, position_id = position_id, "해당 ID의 포지션을 찾을 수 없음");
                PositionError::NotFound {
                    coin: coin.to_string(),
                }
            })?;

        let exit_time = Utc::now();
        let pos = &positions[idx];
        let remaining = pos.size_usdt - partial_size_usdt;

        let closed = Self::build_closed_position(
            pos,
            partial_size_usdt,
            exit_time,
            exit_upbit_price,
            exit_bybit_price,
            exit_usd_krw,
            exit_spread_pct,
            exit_z_score,
            upbit_taker_fee,
            bybit_taker_fee,
            is_liquidated,
        );

        info!(
            coin = %closed.coin,
            id = closed.id,
            partial_size_usdt = %partial_size_usdt,
            remaining_usdt = %remaining,
            net_pnl = %closed.net_pnl,
            "부분 청산 완료"
        );

        self.closed_positions.push(closed.clone());

        let remaining_pos = if remaining > Decimal::ZERO {
            // 잔여 포지션 축소 (ID 동일 유지)
            positions[idx].size_usdt = remaining;
            Some(positions[idx].clone())
        } else {
            // 전량 소진 → 포지션 제거
            positions.remove(idx);
            if positions.is_empty() {
                self.open_positions.remove(coin);
            }
            None
        };

        Ok((closed, remaining_pos))
    }

    /// 코인별 마지막 진입 시각을 반환합니다.
    pub fn last_entry_at(&self, coin: &str) -> Option<DateTime<Utc>> {
        self.last_entry_time.get(coin).copied()
    }

    /// Bybit liquidation 체크: liquidation 조건에 해당하는 포지션 ID들을 반환합니다.
    pub fn check_liquidation(&self, coin: &str, current_bybit_price: Decimal) -> Vec<u64> {
        let Some(positions) = self.open_positions.get(coin) else {
            return Vec::new();
        };

        let mut liquidated_ids = Vec::new();
        for p in positions {
            if current_bybit_price >= p.bybit_liquidation_price {
                warn!(
                    coin = %coin,
                    position_id = p.id,
                    current_bybit_price = %current_bybit_price,
                    liquidation_price = %p.bybit_liquidation_price,
                    "Bybit liquidation 조건 도달"
                );
                liquidated_ids.push(p.id);
            }
        }
        liquidated_ids
    }

    /// ClosedPosition을 생성하는 내부 헬퍼.
    #[allow(clippy::too_many_arguments)]
    fn build_closed_position(
        pos: &VirtualPosition,
        close_size_usdt: Decimal,
        exit_time: DateTime<Utc>,
        exit_upbit_usdt_price: Decimal,
        exit_bybit_price: Decimal,
        exit_usd_krw: f64,
        exit_spread_pct: f64,
        exit_z_score: f64,
        upbit_taker_fee: Decimal,
        bybit_taker_fee: Decimal,
        is_liquidated: bool,
    ) -> ClosedPosition {
        let holding_minutes = (exit_time - pos.entry_time).num_minutes().unsigned_abs();

        // USDT notional matching (close_size 기준)
        let upbit_qty = close_size_usdt / pos.upbit_entry_price;
        let bybit_qty = close_size_usdt / pos.bybit_entry_price;

        // Upbit 현물 PnL
        let upbit_pnl = (exit_upbit_usdt_price - pos.upbit_entry_price) * upbit_qty;

        // Bybit short PnL
        let bybit_pnl = (pos.bybit_entry_price - exit_bybit_price) * bybit_qty;

        // 수수료 (진입 + 청산)
        let upbit_fees = close_size_usdt * upbit_taker_fee * Decimal::from(2u64);
        let bybit_fees = close_size_usdt * bybit_taker_fee * Decimal::from(2u64);
        let total_fees = upbit_fees + bybit_fees;

        // 순 PnL
        let net_pnl = upbit_pnl + bybit_pnl - total_fees;

        ClosedPosition {
            id: pos.id,
            coin: pos.coin.clone(),
            entry_time: pos.entry_time,
            exit_time,
            holding_minutes,
            size_usdt: close_size_usdt,
            upbit_entry_price: pos.upbit_entry_price,
            bybit_entry_price: pos.bybit_entry_price,
            upbit_exit_price: exit_upbit_usdt_price,
            bybit_exit_price: exit_bybit_price,
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
        }
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
            id: 0, // open_position()에서 자동 할당
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
    fn test_open_multiple_positions() {
        // 같은 코인에 복수 포지션 추가 가능
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1000, 100_000))
            .unwrap();
        pm.open_position(make_position("BTC", 500, 100_000))
            .unwrap();
        assert_eq!(pm.open_count(), 2);
        // used_capital = (1000 + 500) × 2 = 3000
        assert_eq!(pm.used_capital(), Decimal::new(3000, 0));
        // ID가 다르게 할당됨
        let positions = pm.open_positions.get("BTC").unwrap();
        assert_eq!(positions[0].id, 0);
        assert_eq!(positions[1].id, 1);
    }

    #[test]
    fn test_coin_used_capital() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1000, 100_000))
            .unwrap();
        pm.open_position(make_position("BTC", 500, 100_000))
            .unwrap();
        pm.open_position(make_position("ETH", 300, 5_000)).unwrap();

        // BTC: 1000 + 500 = 1500
        assert_eq!(pm.coin_used_capital("BTC"), Decimal::new(1500, 0));
        // ETH: 300
        assert_eq!(pm.coin_used_capital("ETH"), Decimal::new(300, 0));
        // 존재하지 않는 코인
        assert_eq!(pm.coin_used_capital("SOL"), Decimal::ZERO);
    }

    #[test]
    fn test_close_position() {
        let mut pm = PositionManager::new();
        let entry_time = Utc::now();
        pm.open_position(VirtualPosition {
            id: 0,
            coin: "BTC".to_string(),
            entry_time,
            upbit_entry_price: Decimal::new(100_000, 0),
            bybit_entry_price: Decimal::new(100_050, 0),
            bybit_liquidation_price: Decimal::new(199_445, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            size_usdt: Decimal::new(1000, 0),
        })
        .unwrap();
        let position_id = 0; // open_position에서 할당된 ID

        let exit_time = entry_time + chrono::Duration::minutes(30);
        let closed = pm
            .close_position(
                "BTC",
                position_id,
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
        assert_eq!(closed.id, 0);
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
            0,
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
    fn test_close_specific_position_in_vec() {
        // 같은 코인에 여러 포지션이 있을 때 특정 ID만 청산
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1000, 100_000))
            .unwrap();
        pm.open_position(make_position("BTC", 500, 100_000))
            .unwrap();
        assert_eq!(pm.open_count(), 2);

        // ID=0인 포지션 청산
        let closed = pm
            .close_position(
                "BTC",
                0,
                Utc::now(),
                Decimal::new(100_020, 0),
                Decimal::new(100_020, 0),
                1381.0,
                0.0,
                0.3,
                Decimal::new(5, 4),
                Decimal::new(55, 5),
                false,
            )
            .unwrap();
        assert_eq!(closed.id, 0);
        assert_eq!(pm.open_count(), 1);

        // 남은 포지션은 ID=1
        let remaining = pm.open_positions.get("BTC").unwrap();
        assert_eq!(remaining[0].id, 1);
        assert_eq!(remaining[0].size_usdt, Decimal::new(500, 0));
    }

    #[test]
    fn test_close_partial() {
        let mut pm = PositionManager::new();
        pm.open_position(VirtualPosition {
            id: 0,
            coin: "BTC".to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(100_000, 0),
            bybit_entry_price: Decimal::new(100_050, 0),
            bybit_liquidation_price: Decimal::new(199_445, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            size_usdt: Decimal::new(500, 0),
        })
        .unwrap();

        // 200 USDT 부분 청산
        let (closed, remaining) = pm
            .close_partial(
                "BTC",
                0,
                Decimal::new(200, 0), // partial_size_usdt
                Decimal::new(100_020, 0),
                Decimal::new(100_020, 0),
                1381.0,
                0.0,
                0.3,
                Decimal::new(5, 4),
                Decimal::new(55, 5),
                false,
            )
            .unwrap();

        assert_eq!(closed.size_usdt, Decimal::new(200, 0));
        assert_eq!(closed.id, 0);

        // 잔여 포지션이 있어야 함
        let rem = remaining.unwrap();
        assert_eq!(rem.size_usdt, Decimal::new(300, 0));
        assert_eq!(rem.id, 0); // ID 동일 유지

        // 오픈 포지션에도 반영됨
        assert!(pm.has_position("BTC"));
        let btc_positions = pm.open_positions.get("BTC").unwrap();
        assert_eq!(btc_positions[0].size_usdt, Decimal::new(300, 0));
    }

    #[test]
    fn test_close_partial_full_exhaust() {
        // partial_size가 전량 이상이면 포지션이 제거됨
        let mut pm = PositionManager::new();
        pm.open_position(VirtualPosition {
            id: 0,
            coin: "BTC".to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(100_000, 0),
            bybit_entry_price: Decimal::new(100_050, 0),
            bybit_liquidation_price: Decimal::new(199_445, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            size_usdt: Decimal::new(500, 0),
        })
        .unwrap();

        // 전량(500 USDT) 부분 청산
        let (closed, remaining) = pm
            .close_partial(
                "BTC",
                0,
                Decimal::new(500, 0),
                Decimal::new(100_020, 0),
                Decimal::new(100_020, 0),
                1381.0,
                0.0,
                0.3,
                Decimal::new(5, 4),
                Decimal::new(55, 5),
                false,
            )
            .unwrap();

        assert_eq!(closed.size_usdt, Decimal::new(500, 0));
        assert!(remaining.is_none());
        assert!(!pm.has_position("BTC"));
    }

    #[test]
    fn test_last_entry_at() {
        let mut pm = PositionManager::new();

        // 초기에는 None
        assert!(pm.last_entry_at("BTC").is_none());

        // 진입 후 시각이 기록됨
        let pos = make_position("BTC", 1000, 100_000);
        let entry_time = pos.entry_time;
        pm.open_position(pos).unwrap();
        assert_eq!(pm.last_entry_at("BTC"), Some(entry_time));

        // 다른 코인은 여전히 None
        assert!(pm.last_entry_at("ETH").is_none());

        // 같은 코인에 재진입 시 시각 갱신
        let pos2 = make_position("BTC", 500, 100_000);
        let entry_time2 = pos2.entry_time;
        pm.open_position(pos2).unwrap();
        assert_eq!(pm.last_entry_at("BTC"), Some(entry_time2));
    }

    #[test]
    fn test_check_liquidation_multiple() {
        let mut pm = PositionManager::new();

        // 다양한 liquidation price를 가진 포지션들
        pm.open_position(VirtualPosition {
            id: 0,
            coin: "BTC".to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(100_000, 0),
            bybit_entry_price: Decimal::new(100_050, 0),
            bybit_liquidation_price: Decimal::new(150_000, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            size_usdt: Decimal::new(1000, 0),
        })
        .unwrap();

        pm.open_position(VirtualPosition {
            id: 0,
            coin: "BTC".to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(100_000, 0),
            bybit_entry_price: Decimal::new(100_050, 0),
            bybit_liquidation_price: Decimal::new(200_000, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            size_usdt: Decimal::new(500, 0),
        })
        .unwrap();

        // 160,000: pos1(liq=150,000)만 liquidation
        let ids = pm.check_liquidation("BTC", Decimal::new(160_000, 0));
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0], 0);

        // 200,000: 양쪽 다 liquidation
        let ids = pm.check_liquidation("BTC", Decimal::new(200_000, 0));
        assert_eq!(ids.len(), 2);

        // 존재하지 않는 코인
        let ids = pm.check_liquidation("ETH", Decimal::new(200_000, 0));
        assert!(ids.is_empty());
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

    #[test]
    fn test_has_position_after_all_closed() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1000, 100_000))
            .unwrap();
        assert!(pm.has_position("BTC"));

        pm.close_position(
            "BTC",
            0,
            Utc::now(),
            Decimal::new(100_020, 0),
            Decimal::new(100_020, 0),
            1381.0,
            0.0,
            0.3,
            Decimal::new(5, 4),
            Decimal::new(55, 5),
            false,
        )
        .unwrap();

        assert!(!pm.has_position("BTC"));
        assert_eq!(pm.open_count(), 0);
    }

    #[test]
    fn test_open_count_multiple_coins() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1000, 100_000))
            .unwrap();
        pm.open_position(make_position("BTC", 500, 100_000))
            .unwrap();
        pm.open_position(make_position("ETH", 300, 5_000)).unwrap();

        // BTC 2개 + ETH 1개 = 3개
        assert_eq!(pm.open_count(), 3);
    }

    #[test]
    fn test_default() {
        let pm = PositionManager::default();
        assert_eq!(pm.open_count(), 0);
        assert_eq!(pm.used_capital(), Decimal::ZERO);
    }
}
