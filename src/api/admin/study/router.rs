use crate::AppState;
use axum::{routing::get, routing::patch, routing::post, Router};

use super::handler::{
    admin_bulk_create_studies, admin_bulk_update_studies, admin_create_study,
    admin_bulk_create_study_tasks, admin_bulk_update_study_tasks, admin_create_study_task,
    admin_create_task_explain, admin_list_studies, admin_list_study_tasks,
    admin_list_task_explains, admin_update_study, admin_update_study_task,
};

pub fn admin_study_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_studies).post(admin_create_study))
        .route("/bulk", post(admin_bulk_create_studies).patch(admin_bulk_update_studies))
        .route("/tasks", get(admin_list_study_tasks).post(admin_create_study_task))
        .route("/tasks/explain", get(admin_list_task_explains))
        .route("/tasks/{task_id}/explain", post(admin_create_task_explain))
        .route(
            "/tasks/bulk",
            post(admin_bulk_create_study_tasks).patch(admin_bulk_update_study_tasks),
        )
        .route("/tasks/{task_id}", patch(admin_update_study_task))
        .route("/{study_id}", patch(admin_update_study))
}
