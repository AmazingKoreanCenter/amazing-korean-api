use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHasher,
};

use crate::error::{AppError, AppResult};

pub fn hash(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let params = Params::new(19_456, 2, 1, None).unwrap();
    let argon2 = Argon2::new(Algorithm::Argon2id, argon2::Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("password hash error: {e}")))?
        .to_string();

    Ok(password_hash)
}
