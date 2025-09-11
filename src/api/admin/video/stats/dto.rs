use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct DailyStatsQuery {
    /// Inclusive start date (YYYY-MM-DD)
    pub from: String,
    /// Inclusive end date (YYYY-MM-DD)
    pub to: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DailyStatItem {
    pub date: chrono::NaiveDate,
    pub views: i64,
    pub watch_seconds: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DailyStatsRes {
    pub video_id: i64,
    pub from_date: chrono::NaiveDate,
    pub to_date: chrono::NaiveDate,
    pub items: Vec<DailyStatItem>,
}
