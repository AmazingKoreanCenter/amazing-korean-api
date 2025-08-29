use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::types::{UserAuth, UserGender, UserState};

// 회원가입 요청 dto
#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "email": "test@example.com",
    "password": "password123",
    "name": "Test User",
    "terms_service": true,
    "terms_personal": true,
    "nickname": "TestNick",
    "language": "en",
    "country": "US",
    "birthday": "2000-01-01",
    "gender": "male"
}))]
pub struct SignupReq {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 72))]
    pub password: String,
    #[validate(length(min = 1, max = 50))]
    pub name: String,

    pub terms_service: bool,
    pub terms_personal: bool,

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

// 회원가입 응답 dto
#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "user_id": 123 }))]
pub struct SignupRes {
    pub user_id: i64,
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
    "user_auth": "user",
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
    pub user_state: UserState,
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
pub struct UpdateReq {
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
#[derive(Serialize, ToSchema, Clone, Debug, PartialEq)]
#[schema(example = json!({
    "user_id": 123,
    "ui_language": "ko",
    "timezone": "Asia/Seoul",
    "notifications_email": true,
    "notifications_push": false,
    "study_languages": [
        {"lang_code":"en","priority":1,"is_primary":false},
        {"lang_code":"ko","priority":2,"is_primary":true}
    ]
}))]
pub struct SettingsRes {
    pub user_id: i64,
    pub ui_language: Option<String>,
    pub timezone: Option<String>,
    pub notifications_email: Option<bool>,
    pub notifications_push: Option<bool>,
    pub study_languages: Vec<StudyLangItem>,
}

// 사용자 환경설정 수정 dto
#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "ui_language": "ko",
    "timezone": "Asia/Seoul",
    "notifications_email": true,
    "notifications_push": false,
    "study_languages": [
        {"lang_code":"ko","priority":2,"is_primary":true},
        {"lang_code":"en","priority":1,"is_primary":false}
    ]
}))]
pub struct SettingsUpdateReq {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 2, max = 2))] // ISO 639-1
    pub ui_language: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))] // IANA timezone format, basic validation
    pub timezone: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications_email: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications_push: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 8))] // Max 8 study languages
    pub study_languages: Option<Vec<StudyLangItem>>,
}