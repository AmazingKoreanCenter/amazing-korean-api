//! 요청 단위 trace_id 생성/전파 미들웨어.
//!
//! - 들어오는 `x-request-id` 헤더가 있고 형식이 유효하면 그대로 승계 (업스트림 CF/LB 상관추적)
//! - 없으면 UUID v7 생성 (시간 정렬 → 로그 검색 유리)
//! - `task_local!` 스코프에 넣어 `AppError::into_response` 등 하위 전 지점에서 `current()` 로 조회
//! - 동일 값을 `TraceId` extension 으로도 주입 (핸들러에서 `Extension<TraceId>` 로 추출 가능)
//! - 응답 헤더 `x-request-id` 로 에코백 (클라이언트/지원 상담 시 매칭용)

use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

const HEADER: HeaderName = HeaderName::from_static("x-request-id");
const MAX_INBOUND_LEN: usize = 128;

tokio::task_local! {
    static REQUEST_ID: String;
}

#[derive(Clone, Debug)]
pub struct TraceId(pub String);

pub fn current() -> Option<String> {
    REQUEST_ID.try_with(|id| id.clone()).ok()
}

pub async fn middleware(mut request: Request, next: Next) -> Response {
    let id = request
        .headers()
        .get(&HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| {
            !s.is_empty()
                && s.len() <= MAX_INBOUND_LEN
                && s.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        })
        .map(str::to_string)
        .unwrap_or_else(|| Uuid::now_v7().to_string());

    request.extensions_mut().insert(TraceId(id.clone()));

    let header_value = HeaderValue::from_str(&id).ok();
    let mut response = REQUEST_ID
        .scope(id, async move { next.run(request).await })
        .await;

    if let Some(v) = header_value {
        response.headers_mut().insert(HEADER, v);
    }
    response
}
