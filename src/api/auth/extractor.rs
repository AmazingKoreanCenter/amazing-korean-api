use axum::extract::{FromRef, FromRequestParts};
use axum::http::{header::AUTHORIZATION, request::Parts};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::api::auth::jwt::Claims;
use crate::error::AppError;
use crate::state::AppState;

pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    // 상위 라우터의 state(S)에서 AppState를 꺼낼 수 있어야 함
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    // 트레이트 원형: fn -> impl Future + Send (async fn 아님)
    fn from_request_parts(parts: &mut Parts, state: &S)
        -> impl core::future::Future<Output = Result<Self, Self::Rejection>> + Send
    {
        // AppState는 소유값으로 얻어와 async move 에 캡쳐
        let app_state = AppState::from_ref(state);
        let secret = app_state.cfg.jwt_secret.clone();

        // 헤더는 참조이므로 미리 복사(cloned)해서 소유값으로 만든 뒤 async move로 넘김
        let auth_header = parts.headers.get(AUTHORIZATION).cloned();

        async move {
            // Authorization: Bearer <token>
            let token = auth_header
                .and_then(|h| h.to_str().ok().map(|s| s.to_string()))
                .and_then(|s| s.strip_prefix("Bearer ").map(|t| t.to_string()))
                .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".into()))?;

            let claims = decode(
                &token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            )
            .map_err(|_| AppError::Unauthorized("Invalid token".into()))?
            .claims;

            Ok(AuthUser(claims))
        }
    }
}
