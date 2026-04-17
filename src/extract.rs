//! AppError envelope 을 반환하는 커스텀 JSON 추출기.
//!
//! axum 기본 `Json<T>` extractor 는 실패 시 `text/plain` 응답을 직접 반환하므로
//! `AMK_API_MASTER §3.4` 표준 에러 envelope (`{error:{code,http_status,message,
//! details,trace_id}}`) 과 `x-request-id` body 매칭 규약을 우회한다.
//!
//! `AppJson<T>` 는 내부적으로 `Json<T>` 를 호출하고 `JsonRejection` 을 `AppError`
//! 로 변환해 표준 파이프라인 (`AppError::into_response`) 을 타도록 강제한다.
//! 핸들러 시그니처에서 `Json(x): Json<T>` 를 `AppJson(x): AppJson<T>` 로 바꾸면
//! 동작 차이 없이 에러 응답만 통일된다.

use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct AppJson<T>(pub T);

impl<S, T> FromRequest<S> for AppJson<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(AppJson(value)),
            Err(rejection) => Err(map_json_rejection(rejection)),
        }
    }
}

fn map_json_rejection(rejection: JsonRejection) -> AppError {
    match rejection {
        JsonRejection::JsonDataError(err) => {
            AppError::Unprocessable(format!("Invalid JSON data: {err}"))
        }
        JsonRejection::JsonSyntaxError(err) => {
            AppError::BadRequest(format!("Malformed JSON: {err}"))
        }
        JsonRejection::MissingJsonContentType(_) => {
            AppError::BadRequest("Expected Content-Type: application/json".to_string())
        }
        JsonRejection::BytesRejection(err) => {
            AppError::BadRequest(format!("Failed to read request body: {err}"))
        }
        other => AppError::Unprocessable(format!("Invalid JSON request: {}", other.body_text())),
    }
}
