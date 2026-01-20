use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use std::fmt;

// -----------------------------------------------------------------------------
// 1. User Related Enums
// -----------------------------------------------------------------------------

/// 사용자 권한 레벨
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_auth_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserAuth {
    #[sqlx(rename = "HYMN")] // DB에 대문자로 저장됨
    #[serde(rename = "HYMN")]
    Hymn,
    Admin,
    Manager,
    Learner,
}

/// 사용자 언어 설정 (프로필용)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_language_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserLanguage {
    Ko,
    En,
}

/// 사용자 성별
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_gender_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserGender {
    None,
    Male,
    Female,
    Other,
}

/// 사용자 로그 액션 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_action_log_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserActionLog {
    Signup,
    FindId,
    ResetPw,
    Update,
}

/// 사용자 UI/설정 언어
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_set_language_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserSetLanguage {
    Ko,
    En,
}

// -----------------------------------------------------------------------------
// 2. Login & Session Enums
// -----------------------------------------------------------------------------

/// 로그인 기기 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "login_device_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LoginDevice {
    Mobile,
    Tablet,
    Desktop,
    Other,
}

/// 로그인 방식
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "login_method_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LoginMethod {
    Email,
    Google,
    Apple,
}

/// 로그인 세션 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "login_state_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LoginState {
    Active,
    Revoked,
    Expired,
    LoggedOut,
    Compromised,
}

/// 로그인 로그 이벤트 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "login_event_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LoginEvent {
    Login,
    Logout,
    Refresh,
    Rotate,
    Fail,
    ReuseDetected,
}

// -----------------------------------------------------------------------------
// 3. Admin & Management Enums
// -----------------------------------------------------------------------------

/// 관리자 활동 로그 액션
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "admin_action_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AdminAction {
    Create,
    Update,
    Banned,
    Reorder,
    Publish,
    Unpublish,
}

// -----------------------------------------------------------------------------
// 4. Content (Video, Study, Lesson) Enums
// -----------------------------------------------------------------------------

/// 비디오 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "video_state_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum VideoState {
    Ready,
    Open,
    Close,
}

/// 비디오 접근 권한
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "video_access_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum VideoAccess {
    Public,
    Paid,
    Private,
    Promote,
}

/// 학습(Study) 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "study_state_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum StudyState {
    Ready,
    Open,
    Close,
}

/// 학습 프로그램 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "study_program_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum StudyProgram {
    BasicPronunciation,
    BasicWord,
    #[sqlx(rename = "basic_900")] // 숫자가 포함된 경우 명시적 rename 필요
    #[serde(rename = "basic_900")]
    Basic900,
    TopikRead,
    TopikListen,
    TopikWrite,
    Tbc,
}

/// 학습 과제(Task) 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "study_task_kind_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum StudyTaskKind {
    Choice,
    Typing,
    Voice,
}

/// 학습 과제 로그 액션
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "study_task_log_action_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum StudyTaskLogAction {
    View,
    Start,
    Answer,
    Finish,
    Explain,
    Status,
}

/// 강의(Lesson) 아이템 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "lesson_item_kind_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LessonItemKind {
    Video,
    Task,
}

// -----------------------------------------------------------------------------
// Helper Implementations (Display)
// -----------------------------------------------------------------------------

impl fmt::Display for UserAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserAuth::Hymn => write!(f, "HYMN"),
            UserAuth::Admin => write!(f, "admin"),
            UserAuth::Manager => write!(f, "manager"),
            UserAuth::Learner => write!(f, "learner"),
        }
    }
}

// 필요한 경우 다른 Enum들도 Display 구현 추가 가능