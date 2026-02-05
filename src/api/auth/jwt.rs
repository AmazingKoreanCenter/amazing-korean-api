use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use uuid::Uuid;

use crate::api::auth::dto::AccessTokenRes;
use crate::error::{AppError, AppResult};
use crate::types::UserAuth;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,           // User ID
    pub session_id: String, // Session ID
    pub role: UserAuth,     // User Role (HYMN, Admin, Manager, Learner)
    pub jti: String,        // JWT Token ID (unique per token)
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
    pub iss: String,        // Issuer
}

/// create_token 반환값: (AccessTokenRes, jti)
pub fn create_token(
    user_id: i64,
    session_id: &str,
    role: UserAuth,
    ttl_minutes: i64,
    secret: &str,
) -> AppResult<(AccessTokenRes, String)> {
    let now = OffsetDateTime::now_utc();
    let duration = Duration::minutes(ttl_minutes);
    let expires_in_dt = now + duration;
    let jti = Uuid::new_v4().to_string();

    let claims = Claims {
        sub: user_id,
        session_id: session_id.to_string(),
        role,
        jti: jti.clone(),
        exp: expires_in_dt.unix_timestamp(),
        iat: now.unix_timestamp(),
        iss: "amk".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Failed to encode JWT: {}", e)))?;

    // ISO 8601 포맷 문자열 생성 (프론트엔드 편의용)
    let expires_at_str = expires_in_dt
        .format(&Rfc3339)
        .map_err(|e| AppError::Internal(format!("Failed to format date: {}", e)))?;

    Ok((AccessTokenRes {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: ttl_minutes * 60,
        expires_at: expires_at_str,
    }, jti))
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}