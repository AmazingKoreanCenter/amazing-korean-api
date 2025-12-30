// FILE: src/api/auth/handler.rs
use axum::http::HeaderMap;
use axum::{extract::State, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar}; // SameSite 등 불필요한 import 제거
// use cookie::time::{Duration, OffsetDateTime}; // <-- 불필요하므로 제거 (Service에서 처리함)

use crate::api::auth::dto::{
    FindIdReq, FindIdRes, LoginReq, LoginRes, LogoutAllReq, LogoutRes, RefreshReq,
};
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
        "0.0.0.0".to_string()
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
    
    // [수정] Service가 이미 완성된 Cookie 객체를 반환합니다.
    // 3번째 리턴값(ttl)은 Handler에서 안 쓰므로 무시(_)합니다.
    let (login_res, cookie, _) = AuthService::login(&st, req, ip, ua).await?;

    // [수정] 복잡한 빌더 로직 제거 -> 바로 jar에 추가
    let jar = jar.add(cookie);

    Ok((jar, Json(login_res)))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    request_body = RefreshReq,
    responses(
        (status = 200, description = "Token refreshed", body = LoginRes, example = json!({
            "user_id": 1,
            "access": {
                "access_token": "eyJ...",
                "expires_in": 3600
            },
            "session_id": "some-uuid"
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 409, description = "Reuse detected", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn refresh(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<RefreshReq>,
) -> Result<(CookieJar, Json<LoginRes>), AppError> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    // [수정] Refresh 함수는 (LoginRes, String, i64)를 반환하므로 여기서는 쿠키를 직접 만들어야 합니다.
    // Service.rs를 다시 보니 refresh는 Cookie가 아니라 String(token_value)을 반환합니다.
    // 따라서 기존 로직을 유지하되, 변수명을 명확히 하고 로직을 확인합니다.
    
    /* Wait! 앞서 service.rs를 보면 refresh는 `(LoginRes, String, i64)`를 반환합니다. 
       String은 'new_refresh_token_value' (순수 값) 입니다.
       그러므로 아래 로직은 기존처럼 'Cookie::build'를 써야 합니다.
       단, Service와 통일성을 위해 Cookie 생성 로직을 확인합니다.
    */

    let (refresh_res, new_refresh_token_string, refresh_ttl_secs) =
        AuthService::refresh(&st, &req.refresh_token, ip, ua).await?;

    // Refresh는 Service가 쿠키를 안 만들어주므로 Handler가 만듭니다.
    // 여기서 new_refresh_token_string은 순수 토큰값이므로 아래 코드는 정상입니다.
    // 단, import 문제(Duration, OffsetDateTime) 해결을 위해 axum_extra의 cookie가 아닌
    // cookie crate의 타입을 명시적으로 써야 할 수도 있습니다.
    
    // [중요] AuthService::login과 통일성을 위해, AuthService::refresh도 Cookie를 반환하도록
    // Service를 고치는 게 베스트지만, 일단 Handler에서 처리하겠습니다.
    
    use cookie::time::{Duration, OffsetDateTime}; // 로컬 import
    use axum_extra::extract::cookie::SameSite;

    let refresh_cookie = Cookie::build(Cookie::new(
        st.cfg.refresh_cookie_name.to_string(),
        new_refresh_token_string, // 순수 String 값
    ))
    .path("/")
    .http_only(true)
    .secure(st.cfg.refresh_cookie_secure)
    .same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    })
    .expires(OffsetDateTime::now_utc() + Duration::seconds(refresh_ttl_secs))
    .domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default())
    .build();

    let jar = jar.add(refresh_cookie);

    Ok((jar, Json(refresh_res)))
}

#[utoipa::path(
    post,
    path = "/auth/find-id",
    tag = "auth",
    request_body = FindIdReq,
    responses(
        (status = 200, description = "Find ID request accepted", body = FindIdRes, example = json!({
            "message": "If the account exists, the ID has been sent to your email."
        })),
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

    // [수정] 만료 쿠키 생성
    use cookie::time::{Duration, OffsetDateTime};
    use axum_extra::extract::cookie::SameSite;

    let refresh_cookie = Cookie::build(Cookie::new(
        st.cfg.refresh_cookie_name.to_string(),
        "",
    ))
    .path("/")
    .http_only(true)
    .secure(st.cfg.refresh_cookie_secure)
    .same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    })
    .expires(OffsetDateTime::now_utc() - Duration::days(1)) // 과거 시간으로 설정
    .domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default())
    .build();

    let jar = jar.add(refresh_cookie);

    Ok((jar, axum::http::StatusCode::NO_CONTENT))
}

// logout_all도 logout과 동일한 로직으로 만료 쿠키 처리 (생략 가능하나 위 패턴 따름)
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
) -> Result<(CookieJar, Json<LogoutRes>), AppError> { // 1. 리턴 타입 변경 (CookieJar 추가)
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);
    let rt = jar
        .get(&st.cfg.refresh_cookie_name)
        .map(|c| c.value().to_string());

    let logout_res = AuthService::logout_all(&st, rt.as_deref(), req, ip, ua).await?;

    use axum_extra::extract::cookie::SameSite;
    use cookie::time::{Duration, OffsetDateTime};

    // 2. 만료 쿠키 생성
    let refresh_cookie = Cookie::build(Cookie::new(
        st.cfg.refresh_cookie_name.to_string(),
        "",
    ))
    .path("/")
    .http_only(true)
    .secure(st.cfg.refresh_cookie_secure)
    .same_site(match st.cfg.refresh_cookie_samesite_or("Lax") {
        "Strict" => SameSite::Strict,
        "Lax" => SameSite::Lax,
        "None" => SameSite::None,
        _ => SameSite::Lax,
    })
    .expires(OffsetDateTime::now_utc() - Duration::days(1))
    .domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default())
    .build();

    // 3. Jar에 쿠키 추가
    let jar = jar.add(refresh_cookie);

    // 4. Jar와 함께 응답 반환
    Ok((jar, Json(logout_res)))
}
