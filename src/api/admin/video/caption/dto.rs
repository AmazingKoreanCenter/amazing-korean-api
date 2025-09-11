use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum CaptionKind {
    Sub,
    Cc,
}

impl CaptionKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            CaptionKind::Sub => "sub",
            CaptionKind::Cc => "cc",
        }
    }
}

impl fmt::Display for CaptionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for CaptionKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sub" => Ok(CaptionKind::Sub),
            "cc" => Ok(CaptionKind::Cc),
            _ => Err(format!("Invalid CaptionKind: {}", s)),
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CaptionCreateReq {
    #[validate(length(min = 2, max = 8))]
    pub lang_code: String,
    pub kind: CaptionKind,
    #[validate(url, length(min = 1, max = 1024))]
    pub url: String,
    #[validate(length(max = 16))]
    pub format: Option<String>,
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Default)]
pub struct CaptionUpdateReq {
    #[validate(length(min = 2, max = 8))]
    pub lang_code: Option<String>,
    pub kind: Option<CaptionKind>,
    #[validate(url, length(min = 1, max = 1024))]
    pub url: Option<String>,
    #[validate(length(max = 16))]
    pub format: Option<String>,
    pub is_default: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct CaptionRes {
    pub caption_id: i64,
    pub video_id: i64,
    pub lang_code: String,
    pub kind: CaptionKind,
    pub url: String,
    pub format: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub updated_by_user_id: i64,
}
