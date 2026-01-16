use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

use crate::types::{StudyProgram, StudyTaskKind};

// =========================================================================
// Request DTOs (요청)
// =========================================================================

/// 학습 목록 조회 요청 (Query String)
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListReq {
    #[serde(default = "default_page")]
    #[validate(range(min = 1))]
    pub page: u64,

    #[serde(default = "default_per_page")]
    #[validate(range(min = 1, max = 100))]
    pub per_page: u64,

    pub program: Option<String>,
    pub sort: Option<String>,
}

fn default_page() -> u64 { 1 }
fn default_per_page() -> u64 { 20 }

/// 정답 제출 요청 (JSON Body)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubmitAnswerReq {
    Choice { pick: i32 },
    Typing { text: String },
    Voice {
        #[allow(dead_code)] // TODO: 추후 AI 음성 분석 로직 구현 시 사용 예정
        audio_url: String,
    },
}

// =========================================================================
// Response DTOs (응답)
// =========================================================================

// --- 1. List Response ---

/// 학습 목록 아이템 (DB Row)
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListItem {
    pub study_id: i64,
    pub study_idx: String,
    pub study_program: StudyProgram,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 학습 목록 메타데이터
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: u64,
    pub per_page: u64,
}

/// 학습 목록 전체 응답
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListRes {
    pub meta: StudyListMeta,
    pub data: Vec<StudyListItem>,
}

// --- 2. Detail & Task Response ---

/// 학습 문제 상세 정보 (Payload 포함)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyTaskDetailRes {
    pub task_id: i64,
    pub study_id: i64,
    pub kind: StudyTaskKind,
    pub seq: i32,
    pub question: Option<String>,
    pub media_url: Option<String>,
    pub payload: TaskPayload,
}

/// 문제 유형별 페이로드
#[derive(Debug, Serialize, ToSchema)]
#[serde(untagged)]
pub enum TaskPayload {
    Choice(ChoicePayload),
    Typing(TypingPayload),
    Voice(VoicePayload),
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChoicePayload {
    pub choice_1: String,
    pub choice_2: String,
    pub choice_3: String,
    pub choice_4: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct TypingPayload {
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VoicePayload {
    pub image_url: Option<String>,
}

// --- 3. Action Response (Answer, Status, Explain) ---

/// 정답 제출 결과
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct SubmitAnswerRes {
    pub task_id: i64,
    pub is_correct: bool,
    pub score: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_answer: Option<String>,
}

/// 문제 풀이 상태 조회
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct TaskStatusRes {
    pub task_id: i64,
    pub attempts: i64,
    pub is_solved: bool,
    pub last_score: Option<i32>,
}

/// 문제 해설 조회
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct TaskExplainRes {
    pub task_id: i64,
    pub correct_answer: String,
    pub explanation_text: Option<String>,
    pub explanation_media_url: Option<String>,
    pub related_video_url: Option<String>,
}