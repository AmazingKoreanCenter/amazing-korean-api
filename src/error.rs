use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
#[allow(unused_imports)] // ValidatorErrors is used in #[from] attribute
use validator::ValidationErrors;

#[derive(Debug, Error)] // Removed Serialize and ToSchema
pub enum AppError {
    #[error("Internal server error")]
    Internal(String),
    #[error("Health check failed: {0}")]
    HealthInternal(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden")]
    Forbidden,
    #[error("Not found")]
    NotFound,
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    Redis(#[from] deadpool_redis::redis::RedisError),
    #[error(transparent)]
    Jsonwebtoken(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
}

impl From<std::convert::Infallible> for AppError {
    fn from(err: std::convert::Infallible) -> Self {
        AppError::Internal(format!("Infallible error: {}", err))
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message, details, retry_after) = match self {
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR".to_string(),
                "Internal server error".to_string(),
                Some(serde_json::json!({ "debug": msg })),
                None,
            ),
            AppError::HealthInternal(reason) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "HEALTH_INTERNAL".to_string(),
                "Health check failed".to_string(),
                Some(serde_json::json!({ "reason": reason })),
                None,
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST".to_string(),
                msg.clone(),
                None,
                None,
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED".to_string(),
                msg.clone(),
                None,
                None,
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN".to_string(),
                "Forbidden".to_string(),
                None,
                None,
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND".to_string(),
                "Not found".to_string(),
                None,
                None,
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                "CONFLICT".to_string(),
                msg.clone(),
                None,
                None,
            ),
            AppError::TooManyRequests(msg) => (
                StatusCode::TOO_MANY_REQUESTS,
                "AUTH_429_TOO_MANY_ATTEMPTS".to_string(), // Specific error code
                msg.clone(),
                None,
                Some(60), // Retry-After header in seconds
            ),
            AppError::Sqlx(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DB_ERROR".to_string(),
                "Database error".to_string(),
                Some(serde_json::json!({ "debug": e.to_string() })),
                None,
            ),
            AppError::Anyhow(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "ANYHOW_ERROR".to_string(),
                "Application error".to_string(),
                Some(serde_json::json!({ "debug": e.to_string() })),
                None,
            ),
            AppError::Redis(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "REDIS_ERROR".to_string(),
                "Redis error".to_string(),
                Some(serde_json::json!({ "debug": e.to_string() })),
                None,
            ),
            AppError::Jsonwebtoken(e) => (
                StatusCode::UNAUTHORIZED,
                "JWT_ERROR".to_string(),
                "JWT error".to_string(),
                Some(serde_json::json!({ "debug": e.to_string() })),
                None,
            ),
            AppError::Validation(e) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR".to_string(),
                "Validation failed".to_string(),
                Some(serde_json::json!({ "errors": e.to_string() })),
                None,
            ),
        };

        let error_body = serde_json::json!({
            "error": {
                "code": error_code,
                "http_status": status.as_u16(),
                "message": message,
                "details": details,
                "trace_id": "req-TODO", // TODO: Add trace ID
            }
        });

        let mut response = (status, Json(error_body)).into_response();
        if let Some(retry_after_secs) = retry_after {
            response.headers_mut().insert(
                header::RETRY_AFTER,
                retry_after_secs.to_string().parse().unwrap(),
            );
        }
        response
    }
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ErrorBody {
    pub error: ErrorDetail,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ErrorDetail {
    pub code: String,
    pub http_status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}
