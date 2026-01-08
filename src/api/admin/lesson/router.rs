use crate::AppState;
use axum::{routing::get, routing::post, routing::patch, Router};

use super::handler::{
    admin_bulk_create_lesson_items, admin_bulk_create_lessons, admin_bulk_update_lessons,
    admin_create_lesson, admin_create_lesson_item, admin_list_lesson_items,
    admin_list_lessons, admin_update_lesson,
};

pub fn admin_lesson_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_lessons).post(admin_create_lesson))
        .route("/items", get(admin_list_lesson_items))
        .route("/bulk/items", post(admin_bulk_create_lesson_items))
        .route(
            "/bulk",
            post(admin_bulk_create_lessons).patch(admin_bulk_update_lessons),
        )
        .route("/{lesson_id}/items", post(admin_create_lesson_item))
        .route("/{lesson_id}", patch(admin_update_lesson))
}
