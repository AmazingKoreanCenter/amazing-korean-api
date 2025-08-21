use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// 회원가입 요청
#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct SignUpReq {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 72))]
    pub password: String,
    #[validate(length(min = 1, max = 50))]
    pub name: String,

    /// 약관 동의
    pub terms_service: bool,
    pub terms_personal: bool,
}

/// 로그인 요청
#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginReq {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 72))]
    pub password: String,
}

/// 로그인 응답(JWT)
#[derive(Serialize, ToSchema)]
pub struct LoginResp {
    /// JWT 액세스 토큰
    pub access_token: String,
    /// "Bearer"
    pub token_type: String,
    /// 만료(초)
    pub expires_in: i64,
}

/// 공개용 사용자 응답
#[derive(Serialize, sqlx::FromRow, ToSchema)]
pub struct UserOut {
    pub user_id: i64,
    pub user_email: String,
    pub user_name: Option<String>,
    pub user_created_at: chrono::DateTime<chrono::Utc>,
    pub user_state: String,
    pub user_auth: String,
}
