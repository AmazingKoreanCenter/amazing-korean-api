use crate::error::AppResult;
use crate::state::AppState;
use axum::extract::State;
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use tokio::time::{timeout, Duration};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthRes {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReadyRes {
    pub ready: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = HealthRes)
    ),
    tag = "health"
)]
pub async fn health() -> impl IntoResponse {
    Json(HealthRes { ok: true })
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
pub async fn ready(State(state): State<AppState>) -> AppResult<Json<ReadyRes>> {
    let db_check_result = timeout(Duration::from_secs(2), async {
        sqlx::query("SELECT 1").execute(&state.db).await
    })
    .await;

    match db_check_result {
        Ok(Ok(_)) => Ok(Json(ReadyRes {
            ready: true,
            reason: None,
        })),
        Ok(Err(e)) => {
            tracing::info!("Readiness check failed: DB query error: {:?}", e);
            Ok(Json(ReadyRes {
                ready: false,
                reason: Some("db_query_error".to_string()),
            }))
        }
        Err(_) => {
            tracing::info!("Readiness check failed: DB query timeout");
            Ok(Json(ReadyRes {
                ready: false,
                reason: Some("db_connect_timeout".to_string()),
            }))
        }
    }
}
