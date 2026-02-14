//! balance_snapshots 테이블 Repository.
//!
//! 잔고 스냅샷 저장 및 조회.
//! 라이브 전략 수행 중 계좌 가치 변동을 추적하는 감사(audit) 보조 도구.

use crate::error::DbError;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::MySqlPool;
use tracing::debug;

/// 잔고 스냅샷 레코드.
///
/// 하나의 스냅샷 트리거(PERIODIC, POS_ENT, POS_EXT)당
/// Upbit(KRW) + Bybit(USDT) 2행이 동일한 `snapshot_group_id`로 기록된다.
#[derive(Debug, Clone)]
pub struct BalanceSnapshotRow {
    /// 기록 시각 (application level에서 동일 값 강제).
    pub created_at: DateTime<Utc>,
    /// 같은 트리거로 생성된 2행의 그룹 식별자.
    pub snapshot_group_id: i64,
    /// 현재 세션 ID (sessions 테이블 FK).
    pub session_id: i64,
    /// 레코드 타입: "PERIODIC" | "POS_ENT" | "POS_EXT".
    pub record_type: String,
    /// 거래소: "UPBIT" | "BYBIT".
    pub cex: String,
    /// 기축통화: "KRW" | "USDT".
    pub currency: String,
    /// 기축통화 주문 가능 잔고.
    pub available: Decimal,
    /// 기축통화 잠긴 잔고 (주문 중).
    pub locked: Decimal,
    /// 보유 코인/포지션 환산 가치.
    pub coin_value: Decimal,
    /// 총 자산 가치.
    pub total: Decimal,
    /// POS_ENT/POS_EXT 시 positions.id FK, PERIODIC 시 None.
    pub position_id: Option<i64>,
    /// 기록 시점 USD/KRW 공시 환율.
    pub usd_krw: f64,
    /// 기록 시점 USDT/KRW 거래소 시세.
    pub usdt_krw: f64,
    /// USD 환산 총자산.
    pub total_usd: Decimal,
    /// USDT 환산 총자산.
    pub total_usdt: Decimal,
}

/// balance_snapshots 테이블 Repository.
#[derive(Debug, Clone)]
pub struct BalanceSnapshotRepository {
    pool: MySqlPool,
}

