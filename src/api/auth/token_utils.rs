use base64::engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD};
use base64::Engine as _;
use percent_encoding::percent_decode_str;
use rand::RngCore;
use uuid::Uuid;

use crate::error::AppError;

pub fn parse_refresh_token_bytes(s: &str) -> Result<Vec<u8>, AppError> {
    // 0) URL 디코딩
    let decoded = percent_decode_str(s)
        .decode_utf8()
        .map_err(|_| AppError::Unauthorized("Invalid refresh token format".into()))?;
    let ss = decoded.as_ref();

    // 1) UUID 허용
    if let Ok(u) = Uuid::parse_str(ss) {
        return Ok(u.as_bytes().to_vec());
    }
    // 2) base64url no-pad
    if let Ok(b) = URL_SAFE_NO_PAD.decode(ss) {
        return Ok(b);
    }
    // 3) base64url with pad
    if let Ok(b) = URL_SAFE.decode(ss) {
        return Ok(b);
    }
    // 4) 일반 base64
    if let Ok(b) = STANDARD.decode(ss) {
        return Ok(b);
    }

    Err(AppError::Unauthorized(
        "Invalid refresh token format".into(),
    ))
}

pub fn generate_refresh_cookie_value() -> (String, [u8; 32]) {
    let mut raw = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut raw);
    let cookie_val = URL_SAFE_NO_PAD.encode(raw);
    (cookie_val, raw)
}
