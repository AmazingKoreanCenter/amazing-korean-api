pub mod api;
pub mod config;
pub mod crypto;
pub mod docs;
pub mod error;
pub mod external;
pub mod state;
pub mod types;

// Re-export: 기존 main.rs 루트에서 `use crate::state::AppState;`로 제공하던 바인딩 유지.
// 내부 모듈에서 `use crate::AppState;`로 접근 가능.
use state::AppState;
