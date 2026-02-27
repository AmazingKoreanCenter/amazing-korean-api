use axum::extract::{Path, State};
use axum::Json;

use crate::error::AppResult;
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
    Json(req): Json<CreateOrderReq>,
) -> AppResult<Json<OrderRes>> {
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
