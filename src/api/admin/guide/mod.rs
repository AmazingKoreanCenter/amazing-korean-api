//! guide(온라인 콘텐츠) admin 편집 도메인
//!
//! 설계 SoT = docs/AMK_GUIDE_CONTENT_DESIGN.md (PR-3). D-0: DB가 편집 정본.
//! 단원 메타(공개 flip·테마·제목) / 블록 텍스트(source_version++) / 문장 메타 편집
//! + stale 번역 대시보드 + 디프 export(맥미니 재번역).

pub mod dto;
pub mod handler;
pub mod repo;
pub mod router;
pub mod service;
