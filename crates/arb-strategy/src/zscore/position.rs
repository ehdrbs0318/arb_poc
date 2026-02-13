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

/// 포지션 상태 머신.
///
/// Opening → Open → Closing → Closed
///           |       |    \
///           |       |     └→ PendingExchangeRecovery → Closed
///           |       └→ PartiallyClosedOneLeg → Closed
///           └→ PartiallyClosedOneLeg → PendingExchangeRecovery → Closed
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PositionState {
    /// 주문 발주 중 (Opening INSERT 완료, 체결 대기).
    Opening,
    /// 양 레그 체결 완료, 활성 포지션.
    #[default]
    Open,
    /// 청산 주문 진행 중.
    Closing,
    /// 청산 완료.
    Closed,
    /// 한쪽 레그만 체결/청산됨 (비상 청산 필요).
    PartiallyClosedOneLeg,
    /// 거래소 장애로 복구 대기 중.
    PendingExchangeRecovery,
}

impl std::fmt::Display for PositionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opening => write!(f, "Opening"),
            Self::Open => write!(f, "Open"),
            Self::Closing => write!(f, "Closing"),
            Self::Closed => write!(f, "Closed"),
            Self::PartiallyClosedOneLeg => write!(f, "PartiallyClosedOneLeg"),
            Self::PendingExchangeRecovery => write!(f, "PendingExchangeRecovery"),
        }
    }
}

/// 가상 포지션.
///
/// Upbit 현물 매수 + Bybit 선물 short 한 쌍을 나타냅니다.
/// 코인당 복수 포지션이 존재할 수 있으며, 고유 ID로 식별합니다.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    /// 포지션 수량 (코인 단위, 양 leg 동일).
    pub qty: Decimal,
    /// DB primary key 참조 (라이브 전용, 시뮬에서는 None).
    #[serde(default)]
    pub db_id: Option<i64>,
    /// Upbit 주문 ID (라이브 전용).
    #[serde(default)]
    pub upbit_order_id: Option<String>,
    /// Bybit 주문 ID (라이브 전용).
    #[serde(default)]
    pub bybit_order_id: Option<String>,
    /// 주문 진행 중 플래그 (in_flight 동안 다른 시그널 무시).
    #[serde(default)]
    pub in_flight: bool,
    /// 포지션 상태.
    #[serde(default)]
    pub state: PositionState,
    /// Closing 전이 시각 (kill switch timeout 판단용).
    #[serde(default)]
    pub closing_started_at: Option<DateTime<Utc>>,
    /// Client Order ID (crash recovery용, UUID v7).
    #[serde(default)]
    pub client_order_id: Option<String>,
    /// 청산 Client Order ID (crash recovery용).
    #[serde(default)]
    pub exit_client_order_id: Option<String>,
    /// 한쪽 레그만 성공한 경우 성공한 레그 ("upbit" or "bybit").
    #[serde(default)]
    pub succeeded_leg: Option<String>,
    /// 비상 청산 시도 횟수.
    #[serde(default)]
    pub emergency_attempts: u32,
}

impl Default for VirtualPosition {
    fn default() -> Self {
        Self {
            id: 0,
            coin: String::new(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::ZERO,
            bybit_entry_price: Decimal::ZERO,
            bybit_liquidation_price: Decimal::ZERO,
            entry_usd_krw: 0.0,
            entry_spread_pct: 0.0,
            entry_z_score: 0.0,
            qty: Decimal::ZERO,
            db_id: None,
            upbit_order_id: None,
            bybit_order_id: None,
            in_flight: false,
            state: PositionState::Open,
            closing_started_at: None,
            client_order_id: None,
            exit_client_order_id: None,
            succeeded_leg: None,
            emergency_attempts: 0,
        }
    }
}

impl VirtualPosition {
    /// USDT 기준 포지션 크기 (qty * bybit_entry_price).
    pub fn size_usdt(&self) -> Decimal {
        self.qty * self.bybit_entry_price
    }
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
            .map(|p| p.size_usdt() * Decimal::from(2u64))
            .sum()
    }

