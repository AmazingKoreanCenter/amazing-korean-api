use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, IntoParams)]
#[schema(as = AdminLessonListReq)]
pub struct LessonListReq {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<u64>,
    pub q: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, FromRow)]
pub struct AdminLessonRes {
    pub lesson_id: i32,
    pub updated_by_user_id: i64,
    pub lesson_idx: String,
    pub lesson_title: String,
    pub lesson_subtitle: Option<String>,
    pub lesson_description: Option<String>,
    pub lesson_created_at: DateTime<Utc>,
    pub lesson_updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AdminLessonListRes {
    pub list: Vec<AdminLessonRes>,
    pub total: i64,
    pub page: u64,
    pub size: u64,
    pub total_pages: i64,
}
