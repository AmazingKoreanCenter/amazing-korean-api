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

    Ok((
        AccessTokenRes {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: ttl_minutes * 60,
            expires_at: expires_at_str,
        },
        jti,
    ))
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &str = "unit-test-secret-do-not-use-in-prod";

    #[test]
    fn test_create_decode_roundtrip_preserves_claims() {
        let (token_res, jti) =
            create_token(42, "sess-abc", UserAuth::Learner, 30, TEST_SECRET).expect("create");

        assert_eq!(token_res.token_type, "Bearer");
        assert_eq!(token_res.expires_in, 30 * 60);
        assert!(!token_res.access_token.is_empty());

        let claims = decode_token(&token_res.access_token, TEST_SECRET).expect("decode");
        assert_eq!(claims.sub, 42);
        assert_eq!(claims.session_id, "sess-abc");
        assert_eq!(claims.role, UserAuth::Learner);
        assert_eq!(claims.jti, jti);
        assert_eq!(claims.iss, "amk");
        assert!(claims.exp > claims.iat, "exp must be after iat");
    }

    #[test]
    fn test_decode_token_rejects_wrong_secret() {
        let (token_res, _) =
            create_token(1, "s", UserAuth::Admin, 30, TEST_SECRET).expect("create");
        let result = decode_token(&token_res.access_token, "different-secret");
        assert!(result.is_err(), "wrong secret must fail signature check");
    }

    #[test]
    fn test_decode_token_rejects_malformed_input() {
        let result = decode_token("not.a.jwt", TEST_SECRET);
        assert!(result.is_err(), "malformed token must error");
    }

    #[test]
    fn test_decode_token_rejects_expired_token() {
        // ttl_minutes=0 = 즉시 만료. jsonwebtoken Validation default leeway=60s 이므로
        // 충분히 과거로 가야 한다 → 음수 ttl 사용
        let (token_res, _) =
            create_token(1, "s", UserAuth::Learner, -120, TEST_SECRET).expect("create");
        let result = decode_token(&token_res.access_token, TEST_SECRET);
        assert!(result.is_err(), "expired token must fail validation");
    }

    #[test]
    fn test_create_token_generates_unique_jti_per_call() {
        let (_, jti1) = create_token(1, "s", UserAuth::Learner, 30, TEST_SECRET).expect("create");
        let (_, jti2) = create_token(1, "s", UserAuth::Learner, 30, TEST_SECRET).expect("create");
        assert_ne!(jti1, jti2, "jti must be unique per token");
    }

    #[test]
    fn test_create_token_ttl_minutes_matches_expires_in_seconds() {
        let (token_res, _) =
            create_token(1, "s", UserAuth::Learner, 90, TEST_SECRET).expect("create");
        assert_eq!(token_res.expires_in, 5400, "90 min = 5400 sec");
    }

    #[test]
    fn test_create_token_includes_rfc3339_expires_at() {
        let (token_res, _) =
            create_token(1, "s", UserAuth::Learner, 30, TEST_SECRET).expect("create");
        // RFC3339 = "YYYY-MM-DDTHH:MM:SS...Z" 형식
        assert!(
            token_res.expires_at.contains('T') && token_res.expires_at.ends_with('Z'),
            "expires_at must be RFC3339, got: {}",
            token_res.expires_at
        );
    }
}
