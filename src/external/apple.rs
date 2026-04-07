use reqwest::Client;
use serde::Deserialize;

use crate::api::auth::service::OAuthUserInfo;
use crate::error::{AppError, AppResult};

const APPLE_JWKS_URL: &str = "https://appleid.apple.com/auth/keys";
const APPLE_ISSUER: &str = "https://appleid.apple.com";

/// Apple OAuth 클라이언트 (Sign in with Apple — 모바일 ID token 직접 검증)
pub struct AppleOAuthClient {
    client: Client,
    client_id: String, // Apple Bundle ID (e.g., net.amazingkorean.app)
}

/// Apple JWKS response
#[derive(Debug, Deserialize)]
struct AppleJwks {
    keys: Vec<AppleJwk>,
}

/// Single JWK from Apple's JWKS endpoint
#[derive(Debug, Deserialize)]
struct AppleJwk {
    kid: String,
    n: String,
    e: String,
}

/// Apple ID Token Claims
/// Apple 특이사항: email은 최초 인증에만 제공, email_verified는 문자열("true"/"false")
#[derive(Debug, Deserialize)]
pub struct AppleIdTokenClaims {
    #[allow(dead_code)]
    pub iss: String,                       // "https://appleid.apple.com" (jsonwebtoken이 검증)
    #[allow(dead_code)]
    pub aud: String,                       // audience (Bundle ID) (jsonwebtoken이 검증)
    pub sub: String,                       // Apple 고유 사용자 ID (team별 안정적, 고유)
    pub email: Option<String>,             // 최초 인증에만 제공!
    pub email_verified: Option<String>,    // Apple은 bool이 아닌 "true"/"false" 문자열
    #[allow(dead_code)]
    pub iat: i64,
    #[allow(dead_code)]
    pub exp: i64,                          // jsonwebtoken이 검증
}

impl AppleOAuthClient {
    pub fn new(client_id: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
        }
    }

    /// ID Token 디코딩 + Apple JWKS 서명 검증 (google.rs::decode_id_token 패턴 복제)
    pub async fn decode_id_token(&self, id_token: &str) -> AppResult<AppleIdTokenClaims> {
        // JWT 헤더에서 kid 추출
        let header = jsonwebtoken::decode_header(id_token)
            .map_err(|e| AppError::External(format!("Invalid Apple ID token header: {}", e)))?;

        let kid = header.kid
            .ok_or_else(|| AppError::External("Apple ID token missing kid in header".into()))?;

        // Apple JWKS에서 공개키 가져오기
        let jwks: AppleJwks = self.client
            .get(APPLE_JWKS_URL)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Failed to fetch Apple JWKS: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Apple JWKS: {}", e)))?;

        // kid에 매칭되는 키 찾기
        let jwk = jwks.keys.iter()
            .find(|k| k.kid == kid)
            .ok_or_else(|| AppError::External("No matching key found in Apple JWKS".into()))?;

        // RSA 공개키로 디코딩 키 생성
        let decoding_key = jsonwebtoken::DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|e| AppError::External(format!("Failed to create Apple decoding key: {}", e)))?;

        // 검증 설정: issuer + audience + 서명
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_issuer(&[APPLE_ISSUER]);
        validation.set_audience(&[&self.client_id]);

        // 서명 검증 + 디코딩
        let token_data = jsonwebtoken::decode::<AppleIdTokenClaims>(id_token, &decoding_key, &validation)
            .map_err(|e| AppError::External(format!("Apple ID token verification failed: {}", e)))?;

        Ok(token_data.claims)
    }

    /// Claims에서 OAuthUserInfo 추출
    /// user_name: 클라이언트가 캐싱한 이름 (Apple은 최초에만 제공)
    pub fn extract_user_info(&self, claims: &AppleIdTokenClaims, user_name: Option<String>) -> OAuthUserInfo {
        let email_verified = claims.email_verified.as_deref() == Some("true");

        OAuthUserInfo {
            sub: claims.sub.clone(),
            email: claims.email.clone().unwrap_or_default(),
            email_verified,
            name: user_name,
            picture: None, // Apple은 프로필 사진 미제공
        }
    }
}
