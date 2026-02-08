use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, AeadCore, Nonce,
};
use base64::engine::{general_purpose::STANDARD, Engine};

use crate::error::{AppError, AppResult};

/// AES-256-GCM으로 평문을 암호화한다.
///
/// - `key`: 32바이트 AES-256 키
/// - `version`: 키 버전 (1~255)
/// - `plaintext`: 암호화할 평문
/// - `aad`: Associated Authenticated Data (예: "users.user_email") — ciphertext 스왑 방지
///
/// 출력 포맷: `enc:v{version}:` + base64(nonce_12bytes || ciphertext || tag_16bytes)
pub fn encrypt(key: &[u8; 32], version: u8, plaintext: &str, aad: &str) -> AppResult<String> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| AppError::Internal(format!("AES key init failed: {e}")))?;

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let payload = aes_gcm::aead::Payload {
        msg: plaintext.as_bytes(),
        aad: aad.as_bytes(),
    };

    let ciphertext = cipher
        .encrypt(&nonce, payload)
        .map_err(|e| AppError::Internal(format!("AES encrypt failed: {e}")))?;

    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(nonce.as_slice());
    combined.extend_from_slice(&ciphertext);

    Ok(format!("enc:v{}:{}", version, STANDARD.encode(&combined)))
}

/// AES-256-GCM으로 암호문을 복호화한다.
///
/// - `key`: 32바이트 AES-256 키 (버전에 맞는 키를 외부에서 제공)
/// - `encrypted`: `enc:v{version}:` + base64(nonce || ciphertext || tag)
/// - `aad`: 암호화 시 사용한 동일한 AAD
pub fn decrypt(key: &[u8; 32], encrypted: &str, aad: &str) -> AppResult<String> {
    // "enc:v{digits}:" 이후의 base64 부분 추출
    let b64 = strip_enc_prefix(encrypted)?;

    let combined = STANDARD
        .decode(b64)
        .map_err(|e| AppError::Internal(format!("Base64 decode failed: {e}")))?;

    if combined.len() < 12 + 16 {
        return Err(AppError::Internal("Encrypted data too short".into()));
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| AppError::Internal(format!("AES key init failed: {e}")))?;

    let payload = aes_gcm::aead::Payload {
        msg: ciphertext,
        aad: aad.as_bytes(),
    };

    let plaintext = cipher
        .decrypt(nonce, payload)
        .map_err(|e| AppError::Internal(format!("AES decrypt failed: {e}")))?;

    String::from_utf8(plaintext)
        .map_err(|e| AppError::Internal(format!("UTF-8 decode failed: {e}")))
}

/// 암호문에서 키 버전을 추출한다 — 엄격한 파싱.
///
/// 형식: `enc:v<digits>:<base64>`
/// - "enc:v" 접두사 없음 → 에러
/// - 버전 숫자 파싱 실패 (비숫자, u8 오버플로, 빈 문자열) → 에러
/// - ":" 구분자 누락 → 에러
pub fn extract_version(encrypted: &str) -> AppResult<u8> {
    let rest = encrypted
        .strip_prefix("enc:v")
        .ok_or_else(|| AppError::Internal("Not an encrypted value (missing 'enc:v' prefix)".into()))?;

    let colon_pos = rest
        .find(':')
        .ok_or_else(|| AppError::Internal("Corrupted encrypted value (missing ':' after version)".into()))?;

    let version_str = &rest[..colon_pos];
    if version_str.is_empty() {
        return Err(AppError::Internal("Corrupted encrypted value (empty version number)".into()));
    }

    version_str.parse::<u8>().map_err(|e| {
        AppError::Internal(format!("Corrupted encrypted value (invalid version '{version_str}'): {e}"))
    })
}

/// 암호문의 `enc:v{digits}:` prefix를 제거하고 base64 부분만 반환.
fn strip_enc_prefix(encrypted: &str) -> AppResult<&str> {
    let rest = encrypted
        .strip_prefix("enc:v")
        .ok_or_else(|| AppError::Internal("Not an encrypted value (missing prefix)".into()))?;

    let colon_pos = rest
        .find(':')
        .ok_or_else(|| AppError::Internal("Corrupted encrypted value (missing ':' delimiter)".into()))?;

    Ok(&rest[colon_pos + 1..])
}

/// 값이 유효한 암호화 프리픽스를 가지고 있는지 확인 (엄격).
/// `extract_version()` 성공 여부로 판정.
pub fn is_encrypted(value: &str) -> bool {
    extract_version(value).is_ok()
}

