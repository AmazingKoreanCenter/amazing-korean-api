use super::{
    dto::{ProfileRes, SettingsRes, SettingsUpdateReq, SignupReq, UpdateReq},
    service::UserService,
};
use crate::{
    api::auth::jwt,
    error::{AppError, AppResult},
    state::AppState,
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

// ← 어트리뷰트 내의 json! 매크로를 위해 필요
#[allow(unused_imports)]
use serde_json::json;

/// Authorization: Bearer <token> 헤더에서 토큰 추출
pub fn bearer_from_headers(headers: &HeaderMap) -> AppResult<String> {
    let auth = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("missing Authorization header".into()))?;

    let Some(rest) = auth.strip_prefix("Bearer ") else {
        return Err(AppError::Unauthorized(
            "invalid Authorization scheme".into(),
        ));
    };
    Ok(rest.to_string())
}

#[utoipa::path(
    post,
    path = "/users",
    tag = "user",
    request_body = SignupReq,
    responses(
        // 예시는 순수 JSON 값으로!
        (status = 201, description = "User created", body = ProfileRes, example = json!({
            "id": 1,
            "email": "newuser@example.com",
            "name": "New User",
            "nickname": "NewNick",
            "language": "en",
            "country": "US",
            "birthday": "2000-01-01",
            "gender": "male",
            "user_state": "on",
            "user_auth": "learner",
            "created_at": "2025-08-21T10:00:00Z"
        })),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody, example = json!({
            "error": "Validation error: email is not valid"
        })),
        (status = 409, description = "Email already exists", body = crate::error::ErrorBody, example = json!({
            "error": "email already exists"
        }))
    )
)]

// 회원가입 handler
pub async fn signup(
    State(st): State<AppState>,
    Json(req): Json<SignupReq>,
) -> AppResult<(StatusCode, Json<ProfileRes>)> {
    let user = UserService::signup(&st, req).await?;
    Ok((StatusCode::CREATED, Json(user)))
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
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody, example = json!({
            "error": "missing Authorization header"
        })),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody, example = json!({
            "error": "forbidden"
        })),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody, example = json!({
            "error": "not found"
        }))
    ),
    security(("bearerAuth" = []))
)]

// 프로필 조회 handler
pub async fn get_me(State(st): State<AppState>, headers: HeaderMap) -> AppResult<Json<ProfileRes>> {
    let token = bearer_from_headers(&headers)?;
    let claims =
        jwt::decode_token(&token).map_err(|_| AppError::Unauthorized("invalid token".into()))?;
    let user = UserService::get_me(&st, claims.sub).await?;
    Ok(Json(user))
}

#[utoipa::path(
    put,
    path = "/users/me",
    tag = "user",
    request_body = UpdateReq,
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
        (status = 400, description = "Bad request", body = crate::error::ErrorBody, example = json!({
            "error": "Validation error: nickname length must be between 1 and 100"
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody, example = json!({
            "error": "missing Authorization header"
        })),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody, example = json!({
            "error": "forbidden"
        })),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody, example = json!({
            "error": "not found"
        }))
    ),
    security(("bearerAuth" = []))
)]

// 프로필 수정 handler
pub async fn update_me(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UpdateReq>,
) -> AppResult<Json<ProfileRes>> {
    let token = bearer_from_headers(&headers)?;
    let claims =
        jwt::decode_token(&token).map_err(|_| AppError::Unauthorized("invalid token".into()))?;
    let user = UserService::update_me(&st, claims.sub, req).await?;
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/users/me/settings",
    tag = "user",
    responses(
        (status = 200, description = "User settings", body = SettingsRes, example = json!({
            "user_id": 1,
            "ui_language": "ko",
            "timezone": "Asia/Seoul",
            "notifications_email": true,
            "notifications_push": false,
            "study_languages": [
                {"lang_code":"en","priority":1,"is_primary":false},
                {"lang_code":"ko","priority":2,"is_primary":true}
            ]
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody, example = json!({
            "error": "missing Authorization header"
        })),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody, example = json!({
            "error": "forbidden"
        })),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody, example = json!({
            "error": "not found"
        }))
    ),
    security(("bearerAuth" = []))
)]

// 환경설정 조회 handler
pub async fn get_settings(
    State(st): State<AppState>,
    headers: HeaderMap,
) -> AppResult<Json<SettingsRes>> {
    let token = bearer_from_headers(&headers)?;
    let claims =
        jwt::decode_token(&token).map_err(|_| AppError::Unauthorized("invalid token".into()))?;
    let settings = UserService::get_settings(&st, claims.sub).await?;
    Ok(Json(settings))
}

#[utoipa::path(
    put,
    path = "/users/me/settings",
    tag = "user",
    request_body = SettingsUpdateReq,
    responses(
        (status = 200, description = "User settings updated", body = SettingsRes, example = json!({
            "user_id": 1,
            "ui_language": "ko",
            "timezone": "Asia/Seoul",
            "notifications_email": true,
            "notifications_push": false,
            "study_languages": [
                {"lang_code":"en","priority":1,"is_primary":false},
                {"lang_code":"ko","priority":2,"is_primary":true}
            ]
        })),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody, example = json!({
            "error": "Validation error: ui_language is not valid"
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody, example = json!({
            "error": "missing Authorization header"
        })),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody, example = json!({
            "error": "forbidden"
        })),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody, example = json!({
            "error": "not found"
        }))
    ),
    security(("bearerAuth" = []))
)]

// 환경설정 수정 handler
pub async fn update_user_settings(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<SettingsUpdateReq>,
) -> AppResult<Json<SettingsRes>> {
    let token = bearer_from_headers(&headers)?;
    let claims =
        jwt::decode_token(&token).map_err(|_| AppError::Unauthorized("invalid token".into()))?;
    let settings = UserService::update_user_settings(&st, claims.sub, req).await?;
    Ok(Json(settings))
}
