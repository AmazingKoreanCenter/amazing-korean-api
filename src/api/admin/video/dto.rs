use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Validate)]
#[derive(ToSchema)]
pub struct VideoCreateReq {
    #[validate(length(min = 1, max = 200))]
    pub video_title: String,
    #[validate(length(max = 500))]
    pub video_subtitle: Option<String>,
    #[validate(length(min = 2, max = 10))]
    pub video_language: Option<String>,
    #[validate(custom(function = "validate_video_state"))]
    pub video_state: String, // ready|open|close
    #[validate(custom(function = "validate_video_access"))]
    pub video_access: String, // public|paid|private
    pub vimeo_video_id: Option<String>,
    #[validate(range(min = 1))]
    pub video_duration_seconds: Option<i32>,
    #[validate(url)]
    pub video_thumbnail_url: Option<String>,
    #[validate(url)]
    pub video_link: Option<String>,
}

fn validate_video_state(state: &str) -> Result<(), ValidationError> {
    if !["ready", "open", "close"].contains(&state) {
        return Err(ValidationError::new("invalid_video_state"));
    }
    Ok(())
}

fn validate_video_access(access: &str) -> Result<(), ValidationError> {
    if !["public", "paid", "private"].contains(&access) {
        return Err(ValidationError::new("invalid_video_access"));
    }
    Ok(())
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct VideoRes {
    pub video_id: i64,
    pub video_title: String,
    pub video_subtitle: Option<String>,
    pub video_language: Option<String>,
    pub video_state: String,
    pub video_access: String,
    pub vimeo_video_id: Option<String>,
    pub video_duration_seconds: Option<i32>,
    pub video_thumbnail_url: Option<String>,
    pub video_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
