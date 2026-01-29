use crate::AppState;
use axum::{routing::get, routing::patch, routing::post, Router};

use super::handler::{
    admin_bulk_create_studies, admin_bulk_update_studies, admin_create_study,
    admin_bulk_create_study_tasks, admin_bulk_create_task_explains,
    admin_bulk_update_study_tasks, admin_bulk_update_task_explains,
    admin_create_study_task, admin_create_task_explain, admin_get_study, admin_get_study_task,
    admin_list_studies, admin_list_study_tasks, admin_list_task_explains, admin_list_task_status,
    admin_update_study, admin_update_study_task, admin_update_task_explain,
    admin_update_task_status, admin_bulk_update_task_status,
};
use super::stats::router::admin_study_stats_router;

pub fn admin_study_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_studies).post(admin_create_study))
        .route("/bulk", post(admin_bulk_create_studies).patch(admin_bulk_update_studies))
        // Study 통계 대시보드
        .nest("/stats", admin_study_stats_router())
        .route("/tasks", get(admin_list_study_tasks).post(admin_create_study_task))
        .route("/tasks/explain", get(admin_list_task_explains))
        .route("/tasks/status", get(admin_list_task_status))
        .route("/tasks/bulk/status", patch(admin_bulk_update_task_status))
        .route("/tasks/{task_id}/status", patch(admin_update_task_status))
        .route(
            "/tasks/bulk/explain",
            post(admin_bulk_create_task_explains).patch(admin_bulk_update_task_explains),
        )
        .route(
            "/tasks/{task_id}/explain",
            post(admin_create_task_explain).patch(admin_update_task_explain),
        )
        .route(
            "/tasks/bulk",
            post(admin_bulk_create_study_tasks).patch(admin_bulk_update_study_tasks),
        )
        .route("/tasks/{task_id}", get(admin_get_study_task).patch(admin_update_study_task))
        .route("/{study_id}", get(admin_get_study).patch(admin_update_study))
}
