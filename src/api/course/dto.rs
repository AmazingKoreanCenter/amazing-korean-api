use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

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

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateCourseReq {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub price: i32,
    pub course_type: String,
    pub subtitle: Option<String>,
}
