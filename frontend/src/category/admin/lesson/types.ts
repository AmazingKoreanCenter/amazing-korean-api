import { z } from "zod";

// Lesson State enum
export const lessonStateSchema = z.enum(["ready", "open", "close"]);
export type LessonState = z.infer<typeof lessonStateSchema>;

// Lesson Access enum
export const lessonAccessSchema = z.enum(["public", "paid", "private", "promote"]);
export type LessonAccess = z.infer<typeof lessonAccessSchema>;

export const lessonListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  size: z.number().int().min(1).max(100).optional(),
  q: z.string().optional(),
  sort: z.string().optional(),
  order: z.string().optional(),
  lesson_state: lessonStateSchema.optional(),
  lesson_access: lessonAccessSchema.optional(),
});

export type LessonListReq = z.infer<typeof lessonListReqSchema>;

export const lessonCreateReqSchema = z.object({
  lesson_idx: z.string().min(1),
  lesson_title: z.string().min(1),
  lesson_subtitle: z.string().optional(),
  lesson_description: z.string().optional(),
  lesson_state: lessonStateSchema.optional(),
  lesson_access: lessonAccessSchema.optional(),
});

export type LessonCreateReq = z.infer<typeof lessonCreateReqSchema>;

export const lessonCreateItemSchema = z.object({
  lesson_idx: z.string().min(1),
  lesson_title: z.string().min(1),
  lesson_subtitle: z.string().optional(),
  lesson_description: z.string().optional(),
  lesson_state: lessonStateSchema.optional(),
  lesson_access: lessonAccessSchema.optional(),
});

export type LessonCreateItem = z.infer<typeof lessonCreateItemSchema>;

export const lessonBulkCreateReqSchema = z.object({
  items: z.array(lessonCreateItemSchema).min(1).max(100),
});

export type LessonBulkCreateReq = z.infer<typeof lessonBulkCreateReqSchema>;

export const lessonBulkResultSchema = z.object({
  lesson_id: z.number().int().optional(),
  lesson_idx: z.string(),
  success: z.boolean(),
  error: z.string().optional(),
});

export type LessonBulkResult = z.infer<typeof lessonBulkResultSchema>;

export const lessonBulkCreateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(lessonBulkResultSchema),
});

export type LessonBulkCreateRes = z.infer<typeof lessonBulkCreateResSchema>;

export const lessonUpdateItemSchema = z.object({
  lesson_id: z.number().int().min(1),
  lesson_idx: z.string().optional(),
  lesson_title: z.string().optional(),
  lesson_subtitle: z.string().optional(),
  lesson_description: z.string().optional(),
  lesson_state: lessonStateSchema.optional(),
  lesson_access: lessonAccessSchema.optional(),
});

export type LessonUpdateItem = z.infer<typeof lessonUpdateItemSchema>;

export const lessonBulkUpdateReqSchema = z.object({
  items: z.array(lessonUpdateItemSchema).min(1).max(100),
});

export type LessonBulkUpdateReq = z.infer<typeof lessonBulkUpdateReqSchema>;

export const lessonBulkUpdateResultSchema = z.object({
  lesson_id: z.number().int(),
  success: z.boolean(),
  error: z.string().optional(),
});

export type LessonBulkUpdateResult = z.infer<typeof lessonBulkUpdateResultSchema>;

export const lessonBulkUpdateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(lessonBulkUpdateResultSchema),
});

export type LessonBulkUpdateRes = z.infer<typeof lessonBulkUpdateResSchema>;

export const lessonUpdateReqSchema = z.object({
  lesson_idx: z.string().optional(),
  lesson_title: z.string().optional(),
  lesson_subtitle: z.string().optional(),
  lesson_description: z.string().optional(),
  lesson_state: lessonStateSchema.optional(),
  lesson_access: lessonAccessSchema.optional(),
});

export type LessonUpdateReq = z.infer<typeof lessonUpdateReqSchema>;

