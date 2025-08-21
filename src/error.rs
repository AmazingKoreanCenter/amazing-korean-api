use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[allow(dead_code)] // 아직 미사용이면 경고 억제
    #[error("forbidden")]
    Forbidden,

    #[error("not found")]
    NotFound,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("internal: {0}")]
    Internal(String),

    // 필요 시 유지
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize, ToSchema)]
pub struct ErrorBody {
    pub error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Sqlx(_) | AppError::Anyhow(_) | AppError::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        (
            status,
            Json(ErrorBody {
                error: self.to_string(),
            }),
        )
            .into_response()
    }
}
