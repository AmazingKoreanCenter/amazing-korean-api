use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::api::auth::dto::AccessTokenRes;
use crate::types::{UserAuth, UserGender};

// 회원가입 요청 dto
#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "email": "new_user_01@example.com",
    "password": "Password123!",
    "name": "홍길동",
    "terms_service": true,
    "terms_personal": true,
    "nickname": "hong",
    "language": "ko",
    "country": "KR",
    "birthday": "1990-01-01",
    "gender": "male"
}))]
pub struct SignupReq {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 72))]
    pub password: String,
    #[validate(length(min = 1, max = 50))]
    pub name: String,

    pub terms_service: bool,
    pub terms_personal: bool,

    #[validate(length(min = 1, max = 100))]
    pub nickname: String,

    #[validate(length(min = 2, max = 2))]
    pub language: String,

    #[validate(length(min = 2, max = 50))]
    pub country: String,

    #[schema(value_type = String, format = "date")]
    #[serde(alias = "birth")]
    pub birthday: NaiveDate,

    pub gender: UserGender,
}

// 회원가입 응답 dto
#[allow(dead_code)]
#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "user_id": 123,
    "email": "new_user_01@example.com",
    "name": "홍길동",
    "nickname": "hong",
    "language": "ko",
    "country": "KR",
    "birthday": "1990-01-01",
    "gender": "male",
    "user_state": "on",
    "user_auth": "learner",
    "created_at": "2025-08-21T10:00:00Z",
    "access": {
        "access_token": "eyJ...",
        "expires_in": 3600
    },
    "session_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef"
}))]
pub struct SignupRes {
    pub user_id: i64,
    pub email: String,
    pub name: String,
    pub nickname: String,
    pub language: String,
    pub country: String,
    #[schema(value_type = String, format = "date")]
    pub birthday: NaiveDate,
    pub gender: UserGender,
    pub user_state: bool,
    pub user_auth: UserAuth,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    pub access: AccessTokenRes,
    pub session_id: String,
}

// 프로필 조회 dto
#[derive(Serialize, sqlx::FromRow, ToSchema)]
#[schema(example = json!({
    "id": 123,
    "email": "test@example.com",
    "name": "Test User",
    "nickname": "TestNick",
    "language": "en",
    "country": "US",
    "birthday": "2000-01-01",
    "gender": "male",
    "user_state": "on",
    "user_auth": "learner",
    "created_at": "2025-08-21T10:00:00Z"
}))]
pub struct ProfileRes {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub nickname: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    #[schema(value_type = String, format = "date")]
    pub birthday: Option<NaiveDate>,
    pub gender: UserGender, // DB에서 String으로 가져오므로
    pub user_state: bool,
    pub user_auth: UserAuth,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

// 프로필 수정 dto
#[derive(Serialize, Deserialize, Validate, ToSchema)]
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

// 사용자 환경 설정 조회 dto
#[derive(Serialize, sqlx::FromRow, ToSchema, Clone, Debug, PartialEq)]
#[schema(example = json!({
    "user_set_language": "ko",
    "user_set_timezone": "UTC",
    "user_set_note_email": false,
    "user_set_note_push": false,
    "updated_at": "2025-08-21T10:00:00Z"
}))]
pub struct SettingsRes {
    pub user_set_language: String,
    pub user_set_timezone: String,
    pub user_set_note_email: bool,
    pub user_set_note_push: bool,
    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
}

// 사용자 환경설정 수정 dto
#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "user_set_language": "ko",
    "user_set_timezone": "UTC",
    "user_set_note_email": true,
    "user_set_note_push": false
}))]
pub struct SettingsUpdateReq {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 2, max = 2))] // ISO 639-1
    pub user_set_language: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))] // IANA timezone format, basic validation
    pub user_set_timezone: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_set_note_email: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_set_note_push: Option<bool>,
}
