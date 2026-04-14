use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use crate::types::{
    StudyProgram, StudyState, StudyTaskKind, SupportedLanguage, WritingLevel,
    WritingPracticeType,
};

// =========================================================================
// Request DTOs (요청)
// =========================================================================

/// 학습 목록 조회 요청 (Query String)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListReq {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub program: Option<String>,
    pub sort: Option<String>,
    /// 번역 언어 (없으면 한국어 원본)
    pub lang: Option<SupportedLanguage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudyListSort {
    Latest,
    Oldest,
    Alphabetical,
}

impl StudyListSort {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "latest" => Some(Self::Latest),
            "oldest" => Some(Self::Oldest),
            "alphabetical" => Some(Self::Alphabetical),
            _ => None,
        }
    }
}

/// 정답 제출 요청 (JSON Body)
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SubmitAnswerReq {
    Choice { pick: i32 },
    Typing { text: String },
    Voice { text: String },
    Writing { text: String, session_id: Option<i64> },
}

// =========================================================================
// Response DTOs (응답)
// =========================================================================

// --- 1. List Response ---

/// 학습 목록 아이템 (DB Row)
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudySummaryDto {
    pub study_id: i32,
    pub study_idx: String,
    pub program: StudyProgram,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub state: StudyState,
    pub created_at: DateTime<Utc>,
}

/// 학습 목록 메타데이터
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListMeta {
    pub page: u32,
    pub per_page: u32,
    pub total_count: i64,
    pub total_pages: u32,
}

/// 학습 목록 전체 응답
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyListResp {
    pub list: Vec<StudySummaryDto>,
    pub meta: StudyListMeta,
}

// --- 1-2. Study Detail Response ---

/// Study 상세 조회 요청 (Query String)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyDetailReq {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    /// 번역 언어 (없으면 한국어 원본)
    pub lang: Option<SupportedLanguage>,
}

/// Study 내 Task 요약 정보
#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyTaskSummaryDto {
    pub task_id: i32,
    pub kind: StudyTaskKind,
    pub seq: i32,
}

/// Study 상세 응답 (Study 정보 + Task 목록)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyDetailRes {
    pub study_id: i32,
    pub study_idx: String,
    pub program: StudyProgram,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub state: StudyState,
    pub tasks: Vec<StudyTaskSummaryDto>,
    pub meta: StudyListMeta,
}

// --- 2. Detail & Task Response ---

/// 학습 문제 상세 정보 (Payload 포함)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StudyTaskDetailRes {
    pub task_id: i32,
    pub study_id: i32,
    pub kind: StudyTaskKind,
    pub seq: i32,
    pub created_at: DateTime<Utc>,
    pub payload: TaskPayload,
}

/// 문제 유형별 페이로드
#[derive(Debug, Serialize, ToSchema)]
#[serde(untagged)]
pub enum TaskPayload {
    Choice(ChoicePayload),
    Typing(TypingPayload),
    Voice(VoicePayload),
    Writing(WritingPayload),
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChoicePayload {
    pub question: String,
    pub choice_1: String,
    pub choice_2: String,
    pub choice_3: String,
    pub choice_4: String,
    pub audio_url: Option<String>, // Added from STUDY_TASK_CHOICE schema
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct TypingPayload {
    pub question: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct VoicePayload {
    pub question: String,
    pub audio_url: Option<String>, // Added from STUDY_TASK_VOICE schema
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingPayload {
    pub prompt: String,
    /// 초급 레벨에서만 클라이언트에 전송 (실시간 피드백용)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    pub hint: Option<String>,
    pub level: WritingLevel,
    pub practice_type: WritingPracticeType,
    pub keyboard_visible: bool,
    pub image_url: Option<String>,
    pub audio_url: Option<String>,
}

// --- 3. Action Response (Answer, Status, Explain) ---

/// 정답 제출 결과
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct SubmitAnswerRes {
    pub is_correct: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

/// 문제 풀이 상태 조회
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct TaskStatusRes {
    pub try_count: i32,
    pub is_solved: bool,
    pub last_attempt_at: Option<DateTime<Utc>>,
}

/// 문제 해설 조회
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct TaskExplainRes {
    pub title: Option<String>,
    pub explanation: Option<String>,
    pub resources: Vec<String>,
}

// =========================================================================
// Writing Practice Session DTOs
// =========================================================================

/// 한글 자판 연습 세션 시작 요청
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct StartWritingSessionReq {
    /// 관리자 등록 태스크 기반 연습이면 study_task_id, 자유 연습이면 null
    pub study_task_id: Option<i32>,
    pub writing_level: WritingLevel,
    pub writing_practice_type: WritingPracticeType,
}

/// 세션 오류 기록 (JSONB 저장)
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingMistake {
    pub position: i32,
    pub expected: String,
    pub actual: String,
}

/// 한글 자판 연습 세션 완료 요청
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct FinishWritingSessionReq {
    pub total_chars: i32,
    pub correct_chars: i32,
    /// 클라이언트가 측정한 실제 타이핑 소요 시간 (ms). CPM 계산에 사용.
    pub duration_ms: i64,
    #[serde(default)]
    pub mistakes: Vec<WritingMistake>,
}

/// 세션 목록 조회 요청 (Query String)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingSessionListReq {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub level: Option<WritingLevel>,
    /// 완료된 세션만 포함 (기본 false)
    pub finished_only: Option<bool>,
}

/// 세션 응답
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingSessionRes {
    pub session_id: i64,
    pub user_id: i64,
    pub study_task_id: Option<i32>,
    pub writing_level: WritingLevel,
    pub writing_practice_type: WritingPracticeType,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub total_chars: i32,
    pub correct_chars: i32,
    pub accuracy_rate: f64,
    pub chars_per_minute: f64,
    pub mistakes: Vec<WritingMistake>,
}

/// 세션 목록 응답
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingSessionListRes {
    pub list: Vec<WritingSessionRes>,
    pub meta: StudyListMeta,
}

/// 통계 조회 요청 (Query String)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingStatsReq {
    /// 집계 기간 (일, 기본 30)
    pub days: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingLevelStat {
    pub writing_level: WritingLevel,
    pub sessions: i64,
    pub avg_accuracy: f64,
    pub avg_cpm: f64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingDailyStat {
    /// 날짜 (UTC, YYYY-MM-DD)
    pub day: String,
    pub sessions: i64,
    pub avg_accuracy: f64,
    pub avg_cpm: f64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingWeakChar {
    pub expected: String,
    pub miss_count: i64,
}

/// 통계 응답
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingStatsRes {
    pub total_sessions: i64,
    pub avg_accuracy: f64,
    pub avg_cpm: f64,
    pub level_breakdown: Vec<WritingLevelStat>,
    pub recent_trend: Vec<WritingDailyStat>,
    pub weak_chars: Vec<WritingWeakChar>,
}

// =========================================================================
// Writing Practice Seed DTOs (자유 연습 드릴 컨텐츠)
// =========================================================================

/// 자유 연습 시드 조회 요청 (Query String)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingPracticeSeedReq {
    pub level: WritingLevel,
    pub practice_type: WritingPracticeType,
    /// 최대 반환 개수 (기본 20, 최대 100)
    pub limit: Option<u32>,
}

/// 자유 연습 시드 아이템
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingPracticeSeedItem {
    pub seed_id: i64,
    pub seq: i32,
    pub prompt: String,
    pub answer: String,
    pub hint: Option<String>,
}

/// 자유 연습 시드 응답
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WritingPracticeSeedRes {
    pub level: WritingLevel,
    pub practice_type: WritingPracticeType,
    pub items: Vec<WritingPracticeSeedItem>,
}
