//! trades 테이블 Repository.
//!
//! 거래 기록 저장 및 조회.

use crate::error::DbError;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::MySqlPool;
use tracing::debug;

/// 거래 레코드.
#[derive(Debug, Clone)]
pub struct TradeRecord {
    pub id: Option<i64>,
    pub session_id: i64,
    pub position_id: i64,
    pub coin: String,
    pub side: String,
    pub qty: Decimal,
    pub upbit_price_krw: Option<Decimal>,
    pub bybit_price_usdt: Option<Decimal>,
    pub upbit_fee: Option<Decimal>,
    pub bybit_fee: Option<Decimal>,
    pub spread_pct: Option<f64>,
    pub z_score: Option<f64>,
    pub realized_pnl: Option<Decimal>,
    pub adjustment_cost: Option<Decimal>,
    pub exit_usd_krw: Option<f64>,
    pub executed_at: DateTime<Utc>,
}

/// trades 테이블 Repository.
#[derive(Debug, Clone)]
pub struct TradeRepository {
    pool: MySqlPool,
}

impl TradeRepository {
    /// 새 Repository 생성.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 거래 기록 INSERT. 생성된 ID 반환.
    pub async fn insert_trade(&self, trade: &TradeRecord) -> Result<i64, DbError> {
        debug!(
            session_id = trade.session_id,
            position_id = trade.position_id,
            coin = %trade.coin,
            side = %trade.side,
            qty = %trade.qty,
            "거래 INSERT"
        );

        let result = sqlx::query(
            r#"
            INSERT INTO trades (
                session_id, position_id, coin, side, qty,
                upbit_price_krw, bybit_price_usdt,
                upbit_fee, bybit_fee,
                spread_pct, z_score, realized_pnl, adjustment_cost,
                exit_usd_krw, executed_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(trade.session_id)
        .bind(trade.position_id)
        .bind(&trade.coin)
        .bind(&trade.side)
        .bind(trade.qty)
        .bind(trade.upbit_price_krw)
        .bind(trade.bybit_price_usdt)
        .bind(trade.upbit_fee)
        .bind(trade.bybit_fee)
        .bind(trade.spread_pct)
        .bind(trade.z_score)
        .bind(trade.realized_pnl)
        .bind(trade.adjustment_cost)
        .bind(trade.exit_usd_krw)
        .bind(trade.executed_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        debug!(trade_id = id, "거래 INSERT 완료");
        Ok(id)
    }

    /// 특정 세션의 거래 기록 조회.
    pub async fn get_trades_by_session(
        &self,
        session_id: i64,
    ) -> Result<Vec<TradeRecord>, DbError> {
        debug!(session_id = session_id, "세션별 거래 조회");

        let rows = sqlx::query_as::<
            _,
            (
                i64,
                i64,
                i64,
                String,
                String,
                Decimal,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<Decimal>,
                Option<f64>,
                Option<f64>,
                Option<Decimal>,
                Option<Decimal>,
                Option<f64>,
                DateTime<Utc>,
            ),
        >(
            r#"
            SELECT
                id, session_id, position_id, coin, side,
                qty,
                upbit_price_krw, bybit_price_usdt,
                upbit_fee, bybit_fee,
                spread_pct, z_score,
                realized_pnl, adjustment_cost,
                exit_usd_krw, executed_at
            FROM trades
            WHERE session_id = ?
            ORDER BY id
            "#,
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await?;

        let trades: Vec<TradeRecord> = rows
            .into_iter()
            .map(|r| TradeRecord {
                id: Some(r.0),
                session_id: r.1,
                position_id: r.2,
                coin: r.3,
                side: r.4,
                qty: r.5,
                upbit_price_krw: r.6,
                bybit_price_usdt: r.7,
                upbit_fee: r.8,
                bybit_fee: r.9,
                spread_pct: r.10,
                z_score: r.11,
                realized_pnl: r.12,
                adjustment_cost: r.13,
                exit_usd_krw: r.14,
                executed_at: r.15,
            })
            .collect();

        debug!(
            session_id = session_id,
            count = trades.len(),
            "세션별 거래 조회 완료"
        );
        Ok(trades)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_record_creation() {
        let record = TradeRecord {
            id: None,
            session_id: 1,
            position_id: 10,
            coin: "BTC".to_string(),
            side: "entry".to_string(),
            qty: Decimal::new(5000, 4),
            upbit_price_krw: Some(Decimal::new(50_000_000, 0)),
            bybit_price_usdt: Some(Decimal::new(35000, 0)),
            upbit_fee: Some(Decimal::new(25, 2)),
            bybit_fee: Some(Decimal::new(10, 2)),
            spread_pct: Some(0.15),
            z_score: Some(2.5),
            realized_pnl: None,
            adjustment_cost: None,
            exit_usd_krw: None,
            executed_at: Utc::now(),
        };
        assert!(record.id.is_none());
        assert_eq!(record.side, "entry");
        assert!(record.realized_pnl.is_none());
    }

    #[test]
    fn test_trade_record_exit() {
        let record = TradeRecord {
            id: Some(100),
            session_id: 1,
            position_id: 10,
            coin: "ETH".to_string(),
            side: "exit".to_string(),
            qty: Decimal::new(1, 0),
            upbit_price_krw: Some(Decimal::new(3_000_000, 0)),
            bybit_price_usdt: Some(Decimal::new(2100, 0)),
            upbit_fee: Some(Decimal::new(15, 2)),
            bybit_fee: Some(Decimal::new(5, 2)),
            spread_pct: Some(-0.05),
            z_score: Some(-1.0),
            realized_pnl: Some(Decimal::new(500, 2)),
            adjustment_cost: None,
            exit_usd_krw: Some(1350.0),
            executed_at: Utc::now(),
        };
        assert_eq!(record.id, Some(100));
        assert_eq!(record.side, "exit");
        assert!(record.realized_pnl.is_some());
        assert!(record.exit_usd_krw.is_some());
    }
}
