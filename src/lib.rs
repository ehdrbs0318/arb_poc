//! # arb_poc
//!
//! 암호화폐 거래소를 위한 차익거래 시스템 개념 증명(PoC)입니다.
//!
//! 이 크레이트는 여러 암호화폐 거래소와 상호작용하고
//! 차익거래 전략을 구현하기 위한 추상화를 제공합니다.
//!
//! ## 아키텍처
//!
//! 이 크레이트는 다음 모듈들로 구성되어 있습니다:
//!
//! - [`exchange`]: 거래소 추상화를 위한 공통 trait 및 타입
//! - [`exchanges`]: 특정 거래소 구현체 (예: Upbit)
//! - [`config`]: 설정 관리
//! - [`logging`]: 구조화된 로깅 시스템

pub mod config;
pub mod exchange;
pub mod exchanges;
pub mod logging;
