use serde::{Deserialize, Serialize};
use jsonwebtoken::{Header, EncodingKey, DecodingKey, Validation, Algorithm, encode, decode};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,     // user_id
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
}

fn secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_super_secret_change_me".to_string())
}

fn ttl_hours() -> i64 {
    std::env::var("JWT_EXPIRE_HOURS").ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(24 * 7)
}

pub fn encode_token(user_id: i64) -> anyhow::Result<(String, i64)> {
    let now = OffsetDateTime::now_utc();
    let exp = now + Duration::hours(ttl_hours());

    let claims = Claims {
        sub: user_id,
        iat: now.unix_timestamp(),
        exp: exp.unix_timestamp(),
        iss: "amazing-korean-api".to_string(),
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret().as_bytes()),
    )?;
    Ok((token, (exp - now).whole_seconds()))
}

pub fn decode_token(token: &str) -> anyhow::Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data.claims)
}
