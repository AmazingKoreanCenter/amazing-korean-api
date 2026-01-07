use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use crate::types::{StudyProgram, StudyState};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Deserialize, IntoParams, Validate, ToSchema)]
pub struct StudyListReq {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    
    #[validate(range(min = 1, max = 100))]
    pub size: Option<u64>, // per_page -> size 로 통일
    
    pub q: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
    
    // String 대신 Enum을 직접 사용하여 Axum이 자동 파싱하게 함
    pub study_state: Option<StudyState>,
    pub study_program: Option<StudyProgram>, 
}

#[derive(Serialize, ToSchema)]
pub struct AdminStudyListRes {
    pub list: Vec<AdminStudyRes>,
    pub total: i64,
    pub page: u64,
    pub size: u64,
    pub total_pages: i64,
}

#[derive(Serialize, FromRow, ToSchema)]
pub struct AdminStudyRes {
    pub study_id: i32,
    pub study_idx: String,
    pub study_title: Option<String>,
    pub study_subtitle: Option<String>,
    pub study_program: StudyProgram,
    pub study_state: StudyState,
    pub study_created_at: DateTime<Utc>,
    pub study_updated_at: DateTime<Utc>,
}