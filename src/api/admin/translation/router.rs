use axum::{routing::{get, patch, post}, Router};

use crate::state::AppState;

use super::handler::{
    admin_auto_translate, admin_bulk_create_translations, admin_create_translation,
    admin_delete_translation, admin_get_translation, admin_list_translations,
    admin_update_translation, admin_update_translation_status,
};

pub fn admin_translation_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_translations).post(admin_create_translation))
        .route("/bulk", post(admin_bulk_create_translations))
        .route("/auto", post(admin_auto_translate))
        .route("/{id}", get(admin_get_translation).put(admin_update_translation).delete(admin_delete_translation))
        .route("/{id}/status", patch(admin_update_translation_status))
}
