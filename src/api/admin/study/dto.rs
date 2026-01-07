use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};
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

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct StudyCreateReq {
    #[validate(custom(function = "validate_study_idx"))]
    pub study_idx: String,
    pub study_title: Option<String>,
    pub study_subtitle: Option<String>,
    pub study_description: Option<String>,
    pub study_program: Option<StudyProgram>,
    pub study_state: Option<StudyState>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct StudyUpdateReq {
    // [수정] Option 내부의 String을 검증하므로 함수 시그니처 수정 필요
    #[validate(custom(function = "validate_optional_study_idx"))]
    pub study_idx: Option<String>,

    pub study_state: Option<StudyState>,
    pub study_program: Option<StudyProgram>,

    #[validate(length(min = 1, max = 80))]
    pub study_title: Option<String>,
    
    #[validate(length(max = 120))]
    pub study_subtitle: Option<String>,
    
    pub study_description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct StudyBulkCreateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<StudyCreateReq>,
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

#[derive(Serialize, ToSchema)]
pub struct StudyBulkResult {
    pub id: Option<i64>,
    pub idx: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct StudyBulkCreateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<StudyBulkResult>,
}

fn validate_study_idx(value: &str) -> Result<(), validator::ValidationError> {
    let trimmed = value.trim();
    if trimmed.len() < 2 {
        return Err(validator::ValidationError::new("invalid_study_idx"));
    }
    Ok(())
}

// [수정] 인자 타입을 &Option<String> -> &String으로 변경
// validator는 값이 Some일 때만 이 함수를 호출하며, 내부 값을 전달합니다.
fn validate_optional_study_idx(value: &String) -> Result<(), ValidationError> {
    if value.len() < 2 {
        return Err(ValidationError::new("length_too_short"));
    }
    // 필요한 추가 검증 로직(예: 공백 체크 등)이 있다면 여기에 작성
    Ok(())
}
