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
    Ready,
    Open,
    Close,
}

impl VideoState {
    pub const fn as_str(&self) -> &'static str {
        match self {
            VideoState::Ready => "ready",
            VideoState::Open => "open",
            VideoState::Close => "close",
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
            "ready" => Ok(VideoState::Ready),
            "open" => Ok(VideoState::Open),
            "close" => Ok(VideoState::Close),
            _ => Err(format!("Invalid VideoState: {}", s)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum VideoAccess {
    Public,
    Paid,
    Private,
    Promote,
}

impl VideoAccess {
    pub const fn as_str(&self) -> &'static str {
        match self {
            VideoAccess::Public => "public",
            VideoAccess::Paid => "paid",
            VideoAccess::Private => "private",
            VideoAccess::Promote => "promote",
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
            "public" => Ok(VideoAccess::Public),
            "paid" => Ok(VideoAccess::Paid),
            "private" => Ok(VideoAccess::Private),
            "promote" => Ok(VideoAccess::Promote),
            _ => Err(format!("Invalid VideoAccess: {}", s)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct VideoCreateReq {
    // 1. video 테이블 컬럼
    #[validate(length(min = 1, max = 100))]
    #[schema(example = "V001")]
    pub video_idx: Option<String>,

    #[validate(custom(function = "validate_state"))]
    #[schema(example = "ready")]
    pub video_state: Option<String>, // 기본값: ready

    #[validate(custom(function = "validate_access"))]
    #[schema(example = "private")]
    pub video_access: String,

    // 2. video_tag 테이블 컬럼
    #[validate(length(min = 1, max = 200))]
    #[schema(example = "Rust 비동기 프로그래밍 기초")]
    pub video_tag_title: String,

    #[validate(length(max = 500))]
    #[schema(example = "Tokio와 Future에 대한 심층 강의입니다.")]
    pub video_tag_subtitle: Option<String>,

    #[validate(length(min = 1, max = 30))]
    #[schema(example = "tag_rust_basic")]
    pub video_tag_key: Option<String>,

    // 3. video URL
    #[validate(url, length(max = 1024))]
    #[schema(example = "https://vimeo.com/123456789")]
    pub video_url_vimeo: String,
}

// video_access 유효성 검증 함수
fn validate_access(access: &str) -> Result<(), validator::ValidationError> {
    match access {
        "public" | "paid" | "private" | "promote" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_video_access")),
    }
}

// video_state 유효성 검증 함수
fn validate_state(state: &str) -> Result<(), validator::ValidationError> {
    match state {
        "ready" | "open" | "close" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_video_state")),
    }
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
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

    #[validate(custom(function = "validate_state"))]
    pub video_state: Option<String>,

    #[validate(length(min = 1, max = 100), custom(function = "validate_not_empty_string"))]
    pub video_idx: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct VideoTagUpdateReq {
    #[validate(
        length(min = 1, max = 200),
        custom(function = "validate_not_empty_string")
    )]
    pub video_tag_title: Option<String>,

    #[validate(length(max = 500))]
    pub video_tag_subtitle: Option<String>,

    #[validate(length(min = 1, max = 30), custom(function = "validate_not_empty_string"))]
    pub video_tag_key: Option<String>,
}

impl From<VideoTagUpdateReq> for VideoUpdateReq {
    fn from(req: VideoTagUpdateReq) -> Self {
        Self {
            video_tag_title: req.video_tag_title,
            video_tag_subtitle: req.video_tag_subtitle,
            video_tag_key: req.video_tag_key,
            video_url_vimeo: None,
            video_access: None,
            video_state: None,
            video_idx: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct VideoTagBulkUpdateItem {
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
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct VideoTagBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<VideoTagBulkUpdateItem>,
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
    pub video_state: VideoState,
    pub video_access: VideoAccess,
    pub video_idx: String,
    pub video_tag_key: Option<String>,
    pub updated_by_user_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// 영상 길이 (초 단위, Vimeo API에서 가져옴)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i32>,
    /// 영상 썸네일 URL (Vimeo API에서 가져옴)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_thumbnail: Option<String>,
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

    #[validate(length(min = 1, max = 100), custom(function = "validate_not_empty_string"))]
    pub video_idx: Option<String>,

    #[validate(custom(function = "validate_state"))]
    pub video_state: Option<String>,

    #[validate(custom(function = "validate_access"))]
    pub video_access: Option<String>,

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
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct VideoBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<VideoBulkUpdateItem>,
}

/// Vimeo URL로 메타데이터 미리보기 요청
#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct VimeoPreviewReq {
    #[validate(url, length(min = 1, max = 1024))]
    pub url: String,
}

/// Vimeo 메타데이터 미리보기 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct VimeoPreviewRes {
    /// Vimeo Video ID (추출된 값)
    pub vimeo_video_id: String,
    /// 영상 제목 (Vimeo name)
    pub title: String,
    /// 영상 설명 (Vimeo description)
    pub description: Option<String>,
    /// 영상 길이 (초)
    pub duration: i32,
    /// 썸네일 URL
    pub thumbnail_url: Option<String>,
}

/// Vimeo 업로드 티켓 생성 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VimeoUploadTicketReq {
    /// 파일 이름
    #[validate(length(min = 1, max = 255))]
    pub file_name: String,
    /// 파일 크기 (bytes, 최대 5GB)
    #[validate(custom(function = "validate_file_size"))]
    pub file_size: i64,
}

// 파일 크기 유효성 검증 (1 ~ 5GB)
fn validate_file_size(size: i64) -> Result<(), ValidationError> {
    const MAX_SIZE: i64 = 5 * 1024 * 1024 * 1024; // 5GB
    if size < 1 {
        return Err(ValidationError::new("file_size_too_small"));
    }
    if size > MAX_SIZE {
        return Err(ValidationError::new("file_size_too_large"));
    }
    Ok(())
}

/// Vimeo 업로드 티켓 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct VimeoUploadTicketRes {
    /// Vimeo video URI (e.g., "/videos/123456789")
    pub video_uri: String,
    /// 추출된 Vimeo video ID
    pub vimeo_video_id: String,
    /// tus 업로드 엔드포인트
    pub upload_link: String,
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
