use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::Json;
use redis::AsyncCommands;
use validator::Validate;

use crate::api::util::extract_client_ip;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

use super::dto::{CatalogRes, CreateOrderReq, OrderRes};
use super::service::TextbookService;

/// GET /textbook/catalog
///
/// 교재 카탈로그 (언어 목록, 가격, 사용 가능 여부) 반환. 인증 불필요.
#[utoipa::path(
    get,
    path = "/textbook/catalog",
    tag = "Textbook",
    responses(
        (status = 200, description = "교재 카탈로그", body = CatalogRes)
    )
)]
pub async fn get_catalog() -> AppResult<Json<CatalogRes>> {
    let res = TextbookService::get_catalog().await?;
    Ok(Json(res))
}

/// POST /textbook/orders
///
/// 교재 주문 생성. 비회원도 주문 가능 (인증 불필요).
#[utoipa::path(
    post,
    path = "/textbook/orders",
    tag = "Textbook",
    request_body = CreateOrderReq,
    responses(
        (status = 200, description = "주문 생성 완료", body = OrderRes),
        (status = 400, description = "유효성 검증 실패")
    )
)]
pub async fn create_order(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateOrderReq>,
) -> AppResult<Json<OrderRes>> {
    // IP 기반 Rate Limiting (비회원 주문 스팸 방지)
    let client_ip = extract_client_ip(&headers);
    let rl_key = format!("rl:textbook_order:{}", client_ip);
    let mut redis_conn = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;

    let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
    if attempts == 1 {
        let _: () = redis_conn.expire(&rl_key, st.cfg.rate_limit_textbook_window_sec).await?;
    }
    if attempts > st.cfg.rate_limit_textbook_max {
        return Err(AppError::TooManyRequests("TEXTBOOK_429_TOO_MANY_ORDERS".into()));
    }

    // 입력 검증 (length, email 형식 등)
    req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

    let res = TextbookService::create_order(&st, req).await?;
    Ok(Json(res))
}

/// GET /textbook/orders/:code
///
/// 주문번호로 주문 상태 조회. 인증 불필요.
#[utoipa::path(
    get,
    path = "/textbook/orders/{code}",
    tag = "Textbook",
    params(
        ("code" = String, Path, description = "주문번호 (TB-YYMMDD-NNNN)")
    ),
    responses(
        (status = 200, description = "주문 상세", body = OrderRes),
        (status = 404, description = "주문 없음")
    )
)]
pub async fn get_order_by_code(
    State(st): State<AppState>,
    Path(code): Path<String>,
) -> AppResult<Json<OrderRes>> {
    let res = TextbookService::get_order_by_code(&st, &code).await?;
    Ok(Json(res))
}
