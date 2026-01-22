//! 설정 관련 에러 타입.

use thiserror::Error;

/// 설정 관련 에러.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// 설정 파일을 찾을 수 없음.
    #[error("Configuration file not found: {0}")]
    FileNotFound(String),

    /// 설정 파일 읽기 실패.
    #[error("Failed to read configuration: {0}")]
    ReadError(String),

    /// 설정 파일 파싱 실패.
    #[error("Failed to parse configuration: {0}")]
    ParseError(String),
}
