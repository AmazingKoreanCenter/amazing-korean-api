use crate::state::AppState;
use axum::Router;

use super::user::router::admin_user_router;
use crate::api::admin::lesson::router::admin_lesson_router;
use crate::api::admin::study::router::admin_study_router;
use crate::api::admin::video::router::admin_video_router;

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .nest("/users", admin_user_router())
        .nest("/lessons", admin_lesson_router())
        .nest("/videos", admin_video_router())
        .nest("/studies", admin_study_router())
    // .nest("/courses", admin_course_router()) // 차후 확장
    // .nest("/reports", admin_report_router())
}
