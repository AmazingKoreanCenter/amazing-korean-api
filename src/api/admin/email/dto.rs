use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// 테스트 이메일 발송 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TestEmailReq {
    /// 수신자 이메일 주소
    #[validate(email(message = "유효한 이메일 주소를 입력하세요"))]
    pub to: String,

    /// 이메일 템플릿 종류
    /// - password_reset: 비밀번호 재설정 인증 코드
    /// - email_verification: 이메일 인증 코드
    /// - welcome: 환영 이메일
    pub template: EmailTemplateType,
}

/// 이메일 템플릿 종류
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum EmailTemplateType {
    /// 비밀번호 재설정 인증 코드
    PasswordReset,
    /// 이메일 인증 코드 (회원가입용)
    EmailVerification,
    /// 환영 이메일
    Welcome,
}

/// 테스트 이메일 발송 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct TestEmailRes {
    /// 발송 성공 여부
    pub success: bool,
    /// 메시지
    pub message: String,
    /// 발송 대상 이메일
    pub to: String,
    /// 사용된 템플릿
    pub template: EmailTemplateType,
}