export const lessonItemListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  size: z.number().int().min(1).max(100).optional(),
  sort: z.string().optional(),
  order: z.string().optional(),
  lesson_id: z.number().int().min(1).optional(),
  lesson_item_kind: z.string().optional(),
});

export type LessonItemListReq = z.infer<typeof lessonItemListReqSchema>;

export const lessonProgressListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  size: z.number().int().min(1).max(100).optional(),
  sort: z.string().optional(),
  order: z.string().optional(),
  lesson_id: z.number().int().min(1).optional(),
  user_id: z.number().int().min(1).optional(),
});

export type LessonProgressListReq = z.infer<typeof lessonProgressListReqSchema>;

export const adminLessonProgressResSchema = z.object({
  lesson_id: z.number().int(),
  user_id: z.number().int(),
  lesson_progress_percent: z.number().int(),
  lesson_progress_last_item_seq: z.number().int().optional(),
  lesson_progress_last_progress_at: z.string().datetime().optional(),
});

export type AdminLessonProgressRes = z.infer<typeof adminLessonProgressResSchema>;

export const adminLessonProgressListResSchema = z.object({
  list: z.array(adminLessonProgressResSchema),
  total: z.number().int(),
  page: z.number().int(),
  size: z.number().int(),
  total_pages: z.number().int(),
});

export type AdminLessonProgressListRes = z.infer<typeof adminLessonProgressListResSchema>;

export const lessonProgressUpdateReqSchema = z.object({
  user_id: z.number().int().min(1),
  lesson_progress_percent: z.number().int().min(0).max(100).optional(),
  lesson_progress_last_item_seq: z.number().int().min(1).optional(),
});

export type LessonProgressUpdateReq = z.infer<typeof lessonProgressUpdateReqSchema>;

export const lessonProgressUpdateItemSchema = z.object({
  lesson_id: z.number().int().min(1),
  user_id: z.number().int().min(1),
  lesson_progress_percent: z.number().int().min(0).max(100).optional(),
  lesson_progress_last_item_seq: z.number().int().min(1).optional(),
});

export type LessonProgressUpdateItem = z.infer<typeof lessonProgressUpdateItemSchema>;

export const lessonProgressBulkUpdateReqSchema = z.object({
  items: z.array(lessonProgressUpdateItemSchema).min(1).max(100),
});

export type LessonProgressBulkUpdateReq = z.infer<typeof lessonProgressBulkUpdateReqSchema>;

export const lessonProgressBulkUpdateResultSchema = z.object({
  lesson_id: z.number().int(),
  user_id: z.number().int(),
  success: z.boolean(),
  error: z.string().optional(),
});

export type LessonProgressBulkUpdateResult = z.infer<typeof lessonProgressBulkUpdateResultSchema>;

export const lessonProgressBulkUpdateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(lessonProgressBulkUpdateResultSchema),
});

export type LessonProgressBulkUpdateRes = z.infer<typeof lessonProgressBulkUpdateResSchema>;

// Insert mode for lesson items
export const insertModeSchema = z.enum(["error", "shift"]);
export type InsertMode = z.infer<typeof insertModeSchema>;

export const lessonItemCreateReqSchema = z.object({
  lesson_item_seq: z.number().int().min(1),
  lesson_item_kind: z.string().min(1),
  video_id: z.number().int().min(1).optional(),
  study_task_id: z.number().int().min(1).optional(),
  insert_mode: insertModeSchema.optional(), // "error" (default) or "shift"
});

export type LessonItemCreateReq = z.infer<typeof lessonItemCreateReqSchema>;

export const lessonItemUpdateReqSchema = z.object({
  lesson_item_seq: z.number().int().min(1).optional(),
  lesson_item_kind: z.string().min(1).optional(),
  video_id: z.number().int().min(1).optional(),
  study_task_id: z.number().int().min(1).optional(),
});

export type LessonItemUpdateReq = z.infer<typeof lessonItemUpdateReqSchema>;

export const lessonItemCreateItemSchema = z.object({
  lesson_id: z.number().int().min(1),
  lesson_item_seq: z.number().int().min(1),
  lesson_item_kind: z.string().min(1),
  video_id: z.number().int().min(1).optional(),
  study_task_id: z.number().int().min(1).optional(),
});

