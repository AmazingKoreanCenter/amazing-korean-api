use reqwest::Client;
use serde::Deserialize;

use crate::error::{AppError, AppResult};

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub(crate) const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
pub(crate) const GOOGLE_JWKS_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";

/// Google OAuth 클라이언트
pub struct GoogleOAuthClient {
    client: Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    /// Token 교환 endpoint URL (default = 공식 Google. test 시 wiremock 으로 override).
    token_url: String,
    /// JWKS endpoint URL (default = 공식 Google. test 시 wiremock 으로 override).
    jwks_url: String,
}

/// Google Token 교환 응답
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // 역직렬화에 필요하지만 id_token만 사용
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
    pub refresh_token: Option<String>,
}

/// Google JWKS response
#[derive(Debug, Deserialize)]
struct GoogleJwks {
    keys: Vec<GoogleJwk>,
}

/// Single JWK from Google's JWKS endpoint
#[derive(Debug, Deserialize)]
struct GoogleJwk {
    kid: String,
    n: String,
    e: String,
}

/// Google ID Token Claims
#[derive(Debug, Deserialize)]
pub struct GoogleIdTokenClaims {
    #[allow(dead_code)]
    pub iss: String, // "https://accounts.google.com" (jsonwebtoken이 검증)
    #[allow(dead_code)]
    pub azp: String, // authorized party (client_id)
    #[allow(dead_code)]
    pub aud: String, // audience (client_id) (jsonwebtoken이 검증)
    pub sub: String, // Google 고유 사용자 ID (핵심!)
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
    #[allow(dead_code)]
    pub exp: i64, // jsonwebtoken이 검증
    pub nonce: Option<String>, // CSRF/Replay 방지용
}

/// Google 사용자 정보 (간소화된 구조)
#[derive(Debug, Clone)]
#[allow(dead_code)] // 확장성을 위해 모든 필드 보관
pub struct GoogleUserInfo {
    pub sub: String, // Google 고유 ID
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub picture: Option<String>,
}

impl GoogleOAuthClient {
    /// 새 Google OAuth 클라이언트 생성 (production = 공식 Google URL).
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> AppResult<Self> {
        Self::with_urls(
            client_id,
            client_secret,
            redirect_uri,
            GOOGLE_TOKEN_URL.to_string(),
            GOOGLE_JWKS_URL.to_string(),
        )
    }

    /// URL override 가능한 생성자. test 환경에서 wiremock URL 주입용.
    ///
    /// B5 Tier 2: builder fail 시 panic 회피 → Result 전파.
    pub fn with_urls(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        token_url: String,
        jwks_url: String,
    ) -> AppResult<Self> {
        // N-10: 외부 서비스 hang 방지 (timeout 15초)
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .map_err(|e| AppError::Internal(format!("google oauth client init: {}", e)))?;
        Ok(Self {
            client,
            client_id,
            client_secret,
            redirect_uri,
            token_url,
            jwks_url,
        })
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

        let query = serde_urlencoded::to_string(params).unwrap_or_default();
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
            .post(&self.token_url)
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

    /// 3단계: ID Token 디코딩 + Google JWKS 서명 검증
    pub async fn decode_id_token(&self, id_token: &str) -> AppResult<GoogleIdTokenClaims> {
        // JWT 헤더에서 kid 추출 (서명 검증에 사용할 키 식별)
        let header = jsonwebtoken::decode_header(id_token)
            .map_err(|e| AppError::External(format!("Invalid ID token header: {}", e)))?;

        let kid = header
            .kid
            .ok_or_else(|| AppError::External("ID token missing kid in header".into()))?;

        // Google JWKS에서 공개키 가져오기
        let jwks: GoogleJwks = self
            .client
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Failed to fetch Google JWKS: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Google JWKS: {}", e)))?;

        // kid에 매칭되는 키 찾기
        let jwk = jwks
            .keys
            .iter()
            .find(|k| k.kid == kid)
            .ok_or_else(|| AppError::External("No matching key found in Google JWKS".into()))?;

        // RSA 공개키로 디코딩 키 생성
        let decoding_key = jsonwebtoken::DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|e| AppError::External(format!("Failed to create decoding key: {}", e)))?;

        // 검증 설정: issuer + audience + 서명
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_issuer(&["https://accounts.google.com", "accounts.google.com"]);
        validation.set_audience(&[&self.client_id]);

        // 서명 검증 + 디코딩
        let token_data =
            jsonwebtoken::decode::<GoogleIdTokenClaims>(id_token, &decoding_key, &validation)
                .map_err(|e| AppError::External(format!("ID token verification failed: {}", e)))?;

        Ok(token_data.claims)
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
        )
        .expect("client init in test");

        let url = client.build_auth_url("test-state", "test-nonce");

        assert!(url.starts_with(GOOGLE_AUTH_URL));
        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("state=test-state"));
        assert!(url.contains("nonce=test-nonce"));
        assert!(url.contains("scope=openid"));
    }
}
