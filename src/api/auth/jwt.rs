use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::api::auth::dto::AccessTokenRes;
use crate::error::AppResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,    // User ID
    pub session_id: String, // Session ID
    pub exp: i64,    // Expiration time
    pub iat: i64,    // Issued at
    pub iss: String, // Issuer
}

pub fn create_token(
    user_id: i64,
    session_id: &str,
    ttl_minutes: i64,
    secret: &str,
) -> AppResult<AccessTokenRes> {
    let now = OffsetDateTime::now_utc();
    let expires_in = now + Duration::minutes(ttl_minutes);

    let claims = Claims {
        sub: user_id,
        session_id: session_id.to_string(),
        exp: expires_in.unix_timestamp(),
        iat: now.unix_timestamp(),
        iss: "amk".to_string(), // Issuer fixed to "amk" as per context
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| crate::error::AppError::Internal(format!("Failed to encode JWT: {}", e)))?;

    Ok(AccessTokenRes {
        access_token: token,
        expires_in: ttl_minutes * 60,
    })
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
