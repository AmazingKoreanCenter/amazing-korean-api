use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};
use super::{
    dto::{LoginReq, LoginResp, SignUpReq, UserOut},
    jwt,
    service::AuthService,
};

#[utoipa::path(
    post,
    path = "/auth/signup",
    tag = "auth",
    request_body = SignUpReq,
    responses(
        (status = 201, description = "User created"),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 409, description = "Email already exists", body = crate::error::ErrorBody)
    )
)]
pub async fn signup(
    State(st): State<AppState>,
    Json(req): Json<SignUpReq>,
) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // ✨ 시그니처에 맞게 개별 인자 전달
    AuthService::signup(
        &st,
        req.email.as_str(),
        req.password.as_str(),
        req.name.as_str(),
        req.terms_service,
        req.terms_personal,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({"ok": true}))))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginReq,
    responses(
        (status = 200, body = LoginResp),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    )
)]
pub async fn login(
    State(st): State<AppState>,
    Json(req): Json<LoginReq>,
) -> AppResult<Json<LoginResp>> {
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // ✨ 서비스 반환은 (access_token, expires_in, user) 튜플이므로 구조분해
    let (access_token, expires_in, _user): (String, i64, UserOut) =
        AuthService::login(&st, req.email.as_str(), req.password.as_str()).await?;

    let resp = LoginResp {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in,
    };
    Ok(Json(resp))
}

#[utoipa::path(
    get,
    path = "/auth/me",
    tag = "auth",
    responses(
        (status = 200, body = UserOut),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn me(
    State(st): State<AppState>,
    headers: HeaderMap,
) -> AppResult<Json<UserOut>> {
    let token = bearer_from_headers(&headers)?;
    let claims =
        jwt::decode_token(&token).map_err(|_| AppError::Unauthorized("invalid token".into()))?;
    let user = AuthService::me(&st, claims.sub).await?;
    Ok(Json(user))
}

/// Authorization: Bearer <token> 헤더에서 토큰 추출
fn bearer_from_headers(headers: &HeaderMap) -> AppResult<String> {
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
