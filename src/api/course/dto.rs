use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::api::admin::translation::dto::TranslationMeta;
use crate::types::SupportedLanguage;

/// 코스 목록 조회 필터
#[derive(Debug, Deserialize, IntoParams)]
pub struct CourseListQuery {
    /// 번역 언어 (없으면 한국어 원본)
    pub lang: Option<SupportedLanguage>,
}

// SQL 결과를 이 구조체로 바로 매핑할 거라 FromRow 파생
#[derive(Serialize, sqlx::FromRow, ToSchema)]
pub struct CourseListItem {
    pub course_id: i64,
    pub course_title: String,
    pub course_subtitle: Option<String>,
    pub course_price: i32,
    pub course_type: String,
    pub course_state: String,
}

/// 코스 목록 응답 (Q1c A — translation_meta 포함 래퍼)
#[derive(Serialize, ToSchema)]
pub struct CourseListRes {
    pub items: Vec<CourseListItem>,
    /// 번역 메타 (Q1c A) — 번역 적용 범위 + 실제 반환 언어
    pub translation_meta: TranslationMeta,
}

/// 코스 상세 응답 (Q1c A — translation_meta 포함 래퍼)
#[derive(Serialize, ToSchema)]
pub struct CourseDetailRes {
    pub course: CourseListItem,
    /// 번역 메타 (Q1c A)
    pub translation_meta: TranslationMeta,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateCourseReq {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub price: i32,
    pub course_type: String,
    pub subtitle: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct CreateCourseRes {
    pub course_id: i64,
}
