use crate::state::AppState;
use axum::Router;

use super::user::router::admin_user_router;

pub fn admin_router() -> Router<AppState> {
    Router::new().nest("/users", admin_user_router())
    // .nest("/courses", admin_course_router()) // 차후 확장
    // .nest("/reports", admin_report_router())
}
