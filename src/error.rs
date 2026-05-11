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
    /// N-36: 인증/비밀번호 endpoint 용 validation 에러. 룰/필드명 미노출 (anti-enumeration).
    /// service 에서 명시적 변환 필요: `req.validate().map_err(|_| AppError::ValidationGeneric)?`.
    #[error("Invalid input")]
    ValidationGeneric,
}

impl From<std::convert::Infallible> for AppError {
    fn from(err: std::convert::Infallible) -> Self {
        AppError::Internal(format!("Infallible error: {}", err))
    }
}

impl From<amazing_korean_crypto::CryptoError> for AppError {
    fn from(err: amazing_korean_crypto::CryptoError) -> Self {
        use amazing_korean_crypto::CryptoError;
        match err {
            CryptoError::InvalidFormat(msg) => AppError::Internal(msg),
            CryptoError::DecryptionFailed(msg) => AppError::Internal(msg),
            CryptoError::Internal(msg) => AppError::Internal(msg),
        }
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
            // N-36: 인증/비밀번호 endpoint validation 실패 — 룰/필드명 미노출.
            // 내부 진단은 호출자 service 에서 tracing::debug 으로 별도 처리.
            AppError::ValidationGeneric => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR".to_string(),
                "Invalid input".to_string(),
                None,
                None,
            ),
        };

        let error_body = serde_json::json!({
            "error": {
                "code": error_code,
                "http_status": status.as_u16(),
                "message": message,
                "details": details,
                "trace_id": crate::trace_id::current().unwrap_or_else(|| "unknown".to_string()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::response::IntoResponse;

    /// 응답을 (status, body_json) 으로 분해. AppError::into_response 의 envelope 검증용.
    async fn into_parts(err: AppError) -> (StatusCode, serde_json::Value) {
        let response = err.into_response();
        let status = response.status();
        let body_bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("read body");
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).expect("parse json");
        (status, body)
    }

    fn extract_error_field<'a>(body: &'a serde_json::Value, field: &str) -> &'a str {
        body["error"][field].as_str().unwrap_or("")
    }

    #[tokio::test]
    async fn internal_error_returns_500_with_internal_server_error_code() {
        let (status, body) = into_parts(AppError::Internal("test".into())).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(extract_error_field(&body, "code"), "INTERNAL_SERVER_ERROR");
    }

    #[tokio::test]
    async fn health_internal_returns_500_with_health_internal_code() {
        let (status, body) = into_parts(AppError::HealthInternal("db".into())).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(extract_error_field(&body, "code"), "HEALTH_INTERNAL");
    }

    #[tokio::test]
    async fn bad_request_returns_400_with_bad_request_code_and_message() {
        let (status, body) = into_parts(AppError::BadRequest("invalid email".into())).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(extract_error_field(&body, "code"), "BAD_REQUEST");
        assert_eq!(extract_error_field(&body, "message"), "invalid email");
    }

    #[tokio::test]
    async fn unprocessable_returns_422_with_unprocessable_entity_code() {
        let (status, body) = into_parts(AppError::Unprocessable("malformed".into())).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(extract_error_field(&body, "code"), "UNPROCESSABLE_ENTITY");
    }

    #[tokio::test]
    async fn unauthorized_returns_401_with_unauthorized_code() {
        let (status, body) = into_parts(AppError::Unauthorized("token".into())).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(extract_error_field(&body, "code"), "UNAUTHORIZED");
    }

    #[tokio::test]
    async fn forbidden_returns_403_with_forbidden_code() {
        let (status, body) = into_parts(AppError::Forbidden("role".into())).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(extract_error_field(&body, "code"), "FORBIDDEN");
    }

    #[tokio::test]
    async fn not_found_returns_404_with_not_found_code() {
        let (status, body) = into_parts(AppError::NotFound).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(extract_error_field(&body, "code"), "NOT_FOUND");
    }

    #[tokio::test]
    async fn conflict_returns_409_with_conflict_code() {
        let (status, body) = into_parts(AppError::Conflict("dup".into())).await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(extract_error_field(&body, "code"), "CONFLICT");
    }

    #[tokio::test]
    async fn too_many_requests_returns_429_with_retry_after_header() {
        let response = AppError::TooManyRequests("rate".into()).into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        // Retry-After 헤더 = 60 (initial).
        let retry_after = response
            .headers()
            .get(header::RETRY_AFTER)
            .expect("Retry-After header");
        assert_eq!(retry_after.to_str().unwrap(), "60");
    }

    #[tokio::test]
    async fn service_unavailable_returns_503_with_service_unavailable_code() {
        let (status, body) = into_parts(AppError::ServiceUnavailable("Payment".into())).await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(extract_error_field(&body, "code"), "SERVICE_UNAVAILABLE");
    }

    #[tokio::test]
    async fn external_returns_502_with_external_service_error_code() {
        let (status, body) = into_parts(AppError::External("paddle".into())).await;
        assert_eq!(status, StatusCode::BAD_GATEWAY);
        assert_eq!(extract_error_field(&body, "code"), "EXTERNAL_SERVICE_ERROR");
    }

    #[tokio::test]
    async fn validation_generic_returns_400_with_anti_enumeration_message() {
        // N-36 anti-enumeration: 룰/필드명 미노출.
        let (status, body) = into_parts(AppError::ValidationGeneric).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(extract_error_field(&body, "code"), "VALIDATION_ERROR");
        assert_eq!(extract_error_field(&body, "message"), "Invalid input");
    }

    #[tokio::test]
    async fn sqlx_error_returns_500_with_db_error_code() {
        // sqlx::Error 는 RowNotFound 가 가장 간단.
        let err: AppError = sqlx::Error::RowNotFound.into();
        let (status, body) = into_parts(err).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(extract_error_field(&body, "code"), "DB_ERROR");
    }

    #[tokio::test]
    async fn jsonwebtoken_error_returns_401() {
        let jwt_err =
            jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken);
        let err: AppError = jwt_err.into();
        let (status, body) = into_parts(err).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(extract_error_field(&body, "code"), "JWT_ERROR");
    }

    #[test]
    fn crypto_error_decryption_failed_maps_to_internal() {
        use amazing_korean_crypto::CryptoError;
        let err: AppError = CryptoError::DecryptionFailed("bad nonce".into()).into();
        match err {
            AppError::Internal(_) => {}
            other => panic!("expected Internal, got {:?}", other),
        }
    }

    #[test]
    fn crypto_error_invalid_format_maps_to_internal() {
        use amazing_korean_crypto::CryptoError;
        let err: AppError = CryptoError::InvalidFormat("hex".into()).into();
        assert!(matches!(err, AppError::Internal(_)));
    }

    #[test]
    fn crypto_error_internal_maps_to_internal() {
        use amazing_korean_crypto::CryptoError;
        let err: AppError = CryptoError::Internal("ring".into()).into();
        assert!(matches!(err, AppError::Internal(_)));
    }
}
