use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use cookie::time::{Duration, OffsetDateTime};

use axum::{extract::Query, response::Redirect};

use crate::api::auth::dto::{
    FindIdReq, FindIdRes, GoogleAuthUrlRes, GoogleCallbackQuery, LoginReq, LoginRes, LogoutAllReq,
    LogoutRes, /* RefreshReq, */ ResetPwReq, ResetPwRes,
};
use crate::api::auth::extractor::AuthUser;
use crate::api::auth::service::AuthService;
use crate::error::AppError;
use crate::state::AppState;

#[allow(unused_imports)]
use serde_json::json;

// =========================================================================
// Helpers
// =========================================================================

fn extract_client_ip(headers: &HeaderMap) -> String {
    // 1. x-forwarded-for
    if let Some(v) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = v.split(',').next() {
            let ip = first.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }
    // 2. x-real-ip
    if let Some(v) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        let ip = v.trim();
        if !ip.is_empty() {
            return ip.to_string();
        }
    }

    // 3. Fallback
    let use_fallback = std::env::var("AK_DEV_IP_FALLBACK")
        .ok()
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true);
    if use_fallback {
        "127.0.0.1".to_string()
    } else {
        "0.0.0.0".to_string()
    }
}

fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

// =========================================================================
// Handlers
// =========================================================================

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginReq,
    responses(
        (status = 200, description = "Login successful", body = LoginRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
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
    
    // Service returns (LoginRes, Cookie, ttl)
    let (login_res, cookie, _) = AuthService::login(&st, req, ip, ua).await?;

    // Add the fully constructed cookie from Service
    let jar = jar.add(cookie);

    Ok((jar, Json(login_res)))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    /* request_body = RefreshReq, */
    responses(
        (status = 200, description = "Token refreshed", body = LoginRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 409, description = "Reuse detected", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn refresh(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    /* Json(req): Json<RefreshReq>, */
) -> Result<(CookieJar, Json<LoginRes>), AppError> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    // 1. 쿠키에서 리프레시 토큰 추출
    let refresh_token = jar
        .get(&st.cfg.refresh_cookie_name)
        .map(|c| c.value().to_string())
        .ok_or(AppError::Unauthorized("Missing refresh token".into()))?;

    // 2. Service 호출
    let (refresh_res, new_token_str, ttl_secs) =
        AuthService::refresh(&st, &refresh_token, ip, ua).await?;

    // 3. 새 쿠키 설정 (Rotation)
    let mut refresh_cookie = Cookie::new(st.cfg.refresh_cookie_name.clone(), new_token_str);
    refresh_cookie.set_path("/");
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(st.cfg.refresh_cookie_secure);
    refresh_cookie.set_same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    });
    refresh_cookie.set_expires(cookie::time::OffsetDateTime::now_utc() + cookie::time::Duration::seconds(ttl_secs));
    
    if let Some(domain) = &st.cfg.refresh_cookie_domain {
        refresh_cookie.set_domain(domain.clone());
    }

    let jar = jar.add(refresh_cookie);

    Ok((jar, Json(refresh_res)))
}

#[utoipa::path(
    post,
    path = "/auth/find-id",
    tag = "auth",
    request_body = FindIdReq,
    responses(
        (status = 200, description = "Find ID request accepted", body = FindIdRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody)
    )
)]
pub async fn find_id(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<FindIdReq>,
) -> Result<Json<FindIdRes>, AppError> {
    let ip = extract_client_ip(&headers);
    let res = AuthService::find_id(&st, req, ip).await?;
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/auth/reset-pw",
    tag = "auth",
    request_body = ResetPwReq,
    responses(
        (status = 200, description = "Reset password success", body = ResetPwRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody)
    )
)]
pub async fn reset_password(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<ResetPwReq>,
) -> Result<Json<ResetPwRes>, AppError> {
    let ip = extract_client_ip(&headers);
    let res = AuthService::reset_password(&st, req, ip).await?;
    Ok(Json(res))
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
) -> Result<(CookieJar, StatusCode), AppError> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);
    
    AuthService::logout(&st, auth_user.sub, &auth_user.session_id, ip, ua).await?;

    // Create expiration cookie
    let mut refresh_cookie = Cookie::new(st.cfg.refresh_cookie_name.clone(), "");
    refresh_cookie.set_path("/");
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(st.cfg.refresh_cookie_secure);
    refresh_cookie.set_same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    });
    // Set expiration to the past
    refresh_cookie.set_expires(OffsetDateTime::now_utc() - Duration::days(1));
    
    if let Some(domain) = &st.cfg.refresh_cookie_domain {
        refresh_cookie.set_domain(domain.clone());
    }

    let jar = jar.add(refresh_cookie);

    Ok((jar, StatusCode::NO_CONTENT))
}

