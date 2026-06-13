//! guide 콘텐츠 조회 DTO
//!
//! 서빙 모델 = 블록 단일 스트림 + 표 재조립 격자(D-7) + 문장 학습항목.
//! 텍스트 해소: `text` = 표시 언어(번역 → en → ko 폴백), `text_ko` = 언어불변
//! 한국어 학습 콘텐츠(항상 병기 — 해설집 이중언어 구조 승계).

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::types::SupportedLanguage;

/// `GET /guides` · `GET /guides/{guide_idx}` 쿼리
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct GuideLangReq {
    /// 표시 언어 (없으면 ko 우선 폴백)
    pub lang: Option<SupportedLanguage>,
}

/// 단원 목록 카드
#[derive(Debug, Serialize, ToSchema)]
pub struct GuideSummaryRes {
    pub guide_idx: String,
    pub guide_seq: i32,
    pub guide_category: String,
    /// 교재 10색 테마 (D-8): blue/green/orange/purple/pink/teal/indigo/rose/amber/slate
    pub guide_theme: String,
    pub sentence_start: Option<i32>,
    pub sentence_end: Option<i32>,
    /// 표시 언어 해소 제목 (번역 → en → ko)
    pub title: Option<String>,
    pub title_ko: Option<String>,
    pub subtitle: Option<String>,
    pub subtitle_ko: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GuideListRes {
    pub items: Vec<GuideSummaryRes>,
    pub lang: String,
}

/// 표 셀 (서버 재조립 격자의 원소)
#[derive(Debug, Serialize, ToSchema)]
pub struct GuideCellRes {
    /// 표시 언어 해소 텍스트 (마커 셀이면 None)
    pub text: Option<String>,
    /// 언어불변 한국어 학습 콘텐츠
    pub text_ko: Option<String>,
    /// symbol_only / table_content_ko_learning / table_content_en / empty:* — 렌더 규칙
    pub marker: Option<String>,
    /// thead 행 여부 (block_type=table_header)
    pub header: bool,
    pub col_span: Option<i32>,
    pub row_span: Option<i32>,
}

/// 콘텐츠 스트림 아이템 — kind="block"(단일 블록) 또는 kind="table"(재조립 격자)
#[derive(Debug, Serialize, ToSchema)]
pub struct GuideItemRes {
    /// "block" | "table"
    pub kind: String,
    /// 스트림 정렬 키 (표는 첫 셀의 block_seq)
    pub block_seq: i32,
    /// 문장 귀속 (NULL=단원 레벨)
    pub sentence_no: Option<i32>,
    // -- kind="block" --
    pub block_type: Option<String>,
    pub text: Option<String>,
    pub text_ko: Option<String>,
    pub marker: Option<String>,
    // -- kind="table" --
    pub table_no: Option<i32>,
    /// 행 우선 격자 (row_no, col_no 순) — col_span/row_span 으로 병합 표현
    pub rows: Option<Vec<Vec<GuideCellRes>>>,
}

/// 문장 학습항목 (복습 4종·쓰기 채점의 데이터 원천)
#[derive(Debug, Serialize, ToSchema)]
pub struct GuideSentenceRes {
    /// 전역 문장 번호 (1~500)
    pub sentence_no: i32,
    /// 한국어 정답 (채점 기준)
    pub text_ko: Option<String>,
    /// 표시 언어 해소 원문 ("N) ..." 접두 포함 — 표시는 프론트 가공)
    pub text: Option<String>,
    pub pron_ko: Option<String>,
    pub audio_url: Option<String>,
}

/// 단원 상세 (학습 페이지 전체 데이터)
#[derive(Debug, Serialize, ToSchema)]
pub struct GuideDetailRes {
    pub guide_idx: String,
    pub guide_seq: i32,
    pub guide_category: String,
    pub guide_theme: String,
    pub sentence_start: Option<i32>,
    pub sentence_end: Option<i32>,
    pub title: Option<String>,
    pub title_ko: Option<String>,
    pub subtitle: Option<String>,
    pub subtitle_ko: Option<String>,
    /// 응답에 적용된 언어 라벨
    pub lang: String,
    /// block_seq 순 콘텐츠 스트림 (표 = 재조립 완료)
    pub items: Vec<GuideItemRes>,
    pub sentences: Vec<GuideSentenceRes>,
}
