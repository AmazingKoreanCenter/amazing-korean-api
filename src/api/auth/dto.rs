use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// =====================================================================
// Request DTOs (요청)
// =====================================================================

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "email": "front@front.com",
    "password": "front123!",
    "device": "web",
    "browser": "chrome",
    "os": "linux",
    "user_agent": "Mozilla/5.0..."
}))]
pub struct LoginReq {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6, max = 72))]
    pub password: String,

    // 아래 정보는 보통 User-Agent 헤더로 분석하지만, 
    // 클라이언트가 명시적으로 보낼 경우를 위해 유지 (Option)
    #[serde(default)]
    pub device: Option<String>,
    #[serde(default)]
    pub browser: Option<String>,
    #[serde(default)]
    pub os: Option<String>,
    /*#[serde(default)]
    pub user_agent: Option<String>,*/
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "refresh_token": "c2Vzc2lvbl9pZDp5Yy1yYW5kb20tdXVpZA"
}))]
pub struct RefreshReq {
    // 쿠키를 사용할 수 없는 환경(앱 등)을 위해 바디로도 받을 수 있게 유지
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "name": "홍길동",
    "email": "test@example.com"
}))]
pub struct FindIdReq {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "reset_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "new_password": "newStrongPassword123!"
}))]
pub struct ResetPwReq {
    #[validate(length(min = 1))]
    pub reset_token: String,
    #[validate(length(min = 1))]
    pub new_password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({ "everywhere": true }))]
pub struct LogoutAllReq {
    #[serde(default = "default_true")]
    pub everywhere: bool,
}

fn default_true() -> bool {
    true
}

// =====================================================================
// Response DTOs (응답)
// =====================================================================

/// 액세스 토큰 공통 규격
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "expires_at": "2025-01-01T12:00:00Z"
}))]
pub struct AccessTokenRes {
    pub access_token: String,
    pub token_type: String, // "Bearer" 고정
    pub expires_in: i64,    // 초 단위
    pub expires_at: String, // 프론트엔드 편의용 ISO String
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "user_id": 123,
    "access": {
        "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
        "token_type": "Bearer",
        "expires_in": 3600,
        "expires_at": "2025-01-01T12:00:00Z"
    },
    "session_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef"
}))]
pub struct LoginRes {
    pub user_id: i64,
    pub access: AccessTokenRes,
    pub session_id: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "expires_at": "2025-01-01T12:00:00Z"
}))]
pub struct RefreshRes {
    // AccessTokenRes와 구조가 같지만 명시적 분리
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub expires_at: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "message": "If the account exists, the ID has been sent to your email."
}))]
pub struct FindIdRes {
    pub message: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "message": "Password has been reset. All active sessions are invalidated."
}))]
pub struct ResetPwRes {
    pub message: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({ "ok": true }))]
pub struct LogoutRes {
    pub ok: bool,
}

// =====================================================================
// Google OAuth DTOs
// =====================================================================

/// Google OAuth 시작 응답 (인증 URL 반환)
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "auth_url": "https://accounts.google.com/o/oauth2/v2/auth?client_id=..."
}))]
pub struct GoogleAuthUrlRes {
    pub auth_url: String,
}

/// Google OAuth 콜백 쿼리 파라미터
#[derive(Deserialize, Validate)]
pub struct GoogleCallbackQuery {
    /// Authorization Code (성공 시)
    pub code: Option<String>,

    /// State 파라미터 (CSRF 방지)
    pub state: String,

    /// 에러 코드 (사용자 취소 등)
    pub error: Option<String>,

    /// 에러 상세 설명
    pub error_description: Option<String>,
}

// =====================================================================
// Password Reset DTOs (비밀번호 재설정 - 이메일 인증 기반)
// =====================================================================

/// 비밀번호 재설정 요청 (인증코드 발송)
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "email": "user@example.com"
}))]
pub struct RequestResetReq {
    #[validate(email)]
    pub email: String,
}

/// 비밀번호 재설정 요청 응답
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "message": "If the email exists, a verification code has been sent."
}))]
pub struct RequestResetRes {
    pub message: String,
}

/// 인증코드 검증 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "email": "user@example.com",
    "code": "123456"
}))]
pub struct VerifyResetReq {
    #[validate(email)]
    pub email: String,
    #[validate(length(equal = 6, message = "Code must be 6 digits"))]
    pub code: String,
}

/// 인증코드 검증 응답 (reset_token 발급)
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = json!({
    "reset_token": "ak_reset_xxxxxx",
    "expires_in": 1800
}))]
pub struct VerifyResetRes {
    pub reset_token: String,
    pub expires_in: i64,  // 초 단위
}