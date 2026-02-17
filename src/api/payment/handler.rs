use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use paddle_rust_sdk::webhooks::MaximumVariance;
use paddle_rust_sdk::Paddle;

use crate::api::auth::extractor::AuthUser;
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{CancelSubscriptionReq, PlansRes, SubscriptionRes};
use super::service::PaymentService;

/// GET /payment/plans
///
/// 사용 가능한 구독 플랜 목록 반환 (인증 불필요).
/// 프론트엔드에서 Paddle.js checkout을 위해 client_token과 price_id를 제공.
#[utoipa::path(
    get,
    path = "/payment/plans",
    tag = "Payment",
    responses(
        (status = 200, description = "구독 플랜 목록", body = PlansRes),
        (status = 503, description = "결제 서비스 미설정")
    )
)]
pub async fn get_plans(State(st): State<AppState>) -> AppResult<Json<PlansRes>> {
    let res = PaymentService::get_plans(&st).await?;
    Ok(Json(res))
}

/// GET /payment/subscription
///
/// 현재 로그인된 사용자의 구독 상태 조회.
#[utoipa::path(
    get,
    path = "/payment/subscription",
    tag = "Payment",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "구독 상태", body = SubscriptionRes),
        (status = 401, description = "인증 필요")
    )
)]
pub async fn get_subscription(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
) -> AppResult<Json<SubscriptionRes>> {
    let res = PaymentService::get_subscription(&st, auth_user.sub).await?;
    Ok(Json(res))
}

/// POST /payment/subscription/cancel
///
/// 구독 취소. immediately=true면 즉시 취소, false면 현재 기간 종료 후 취소.
#[utoipa::path(
    post,
    path = "/payment/subscription/cancel",
    tag = "Payment",
    security(("bearerAuth" = [])),
    request_body = CancelSubscriptionReq,
    responses(
        (status = 200, description = "구독 취소 완료", body = SubscriptionRes),
        (status = 400, description = "활성 구독 없음"),
        (status = 401, description = "인증 필요")
    )
)]
pub async fn cancel_subscription(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Json(req): Json<CancelSubscriptionReq>,
) -> AppResult<Json<SubscriptionRes>> {
    let res = PaymentService::cancel_subscription(&st, auth_user.sub, req.immediately).await?;
    Ok(Json(res))
}

/// POST /payment/webhook
///
/// Paddle Webhook 수신 엔드포인트.
/// Paddle-Signature 헤더로 서명 검증 후 이벤트 처리.
/// 인증 미들웨어 없이 공개 — 서명 검증으로 보안.
pub async fn handle_webhook(
    State(st): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    // 1. Paddle-Signature 헤더 추출
    let signature = match headers.get("Paddle-Signature") {
        Some(v) => match v.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => {
                tracing::warn!("Paddle webhook: invalid Paddle-Signature header encoding");
                return StatusCode::BAD_REQUEST;
            }
        },
        None => {
            tracing::warn!("Paddle webhook: missing Paddle-Signature header");
            return StatusCode::BAD_REQUEST;
        }
    };

    // 2. Webhook secret 확인
    let webhook_secret = match &st.cfg.paddle_webhook_secret {
        Some(s) => s.clone(),
        None => {
            tracing::error!("Paddle webhook secret not configured (PADDLE_WEBHOOK_SECRET)");
            return StatusCode::OK; // Paddle 재시도 방지
        }
    };

    // 3. Raw body → string
    let body_str = match std::str::from_utf8(&body) {
        Ok(s) => s.to_string(),
        Err(_) => {
            tracing::warn!("Paddle webhook: invalid UTF-8 body");
            return StatusCode::BAD_REQUEST;
        }
    };

    // 4. 서명 검증 + Event 파싱 (5분 허용)
    let event = match Paddle::unmarshal(
        &body_str,
        &webhook_secret,
        &signature,
        MaximumVariance::seconds(300),
    ) {
        Ok(e) => e,
        Err(e) => {
            tracing::warn!(error = %e, "Paddle webhook signature verification failed");
            return StatusCode::BAD_REQUEST;
        }
    };

    // 5. 이벤트 처리 (내부 에러가 있어도 항상 200 반환 — Paddle 재시도 방지)
    if let Err(e) = PaymentService::process_webhook_event(&st, event, &body_str).await {
        tracing::error!(error = %e, "Failed to process Paddle webhook event");
    }

    StatusCode::OK
}
