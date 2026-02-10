use super::{
    dto::{ProfileRes, ProfileUpdateReq, SettingsRes, SettingsUpdateReq, SignupReq, SignupRes},
    service::UserService,
};
use crate::{
    api::auth::extractor::AuthUser,
    error::AppResult,
    state::AppState,
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

// -------------------------------------------------------------------------
// 1. 회원가입 (POST /users)
// -------------------------------------------------------------------------
#[utoipa::path(
    post,
    path = "/users",
    tag = "user",
    request_body = SignupReq,
    responses(
        (status = 201, description = "회원가입 성공 (이메일 인증 필요)", body = SignupRes, example = json!({
            "message": "Verification code sent to your email.",
            "requires_verification": true
        })),
        (status = 400, description = "잘못된 요청", body = crate::error::ErrorBody),
        (status = 409, description = "이메일 중복 (인증 완료된 계정)", body = crate::error::ErrorBody),
        (status = 422, description = "유효성 검증 실패", body = crate::error::ErrorBody),
        (status = 429, description = "요청 횟수 초과", body = crate::error::ErrorBody)
    )
)]
pub async fn signup(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<SignupReq>,
) -> AppResult<(StatusCode, Json<SignupRes>)> {
    let ip = extract_client_ip(&headers);

    let res = UserService::signup(&st, req, ip).await?;

    Ok((StatusCode::CREATED, Json(res)))
}

// -------------------------------------------------------------------------
// 2. 내 정보 조회 (GET /users/me)
// -------------------------------------------------------------------------
#[utoipa::path(
    get,
    path = "/users/me",
    tag = "user",
    responses(
        (status = 200, description = "내 프로필 조회 성공", body = ProfileRes, example = json!({
            "id": 1,
            "email": "user@example.com",
            "name": "Existing User",
            "nickname": "ExistingNick",
            "language": "ko",
            "country": "KR",
            "birthday": "1990-05-10",
            "gender": "female",
            "user_state": true,
            "user_auth": "learner",
            "created_at": "2025-08-21T10:00:00Z"
        })),
        (status = 401, description = "인증 실패 (토큰 만료/없음)", body = crate::error::ErrorBody),
        (status = 404, description = "사용자 정보 없음", body = crate::error::ErrorBody)
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

// -------------------------------------------------------------------------
// 3. 내 정보 수정 (POST /users/me)
// -------------------------------------------------------------------------
#[utoipa::path(
    post,
    path = "/users/me",
    tag = "user",
    request_body = ProfileUpdateReq,
    responses(
        (status = 200, description = "프로필 수정 성공", body = ProfileRes),
        (status = 400, description = "잘못된 요청", body = crate::error::ErrorBody),
        (status = 422, description = "유효성 검증 실패", body = crate::error::ErrorBody)
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

// -------------------------------------------------------------------------
// 4. 환경설정 조회 (GET /users/me/settings)
// -------------------------------------------------------------------------
#[utoipa::path(
    get,
    path = "/users/me/settings",
    tag = "user",
    responses(
        (status = 200, description = "설정 조회 성공", body = SettingsRes, example = json!({
            "user_set_language": "ko",
            "user_set_timezone": "UTC",
            "user_set_note_email": false,
            "user_set_note_push": false,
            "updated_at": "2025-08-21T10:00:00Z"
        })),
        (status = 401, description = "인증 실패", body = crate::error::ErrorBody)
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

// -------------------------------------------------------------------------
// 5. 환경설정 수정 (POST /users/me/settings)
// -------------------------------------------------------------------------
#[utoipa::path(
    post,
    path = "/users/me/settings",
    tag = "user",
    request_body = SettingsUpdateReq,
    responses(
        (status = 200, description = "설정 수정 성공", body = SettingsRes),
        (status = 400, description = "잘못된 요청", body = crate::error::ErrorBody),
        (status = 401, description = "인증 실패", body = crate::error::ErrorBody)
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

// =========================================================================
// Utilities
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
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true);
        
    if use_fallback { "127.0.0.1".to_string() } else { "0.0.0.0".to_string() }
}

