//! arb-db와 arb-strategy 간의 PositionStore 어댑터.
//!
//! arb-strategy는 arb-db에 직접 의존하지 않으므로(DI 패턴),
//! 자체 `PositionStore` trait을 정의합니다.
//! arb-db의 `DbPositionStore`는 `arb_db::positions::PositionStore` trait을
//! 구현하며, 에러 타입이 `DbError`입니다.
//!
//! 이 모듈은 arb-db의 `DbPositionStore`를 arb-strategy의 `PositionStore` trait에
//! 맞추는 어댑터를 제공합니다.

use arb_db::positions::{
    DbPositionStore, PositionStore as DbPositionStoreTrait, TransitionResult as DbTransitionResult,
};
use arb_strategy::zscore::position_store::{
    PositionRecord as StrategyPositionRecord, PositionStore as StrategyPositionStoreTrait,
    TransitionResult as StrategyTransitionResult, UpdateFields as StrategyUpdateFields,
};

/// arb-db → arb-strategy PositionStore 어댑터.
///
/// `DbPositionStore`를 감싸서 arb-strategy의 `PositionStore` trait을 구현합니다.
/// 에러 변환: `DbError` → `String` (arb-strategy trait 규격).
/// 필드 변환: 동일 스키마이므로 1:1 매핑.
pub struct DbPositionStoreAdapter {
    inner: DbPositionStore,
}

impl DbPositionStoreAdapter {
    /// 새 어댑터 생성.
    pub fn new(inner: DbPositionStore) -> Self {
        Self { inner }
    }
}

impl StrategyPositionStoreTrait for DbPositionStoreAdapter {
    async fn save(&self, pos: &StrategyPositionRecord) -> Result<i64, String> {
        let db_record = to_db_record(pos);
        self.inner.save(&db_record).await.map_err(|e| e.to_string())
    }

    async fn update_state(
        &self,
        id: i64,
        from: &str,
        to: &str,
        fields: StrategyUpdateFields,
    ) -> Result<StrategyTransitionResult, String> {
        let db_fields = to_db_update_fields(fields);
        let result = self
            .inner
            .update_state(id, from, to, db_fields)
            .await
            .map_err(|e| e.to_string())?;

        Ok(match result {
            DbTransitionResult::Applied => StrategyTransitionResult::Applied,
            DbTransitionResult::AlreadyTransitioned(_) => {
                StrategyTransitionResult::AlreadyTransitioned
            }
        })
    }

