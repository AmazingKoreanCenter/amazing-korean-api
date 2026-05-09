use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};

use crate::error::{AppError, AppResult};

/// 비밀번호 해싱
pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2id 설정 (메모리 19MB, 2 iterations, 1 parallelism)
    let params = Params::new(19_456, 2, 1, None)
        .map_err(|e| AppError::Internal(format!("Failed to create Argon2 params: {}", e)))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))?
        .to_string();

    Ok(password_hash)
}

/// 비밀번호 검증
#[allow(dead_code)]
pub fn verify_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|_| AppError::Internal("Failed to parse password hash".into()))?;

    let valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_returns_argon2id_format() {
        let hash = hash_password("test-password-123").expect("hash failed");
        assert!(
            hash.starts_with("$argon2id$v=19$"),
            "expected argon2id v19 prefix, got: {}",
            hash
        );
    }

    #[test]
    fn test_hash_password_uses_unique_salt_per_call() {
        let h1 = hash_password("same-password").expect("hash failed");
        let h2 = hash_password("same-password").expect("hash failed");
        assert_ne!(h1, h2, "salt should differ between calls");
    }

    #[test]
    fn test_verify_password_returns_true_for_correct_password() {
        let password = "correct-password-456";
        let hash = hash_password(password).expect("hash failed");
        let result = verify_password(password, &hash).expect("verify errored");
        assert!(result, "verification should succeed for correct password");
    }

    #[test]
    fn test_verify_password_returns_false_for_wrong_password() {
        let hash = hash_password("real-password").expect("hash failed");
        let result = verify_password("wrong-password", &hash).expect("verify errored");
        assert!(!result, "verification should fail for incorrect password");
    }

    #[test]
    fn test_verify_password_errors_on_malformed_hash() {
        let result = verify_password("any", "not-a-valid-hash");
        assert!(result.is_err(), "malformed hash should return AppError");
    }

    #[test]
    fn test_hash_and_verify_roundtrip_with_unicode_password() {
        let password = "한글비밀번호🔐";
        let hash = hash_password(password).expect("hash failed");
        let valid = verify_password(password, &hash).expect("verify errored");
        assert!(valid, "unicode password should round-trip");
    }
}
