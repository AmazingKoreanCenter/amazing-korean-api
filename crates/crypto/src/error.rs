/// 크레이트 전용 에러 타입.
///
/// 백엔드에서는 `From<CryptoError> for AppError`로 변환.
/// Flutter/Tauri에서는 직접 처리.
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Crypto: {0}")]
    Internal(String),
}

pub type CryptoResult<T> = Result<T, CryptoError>;
