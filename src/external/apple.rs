use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::api::auth::service::OAuthUserInfo;
use crate::error::{AppError, AppResult};

const APPLE_JWKS_URL: &str = "https://appleid.apple.com/auth/keys";
const APPLE_ISSUER: &str = "https://appleid.apple.com";

/// Apple OAuth 클라이언트 (Sign in with Apple — 모바일 ID token 직접 검증).
///
/// AppState 에 `Arc<AppleOAuthClient>` 싱글톤으로 보관해 요청마다 재생성하지 않도록
/// 한다. `reqwest::Client` 는 내부 커넥션 풀을 가지고 있어 재사용이 필수. JWKS 는
/// 토큰 `kid` 기반으로 캐싱해 매 로그인마다 Apple 서버를 때리지 않는다.
pub struct AppleOAuthClient {
    client: Client,
    client_id: String, // Apple Bundle ID (e.g., net.amazingkorean.app)
    /// kid → DecodingKey 캐시. Apple 이 키 로테이션을 해도 토큰 kid 가 바뀌면
    /// cache miss 로 떨어져 자동으로 새 키를 가져온다.
    jwks_cache: Arc<RwLock<HashMap<String, jsonwebtoken::DecodingKey>>>,
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
            jwks_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// kid 에 대응하는 DecodingKey 를 캐시에서 조회하거나, miss 시 Apple JWKS 를
    /// 한 번만 호출해 캐시에 적재한다.
    async fn get_decoding_key(&self, kid: &str) -> AppResult<jsonwebtoken::DecodingKey> {
        // 1차 read-lock 으로 캐시 조회
        {
            let cache = self.jwks_cache.read().await;
            if let Some(key) = cache.get(kid) {
                return Ok(key.clone());
            }
        }

        // Cache miss — JWKS 전체를 가져와 캐시에 모두 적재.
        let jwks: AppleJwks = self.client
            .get(APPLE_JWKS_URL)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Failed to fetch Apple JWKS: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::External(format!("Failed to parse Apple JWKS: {}", e)))?;

        let mut cache = self.jwks_cache.write().await;
        for jwk in &jwks.keys {
            let decoding_key = jsonwebtoken::DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
                .map_err(|e| AppError::External(format!("Failed to create Apple decoding key: {}", e)))?;
            cache.insert(jwk.kid.clone(), decoding_key);
        }

        cache.get(kid)
            .cloned()
            .ok_or_else(|| AppError::External("No matching key found in Apple JWKS".into()))
    }

    /// ID Token 디코딩 + Apple JWKS 서명 검증 (google.rs::decode_id_token 패턴 복제)
    pub async fn decode_id_token(&self, id_token: &str) -> AppResult<AppleIdTokenClaims> {
        // JWT 헤더에서 kid 추출
        let header = jsonwebtoken::decode_header(id_token)
            .map_err(|e| AppError::External(format!("Invalid Apple ID token header: {}", e)))?;

        let kid = header.kid
            .ok_or_else(|| AppError::External("Apple ID token missing kid in header".into()))?;

        // 캐시 우선 조회, miss 면 JWKS 1회 fetch + 전체 적재
        let decoding_key = self.get_decoding_key(&kid).await?;

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