export type LessonItemCreateItem = z.infer<typeof lessonItemCreateItemSchema>;

export const lessonItemBulkCreateReqSchema = z.object({
  items: z.array(lessonItemCreateItemSchema).min(1).max(100),
});

export type LessonItemBulkCreateReq = z.infer<typeof lessonItemBulkCreateReqSchema>;

export const lessonItemBulkCreateResultSchema = z.object({
  lesson_id: z.number().int(),
  lesson_item_seq: z.number().int(),
  success: z.boolean(),
  error: z.string().optional(),
});

export type LessonItemBulkCreateResult = z.infer<typeof lessonItemBulkCreateResultSchema>;

export const lessonItemBulkCreateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(lessonItemBulkCreateResultSchema),
});

export type LessonItemBulkCreateRes = z.infer<typeof lessonItemBulkCreateResSchema>;

export const lessonItemUpdateItemSchema = z.object({
  lesson_id: z.number().int().min(1),
  current_lesson_item_seq: z.number().int().min(1),
  new_lesson_item_seq: z.number().int().min(1).optional(),
  lesson_item_kind: z.string().min(1).optional(),
  video_id: z.number().int().min(1).optional(),
  study_task_id: z.number().int().min(1).optional(),
});

export type LessonItemUpdateItem = z.infer<typeof lessonItemUpdateItemSchema>;

export const lessonItemBulkUpdateReqSchema = z.object({
  items: z.array(lessonItemUpdateItemSchema).min(1).max(100),
});

export type LessonItemBulkUpdateReq = z.infer<typeof lessonItemBulkUpdateReqSchema>;

export const lessonItemBulkUpdateResultSchema = z.object({
  lesson_id: z.number().int(),
  lesson_item_seq: z.number().int(),
  success: z.boolean(),
  error: z.string().optional(),
});

export type LessonItemBulkUpdateResult = z.infer<typeof lessonItemBulkUpdateResultSchema>;

export const lessonItemBulkUpdateResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(lessonItemBulkUpdateResultSchema),
});

export type LessonItemBulkUpdateRes = z.infer<typeof lessonItemBulkUpdateResSchema>;

export const lessonItemDeleteItemSchema = z.object({
  lesson_id: z.number().int().min(1),
  lesson_item_seq: z.number().int().min(1),
});

export type LessonItemDeleteItem = z.infer<typeof lessonItemDeleteItemSchema>;

export const lessonItemBulkDeleteReqSchema = z.object({
  items: z.array(lessonItemDeleteItemSchema).min(1).max(100),
});

export type LessonItemBulkDeleteReq = z.infer<typeof lessonItemBulkDeleteReqSchema>;

export const lessonItemBulkDeleteResultSchema = z.object({
  lesson_id: z.number().int(),
  lesson_item_seq: z.number().int(),
  success: z.boolean(),
  error: z.string().optional(),
});

export type LessonItemBulkDeleteResult = z.infer<typeof lessonItemBulkDeleteResultSchema>;

export const lessonItemBulkDeleteResSchema = z.object({
  success_count: z.number().int(),
  failure_count: z.number().int(),
  results: z.array(lessonItemBulkDeleteResultSchema),
});

export type LessonItemBulkDeleteRes = z.infer<typeof lessonItemBulkDeleteResSchema>;

export const adminLessonItemResSchema = z.object({
  lesson_id: z.number().int(),
  lesson_item_seq: z.number().int(),
  lesson_item_kind: z.string(),
  video_id: z.number().int().optional(),
  study_task_id: z.number().int().optional(),
});

export type AdminLessonItemRes = z.infer<typeof adminLessonItemResSchema>;

export const adminLessonItemListResSchema = z.object({
  list: z.array(adminLessonItemResSchema),
  total: z.number().int(),
  page: z.number().int(),
  size: z.number().int(),
  total_pages: z.number().int(),
});

