use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
#[allow(unused_imports)]
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal server error")]
    Internal(String),
    #[error("Health check failed: {0}")]
    HealthInternal(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unprocessable entity: {0}")]
    Unprocessable(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Not found")]
    NotFound,
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Too many requests: {0}")]
    TooManyRequests(String),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("External service error: {0}")]
    External(String),

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
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR".to_string(),
                    "Internal server error".to_string(),
                    None,
                    None,
                )
            }
            AppError::HealthInternal(reason) => {
                tracing::error!("Health check failed: {}", reason);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "HEALTH_INTERNAL".to_string(),
                    "Health check failed".to_string(),
                    None,
                    None,
                )
            }
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST".to_string(),
                msg.clone(),
                None,
                None,
            ),
            AppError::Unprocessable(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "UNPROCESSABLE_ENTITY".to_string(),
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
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN".to_string(),
                msg.clone(),
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
                "AUTH_429_TOO_MANY_ATTEMPTS".to_string(),
                msg.clone(),
                None,
                Some(60),
            ),
            AppError::ServiceUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE".to_string(),
                msg.clone(),
                None,
                None,
            ),
            AppError::External(msg) => {
                tracing::error!("External service error: {}", msg);
                (
                    StatusCode::BAD_GATEWAY,
                    "EXTERNAL_SERVICE_ERROR".to_string(),
                    "External service error".to_string(),
                    None,
                    None,
                )
            }
            AppError::Sqlx(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DB_ERROR".to_string(),
                    "Database error".to_string(),
                    None,
                    None,
                )
            }
            AppError::Anyhow(e) => {
                tracing::error!("Application error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "ANYHOW_ERROR".to_string(),
                    "Application error".to_string(),
                    None,
                    None,
                )
            }
            AppError::Redis(e) => {
                tracing::error!("Redis error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "REDIS_ERROR".to_string(),
                    "Cache error".to_string(),
                    None,
                    None,
                )
            }
            AppError::Jsonwebtoken(e) => {
                tracing::warn!("JWT error: {:?}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    "JWT_ERROR".to_string(),
                    "Authentication failed".to_string(),
                    None,
                    None,
                )
            }
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
                "trace_id": "req-TODO",
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
