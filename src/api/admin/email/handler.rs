use axum::{extract::State, Json};
use validator::Validate;

use crate::{
    api::auth::extractor::AuthUser,
    error::{AppError, AppResult},
    external::email::EmailTemplate,
    state::AppState,
    types::UserAuth,
};

use super::dto::{EmailTemplateType, TestEmailReq, TestEmailRes};

#[allow(unused_imports)]
use serde_json::json;

/// 테스트 이메일 발송
///
/// Admin/Manager 권한 필요. 이메일 시스템 동작 확인용.
#[utoipa::path(
    post,
    path = "/admin/email/test",
    tag = "admin_email",
    request_body = TestEmailReq,
    responses(
        (status = 200, description = "이메일 발송 성공", body = TestEmailRes, example = json!({
            "success": true,
            "message": "테스트 이메일이 성공적으로 발송되었습니다.",
            "to": "test@example.com",
            "template": "password_reset"
        })),
        (status = 400, description = "잘못된 요청", body = crate::error::ErrorBody),
        (status = 401, description = "인증 필요", body = crate::error::ErrorBody),
        (status = 403, description = "권한 없음", body = crate::error::ErrorBody),
        (status = 503, description = "이메일 서비스 비활성화", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn send_test_email(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Json(req): Json<TestEmailReq>,
) -> AppResult<Json<TestEmailRes>> {
    // 1. Admin/Manager 권한 확인
    if !matches!(auth_user.role, UserAuth::Admin | UserAuth::Manager | UserAuth::Hymn) {
        return Err(AppError::Forbidden(
            "이메일 테스트는 Admin 또는 Manager 권한이 필요합니다.".to_string(),
        ));
    }

    // 2. 입력값 검증
    req.validate()?;

    // 3. 이메일 클라이언트 확인
    let email_sender = st.email.as_ref().ok_or_else(|| {
        AppError::ServiceUnavailable(
            "이메일 서비스가 비활성화되어 있습니다. EMAIL_PROVIDER 환경변수를 설정하세요."
                .to_string(),
        )
    })?;

    // 4. 템플릿 생성
    let template = match req.template {
        EmailTemplateType::PasswordReset => EmailTemplate::PasswordResetCode {
            code: "123456".to_string(),
            expires_in_min: 10,
        },
        EmailTemplateType::EmailVerification => EmailTemplate::EmailVerification {
            code: "654321".to_string(),
            expires_in_min: 10,
        },
        EmailTemplateType::Welcome => EmailTemplate::Welcome {
            nickname: "테스트 사용자".to_string(),
        },
    };

    // 5. 이메일 발송
    crate::external::email::send_templated(email_sender.as_ref(), &req.to, template).await?;

    tracing::info!(
        admin_id = auth_user.sub,
        to = %req.to,
        template = ?req.template,
        "Test email sent successfully"
    );

    Ok(Json(TestEmailRes {
        success: true,
        message: "테스트 이메일이 성공적으로 발송되었습니다.".to_string(),
        to: req.to,
        template: req.template,
    }))
}
