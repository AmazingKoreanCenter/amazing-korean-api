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