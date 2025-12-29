use crate::api::health::dto::{HealthRes, ReadyRes};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use tokio::time::{timeout, Duration};

#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = 200, description = "Health check successful", body = HealthRes),
        (status = 500, description = "Health check failed", body = crate::error::ErrorBody)
    ),
    tag = "health"
)]
pub async fn health(State(state): State<AppState>) -> AppResult<Json<HealthRes>> {
    if !state.cfg.skip_db {
        let db_check_result = timeout(Duration::from_secs(2), async {
            sqlx::query("SELECT 1").execute(&state.db).await
        })
        .await;

        match db_check_result {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => {
                tracing::warn!("Health check failed: DB query error: {:?}", e);
                return Err(AppError::HealthInternal("db_query_error".to_string()));
            }
            Err(_) => {
                tracing::warn!("Health check failed: DB query timeout");
                return Err(AppError::HealthInternal("db_connect_timeout".to_string()));
            }
        }
    }

    let uptime_ms = state.started_at.elapsed().as_millis();
    let uptime_ms = u64::try_from(uptime_ms).unwrap_or(u64::MAX);

    Ok(Json(HealthRes {
        status: "live".to_string(),
        uptime_ms,
        version: format!("v{}", env!("CARGO_PKG_VERSION")),
    }))
}

#[utoipa::path(
    get,
    path = "/ready",
    responses(
        (status = 200, description = "Service is ready", body = ReadyRes),
        (status = 503, description = "Service is not ready", body = ReadyRes)
    ),
    tag = "health"
)]
pub async fn ready(State(state): State<AppState>) -> impl IntoResponse {
    let db_check_result = timeout(Duration::from_secs(2), async {
        sqlx::query("SELECT 1").execute(&state.db).await
    })
    .await;

    match db_check_result {
        Ok(Ok(_)) => (
            StatusCode::OK,
            Json(ReadyRes {
                ready: true,
                reason: None,
            }),
        ),
        Ok(Err(e)) => {
            tracing::info!("Readiness check failed: DB query error: {:?}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ReadyRes {
                    ready: false,
                    reason: Some("db_query_error".to_string()),
                }),
            )
        }
        Err(_) => {
            tracing::info!("Readiness check failed: DB query timeout");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ReadyRes {
                    ready: false,
                    reason: Some("db_connect_timeout".to_string()),
                }),
            )
        }
    }
}
