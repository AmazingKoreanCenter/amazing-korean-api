use axum::{routing::{get, post, patch}, Router};

use crate::state::AppState;

use super::handler::{
    admin_auto_translate, admin_auto_translate_bulk, admin_bulk_create_translations,
    admin_create_translation, admin_delete_translation, admin_get_source_fields,
    admin_get_translation, admin_get_translation_stats, admin_list_content_records,
    admin_list_translations, admin_search_translations, admin_update_translation,
    admin_update_translation_status,
};

pub fn admin_translation_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_translations).post(admin_create_translation))
        .route("/bulk", post(admin_bulk_create_translations))
        .route("/auto", post(admin_auto_translate))
        .route("/auto-bulk", post(admin_auto_translate_bulk))
        .route("/content-records", get(admin_list_content_records))
        .route("/source-fields", get(admin_get_source_fields))
        .route("/search", get(admin_search_translations))
        .route("/stats", get(admin_get_translation_stats))
        .route("/{id}", get(admin_get_translation).patch(admin_update_translation).delete(admin_delete_translation))
        .route("/{id}/status", patch(admin_update_translation_status))
}