    async fn load_open(&self, session_id: i64) -> Result<Vec<StrategyPositionRecord>, String> {
        let db_records = self
            .inner
            .load_open(session_id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(db_records.into_iter().map(to_strategy_record).collect())
    }

    async fn remove(&self, id: i64) -> Result<(), String> {
        self.inner.remove(id).await.map_err(|e| e.to_string())
    }
}

// ---------------------------------------------------------------------------
// 변환 함수
// ---------------------------------------------------------------------------

/// arb-strategy PositionRecord → arb-db PositionRecord.
fn to_db_record(s: &StrategyPositionRecord) -> arb_db::positions::PositionRecord {
    arb_db::positions::PositionRecord {
        id: s.id,
        session_id: s.session_id,
        coin: s.coin.clone(),
        state: s.state.clone(),
        upbit_qty: s.upbit_qty,
        bybit_qty: s.bybit_qty,
        upbit_entry_price: s.upbit_entry_price,
        bybit_entry_price: s.bybit_entry_price,
        upbit_order_id: s.upbit_order_id.clone(),
        bybit_order_id: s.bybit_order_id.clone(),
        entry_spread_pct: s.entry_spread_pct,
        entry_z_score: s.entry_z_score,
        entry_usd_krw: s.entry_usd_krw,
        opened_at: s.opened_at,
        closed_at: s.closed_at,
        realized_pnl: s.realized_pnl,
        exit_upbit_order_id: s.exit_upbit_order_id.clone(),
        exit_bybit_order_id: s.exit_bybit_order_id.clone(),
        client_order_id: s.client_order_id.clone(),
        exit_client_order_id: s.exit_client_order_id.clone(),
        in_flight: s.in_flight,
        succeeded_leg: s.succeeded_leg.clone(),
        emergency_attempts: s.emergency_attempts,
    }
}

/// arb-db PositionRecord → arb-strategy PositionRecord.
fn to_strategy_record(d: arb_db::positions::PositionRecord) -> StrategyPositionRecord {
    StrategyPositionRecord {
        id: d.id,
        session_id: d.session_id,
        coin: d.coin,
        state: d.state,
        upbit_qty: d.upbit_qty,
        bybit_qty: d.bybit_qty,
        upbit_entry_price: d.upbit_entry_price,
        bybit_entry_price: d.bybit_entry_price,
        upbit_order_id: d.upbit_order_id,
        bybit_order_id: d.bybit_order_id,
        entry_spread_pct: d.entry_spread_pct,
        entry_z_score: d.entry_z_score,
        entry_usd_krw: d.entry_usd_krw,
        opened_at: d.opened_at,
        closed_at: d.closed_at,
        realized_pnl: d.realized_pnl,
        exit_upbit_order_id: d.exit_upbit_order_id,
        exit_bybit_order_id: d.exit_bybit_order_id,
        client_order_id: d.client_order_id,
        exit_client_order_id: d.exit_client_order_id,
        in_flight: d.in_flight,
        succeeded_leg: d.succeeded_leg,
        emergency_attempts: d.emergency_attempts,
    }
}

/// arb-strategy UpdateFields → arb-db UpdateFields.
fn to_db_update_fields(s: StrategyUpdateFields) -> arb_db::positions::UpdateFields {
    arb_db::positions::UpdateFields {
        upbit_order_id: s.upbit_order_id,
        bybit_order_id: s.bybit_order_id,
        upbit_qty: s.upbit_qty,
        bybit_qty: s.bybit_qty,
        upbit_entry_price: s.upbit_entry_price,
        bybit_entry_price: s.bybit_entry_price,
        entry_spread_pct: s.entry_spread_pct,
        entry_z_score: s.entry_z_score,
        entry_usd_krw: s.entry_usd_krw,
        opened_at: None,
        closed_at: None,
        realized_pnl: s.realized_pnl,
        exit_upbit_order_id: s.exit_upbit_order_id,
        exit_bybit_order_id: s.exit_bybit_order_id,
        exit_client_order_id: s.exit_client_order_id,
        in_flight: s.in_flight,
        succeeded_leg: s.succeeded_leg,
        emergency_attempts: s.emergency_attempts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_to_db_record_roundtrip() {
        let strategy_rec = StrategyPositionRecord {
            id: Some(1),
            session_id: 42,
            coin: "BTC".to_string(),
            state: "Open".to_string(),
            upbit_qty: Decimal::new(1, 2),
            bybit_qty: Decimal::new(1, 2),
            upbit_entry_price: Some(Decimal::new(60_000_000, 0)),
            bybit_entry_price: Some(Decimal::new(42000, 0)),
            upbit_order_id: Some("upbit-123".to_string()),
            bybit_order_id: Some("bybit-456".to_string()),
            entry_spread_pct: Some(0.2),
            entry_z_score: Some(2.5),
            entry_usd_krw: Some(1380.0),
            opened_at: None,
            closed_at: None,
            realized_pnl: None,
            exit_upbit_order_id: None,
            exit_bybit_order_id: None,
            client_order_id: Some("client-001".to_string()),
            exit_client_order_id: None,
            in_flight: false,
            succeeded_leg: None,
            emergency_attempts: 0,
        };

        let db_rec = to_db_record(&strategy_rec);
        assert_eq!(db_rec.id, Some(1));
        assert_eq!(db_rec.coin, "BTC");
        assert_eq!(db_rec.session_id, 42);

        let back = to_strategy_record(db_rec);
        assert_eq!(back.id, Some(1));
        assert_eq!(back.coin, "BTC");
        assert_eq!(back.session_id, 42);
        assert_eq!(back.upbit_order_id, Some("upbit-123".to_string()));
    }

    #[test]
    fn test_to_db_update_fields() {
        let fields = StrategyUpdateFields {
            upbit_order_id: Some("upbit-new".to_string()),
            in_flight: Some(false),
            ..Default::default()
        };

        let db_fields = to_db_update_fields(fields);
        assert_eq!(db_fields.upbit_order_id, Some("upbit-new".to_string()));
        assert_eq!(db_fields.in_flight, Some(false));
        assert!(db_fields.bybit_order_id.is_none());
    }

    #[test]
    fn test_transition_result_mapping() {
        // DbTransitionResult::Applied → StrategyTransitionResult::Applied
        let db_applied = DbTransitionResult::Applied;
        let strategy_result = match db_applied {
            DbTransitionResult::Applied => StrategyTransitionResult::Applied,
            DbTransitionResult::AlreadyTransitioned(_) => {
                StrategyTransitionResult::AlreadyTransitioned
            }
        };
        assert_eq!(strategy_result, StrategyTransitionResult::Applied);

        // DbTransitionResult::AlreadyTransitioned → StrategyTransitionResult::AlreadyTransitioned
        let db_already = DbTransitionResult::AlreadyTransitioned("Closed".to_string());
        let strategy_result = match db_already {
            DbTransitionResult::Applied => StrategyTransitionResult::Applied,
            DbTransitionResult::AlreadyTransitioned(_) => {
                StrategyTransitionResult::AlreadyTransitioned
            }
        };
        assert_eq!(
            strategy_result,
            StrategyTransitionResult::AlreadyTransitioned
        );
    }
}
