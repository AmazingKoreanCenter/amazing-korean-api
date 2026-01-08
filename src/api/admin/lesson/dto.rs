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

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct LessonCreateReq {
    #[validate(length(min = 1))]
    pub lesson_idx: String,
    #[validate(length(min = 1))]
    pub lesson_title: String,
    pub lesson_subtitle: Option<String>,
    pub lesson_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonCreateItem {
    #[validate(length(min = 1))]
    pub lesson_idx: String,
    #[validate(length(min = 1))]
    pub lesson_title: String,
    pub lesson_subtitle: Option<String>,
    pub lesson_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkCreateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<LessonCreateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkResult {
    pub lesson_id: Option<i32>,
    pub lesson_idx: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkCreateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<LessonBulkResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonUpdateItem {
    #[validate(range(min = 1))]
    pub lesson_id: i32,
    pub lesson_idx: Option<String>,
    pub lesson_title: Option<String>,
    pub lesson_subtitle: Option<String>,
    pub lesson_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<LessonUpdateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkUpdateResult {
    pub lesson_id: i32,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkUpdateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<LessonBulkUpdateResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonUpdateReq {
    pub lesson_idx: Option<String>,
    pub lesson_title: Option<String>,
    pub lesson_subtitle: Option<String>,
    pub lesson_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, IntoParams)]
pub struct LessonItemListReq {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<u64>,
    pub sort: Option<String>,
    pub order: Option<String>,
    #[validate(range(min = 1))]
    pub lesson_id: Option<i32>,
    pub lesson_item_kind: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemCreateReq {
    #[validate(range(min = 1))]
    pub lesson_item_seq: i32,
    #[validate(length(min = 1))]
    pub lesson_item_kind: String,
    #[validate(range(min = 1))]
    pub video_id: Option<i32>,
    #[validate(range(min = 1))]
    pub study_task_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemUpdateReq {
    #[validate(range(min = 1))]
    pub lesson_item_seq: Option<i32>,
    #[validate(length(min = 1))]
    pub lesson_item_kind: Option<String>,
    #[validate(range(min = 1))]
    pub video_id: Option<i32>,
    #[validate(range(min = 1))]
    pub study_task_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemCreateItem {
    #[validate(range(min = 1))]
    pub lesson_id: i32,
    #[validate(range(min = 1))]
    pub lesson_item_seq: i32,
    #[validate(length(min = 1))]
    pub lesson_item_kind: String,
    #[validate(range(min = 1))]
    pub video_id: Option<i32>,
    #[validate(range(min = 1))]
    pub study_task_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkCreateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<LessonItemCreateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkCreateResult {
    pub lesson_id: i32,
    pub lesson_item_seq: i32,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkCreateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<LessonItemBulkCreateResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, FromRow)]
pub struct AdminLessonItemRes {
    pub lesson_id: i32,
    pub lesson_item_seq: i32,
    pub lesson_item_kind: String,
    pub video_id: Option<i32>,
    pub study_task_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AdminLessonItemListRes {
    pub list: Vec<AdminLessonItemRes>,
    pub total: i64,
    pub page: u64,
    pub size: u64,
    pub total_pages: i64,
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
