use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::fmt;
use std::str::FromStr;
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum VideoState {
    Draft,
    Ready,
    Hidden,
}

impl VideoState {
    pub const fn as_str(&self) -> &'static str {
        match self {
            VideoState::Draft => "draft",
            VideoState::Ready => "ready",
            VideoState::Hidden => "hidden",
        }
    }
}

impl fmt::Display for VideoState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for VideoState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(VideoState::Draft),
            "ready" => Ok(VideoState::Ready),
            "hidden" => Ok(VideoState::Hidden),
            _ => Err(format!("Invalid VideoState: {}", s)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum VideoAccess {
    Free,
    Paid,
    Private,
}

impl VideoAccess {
    pub const fn as_str(&self) -> &'static str {
        match self {
            VideoAccess::Free => "free",
            VideoAccess::Paid => "paid",
            VideoAccess::Private => "private",
        }
    }
}

impl fmt::Display for VideoAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for VideoAccess {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "free" => Ok(VideoAccess::Free),
            "paid" => Ok(VideoAccess::Paid),
            "private" => Ok(VideoAccess::Private),
            _ => Err(format!("Invalid VideoAccess: {}", s)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct VideoCreateReq {
    // 1. video_tag 테이블 컬럼
    #[validate(length(min = 1, max = 200))]
    #[schema(example = "Rust 비동기 프로그래밍 기초")]
    pub video_tag_title: String, // title -> video_tag_title

    #[validate(length(max = 500))]
    #[schema(example = "Tokio와 Future에 대한 심층 강의입니다.")]
    pub video_tag_subtitle: Option<String>, // description -> video_tag_subtitle

    #[validate(length(min = 1, max = 30))]
    #[schema(example = "tag_rust_basic")]
    pub video_tag_key: Option<String>, // tag_key -> video_tag_key

    // 2. video 테이블 컬럼
    #[validate(url, length(max = 1024))]
    #[schema(example = "https://vimeo.com/123456789")]
    pub video_url_vimeo: String, // url -> video_url_vimeo

    #[validate(custom(function = "validate_access"))] // "public" or "private" 검증 필요
    #[schema(example = "public")]
    pub video_access: String, // is_public -> video_access

    #[validate(length(min = 1, max = 100))]
    #[schema(example = "lesson_01_rust")]
    pub video_idx: Option<String>,
}

// video_access 유효성 검증 함수
fn validate_access(access: &str) -> Result<(), validator::ValidationError> {
    match access {
        "public" | "private" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_video_access")),
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VideoUpdateReq {
    #[validate(
        length(min = 1, max = 200),
        custom(function = "validate_not_empty_string")
    )]
    pub video_tag_title: Option<String>,

    #[validate(length(max = 500))]
    pub video_tag_subtitle: Option<String>,

    #[validate(length(min = 1, max = 30), custom(function = "validate_not_empty_string"))]
    pub video_tag_key: Option<String>,

    #[validate(url, length(max = 1024))]
    pub video_url_vimeo: Option<String>,

    #[validate(custom(function = "validate_access"))]
    pub video_access: Option<String>,

    #[validate(length(min = 1, max = 100), custom(function = "validate_not_empty_string"))]
    pub video_idx: Option<String>,
}

fn validate_not_empty_string(s: &str) -> Result<(), ValidationError> {
    if s.trim().is_empty() {
        return Err(ValidationError::new("empty_string"));
    }
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct VideoRes {
    pub video_id: i64,
    pub video_title: String,
    pub video_subtitle: Option<String>,
    pub video_language: Option<String>,
    pub video_state: VideoState,
    pub video_access: VideoAccess,
    pub vimeo_video_id: Option<String>,
    pub video_duration_seconds: Option<i32>,
    pub video_thumbnail_url: Option<String>,
    pub video_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub updated_by_user_id: i64,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct AdminVideoListReq {
    #[validate(range(min = 1))]
    pub page: Option<i64>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<i64>,
    pub q: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct AdminVideoRes {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub description: Option<String>,
    pub views: i64,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Pagination {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminVideoListRes {
    pub items: Vec<AdminVideoRes>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct VideoBulkCreateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<VideoCreateReq>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoBulkItemError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoBulkItemResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AdminVideoRes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<VideoBulkItemError>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoBulkSummary {
    pub total: i64,
    pub success: i64,
    pub failure: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoBulkCreateRes {
    pub summary: VideoBulkSummary,
    pub results: Vec<VideoBulkItemResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct VideoBulkUpdateItem {
    pub id: i64,

    #[validate(
        length(min = 1, max = 200),
        custom(function = "validate_not_empty_string")
    )]
    pub video_tag_title: Option<String>,

    #[validate(length(max = 500))]
    pub video_tag_subtitle: Option<String>,

    #[validate(length(min = 1, max = 30), custom(function = "validate_not_empty_string"))]
    pub video_tag_key: Option<String>,

    #[validate(url, length(max = 1024))]
    pub video_url_vimeo: Option<String>,

    #[validate(custom(function = "validate_access"))]
    pub video_access: Option<String>,

    #[validate(length(min = 1, max = 100), custom(function = "validate_not_empty_string"))]
    pub video_idx: Option<String>,
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct VideoBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<VideoBulkUpdateItem>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoBulkUpdateItemResult {
    pub id: i64,
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AdminVideoRes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<VideoBulkItemError>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoBulkUpdateRes {
    pub summary: VideoBulkSummary,
    pub results: Vec<VideoBulkUpdateItemResult>,
}
