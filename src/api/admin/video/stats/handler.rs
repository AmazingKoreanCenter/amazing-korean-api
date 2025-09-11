use super::dto::{DailyStatsQuery, DailyStatsRes};
use crate::error::AppError;
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    Json,
};

/// 비디오 일별 통계 조회
#[utoipa::path(
    get,
    path = "/admin/videos/{video_id}/stats/daily",
    tag = "admin",
    params(
        ("video_id" = i64, Path, description = "Video ID"),
        ("from" = String, Query, description = "YYYY-MM-DD"),
        ("to" = String, Query, description = "YYYY-MM-DD")
    ),
    responses(
        (status = 200, description = "OK", body = DailyStatsRes),
        (status = 400, description = "Invalid date or range")
    )
)]
pub async fn admin_get_video_daily_stats(
    State(st): State<AppState>,
    Path(video_id): Path<i64>,
    Query(q): Query<DailyStatsQuery>,
) -> Result<Json<DailyStatsRes>, AppError> {
    let res = super::service::get_daily_stats(&st, video_id, q).await?;
    Ok(Json(res))
}
