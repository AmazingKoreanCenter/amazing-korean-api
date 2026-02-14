use crate::types::{UserAuth, UserGender};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// =====================================================================
// Request DTOs (요청)
// =====================================================================

/// 회원가입 요청
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")] // JSON 키 강제: snake_case
#[schema(example = json!({
    "email": "new_user_01@example.com",
    "password": "Password123!",
    "name": "Hong Gil Dong",
    "nickname": "hong",
    "language": "ko",
    "country": "KR",
    "birthday": "1990-01-01",
    "gender": "male",
    "terms_service": true,
    "terms_personal": true
}))]
pub struct SignupReq {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8, max = 72))]
    pub password: String,
    
    #[validate(length(min = 1, max = 50))]
    pub name: String,

    #[validate(length(min = 1, max = 100))]
    pub nickname: String,

    /// ISO 639-1 언어 코드 (예: "ko", "en")
    #[validate(length(min = 2, max = 2))]
    pub language: String,

    /// ISO 3166-1 alpha-2 국가 코드 (예: "KR", "US")
    #[validate(length(min = 2, max = 50))]
    pub country: String,

    #[schema(value_type = String, format = "date")]
    pub birthday: NaiveDate,

    pub gender: UserGender, // Enum: male, female, other

    pub terms_service: bool,
    pub terms_personal: bool,
}

/// 프로필 수정 요청 (PATCH)
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "nickname": "UpdatedNick",
    "language": "ko",
    "country": "KR",
    "birthday": "1990-12-25",
    "gender": "female"
}))]
pub struct ProfileUpdateReq {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 100))]
    pub nickname: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 50))]
    pub language: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 50))]
    pub country: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, format = "date")]
    pub birthday: Option<NaiveDate>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<UserGender>,
}

/// 환경설정 수정 요청
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "user_set_language": "ko",
    "user_set_timezone": "UTC",
    "user_set_note_email": true,
    "user_set_note_push": false
}))]
pub struct SettingsUpdateReq {
    /// 앱 표시 언어 (21개 언어: ko, en, ja, zh-CN, zh-TW, vi 등)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 2, max = 5))]
    pub user_set_language: Option<String>,

    /// 타임존 (예: "Asia/Seoul")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))] 
    pub user_set_timezone: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_set_note_email: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_set_note_push: Option<bool>,
}


// =====================================================================
// Response DTOs (응답)
// =====================================================================

/// 회원가입 완료 응답 (이메일 인증 필요)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct SignupRes {
    pub message: String,
    pub requires_verification: bool,
}

/// 사용자 프로필 정보
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProfileRes {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub nickname: Option<String>,

    /// 학습 모국어 (Native Language)
    pub language: Option<String>,
    pub country: Option<String>,

    #[schema(value_type = String, format = "date")]
    pub birthday: Option<String>,
    pub gender: UserGender,

    pub user_state: bool,
    pub user_auth: UserAuth,

    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,

    /// 비밀번호 설정 여부 (OAuth 전용 계정은 false)
    pub has_password: bool,

    /// MFA (2단계 인증) 활성화 여부
    pub mfa_enabled: bool,
}

/// 사용자 환경설정 정보
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SettingsRes {
    /// 앱 UI 표시 언어
    pub user_set_language: String,
    pub user_set_timezone: String,
    pub user_set_note_email: bool,
    pub user_set_note_push: bool,
    
    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
}

// =====================================================================
// 향후 추가할 내용
// =====================================================================

/*

// 학습 언어 선택 dto
#[derive(
    Serialize,
    Deserialize,
    Validate,
    ToSchema,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    sqlx::FromRow,
)]
pub struct StudyLangItem {
    #[validate(length(min = 2, max = 2))] // ISO 639-1
    pub lang_code: String,
    #[validate(range(min = 1))]
    pub priority: i32,
    pub is_primary: bool,
}

*/