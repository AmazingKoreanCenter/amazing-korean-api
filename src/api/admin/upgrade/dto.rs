//! 관리자 초대/승격 DTO
//!
//! - 관리자는 오직 초대를 통해서만 생성 가능
//! - OAuth 로그인 불가 (이메일/비밀번호만)

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::types::{UserAuth, UserGender, UserLanguage};

// =============================================================================
// 7-68: POST /admin/upgrade (관리자 초대)
// =============================================================================

/// 관리자 초대 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "email": "new-admin@example.com",
    "role": "admin"
}))]
pub struct UpgradeInviteReq {
    /// 초대할 이메일 주소
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    /// 부여할 권한 (admin | manager)
    #[validate(custom(function = "validate_invite_role"))]
    pub role: String,
}

fn validate_invite_role(role: &str) -> Result<(), validator::ValidationError> {
    match role {
        "admin" | "manager" => Ok(()),
        _ => {
            let mut err = validator::ValidationError::new("invalid_role");
            err.message = Some("Role must be 'admin' or 'manager'".into());
            Err(err)
        }
    }
}

/// 관리자 초대 응답
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({
    "message": "Invitation sent successfully",
    "expires_at": "2026-02-04T12:10:00Z"
}))]
pub struct UpgradeInviteRes {
    pub message: String,
    #[schema(value_type = String, format = "date-time")]
    pub expires_at: DateTime<Utc>,
}

// =============================================================================
// 7-69: GET /admin/upgrade/verify (초대 코드 검증)
// =============================================================================

/// 초대 코드 검증 요청 (Query Parameter)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpgradeVerifyReq {
    /// 초대 코드 (ak_upgrade_xxx)
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,
}

/// 초대 코드 검증 응답
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({
    "email": "new-admin@example.com",
    "role": "admin",
    "invited_by": "hymn@amazingkorean.net",
    "expires_at": "2026-02-04T12:10:00Z"
}))]
pub struct UpgradeVerifyRes {
    pub email: String,
    pub role: String,
    pub invited_by: String,
    #[schema(value_type = String, format = "date-time")]
    pub expires_at: DateTime<Utc>,
}

// =============================================================================
// 7-70: POST /admin/upgrade/accept (관리자 계정 생성)
// =============================================================================

/// 초대 수락 및 관리자 계정 생성 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[schema(example = json!({
    "code": "ak_upgrade_xxx",
    "password": "SecureP@ss123",
    "name": "홍길동",
    "nickname": "admin_hong",
    "country": "KR",
    "birthday": "1990-01-01",
    "gender": "male",
    "language": "ko"
}))]
pub struct UpgradeAcceptReq {
    /// 초대 코드
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,

    /// 비밀번호 (최소 8자, 영문+숫자 포함)
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    /// 이름
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,

    /// 닉네임
    #[validate(length(min = 1, max = 100, message = "Nickname must be 1-100 characters"))]
    pub nickname: String,

    /// 국가 코드
    #[validate(length(min = 1, max = 50, message = "Country must be 1-50 characters"))]
    pub country: String,

    /// 생년월일
    #[schema(value_type = String, format = "date")]
    pub birthday: NaiveDate,

    /// 성별 (male | female | none)
    pub gender: UserGender,

    /// 언어 (ko | en | ...)
    pub language: UserLanguage,
}

/// 관리자 계정 생성 응답
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({
    "user_id": 123,
    "email": "new-admin@example.com",
    "user_auth": "admin",
    "message": "Admin account created successfully"
}))]
pub struct UpgradeAcceptRes {
    pub user_id: i64,
    pub email: String,
    pub user_auth: UserAuth,
    pub message: String,
}

// =============================================================================
// Redis 저장용 내부 구조체
// =============================================================================

/// Redis에 저장할 초대 정보
#[derive(Debug, Serialize, Deserialize)]
pub struct InviteData {
    pub email: String,
    pub role: String,
    pub invited_by_id: i64,
    pub invited_by_email: String,
    pub created_at: DateTime<Utc>,
}
