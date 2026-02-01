use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ==========================================
// 기존: 특정 비디오 일별 통계
// ==========================================

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
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
    pub completes: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DailyStatsRes {
    pub video_id: i64,
    pub from_date: chrono::NaiveDate,
    pub to_date: chrono::NaiveDate,
    pub items: Vec<DailyStatItem>,
}

// ==========================================
// 신규: 전체 통계 대시보드용
// ==========================================

/// 전체 통계 요약 응답
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct StatsSummaryRes {
    /// 기간 내 총 조회수
    pub total_views: i64,
    /// 기간 내 총 완료수
    pub total_completes: i64,
    /// 활성 비디오 수 (통계 데이터가 있는)
    pub active_video_count: i64,
    /// 조회 기간 시작
    pub from_date: chrono::NaiveDate,
    /// 조회 기간 종료
    pub to_date: chrono::NaiveDate,
}

/// TOP 비디오 조회 쿼리
#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct TopVideosQuery {
    /// Inclusive start date (YYYY-MM-DD)
    pub from: String,
    /// Inclusive end date (YYYY-MM-DD)
    pub to: String,
    /// 조회할 개수 (기본값: 10, 최대: 50)
    #[serde(default = "default_limit")]
    pub limit: i32,
    /// 정렬 기준 (views | completes, 기본값: views)
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
}

fn default_limit() -> i32 {
    10
}
fn default_sort_by() -> String {
    "views".to_string()
}

/// TOP 비디오 아이템
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TopVideoItem {
    pub rank: i32,
    pub video_id: i64,
    pub video_idx: String,
    pub title: Option<String>,
    pub views: i64,
    pub completes: i64,
}

/// TOP 비디오 응답
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TopVideosRes {
    pub from_date: chrono::NaiveDate,
    pub to_date: chrono::NaiveDate,
    pub sort_by: String,
    pub items: Vec<TopVideoItem>,
}

/// 전체 비디오 일별 집계 응답
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AggregateDailyStatsRes {
    pub from_date: chrono::NaiveDate,
    pub to_date: chrono::NaiveDate,
    pub items: Vec<DailyStatItem>,
}
