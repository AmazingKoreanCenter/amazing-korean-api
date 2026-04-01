//! Re-export from `amazing-korean-crypto` crate.
//!
//! 기존 `use crate::crypto::*` 코드를 그대로 유지하기 위한 얇은 래퍼.
//! 실 구현은 `crates/crypto/`에 있음.

pub use amazing_korean_crypto::blind_index;
pub use amazing_korean_crypto::cipher;
pub use amazing_korean_crypto::service;

pub use amazing_korean_crypto::{CryptoService, KeyRing};
