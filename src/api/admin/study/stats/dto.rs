use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ==========================================
// Query Parameters
// ==========================================

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct StatsQuery {
    /// Inclusive start date (YYYY-MM-DD)
    pub from: String,
    /// Inclusive end date (YYYY-MM-DD)
    pub to: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct TopStudiesQuery {
    /// Inclusive start date (YYYY-MM-DD)
    pub from: String,
    /// Inclusive end date (YYYY-MM-DD)
    pub to: String,
    /// Number of items to return (default: 10, max: 50)
    #[serde(default = "default_limit")]
    pub limit: i32,
    /// Sort by: "attempts" | "solves" | "solve_rate" (default: attempts)
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
}

fn default_limit() -> i32 {
    10
}

fn default_sort_by() -> String {
    "attempts".to_string()
}

// ==========================================
// Response DTOs
// ==========================================

/// Program distribution statistics
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProgramStats {
    pub basic_pronunciation: i64,
    pub basic_word: i64,
    pub basic_900: i64,
    pub topik_read: i64,
    pub topik_listen: i64,
    pub topik_write: i64,
    pub tbc: i64,
}

/// State distribution statistics
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct StateStats {
    pub ready: i64,
    pub open: i64,
    pub close: i64,
}

/// Summary statistics response
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct StudyStatsSummaryRes {
    /// Total number of studies
    pub total_studies: i64,
    /// Number of open (active) studies
    pub open_studies: i64,
    /// Total number of tasks across all studies
    pub total_tasks: i64,
    /// Total attempts (from study_task_status)
    pub total_attempts: i64,
    /// Total solved tasks
    pub total_solves: i64,
    /// Solve rate percentage
    pub solve_rate: f64,
    /// Distribution by program
    pub by_program: ProgramStats,
    /// Distribution by state
    pub by_state: StateStats,
    /// Query start date
    pub from_date: chrono::NaiveDate,
    /// Query end date
    pub to_date: chrono::NaiveDate,
}

/// Top study item
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TopStudyItem {
    pub rank: i32,
    pub study_id: i64,
    pub study_idx: String,
    pub study_title: Option<String>,
    pub study_program: String,
    pub task_count: i64,
    pub attempt_count: i64,
    pub solve_count: i64,
    pub solve_rate: f64,
}

/// Top studies response
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TopStudiesRes {
    pub from_date: chrono::NaiveDate,
    pub to_date: chrono::NaiveDate,
    pub sort_by: String,
    pub items: Vec<TopStudyItem>,
}

/// Daily statistics item
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DailyStatItem {
    pub date: chrono::NaiveDate,
    pub attempts: i64,
    pub solves: i64,
    pub active_users: i64,
}

/// Daily statistics response
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DailyStatsRes {
    pub from_date: chrono::NaiveDate,
    pub to_date: chrono::NaiveDate,
    pub items: Vec<DailyStatItem>,
}
