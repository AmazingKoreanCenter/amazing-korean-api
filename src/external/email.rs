use async_trait::async_trait;

use crate::error::{AppError, AppResult};

// =============================================================================
// EmailSender trait
// =============================================================================

/// 이메일 발송 추상화 trait
///
/// `EMAIL_PROVIDER` 환경변수로 구현체 전환:
/// - `resend`: Resend API (ResendEmailSender)
/// - `none`: 비활성 (개발 환경 전용)
#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, html: &str, text: &str) -> AppResult<()>;
}

/// 템플릿 기반 이메일 발송 (provider 무관)
pub async fn send_templated(
    sender: &dyn EmailSender,
    to: &str,
    template: EmailTemplate,
) -> AppResult<()> {
    let (subject, html_body, text_body) = render_template(template);
    sender.send_email(to, &subject, &html_body, &text_body).await
}

// =============================================================================
// Resend 구현
// =============================================================================

/// Resend API 이메일 클라이언트
///
/// https://resend.com/docs/api-reference/emails/send-email
#[derive(Clone)]
pub struct ResendEmailSender {
    http: reqwest::Client,
    api_key: String,
    from_address: String,
}

impl ResendEmailSender {
    pub fn new(api_key: String, from_address: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key,
            from_address,
        }
    }
}

#[async_trait]
impl EmailSender for ResendEmailSender {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html: &str,
        text: &str,
    ) -> AppResult<()> {
        let body = serde_json::json!({
            "from": self.from_address,
            "to": [to],
            "subject": subject,
            "html": html,
            "text": text
        });

        let resp = self
            .http
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(error = %e, to = %to, "Failed to call Resend API");
                AppError::External(format!("Failed to call Resend API: {}", e))
            })?;

        let status = resp.status();
        if !status.is_success() {
            let error_body = resp.text().await.unwrap_or_default();
            tracing::warn!(
                status = %status,
                error_body = %error_body,
                to = %to,
                "Resend API error"
            );
            let msg = match status.as_u16() {
                401 => "Resend API key is invalid",
                403 => "Resend domain not verified or sending not allowed",
                429 => "Resend rate limit exceeded",
                _ => "Resend API error",
            };
            return Err(AppError::External(format!("{}: {} {}", msg, status, error_body)));
        }

        tracing::info!(to = %to, subject = %subject, provider = "resend", "Email sent successfully");

        Ok(())
    }
}

// =============================================================================
// 이메일 템플릿
// =============================================================================

/// 이메일 템플릿 종류
pub enum EmailTemplate {
    /// 비밀번호 재설정 인증 코드
    PasswordResetCode { code: String, expires_in_min: i32 },
    /// 이메일 인증 코드 (회원가입용)
    EmailVerification { code: String, expires_in_min: i32 },
    /// 환영 이메일
    Welcome { nickname: String },
    /// 관리자 초대
    AdminInvite {
        invite_url: String,
        role: String,
        invited_by: String,
        expires_in_min: i32,
    },
    /// 교재 주문 접수 확인
    TextbookOrderConfirmation {
        order_code: String,
        orderer_name: String,
        total_quantity: i32,
        total_amount: i32,
    },
    /// E-book 구매 접수 확인 (pending)
    EbookPurchaseConfirmation {
        purchase_code: String,
        language_name: String,
        edition_label: String,
        price: String,
        currency: String,
    },
    /// E-book 결제 완료 알림 (completed)
    EbookPurchaseCompleted {
        purchase_code: String,
        language_name: String,
        edition_label: String,
    },
    /// 교재 주문 상태 변경 알림
    TextbookOrderStatusUpdate {
        order_code: String,
        orderer_name: String,
        new_status: String,
        status_label: String,
        tracking_number: Option<String>,
        tracking_provider: Option<String>,
    },
}

