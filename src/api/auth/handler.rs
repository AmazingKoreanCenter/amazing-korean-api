use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use cookie::time::{Duration, OffsetDateTime};

use axum::{extract::Query, response::Redirect};

use crate::api::auth::dto::{
    FindIdReq, FindIdRes, FindPasswordReq, FindPasswordRes,
    GoogleAuthUrlRes, GoogleCallbackQuery, LoginReq, LoginRes, LogoutAllReq,
    LogoutRes, /* RefreshReq, */ ResetPwReq, ResetPwRes,
    RequestResetReq, RequestResetRes, VerifyResetReq, VerifyResetRes,
    VerifyEmailReq, VerifyEmailRes, ResendVerificationReq, ResendVerificationRes,
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

/// User-Agent 서버사이드 파싱 결과
pub struct ParsedUa {
    pub os: Option<String>,
    pub browser: Option<String>,
    pub device: String,  // "desktop", "mobile", "other"
}

pub fn parse_user_agent(headers: &HeaderMap) -> ParsedUa {
    let ua_str = headers.get("user-agent").and_then(|v| v.to_str().ok());

    let Some(ua) = ua_str else {
        return ParsedUa { os: None, browser: None, device: "other".into() };
    };

    let parser = woothee::parser::Parser::new();
    match parser.parse(ua) {
        Some(result) => ParsedUa {
            os: if result.os != "UNKNOWN" { Some(result.os.to_string()) } else { None },
            browser: if result.name != "UNKNOWN" { Some(result.name.to_string()) } else { None },
            device: match result.category {
                "pc" => "desktop",
                "smartphone" | "mobilephone" => "mobile",
                _ => "other",
            }.into(),
        },
        None => ParsedUa { os: None, browser: None, device: "other".into() },
    }
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
    let parsed_ua = parse_user_agent(&headers);

    // Service returns (LoginRes, Cookie, ttl)
    let (login_res, cookie, _) = AuthService::login(&st, req, ip, ua, parsed_ua).await?;

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

/// 비밀번호 찾기 (본인 확인 후 인증코드 발송)
#[utoipa::path(
    post,
    path = "/auth/find-password",
    tag = "auth",
    request_body = FindPasswordReq,
    responses(
        (status = 200, description = "Request processed", body = FindPasswordRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody)
    )
)]
pub async fn find_password(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<FindPasswordReq>,
) -> Result<Json<FindPasswordRes>, AppError> {
    let ip = extract_client_ip(&headers);
    let res = AuthService::find_password(&st, req, ip).await?;
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
        (status = 401, description = "Invalid or expired token", body = crate::error::ErrorBody),
        (status = 422, description = "Password policy violation", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody)
    )
)]
pub async fn reset_password(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<ResetPwReq>,
) -> Result<Json<ResetPwRes>, AppError> {
    let ip = extract_client_ip(&headers);
    // 새 서비스 메서드 사용 (JWT 토큰 + Redis 토큰 모두 지원)
    let res = AuthService::reset_password_with_token(&st, &req.reset_token, &req.new_password, ip).await?;
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
// Password Reset Handlers (이메일 인증 기반)
// =========================================================================

/// 비밀번호 재설정 요청 - 이메일로 인증코드 발송
#[utoipa::path(
    post,
    path = "/auth/request-reset",
    tag = "auth",
    request_body = RequestResetReq,
    responses(
        (status = 200, description = "Reset request accepted", body = RequestResetRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody),
        (status = 500, description = "Internal Server Error", body = crate::error::ErrorBody)
    )
)]
pub async fn request_reset(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<RequestResetReq>,
) -> Result<Json<RequestResetRes>, AppError> {
    let ip = extract_client_ip(&headers);
    let res = AuthService::request_password_reset(&st, &req.email, ip).await?;
    Ok(Json(res))
}

/// 인증코드 검증 - reset_token 발급
#[utoipa::path(
    post,
    path = "/auth/verify-reset",
    tag = "auth",
    request_body = VerifyResetReq,
    responses(
        (status = 200, description = "Code verified, token issued", body = VerifyResetRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Invalid or expired code", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody)
    )
)]
pub async fn verify_reset(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<VerifyResetReq>,
) -> Result<Json<VerifyResetRes>, AppError> {
    let ip = extract_client_ip(&headers);
    let res = AuthService::verify_reset_code(&st, &req.email, &req.code, ip).await?;
    Ok(Json(res))
}

// =========================================================================
// Email Verification Handlers (회원가입 이메일 인증)
// =========================================================================

/// 이메일 인증코드 확인
#[utoipa::path(
    post,
    path = "/auth/verify-email",
    tag = "auth",
    request_body = VerifyEmailReq,
    responses(
        (status = 200, description = "Email verified", body = VerifyEmailRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Invalid or expired code", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody)
    )
)]
pub async fn verify_email(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<VerifyEmailReq>,
) -> Result<Json<VerifyEmailRes>, AppError> {
    let ip = extract_client_ip(&headers);
    let res = AuthService::verify_email(&st, req, ip).await?;
    Ok(Json(res))
}

/// 이메일 인증코드 재발송
#[utoipa::path(
    post,
    path = "/auth/resend-verification",
    tag = "auth",
    request_body = ResendVerificationReq,
    responses(
        (status = 200, description = "Verification code resent", body = ResendVerificationRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 429, description = "Too Many Requests", body = crate::error::ErrorBody),
        (status = 503, description = "Email service unavailable", body = crate::error::ErrorBody)
    )
)]
pub async fn resend_verification(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<ResendVerificationReq>,
) -> Result<Json<ResendVerificationRes>, AppError> {
    let ip = extract_client_ip(&headers);
    let res = AuthService::resend_verification(&st, req, ip).await?;
    Ok(Json(res))
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
    let parsed_ua = parse_user_agent(&headers);

    // OAuth 콜백 처리
    let result = AuthService::google_auth_callback(&st, &code, &query.state, ip, ua, parsed_ua).await;

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