#[utoipa::path(
    post,
    path = "/auth/logout/all",
    tag = "auth",
    request_body = LogoutAllReq,
    responses(
        (status = 200, description = "Logout all successful", body = LogoutRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn logout_all(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<LogoutAllReq>,
) -> Result<(CookieJar, Json<LogoutRes>), AppError> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);
    
    // Extract refresh token from cookie to identify the current session context if needed
    let rt = jar
        .get(&st.cfg.refresh_cookie_name)
        .map(|c| c.value().to_string());

    let logout_res = AuthService::logout_all(&st, rt.as_deref(), req, ip, ua).await?;

    // Create expiration cookie
    let mut refresh_cookie = Cookie::new(st.cfg.refresh_cookie_name.clone(), "");
    refresh_cookie.set_path("/");
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(st.cfg.refresh_cookie_secure);
    refresh_cookie.set_same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    });
    refresh_cookie.set_expires(OffsetDateTime::now_utc() - Duration::days(1));
    
    if let Some(domain) = &st.cfg.refresh_cookie_domain {
        refresh_cookie.set_domain(domain.clone());
    }

    let jar = jar.add(refresh_cookie);

    Ok((jar, Json(logout_res)))
}

// =========================================================================
// Google OAuth Handlers
// =========================================================================

/// Google OAuth 시작 - 인증 URL 반환
#[utoipa::path(
    get,
    path = "/auth/google",
    tag = "auth",
    responses(
        (status = 200, description = "Google OAuth URL", body = GoogleAuthUrlRes),
        (status = 500, description = "Internal Server Error", body = crate::error::ErrorBody)
    )
)]
pub async fn google_auth_start(
    State(st): State<AppState>,
) -> Result<Json<GoogleAuthUrlRes>, AppError> {
    let auth_url = AuthService::google_auth_start(&st).await?;
    Ok(Json(GoogleAuthUrlRes { auth_url }))
}

/// Google OAuth 콜백 처리
/// 성공 시 프론트엔드로 리다이렉트 (쿠키에 토큰 포함)
#[utoipa::path(
    get,
    path = "/auth/google/callback",
    tag = "auth",
    params(
        ("code" = Option<String>, Query, description = "Authorization code"),
        ("state" = String, Query, description = "State parameter for CSRF protection"),
        ("error" = Option<String>, Query, description = "Error code if authorization failed"),
        ("error_description" = Option<String>, Query, description = "Error description")
    ),
    responses(
        (status = 302, description = "Redirect to frontend"),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    )
)]
pub async fn google_auth_callback(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Query(query): Query<GoogleCallbackQuery>,
) -> Result<(CookieJar, Redirect), AppError> {
    // 에러 처리 (사용자 취소 등)
    if let Some(error) = query.error {
        let desc = query.error_description.unwrap_or_default();
        // 에러 시 프론트엔드 로그인 페이지로 리다이렉트 (에러 정보 포함)
        let error_url = format!(
            "{}/login?error=oauth_error&error_description={}",
            st.cfg.frontend_url,
            urlencoding::encode(&format!("{}: {}", error, desc))
        );
        return Ok((jar, Redirect::temporary(&error_url)));
    }

    // Authorization Code 확인
    let code = query.code
        .ok_or_else(|| AppError::BadRequest("Missing authorization code".into()))?;

    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    // OAuth 콜백 처리
    let result = AuthService::google_auth_callback(&st, &code, &query.state, ip, ua).await;

    match result {
        Ok((login_res, cookie, _, is_new_user)) => {
            // 성공: 프론트엔드 로그인 페이지로 리다이렉트 (콜백 처리)
            let success_url = format!(
                "{}/login?login=success&user_id={}&is_new_user={}",
                st.cfg.frontend_url,
                login_res.user_id,
                is_new_user
            );
            let jar = jar.add(cookie);
            Ok((jar, Redirect::temporary(&success_url)))
        }
        Err(e) => {
            // 실패: 프론트엔드 로그인 페이지로 에러 리다이렉트
            let error_url = format!(
                "{}/login?error=oauth_failed&error_description={}",
                st.cfg.frontend_url,
                urlencoding::encode(&e.to_string())
            );
            Ok((jar, Redirect::temporary(&error_url)))
        }
    }
}