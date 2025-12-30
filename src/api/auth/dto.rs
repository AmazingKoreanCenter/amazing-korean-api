// FILE: src/api/auth/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "email": "test@example.com",
    "password": "password123",
    "device": "web",
    "browser": "chrome",
    "os": "linux",
    "user_agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
}))]
pub struct LoginReq {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 72))]
    pub password: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 3600
}))]
pub struct AccessTokenRes {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "user_id": 123,
    "access": {
        "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
        "expires_in": 3600
    },
    "session_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef"
}))]
pub struct LoginRes {
    pub user_id: i64,
    pub access: AccessTokenRes,
    pub session_id: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 3600
}))]
pub struct RefreshRes {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "refresh_token": "c2Vzc2lvbl9pZDp5Yy1yYW5kb20tdXVpZA"
}))]
pub struct RefreshReq {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "ok": true }))]
pub struct LogoutRes {
    pub ok: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({ "everywhere": true }))]
pub struct LogoutAllReq {
    #[serde(default = "default_true")]
    pub everywhere: bool,
}

fn default_true() -> bool {
    true
}
