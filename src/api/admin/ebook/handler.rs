use crate::extract::AppJson;
use axum::extract::{Path, Query, State};
use axum::Json;

use crate::api::auth::extractor::AuthUser;
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{
    AdminEbookDeleteRes, AdminEbookListReq, AdminEbookListRes, AdminEbookPurchaseItem,
    AdminUpdateEbookStatusReq, WatermarkVerifyRes,
};
use super::service::AdminEbookService;

/// GET /admin/ebook/purchases
#[utoipa::path(
    get,
    path = "/admin/ebook/purchases",
    tag = "Admin Ebook",
    security(("bearerAuth" = [])),
    params(AdminEbookListReq),
    responses(
        (status = 200, description = "Ebook purchase list", body = AdminEbookListRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    )
)]
pub async fn list_purchases(
    State(st): State<AppState>,
    AuthUser(_claims): AuthUser,
    Query(req): Query<AdminEbookListReq>,
) -> AppResult<Json<AdminEbookListRes>> {
    let res = AdminEbookService::list_purchases(&st, req).await?;
    Ok(Json(res))
}

/// GET /admin/ebook/purchases/:id
#[utoipa::path(
    get,
    path = "/admin/ebook/purchases/{id}",
    tag = "Admin Ebook",
    security(("bearerAuth" = [])),
    params(
        ("id" = i64, Path, description = "Purchase ID")
    ),
    responses(
        (status = 200, description = "Ebook purchase detail", body = AdminEbookPurchaseItem),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Purchase not found", body = crate::error::ErrorBody)
    )
)]
pub async fn get_purchase(
    State(st): State<AppState>,
    AuthUser(_claims): AuthUser,
    Path(id): Path<i64>,
) -> AppResult<Json<AdminEbookPurchaseItem>> {
    let res = AdminEbookService::get_purchase(&st, id).await?;
    Ok(Json(res))
}

/// PATCH /admin/ebook/purchases/:id/status
#[utoipa::path(
    patch,
    path = "/admin/ebook/purchases/{id}/status",
    tag = "Admin Ebook",
    security(("bearerAuth" = [])),
    params(
        ("id" = i64, Path, description = "Purchase ID")
    ),
    request_body = AdminUpdateEbookStatusReq,
    responses(
        (status = 200, description = "Status updated", body = AdminEbookPurchaseItem),
        (status = 400, description = "Invalid status transition", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    )
)]
pub async fn update_status(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
    AppJson(req): AppJson<AdminUpdateEbookStatusReq>,
) -> AppResult<Json<AdminEbookPurchaseItem>> {
    let res = AdminEbookService::update_status(&st, claims.sub, id, req).await?;
    Ok(Json(res))
}

/// GET /admin/ebook/verify/:watermark_id
///
/// 워터마크 진위확인 — 유출된 이미지에서 추출한 watermark_id로 구매자 정보 조회.
#[utoipa::path(
    get,
    path = "/admin/ebook/verify/{watermark_id}",
    tag = "Admin Ebook",
    security(("bearerAuth" = [])),
    params(
        ("watermark_id" = String, Path, description = "Watermark ID extracted from leaked image")
    ),
    responses(
        (status = 200, description = "Watermark verified", body = WatermarkVerifyRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Watermark not found", body = crate::error::ErrorBody)
    )
)]
pub async fn verify_watermark(
    State(st): State<AppState>,
    AuthUser(_claims): AuthUser,
    Path(watermark_id): Path<String>,
) -> AppResult<Json<WatermarkVerifyRes>> {
    let res = AdminEbookService::verify_watermark(&st, &watermark_id).await?;
    Ok(Json(res))
}

/// DELETE /admin/ebook/purchases/:id
#[utoipa::path(
    delete,
    path = "/admin/ebook/purchases/{id}",
    tag = "Admin Ebook",
    security(("bearerAuth" = [])),
    params(
        ("id" = i64, Path, description = "Purchase ID")
    ),
    responses(
        (status = 200, description = "Purchase deleted", body = AdminEbookDeleteRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Purchase not found", body = crate::error::ErrorBody)
    )
)]
pub async fn delete_purchase(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
) -> AppResult<Json<AdminEbookDeleteRes>> {
    AdminEbookService::delete_purchase(&st, claims.sub, id).await?;
    Ok(Json(AdminEbookDeleteRes {
        message: "deleted".to_string(),
    }))
}
