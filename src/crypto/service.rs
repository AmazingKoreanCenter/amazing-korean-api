use std::collections::HashMap;

use crate::error::{AppError, AppResult};

use super::{blind_index, cipher};

/// 다중 키 버전을 관리하는 키링.
///
/// - `current_version` 키는 반드시 존재해야 함 (신규 암호화에 사용)
/// - 과거 버전 키는 복호화가 필요한 동안만 유지 (rekey 완료 후 제거 가능)
#[derive(Clone)]
pub struct KeyRing {
    keys: HashMap<u8, [u8; 32]>,
    current_version: u8,
}

impl KeyRing {
    pub fn new(keys: HashMap<u8, [u8; 32]>, current_version: u8) -> AppResult<Self> {
        if current_version < 1 {
            return Err(AppError::Internal(
                "KeyRing: current_version must be >= 1".into(),
            ));
        }
        if !keys.contains_key(&current_version) {
            return Err(AppError::Internal(format!(
                "KeyRing: current_version v{} key not found in provided keys",
                current_version
            )));
        }
        Ok(Self {
            keys,
            current_version,
        })
    }

    pub fn current_key(&self) -> &[u8; 32] {
        &self.keys[&self.current_version]
    }

    pub fn current_version(&self) -> u8 {
        self.current_version
    }

    pub fn get_key(&self, version: u8) -> AppResult<&[u8; 32]> {
        self.keys.get(&version).ok_or_else(|| {
            AppError::Internal(format!(
                "Unknown encryption key version: v{} (available: {:?})",
                version,
                self.keys.keys().collect::<Vec<_>>()
            ))
        })
    }
}

impl std::fmt::Debug for KeyRing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyRing")
            .field("current_version", &self.current_version)
            .field("available_versions", &self.keys.keys().collect::<Vec<_>>())
            .finish()
    }
}

/// 애플리케이션 레벨 암호화 서비스.
///
/// KeyRing에서 키를 읽어 encrypt/decrypt/blind_index를 제공.
/// 키가 없으면 에러 (암호화 필수).
pub struct CryptoService<'a> {
    encryption: &'a KeyRing,
    hmac_key: &'a [u8; 32],
}

impl<'a> CryptoService<'a> {
    pub fn new(ring: &'a KeyRing, hmac_key: &'a [u8; 32]) -> Self {
        Self {
            encryption: ring,
            hmac_key,
        }
    }

    /// 필드 값을 현재 버전 키로 암호화한다.
    pub fn encrypt(&self, plaintext: &str, aad: &str) -> AppResult<String> {
        let key = self.encryption.current_key();
        let ver = self.encryption.current_version();
        cipher::encrypt(key, ver, plaintext, aad)
    }

    /// 필드 값을 복호화한다. 암호문에 포함된 버전으로 적절한 키를 선택.
    pub fn decrypt(&self, value: &str, aad: &str) -> AppResult<String> {
        let ver = cipher::extract_version(value)?;
        let key = self.encryption.get_key(ver)?;
        cipher::decrypt(key, value, aad)
    }

    /// 레거시 호환 복호화 — 평문/암호문/손상 혼재 상황에서 안전하게 처리.
    ///
    /// - 평문 (enc:v 로 시작하지 않음) → 그대로 반환
    /// - 유효 암호문 → decrypt 후 반환
    /// - 손상된 암호문 (enc:v 로 시작하지만 깨짐) → `"<decryption_failed>"` 반환 (500 방지)
    ///
    /// `has_enc_prefix()` (느슨한 검사)를 사용하여 깨진 문자열이 평문으로 노출되는 것을 방지.
    pub fn try_decrypt_or_plaintext(&self, value: &str, aad: &str) -> String {
        if cipher::has_enc_prefix(value) {
            self.decrypt(value, aad)
                .unwrap_or_else(|_| "<decryption_failed>".to_string())
        } else {
            value.to_string()
        }
    }

    /// Blind index를 계산한다 (trim + lowercase 정규화).
    pub fn blind_index(&self, plaintext: &str) -> AppResult<String> {
        blind_index::compute_blind_index(self.hmac_key, plaintext)
    }

