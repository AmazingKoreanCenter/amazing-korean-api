use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthRes {
    pub ok: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VideoListReq {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<u64>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct VideoInfo {
    pub video_id: i64,
    pub video_url_vimeo: String,
    pub video_state: String,
    pub video_access: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoListMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: u64,
    pub per_page: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoListRes {
    pub meta: VideoListMeta,
    pub data: Vec<VideoInfo>,
}

// 영상 목록/검색 API의 쿼리 파라미터 표준 묶음 개발시에 사용

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct VideosQuery {
    pub q: Option<String>,
    pub tag: Option<Vec<String>>,
    pub lang: Option<String>,
    pub access: Option<String>,
    pub state: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
    pub sort: Option<String>,
    pub order: Option<String>,
}

fn default_limit() -> i64 {
    20
}
fn default_offset() -> i64 {
    0
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct VideoListItem {
    pub video_id: i64,
    pub video_idx: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub duration_seconds: Option<i32>,
    pub language: Option<String>,
    pub thumbnail_url: Option<String>,
    pub state: String,
    pub access: String,
    pub tags: Vec<String>,
    pub has_captions: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct IdParam {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VideoTagDetail {
    pub key: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct VideoDetailRes {
    pub video_id: i64,
    pub video_url_vimeo: String,
    pub video_state: String,
    #[schema(value_type = Vec<VideoTagDetail>)]
    pub tags: Json<Vec<VideoTagDetail>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct CaptionItem {
    pub caption_id: i64,
    pub lang_code: Option<String>,
    pub label: Option<String>,
    pub kind: String,
    pub is_default: bool,
    pub is_active: bool,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct VideoProgressRes {
    pub video_id: i64,
    #[sqlx(rename = "video_progress_log")]
    pub progress_rate: i32,
    #[sqlx(rename = "video_completed_log")]
    pub is_completed: bool,
    #[sqlx(rename = "video_last_watched_at_log")]
    pub last_watched_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct VideoProgressUpdateReq {
    #[validate(range(min = 0, max = 100))]
    pub progress_rate: i32,
}
