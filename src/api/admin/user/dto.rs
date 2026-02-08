use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::types::{AdminAction, UserActionLog, UserAuth, UserGender, UserLanguage};

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
    pub birthday: Option<String>,
    pub gender: UserGender,
    pub user_state: bool,
    pub user_auth: UserAuth,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
    pub quit_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AdminUserListReq {
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub q: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct AdminUserSummary {
    pub id: i64,
    pub email: String,
    pub nickname: Option<String>,
    pub role: UserAuth,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminUserListMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminUserListRes {
    pub items: Vec<AdminUserSummary>,
    pub meta: AdminUserListMeta,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AdminCreateUserReq {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1, max = 100))]
    pub nickname: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,

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
    pub user_state: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_auth: Option<String>,
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct AdminBulkCreateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<AdminCreateUserReq>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BulkSummary {
    pub total: i64,
    pub success: i64,
    pub failure: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BulkItemError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BulkItemResult {
    pub email: String,
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AdminUserRes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<BulkItemError>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminBulkCreateRes {
    pub summary: BulkSummary,
    pub results: Vec<BulkItemResult>,
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct AdminBulkUpdateItemReq {
    pub id: i64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(email)]
    pub email: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 8))]
    pub password: Option<String>,

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
    pub user_state: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_auth: Option<UserAuth>,
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct AdminBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<AdminBulkUpdateItemReq>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BulkUpdateItemResult {
    pub id: i64,
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AdminUserRes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<BulkItemError>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminBulkUpdateRes {
    pub summary: BulkSummary,
    pub results: Vec<BulkUpdateItemResult>,
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
    #[validate(length(min = 8))]
    pub password: Option<String>,

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
    pub user_state: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_auth: Option<UserAuth>,
}

// ==========================================
// Admin User Logs DTOs
// ==========================================

/// 관리자 변경 로그 요청 파라미터
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AdminUserLogsReq {
    pub page: Option<i64>,
    pub size: Option<i64>,
}

/// 관리자 변경 로그 아이템
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct AdminUserLogItem {
    pub id: i64,
    pub admin_id: i64,
    pub admin_email: Option<String>,
    pub action: AdminAction,
    #[schema(value_type = Object)]
    pub before: Option<JsonValue>,
    #[schema(value_type = Object)]
    pub after: Option<JsonValue>,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

/// 관리자 변경 로그 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct AdminUserLogsRes {
    pub items: Vec<AdminUserLogItem>,
    pub meta: AdminUserListMeta,
}

/// 사용자 자체 변경 로그 아이템
#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct UserLogItem {
    pub id: i64,
    pub action: UserActionLog,
    pub success: bool,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub language: Option<UserLanguage>,
    pub country: Option<String>,
    #[schema(value_type = String, format = "date")]
    pub birthday: Option<String>,
    pub gender: Option<UserGender>,
    pub password_changed: bool,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

/// 사용자 자체 변경 로그 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct UserLogsRes {
    pub items: Vec<UserLogItem>,
    pub meta: AdminUserListMeta,
}