    /// Case-sensitive blind index를 계산한다 (trim만 적용).
    pub fn blind_index_preserve_case(&self, plaintext: &str) -> AppResult<String> {
        blind_index::compute_blind_index_preserve_case(self.hmac_key, plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_keys() -> (KeyRing, [u8; 32]) {
        let key_v1: [u8; 32] = [0x42; 32];
        let hmac_key: [u8; 32] = [0xAB; 32];
        let mut keys = HashMap::new();
        keys.insert(1, key_v1);
        let ring = KeyRing::new(keys, 1).unwrap();
        (ring, hmac_key)
    }

    fn test_keys_v1_v2() -> (KeyRing, [u8; 32]) {
        let key_v1: [u8; 32] = [0x42; 32];
        let key_v2: [u8; 32] = [0x43; 32];
        let hmac_key: [u8; 32] = [0xAB; 32];
        let mut keys = HashMap::new();
        keys.insert(1, key_v1);
        keys.insert(2, key_v2);
        let ring = KeyRing::new(keys, 2).unwrap();
        (ring, hmac_key)
    }

    // --- KeyRing 테스트 ---

    #[test]
    fn test_keyring_current_key() {
        let (ring, _) = test_keys();
        assert_eq!(ring.current_version(), 1);
        assert_eq!(ring.current_key(), &[0x42; 32]);
    }

    #[test]
    fn test_keyring_get_key_missing_version() {
        let (ring, _) = test_keys();
        assert!(ring.get_key(2).is_err());
    }

    #[test]
    fn test_keyring_version_zero_rejected() {
        let mut keys = HashMap::new();
        keys.insert(0, [0x42; 32]);
        assert!(KeyRing::new(keys, 0).is_err());
    }

    #[test]
    fn test_keyring_missing_current_version() {
        let mut keys = HashMap::new();
        keys.insert(1, [0x42; 32]);
        assert!(KeyRing::new(keys, 2).is_err());
    }

    // --- CryptoService encrypt/decrypt 테스트 ---

    #[test]
    fn test_encrypt_decrypt_v1() {
        let (ring, hmac_key) = test_keys();
        let crypto = CryptoService::new(&ring, &hmac_key);

        let encrypted = crypto.encrypt("test@example.com", "users.user_email").unwrap();
        assert!(encrypted.starts_with("enc:v1:"));

        let decrypted = crypto.decrypt(&encrypted, "users.user_email").unwrap();
        assert_eq!(decrypted, "test@example.com");
    }

    #[test]
    fn test_v1_data_decrypted_with_v1_v2_ring() {
        // v1으로 암호화 → v1+v2 KeyRing으로 복호화 OK
        let (ring_v1, hmac_key) = test_keys();
        let crypto_v1 = CryptoService::new(&ring_v1, &hmac_key);
        let encrypted = crypto_v1.encrypt("secret", "test.aad").unwrap();

        let (ring_v1v2, _) = test_keys_v1_v2();
        let crypto_v1v2 = CryptoService::new(&ring_v1v2, &hmac_key);
        let decrypted = crypto_v1v2.decrypt(&encrypted, "test.aad").unwrap();
        assert_eq!(decrypted, "secret");
    }

    #[test]
    fn test_v2_encrypt_with_multi_ring() {
        let (ring, hmac_key) = test_keys_v1_v2();
        let crypto = CryptoService::new(&ring, &hmac_key);

        let encrypted = crypto.encrypt("hello", "test.aad").unwrap();
        assert!(encrypted.starts_with("enc:v2:")); // 최신 버전으로 암호화

        let decrypted = crypto.decrypt(&encrypted, "test.aad").unwrap();
        assert_eq!(decrypted, "hello");
    }

    #[test]
    fn test_v1_encrypted_fails_with_v2_only_ring() {
        // v1으로 암호화 → v2만 있는 KeyRing으로 복호화 실패
        let (ring_v1, hmac_key) = test_keys();
        let crypto_v1 = CryptoService::new(&ring_v1, &hmac_key);
        let encrypted = crypto_v1.encrypt("secret", "test.aad").unwrap();

        let mut keys_v2_only = HashMap::new();
        keys_v2_only.insert(2, [0x43u8; 32]);
        let ring_v2_only = KeyRing::new(keys_v2_only, 2).unwrap();
        let crypto_v2_only = CryptoService::new(&ring_v2_only, &hmac_key);

        assert!(crypto_v2_only.decrypt(&encrypted, "test.aad").is_err());
    }

    // --- try_decrypt_or_plaintext 테스트 ---

    #[test]
    fn test_try_decrypt_plaintext() {
        let (ring, hmac_key) = test_keys();
        let crypto = CryptoService::new(&ring, &hmac_key);

        assert_eq!(crypto.try_decrypt_or_plaintext("192.168.1.1", "aad"), "192.168.1.1");
        assert_eq!(crypto.try_decrypt_or_plaintext("user@example.com", "aad"), "user@example.com");
    }

    #[test]
    fn test_try_decrypt_valid_encrypted() {
        let (ring, hmac_key) = test_keys();
        let crypto = CryptoService::new(&ring, &hmac_key);

        let encrypted = crypto.encrypt("10.0.0.1", "admin_action_log.ip_address").unwrap();
        let result = crypto.try_decrypt_or_plaintext(&encrypted, "admin_action_log.ip_address");
        assert_eq!(result, "10.0.0.1");
    }

    #[test]
    fn test_try_decrypt_corrupted_enc_prefix() {
        let (ring, hmac_key) = test_keys();
        let crypto = CryptoService::new(&ring, &hmac_key);

        // 손상된 암호문 → "<decryption_failed>"
        assert_eq!(
            crypto.try_decrypt_or_plaintext("enc:v1:corrupted_base64!!!", "aad"),
            "<decryption_failed>"
        );
        // 구분자 누락
        assert_eq!(
            crypto.try_decrypt_or_plaintext("enc:v1", "aad"),
            "<decryption_failed>"
        );
        // 비숫자 버전
        assert_eq!(
            crypto.try_decrypt_or_plaintext("enc:vabc:data", "aad"),
            "<decryption_failed>"
        );
    }
}
