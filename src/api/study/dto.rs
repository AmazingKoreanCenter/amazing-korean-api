use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

use crate::types::StudyProgram;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct StudyListReq {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<u64>,
    pub program: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct StudyListItem {
    pub study_id: i64,
    pub study_idx: String,
    pub study_program: StudyProgram,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StudyListMeta {
    pub page: u64,
    pub per_page: u64,
    pub total: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StudyListRes {
    pub data: Vec<StudyListItem>,
    pub meta: StudyListMeta,
}
