//! # DB 마이그레이션 실행 예제
//!
//! DATABASE_URL 환경변수로 MySQL 연결 후 마이그레이션 실행.
//!
//! ```bash
//! DATABASE_URL="mysql://user:pass@localhost/arb_db" cargo run --example migrate
//! ```

use arb_db::migration::run_migrations;
use arb_db::pool::{DbPool, DbPoolConfig};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 초기화
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // DATABASE_URL 환경변수에서 연결 URL 읽기
    let database_url = std::env::var("DATABASE_URL").map_err(|_| {
        "DATABASE_URL environment variable not set. Example: DATABASE_URL=\"mysql://user:pass@localhost/arb_db\""
    })?;

    println!("DB 연결 중...");
    let config = DbPoolConfig {
        max_connections: 2,
        min_connections: 1,
        acquire_timeout_secs: 10,
    };
    let pool = DbPool::connect(&database_url, &config).await?;

    println!("마이그레이션 실행 중...");
    let migrations_dir = Path::new("crates/arb-db/migrations");
    run_migrations(pool.inner(), migrations_dir).await?;

    println!("마이그레이션 완료!");
    Ok(())
}
