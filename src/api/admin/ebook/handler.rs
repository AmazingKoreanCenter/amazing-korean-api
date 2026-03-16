use axum::extract::{Path, Query, State};
use axum::Json;

use crate::api::auth::extractor::AuthUser;
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{
    AdminEbookListReq, AdminEbookListRes, AdminEbookPurchaseItem, AdminUpdateEbookStatusReq,
};
use super::service::AdminEbookService;

/// GET /admin/ebook/purchases
pub async fn list_purchases(
    State(st): State<AppState>,
    AuthUser(_claims): AuthUser,
    Query(req): Query<AdminEbookListReq>,
) -> AppResult<Json<AdminEbookListRes>> {
    let res = AdminEbookService::list_purchases(&st, req).await?;
    Ok(Json(res))
}

/// GET /admin/ebook/purchases/:id
pub async fn get_purchase(
    State(st): State<AppState>,
    AuthUser(_claims): AuthUser,
    Path(id): Path<i64>,
) -> AppResult<Json<AdminEbookPurchaseItem>> {
    let res = AdminEbookService::get_purchase(&st, id).await?;
    Ok(Json(res))
}

/// PATCH /admin/ebook/purchases/:id/status
pub async fn update_status(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<AdminUpdateEbookStatusReq>,
) -> AppResult<Json<AdminEbookPurchaseItem>> {
    let res = AdminEbookService::update_status(&st, claims.sub, id, req).await?;
    Ok(Json(res))
}

/// DELETE /admin/ebook/purchases/:id
pub async fn delete_purchase(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
) -> AppResult<Json<serde_json::Value>> {
    AdminEbookService::delete_purchase(&st, claims.sub, id).await?;
    Ok(Json(serde_json::json!({ "message": "deleted" })))
}
