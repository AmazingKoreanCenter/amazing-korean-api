use axum::{extract::State, http::StatusCode, Json};
use crate::{
    error::AppResult,
    state::AppState,
};
use super::{dto::CreateUserReq, service::UserService};

#[utoipa::path(
    post,
    path = "/users",
    tag = "user",
    request_body = CreateUserReq,
    responses(
        (status = 201, description = "User created", body = crate::api::auth::dto::UserOut),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 409, description = "Email already exists", body = crate::error::ErrorBody)
    )
)]
pub async fn create_user(
    State(st): State<AppState>,
    Json(req): Json<CreateUserReq>,
) -> AppResult<(StatusCode, Json<crate::api::auth::dto::UserOut>)> {
    let user = UserService::create_user(&st, req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}
