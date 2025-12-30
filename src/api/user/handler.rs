use super::{
    dto::{ProfileRes, ProfileUpdateReq, SettingsRes, SettingsUpdateReq, SignupReq, SignupRes},
    service::UserService,
};
use crate::{
    api::auth::extractor::AuthUser,
    error::{AppError, AppResult},
    state::AppState,
};
use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

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
    path = "/users",
    tag = "user",
    request_body = SignupReq,
    responses(
        (status = 201, description = "User created", body = SignupRes, example = json!({
            "user_id": 1,
            "email": "newuser@example.com",
            "name": "New User",
            "nickname": "NewNick",
            "language": "ko",
            "country": "KR",
            "birthday": "2000-01-01",
            "gender": "male",
            "user_state": "on",
            "user_auth": "learner",
            "created_at": "2025-08-21T10:00:00Z",
            "access": {
                "access_token": "eyJ...",
                "expires_in": 3600
            },
            "session_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef"
        })),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 409, description = "Email already exists", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable entity", body = crate::error::ErrorBody),
        (status = 429, description = "Too many requests", body = crate::error::ErrorBody)
    )
)]
pub async fn signup(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(req): Json<SignupReq>,
) -> AppResult<(CookieJar, (StatusCode, HeaderMap, Json<SignupRes>))> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);

    let (res, refresh_token, refresh_ttl_secs) = UserService::signup(&st, req, ip, ua).await?;

    let refresh_cookie = Cookie::build(Cookie::new(
        st.cfg.refresh_cookie_name.to_string(),
        refresh_token,
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
    .expires(
        cookie::time::OffsetDateTime::now_utc()
            + cookie::time::Duration::seconds(refresh_ttl_secs),
    )
    .domain(st.cfg.refresh_cookie_domain.clone().unwrap_or_default())
    .build();

    let jar = jar.add(refresh_cookie);

    let mut resp_headers = HeaderMap::new();
    let location = format!("/users/{}", res.user_id);
    let location_val = HeaderValue::from_str(&location)
        .map_err(|e| AppError::Internal(format!("Invalid Location header: {e}")))?;
    resp_headers.insert(axum::http::header::LOCATION, location_val);

    Ok((jar, (StatusCode::CREATED, resp_headers, Json(res))))
}

#[utoipa::path(
    get,
    path = "/users/me",
    tag = "user",
    responses(
        (status = 200, description = "User profile", body = ProfileRes, example = json!({
            "id": 1,
            "email": "user@example.com",
            "name": "Existing User",
            "nickname": "ExistingNick",
            "language": "ko",
            "country": "KR",
            "birthday": "1990-05-10",
            "gender": "female",
            "user_state": "on",
            "user_auth": "learner",
            "created_at": "2025-08-21T10:00:00Z"
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_me(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
) -> AppResult<Json<ProfileRes>> {
    let user = UserService::get_me(&st, auth_user.sub).await?;
    Ok(Json(user))
}

#[utoipa::path(
    post,
    path = "/users/me",
    tag = "user",
    request_body = ProfileUpdateReq,
    responses(
        (status = 200, description = "User profile updated", body = ProfileRes, example = json!({
            "id": 1,
            "email": "user@example.com",
            "name": "Existing User",
            "nickname": "UpdatedNick",
            "language": "ko",
            "country": "KR",
            "birthday": "1990-05-10",
            "gender": "female",
            "user_state": "on",
            "user_auth": "learner",
            "created_at": "2025-08-21T10:00:00Z"
        })),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn update_me(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Json(req): Json<ProfileUpdateReq>,
) -> AppResult<Json<ProfileRes>> {
    let user = UserService::update_me(&st, auth_user.sub, req).await?;
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/users/me/settings",
    tag = "user",
    responses(
        (status = 200, description = "User settings", body = SettingsRes, example = json!({
            "user_set_language": "ko",
            "user_set_timezone": "UTC",
            "user_set_note_email": false,
            "user_set_note_push": false,
            "updated_at": "2025-08-21T10:00:00Z"
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_settings(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
) -> AppResult<Json<SettingsRes>> {
    let settings = UserService::get_settings(&st, auth_user.sub).await?;
    Ok(Json(settings))
}

#[utoipa::path(
    post,
    path = "/users/me/settings",
    tag = "user",
    request_body = SettingsUpdateReq,
    responses(
        (status = 200, description = "User settings updated", body = SettingsRes, example = json!({
            "user_set_language": "ko",
            "user_set_timezone": "UTC",
            "user_set_note_email": false,
            "user_set_note_push": false,
            "updated_at": "2025-08-21T10:00:00Z"
        })),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn update_settings(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Json(req): Json<SettingsUpdateReq>,
) -> AppResult<Json<SettingsRes>> {
    let settings = UserService::update_settings(&st, auth_user.sub, req).await?;
    Ok(Json(settings))
}