/// 값이 `enc:v` 로 시작하는지 확인 (느슨한 prefix 검사).
/// 손상된 암호문("enc:v1" 구분자 누락 등)도 true를 반환하므로,
/// `try_decrypt_or_plaintext()`에서 사용하여 깨진 문자열이 평문으로 노출되는 것을 방지.
pub fn has_enc_prefix(value: &str) -> bool {
    value.starts_with("enc:v")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; 32] {
        [0x42; 32]
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = test_key();
        let plaintext = "test@example.com";
        let aad = "users.user_email";

        let encrypted = encrypt(&key, 1, plaintext, aad).unwrap();
        assert!(encrypted.starts_with("enc:v1:"));
        assert_ne!(encrypted, plaintext);

        let decrypted = decrypt(&key, &encrypted, aad).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_v2() {
        let key = test_key();
        let encrypted = encrypt(&key, 2, "hello", "test.aad").unwrap();
        assert!(encrypted.starts_with("enc:v2:"));
        let decrypted = decrypt(&key, &encrypted, "test.aad").unwrap();
        assert_eq!(decrypted, "hello");
    }

    #[test]
    fn test_encrypt_produces_different_ciphertext() {
        let key = test_key();
        let plaintext = "test@example.com";
        let aad = "users.user_email";

        let enc1 = encrypt(&key, 1, plaintext, aad).unwrap();
        let enc2 = encrypt(&key, 1, plaintext, aad).unwrap();
        assert_ne!(enc1, enc2); // 랜덤 nonce로 다른 ciphertext
    }

    #[test]
    fn test_aad_mismatch_fails_decrypt() {
        let key = test_key();
        let plaintext = "test@example.com";

        let encrypted = encrypt(&key, 1, plaintext, "users.user_email").unwrap();
        let result = decrypt(&key, &encrypted, "user_oauth.oauth_email");
        assert!(result.is_err()); // AAD 불일치 시 복호화 실패
    }

    #[test]
    fn test_wrong_key_fails_decrypt() {
        let key1: [u8; 32] = [0x42; 32];
        let key2: [u8; 32] = [0x43; 32];
        let aad = "users.user_email";

        let encrypted = encrypt(&key1, 1, "secret", aad).unwrap();
        assert!(decrypt(&key2, &encrypted, aad).is_err());
    }

    #[test]
    fn test_empty_string_encrypt() {
        let key = test_key();
        let aad = "users.user_email";

        let encrypted = encrypt(&key, 1, "", aad).unwrap();
        let decrypted = decrypt(&key, &encrypted, aad).unwrap();
        assert_eq!(decrypted, "");
    }

    #[test]
    fn test_unicode_encrypt() {
        let key = test_key();
        let aad = "users.user_name";

        let encrypted = encrypt(&key, 1, "홍길동", aad).unwrap();
        let decrypted = decrypt(&key, &encrypted, aad).unwrap();
        assert_eq!(decrypted, "홍길동");
    }

    // --- extract_version 테스트 ---

    #[test]
    fn test_extract_version_v1() {
        assert_eq!(extract_version("enc:v1:somebase64data").unwrap(), 1);
    }

    #[test]
    fn test_extract_version_v2() {
        assert_eq!(extract_version("enc:v2:somebase64data").unwrap(), 2);
    }

    #[test]
    fn test_extract_version_v255() {
        assert_eq!(extract_version("enc:v255:data").unwrap(), 255);
    }

    #[test]
    fn test_extract_version_invalid_no_prefix() {
        assert!(extract_version("plaintext").is_err());
    }

    #[test]
    fn test_extract_version_overflow() {
        // u8 최대 255, 999는 오버플로
        assert!(extract_version("enc:v999:data").is_err());
    }

    #[test]
    fn test_extract_version_empty_number() {
        assert!(extract_version("enc:v:data").is_err());
    }

    #[test]
    fn test_extract_version_non_digit() {
        assert!(extract_version("enc:vabc:data").is_err());
    }

    #[test]
    fn test_extract_version_missing_colon() {
        // "enc:v1" — 버전 뒤 ':' 누락
        assert!(extract_version("enc:v1").is_err());
    }

    // --- is_encrypted / has_enc_prefix 테스트 ---

    #[test]
    fn test_is_encrypted_valid() {
        let key = test_key();
        let encrypted = encrypt(&key, 1, "test", "aad").unwrap();
        assert!(is_encrypted(&encrypted));
    }

    #[test]
    fn test_is_encrypted_plaintext() {
        assert!(!is_encrypted("192.168.1.1"));
        assert!(!is_encrypted("user@example.com"));
    }

    #[test]
    fn test_is_encrypted_non_digit_version() {
        // 엄격 검사: 버전이 숫자가 아니면 false
        assert!(!is_encrypted("enc:vabc:data"));
    }

    #[test]
    fn test_has_enc_prefix_corrupted() {
        // 느슨 검사: "enc:v"로 시작하면 true (손상된 데이터 포함)
        assert!(has_enc_prefix("enc:v1"));         // ':' 누락
        assert!(has_enc_prefix("enc:vabc:data"));  // 숫자 아님
        assert!(has_enc_prefix("enc:v1:corrupted_base64"));
    }

    #[test]
    fn test_has_enc_prefix_plaintext() {
        assert!(!has_enc_prefix("192.168.1.1"));
        assert!(!has_enc_prefix("user@example.com"));
        assert!(!has_enc_prefix(""));
    }
}
