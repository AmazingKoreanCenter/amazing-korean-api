use std::fmt::Write;

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::{AppError, AppResult};

type HmacSha256 = Hmac<Sha256>;

/// 검색 가능한 Blind Index를 생성한다 (HMAC-SHA256).
///
/// - 입력을 lowercase + trim으로 정규화 (대소문자 무관 검색 보장)
/// - 출력: 64자 hex 문자열 (256비트)
/// - 동일 입력 → 항상 동일 출력 (결정적)
/// - 이메일, 이름 등 대소문자 구분 불필요한 필드에 사용
pub fn compute_blind_index(key: &[u8; 32], plaintext: &str) -> AppResult<String> {
    let normalized = plaintext.trim().to_lowercase();
    compute_hmac(key, &normalized)
}

/// Case-sensitive Blind Index 생성 (trim만 적용, lowercase 안 함).
///
/// - OAuth subject 등 case-sensitive 식별자에 사용
/// - Google subject는 숫자라 현재 영향 없지만, 다른 provider 대비
pub fn compute_blind_index_preserve_case(key: &[u8; 32], plaintext: &str) -> AppResult<String> {
    let trimmed = plaintext.trim();
    compute_hmac(key, trimmed)
}

fn compute_hmac(key: &[u8; 32], input: &str) -> AppResult<String> {
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| AppError::Internal(format!("HMAC key init failed: {e}")))?;

    mac.update(input.as_bytes());
    let result = mac.finalize().into_bytes();

    let mut hex_string = String::with_capacity(64);
    for byte in result {
        write!(&mut hex_string, "{:02x}", byte)
            .expect("Writing to String should not fail");
    }

    Ok(hex_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; 32] {
        [0xAB; 32]
    }

    #[test]
    fn test_deterministic() {
        let key = test_key();
        let idx1 = compute_blind_index(&key, "test@example.com").unwrap();
        let idx2 = compute_blind_index(&key, "test@example.com").unwrap();
        assert_eq!(idx1, idx2);
    }

    #[test]
    fn test_output_length() {
        let key = test_key();
        let idx = compute_blind_index(&key, "test@example.com").unwrap();
        assert_eq!(idx.len(), 64); // 256 bits = 64 hex chars
    }

    #[test]
    fn test_case_insensitive() {
        let key = test_key();
        let idx1 = compute_blind_index(&key, "Test@Example.COM").unwrap();
        let idx2 = compute_blind_index(&key, "test@example.com").unwrap();
        assert_eq!(idx1, idx2);
    }

    #[test]
    fn test_trim_whitespace() {
        let key = test_key();
        let idx1 = compute_blind_index(&key, "  test@example.com  ").unwrap();
        let idx2 = compute_blind_index(&key, "test@example.com").unwrap();
        assert_eq!(idx1, idx2);
    }

    #[test]
    fn test_different_inputs_different_indexes() {
        let key = test_key();
        let idx1 = compute_blind_index(&key, "user1@example.com").unwrap();
        let idx2 = compute_blind_index(&key, "user2@example.com").unwrap();
        assert_ne!(idx1, idx2);
    }

    #[test]
    fn test_different_keys_different_indexes() {
        let key1: [u8; 32] = [0xAB; 32];
        let key2: [u8; 32] = [0xCD; 32];
        let idx1 = compute_blind_index(&key1, "test@example.com").unwrap();
        let idx2 = compute_blind_index(&key2, "test@example.com").unwrap();
        assert_ne!(idx1, idx2);
    }

    #[test]
    fn test_hex_format() {
        let key = test_key();
        let idx = compute_blind_index(&key, "test@example.com").unwrap();
        // 모든 문자가 hex 문자인지 확인
        assert!(idx.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_preserve_case_differs_from_normalized() {
        let key = test_key();
        let idx_normalized = compute_blind_index(&key, "AbCdEf").unwrap();
        let idx_preserve = compute_blind_index_preserve_case(&key, "AbCdEf").unwrap();
        // 대소문자가 섞인 입력은 두 함수의 결과가 달라야 함
        assert_ne!(idx_normalized, idx_preserve);
    }

    #[test]
    fn test_preserve_case_same_for_lowercase() {
        let key = test_key();
        let idx_normalized = compute_blind_index(&key, "abcdef").unwrap();
        let idx_preserve = compute_blind_index_preserve_case(&key, "abcdef").unwrap();
        // 이미 소문자인 입력은 두 함수의 결과가 같아야 함
        assert_eq!(idx_normalized, idx_preserve);
    }

    #[test]
    fn test_preserve_case_numeric_subject() {
        let key = test_key();
        // Google OAuth subject는 순수 숫자 → 두 함수 결과 동일
        let idx1 = compute_blind_index(&key, "107712981768667758409").unwrap();
        let idx2 = compute_blind_index_preserve_case(&key, "107712981768667758409").unwrap();
        assert_eq!(idx1, idx2);
    }
}
