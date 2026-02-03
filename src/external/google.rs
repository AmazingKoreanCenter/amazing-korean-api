use base64::engine::{general_purpose::URL_SAFE_NO_PAD, Engine};
use reqwest::Client;
use serde::Deserialize;

use crate::error::{AppError, AppResult};

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

/// Google OAuth 클라이언트
pub struct GoogleOAuthClient {
    client: Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

/// Google Token 교환 응답
#[derive(Debug, Deserialize)]
#[allow(dead_code)]  // 역직렬화에 필요하지만 id_token만 사용
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
    pub refresh_token: Option<String>,
}

/// Google ID Token Claims
#[derive(Debug, Deserialize)]
pub struct GoogleIdTokenClaims {
    pub iss: String,              // "https://accounts.google.com"
    #[allow(dead_code)]
    pub azp: String,              // authorized party (client_id)
    pub aud: String,              // audience (client_id)
    pub sub: String,              // Google 고유 사용자 ID (핵심!)
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub picture: Option<String>,
    #[allow(dead_code)]
    pub given_name: Option<String>,
    #[allow(dead_code)]
    pub family_name: Option<String>,
    #[allow(dead_code)]
    pub locale: Option<String>,
    #[allow(dead_code)]
    pub iat: i64,
    pub exp: i64,
    pub nonce: Option<String>,    // CSRF/Replay 방지용
}

/// Google 사용자 정보 (간소화된 구조)
#[derive(Debug, Clone)]
#[allow(dead_code)]  // 확장성을 위해 모든 필드 보관
pub struct GoogleUserInfo {
    pub sub: String,              // Google 고유 ID
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub picture: Option<String>,
}

impl GoogleOAuthClient {
    /// 새 Google OAuth 클라이언트 생성
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    /// 1단계: Authorization URL 생성
    /// state: CSRF 방지용 (Redis에 저장)
    /// nonce: ID Token Replay 방지용
    pub fn build_auth_url(&self, state: &str, nonce: &str) -> String {
        let params = [
            ("client_id", self.client_id.as_str()),
            ("redirect_uri", self.redirect_uri.as_str()),
            ("response_type", "code"),
            ("scope", "openid email profile"),
            ("state", state),
            ("nonce", nonce),
            ("access_type", "offline"),
            ("prompt", "consent"),
        ];

        let query = serde_urlencoded::to_string(&params).unwrap_or_default();
        format!("{}?{}", GOOGLE_AUTH_URL, query)
    }

    /// 2단계: Authorization Code → Tokens 교환
    pub async fn exchange_code(&self, code: &str) -> AppResult<GoogleTokenResponse> {
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("redirect_uri", self.redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
            ("code", code),
        ];

        let response = self
            .client
            .post(GOOGLE_TOKEN_URL)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Google token exchange failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::External(format!(
                "Google OAuth error: {} - {}",
                status, body
            )));
        }

        response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Google token parse error: {}", e)))
    }

    /// 3단계: ID Token 디코딩
    /// 주의: 이 함수는 서명 검증을 수행하지 않음 (Google에서 직접 받은 토큰이므로)
    /// 프로덕션에서는 Google Public Keys로 서명 검증 추가 권장
    pub fn decode_id_token(&self, id_token: &str) -> AppResult<GoogleIdTokenClaims> {
        // JWT 형식: header.payload.signature
        let parts: Vec<&str> = id_token.split('.').collect();
        if parts.len() != 3 {
            return Err(AppError::External("Invalid ID token format".into()));
        }

        // payload 디코딩 (Base64 URL-safe)
        let payload = URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|_| AppError::External("Failed to decode ID token payload".into()))?;

        let claims: GoogleIdTokenClaims = serde_json::from_slice(&payload)
            .map_err(|e| AppError::External(format!("Failed to parse ID token: {}", e)))?;

        // 기본 검증: issuer
        if claims.iss != "https://accounts.google.com" && claims.iss != "accounts.google.com" {
            return Err(AppError::External("Invalid ID token issuer".into()));
        }

        // 기본 검증: 만료
        let now = chrono::Utc::now().timestamp();
        if claims.exp < now {
            return Err(AppError::External("ID token expired".into()));
        }

        Ok(claims)
    }

    /// ID Token Claims에서 사용자 정보 추출
    pub fn extract_user_info(&self, claims: &GoogleIdTokenClaims) -> GoogleUserInfo {
        GoogleUserInfo {
            sub: claims.sub.clone(),
            email: claims.email.clone(),
            email_verified: claims.email_verified,
            name: claims.name.clone(),
            picture: claims.picture.clone(),
        }
    }

    /// Client ID getter (audience 검증용)
    #[allow(dead_code)]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_auth_url() {
        let client = GoogleOAuthClient::new(
            "test-client-id".into(),
            "test-secret".into(),
            "http://localhost:3000/callback".into(),
        );

        let url = client.build_auth_url("test-state", "test-nonce");

        assert!(url.starts_with(GOOGLE_AUTH_URL));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
        assert!(url.contains("nonce=test-nonce"));
        assert!(url.contains("scope=openid"));
    }
}
