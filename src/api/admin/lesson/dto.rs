use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::types::{LessonAccess, LessonState, VideoAccess, VideoState};

/// Insert mode for lesson items
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InsertMode {
    /// Return error if seq already exists (default)
    #[default]
    Error,
    /// Shift existing items down (seq >= target becomes seq + 1)
    Shift,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, IntoParams)]
#[schema(as = AdminLessonListReq)]
pub struct LessonListReq {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<u64>,
    pub q: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub lesson_state: Option<LessonState>,
    pub lesson_access: Option<LessonAccess>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct LessonCreateReq {
    #[validate(length(min = 1))]
    pub lesson_idx: String,
    #[validate(length(min = 1))]
    pub lesson_title: String,
    pub lesson_subtitle: Option<String>,
    pub lesson_description: Option<String>,
    pub lesson_state: Option<LessonState>,
    pub lesson_access: Option<LessonAccess>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonCreateItem {
    #[validate(length(min = 1))]
    pub lesson_idx: String,
    #[validate(length(min = 1))]
    pub lesson_title: String,
    pub lesson_subtitle: Option<String>,
    pub lesson_description: Option<String>,
    pub lesson_state: Option<LessonState>,
    pub lesson_access: Option<LessonAccess>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkCreateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<LessonCreateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkResult {
    pub lesson_id: Option<i32>,
    pub lesson_idx: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkCreateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<LessonBulkResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonUpdateItem {
    #[validate(range(min = 1))]
    pub lesson_id: i32,
    pub lesson_idx: Option<String>,
    pub lesson_title: Option<String>,
    pub lesson_subtitle: Option<String>,
    pub lesson_description: Option<String>,
    pub lesson_state: Option<LessonState>,
    pub lesson_access: Option<LessonAccess>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<LessonUpdateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkUpdateResult {
    pub lesson_id: i32,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonBulkUpdateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<LessonBulkUpdateResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonUpdateReq {
    pub lesson_idx: Option<String>,
    pub lesson_title: Option<String>,
    pub lesson_subtitle: Option<String>,
    pub lesson_description: Option<String>,
    pub lesson_state: Option<LessonState>,
    pub lesson_access: Option<LessonAccess>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, IntoParams)]
pub struct LessonItemListReq {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<u64>,
    pub sort: Option<String>,
    pub order: Option<String>,
    #[validate(range(min = 1))]
    pub lesson_id: Option<i32>,
    pub lesson_item_kind: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, IntoParams)]
pub struct LessonProgressListReq {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<u64>,
    pub sort: Option<String>,
    pub order: Option<String>,
    #[validate(range(min = 1))]
    pub lesson_id: Option<i32>,
    #[validate(range(min = 1))]
    pub user_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, FromRow)]
pub struct AdminLessonProgressRes {
    pub lesson_id: i32,
    pub user_id: i64,
    pub lesson_progress_percent: i32,
    pub lesson_progress_last_item_seq: Option<i32>,
    pub lesson_progress_last_progress_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AdminLessonProgressListRes {
    pub list: Vec<AdminLessonProgressRes>,
    pub total: i64,
    pub page: u64,
    pub size: u64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonProgressUpdateReq {
    #[validate(range(min = 1))]
    pub user_id: i64,
    #[validate(range(min = 0, max = 100))]
    pub lesson_progress_percent: Option<i32>,
    #[validate(range(min = 1))]
    pub lesson_progress_last_item_seq: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonProgressUpdateItem {
    #[validate(range(min = 1))]
    pub lesson_id: i32,
    #[validate(range(min = 1))]
    pub user_id: i64,
    #[validate(range(min = 0, max = 100))]
    pub lesson_progress_percent: Option<i32>,
    #[validate(range(min = 1))]
    pub lesson_progress_last_item_seq: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonProgressBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<LessonProgressUpdateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonProgressBulkUpdateResult {
    pub lesson_id: i32,
    pub user_id: i64,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonProgressBulkUpdateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<LessonProgressBulkUpdateResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemCreateReq {
    #[validate(range(min = 1))]
    pub lesson_item_seq: i32,
    #[validate(length(min = 1))]
    pub lesson_item_kind: String,
    #[validate(range(min = 1))]
    pub video_id: Option<i32>,
    #[validate(range(min = 1))]
    pub study_task_id: Option<i32>,
    /// Insert mode: "error" (default) or "shift"
    #[serde(default)]
    pub insert_mode: InsertMode,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemUpdateReq {
    #[validate(range(min = 1))]
    pub lesson_item_seq: Option<i32>,
    #[validate(length(min = 1))]
    pub lesson_item_kind: Option<String>,
    #[validate(range(min = 1))]
    pub video_id: Option<i32>,
    #[validate(range(min = 1))]
    pub study_task_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemCreateItem {
    #[validate(range(min = 1))]
    pub lesson_id: i32,
    #[validate(range(min = 1))]
    pub lesson_item_seq: i32,
    #[validate(length(min = 1))]
    pub lesson_item_kind: String,
    #[validate(range(min = 1))]
    pub video_id: Option<i32>,
    #[validate(range(min = 1))]
    pub study_task_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkCreateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<LessonItemCreateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkCreateResult {
    pub lesson_id: i32,
    pub lesson_item_seq: i32,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkCreateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<LessonItemBulkCreateResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemUpdateItem {
    #[validate(range(min = 1))]
    pub lesson_id: i32,
    #[validate(range(min = 1))]
    pub current_lesson_item_seq: i32,
    #[validate(range(min = 1))]
    pub new_lesson_item_seq: Option<i32>,
    #[validate(length(min = 1))]
    pub lesson_item_kind: Option<String>,
    #[validate(range(min = 1))]
    pub video_id: Option<i32>,
    #[validate(range(min = 1))]
    pub study_task_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkUpdateReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<LessonItemUpdateItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkUpdateResult {
    pub lesson_id: i32,
    pub lesson_item_seq: i32,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkUpdateRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<LessonItemBulkUpdateResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemDeleteItem {
    #[validate(range(min = 1))]
    pub lesson_id: i32,
    #[validate(range(min = 1))]
    pub lesson_item_seq: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkDeleteReq {
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub items: Vec<LessonItemDeleteItem>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkDeleteResult {
    pub lesson_id: i32,
    pub lesson_item_seq: i32,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Clone)]
pub struct LessonItemBulkDeleteRes {
    pub success_count: i64,
    pub failure_count: i64,
    pub results: Vec<LessonItemBulkDeleteResult>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, FromRow, Clone)]
pub struct AdminLessonItemRes {
    pub lesson_id: i32,
    pub lesson_item_seq: i32,
    pub lesson_item_kind: String,
    pub video_id: Option<i32>,
    pub study_task_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AdminLessonItemListRes {
    pub list: Vec<AdminLessonItemRes>,
    pub total: i64,
    pub page: u64,
    pub size: u64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, FromRow)]
pub struct AdminLessonRes {
    pub lesson_id: i32,
    pub updated_by_user_id: Option<i64>,
    pub lesson_idx: String,
    pub lesson_title: String,
    pub lesson_subtitle: Option<String>,
    pub lesson_description: Option<String>,
    pub lesson_state: LessonState,
    pub lesson_access: LessonAccess,
    pub lesson_created_at: DateTime<Utc>,
    pub lesson_updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct AdminLessonListRes {
    pub list: Vec<AdminLessonRes>,
    pub total: i64,
    pub page: u64,
    pub size: u64,
    pub total_pages: i64,
}

// ============================================
// 7-52: Lesson Item Detail DTOs
// ============================================

/// Video 상세 정보 (lesson item에 포함)
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct LessonItemVideoDetail {
    pub video_id: i32,
    pub video_idx: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_tag_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url_vimeo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_tag_subtitle: Option<String>,
    pub video_views: i64,
    pub video_state: VideoState,
    pub video_access: VideoAccess,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_thumbnail: Option<String>,
    pub video_created_at: DateTime<Utc>,
    pub video_updated_at: DateTime<Utc>,
}

/// Study Task 상세 정보 (lesson item에 포함)
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct LessonItemStudyTaskDetail {
    pub study_task_id: i32,
    pub study_id: i32,
    pub study_task_kind: String,
    pub study_task_seq: i32,
    pub study_task_created_at: DateTime<Utc>,
    pub study_task_updated_at: DateTime<Utc>,
}

/// Lesson Item 상세 응답 (video/study_task 정보 포함)
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct AdminLessonItemDetailRes {
    pub lesson_id: i32,
    pub lesson_item_seq: i32,
    pub lesson_item_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<LessonItemVideoDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub study_task: Option<LessonItemStudyTaskDetail>,
}

/// Lesson Items 상세 목록 응답
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminLessonItemsDetailRes {
    pub lesson_id: i32,
    pub lesson_title: String,
    pub total_items: i64,
    pub items: Vec<AdminLessonItemDetailRes>,
}

// ============================================
// 7-58: Lesson Progress Detail DTOs
// ============================================

/// Lesson Progress 상세 응답 (현재 진행 item 정보 포함)
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct AdminLessonProgressDetailRes {
    pub lesson_id: i32,
    pub user_id: i64,
    pub lesson_progress_percent: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lesson_progress_last_item_seq: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lesson_progress_last_progress_at: Option<DateTime<Utc>>,
    /// 현재 진행 중인 item 정보 (last_item_seq에 해당)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_item: Option<AdminLessonItemRes>,
}

/// Lesson Progress 상세 목록 응답
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AdminLessonProgressListDetailRes {
    pub lesson_id: i32,
    pub lesson_title: String,
    pub total_progress: i64,
    pub list: Vec<AdminLessonProgressDetailRes>,
}
