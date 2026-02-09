//! 세션 출력 모듈.
//!
//! 거래 내역, 분봉 통계, 세션 요약을 CSV/JSON 파일로 저장합니다.
//! CSV는 실시간 append, JSON은 종료 시 일괄 저장합니다.

pub mod summary;
pub mod writer;
