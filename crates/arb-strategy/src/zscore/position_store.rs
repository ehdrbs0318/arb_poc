//! 포지션 영속화 trait.
//!
//! PositionStore trait으로 DB 기반 포지션 저장/조회를 추상화합니다.
//! 라이브 전용 기능이며, 시뮬레이션에서는 사용하지 않습니다.

use std::future::Future;

/// 포지션 상태 전이 결과.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionResult {
    /// 정상 전이 완료.
    Applied,
    /// 이미 다른 경로에서 전이됨 (affected_rows == 0).
    AlreadyTransitioned,
}

/// DB 포지션 레코드 (영속화용).
#[derive(Debug, Clone)]
pub struct PositionRecord {
    pub id: Option<i64>,
    pub session_id: i64,
    pub coin: String,
    pub state: String,
    pub upbit_qty: rust_decimal::Decimal,
    pub bybit_qty: rust_decimal::Decimal,
    pub upbit_entry_price: Option<rust_decimal::Decimal>,
    pub bybit_entry_price: Option<rust_decimal::Decimal>,
    pub upbit_order_id: Option<String>,
    pub bybit_order_id: Option<String>,
    pub entry_spread_pct: Option<f64>,
    pub entry_z_score: Option<f64>,
    pub entry_usd_krw: Option<f64>,
    pub opened_at: Option<chrono::DateTime<chrono::Utc>>,
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub realized_pnl: Option<rust_decimal::Decimal>,
    pub exit_upbit_order_id: Option<String>,
    pub exit_bybit_order_id: Option<String>,
    pub client_order_id: Option<String>,
    pub exit_client_order_id: Option<String>,
    pub in_flight: bool,
    pub succeeded_leg: Option<String>,
    pub emergency_attempts: i32,
}

/// 포지션 상태 갱신 필드 (부분 업데이트).
#[derive(Debug, Clone, Default)]
pub struct UpdateFields {
    pub upbit_order_id: Option<String>,
    pub bybit_order_id: Option<String>,
    pub exit_upbit_order_id: Option<String>,
    pub exit_bybit_order_id: Option<String>,
    pub upbit_qty: Option<rust_decimal::Decimal>,
    pub bybit_qty: Option<rust_decimal::Decimal>,
    pub upbit_entry_price: Option<rust_decimal::Decimal>,
    pub bybit_entry_price: Option<rust_decimal::Decimal>,
    pub entry_spread_pct: Option<f64>,
    pub entry_z_score: Option<f64>,
    pub entry_usd_krw: Option<f64>,
    pub realized_pnl: Option<rust_decimal::Decimal>,
    pub in_flight: Option<bool>,
    pub succeeded_leg: Option<String>,
    pub emergency_attempts: Option<i32>,
    pub client_order_id: Option<String>,
    pub exit_client_order_id: Option<String>,
}

/// 포지션 영속화 trait.
///
/// RPITIT 패턴 사용. 메모리가 authoritative이며, DB는 비동기 shadow입니다.
/// 테스트에서는 MockPositionStore 구체 타입을 사용합니다.
pub trait PositionStore: Send + Sync {
    /// 포지션 INSERT (Opening 상태). DB PK를 반환합니다.
    fn save(&self, pos: &PositionRecord) -> impl Future<Output = Result<i64, String>> + Send;

    /// 포지션 상태 전이 (낙관적 잠금: WHERE state = expected_state).
    fn update_state(
        &self,
        id: i64,
        from: &str,
        to: &str,
        fields: UpdateFields,
    ) -> impl Future<Output = Result<TransitionResult, String>> + Send;

    /// 특정 세션의 non-Closed 포지션 조회 (crash recovery용).
    fn load_open(
        &self,
        session_id: i64,
    ) -> impl Future<Output = Result<Vec<PositionRecord>, String>> + Send;

    /// 포지션 삭제 (Opening 미발주 건).
    fn remove(&self, id: i64) -> impl Future<Output = Result<(), String>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// 테스트용 MockPositionStore.
    struct MockPositionStore {
        records: Arc<Mutex<Vec<PositionRecord>>>,
        next_id: Arc<Mutex<i64>>,
    }

    impl MockPositionStore {
        fn new() -> Self {
            Self {
                records: Arc::new(Mutex::new(Vec::new())),
                next_id: Arc::new(Mutex::new(1)),
            }
        }
    }

    impl PositionStore for MockPositionStore {
        async fn save(&self, pos: &PositionRecord) -> Result<i64, String> {
            let mut id_guard = self.next_id.lock().unwrap();
            let id = *id_guard;
            *id_guard += 1;

            let mut record = pos.clone();
            record.id = Some(id);
            self.records.lock().unwrap().push(record);
            Ok(id)
        }

