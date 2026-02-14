//! DB 상태 점검용 프로브.
//!
//! 최근 세션 기준으로 핵심 테이블 상태를 요약 출력합니다.

use std::error::Error;

use arb_poc::config::Config;
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySqlPool, Row};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load_or_default();
    let database_url = std::env::var("DATABASE_URL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| config.database.url.clone());

    if database_url.trim().is_empty() {
        return Err("database url is empty (DATABASE_URL or config.toml [database].url)".into());
    }

    let pool = MySqlPoolOptions::new()
        .max_connections(2)
        .min_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    emit_summary(&pool).await?;
    Ok(())
}

async fn emit_summary(pool: &MySqlPool) -> Result<(), Box<dyn Error>> {
    let now = Utc::now();
    println!("=== db_probe ts={} ===", now.to_rfc3339());

    let session_row = if let Ok(raw) = std::env::var("SESSION_ID") {
        let session_id = raw
            .parse::<i64>()
            .map_err(|e| format!("invalid SESSION_ID '{raw}': {e}"))?;
        sqlx::query("SELECT id, status, started_at, ended_at FROM sessions WHERE id = ? LIMIT 1")
            .bind(session_id)
            .fetch_optional(pool)
            .await?
    } else {
        sqlx::query("SELECT id, status, started_at, ended_at FROM sessions ORDER BY id DESC LIMIT 1")
            .fetch_optional(pool)
            .await?
    };

    let Some(session_row) = session_row else {
        println!("session:none");
        return Ok(());
    };

    let session_id: i64 = session_row.get("id");
    let session_status: String = session_row.get("status");
    let started_at: chrono::NaiveDateTime = session_row.get("started_at");
    let ended_at: Option<chrono::NaiveDateTime> = session_row.get("ended_at");
    println!(
        "session:id={} status={} started_at={} ended_at={:?}",
        session_id, session_status, started_at, ended_at
    );

    let minute_row =
        sqlx::query("SELECT COUNT(*) AS cnt, MAX(ts) AS max_ts FROM minutes WHERE session_id = ?")
            .bind(session_id)
            .fetch_one(pool)
            .await?;
    let minute_count: i64 = minute_row.get("cnt");
    let minute_max_ts: Option<chrono::NaiveDateTime> = minute_row.get("max_ts");
    println!("minutes:count={} max_ts={:?}", minute_count, minute_max_ts);

    let spread_row = sqlx::query(
        "SELECT \
            CAST(SUM(CASE WHEN spread_pct < 0 THEN 1 ELSE 0 END) AS SIGNED) AS neg_cnt, \
            CAST(SUM(CASE WHEN spread_pct > 0 THEN 1 ELSE 0 END) AS SIGNED) AS pos_cnt, \
            MIN(spread_pct) AS min_spread, \
            MAX(spread_pct) AS max_spread, \
            AVG(spread_pct) AS avg_spread, \
            AVG(stddev) AS avg_stddev \
         FROM minutes \
         WHERE session_id = ? AND spread_pct IS NOT NULL",
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;
    let spread_neg: Option<i64> = spread_row.get("neg_cnt");
    let spread_pos: Option<i64> = spread_row.get("pos_cnt");
    let spread_min: Option<f64> = spread_row.get("min_spread");
    let spread_max: Option<f64> = spread_row.get("max_spread");
    let spread_avg: Option<f64> = spread_row.get("avg_spread");
    let avg_stddev: Option<f64> = spread_row.get("avg_stddev");
    println!(
        "minutes_spread:neg={} pos={} min={:?} max={:?} avg={:?} avg_stddev={:?}",
        spread_neg.unwrap_or(0),
        spread_pos.unwrap_or(0),
        spread_min,
        spread_max,
        spread_avg,
        avg_stddev
    );

    let z_row = sqlx::query(
        "SELECT COUNT(*) AS cnt, \
                CAST(SUM(CASE WHEN z_score < 0 THEN 1 ELSE 0 END) AS SIGNED) AS neg_cnt, \
                CAST(SUM(CASE WHEN z_score > 0 THEN 1 ELSE 0 END) AS SIGNED) AS pos_cnt, \
                CAST(SUM(CASE WHEN z_score = 0 THEN 1 ELSE 0 END) AS SIGNED) AS zero_cnt, \
                MIN(z_score) AS min_z, \
                MAX(z_score) AS max_z \
         FROM minutes \
         WHERE session_id = ? AND z_score IS NOT NULL",
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;
    let z_count: i64 = z_row.get("cnt");
    let z_neg: Option<i64> = z_row.get("neg_cnt");
    let z_pos: Option<i64> = z_row.get("pos_cnt");
    let z_zero: Option<i64> = z_row.get("zero_cnt");
    let z_min: Option<f64> = z_row.get("min_z");
    let z_max: Option<f64> = z_row.get("max_z");
    println!(
        "minutes_zscore:count={} neg={} pos={} zero={} min={:?} max={:?}",
        z_count,
        z_neg.unwrap_or(0),
        z_pos.unwrap_or(0),
        z_zero.unwrap_or(0),
        z_min,
        z_max
    );

    let entry_z_threshold = std::env::var("ENTRY_Z")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.0);
    let exit_z_threshold = std::env::var("EXIT_Z")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.1);
    let signal_row = sqlx::query(
        "SELECT \
            CAST(SUM(CASE WHEN z_score >= ? THEN 1 ELSE 0 END) AS SIGNED) AS ge_entry, \
            CAST(SUM(CASE WHEN z_score <= -? THEN 1 ELSE 0 END) AS SIGNED) AS le_neg_entry, \
            CAST(SUM(CASE WHEN ABS(z_score) <= ? THEN 1 ELSE 0 END) AS SIGNED) AS in_exit_band, \
            AVG(ABS(z_score)) AS avg_abs_z \
         FROM minutes \
         WHERE session_id = ? AND z_score IS NOT NULL",
    )
    .bind(entry_z_threshold)
    .bind(entry_z_threshold)
    .bind(exit_z_threshold)
    .bind(session_id)
    .fetch_one(pool)
    .await?;
    let ge_entry: Option<i64> = signal_row.get("ge_entry");
    let le_neg_entry: Option<i64> = signal_row.get("le_neg_entry");
    let in_exit_band: Option<i64> = signal_row.get("in_exit_band");
    let avg_abs_z: Option<f64> = signal_row.get("avg_abs_z");
    println!(
        "minutes_signal:entry_z={} exit_z={} ge_entry={} le_neg_entry={} in_exit_band={} avg_abs_z={:?}",
        entry_z_threshold,
        exit_z_threshold,
        ge_entry.unwrap_or(0),
        le_neg_entry.unwrap_or(0),
        in_exit_band.unwrap_or(0),
        avg_abs_z
    );

    let coin_rows = sqlx::query(
        "SELECT coin, COUNT(*) AS cnt, \
                MIN(z_score) AS min_z, MAX(z_score) AS max_z, AVG(z_score) AS avg_z \
         FROM minutes \
         WHERE session_id = ? AND z_score IS NOT NULL \
         GROUP BY coin \
         ORDER BY cnt DESC, coin ASC \
         LIMIT 10",
    )
    .bind(session_id)
    .fetch_all(pool)
    .await?;
    println!("minutes_top_coins:");
    for row in coin_rows {
        let coin: String = row.get("coin");
        let cnt: i64 = row.get("cnt");
        let min_z: Option<f64> = row.get("min_z");
        let max_z: Option<f64> = row.get("max_z");
        let avg_z: Option<f64> = row.get("avg_z");
        println!(
            "  coin={} count={} min_z={:?} max_z={:?} avg_z={:?}",
            coin, cnt, min_z, max_z, avg_z
        );
    }

    let funding_row = sqlx::query(
        "SELECT COUNT(*) AS cnt, MAX(updated_at) AS max_updated, MAX(next_funding_time) AS max_next FROM funding_schedules",
    )
    .fetch_one(pool)
    .await?;
    let funding_count: i64 = funding_row.get("cnt");
    let funding_max_updated: Option<chrono::NaiveDateTime> = funding_row.get("max_updated");
    let funding_max_next: Option<chrono::NaiveDateTime> = funding_row.get("max_next");
    println!(
        "funding_schedules:count={} max_updated={:?} max_next={:?}",
        funding_count, funding_max_updated, funding_max_next
    );

    let snapshot_row = sqlx::query(
        "SELECT COUNT(*) AS cnt, MAX(created_at) AS max_created FROM balance_snapshots WHERE session_id = ?",
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;
    let snapshot_count: i64 = snapshot_row.get("cnt");
    let snapshot_max_created: Option<chrono::NaiveDateTime> = snapshot_row.get("max_created");
    println!(
        "balance_snapshots:count={} max_created={:?}",
        snapshot_count, snapshot_max_created
    );

    let latest_rows = sqlx::query(
        "SELECT id, created_at, cex, currency, available, locked, coin_value, total \
         FROM balance_snapshots WHERE session_id = ? ORDER BY id DESC LIMIT 8",
    )
    .bind(session_id)
    .fetch_all(pool)
    .await?;
    for row in latest_rows {
        let id: i64 = row.get("id");
        let created_at: chrono::NaiveDateTime = row.get("created_at");
        let cex: String = row.get("cex");
        let currency: String = row.get("currency");
        let available: Decimal = row.get("available");
        let locked: Decimal = row.get("locked");
        let coin_value: Decimal = row.get("coin_value");
        let total: Decimal = row.get("total");
        println!(
            "snapshot:id={} created_at={} cex={} currency={} available={} locked={} coin_value={} total={}",
            id, created_at, cex, currency, available, locked, coin_value, total
        );
    }

    let positions_row = sqlx::query(
        "SELECT COUNT(*) AS open_cnt FROM positions WHERE session_id = ? AND state NOT IN ('Closed','Canceled','Error')",
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;
    let open_positions: i64 = positions_row.get("open_cnt");
    println!("positions:open_count={}", open_positions);

    let trades_row = sqlx::query(
        "SELECT COUNT(*) AS cnt, MAX(executed_at) AS max_ts FROM trades WHERE session_id = ?",
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?;
    let trade_count: i64 = trades_row.get("cnt");
    let trade_max_ts: Option<chrono::NaiveDateTime> = trades_row.get("max_ts");
    println!("trades:count={} max_ts={:?}", trade_count, trade_max_ts);

    Ok(())
}
