/// 크레이트 전용 에러 타입.
///
/// 백엔드에서는 `From<CryptoError> for AppError`로 변환.
/// Flutter/Tauri에서는 직접 처리.
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    /// 입력값 형식 오류 (잘못된 암호문 포맷, prefix 누락, 버전 파싱 실패 등)
    #[error("Crypto format error: {0}")]
    InvalidFormat(String),

    /// 복호화 실패 (키 불일치, 데이터 손상 등)
    #[error("Crypto decryption failed: {0}")]
    DecryptionFailed(String),

    /// 내부 오류 (키 초기화 실패, 알고리즘 오류 등)
    #[error("Crypto internal error: {0}")]
    Internal(String),
}

pub type CryptoResult<T> = Result<T, CryptoError>;
