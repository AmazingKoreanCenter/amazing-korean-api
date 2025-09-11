use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;
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

#[allow(dead_code)]
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VideoCreateReq {
    #[validate(length(min = 1, max = 200))]
    pub video_title: String,
    #[validate(length(max = 500))]
    pub video_subtitle: Option<String>,
    #[validate(length(min = 2, max = 10))]
    pub video_language: Option<String>,
    pub video_state: Option<VideoState>,
    pub video_access: Option<VideoAccess>,
    #[validate(length(max = 32))]
    pub vimeo_video_id: Option<String>,
    #[validate(range(min = 1))]
    pub video_duration_seconds: Option<i32>,
    #[validate(url, length(max = 1024))]
    pub video_thumbnail_url: Option<String>,
    #[validate(url, length(max = 1024))]
    pub video_link: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VideoUpdateReq {
    #[validate(
        length(min = 1, max = 200),
        custom(function = "validate_not_empty_string")
    )]
    pub video_title: Option<String>,
    #[validate(length(max = 500))]
    pub video_subtitle: Option<String>,
    #[validate(length(min = 2, max = 10))]
    pub video_language: Option<String>,
    pub video_state: Option<VideoState>,
    pub video_access: Option<VideoAccess>,
    #[validate(length(max = 32))]
    pub vimeo_video_id: Option<String>,
    #[validate(range(min = 1))]
    pub video_duration_seconds: Option<i32>,
    #[validate(url, length(max = 1024))]
    pub video_thumbnail_url: Option<String>,
    #[validate(url, length(max = 1024))]
    pub video_link: Option<String>,
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