impl BalanceSnapshotRepository {
    /// 새 Repository 생성.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 단일 스냅샷 INSERT. 생성된 ID 반환.
    pub async fn insert_snapshot(&self, row: &BalanceSnapshotRow) -> Result<i64, DbError> {
        debug!(
            session_id = row.session_id,
            snapshot_group_id = row.snapshot_group_id,
            record_type = %row.record_type,
            cex = %row.cex,
            currency = %row.currency,
            total = %row.total,
            "잔고 스냅샷 INSERT"
        );

        let result = sqlx::query(
            r#"
            INSERT INTO balance_snapshots (
                created_at, snapshot_group_id, session_id,
                record_type, cex, currency,
                available, locked, coin_value, total,
                position_id,
                usd_krw, usdt_krw,
                total_usd, total_usdt
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(row.created_at)
        .bind(row.snapshot_group_id)
        .bind(row.session_id)
        .bind(&row.record_type)
        .bind(&row.cex)
        .bind(&row.currency)
        .bind(row.available)
        .bind(row.locked)
        .bind(row.coin_value)
        .bind(row.total)
        .bind(row.position_id)
        .bind(row.usd_krw)
        .bind(row.usdt_krw)
        .bind(row.total_usd)
        .bind(row.total_usdt)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        debug!(
            snapshot_id = id,
            cex = %row.cex,
            record_type = %row.record_type,
            "잔고 스냅샷 INSERT 완료"
        );
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// BalanceSnapshotRow PERIODIC 타입 생성 테스트.
    #[test]
    fn test_balance_snapshot_row_periodic() {
        let row = BalanceSnapshotRow {
            created_at: Utc::now(),
            snapshot_group_id: 1,
            session_id: 100,
            record_type: "PERIODIC".to_string(),
            cex: "UPBIT".to_string(),
            currency: "KRW".to_string(),
            available: Decimal::new(1_000_000, 0),
            locked: Decimal::new(50_000, 0),
            coin_value: Decimal::new(500_000, 0),
            total: Decimal::new(1_550_000, 0),
            position_id: None,
            usd_krw: 1350.0,
            usdt_krw: 1380.0,
            total_usd: Decimal::new(114815, 2),
            total_usdt: Decimal::new(112319, 2),
        };
        assert_eq!(row.record_type, "PERIODIC");
        assert_eq!(row.cex, "UPBIT");
        assert_eq!(row.currency, "KRW");
        assert!(row.position_id.is_none());
        assert_eq!(row.total, Decimal::new(1_550_000, 0));
    }

    /// BalanceSnapshotRow POS_ENT 타입 (Bybit) 생성 테스트.
    #[test]
    fn test_balance_snapshot_row_pos_ent_bybit() {
        let row = BalanceSnapshotRow {
            created_at: Utc::now(),
            snapshot_group_id: 2,
            session_id: 100,
            record_type: "POS_ENT".to_string(),
            cex: "BYBIT".to_string(),
            currency: "USDT".to_string(),
            available: Decimal::new(5000, 0),
            locked: Decimal::new(100, 0),
            coin_value: Decimal::new(-50, 0),
            total: Decimal::new(5050, 0),
            position_id: Some(42),
            usd_krw: 1350.0,
            usdt_krw: 1380.0,
            total_usd: Decimal::new(5050, 0),
            total_usdt: Decimal::new(5050, 0),
        };
        assert_eq!(row.record_type, "POS_ENT");
        assert_eq!(row.cex, "BYBIT");
        assert_eq!(row.currency, "USDT");
        assert_eq!(row.position_id, Some(42));
        // Bybit coin_value(unrealisedPnl)는 음수 가능
        assert!(row.coin_value < Decimal::ZERO);
    }

    /// BalanceSnapshotRow POS_EXT 타입 생성 테스트.
    #[test]
    fn test_balance_snapshot_row_pos_ext() {
        let row = BalanceSnapshotRow {
            created_at: Utc::now(),
            snapshot_group_id: 3,
            session_id: 100,
            record_type: "POS_EXT".to_string(),
            cex: "UPBIT".to_string(),
            currency: "KRW".to_string(),
            available: Decimal::new(2_000_000, 0),
            locked: Decimal::ZERO,
            coin_value: Decimal::ZERO,
            total: Decimal::new(2_000_000, 0),
            position_id: Some(42),
            usd_krw: 1350.0,
            usdt_krw: 1380.0,
            total_usd: Decimal::new(148148, 2),
            total_usdt: Decimal::new(144928, 2),
        };
        assert_eq!(row.record_type, "POS_EXT");
        assert_eq!(row.position_id, Some(42));
    }

    /// BalanceSnapshotRepository 생성 테스트 (pool mock 없이 구조체 확인).
    #[test]
    fn test_balance_snapshot_repository_debug() {
        // MySqlPool은 mock 없이 직접 생성 불가하므로
        // Debug trait 구현 여부와 Clone 여부만 확인
        // 실제 DB 연동 테스트는 통합 테스트에서 수행
        fn assert_debug_clone<T: std::fmt::Debug + Clone>() {}
        assert_debug_clone::<BalanceSnapshotRepository>();
    }

    /// BalanceSnapshotRow의 Clone, Debug trait 구현 확인.
    #[test]
    fn test_balance_snapshot_row_clone_debug() {
        let row = BalanceSnapshotRow {
            created_at: Utc::now(),
            snapshot_group_id: 1,
            session_id: 1,
            record_type: "PERIODIC".to_string(),
            cex: "UPBIT".to_string(),
            currency: "KRW".to_string(),
            available: Decimal::ONE,
            locked: Decimal::ZERO,
            coin_value: Decimal::ZERO,
            total: Decimal::ONE,
            position_id: None,
            usd_krw: 1350.0,
            usdt_krw: 1380.0,
            total_usd: Decimal::ONE,
            total_usdt: Decimal::ONE,
        };

        // Clone 확인
        let cloned = row.clone();
        assert_eq!(cloned.session_id, row.session_id);
        assert_eq!(cloned.record_type, row.record_type);
        assert_eq!(cloned.cex, row.cex);

        // Debug 확인
        let debug_str = format!("{:?}", row);
        assert!(debug_str.contains("PERIODIC"));
        assert!(debug_str.contains("UPBIT"));
    }

    /// 모든 record_type 값 검증.
    #[test]
    fn test_record_type_values() {
        let valid_types = ["PERIODIC", "POS_ENT", "POS_EXT"];
        for rt in &valid_types {
            let row = BalanceSnapshotRow {
                created_at: Utc::now(),
                snapshot_group_id: 1,
                session_id: 1,
                record_type: rt.to_string(),
                cex: "UPBIT".to_string(),
                currency: "KRW".to_string(),
                available: Decimal::ZERO,
                locked: Decimal::ZERO,
                coin_value: Decimal::ZERO,
                total: Decimal::ZERO,
                position_id: None,
                usd_krw: 0.0,
                usdt_krw: 0.0,
                total_usd: Decimal::ZERO,
                total_usdt: Decimal::ZERO,
            };
            assert_eq!(row.record_type, *rt);
        }
    }

    /// 모든 cex/currency 조합 검증.
    #[test]
    fn test_cex_currency_combinations() {
        // Upbit -> KRW
        let upbit_row = BalanceSnapshotRow {
            created_at: Utc::now(),
            snapshot_group_id: 1,
            session_id: 1,
            record_type: "PERIODIC".to_string(),
            cex: "UPBIT".to_string(),
            currency: "KRW".to_string(),
            available: Decimal::ZERO,
            locked: Decimal::ZERO,
            coin_value: Decimal::ZERO,
            total: Decimal::ZERO,
            position_id: None,
            usd_krw: 1350.0,
            usdt_krw: 1380.0,
            total_usd: Decimal::ZERO,
            total_usdt: Decimal::ZERO,
        };
        assert_eq!(upbit_row.cex, "UPBIT");
        assert_eq!(upbit_row.currency, "KRW");

        // Bybit -> USDT
        let bybit_row = BalanceSnapshotRow {
            created_at: Utc::now(),
            snapshot_group_id: 1,
            session_id: 1,
            record_type: "PERIODIC".to_string(),
            cex: "BYBIT".to_string(),
            currency: "USDT".to_string(),
            available: Decimal::ZERO,
            locked: Decimal::ZERO,
            coin_value: Decimal::ZERO,
            total: Decimal::ZERO,
            position_id: None,
            usd_krw: 1350.0,
            usdt_krw: 1380.0,
            total_usd: Decimal::ZERO,
            total_usdt: Decimal::ZERO,
        };
        assert_eq!(bybit_row.cex, "BYBIT");
        assert_eq!(bybit_row.currency, "USDT");
    }

    /// Decimal 정밀도 테스트 (DECIMAL(20,8) 호환성).
    #[test]
    fn test_decimal_precision() {
        let row = BalanceSnapshotRow {
            created_at: Utc::now(),
            snapshot_group_id: 1,
            session_id: 1,
            record_type: "PERIODIC".to_string(),
            cex: "UPBIT".to_string(),
            currency: "KRW".to_string(),
            available: Decimal::new(123_456_789_012, 8), // 1234.56789012
            locked: Decimal::new(1, 8),                  // 0.00000001 (최소 단위)
            coin_value: Decimal::ZERO,
            total: Decimal::new(123_456_789_013, 8),
            position_id: None,
            usd_krw: 1350.0,
            usdt_krw: 1380.0,
            total_usd: Decimal::new(9145688, 6),
            total_usdt: Decimal::new(8945420, 6),
        };
        // 소수점 8자리 정밀도 확인
        assert_eq!(row.locked, Decimal::new(1, 8));
        assert_eq!(row.available, Decimal::new(123_456_789_012, 8));
    }
}
