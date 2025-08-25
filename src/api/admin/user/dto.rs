use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// 사용자 권한 enum
#[derive(Serialize, Deserialize, ToSchema, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserAuth {
    Hymn,
    Admin,
    Manager,
    User,
}

impl std::fmt::Display for UserAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserAuth::Hymn => "HYMN",
            UserAuth::Admin => "admin",
            UserAuth::Manager => "manager",
            UserAuth::User => "user",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for UserAuth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HYMN" => Ok(UserAuth::Hymn),
            "admin" => Ok(UserAuth::Admin),
            "manager" => Ok(UserAuth::Manager),
            "user" => Ok(UserAuth::User),
            _ => Err(format!("Invalid UserAuth: {}", s)),
        }
    }
}

/// 사용자 상태 enum
#[derive(Serialize, Deserialize, ToSchema, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserState {
    On,
    Off,
}

impl std::fmt::Display for UserState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserState::On => "on",
            UserState::Off => "off",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for UserState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(UserState::On),
            "off" => Ok(UserState::Off),
            _ => Err(format!("Invalid UserState: {}", s)),
        }
    }
}

/// 관리자용 사용자 프로필 응답
#[derive(Serialize, sqlx::FromRow, ToSchema, Clone, Debug, PartialEq)]
#[schema(example = json!({
    "id": 123,
    "email": "admin_user@example.com",
    "name": "Admin User",
    "nickname": "AdminNick",
    "language": "en",
    "country": "US",
    "birthday": "2000-01-01",
    "gender": "male",
    "user_state": "on",
    "user_auth": "user",
    "created_at": "2025-08-21T10:00:00Z",
    "quit_at": null
}))]
pub struct AdminUserRes {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub nickname: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    #[schema(value_type = String, format = "date")]
    pub birthday: Option<NaiveDate>,
    pub gender: String,
    pub user_state: String,
    pub user_auth: String,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
    pub quit_at: Option<DateTime<Utc>>,
}

/// 관리자용 사용자 목록 응답
#[derive(Serialize, ToSchema, Clone, Debug, PartialEq)]
#[schema(example = json!({
    "total": 1,
    "items": [
        {
            "id": 123,
            "email": "admin_user@example.com",
            "name": "Admin User",
            "nickname": "AdminNick",
            "language": "en",
            "country": "US",
            "birthday": "2000-01-01",
            "gender": "male",
            "user_state": "on",
            "user_auth": "user",
            "created_at": "2025-08-21T10:00:00Z",
            "quit_at": null
        }
    ]
}))]
pub struct AdminListUsersRes {
    pub total: i64,
    pub items: Vec<AdminUserRes>,
}

/// 관리자용 사용자 수정 요청
#[derive(Serialize, Deserialize, Validate, ToSchema, Clone, Debug, PartialEq)]
#[schema(example = json!({
    "email": "updated_admin@example.com",
    "name": "Updated Admin",
    "nickname": "UpdatedAdminNick",
    "language": "ko",
    "country": "KR",
    "birthday": "1990-12-25",
    "gender": "female",
    "user_state": "off",
    "user_auth": "manager"
}))]
pub struct AdminUpdateUserReq {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(email)]
    pub email: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,

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
    pub gender: Option<String>, // Use String for flexibility, validation in service

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_state: Option<UserState>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_auth: Option<UserAuth>,
}
