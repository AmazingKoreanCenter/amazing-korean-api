use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct LessonListReq {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct LessonRes {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub lesson_idx: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LessonListMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LessonListRes {
    pub items: Vec<LessonRes>,
    pub meta: LessonListMeta,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct LessonDetailReq {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct LessonItemRes {
    pub seq: i32,
    pub kind: crate::types::LessonItemKind,
    pub video_id: Option<i64>,
    pub task_id: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LessonDetailRes {
    pub lesson_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub items: Vec<LessonItemRes>,
    pub meta: LessonListMeta,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct LessonItemsReq {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct LessonItemDetailRes {
    pub seq: i32,
    pub kind: crate::types::LessonItemKind,
    pub video_id: Option<i64>,
    pub study_task_id: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LessonItemsRes {
    pub items: Vec<LessonItemDetailRes>,
    pub meta: LessonListMeta,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct LessonProgressRes {
    pub percent: i32,
    pub last_seq: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LessonProgressUpdateReq {
    pub percent: i32,
    pub last_seq: Option<i32>,
}
