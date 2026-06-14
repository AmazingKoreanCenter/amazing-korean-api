//! guide admin 편집 DTO

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::types::SupportedLanguage;

// ── 단원 목록/상세 (편집용 — 모든 state) ──────────────────────────

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct AdminGuideSummary {
    pub guide_id: i64,
    pub guide_idx: String,
    pub guide_seq: i32,
    pub guide_state: String,
    pub guide_category: String,
    pub guide_theme: String,
    pub sentence_start: Option<i32>,
    pub sentence_end: Option<i32>,
    pub title_ko: Option<String>,
    pub title_en: Option<String>,
    /// 단원 내 stale 번역 블록 수 (집계, lang 무관 — source_version 불일치)
    pub stale_count: i64,
    pub block_count: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminGuideListRes {
    pub items: Vec<AdminGuideSummary>,
}

/// 편집용 블록 (재조립 안 함 — 원본 셀·좌표·source_version 노출)
#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct AdminGuideBlock {
    pub guide_block_id: i64,
    pub block_seq: i32,
    pub block_type: String,
    pub sentence_no: Option<i32>,
    pub text_ko: Option<String>,
    pub text_en: Option<String>,
    pub marker: Option<String>,
    pub table_no: Option<i32>,
    pub row_no: Option<i32>,
    pub col_no: Option<i32>,
    pub col_span: Option<i32>,
    pub row_span: Option<i32>,
    pub source_version: i32,
    pub legacy_key: Option<String>,
    pub edited: bool,
}

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct AdminGuideSentence {
    pub guide_sentence_id: i64,
    pub sentence_no: i32,
    pub pron_ko: Option<String>,
    pub speech_level: Option<String>,
    pub subject_honorific: Option<bool>,
    pub audio_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminGuideDetailRes {
    pub guide_id: i64,
    pub guide_idx: String,
    pub guide_seq: i32,
    pub guide_state: String,
    pub guide_category: String,
    pub guide_theme: String,
    pub sentence_start: Option<i32>,
    pub sentence_end: Option<i32>,
    pub title_ko: Option<String>,
    pub title_en: Option<String>,
    pub subtitle_ko: Option<String>,
    pub subtitle_en: Option<String>,
    pub blocks: Vec<AdminGuideBlock>,
    pub sentences: Vec<AdminGuideSentence>,
}

// ── 편집 요청 ─────────────────────────────────────────────────────

/// 단원 메타 수정 (공개 flip·테마·제목)
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct GuideMetaUpdateReq {
    /// 'ready' | 'open' | 'close' (공개 flip)
    pub guide_state: Option<String>,
    /// 교재 10색 테마
    pub guide_theme: Option<String>,
    #[validate(length(max = 200))]
    pub title_ko: Option<String>,
    #[validate(length(max = 200))]
    pub title_en: Option<String>,
    #[validate(length(max = 300))]
    pub subtitle_ko: Option<String>,
    #[validate(length(max = 300))]
    pub subtitle_en: Option<String>,
}

/// 블록 텍스트 수정 — text_ko/text_en 변경 시 source_version++ (번역 stale)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct GuideBlockUpdateReq {
    /// null 명시 = 비움, 미포함 = 변경 안 함 (Option<Option>)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_ko: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_en: Option<Option<String>>,
}

/// 문장 메타 수정 (발음형·화계·오디오)
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct GuideSentenceUpdateReq {
    pub pron_ko: Option<String>,
    #[validate(length(max = 30))]
    pub speech_level: Option<String>,
    pub subject_honorific: Option<bool>,
    pub audio_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminOkRes {
    pub ok: bool,
    pub message: String,
}

// ── stale 대시보드 + 디프 export ──────────────────────────────────

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StaleReq {
    /// 대상 언어 (없으면 전 언어 집계)
    pub lang: Option<SupportedLanguage>,
}

#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct StaleSummaryRow {
    pub lang: String,
    /// 번역이 원문보다 옛 버전(source_version 불일치)인 블록 수
    pub stale_count: i64,
    /// 번역이 아직 없는 블록 수 (원문 있는데 해당 lang 미적재)
    pub missing_count: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StaleDashboardRes {
    pub rows: Vec<StaleSummaryRow>,
}

/// 디프 export 항목 — 맥미니 재번역 입력 (id + 현재 원문)
#[derive(Debug, Serialize, ToSchema, sqlx::FromRow)]
pub struct DiffExportItem {
    /// guidev2:NN_MMM (legacy_key) 또는 db:{block_id}(신규 편집 블록)
    pub id: String,
    pub guide_block_id: i64,
    pub source_text: String,
    pub source_version: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DiffExportRes {
    pub lang: String,
    pub count: i64,
    pub items: Vec<DiffExportItem>,
}