/// 템플릿을 HTML/텍스트로 렌더링
fn render_template(template: EmailTemplate) -> (String, String, String) {
    match template {
        EmailTemplate::PasswordResetCode { code, expires_in_min } => {
            let subject = "[Amazing Korean] 비밀번호 재설정 인증 코드".to_string();
            let html_body = format!(
                r#"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin: 0; padding: 0; font-family: 'Apple SD Gothic Neo', 'Malgun Gothic', sans-serif; background-color: #f5f5f5;">
    <table role="presentation" style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 40px 0;">
                <table role="presentation" style="width: 100%; max-width: 600px; margin: 0 auto; background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="padding: 40px 40px 20px 40px; text-align: center; border-bottom: 1px solid #eee;">
                            <h1 style="margin: 0; color: #333; font-size: 24px;">Amazing Korean</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="margin: 0 0 20px 0; color: #333; font-size: 20px;">비밀번호 재설정</h2>
                            <p style="margin: 0 0 30px 0; color: #666; font-size: 16px; line-height: 1.6;">
                                아래 인증 코드를 입력하여 비밀번호를 재설정하세요.
                            </p>
                            <div style="background-color: #f8f9fa; border-radius: 8px; padding: 30px; text-align: center; margin-bottom: 30px;">
                                <span style="font-size: 36px; font-weight: bold; letter-spacing: 8px; color: #333;">{code}</span>
                            </div>
                            <p style="margin: 0 0 10px 0; color: #999; font-size: 14px;">
                                이 코드는 <strong>{expires_in_min}분</strong> 후 만료됩니다.
                            </p>
                            <p style="margin: 0; color: #999; font-size: 14px;">
                                본인이 요청하지 않았다면 이 이메일을 무시하세요.
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px 40px; background-color: #f8f9fa; border-radius: 0 0 8px 8px;">
                            <p style="margin: 0; color: #999; font-size: 12px; text-align: center;">
                                © Amazing Korean. All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#
            );
            let text_body = format!(
                "[Amazing Korean] 비밀번호 재설정\n\n인증 코드: {code}\n\n이 코드는 {expires_in_min}분 후 만료됩니다.\n\n본인이 요청하지 않았다면 이 이메일을 무시하세요."
            );
            (subject, html_body, text_body)
        }

        EmailTemplate::EmailVerification { code, expires_in_min } => {
            let subject = "[Amazing Korean] 이메일 인증 코드".to_string();
            let html_body = format!(
                r#"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin: 0; padding: 0; font-family: 'Apple SD Gothic Neo', 'Malgun Gothic', sans-serif; background-color: #f5f5f5;">
    <table role="presentation" style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 40px 0;">
                <table role="presentation" style="width: 100%; max-width: 600px; margin: 0 auto; background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="padding: 40px 40px 20px 40px; text-align: center; border-bottom: 1px solid #eee;">
                            <h1 style="margin: 0; color: #333; font-size: 24px;">Amazing Korean</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="margin: 0 0 20px 0; color: #333; font-size: 20px;">이메일 인증</h2>
                            <p style="margin: 0 0 30px 0; color: #666; font-size: 16px; line-height: 1.6;">
                                Amazing Korean에 가입해 주셔서 감사합니다!<br>
                                아래 인증 코드를 입력하여 이메일을 인증하세요.
                            </p>
                            <div style="background-color: #f8f9fa; border-radius: 8px; padding: 30px; text-align: center; margin-bottom: 30px;">
                                <span style="font-size: 36px; font-weight: bold; letter-spacing: 8px; color: #333;">{code}</span>
                            </div>
                            <p style="margin: 0 0 10px 0; color: #999; font-size: 14px;">
                                이 코드는 <strong>{expires_in_min}분</strong> 후 만료됩니다.
                            </p>
                            <p style="margin: 0; color: #999; font-size: 14px;">
                                본인이 요청하지 않았다면 이 이메일을 무시하세요.
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px 40px; background-color: #f8f9fa; border-radius: 0 0 8px 8px;">
                            <p style="margin: 0; color: #999; font-size: 12px; text-align: center;">
                                © Amazing Korean. All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#
            );
            let text_body = format!(
                "[Amazing Korean] 이메일 인증\n\nAmazing Korean에 가입해 주셔서 감사합니다!\n\n인증 코드: {code}\n\n이 코드는 {expires_in_min}분 후 만료됩니다.\n\n본인이 요청하지 않았다면 이 이메일을 무시하세요."
            );
            (subject, html_body, text_body)
        }

        EmailTemplate::Welcome { nickname } => {
            let subject = "[Amazing Korean] 가입을 환영합니다!".to_string();
            let html_body = format!(
                r#"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin: 0; padding: 0; font-family: 'Apple SD Gothic Neo', 'Malgun Gothic', sans-serif; background-color: #f5f5f5;">
    <table role="presentation" style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 40px 0;">
                <table role="presentation" style="width: 100%; max-width: 600px; margin: 0 auto; background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="padding: 40px 40px 20px 40px; text-align: center; border-bottom: 1px solid #eee;">
                            <h1 style="margin: 0; color: #333; font-size: 24px;">Amazing Korean</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="margin: 0 0 20px 0; color: #333; font-size: 20px;">{nickname}님, 환영합니다!</h2>
                            <p style="margin: 0 0 20px 0; color: #666; font-size: 16px; line-height: 1.6;">
                                Amazing Korean에 가입해 주셔서 감사합니다.
                            </p>
                            <p style="margin: 0 0 30px 0; color: #666; font-size: 16px; line-height: 1.6;">
                                지금 바로 한국어 학습을 시작해 보세요!
                            </p>
                            <div style="text-align: center;">
                                <a href="https://amazingkorean.net" style="display: inline-block; background-color: #333; color: #ffffff; text-decoration: none; padding: 14px 30px; border-radius: 6px; font-size: 16px; font-weight: bold;">
                                    학습 시작하기
                                </a>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px 40px; background-color: #f8f9fa; border-radius: 0 0 8px 8px;">
                            <p style="margin: 0; color: #999; font-size: 12px; text-align: center;">
                                © Amazing Korean. All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#
            );
            let text_body = format!(
                "[Amazing Korean] 가입을 환영합니다!\n\n{nickname}님, Amazing Korean에 가입해 주셔서 감사합니다.\n\n지금 바로 한국어 학습을 시작해 보세요!\n\nhttps://amazingkorean.net"
            );
            (subject, html_body, text_body)
        }

        EmailTemplate::AdminInvite {
            invite_url,
            role,
            invited_by,
            expires_in_min,
        } => {
            let role_display = match role.as_str() {
                "admin" => "관리자 (Admin)",
                "manager" => "매니저 (Manager)",
                _ => &role,
            };
            let subject = "[Amazing Korean] 관리자 초대".to_string();
            let html_body = format!(
                r#"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin: 0; padding: 0; font-family: 'Apple SD Gothic Neo', 'Malgun Gothic', sans-serif; background-color: #f5f5f5;">
    <table role="presentation" style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 40px 0;">
                <table role="presentation" style="width: 100%; max-width: 600px; margin: 0 auto; background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="padding: 40px 40px 20px 40px; text-align: center; border-bottom: 1px solid #eee;">
                            <h1 style="margin: 0; color: #333; font-size: 24px;">Amazing Korean</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="margin: 0 0 20px 0; color: #333; font-size: 20px;">관리자 초대</h2>
                            <p style="margin: 0 0 20px 0; color: #666; font-size: 16px; line-height: 1.6;">
                                Amazing Korean 관리자로 초대되었습니다.
                            </p>
                            <div style="background-color: #f8f9fa; border-radius: 8px; padding: 20px; margin-bottom: 30px;">
                                <p style="margin: 0 0 10px 0; color: #666; font-size: 14px;">
                                    <strong>권한:</strong> {role_display}
                                </p>
                                <p style="margin: 0; color: #666; font-size: 14px;">
                                    <strong>초대자:</strong> {invited_by}
                                </p>
                            </div>
                            <p style="margin: 0 0 30px 0; color: #666; font-size: 16px; line-height: 1.6;">
                                아래 버튼을 클릭하여 관리자 계정을 생성하세요.
                            </p>
                            <div style="text-align: center; margin-bottom: 30px;">
                                <a href="{invite_url}" style="display: inline-block; background-color: #333; color: #ffffff; text-decoration: none; padding: 14px 30px; border-radius: 6px; font-size: 16px; font-weight: bold;">
                                    관리자 계정 생성
                                </a>
                            </div>
                            <p style="margin: 0 0 10px 0; color: #999; font-size: 14px;">
                                이 링크는 <strong>{expires_in_min}분</strong> 후 만료됩니다.
                            </p>
                            <p style="margin: 0; color: #999; font-size: 14px;">
                                본인이 요청하지 않았다면 이 이메일을 무시하세요.
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px 40px; background-color: #f8f9fa; border-radius: 0 0 8px 8px;">
                            <p style="margin: 0; color: #999; font-size: 12px; text-align: center;">
                                © Amazing Korean. All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#
            );
            let text_body = format!(
                "[Amazing Korean] 관리자 초대\n\nAmazing Korean 관리자로 초대되었습니다.\n\n권한: {role_display}\n초대자: {invited_by}\n\n아래 링크를 클릭하여 관리자 계정을 생성하세요:\n{invite_url}\n\n이 링크는 {expires_in_min}분 후 만료됩니다.\n\n본인이 요청하지 않았다면 이 이메일을 무시하세요."
            );
            (subject, html_body, text_body)
        }

        EmailTemplate::TextbookOrderConfirmation {
            order_code,
            orderer_name,
            total_quantity,
            total_amount,
        } => {
            let formatted_amount = format_krw(total_amount);
            let subject = format!("[Amazing Korean] 교재 주문 접수 확인 ({})", order_code);
            let html_body = format!(
                r#"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin: 0; padding: 0; font-family: 'Apple SD Gothic Neo', 'Malgun Gothic', sans-serif; background-color: #f5f5f5;">
    <table role="presentation" style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 40px 0;">
                <table role="presentation" style="width: 100%; max-width: 600px; margin: 0 auto; background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="padding: 40px 40px 20px 40px; text-align: center; border-bottom: 1px solid #eee;">
                            <h1 style="margin: 0; color: #333; font-size: 24px;">Amazing Korean</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="margin: 0 0 20px 0; color: #333; font-size: 20px;">교재 주문이 접수되었습니다</h2>
                            <p style="margin: 0 0 30px 0; color: #666; font-size: 16px; line-height: 1.6;">
                                {orderer_name}님, 교재 주문이 정상적으로 접수되었습니다.
                            </p>
                            <div style="background-color: #f8f9fa; border-radius: 8px; padding: 20px; margin-bottom: 30px;">
                                <p style="margin: 0 0 10px 0; color: #666; font-size: 14px;">
                                    <strong>주문번호:</strong> {order_code}
                                </p>
                                <p style="margin: 0 0 10px 0; color: #666; font-size: 14px;">
                                    <strong>총 수량:</strong> {total_quantity}권
                                </p>
                                <p style="margin: 0; color: #666; font-size: 14px;">
                                    <strong>총 금액:</strong> {formatted_amount}원
                                </p>
                            </div>
                            <h3 style="margin: 0 0 15px 0; color: #333; font-size: 16px;">입금 안내</h3>
                            <div style="background-color: #fff3cd; border-radius: 8px; padding: 20px; margin-bottom: 30px; border: 1px solid #ffc107;">
                                <p style="margin: 0 0 10px 0; color: #856404; font-size: 14px;">
                                    <strong>은행:</strong> 기업은행
                                </p>
                                <p style="margin: 0 0 10px 0; color: #856404; font-size: 14px;">
                                    <strong>계좌번호:</strong> 301-113949-01-011
                                </p>
                                <p style="margin: 0; color: #856404; font-size: 14px;">
                                    <strong>예금주:</strong> (주)놀라운한국어
                                </p>
                            </div>
                            <p style="margin: 0 0 10px 0; color: #999; font-size: 14px;">
                                입금 확인 후 인쇄 및 발송이 진행됩니다.
                            </p>
                            <p style="margin: 0; color: #999; font-size: 14px;">
                                주문 상태는 아래 링크에서 확인하실 수 있습니다.
                            </p>
                            <div style="text-align: center; margin-top: 30px;">
                                <a href="https://amazingkorean.net/textbook/order/{order_code}" style="display: inline-block; background-color: #333; color: #ffffff; text-decoration: none; padding: 14px 30px; border-radius: 6px; font-size: 16px; font-weight: bold;">
                                    주문 상태 확인
                                </a>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px 40px; background-color: #f8f9fa; border-radius: 0 0 8px 8px;">
                            <p style="margin: 0; color: #999; font-size: 12px; text-align: center;">
                                © Amazing Korean. All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#
            );
            let text_body = format!(
                "[Amazing Korean] 교재 주문 접수 확인\n\n{orderer_name}님, 교재 주문이 정상적으로 접수되었습니다.\n\n주문번호: {order_code}\n총 수량: {total_quantity}권\n총 금액: {formatted_amount}원\n\n[입금 안내]\n은행: 기업은행\n계좌번호: 301-113949-01-011\n예금주: (주)놀라운한국어\n\n입금 확인 후 인쇄 및 발송이 진행됩니다.\n\n주문 상태 확인: https://amazingkorean.net/textbook/order/{order_code}"
            );
            (subject, html_body, text_body)
        }

        EmailTemplate::TextbookOrderStatusUpdate {
            order_code,
            orderer_name,
            new_status: _,
            status_label,
            tracking_number,
            tracking_provider,
        } => {
            let tracking_section = if let Some(ref tn) = tracking_number {
                let provider = tracking_provider.as_deref().unwrap_or("-");
                format!(
                    r#"<div style="background-color: #d4edda; border-radius: 8px; padding: 20px; margin-bottom: 30px; border: 1px solid #28a745;">
                        <p style="margin: 0 0 10px 0; color: #155724; font-size: 14px;"><strong>택배사:</strong> {provider}</p>
                        <p style="margin: 0; color: #155724; font-size: 14px;"><strong>운송장번호:</strong> {tn}</p>
                    </div>"#
                )
            } else {
                String::new()
            };

            let subject = format!("[Amazing Korean] 교재 주문 상태 변경 ({})", order_code);
            let html_body = format!(
                r#"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin: 0; padding: 0; font-family: 'Apple SD Gothic Neo', 'Malgun Gothic', sans-serif; background-color: #f5f5f5;">
    <table role="presentation" style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 40px 0;">
                <table role="presentation" style="width: 100%; max-width: 600px; margin: 0 auto; background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="padding: 40px 40px 20px 40px; text-align: center; border-bottom: 1px solid #eee;">
                            <h1 style="margin: 0; color: #333; font-size: 24px;">Amazing Korean</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="margin: 0 0 20px 0; color: #333; font-size: 20px;">주문 상태가 변경되었습니다</h2>
                            <p style="margin: 0 0 30px 0; color: #666; font-size: 16px; line-height: 1.6;">
                                {orderer_name}님, 교재 주문의 상태가 변경되었습니다.
                            </p>
                            <div style="background-color: #f8f9fa; border-radius: 8px; padding: 20px; margin-bottom: 30px;">
                                <p style="margin: 0 0 10px 0; color: #666; font-size: 14px;"><strong>주문번호:</strong> {order_code}</p>
                                <p style="margin: 0; color: #333; font-size: 18px; font-weight: bold;">{status_label}</p>
                            </div>
                            {tracking_section}
                            <div style="text-align: center; margin-top: 30px;">
                                <a href="https://amazingkorean.net/textbook/order/{order_code}" style="display: inline-block; background-color: #333; color: #ffffff; text-decoration: none; padding: 14px 30px; border-radius: 6px; font-size: 16px; font-weight: bold;">
                                    주문 상태 확인
                                </a>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px 40px; background-color: #f8f9fa; border-radius: 0 0 8px 8px;">
                            <p style="margin: 0; color: #999; font-size: 12px; text-align: center;">
                                © Amazing Korean. All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#
            );

            let tracking_text = if let Some(ref tn) = tracking_number {
                let provider = tracking_provider.as_deref().unwrap_or("-");
                format!("\n택배사: {}\n운송장번호: {}\n", provider, tn)
            } else {
                String::new()
            };

            let text_body = format!(
                "[Amazing Korean] 교재 주문 상태 변경\n\n{orderer_name}님, 교재 주문의 상태가 변경되었습니다.\n\n주문번호: {order_code}\n상태: {status_label}\n{tracking_text}\n주문 상태 확인: https://amazingkorean.net/textbook/order/{order_code}"
            );
            (subject, html_body, text_body)
        }

        EmailTemplate::EbookPurchaseConfirmation {
            purchase_code,
            language_name,
            edition_label,
            price,
            currency,
        } => {
            let subject = format!("[Amazing Korean] E-book 구매 접수 확인 ({})", purchase_code);
            let html_body = format!(
                r#"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin: 0; padding: 0; font-family: 'Apple SD Gothic Neo', 'Malgun Gothic', sans-serif; background-color: #f5f5f5;">
    <table role="presentation" style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 40px 0;">
                <table role="presentation" style="width: 100%; max-width: 600px; margin: 0 auto; background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="padding: 40px 40px 20px 40px; text-align: center; border-bottom: 1px solid #eee;">
                            <h1 style="margin: 0; color: #333; font-size: 24px;">Amazing Korean</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="margin: 0 0 20px 0; color: #333; font-size: 20px;">E-book 구매가 접수되었습니다</h2>
                            <p style="margin: 0 0 30px 0; color: #666; font-size: 16px; line-height: 1.6;">
                                E-book 구매 주문이 정상적으로 접수되었습니다.
                            </p>
                            <div style="background-color: #f8f9fa; border-radius: 8px; padding: 20px; margin-bottom: 30px;">
                                <p style="margin: 0 0 10px 0; color: #666; font-size: 14px;">
                                    <strong>구매코드:</strong> {purchase_code}
                                </p>
                                <p style="margin: 0 0 10px 0; color: #666; font-size: 14px;">
                                    <strong>교재:</strong> {language_name} ({edition_label})
                                </p>
                                <p style="margin: 0; color: #666; font-size: 14px;">
                                    <strong>금액:</strong> {price} {currency}
                                </p>
                            </div>
                            <h3 style="margin: 0 0 15px 0; color: #333; font-size: 16px;">입금 안내</h3>
                            <div style="background-color: #fff3cd; border-radius: 8px; padding: 20px; margin-bottom: 30px; border: 1px solid #ffc107;">
                                <p style="margin: 0 0 10px 0; color: #856404; font-size: 14px;">
                                    <strong>은행:</strong> 하나은행
                                </p>
                                <p style="margin: 0 0 10px 0; color: #856404; font-size: 14px;">
                                    <strong>계좌번호:</strong> 915-910012-71304
                                </p>
                                <p style="margin: 0; color: #856404; font-size: 14px;">
                                    <strong>예금주:</strong> 주식회사 힘
                                </p>
                            </div>
                            <p style="margin: 0 0 10px 0; color: #999; font-size: 14px;">
                                입금 확인 후 E-book 열람이 가능합니다.
                            </p>
                            <div style="text-align: center; margin-top: 30px;">
                                <a href="https://amazingkorean.net/ebook/my" style="display: inline-block; background-color: #333; color: #ffffff; text-decoration: none; padding: 14px 30px; border-radius: 6px; font-size: 16px; font-weight: bold;">
                                    내 E-book 확인
                                </a>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px 40px; background-color: #f8f9fa; border-radius: 0 0 8px 8px;">
                            <p style="margin: 0; color: #999; font-size: 12px; text-align: center;">
                                © Amazing Korean. All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#
            );
            let text_body = format!(
                "[Amazing Korean] E-book 구매 접수 확인\n\nE-book 구매 주문이 정상적으로 접수되었습니다.\n\n구매코드: {purchase_code}\n교재: {language_name} ({edition_label})\n금액: {price} {currency}\n\n[입금 안내]\n은행: 하나은행\n계좌번호: 915-910012-71304\n예금주: 주식회사 힘\n\n입금 확인 후 E-book 열람이 가능합니다.\n\n내 E-book 확인: https://amazingkorean.net/ebook/my"
            );
            (subject, html_body, text_body)
        }

        EmailTemplate::EbookPurchaseCompleted {
            purchase_code,
            language_name,
            edition_label,
        } => {
            let subject = format!("[Amazing Korean] E-book 결제 완료 — 열람 가능 ({})", purchase_code);
            let html_body = format!(
                r#"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin: 0; padding: 0; font-family: 'Apple SD Gothic Neo', 'Malgun Gothic', sans-serif; background-color: #f5f5f5;">
    <table role="presentation" style="width: 100%; border-collapse: collapse;">
        <tr>
            <td style="padding: 40px 0;">
                <table role="presentation" style="width: 100%; max-width: 600px; margin: 0 auto; background-color: #ffffff; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <tr>
                        <td style="padding: 40px 40px 20px 40px; text-align: center; border-bottom: 1px solid #eee;">
                            <h1 style="margin: 0; color: #333; font-size: 24px;">Amazing Korean</h1>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 40px;">
                            <h2 style="margin: 0 0 20px 0; color: #333; font-size: 20px;">E-book 결제가 완료되었습니다</h2>
                            <p style="margin: 0 0 30px 0; color: #666; font-size: 16px; line-height: 1.6;">
                                결제가 확인되어 E-book을 열람하실 수 있습니다.
                            </p>
                            <div style="background-color: #d4edda; border-radius: 8px; padding: 20px; margin-bottom: 30px; border: 1px solid #28a745;">
                                <p style="margin: 0 0 10px 0; color: #155724; font-size: 14px;">
                                    <strong>구매코드:</strong> {purchase_code}
                                </p>
                                <p style="margin: 0; color: #155724; font-size: 14px;">
                                    <strong>교재:</strong> {language_name} ({edition_label})
                                </p>
                            </div>
                            <div style="text-align: center; margin-top: 30px;">
                                <a href="https://amazingkorean.net/ebook/my" style="display: inline-block; background-color: #333; color: #ffffff; text-decoration: none; padding: 14px 30px; border-radius: 6px; font-size: 16px; font-weight: bold;">
                                    지금 E-book 열람하기
                                </a>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px 40px; background-color: #f8f9fa; border-radius: 0 0 8px 8px;">
                            <p style="margin: 0; color: #999; font-size: 12px; text-align: center;">
                                © Amazing Korean. All rights reserved.
                            </p>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>
</html>"#
            );
            let text_body = format!(
                "[Amazing Korean] E-book 결제 완료\n\n결제가 확인되어 E-book을 열람하실 수 있습니다.\n\n구매코드: {purchase_code}\n교재: {language_name} ({edition_label})\n\n지금 E-book 열람하기: https://amazingkorean.net/ebook/my"
            );
            (subject, html_body, text_body)
        }
    }
}

/// KRW 금액을 쉼표 포맷으로 변환 (예: 250000 → "250,000")
fn format_krw(amount: i32) -> String {
    let s = amount.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