    /// 특정 코인의 사용 중인 자본 합계 (단일 leg 기준).
    pub fn coin_used_capital(&self, coin: &str) -> Decimal {
        self.open_positions
            .get(coin)
            .map(|positions| positions.iter().map(|p| p.size_usdt()).sum())
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
            qty = %position.qty,
            size_usdt = %position.size_usdt(),
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
            pos.qty,
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
    /// `partial_qty`만큼 ClosedPosition을 생성하고,
    /// 잔여 qty로 VirtualPosition을 축소합니다 (ID 동일 유지).
    /// `instrument_info`가 제공되면 qty_step으로 라운딩하고,
    /// 라운딩 후 0이면 전량 청산으로 전환합니다.
    ///
    /// # 반환값
    ///
    /// `(ClosedPosition, Option<VirtualPosition>)` — 잔여가 0이면 None.
    #[allow(clippy::too_many_arguments)]
    pub fn close_partial(
        &mut self,
        coin: &str,
        position_id: u64,
        partial_qty: Decimal,
        instrument_info: Option<&crate::zscore::instrument::InstrumentInfo>,
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

        // instrument_info가 있으면 qty_step 라운딩 적용
        let close_qty = if let Some(info) = instrument_info {
            let rounded = crate::zscore::instrument::floor_to_step(partial_qty, info.qty_step);
            if rounded.is_zero() {
                // qty_step 라운딩 후 0이면 전량 청산으로 전환
                pos.qty
            } else {
                let remaining = pos.qty.saturating_sub(rounded);
                if remaining < info.min_order_qty && remaining > Decimal::ZERO {
                    // 잔량이 min_order_qty 미만이면 전량 청산
                    pos.qty
                } else {
                    rounded
                }
            }
        } else {
            // fallback: 라운딩 없이 원본 qty
            partial_qty
        };

        let remaining_qty = pos.qty.saturating_sub(close_qty);

        let closed = Self::build_closed_position(
            pos,
            close_qty,
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
            close_qty = %close_qty,
            remaining_qty = %remaining_qty,
            net_pnl = %closed.net_pnl,
            "부분 청산 완료"
        );

        self.closed_positions.push(closed.clone());

        let remaining_pos = if remaining_qty > Decimal::ZERO {
            // 잔여 포지션 축소 (ID 동일 유지)
            positions[idx].qty = remaining_qty;
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
    ///
    /// `close_qty`는 코인 수량 기반이며, 양 leg 동일 수량으로 계산합니다.
    #[allow(clippy::too_many_arguments)]
    fn build_closed_position(
        pos: &VirtualPosition,
        close_qty: Decimal,
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

        let qty = close_qty; // 양 leg 동일 수량

        // Upbit 현물 PnL
        let upbit_pnl = (exit_upbit_usdt_price - pos.upbit_entry_price) * qty;

        // Bybit short PnL
        let bybit_pnl = (pos.bybit_entry_price - exit_bybit_price) * qty;

        // 수수료: 진입/청산 각각의 가격에 수수료를 개별 적용
        let upbit_fees =
            (pos.upbit_entry_price * qty + exit_upbit_usdt_price * qty) * upbit_taker_fee;
        let bybit_fees = (pos.bybit_entry_price * qty + exit_bybit_price * qty) * bybit_taker_fee;
        let total_fees = upbit_fees + bybit_fees;

        // 순 PnL
        let net_pnl = upbit_pnl + bybit_pnl - total_fees;

        ClosedPosition {
            id: pos.id,
            coin: pos.coin.clone(),
            entry_time: pos.entry_time,
            exit_time,
            holding_minutes,
            qty: close_qty,
            size_usdt: close_qty * pos.bybit_entry_price,
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
            actual_upbit_fee: None,
            actual_bybit_fee: None,
            funding_fee: None,
            adjustment_cost: None,
        }
    }

    /// 포지션을 Opening 상태로 등록합니다 (LivePolicy 전용).
    pub fn register_opening(&mut self, pos: VirtualPosition) {
        let coin = pos.coin.clone();
        self.open_positions.entry(coin).or_default().push(pos);
    }

    /// 포지션 상태를 전이합니다.
    /// 이중 전이를 방지하기 위해 현재 상태를 검증합니다.
    pub fn transition_state(&mut self, coin: &str, pos_id: u64, to: PositionState) -> bool {
        if let Some(positions) = self.open_positions.get_mut(coin)
            && let Some(pos) = positions.iter_mut().find(|p| p.id == pos_id)
        {
            pos.state = to;
            return true;
        }
        false
    }

    /// Closing 전이를 시도합니다 (TOCTOU 방지: 이미 Closing이면 false).
    pub fn try_transition_to_closing(&mut self, coin: &str, pos_id: u64) -> bool {
        if let Some(positions) = self.open_positions.get_mut(coin)
            && let Some(pos) = positions
                .iter_mut()
                .find(|p| p.id == pos_id && p.state == PositionState::Open)
        {
            pos.state = PositionState::Closing;
            pos.closing_started_at = Some(Utc::now());
            return true;
        }
        false
    }

    /// in_flight 포지션 목록을 반환합니다 (kill switch 재스캔용).
    pub fn in_flight_positions(&self) -> Vec<(String, u64)> {
        self.open_positions
            .iter()
            .flat_map(|(coin, positions)| {
                positions
                    .iter()
                    .filter(|p| p.in_flight)
                    .map(move |p| (coin.clone(), p.id))
            })
            .collect()
    }

    /// 특정 포지션의 in_flight 플래그를 설정합니다.
    pub fn set_in_flight(&mut self, coin: &str, pos_id: u64, in_flight: bool) {
        if let Some(positions) = self.open_positions.get_mut(coin)
            && let Some(pos) = positions.iter_mut().find(|p| p.id == pos_id)
        {
            pos.in_flight = in_flight;
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

    fn make_position(coin: &str, qty: i64, entry_price: i64) -> VirtualPosition {
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
            qty: Decimal::new(qty, 0),
            ..Default::default()
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
        // qty=1, bybit_entry_price=100_050 → size_usdt()=100_050 → used_capital=200_100
        let pos = make_position("BTC", 1, 100_000);
        pm.open_position(pos).unwrap();
        assert!(pm.has_position("BTC"));
        assert_eq!(pm.open_count(), 1);
        assert_eq!(pm.used_capital(), Decimal::new(200_100, 0));
    }

    #[test]
    fn test_open_multiple_positions() {
        // 같은 코인에 복수 포지션 추가 가능
        let mut pm = PositionManager::new();
        // qty=1, bybit_entry=100050 → size_usdt=100050
        pm.open_position(make_position("BTC", 1, 100_000)).unwrap();
        // qty=2, bybit_entry=100050 → size_usdt=200100
        pm.open_position(make_position("BTC", 2, 100_000)).unwrap();
        assert_eq!(pm.open_count(), 2);
        // used_capital = (100050 + 200100) × 2 = 600300
        assert_eq!(pm.used_capital(), Decimal::new(600_300, 0));
        // ID가 다르게 할당됨
        let positions = pm.open_positions.get("BTC").unwrap();
        assert_eq!(positions[0].id, 0);
        assert_eq!(positions[1].id, 1);
    }

    #[test]
    fn test_coin_used_capital() {
        let mut pm = PositionManager::new();
        // BTC qty=1, bybit_entry=100050 → size_usdt=100050
        pm.open_position(make_position("BTC", 1, 100_000)).unwrap();
        // BTC qty=2, bybit_entry=100050 → size_usdt=200100
        pm.open_position(make_position("BTC", 2, 100_000)).unwrap();
        // ETH qty=10, bybit_entry=5050 → size_usdt=50500
        pm.open_position(make_position("ETH", 10, 5_000)).unwrap();

        // BTC: 100050 + 200100 = 300150
        assert_eq!(pm.coin_used_capital("BTC"), Decimal::new(300_150, 0));
        // ETH: 50500
        assert_eq!(pm.coin_used_capital("ETH"), Decimal::new(50_500, 0));
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
            qty: Decimal::ONE,
            ..Default::default()
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
        pm.open_position(make_position("BTC", 1, 100_000)).unwrap();
        pm.open_position(make_position("BTC", 2, 100_000)).unwrap();
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

        // 남은 포지션은 ID=1, qty=2
        let remaining = pm.open_positions.get("BTC").unwrap();
        assert_eq!(remaining[0].id, 1);
        assert_eq!(remaining[0].qty, Decimal::new(2, 0));
    }

    #[test]
    fn test_close_partial() {
        let mut pm = PositionManager::new();
        // qty=5 코인
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
            qty: Decimal::new(5, 0),
            ..Default::default()
        })
        .unwrap();

        // 2 코인 부분 청산 (instrument_info 없음)
        let (closed, remaining) = pm
            .close_partial(
                "BTC",
                0,
                Decimal::new(2, 0), // partial_qty
                None,               // instrument_info
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

        assert_eq!(closed.qty, Decimal::new(2, 0));
        // size_usdt = qty(2) * bybit_entry_price(100050) = 200100
        assert_eq!(closed.size_usdt, Decimal::new(200_100, 0));
        assert_eq!(closed.id, 0);

        // 잔여 포지션: qty=3
        let rem = remaining.unwrap();
        assert_eq!(rem.qty, Decimal::new(3, 0));
        assert_eq!(rem.id, 0); // ID 동일 유지

        // 오픈 포지션에도 반영됨
        assert!(pm.has_position("BTC"));
        let btc_positions = pm.open_positions.get("BTC").unwrap();
        assert_eq!(btc_positions[0].qty, Decimal::new(3, 0));
    }

    #[test]
    fn test_close_partial_full_exhaust() {
        // partial_qty가 전량 이상이면 포지션이 제거됨
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
            qty: Decimal::new(5, 0),
            ..Default::default()
        })
        .unwrap();

        // 전량(5 코인) 부분 청산
        let (closed, remaining) = pm
            .close_partial(
                "BTC",
                0,
                Decimal::new(5, 0), // partial_qty = 전량
                None,               // instrument_info
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

        assert_eq!(closed.qty, Decimal::new(5, 0));
        // size_usdt = 5 * 100050 = 500250
        assert_eq!(closed.size_usdt, Decimal::new(500_250, 0));
        assert!(remaining.is_none());
        assert!(!pm.has_position("BTC"));
    }

    #[test]
    fn test_last_entry_at() {
        let mut pm = PositionManager::new();

        // 초기에는 None
        assert!(pm.last_entry_at("BTC").is_none());

        // 진입 후 시각이 기록됨
        let pos = make_position("BTC", 1, 100_000);
        let entry_time = pos.entry_time;
        pm.open_position(pos).unwrap();
        assert_eq!(pm.last_entry_at("BTC"), Some(entry_time));

        // 다른 코인은 여전히 None
        assert!(pm.last_entry_at("ETH").is_none());

        // 같은 코인에 재진입 시 시각 갱신
        let pos2 = make_position("BTC", 2, 100_000);
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
            qty: Decimal::ONE,
            ..Default::default()
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
            qty: Decimal::new(2, 0),
            ..Default::default()
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
        pm.open_position(make_position("BTC", 1, 100_000)).unwrap();
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
        pm.open_position(make_position("BTC", 1, 100_000)).unwrap();
        pm.open_position(make_position("BTC", 2, 100_000)).unwrap();
        pm.open_position(make_position("ETH", 10, 5_000)).unwrap();

        // BTC 2개 + ETH 1개 = 3개
        assert_eq!(pm.open_count(), 3);
    }

    #[test]
    fn test_default() {
        let pm = PositionManager::default();
        assert_eq!(pm.open_count(), 0);
        assert_eq!(pm.used_capital(), Decimal::ZERO);
    }

    #[test]
    fn test_close_partial_with_instrument_info_rounding() {
        // instrument_info가 있으면 qty_step으로 라운딩
        use crate::zscore::instrument::InstrumentInfo;

        let mut pm = PositionManager::new();
        pm.open_position(VirtualPosition {
            id: 0,
            coin: "ETH".to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(3000, 0),
            bybit_entry_price: Decimal::new(3010, 0),
            bybit_liquidation_price: Decimal::new(6000, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            qty: Decimal::new(10, 0), // 10 ETH
            ..Default::default()
        })
        .unwrap();

        let info = InstrumentInfo {
            tick_size: Decimal::new(1, 2),
            qty_step: Decimal::new(1, 1),      // 0.1
            min_order_qty: Decimal::new(1, 1), // 0.1
            min_notional: Decimal::new(5, 0),
            max_order_qty: Decimal::new(1000, 0),
        };

        // partial_qty = 3.15 → floor(3.15, 0.1) = 3.1
        let (closed, remaining) = pm
            .close_partial(
                "ETH",
                0,
                Decimal::new(315, 2), // 3.15
                Some(&info),
                Decimal::new(3020, 0),
                Decimal::new(3020, 0),
                1381.0,
                0.0,
                0.3,
                Decimal::new(5, 4),
                Decimal::new(55, 5),
                false,
            )
            .unwrap();

        // 라운딩 후 3.1 코인 청산
        assert_eq!(closed.qty, Decimal::new(31, 1));
        // 잔여: 10 - 3.1 = 6.9
        let rem = remaining.unwrap();
        assert_eq!(rem.qty, Decimal::new(69, 1));
    }

    #[test]
    fn test_close_partial_instrument_info_zero_rounding_becomes_full() {
        // partial_qty가 qty_step보다 작으면 라운딩 후 0 → 전량 청산 전환
        use crate::zscore::instrument::InstrumentInfo;

        let mut pm = PositionManager::new();
        pm.open_position(VirtualPosition {
            id: 0,
            coin: "BTC".to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(50000, 0),
            bybit_entry_price: Decimal::new(50050, 0),
            bybit_liquidation_price: Decimal::new(100000, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            qty: Decimal::new(1, 3), // 0.001 BTC
            ..Default::default()
        })
        .unwrap();

        let info = InstrumentInfo {
            tick_size: Decimal::new(1, 2),
            qty_step: Decimal::new(1, 3),      // 0.001
            min_order_qty: Decimal::new(1, 3), // 0.001
            min_notional: Decimal::new(5, 0),
            max_order_qty: Decimal::new(100, 0),
        };

        // partial_qty = 0.0005 → floor(0.0005, 0.001) = 0 → 전량 청산 전환
        let (closed, remaining) = pm
            .close_partial(
                "BTC",
                0,
                Decimal::new(5, 4), // 0.0005
                Some(&info),
                Decimal::new(50020, 0),
                Decimal::new(50020, 0),
                1381.0,
                0.0,
                0.3,
                Decimal::new(5, 4),
                Decimal::new(55, 5),
                false,
            )
            .unwrap();

        // 전량 청산으로 전환 (pos.qty = 0.001)
        assert_eq!(closed.qty, Decimal::new(1, 3));
        assert!(remaining.is_none());
        assert!(!pm.has_position("BTC"));
    }

    #[test]
    fn test_close_partial_remaining_below_min_order_qty() {
        // 잔량이 min_order_qty 미만이면 전량 청산으로 전환
        use crate::zscore::instrument::InstrumentInfo;

        let mut pm = PositionManager::new();
        pm.open_position(VirtualPosition {
            id: 0,
            coin: "ETH".to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(3000, 0),
            bybit_entry_price: Decimal::new(3010, 0),
            bybit_liquidation_price: Decimal::new(6000, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            qty: Decimal::new(10, 1), // 1.0 ETH
            ..Default::default()
        })
        .unwrap();

        let info = InstrumentInfo {
            tick_size: Decimal::new(1, 2),
            qty_step: Decimal::new(1, 1),      // 0.1
            min_order_qty: Decimal::new(2, 1), // 0.2
            min_notional: Decimal::new(5, 0),
            max_order_qty: Decimal::new(1000, 0),
        };

        // partial_qty = 0.9 → floor(0.9, 0.1) = 0.9
        // remaining = 1.0 - 0.9 = 0.1 < min_order_qty(0.2) → 전량 청산
        let (closed, remaining) = pm
            .close_partial(
                "ETH",
                0,
                Decimal::new(9, 1), // 0.9
                Some(&info),
                Decimal::new(3020, 0),
                Decimal::new(3020, 0),
                1381.0,
                0.0,
                0.3,
                Decimal::new(5, 4),
                Decimal::new(55, 5),
                false,
            )
            .unwrap();

        // 전량 청산으로 전환 (pos.qty = 1.0)
        assert_eq!(closed.qty, Decimal::new(10, 1));
        assert!(remaining.is_none());
        assert!(!pm.has_position("ETH"));
    }

    #[test]
    fn test_size_usdt_derived_method() {
        // size_usdt()가 qty * bybit_entry_price를 반환하는지 확인
        let pos = VirtualPosition {
            id: 0,
            coin: "BTC".to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(100_000, 0),
            bybit_entry_price: Decimal::new(100_050, 0),
            bybit_liquidation_price: Decimal::new(199_445, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            qty: Decimal::new(3, 0), // 3 BTC
            ..Default::default()
        };
        // 3 * 100050 = 300150
        assert_eq!(pos.size_usdt(), Decimal::new(300_150, 0));
    }

    #[test]
    fn test_build_closed_position_qty_based_pnl() {
        // qty 기반 PnL 계산이 올바른지 검증
        let mut pm = PositionManager::new();
        let entry_time = Utc::now();
        pm.open_position(VirtualPosition {
            id: 0,
            coin: "ETH".to_string(),
            entry_time,
            upbit_entry_price: Decimal::new(3000, 0),
            bybit_entry_price: Decimal::new(3000, 0),
            bybit_liquidation_price: Decimal::new(6000, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            qty: Decimal::new(10, 0), // 10 ETH
            ..Default::default()
        })
        .unwrap();

        let exit_time = entry_time + chrono::Duration::minutes(60);
        let closed = pm
            .close_position(
                "ETH",
                0,
                exit_time,
                Decimal::new(3100, 0), // Upbit 청산가 (USDT 환산)
                Decimal::new(2900, 0), // Bybit 청산가
                1381.0,
                0.0,
                0.3,
                Decimal::ZERO, // 수수료 0으로 설정하여 PnL만 검증
                Decimal::ZERO,
                false,
            )
            .unwrap();

        // qty=10
        assert_eq!(closed.qty, Decimal::new(10, 0));
        // upbit_pnl = (3100 - 3000) * 10 = 1000
        assert_eq!(closed.upbit_pnl, Decimal::new(1000, 0));
        // bybit_pnl = (3000 - 2900) * 10 = 1000
        assert_eq!(closed.bybit_pnl, Decimal::new(1000, 0));
        // net_pnl = 1000 + 1000 - 0 = 2000
        assert_eq!(closed.net_pnl, Decimal::new(2000, 0));
        // size_usdt = 10 * 3000 = 30000
        assert_eq!(closed.size_usdt, Decimal::new(30000, 0));
    }

    // --- PositionState 테스트 ---

    #[test]
    fn test_position_state_default() {
        assert_eq!(PositionState::default(), PositionState::Open);
    }

    #[test]
    fn test_position_state_display() {
        assert_eq!(PositionState::Opening.to_string(), "Opening");
        assert_eq!(PositionState::Open.to_string(), "Open");
        assert_eq!(PositionState::Closing.to_string(), "Closing");
        assert_eq!(PositionState::Closed.to_string(), "Closed");
        assert_eq!(
            PositionState::PartiallyClosedOneLeg.to_string(),
            "PartiallyClosedOneLeg"
        );
        assert_eq!(
            PositionState::PendingExchangeRecovery.to_string(),
            "PendingExchangeRecovery"
        );
    }

    #[test]
    fn test_position_state_equality() {
        assert_eq!(PositionState::Open, PositionState::Open);
        assert_ne!(PositionState::Open, PositionState::Closing);
    }

    #[test]
    fn test_position_state_serde_roundtrip() {
        let state = PositionState::PendingExchangeRecovery;
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: PositionState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deserialized);
    }

    // --- VirtualPosition 새 필드 테스트 ---

    #[test]
    fn test_virtual_position_default() {
        let pos = VirtualPosition::default();
        assert_eq!(pos.id, 0);
        assert!(pos.coin.is_empty());
        assert_eq!(pos.qty, Decimal::ZERO);
        assert!(pos.db_id.is_none());
        assert!(pos.upbit_order_id.is_none());
        assert!(pos.bybit_order_id.is_none());
        assert!(!pos.in_flight);
        assert_eq!(pos.state, PositionState::Open);
        assert!(pos.closing_started_at.is_none());
        assert!(pos.client_order_id.is_none());
        assert!(pos.exit_client_order_id.is_none());
        assert!(pos.succeeded_leg.is_none());
        assert_eq!(pos.emergency_attempts, 0);
    }

    #[test]
    fn test_virtual_position_serde_roundtrip() {
        let pos = VirtualPosition {
            id: 42,
            coin: "BTC".to_string(),
            entry_time: Utc::now(),
            upbit_entry_price: Decimal::new(100_000, 0),
            bybit_entry_price: Decimal::new(100_050, 0),
            bybit_liquidation_price: Decimal::new(199_445, 0),
            entry_usd_krw: 1380.0,
            entry_spread_pct: 0.05,
            entry_z_score: 2.5,
            qty: Decimal::ONE,
            db_id: Some(123),
            upbit_order_id: Some("upbit-uuid-1".to_string()),
            bybit_order_id: Some("bybit-uuid-1".to_string()),
            in_flight: true,
            state: PositionState::Opening,
            closing_started_at: None,
            client_order_id: Some("client-uuid-1".to_string()),
            exit_client_order_id: None,
            succeeded_leg: None,
            emergency_attempts: 0,
        };

        let json = serde_json::to_string(&pos).unwrap();
        let deserialized: VirtualPosition = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, 42);
        assert_eq!(deserialized.coin, "BTC");
        assert_eq!(deserialized.db_id, Some(123));
        assert_eq!(deserialized.upbit_order_id.as_deref(), Some("upbit-uuid-1"));
        assert!(deserialized.in_flight);
        assert_eq!(deserialized.state, PositionState::Opening);
        assert_eq!(
            deserialized.client_order_id.as_deref(),
            Some("client-uuid-1")
        );
    }

    #[test]
    fn test_virtual_position_serde_default_fields() {
        // 새 필드가 없는 JSON에서 역직렬화 시 default 적용
        let json = r#"{
            "id": 1,
            "coin": "ETH",
            "entry_time": "2026-01-01T00:00:00Z",
            "upbit_entry_price": "3000",
            "bybit_entry_price": "3010",
            "bybit_liquidation_price": "6000",
            "entry_usd_krw": 1380.0,
            "entry_spread_pct": 0.05,
            "entry_z_score": 2.5,
            "qty": "10"
        }"#;

        let pos: VirtualPosition = serde_json::from_str(json).unwrap();
        assert_eq!(pos.id, 1);
        assert!(pos.db_id.is_none());
        assert!(!pos.in_flight);
        assert_eq!(pos.state, PositionState::Open);
        assert_eq!(pos.emergency_attempts, 0);
    }

    // --- register_opening / transition_state 테스트 ---

    #[test]
    fn test_register_opening() {
        let mut pm = PositionManager::new();
        let pos = VirtualPosition {
            id: 99,
            coin: "BTC".to_string(),
            state: PositionState::Opening,
            qty: Decimal::ONE,
            bybit_entry_price: Decimal::new(50000, 0),
            ..Default::default()
        };

        pm.register_opening(pos);
        assert!(pm.has_position("BTC"));
        assert_eq!(pm.open_count(), 1);

        let positions = pm.open_positions.get("BTC").unwrap();
        assert_eq!(positions[0].id, 99); // ID가 유지됨 (open_position과 다르게)
        assert_eq!(positions[0].state, PositionState::Opening);
    }

    #[test]
    fn test_transition_state_success() {
        let mut pm = PositionManager::new();
        let pos = VirtualPosition {
            id: 1,
            coin: "BTC".to_string(),
            state: PositionState::Opening,
            ..Default::default()
        };
        pm.register_opening(pos);

        let result = pm.transition_state("BTC", 1, PositionState::Open);
        assert!(result);

        let positions = pm.open_positions.get("BTC").unwrap();
        assert_eq!(positions[0].state, PositionState::Open);
    }

    #[test]
    fn test_transition_state_not_found() {
        let mut pm = PositionManager::new();
        let result = pm.transition_state("BTC", 999, PositionState::Open);
        assert!(!result);
    }

    #[test]
    fn test_try_transition_to_closing_success() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1, 50_000)).unwrap();

        // 포지션 ID=0, state=Open (default)
        let result = pm.try_transition_to_closing("BTC", 0);
        assert!(result);

        let positions = pm.open_positions.get("BTC").unwrap();
        assert_eq!(positions[0].state, PositionState::Closing);
        assert!(positions[0].closing_started_at.is_some());
    }

    #[test]
    fn test_try_transition_to_closing_already_closing() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1, 50_000)).unwrap();

