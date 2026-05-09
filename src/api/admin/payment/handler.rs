use crate::extract::AppJson;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::api::admin::header_utils::{extract_client_ip, extract_user_agent};
use crate::{api::auth::extractor::AuthUser, error::AppResult, state::AppState};

use super::{
    dto::{
        AdminCancelSubReq, AdminGrantListReq, AdminGrantListRes, AdminGrantReq, AdminGrantRes,
        AdminSubDetailRes, AdminSubListReq, AdminSubListRes, AdminTxnListReq, AdminTxnListRes,
    },
    service::AdminPaymentService,
};

#[allow(unused_imports)]
use serde_json::json;

// =============================================================================
// 구독 관리
// =============================================================================

#[utoipa::path(
    get,
    path = "/admin/payment/subscriptions",
    tag = "admin_payment",
    params(
        ("q", Query, description = "Search email or nickname"),
        ("status", Query, description = "Filter by status (trialing, active, past_due, paused, canceled)"),
        ("sort", Query, description = "Sort field (id, created_at, status, billing_interval, price)"),
        ("order", Query, description = "Sort order (asc, desc)"),
        ("page", Query, description = "Page number"),
        ("size", Query, description = "Page size (max 100)")
    ),
    responses(
        (status = 200, description = "Subscription list", body = AdminSubListRes),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearerAuth" = []))
)]
pub async fn list_subscriptions(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Query(params): Query<AdminSubListReq>,
) -> AppResult<Json<AdminSubListRes>> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let res = AdminPaymentService::list_subscriptions(&st, auth_user.sub, params, ip, ua).await?;

    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/admin/payment/subscriptions/{id}",
    tag = "admin_payment",
    params(("id", Path, description = "Subscription ID")),
    responses(
        (status = 200, description = "Subscription detail", body = AdminSubDetailRes),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found")
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_subscription(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> AppResult<Json<AdminSubDetailRes>> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let res = AdminPaymentService::get_subscription(&st, auth_user.sub, id, ip, ua).await?;

    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/admin/payment/subscriptions/{id}/cancel",
    tag = "admin_payment",
    params(("id", Path, description = "Subscription ID")),
    request_body = AdminCancelSubReq,
    responses(
        (status = 200, description = "Subscription canceled", body = AdminSubDetailRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found")
    ),
    security(("bearerAuth" = []))
)]
pub async fn cancel_subscription(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Path(id): Path<i64>,
    AppJson(req): AppJson<AdminCancelSubReq>,
) -> AppResult<Json<AdminSubDetailRes>> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let res = AdminPaymentService::cancel_subscription(&st, auth_user.sub, id, req, ip, ua).await?;

    Ok(Json(res))
}

// =============================================================================
// 트랜잭션
// =============================================================================

#[utoipa::path(
    get,
    path = "/admin/payment/transactions",
    tag = "admin_payment",
    params(
        ("q", Query, description = "Search email"),
        ("status", Query, description = "Filter by status (completed, refunded, partially_refunded)"),
        ("sort", Query, description = "Sort field (id, occurred_at, amount, status)"),
        ("order", Query, description = "Sort order (asc, desc)"),
        ("page", Query, description = "Page number"),
        ("size", Query, description = "Page size (max 100)")
    ),
    responses(
        (status = 200, description = "Transaction list", body = AdminTxnListRes),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearerAuth" = []))
)]
pub async fn list_transactions(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Query(params): Query<AdminTxnListReq>,
) -> AppResult<Json<AdminTxnListRes>> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let res = AdminPaymentService::list_transactions(&st, auth_user.sub, params, ip, ua).await?;

    Ok(Json(res))
}

// =============================================================================
// 수동 수강권
// =============================================================================

#[utoipa::path(
    post,
    path = "/admin/payment/grants",
    tag = "admin_payment",
    request_body = AdminGrantReq,
    responses(
        (status = 201, description = "Courses granted", body = AdminGrantRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearerAuth" = []))
)]
pub async fn create_grant(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    AppJson(req): AppJson<AdminGrantReq>,
) -> AppResult<(StatusCode, Json<AdminGrantRes>)> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let res = AdminPaymentService::create_grant(&st, auth_user.sub, req, ip, ua).await?;

    Ok((StatusCode::CREATED, Json(res)))
}

#[utoipa::path(
    get,
    path = "/admin/payment/grants",
    tag = "admin_payment",
    params(
        ("page", Query, description = "Page number"),
        ("size", Query, description = "Page size (max 100)")
    ),
    responses(
        (status = 200, description = "Manual grant list", body = AdminGrantListRes),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearerAuth" = []))
)]
pub async fn list_grants(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Query(params): Query<AdminGrantListReq>,
) -> AppResult<Json<AdminGrantListRes>> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let res = AdminPaymentService::list_grants(&st, auth_user.sub, params, ip, ua).await?;

    Ok(Json(res))
}

#[utoipa::path(
    delete,
    path = "/admin/payment/grants/{user_id}",
    tag = "admin_payment",
    params(("user_id", Path, description = "User ID to revoke courses from")),
    responses(
        (status = 204, description = "Courses revoked"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearerAuth" = []))
)]
pub async fn revoke_grant(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Path(user_id): Path<i64>,
) -> AppResult<StatusCode> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    AdminPaymentService::revoke_grant(&st, auth_user.sub, user_id, ip, ua).await?;

    Ok(StatusCode::NO_CONTENT)
}
