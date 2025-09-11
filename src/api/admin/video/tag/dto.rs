use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TagsModifyReq {
    pub tags: Vec<String>, // 1..50, 각 태그 1..32
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TagItem {
    pub tag_id: i64,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VideoTagsRes {
    pub video_id: i64,
    pub tags: Vec<TagItem>,
}