        // 첫 번째 전이 성공
        assert!(pm.try_transition_to_closing("BTC", 0));
        // 두 번째 전이 실패 (이미 Closing)
        assert!(!pm.try_transition_to_closing("BTC", 0));
    }

    #[test]
    fn test_try_transition_to_closing_wrong_coin() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1, 50_000)).unwrap();

        let result = pm.try_transition_to_closing("ETH", 0);
        assert!(!result);
    }

    // --- in_flight 테스트 ---

    #[test]
    fn test_set_in_flight() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1, 50_000)).unwrap();

        // 초기값: false
        let positions = pm.open_positions.get("BTC").unwrap();
        assert!(!positions[0].in_flight);

        // true로 설정
        pm.set_in_flight("BTC", 0, true);
        let positions = pm.open_positions.get("BTC").unwrap();
        assert!(positions[0].in_flight);

        // false로 되돌림
        pm.set_in_flight("BTC", 0, false);
        let positions = pm.open_positions.get("BTC").unwrap();
        assert!(!positions[0].in_flight);
    }

    #[test]
    fn test_in_flight_positions_empty() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1, 50_000)).unwrap();

        let in_flight = pm.in_flight_positions();
        assert!(in_flight.is_empty());
    }

    #[test]
    fn test_in_flight_positions_some() {
        let mut pm = PositionManager::new();
        pm.open_position(make_position("BTC", 1, 50_000)).unwrap();
        pm.open_position(make_position("ETH", 1, 3_000)).unwrap();

        pm.set_in_flight("BTC", 0, true);

        let in_flight = pm.in_flight_positions();
        assert_eq!(in_flight.len(), 1);
        assert_eq!(in_flight[0].0, "BTC");
        assert_eq!(in_flight[0].1, 0);
    }

    #[test]
    fn test_set_in_flight_nonexistent() {
        let mut pm = PositionManager::new();
        // 존재하지 않는 포지션에 대해 set_in_flight → 무시 (패닉 없음)
        pm.set_in_flight("BTC", 999, true);
        assert!(pm.in_flight_positions().is_empty());
    }
}
