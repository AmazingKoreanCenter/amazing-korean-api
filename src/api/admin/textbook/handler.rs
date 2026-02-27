use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::api::auth::extractor::AuthUser;
use crate::api::textbook::dto::OrderRes;
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{AdminTextbookListReq, AdminTextbookListRes, AdminUpdateStatusReq};
use super::service::AdminTextbookService;

/// GET /admin/textbook/orders
///
/// 교재 주문 목록 조회. 상태 필터, 검색, 페이지네이션 지원.
#[utoipa::path(
    get,
    path = "/admin/textbook/orders",
    tag = "Admin Textbook",
    security(("bearerAuth" = [])),
    params(AdminTextbookListReq),
    responses(
        (status = 200, description = "주문 목록", body = AdminTextbookListRes),
        (status = 401, description = "인증 필요"),
        (status = 403, description = "권한 없음")
    )
)]
pub async fn list_orders(
    State(st): State<AppState>,
    _auth: AuthUser,
    Query(req): Query<AdminTextbookListReq>,
) -> AppResult<Json<AdminTextbookListRes>> {
    let page = req.page.unwrap_or(1).max(1);
    let per_page = req.size.unwrap_or(20).clamp(1, 100);

    let res = AdminTextbookService::list_orders(
        &st,
        req.status,
        req.q.as_deref(),
        page,
        per_page,
    )
    .await?;

    Ok(Json(res))
}

/// GET /admin/textbook/orders/:id
///
/// 교재 주문 상세 조회.
#[utoipa::path(
    get,
    path = "/admin/textbook/orders/{id}",
    tag = "Admin Textbook",
    security(("bearerAuth" = [])),
    params(
        ("id" = i64, Path, description = "주문 ID")
    ),
    responses(
        (status = 200, description = "주문 상세", body = OrderRes),
        (status = 404, description = "주문 없음")
    )
)]
pub async fn get_order(
    State(st): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> AppResult<Json<OrderRes>> {
    let res = AdminTextbookService::get_order(&st, id).await?;
    Ok(Json(res))
}

/// PATCH /admin/textbook/orders/:id/status
///
/// 교재 주문 상태 변경.
#[utoipa::path(
    patch,
    path = "/admin/textbook/orders/{id}/status",
    tag = "Admin Textbook",
    security(("bearerAuth" = [])),
    params(
        ("id" = i64, Path, description = "주문 ID")
    ),
    request_body = AdminUpdateStatusReq,
    responses(
        (status = 200, description = "상태 변경 완료", body = OrderRes),
        (status = 404, description = "주문 없음")
    )
)]
pub async fn update_status(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<AdminUpdateStatusReq>,
) -> AppResult<Json<OrderRes>> {
    let res =
        AdminTextbookService::update_status(&st, auth_user.sub, id, req.status).await?;
    Ok(Json(res))
}

/// DELETE /admin/textbook/orders/:id
///
/// 교재 주문 삭제.
#[utoipa::path(
    delete,
    path = "/admin/textbook/orders/{id}",
    tag = "Admin Textbook",
    security(("bearerAuth" = [])),
    params(
        ("id" = i64, Path, description = "주문 ID")
    ),
    responses(
        (status = 204, description = "삭제 완료"),
        (status = 404, description = "주문 없음")
    )
)]
pub async fn delete_order(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(id): Path<i64>,
) -> AppResult<StatusCode> {
    AdminTextbookService::delete_order(&st, auth_user.sub, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
