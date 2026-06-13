//! guide(온라인 콘텐츠/해설집) 조회 도메인
//!
//! 설계 SoT = docs/AMK_GUIDE_CONTENT_DESIGN.md. 서빙 모델:
//! 블록 단일 스트림(표는 서버 재조립 격자) + 문장 학습항목 + i18n 해소.

pub mod dto;
pub mod handler;
pub mod repo;
pub mod router;
pub mod service;
