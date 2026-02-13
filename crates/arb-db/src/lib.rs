//! # arb-db
//!
//! MySQL(sqlx) 기반 영속화 레이어.
//!
//! 라이브 트레이딩 시스템의 세션, 포지션, 거래, 분봉, 알림, 펀딩 스케줄을
//! MySQL DB에 영속화하는 모듈.
//!
//! ## 구성
//!
//! - [`pool`]: MySQL 커넥션 풀 관리
//! - [`error`]: DB 에러 타입
//! - [`sessions`]: 세션 CRUD
//! - [`positions`]: 포지션 상태 머신 영속화 (PositionStore trait)
//! - [`trades`]: 거래 기록
//! - [`minutes`]: 분봉 스프레드 데이터
//! - [`alerts`]: 알림 기록
//! - [`funding`]: 펀딩 스케줄
//! - [`writer`]: Background DB Writer (mpsc 채널 기반)
//! - [`migration`]: 커스텀 마이그레이션 러너

pub mod alerts;
pub mod error;
pub mod funding;
pub mod migration;
pub mod minutes;
pub mod pool;
pub mod positions;
pub mod sessions;
pub mod trades;
pub mod writer;
