// FILE: src/api/auth/handler.rs
use axum::http::HeaderMap;
use axum::{extract::State, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use cookie::time::{Duration, OffsetDateTime}; // Explicitly import from cookie crate

use crate::api::auth::dto::{AccessTokenRes, LoginReq, LoginRes, LogoutAllReq, LogoutRes};
use crate::api::auth::extractor::AuthUser;
use crate::api::auth::service::AuthService;
use crate::error::AppError;
use crate::state::AppState;

#[allow(unused_imports)]
use serde_json::json;

fn extract_client_ip(headers: &HeaderMap) -> String {
    if let Some(v) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = v.split(',').next() {
            let ip = first.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }
    if let Some(v) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        let ip = v.trim();
        if !ip.is_empty() {
            return ip.to_string();
        }
    }

    let use_fallback = std::env::var("AK_DEV_IP_FALLBACK")
        .ok()
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true);
    if use_fallback {
        "127.0.0.1".to_string()
    } else {
        "0.0.0.0".to_string() // Should not be reached if fallback is true.
    }
}

fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginReq,
    responses(
        (status = 200, description = "Login successful", body = LoginRes, example = json!({
            "user_id": 1,
            "access": {
                "access_token": "eyJ...",
                "expires_in": 3600
            },
            "session_id": "some-uuid"
        })),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody)
    )
)]
pub async fn login(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<LoginReq>,
) -> Result<(CookieJar, Json<LoginRes>), AppError> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);
    let (login_res, refresh_token_string, refresh_ttl_secs) =
        AuthService::login(&st, req, ip, ua).await?;

    let refresh_cookie = Cookie::build(Cookie::new(
        st.cfg.refresh_cookie_name.to_string(),
        refresh_token_string.to_string(),
    ))
    .path("/")
    .http_only(true)
    .secure(st.cfg.refresh_cookie_secure)
    .same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax, // Default to Lax
    })
    .expires(OffsetDateTime::now_utc() + Duration::seconds(refresh_ttl_secs))
    .domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default())
    .build();

    let jar = jar.add(refresh_cookie);

    Ok((jar, Json(login_res)))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    responses(
        (status = 200, description = "Token refreshed", body = AccessTokenRes, example = json!({
            "access_token": "eyJ...",
            "expires_in": 3600
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn refresh(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<(CookieJar, Json<AccessTokenRes>), AppError> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);
    let rt = jar
        .get(&st.cfg.refresh_cookie_name)
        .map(|c| c.value().to_string());

    let refresh_token_str = rt.ok_or(AppError::Unauthorized("AUTH_401_INVALID_REFRESH".into()))?;

    let (refresh_res, new_refresh_token_string, refresh_ttl_secs) =
        AuthService::refresh(&st, &refresh_token_str, ip, ua).await?;

    let refresh_cookie = Cookie::build(Cookie::new(
        st.cfg.refresh_cookie_name.to_string(),
        new_refresh_token_string.to_string(),
    ))
    .path("/")
    .http_only(true)
    .secure(st.cfg.refresh_cookie_secure)
    .same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax, // Default to Lax
    })
    .expires(OffsetDateTime::now_utc() + Duration::seconds(refresh_ttl_secs))
    .domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default())
    .build();

    let jar = jar.add(refresh_cookie);

    Ok((
        jar,
        Json(AccessTokenRes {
            access_token: refresh_res.access_token,
            expires_in: refresh_res.expires_in,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    responses(
        (status = 204, description = "Logout successful"),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn logout(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<(CookieJar, axum::http::StatusCode), AppError> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);
    AuthService::logout(&st, auth_user.sub, &auth_user.session_id, ip, ua).await?;

    let refresh_cookie = Cookie::build(Cookie::new(
        st.cfg.refresh_cookie_name.to_string(),
        "".to_string(),
    ))
    .path("/")
    .http_only(true)
    .secure(st.cfg.refresh_cookie_secure)
    .same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax, // Default to Lax
    })
    .expires(OffsetDateTime::now_utc() - Duration::days(1))
    .domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default())
    .build();

    let jar = jar.add(refresh_cookie);

    Ok((jar, axum::http::StatusCode::NO_CONTENT))
}

#[utoipa::path(
    post,
    path = "/auth/logout-all",
    tag = "auth",
    request_body = LogoutAllReq,
    responses(
        (status = 200, description = "Logout all successful", body = LogoutRes, example = json!({
            "ok": true
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn logout_all(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<LogoutAllReq>,
) -> Result<Json<LogoutRes>, AppError> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);
    let rt = jar
        .get(&st.cfg.refresh_cookie_name)
        .map(|c| c.value().to_string());

    let logout_res = AuthService::logout_all(&st, rt.as_deref(), req, ip, ua).await?;

    let refresh_cookie = Cookie::build(Cookie::new(
        st.cfg.refresh_cookie_name.to_string(),
        "".to_string(),
    ))
    .path("/")
    .http_only(true)
    .secure(st.cfg.refresh_cookie_secure)
    .same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax, // Default to Lax
    })
    .expires(OffsetDateTime::now_utc() - Duration::days(1))
    .domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default())
    .build();

    let _jar = jar.add(refresh_cookie); // _jar to suppress unused variable warning if jar is not returned.

    Ok(Json(logout_res))
}
