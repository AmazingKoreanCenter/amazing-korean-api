use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::api::user::dto::ProfileRes;
use crate::types::{UserAuth, UserState};

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[schema(example = json!({ "email": "test@example.com", "password": "password123" }))]
pub struct LoginReq {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 72))]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "token": "eyJ...", "expires_in": 900, "user": { "id": 1, "email": "test@example.com", "name": "Test User", "user_state": "on", "user_auth": "learner", "created_at": "2025-08-21T10:00:00Z" } } ))]
pub struct LoginRes {
    pub token: String,
    pub expires_in: i64,
    pub user: ProfileRes,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({ "token": "eyJ...", "expires_in": 900 }))]
pub struct RefreshRes {
    pub token: String,
    pub expires_in: i64,
}

#[derive(Serialize, ToSchema, sqlx::FromRow)]
#[schema(example = json!({ "id": 1, "email": "test@example.com", "name": "Test User", "user_state": "on", "user_auth": "learner", "created_at": "2025-08-21T10:00:00Z" }))]
pub struct UserOut {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub user_state: UserState,
    pub user_auth: UserAuth,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
