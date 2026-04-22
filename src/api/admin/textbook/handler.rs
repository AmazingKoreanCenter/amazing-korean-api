use crate::extract::AppJson;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::api::auth::extractor::AuthUser;
use crate::api::textbook::dto::OrderRes;
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{
    AdminCreateOrderReq, AdminTextbookListReq, AdminTextbookListRes, AdminTextbookLogListRes,
    AdminTextbookLogQuery, AdminUpdateStatusReq, AdminUpdateTrackingReq,
};
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
    AppJson(req): AppJson<AdminUpdateStatusReq>,
) -> AppResult<Json<OrderRes>> {
    let res =
        AdminTextbookService::update_status(&st, auth_user.sub, id, req.status).await?;
    Ok(Json(res))
}

/// PATCH /admin/textbook/orders/:id/tracking
///
/// 배송 추적 정보 업데이트.
#[utoipa::path(
    patch,
    path = "/admin/textbook/orders/{id}/tracking",
    tag = "Admin Textbook",
    security(("bearerAuth" = [])),
    params(
        ("id" = i64, Path, description = "주문 ID")
    ),
    request_body = AdminUpdateTrackingReq,
    responses(
        (status = 200, description = "추적 정보 업데이트 완료", body = OrderRes),
        (status = 404, description = "주문 없음")
    )
)]
pub async fn update_tracking(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(id): Path<i64>,
    AppJson(req): AppJson<AdminUpdateTrackingReq>,
) -> AppResult<Json<OrderRes>> {
    let res = AdminTextbookService::update_tracking(
        &st,
        auth_user.sub,
        id,
        req.tracking_number.as_deref(),
        req.tracking_provider.as_deref(),
    )
    .await?;
    Ok(Json(res))
}

/// POST /admin/textbook/orders
///
/// 관리자 대리 주문 생성. 외부 채널(전화·이메일·오프라인) 주문 시스템 입력용.
/// 최소 수량(10권) 제약 기본 면제. `initial_status` 로 paid 즉시 세팅 가능.
#[utoipa::path(
    post,
    path = "/admin/textbook/orders",
    tag = "Admin Textbook",
    security(("bearerAuth" = [])),
    request_body = AdminCreateOrderReq,
    responses(
        (status = 200, description = "주문 생성 완료", body = OrderRes),
        (status = 400, description = "요청 형식 오류"),
        (status = 401, description = "인증 필요"),
        (status = 403, description = "권한 없음"),
        (status = 422, description = "도메인 제약 위반")
    )
)]
pub async fn admin_create_order(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    AppJson(req): AppJson<AdminCreateOrderReq>,
) -> AppResult<Json<OrderRes>> {
    let res = AdminTextbookService::create_order(&st, auth_user.sub, req).await?;
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

/// GET /admin/textbook/logs (Q6, 2026-04-22)
///
/// admin_textbook_log 감사 로그 조회. action / order_id / admin_user_id 필터 +
/// 페이지네이션. 관리자 대리 주문 생성(Create 액션) 기록 추적 등 용도.
#[utoipa::path(
    get,
    path = "/admin/textbook/logs",
    tag = "Admin Textbook",
    security(("bearerAuth" = [])),
    params(AdminTextbookLogQuery),
    responses(
        (status = 200, description = "감사 로그 목록", body = AdminTextbookLogListRes),
        (status = 401, description = "인증 필요"),
        (status = 403, description = "권한 없음")
    )
)]
pub async fn list_admin_logs(
    State(st): State<AppState>,
    _auth: AuthUser,
    Query(req): Query<AdminTextbookLogQuery>,
) -> AppResult<Json<AdminTextbookLogListRes>> {
    let res = AdminTextbookService::list_admin_logs(&st, req).await?;
    Ok(Json(res))
}
