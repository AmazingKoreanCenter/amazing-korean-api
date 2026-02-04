use aws_sdk_sesv2::{
    config::Region,
    types::{Body, Content, Destination, EmailContent, Message},
    Client,
};

use crate::error::{AppError, AppResult};

/// AWS SES 이메일 클라이언트
#[derive(Clone)]
pub struct EmailClient {
    client: Client,
    from_address: String,
    reply_to: Option<String>,
}

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
}

impl EmailClient {
    /// AWS 설정에서 이메일 클라이언트 생성
    pub async fn new(region: &str, from_address: String, reply_to: Option<String>) -> Self {
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(Region::new(region.to_string()))
            .load()
            .await;

        let client = Client::new(&config);

        Self {
            client,
            from_address,
            reply_to,
        }
    }

    /// 단순 이메일 발송
    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> AppResult<()> {
        let destination = Destination::builder().to_addresses(to).build();

        let subject_content = Content::builder()
            .data(subject)
            .charset("UTF-8")
            .build()
            .map_err(|e| AppError::External(format!("Failed to build email subject: {}", e)))?;
        let html_content = Content::builder()
            .data(html_body)
            .charset("UTF-8")
            .build()
            .map_err(|e| AppError::External(format!("Failed to build email HTML body: {}", e)))?;
        let text_content = Content::builder()
            .data(text_body)
            .charset("UTF-8")
            .build()
            .map_err(|e| AppError::External(format!("Failed to build email text body: {}", e)))?;

        let body = Body::builder()
            .html(html_content)
            .text(text_content)
            .build();

        let message = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();

        let email_content = EmailContent::builder().simple(message).build();

        let mut request = self
            .client
            .send_email()
            .from_email_address(&self.from_address)
            .destination(destination)
            .content(email_content);

        // Reply-To 설정
        if let Some(ref reply_to) = self.reply_to {
            request = request.reply_to_addresses(reply_to);
        }

        request.send().await.map_err(|e| {
            tracing::error!(
                error = %e,
                error_debug = ?e,
                to = %to,
                from = %self.from_address,
                "Failed to send email via SES"
            );
            AppError::External(format!("Failed to send email via SES: {}", e))
        })?;

        tracing::info!(to = %to, subject = %subject, "Email sent successfully");

        Ok(())
    }

    /// 템플릿 기반 이메일 발송
    pub async fn send_templated(&self, to: &str, template: EmailTemplate) -> AppResult<()> {
        let (subject, html_body, text_body) = self.render_template(template);
        self.send_email(to, &subject, &html_body, &text_body).await
    }

    /// 템플릿을 HTML/텍스트로 렌더링
    fn render_template(&self, template: EmailTemplate) -> (String, String, String) {
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
        }
    }
}
