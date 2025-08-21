use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DB의 CHECK 제약과 1:1로 맞춘 enum
#[derive(Serialize, Deserialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    None,
    Male,
    Female,
    Other,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Gender::None => "none",
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Other => "other",
        };
        write!(f, "{s}")
    }
}

/// 회원가입 요청
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
    pub gender: Option<Gender>,
}

/// 회원가입 응답
#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "user_id": 123 }))]
pub struct SignupRes {
    pub user_id: i64,
}

/// 내 프로필 응답
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
    pub gender: String, // DB에서 String으로 가져오므로
    pub user_state: String,
    pub user_auth: String,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

/// 내 프로필 수정 요청
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
    pub gender: Option<Gender>,
}
