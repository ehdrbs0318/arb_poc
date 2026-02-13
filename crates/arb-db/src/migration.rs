//! 커스텀 마이그레이션 러너.
//!
//! `_migrations` 테이블을 자동 생성하고, `migrations/` 디렉토리의 SQL 파일을 순차 적용.
//! MySQL DDL은 암묵적 COMMIT을 발생시키므로 파일당 하나의 DDL만 포함해야 함.

use crate::error::DbError;
use sqlx::MySqlPool;
use std::path::Path;
use tracing::{debug, info, warn};

/// 마이그레이션 파일 정보.
#[derive(Debug)]
struct MigrationFile {
    version: i32,
    name: String,
    sql: String,
}

/// 마이그레이션 실행.
///
/// # 인자
///
/// * `pool` - MySQL 커넥션 풀
/// * `migrations_dir` - 마이그레이션 SQL 파일 디렉토리 경로
///
/// # 동작
///
/// 1. `_migrations` 테이블 존재 확인 (CREATE IF NOT EXISTS)
/// 2. 적용된 버전 목록 조회
/// 3. 미적용 버전 필터링 + 버전 오름차순 정렬
/// 4. 각 파일: BEGIN -> SQL 실행 -> INSERT INTO _migrations -> COMMIT
/// 5. 실패 시 ROLLBACK + 에러 반환
pub async fn run_migrations(pool: &MySqlPool, migrations_dir: &Path) -> Result<(), DbError> {
    info!(
        dir = %migrations_dir.display(),
        "마이그레이션 시작"
    );

    // 1. _migrations 테이블 생성
    ensure_migrations_table(pool).await?;

    // 2. 적용된 버전 목록 조회
    let applied = get_applied_versions(pool).await?;
    debug!(applied_versions = ?applied, "적용된 마이그레이션 버전");

    // 3. 마이그레이션 파일 스캔
    let mut files = scan_migration_files(migrations_dir)?;
    files.sort_by_key(|f| f.version);

    // 4. 미적용 버전 필터링 + 적용
    let mut applied_count = 0;
    for file in &files {
        if applied.contains(&file.version) {
            debug!(version = file.version, name = %file.name, "이미 적용됨, 건너뜀");
            continue;
        }

        let start = std::time::Instant::now();
        apply_migration(pool, file).await?;
        let elapsed = start.elapsed();

        info!(
            version = file.version,
            name = %file.name,
            elapsed_ms = elapsed.as_millis() as u64,
            "Applied V{:03}__{} ({}ms)",
            file.version,
            file.name,
            elapsed.as_millis()
        );
        applied_count += 1;
    }

    if applied_count == 0 {
        info!("적용할 마이그레이션 없음");
    } else {
        info!(applied_count = applied_count, "마이그레이션 완료");
    }

    Ok(())
}

/// `_migrations` 테이블 생성 (이미 존재하면 무시).
async fn ensure_migrations_table(pool: &MySqlPool) -> Result<(), DbError> {
    debug!("_migrations 테이블 확인");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _migrations (
            version     INT PRIMARY KEY,
            name        VARCHAR(255) NOT NULL,
            applied_at  DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::MigrationFailed(format!("failed to create _migrations table: {}", e)))?;

    Ok(())
}

/// 적용된 마이그레이션 버전 목록 조회.
async fn get_applied_versions(pool: &MySqlPool) -> Result<Vec<i32>, DbError> {
    let rows = sqlx::query_as::<_, (i32,)>("SELECT version FROM _migrations ORDER BY version")
        .fetch_all(pool)
        .await
        .map_err(|e| DbError::MigrationFailed(format!("failed to query _migrations: {}", e)))?;

    Ok(rows.into_iter().map(|(v,)| v).collect())
}

