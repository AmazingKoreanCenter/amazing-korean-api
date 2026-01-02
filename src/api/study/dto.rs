use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

use crate::types::{StudyProgram, StudyTaskKind};

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

#[derive(Debug, Serialize, ToSchema)]
pub struct StudyTaskDetailRes {
    pub task_id: i64,
    pub study_id: i64,
    pub kind: StudyTaskKind,
    pub seq: i32,
    pub question: Option<String>,
    pub media_url: Option<String>,
    pub payload: TaskPayload,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(untagged)]
pub enum TaskPayload {
    Choice(ChoicePayload),
    Typing(TypingPayload),
    Voice(VoicePayload),
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChoicePayload {
    pub choice_1: String,
    pub choice_2: String,
    pub choice_3: String,
    pub choice_4: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TypingPayload {
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VoicePayload {
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubmitAnswerReq {
    Choice { pick: i32 },
    Typing { text: String },
    Voice { audio_url: String },
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubmitAnswerRes {
    pub task_id: i64,
    pub is_correct: bool,
    pub score: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_answer: Option<String>,
}