export type AdminLessonItemListRes = z.infer<typeof adminLessonItemListResSchema>;

export const adminLessonResSchema = z.object({
  lesson_id: z.number().int(),
  updated_by_user_id: z.number().int(),
  lesson_idx: z.string(),
  lesson_title: z.string(),
  lesson_subtitle: z.string().optional().nullable(),
  lesson_description: z.string().optional().nullable(),
  lesson_state: lessonStateSchema,
  lesson_access: lessonAccessSchema,
  lesson_created_at: z.string().datetime(),
  lesson_updated_at: z.string().datetime(),
});

export type AdminLessonRes = z.infer<typeof adminLessonResSchema>;

export const adminLessonListResSchema = z.object({
  list: z.array(adminLessonResSchema),
  total: z.number().int(),
  page: z.number().int(),
  size: z.number().int(),
  total_pages: z.number().int(),
});

export type AdminLessonListRes = z.infer<typeof adminLessonListResSchema>;

// ============================================
// Detail Response DTOs (7-52, 7-58)
// ============================================

// Video detail for lesson item
export const lessonItemVideoDetailSchema = z.object({
  video_id: z.number().int(),
  video_idx: z.string(),
  video_tag_title: z.string().optional().nullable(),
  video_url_vimeo: z.string().optional().nullable(),
  video_tag_subtitle: z.string().optional().nullable(),
  video_views: z.number().int(),
  video_state: z.enum(["ready", "open", "close"]),
  video_access: z.enum(["public", "paid", "private", "promote"]),
  video_duration: z.number().int().optional().nullable(),
  video_thumbnail: z.string().optional().nullable(),
  video_created_at: z.string().datetime(),
  video_updated_at: z.string().datetime(),
});

export type LessonItemVideoDetail = z.infer<typeof lessonItemVideoDetailSchema>;

// Study task detail for lesson item
export const lessonItemStudyTaskDetailSchema = z.object({
  study_task_id: z.number().int(),
  study_id: z.number().int(),
  study_task_kind: z.string(),
  study_task_seq: z.number().int(),
  study_task_created_at: z.string().datetime(),
  study_task_updated_at: z.string().datetime(),
});

export type LessonItemStudyTaskDetail = z.infer<typeof lessonItemStudyTaskDetailSchema>;

// Lesson item detail response (with video/task info)
export const adminLessonItemDetailResSchema = z.object({
  lesson_id: z.number().int(),
  lesson_item_seq: z.number().int(),
  lesson_item_kind: z.string(),
  video: lessonItemVideoDetailSchema.optional().nullable(),
  study_task: lessonItemStudyTaskDetailSchema.optional().nullable(),
});

export type AdminLessonItemDetailRes = z.infer<typeof adminLessonItemDetailResSchema>;

// Lesson items detail list response (7-52)
export const adminLessonItemsDetailResSchema = z.object({
  lesson_id: z.number().int(),
  lesson_title: z.string(),
  total_items: z.number().int(),
  items: z.array(adminLessonItemDetailResSchema),
});

export type AdminLessonItemsDetailRes = z.infer<typeof adminLessonItemsDetailResSchema>;

// Lesson progress detail response (with current item info)
export const adminLessonProgressDetailResSchema = z.object({
  lesson_id: z.number().int(),
  user_id: z.number().int(),
  lesson_progress_percent: z.number().int(),
  lesson_progress_last_item_seq: z.number().int().optional().nullable(),
  lesson_progress_last_progress_at: z.string().datetime().optional().nullable(),
  current_item: adminLessonItemResSchema.optional().nullable(),
});

export type AdminLessonProgressDetailRes = z.infer<typeof adminLessonProgressDetailResSchema>;

// Lesson progress detail list response (7-58)
export const adminLessonProgressListDetailResSchema = z.object({
  lesson_id: z.number().int(),
  lesson_title: z.string(),
  total_progress: z.number().int(),
  list: z.array(adminLessonProgressDetailResSchema),
});

export type AdminLessonProgressListDetailRes = z.infer<typeof adminLessonProgressListDetailResSchema>;