/// 마이그레이션 디렉토리에서 SQL 파일 스캔.
///
/// 파일명 형식: `V{version}__{name}.sql` (예: `V001__create_sessions.sql`)
fn scan_migration_files(dir: &Path) -> Result<Vec<MigrationFile>, DbError> {
    if !dir.exists() {
        warn!(dir = %dir.display(), "마이그레이션 디렉토리 없음");
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    let entries = std::fs::read_dir(dir).map_err(|e| {
        DbError::MigrationFailed(format!("failed to read migrations directory: {}", e))
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            DbError::MigrationFailed(format!("failed to read directory entry: {}", e))
        })?;

        let path = entry.path();
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) if name.ends_with(".sql") => name.to_string(),
            _ => continue,
        };

        // V001__create_sessions.sql -> version=1, name=create_sessions
        let (version, name) = parse_migration_filename(&file_name)?;
        let sql = std::fs::read_to_string(&path).map_err(|e| {
            DbError::MigrationFailed(format!(
                "failed to read migration file {}: {}",
                file_name, e
            ))
        })?;

        files.push(MigrationFile { version, name, sql });
    }

    Ok(files)
}

/// 마이그레이션 파일명 파싱.
///
/// `V001__create_sessions.sql` -> (1, "create_sessions")
fn parse_migration_filename(filename: &str) -> Result<(i32, String), DbError> {
    let without_ext = filename
        .strip_suffix(".sql")
        .ok_or_else(|| DbError::MigrationFailed(format!("invalid migration file: {}", filename)))?;

    let rest = without_ext.strip_prefix('V').ok_or_else(|| {
        DbError::MigrationFailed(format!("migration file must start with 'V': {}", filename))
    })?;

    let sep_pos = rest.find("__").ok_or_else(|| {
        DbError::MigrationFailed(format!("migration file must contain '__': {}", filename))
    })?;

    let version_str = &rest[..sep_pos];
    let name = &rest[sep_pos + 2..];

    let version: i32 = version_str.parse().map_err(|_| {
        DbError::MigrationFailed(format!(
            "invalid version number in {}: '{}'",
            filename, version_str
        ))
    })?;

    Ok((version, name.to_string()))
}

/// 단일 마이그레이션 파일 적용.
async fn apply_migration(pool: &MySqlPool, file: &MigrationFile) -> Result<(), DbError> {
    debug!(
        version = file.version,
        name = %file.name,
        sql_len = file.sql.len(),
        "마이그레이션 적용 시작"
    );

    // MySQL DDL은 암묵적 COMMIT을 발생시키므로
    // DDL 실행 후 INSERT INTO _migrations를 별도 실행
    sqlx::query(&file.sql).execute(pool).await.map_err(|e| {
        DbError::MigrationFailed(format!(
            "failed to execute V{:03}__{}: {}",
            file.version, file.name, e
        ))
    })?;

    sqlx::query("INSERT INTO _migrations (version, name) VALUES (?, ?)")
        .bind(file.version)
        .bind(&file.name)
        .execute(pool)
        .await
        .map_err(|e| {
            DbError::MigrationFailed(format!(
                "failed to record migration V{:03}__{}: {}",
                file.version, file.name, e
            ))
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_migration_filename_valid() {
        let (version, name) = parse_migration_filename("V001__create_sessions.sql").unwrap();
        assert_eq!(version, 1);
        assert_eq!(name, "create_sessions");
    }

    #[test]
    fn test_parse_migration_filename_high_version() {
        let (version, name) = parse_migration_filename("V123__add_index.sql").unwrap();
        assert_eq!(version, 123);
        assert_eq!(name, "add_index");
    }

    #[test]
    fn test_parse_migration_filename_no_v_prefix() {
        let result = parse_migration_filename("001__create_sessions.sql");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_migration_filename_no_double_underscore() {
        let result = parse_migration_filename("V001_create_sessions.sql");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_migration_filename_no_sql_ext() {
        let result = parse_migration_filename("V001__create_sessions.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_migration_filename_invalid_version() {
        let result = parse_migration_filename("Vabc__create_sessions.sql");
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_nonexistent_dir() {
        let files = scan_migration_files(Path::new("/nonexistent/path")).unwrap();
        assert!(files.is_empty());
    }
}
