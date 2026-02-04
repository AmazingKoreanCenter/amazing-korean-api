use axum::Router;

use crate::state::AppState;

use super::email::router::admin_email_router;
use super::lesson::router::admin_lesson_router;
use super::study::router::admin_study_router;
use super::upgrade::router::admin_upgrade_router;
use super::user::router::admin_user_router;
use super::user::stats::router::admin_login_stats_router;
use super::video::router::admin_video_router;

/// Admin 라우터
///
/// # IP Allowlist
/// `ADMIN_IP_ALLOWLIST` 환경변수가 설정되면 해당 IP만 접근 허용
/// 각 핸들러에서 `AdminIpGuard` 추출기를 사용하거나,
/// main.rs에서 admin 라우트에 미들웨어를 적용할 수 있음
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .nest("/users", admin_user_router())
        .nest("/logins/stats", admin_login_stats_router())
        .nest("/lessons", admin_lesson_router())
        .nest("/videos", admin_video_router())
        .nest("/studies", admin_study_router())
        .nest("/email", admin_email_router())
        .nest("/upgrade", admin_upgrade_router())
    // .nest("/courses", admin_course_router()) // 차후 확장
    // .nest("/reports", admin_report_router())
}
