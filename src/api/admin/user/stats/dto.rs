use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// ==========================================
// Request DTOs
// ==========================================

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct UserStatsQuery {
    /// Inclusive start date (YYYY-MM-DD)
    pub from: String,
    /// Inclusive end date (YYYY-MM-DD)
    pub to: String,
}

// ==========================================
// User Stats Response DTOs
// ==========================================

/// 역할별 사용자 수
#[derive(Debug, Clone, Serialize, ToSchema, Default)]
pub struct UsersByRole {
    pub hymn: i64,
    pub admin: i64,
    pub manager: i64,
    pub learner: i64,
}

/// 7-53: 사용자 요약 통계 응답
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UserStatsSummaryRes {
    /// 전체 사용자 수
    pub total_users: i64,
    /// 기간 내 신규 가입자 수
    pub new_users: i64,
    /// 활성 사용자 수
    pub active_users: i64,
    /// 비활성 사용자 수
    pub inactive_users: i64,
    /// 역할별 신규 가입자 수
    pub by_role: UsersByRole,
    /// 조회 기간 시작
    pub from_date: chrono::NaiveDate,
    /// 조회 기간 종료
    pub to_date: chrono::NaiveDate,
}

/// 일별 가입 통계 아이템
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DailySignupItem {
    pub date: chrono::NaiveDate,
    pub signups: i64,
    pub by_role: UsersByRole,
}

/// 7-54: 일별 가입 통계 응답
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UserStatsSignupsRes {
    pub from_date: chrono::NaiveDate,
    pub to_date: chrono::NaiveDate,
    pub items: Vec<DailySignupItem>,
}

// ==========================================
// Login Stats Response DTOs
// ==========================================

/// 7-55: 로그인 요약 통계 응답
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LoginStatsSummaryRes {
    /// 기간 내 총 로그인 시도 수 (성공 + 실패)
    pub total_logins: i64,
    /// 로그인 성공 수
    pub success_count: i64,
    /// 로그인 실패 수
    pub fail_count: i64,
    /// 고유 사용자 수 (로그인 성공한)
    pub unique_users: i64,
    /// 현재 활성 세션 수
    pub active_sessions: i64,
    /// 조회 기간 시작
    pub from_date: chrono::NaiveDate,
    /// 조회 기간 종료
    pub to_date: chrono::NaiveDate,
}

/// 일별 로그인 통계 아이템
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DailyLoginItem {
    pub date: chrono::NaiveDate,
    pub success: i64,
    pub fail: i64,
    pub unique_users: i64,
}

/// 7-56: 일별 로그인 통계 응답
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LoginStatsDailyRes {
    pub from_date: chrono::NaiveDate,
    pub to_date: chrono::NaiveDate,
    pub items: Vec<DailyLoginItem>,
}

/// 디바이스별 통계 아이템
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DeviceStatsItem {
    pub device: String,
    pub count: i64,
    pub percentage: f64,
}

/// 7-57: 디바이스별 통계 응답
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LoginStatsDevicesRes {
    pub from_date: chrono::NaiveDate,
    pub to_date: chrono::NaiveDate,
    pub items: Vec<DeviceStatsItem>,
}
