//! OpenAPI 문서 집계
use utoipa::OpenApi;

#[allow(dead_code)]
pub struct ApiDoc;

#[derive(OpenApi)]
#[openapi(
    // NOTE: 여기의 tags/order는 기존 프로젝트 설정을 유지해야 함
    paths(
        // ... (기존 경로들 유지)
        crate::api::admin::video::handler::admin_update_video, // B2
        crate::api::admin::video::handler::admin_delete_video // B3 ← 추가
    ),
    // components/schemas 등 기존 설정 유지
    tags(
        // 기존 태그 순서: health → auth → user → videos → admin
        (name = "admin", description = "Admin endpoints")
    )
)]
pub struct Docs;
