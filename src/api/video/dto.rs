use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthRes {
    pub ok: bool,
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

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct VideoDetail {
    pub video_id: i64,
    pub video_idx: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub state: String,
    pub access: String,
    pub duration_seconds: Option<i32>,
    pub language: Option<String>,
    pub thumbnail_url: Option<String>,
    pub vimeo_video_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub has_captions: Option<bool>,
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
    pub user_id: i64,
    pub last_position_seconds: Option<i32>,
    pub total_duration_seconds: Option<i32>,
    pub progress: Option<i32>,
    pub completed: bool,
    pub last_watched_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VideoProgressUpdateReq {
    pub last_position_seconds: i32,
    pub total_duration_seconds: Option<i32>,
    pub progress: Option<i32>,
    pub completed: Option<bool>,
}
