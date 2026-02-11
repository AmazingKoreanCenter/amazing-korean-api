use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};
use utoipa::ToSchema;
use validator::Validate;

use crate::types::SupportedLanguage;

// =====================================================================
// Request DTOs (요청)
// =====================================================================

/// 비디오 목록 조회 및 검색 요청 (Query String)
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "page": 1,
    "per_page": 20,
    "q": "Korean Alphabet",
    "sort": "latest",
    "state": "published"
}))]
pub struct VideoListReq {
    #[serde(default = "default_page")]
    #[validate(range(min = 1))]
    pub page: u64,

    #[serde(default = "default_per_page")]
    #[validate(range(min = 1, max = 100))]
    pub per_page: u64,

    // 검색 및 필터
    pub q: Option<String>,          // 검색어
    pub tag: Option<String>,        // 태그 필터
    pub state: Option<String>,      // 상태 필터 (published, etc)

    // 정렬 (latest, views, etc.)
    pub sort: Option<String>,

    /// 번역 언어 (없으면 한국어 원본)
    pub lang: Option<SupportedLanguage>,
}

fn default_page() -> u64 { 1 }
fn default_per_page() -> u64 { 20 }

/// 비디오 ID 파라미터 (Path Variable 등)
#[derive(Debug, Deserialize)]
pub struct IdParam {
    pub id: i64,
}

/// 학습 진도 업데이트 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "progress_rate": 80,
    "watch_duration_sec": 120
}))]
pub struct VideoProgressUpdateReq {
    #[validate(range(min = 0, max = 100))]
    pub progress_rate: i32,

    /// 이번 세션에서 시청한 시간 (초) - 누적됨
    #[validate(range(min = 0))]
    #[serde(default)]
    pub watch_duration_sec: i32,
}

// =====================================================================
// Response DTOs (응답)
// =====================================================================

// ----------------------
// 1. List Response
// ----------------------

/// 비디오 목록 아이템 (경량 정보)
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoListItem {
    pub video_id: i64,
    pub video_idx: String, // 비즈니스 식별 코드 (예: VID-001)
    
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub duration_seconds: Option<i32>,
    pub language: Option<String>,
    pub thumbnail_url: Option<String>,
    
    pub state: String,
    pub access: String,
    
    // 목록에서는 단순 문자열 태그 배열
    #[serde(default)] 
    pub tags: Vec<String>, 
    
    pub has_captions: bool,
    pub created_at: DateTime<Utc>,
}

/// 목록 페이징 메타데이터
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoListMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: u64,
    pub per_page: u64,
}

/// 비디오 목록 응답 (Data + Meta)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoListRes {
    pub meta: VideoListMeta,
    pub data: Vec<VideoListItem>,
}

// ----------------------
// 2. Detail Response
// ----------------------

/// 상세 태그 정보 (JSONB 구조)
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub struct VideoTagDetail {
    pub key: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
}

/// 비디오 상세 정보
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoDetailRes {
    pub video_id: i64,
    pub video_url_vimeo: String,
    pub video_state: String,
    
    // DB의 JSONB 타입을 Rust 구조체로 매핑
    // Swagger 문서에는 Vec<VideoTagDetail>로 표시되도록 설정
    #[schema(value_type = Vec<VideoTagDetail>)]
    pub tags: Json<Vec<VideoTagDetail>>,
    
    pub created_at: DateTime<Utc>,
}

// ----------------------
// 3. Progress Response
// ----------------------

/// 학습 진도 조회 응답
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VideoProgressRes {
    pub video_id: i64,

    #[sqlx(rename = "video_progress_log")]
    pub progress_rate: i32,

    #[sqlx(rename = "video_completed_log")]
    pub is_completed: bool,

    #[sqlx(rename = "video_last_watched_at_log")]
    pub last_watched_at: Option<DateTime<Utc>>,

    /// 총 누적 시청 시간 (초)
    #[sqlx(rename = "video_watch_duration_sec")]
    pub watch_duration_sec: i32,
}