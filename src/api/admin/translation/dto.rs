use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::types::{ContentType, SupportedLanguage, TranslationStatus};

// =============================================================================
// Request DTOs
// =============================================================================

/// 번역 목록 조회 필터
#[derive(Debug, Deserialize, IntoParams, Validate, ToSchema)]
pub struct TranslationListReq {
    pub content_type: Option<ContentType>,
    pub content_id: Option<i64>,
    pub lang: Option<SupportedLanguage>,
    pub status: Option<TranslationStatus>,

    #[validate(range(min = 1))]
    pub page: Option<i64>,

    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<i64>,
}

/// 번역 생성 요청 (단건)
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct TranslationCreateReq {
    pub content_type: ContentType,
    pub content_id: i64,

    #[validate(length(min = 1, max = 100))]
    pub field_name: String,

    pub lang: SupportedLanguage,

    #[validate(length(min = 1))]
    pub translated_text: String,
}

/// 번역 벌크 생성 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TranslationBulkCreateReq {
    #[validate(length(min = 1, max = 200))]
    #[validate(nested)]
    pub items: Vec<TranslationCreateReq>,
}

/// 번역 수정 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TranslationUpdateReq {
    #[validate(length(min = 1))]
    pub translated_text: Option<String>,

    pub status: Option<TranslationStatus>,
}

/// 번역 상태 변경 요청
#[derive(Debug, Deserialize, ToSchema)]
pub struct TranslationStatusReq {
    pub status: TranslationStatus,
}

// =============================================================================
// Response DTOs
// =============================================================================

/// 번역 응답 (단건)
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct TranslationRes {
    pub translation_id: i64,
    pub content_type: ContentType,
    pub content_id: i64,
    pub field_name: String,
    pub lang: SupportedLanguage,
    pub translated_text: String,
    pub status: TranslationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 번역 목록 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct TranslationListRes {
    pub items: Vec<TranslationRes>,
    pub meta: TranslationListMeta,
}

/// 페이지네이션 메타
#[derive(Debug, Serialize, ToSchema)]
pub struct TranslationListMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub per_page: i64,
}

/// 벌크 생성 결과 (단건)
#[derive(Debug, Serialize, ToSchema)]
pub struct TranslationBulkItemResult {
    pub index: usize,
    pub success: bool,
    pub translation_id: Option<i64>,
    pub error: Option<String>,
}

/// 벌크 생성 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct TranslationBulkCreateRes {
    pub total: usize,
    pub success_count: usize,
    pub fail_count: usize,
    pub results: Vec<TranslationBulkItemResult>,
}

// =============================================================================
// 자동 번역 DTOs
// =============================================================================

/// 자동 번역 요청 (Google Cloud Translation 등을 통한 자동 초안 생성)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AutoTranslateReq {
    pub content_type: ContentType,
    pub content_id: i64,

    #[validate(length(min = 1, max = 100))]
    pub field_name: String,

    /// 원본 텍스트 (ko)
    #[validate(length(min = 1))]
    pub source_text: String,

    /// 번역 대상 언어 목록
    #[validate(length(min = 1, max = 20))]
    pub target_langs: Vec<SupportedLanguage>,
}

/// 자동 번역 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct AutoTranslateRes {
    pub total: usize,
    pub success_count: usize,
    pub results: Vec<AutoTranslateItemResult>,
}

/// 자동 번역 개별 결과
#[derive(Debug, Serialize, ToSchema)]
pub struct AutoTranslateItemResult {
    pub lang: SupportedLanguage,
    pub success: bool,
    pub translation_id: Option<i64>,
    pub translated_text: Option<String>,
    pub error: Option<String>,
}

// =============================================================================
// 콘텐츠 목록 조회 (Content Records)
// =============================================================================

/// 콘텐츠 목록 조회 요청
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ContentRecordsReq {
    pub content_type: ContentType,
}

/// 콘텐츠 목록 개별 항목
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct ContentRecordItem {
    pub id: i64,
    pub label: String,
    pub detail: Option<String>,
}

/// 콘텐츠 목록 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct ContentRecordsRes {
    pub items: Vec<ContentRecordItem>,
}

// =============================================================================
// 원본 텍스트 조회 (Source Fields)
// =============================================================================

/// 원본 필드 조회 요청
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct SourceFieldsReq {
    pub content_type: ContentType,
    pub content_id: i64,
}

/// 원본 필드 개별 항목
#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct SourceFieldItem {
    pub content_type: ContentType,
    pub content_id: i64,
    pub field_name: String,
    pub source_text: Option<String>,
}

/// 원본 필드 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct SourceFieldsRes {
    pub fields: Vec<SourceFieldItem>,
}

// =============================================================================
// 벌크 자동 번역 (Auto Translate Bulk)
// =============================================================================

/// 벌크 자동 번역 개별 항목
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AutoTranslateBulkItem {
    pub content_type: ContentType,
    pub content_id: i64,

    #[validate(length(min = 1, max = 100))]
    pub field_name: String,

    #[validate(length(min = 1))]
    pub source_text: String,
}

/// 벌크 자동 번역 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AutoTranslateBulkReq {
    #[validate(length(min = 1, max = 200))]
    #[validate(nested)]
    pub items: Vec<AutoTranslateBulkItem>,

    /// 번역 대상 언어 목록
    #[validate(length(min = 1, max = 21))]
    pub target_langs: Vec<SupportedLanguage>,
}

/// 벌크 자동 번역 개별 결과
#[derive(Debug, Serialize, ToSchema)]
pub struct AutoTranslateBulkItemResult {
    pub content_type: ContentType,
    pub content_id: i64,
    pub field_name: String,
    pub lang: SupportedLanguage,
    pub success: bool,
    pub translation_id: Option<i64>,
    pub translated_text: Option<String>,
    pub error: Option<String>,
}

/// 벌크 자동 번역 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct AutoTranslateBulkRes {
    pub total: usize,
    pub success_count: usize,
    pub fail_count: usize,
    pub results: Vec<AutoTranslateBulkItemResult>,
}

// =============================================================================
// 번역 검색 (Translation Search — 재사용용)
// =============================================================================

/// 번역 검색 요청 (동일 소스 텍스트 기존 번역 찾기)
#[derive(Debug, Deserialize, IntoParams, Validate, ToSchema)]
pub struct TranslationSearchReq {
    #[validate(length(min = 1))]
    pub source_text: String,
    pub lang: Option<SupportedLanguage>,
}

/// 번역 검색 결과 항목
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct TranslationSearchItem {
    pub translation_id: i64,
    pub content_type: ContentType,
    pub content_id: i64,
    pub field_name: String,
    pub lang: SupportedLanguage,
    pub translated_text: String,
    pub status: TranslationStatus,
}

/// 번역 검색 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct TranslationSearchRes {
    pub items: Vec<TranslationSearchItem>,
}

// =============================================================================
// 공용 번역 조회 (기존 도메인 API에서 사용)
// =============================================================================

/// 번역된 필드 정보 (fallback 여부 포함)
#[derive(Debug, Clone)]
pub struct TranslatedField {
    pub text: String,
    pub fallback_used: bool,
}
