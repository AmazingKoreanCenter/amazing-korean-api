use crate::AppState;
use axum::{routing::get, routing::patch, routing::post, Router};

use super::handler::{
    admin_bulk_create_lesson_items, admin_bulk_delete_lesson_items, admin_bulk_create_lessons,
    admin_bulk_update_lesson_items, admin_bulk_update_lesson_progress, admin_bulk_update_lessons,
    admin_create_lesson, admin_create_lesson_item, admin_delete_lesson_item,
    admin_get_lesson_detail, admin_get_lesson_items_detail, admin_get_lesson_progress_detail,
    admin_list_lesson_items, admin_list_lesson_progress, admin_list_lessons, admin_update_lesson,
    admin_update_lesson_item, admin_update_lesson_progress,
};

pub fn admin_lesson_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_lessons).post(admin_create_lesson))
        .route("/items", get(admin_list_lesson_items))
        .route("/items/{lesson_id}", get(admin_get_lesson_items_detail))
        .route("/progress", get(admin_list_lesson_progress))
        .route("/progress/{lesson_id}", get(admin_get_lesson_progress_detail))
        .route(
            "/bulk/items",
            post(admin_bulk_create_lesson_items)
                .patch(admin_bulk_update_lesson_items)
                .delete(admin_bulk_delete_lesson_items),
        )
        .route("/bulk/progress", patch(admin_bulk_update_lesson_progress))
        .route(
            "/bulk",
            post(admin_bulk_create_lessons).patch(admin_bulk_update_lessons),
        )
        .route("/{lesson_id}/items", post(admin_create_lesson_item))
        .route(
            "/{lesson_id}/items/{seq}",
            patch(admin_update_lesson_item).delete(admin_delete_lesson_item),
        )
        .route("/{lesson_id}/progress", patch(admin_update_lesson_progress))
        .route("/{lesson_id}", get(admin_get_lesson_detail).patch(admin_update_lesson))
}
