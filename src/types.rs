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

/// 사용자 언어 설정 (프로필용) — 21개 언어 (아랍어 제외)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_language_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserLanguage {
    Ko,
    En,
    Ja,
    #[sqlx(rename = "zh_cn")]
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[sqlx(rename = "zh_tw")]
    #[serde(rename = "zh-TW")]
    ZhTw,
    Vi,
    Th,
    Id,
    My,
    Mn,
    Ru,
    Es,
    Pt,
    Fr,
    De,
    Hi,
    Ne,
    Si,
    Km,
    Uz,
    Kk,
    Tg,
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

/// 사용자 UI/설정 언어 — 21개 언어 (아랍어 제외)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_set_language_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserSetLanguage {
    Ko,
    En,
    Ja,
    #[sqlx(rename = "zh_cn")]
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[sqlx(rename = "zh_tw")]
    #[serde(rename = "zh-TW")]
    ZhTw,
    Vi,
    Th,
    Id,
    My,
    Mn,
    Ru,
    Es,
    Pt,
    Fr,
    De,
    Hi,
    Ne,
    Si,
    Km,
    Uz,
    Kk,
    Tg,
}

impl UserSetLanguage {
    /// 프론트엔드 코드("zh-CN") → DB enum 문자열("zh_cn")로 변환
    pub fn frontend_to_db(code: &str) -> String {
        code.to_lowercase().replace('-', "_")
    }

    /// DB enum 문자열("zh_cn") → 프론트엔드 코드("zh-CN")로 변환
    pub fn db_to_frontend(db_val: &str) -> String {
        match db_val {
            "zh_cn" => "zh-CN".to_string(),
            "zh_tw" => "zh-TW".to_string(),
            other => other.to_string(),
        }
    }
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
// 4. Translation & i18n Enums
// -----------------------------------------------------------------------------

/// 번역 대상 콘텐츠 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "content_type_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Course,
    Lesson,
    Video,
    VideoTag,
    Study,
    StudyTaskChoice,
    StudyTaskTyping,
    StudyTaskVoice,
    StudyTaskExplain,
}

/// 번역 상태 (draft → reviewed → approved)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "translation_status_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TranslationStatus {
    Draft,
    Reviewed,
    Approved,
}

/// 번역 지원 언어 — 21개 (아랍어 제외)
/// content_translations 테이블 전용 (user 테이블과 독립적으로 확장 가능)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "supported_language_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SupportedLanguage {
    Ko,
    En,
    Ja,
    #[sqlx(rename = "zh_cn")]
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[sqlx(rename = "zh_tw")]
    #[serde(rename = "zh-TW")]
    ZhTw,
    Vi,
    Th,
    Id,
    My,
    Mn,
    Ru,
    Es,
    Pt,
    Fr,
    De,
    Hi,
    Ne,
    Si,
    Km,
    Uz,
    Kk,
    Tg,
}

impl SupportedLanguage {
    /// Google Cloud Translation API용 언어 코드 반환
    pub fn to_gcp_code(&self) -> &'static str {
        match self {
            Self::Ko => "ko",
            Self::En => "en",
            Self::Ja => "ja",
            Self::ZhCn => "zh-CN",
            Self::ZhTw => "zh-TW",
            Self::Vi => "vi",
            Self::Th => "th",
            Self::Id => "id",
            Self::My => "my",
            Self::Mn => "mn",
            Self::Ru => "ru",
            Self::Es => "es",
            Self::Pt => "pt",
            Self::Fr => "fr",
            Self::De => "de",
            Self::Hi => "hi",
            Self::Ne => "ne",
            Self::Si => "si",
            Self::Km => "km",
            Self::Uz => "uz",
            Self::Kk => "kk",
            Self::Tg => "tg",
        }
    }
}

// -----------------------------------------------------------------------------
// 5. Content (Video, Study, Lesson) Enums
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

/// 학습(Study) 접근 권한
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "study_access_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum StudyAccess {
    Public,
    Paid,
    Private,
    Promote,
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

/// 강의(Lesson) 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "lesson_state_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LessonState {
    Ready,
    Open,
    Close,
}

/// 강의(Lesson) 접근 권한
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "lesson_access_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LessonAccess {
    Public,
    Paid,
    Private,
    Promote,
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
// 6. Payment & Subscription Enums
// -----------------------------------------------------------------------------

/// 결제 제공자
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "payment_provider_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PaymentProvider {
    Paddle,
}

/// 구독 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "subscription_status_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Trialing,
    Active,
    PastDue,
    Paused,
    Canceled,
}

/// 결제 트랜잭션 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "transaction_status_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Completed,
    Refunded,
    PartiallyRefunded,
}

/// 구독 주기
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "billing_interval_enum")]
pub enum BillingInterval {
    #[sqlx(rename = "month_1")]
    #[serde(rename = "month_1")]
    Month1,
    #[sqlx(rename = "month_3")]
    #[serde(rename = "month_3")]
    Month3,
    #[sqlx(rename = "month_6")]
    #[serde(rename = "month_6")]
    Month6,
    #[sqlx(rename = "month_12")]
    #[serde(rename = "month_12")]
    Month12,
}

impl BillingInterval {
    /// 개월 수 반환
    pub fn months(&self) -> i32 {
        match self {
            Self::Month1 => 1,
            Self::Month3 => 3,
            Self::Month6 => 6,
            Self::Month12 => 12,
        }
    }

    /// 센트 단위 가격 반환
    pub fn price_cents(&self) -> i32 {
        match self {
            Self::Month1 => 1000,   // $10.00
            Self::Month3 => 2500,   // $25.00
            Self::Month6 => 5000,   // $50.00
            Self::Month12 => 10000, // $100.00
        }
    }
}

impl fmt::Display for BillingInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Month1 => write!(f, "month_1"),
            Self::Month3 => write!(f, "month_3"),
            Self::Month6 => write!(f, "month_6"),
            Self::Month12 => write!(f, "month_12"),
        }
    }
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