        async fn update_state(
            &self,
            id: i64,
            from: &str,
            to: &str,
            _fields: UpdateFields,
        ) -> Result<TransitionResult, String> {
            let mut records = self.records.lock().unwrap();
            if let Some(rec) = records
                .iter_mut()
                .find(|r| r.id == Some(id) && r.state == from)
            {
                rec.state = to.to_string();
                Ok(TransitionResult::Applied)
            } else {
                Ok(TransitionResult::AlreadyTransitioned)
            }
        }

        async fn load_open(&self, session_id: i64) -> Result<Vec<PositionRecord>, String> {
            let records = self.records.lock().unwrap();
            Ok(records
                .iter()
                .filter(|r| r.session_id == session_id && r.state != "Closed")
                .cloned()
                .collect())
        }

        async fn remove(&self, id: i64) -> Result<(), String> {
            let mut records = self.records.lock().unwrap();
            records.retain(|r| r.id != Some(id));
            Ok(())
        }
    }

    fn make_record(session_id: i64, coin: &str) -> PositionRecord {
        PositionRecord {
            id: None,
            session_id,
            coin: coin.to_string(),
            state: "Opening".to_string(),
            upbit_qty: rust_decimal::Decimal::ONE,
            bybit_qty: rust_decimal::Decimal::ONE,
            upbit_entry_price: Some(rust_decimal::Decimal::new(100_000, 0)),
            bybit_entry_price: Some(rust_decimal::Decimal::new(100_050, 0)),
            upbit_order_id: None,
            bybit_order_id: None,
            entry_spread_pct: Some(0.05),
            entry_z_score: Some(2.5),
            entry_usd_krw: Some(1380.0),
            opened_at: Some(chrono::Utc::now()),
            closed_at: None,
            realized_pnl: None,
            exit_upbit_order_id: None,
            exit_bybit_order_id: None,
            client_order_id: None,
            exit_client_order_id: None,
            in_flight: false,
            succeeded_leg: None,
            emergency_attempts: 0,
        }
    }

    #[tokio::test]
    async fn test_mock_save_and_load() {
        let store = MockPositionStore::new();

        let rec = make_record(1, "BTC");
        let id = store.save(&rec).await.unwrap();
        assert_eq!(id, 1);

        let open = store.load_open(1).await.unwrap();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].coin, "BTC");
        assert_eq!(open[0].id, Some(1));
    }

    #[tokio::test]
    async fn test_mock_update_state() {
        let store = MockPositionStore::new();

        let rec = make_record(1, "ETH");
        let id = store.save(&rec).await.unwrap();

        // Opening → Open 전이
        let result = store
            .update_state(id, "Opening", "Open", UpdateFields::default())
            .await
            .unwrap();
        assert_eq!(result, TransitionResult::Applied);

        // 같은 전이 재시도 → AlreadyTransitioned
        let result = store
            .update_state(id, "Opening", "Open", UpdateFields::default())
            .await
            .unwrap();
        assert_eq!(result, TransitionResult::AlreadyTransitioned);
    }

    #[tokio::test]
    async fn test_mock_remove() {
        let store = MockPositionStore::new();

        let rec = make_record(1, "BTC");
        let id = store.save(&rec).await.unwrap();

        store.remove(id).await.unwrap();

        let open = store.load_open(1).await.unwrap();
        assert!(open.is_empty());
    }

    #[tokio::test]
    async fn test_mock_load_open_excludes_closed() {
        let store = MockPositionStore::new();

        let rec1 = make_record(1, "BTC");
        let id1 = store.save(&rec1).await.unwrap();

        let rec2 = make_record(1, "ETH");
        let _id2 = store.save(&rec2).await.unwrap();

        // BTC를 Closed로 전이
        store
            .update_state(id1, "Opening", "Closed", UpdateFields::default())
            .await
            .unwrap();

        let open = store.load_open(1).await.unwrap();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].coin, "ETH");
    }

    #[tokio::test]
    async fn test_mock_load_open_filters_by_session() {
        let store = MockPositionStore::new();

        let rec1 = make_record(1, "BTC");
        store.save(&rec1).await.unwrap();

        let rec2 = make_record(2, "ETH");
        store.save(&rec2).await.unwrap();

        // session 1만 조회
        let open = store.load_open(1).await.unwrap();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].coin, "BTC");
    }

    #[tokio::test]
    async fn test_mock_multiple_saves() {
        let store = MockPositionStore::new();

        let id1 = store.save(&make_record(1, "BTC")).await.unwrap();
        let id2 = store.save(&make_record(1, "ETH")).await.unwrap();
        let id3 = store.save(&make_record(1, "XRP")).await.unwrap();

        // ID가 순차적으로 할당됨
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);

        let open = store.load_open(1).await.unwrap();
        assert_eq!(open.len(), 3);
    }
}
