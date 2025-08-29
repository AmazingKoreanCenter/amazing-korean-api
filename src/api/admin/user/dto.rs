use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::types::{UserAuth, UserGender, UserState};

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
    "user_auth": "learner",
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
    pub gender: UserGender,
    pub user_state: UserState,
    pub user_auth: UserAuth,
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
            "user_auth": "learner",
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
    pub gender: Option<UserGender>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_state: Option<UserState>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_auth: Option<UserAuth>,
}
