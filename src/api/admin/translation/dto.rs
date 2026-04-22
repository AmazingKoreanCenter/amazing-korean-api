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
    /// 복수 content_type 필터 (쉼표 구분, content_type보다 우선)
    pub content_types: Option<String>,
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
// 번역 검색 (Translation Search — 재사용용)
// =============================================================================

/// 번역 검색 요청 (언어 + 상태 기반 최근 번역 조회)
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct TranslationSearchReq {
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
// 번역 통계 (Translation Stats)
// =============================================================================

/// 번역 통계 개별 항목 (content_type × lang × status 별 집계)
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct TranslationStatItem {
    pub content_type: ContentType,
    pub lang: SupportedLanguage,
    pub status: TranslationStatus,
    pub count: i64,
}

/// 번역 통계 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct TranslationStatsRes {
    pub items: Vec<TranslationStatItem>,
    pub total_translations: i64,
}

// =============================================================================
// 공용 번역 조회 (기존 도메인 API에서 사용)
// =============================================================================

/// 번역된 필드 정보 (fallback 여부 포함)
#[derive(Debug, Clone)]
pub struct TranslatedField {
    pub text: String,
    /// 실제 반환된 번역의 언어 (user_lang 일치 시 user_lang, en fallback 시 En, 원본 사용 시 Ko)
    pub actual_lang: crate::types::SupportedLanguage,
    pub fallback_used: bool,
}

impl TranslatedField {
    /// 번역 1건에 대해 user_lang 일치 / fallback 여부를 카운터에 집계.
    /// Q1c A 메타 계산 시 각 Consumer service 에서 사용.
    pub fn count_to(
        &self,
        user_lang: crate::types::SupportedLanguage,
        translated: &mut usize,
        fallback: &mut usize,
    ) {
        if self.actual_lang == user_lang {
            *translated += 1;
        } else {
            *fallback += 1;
        }
    }
}

/// Consumer `?lang=` 응답의 번역 메타 범위 — Q1c 결정 A (2026-04-21)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TranslationCoverage {
    /// `?lang=` 파라미터 없이 호출 (번역 조회 스킵)
    NotRequested,
    /// 요청 필드 전부 사용자 언어 번역 반환
    Full,
    /// 일부 필드는 사용자 언어, 일부는 fallback (en 또는 ko)
    Partial,
    /// 번역 데이터 없음 — 전부 원본 반환
    None,
}

/// Consumer `?lang=` 응답 루트에 포함되는 번역 메타 — Q1c 결정 A (2026-04-21)
#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct TranslationMeta {
    /// 실제로 반환된 번역 언어 (요청 lang 과 다를 수 있음 — fallback 발생 시)
    /// `None` 이면 `?lang=` 미요청 또는 번역 전무.
    pub translation_lang: Option<crate::types::SupportedLanguage>,

    /// 번역 범위
    pub translation_coverage: TranslationCoverage,
}

impl TranslationMeta {
    /// `?lang=` 요청이 없는 경우의 메타
    pub fn not_requested() -> Self {
        Self {
            translation_lang: None,
            translation_coverage: TranslationCoverage::NotRequested,
        }
    }

    /// `?lang=ko` 로 원본 언어 자체 요청 (원본이 ko 라 별도 번역 없음, 원본이 곧 번역)
    pub fn ko_full() -> Self {
        Self {
            translation_lang: Some(crate::types::SupportedLanguage::Ko),
            translation_coverage: TranslationCoverage::Full,
        }
    }

    /// 번역 조회 결과로부터 메타 계산
    ///
    /// - `requested_fields`: 이 응답이 번역을 시도하려 한 필드 수
    /// - `translated_fields`: 실제로 user_lang 으로 반환된 필드 수
    /// - `fallback_fields`: en 또는 ko 로 fallback 된 필드 수
    /// - `user_lang`: 요청된 언어
    pub fn from_counts(
        user_lang: crate::types::SupportedLanguage,
        requested_fields: usize,
        translated_fields: usize,
        fallback_fields: usize,
    ) -> Self {
        if requested_fields == 0 {
            // 번역 대상이 아예 없는 경우 (id 만 반환된 리소스 등)
            return Self {
                translation_lang: Some(user_lang),
                translation_coverage: TranslationCoverage::None,
            };
        }

        let covered = translated_fields + fallback_fields;
        let coverage = if covered == 0 {
            TranslationCoverage::None
        } else if translated_fields == requested_fields {
            TranslationCoverage::Full
        } else {
            TranslationCoverage::Partial
        };

        Self {
            translation_lang: Some(user_lang),
            translation_coverage: coverage,
        }
    }
}

impl Default for TranslationMeta {
    fn default() -> Self {
        Self::not_requested()
    }
}
