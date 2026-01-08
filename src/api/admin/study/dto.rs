use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};
use crate::types::{StudyProgram, StudyState, StudyTaskKind, UserSetLanguage};
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

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct StudyBulkUpdateItem {
    #[validate(range(min = 1))]
    pub id: i64,
    #[validate(custom(function = "validate_optional_study_idx"))]
    pub study_idx: Option<String>,
    pub study_title: Option<String>,
    pub study_subtitle: Option<String>,
    pub study_description: Option<String>,
    pub study_program: Option<StudyProgram>,
    pub study_state: Option<StudyState>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct StudyBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<StudyBulkUpdateItem>,
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

#[derive(Serialize, ToSchema)]
pub struct StudyBulkUpdateResult {
    pub id: i64,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct StudyBulkUpdateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<StudyBulkUpdateResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct StudyTaskListReq {
    #[validate(range(min = 1))]
    pub study_id: i32,
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct TaskExplainListReq {
    #[validate(range(min = 1))]
    pub task_id: i32,
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskExplainCreateReq {
    pub explain_lang: UserSetLanguage,
    pub explain_title: Option<String>,
    pub explain_text: Option<String>,
    pub explain_media_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskExplainUpdateReq {
    pub explain_lang: UserSetLanguage,
    pub explain_title: Option<String>,
    pub explain_text: Option<String>,
    pub explain_media_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskExplainCreateItem {
    #[validate(range(min = 1))]
    pub study_task_id: i32,
    pub explain_lang: UserSetLanguage,
    pub explain_title: Option<String>,
    pub explain_text: Option<String>,
    pub explain_media_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskExplainBulkCreateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<TaskExplainCreateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskExplainBulkResult {
    pub study_task_id: i32,
    pub explain_lang: UserSetLanguage,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskExplainBulkCreateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<TaskExplainBulkResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskExplainUpdateItem {
    #[validate(range(min = 1))]
    pub study_task_id: i32,
    pub explain_lang: UserSetLanguage,
    pub explain_title: Option<String>,
    pub explain_text: Option<String>,
    pub explain_media_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskExplainBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<TaskExplainUpdateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskExplainBulkUpdateResult {
    pub study_task_id: i32,
    pub explain_lang: UserSetLanguage,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskExplainBulkUpdateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<TaskExplainBulkUpdateResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, FromRow)]
pub struct AdminTaskExplainRes {
    pub study_task_id: i64,
    pub explain_lang: UserSetLanguage,
    pub explain_title: Option<String>,
    pub explain_text: Option<String>,
    pub explain_media_url: Option<String>,
    pub explain_created_at: DateTime<Utc>,
    pub explain_updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AdminTaskExplainListRes {
    pub list: Vec<AdminTaskExplainRes>,
    pub total: i64,
    pub page: u64,
    pub size: u64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct TaskStatusListReq {
    #[validate(range(min = 1))]
    pub task_id: Option<i32>,
    #[validate(range(min = 1))]
    pub user_id: Option<i64>,
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, FromRow)]
pub struct AdminTaskStatusRes {
    pub study_task_id: i64,
    pub user_id: i64,
    pub study_task_status_try: i32,
    pub study_task_status_best: i32,
    pub study_task_status_completed: bool,
    pub study_task_status_last_answer: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AdminTaskStatusListRes {
    pub list: Vec<AdminTaskStatusRes>,
    pub total: i64,
    pub page: u64,
    pub size: u64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct TaskStatusUpdateReq {
    #[validate(range(min = 1))]
    pub user_id: i64,
    pub study_task_status_try: Option<i32>,
    pub study_task_status_best: Option<i32>,
    pub study_task_status_completed: Option<bool>,
    pub study_task_status_last_answer: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone, FromRow)]
pub struct AdminStudyTaskRes {
    pub study_task_id: i64,
    pub study_task_kind: StudyTaskKind,
    pub study_task_seq: i32,
    pub question: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct AdminStudyTaskListRes {
    pub list: Vec<AdminStudyTaskRes>,
    pub total: i64,
    pub page: u64,
    pub size: u64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct StudyTaskCreateReq {
    #[validate(range(min = 1))]
    pub study_id: i32,
    pub study_task_kind: StudyTaskKind,
    #[validate(range(min = 1))]
    pub study_task_seq: Option<i32>,
    pub question: Option<String>,
    pub answer: Option<String>,
    pub image_url: Option<String>,
    pub audio_url: Option<String>,
    pub choice_1: Option<String>,
    pub choice_2: Option<String>,
    pub choice_3: Option<String>,
    pub choice_4: Option<String>,
    pub choice_correct: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct StudyTaskBulkCreateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<StudyTaskCreateReq>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct StudyTaskBulkResult {
    pub task_id: Option<i64>,
    pub seq: i32,
    pub kind: StudyTaskKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct StudyTaskBulkCreateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<StudyTaskBulkResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct StudyTaskUpdateReq {
    pub study_task_seq: Option<i32>,
    pub question: Option<String>,
    pub answer: Option<String>,
    pub image_url: Option<String>,
    pub audio_url: Option<String>,
    pub choice_1: Option<String>,
    pub choice_2: Option<String>,
    pub choice_3: Option<String>,
    pub choice_4: Option<String>,
    pub choice_correct: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct StudyTaskUpdateItem {
    #[validate(range(min = 1))]
    pub study_task_id: i32,
    #[validate(range(min = 1))]
    pub study_task_seq: Option<i32>,
    pub question: Option<String>,
    pub answer: Option<String>,
    pub image_url: Option<String>,
    pub audio_url: Option<String>,
    pub choice_1: Option<String>,
    pub choice_2: Option<String>,
    pub choice_3: Option<String>,
    pub choice_4: Option<String>,
    pub choice_correct: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct StudyTaskBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<StudyTaskUpdateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct StudyTaskBulkUpdateResult {
    pub task_id: i64,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct StudyTaskBulkUpdateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<StudyTaskBulkUpdateResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone, FromRow)]
pub struct AdminStudyTaskDetailRes {
    pub study_task_id: i64,
    pub study_id: i64,
    pub study_task_kind: StudyTaskKind,
    pub study_task_seq: i32,
    pub question: Option<String>,
    pub answer: Option<String>,
    pub image_url: Option<String>,
    pub audio_url: Option<String>,
    pub choice_1: Option<String>,
    pub choice_2: Option<String>,
    pub choice_3: Option<String>,
    pub choice_4: Option<String>,
    pub choice_correct: Option<i32>,
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
