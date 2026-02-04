//! 관리자 초대/승격 핸들러
//!
//! - POST /admin/upgrade: 관리자 초대
//! - GET /admin/upgrade/verify: 초대 코드 검증
//! - POST /admin/upgrade/accept: 관리자 계정 생성

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    api::admin::upgrade::{dto::*, service::UpgradeService},
    api::auth::extractor::AuthUser,
    error::AppError,
    state::AppState,
};

/// POST /admin/upgrade - 관리자 초대
///
/// RBAC:
/// - HYMN -> admin, manager 초대 가능
/// - Admin -> manager만 초대 가능
/// - Manager -> 불가
#[utoipa::path(
    post,
    path = "/admin/upgrade",
    request_body = UpgradeInviteReq,
    responses(
        (status = 200, description = "Invitation sent", body = UpgradeInviteRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - insufficient permission"),
        (status = 409, description = "Conflict - email already exists"),
        (status = 422, description = "Unprocessable - invalid role"),
    ),
    security(("bearer_auth" = [])),
    tag = "Admin Upgrade"
)]
pub async fn create_invite(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(req): Json<UpgradeInviteReq>,
) -> Result<Json<UpgradeInviteRes>, AppError> {
    let res = UpgradeService::create_invite(&st, claims.sub, req).await?;
    Ok(Json(res))
}

/// GET /admin/upgrade/verify - 초대 코드 검증
///
/// Public endpoint (인증 불필요, 코드로 검증)
#[utoipa::path(
    get,
    path = "/admin/upgrade/verify",
    params(
        ("code" = String, Query, description = "Invite code (ak_upgrade_xxx)")
    ),
    responses(
        (status = 200, description = "Valid invite", body = UpgradeVerifyRes),
        (status = 400, description = "Bad request - missing code"),
        (status = 401, description = "Unauthorized - invalid or expired code"),
    ),
    tag = "Admin Upgrade"
)]
pub async fn verify_invite(
    State(st): State<AppState>,
    Query(req): Query<UpgradeVerifyReq>,
) -> Result<Json<UpgradeVerifyRes>, AppError> {
    let res = UpgradeService::verify_invite(&st, &req.code).await?;
    Ok(Json(res))
}

/// POST /admin/upgrade/accept - 관리자 계정 생성
///
/// Public endpoint (인증 불필요, 코드로 검증)
#[utoipa::path(
    post,
    path = "/admin/upgrade/accept",
    request_body = UpgradeAcceptReq,
    responses(
        (status = 201, description = "Admin account created", body = UpgradeAcceptRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized - invalid or expired code"),
        (status = 409, description = "Conflict - email or nickname exists"),
        (status = 422, description = "Unprocessable - weak password"),
    ),
    tag = "Admin Upgrade"
)]
pub async fn accept_invite(
    State(st): State<AppState>,
    Json(req): Json<UpgradeAcceptReq>,
) -> Result<(StatusCode, Json<UpgradeAcceptRes>), AppError> {
    let res = UpgradeService::accept_invite(&st, req).await?;
    Ok((StatusCode::CREATED, Json(res)))
}
