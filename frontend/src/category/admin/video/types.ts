import { z } from "zod";

export const videoStateSchema = z.enum(["draft", "ready", "hidden"]);

export type VideoState = z.infer<typeof videoStateSchema>;

export const videoAccessSchema = z.enum(["free", "paid", "private"]);

export type VideoAccess = z.infer<typeof videoAccessSchema>;

export const videoAccessInputSchema = z.enum(["public", "private"]);

export type VideoAccessInput = z.infer<typeof videoAccessInputSchema>;

export const videoCreateReqSchema = z.object({
  video_tag_title: z.string().min(1).max(200),
  video_tag_subtitle: z.string().max(500).optional(),
  video_tag_key: z.string().min(1).max(30).optional(),
  video_url_vimeo: z.string().url().max(1024),
  video_access: videoAccessInputSchema,
  video_idx: z.string().min(1).max(100).optional(),
});

export type VideoCreateReq = z.infer<typeof videoCreateReqSchema>;

export const videoUpdateReqSchema = z.object({
  video_tag_title: z.string().min(1).max(200).optional(),
  video_tag_subtitle: z.string().max(500).optional(),
  video_tag_key: z.string().min(1).max(30).optional(),
  video_url_vimeo: z.string().url().max(1024).optional(),
  video_access: videoAccessInputSchema.optional(),
  video_idx: z.string().min(1).max(100).optional(),
});

export type VideoUpdateReq = z.infer<typeof videoUpdateReqSchema>;

export const videoTagUpdateReqSchema = z.object({
  video_tag_title: z.string().min(1).max(200).optional(),
  video_tag_subtitle: z.string().max(500).optional(),
  video_tag_key: z.string().min(1).max(30).optional(),
});

export type VideoTagUpdateReq = z.infer<typeof videoTagUpdateReqSchema>;

export const videoTagBulkUpdateItemSchema = z.object({
  id: z.number().int(),
  video_tag_title: z.string().min(1).max(200).optional(),
  video_tag_subtitle: z.string().max(500).optional(),
  video_tag_key: z.string().min(1).max(30).optional(),
});

export type VideoTagBulkUpdateItem = z.infer<typeof videoTagBulkUpdateItemSchema>;

export const videoTagBulkUpdateReqSchema = z.object({
  items: z.array(videoTagBulkUpdateItemSchema).min(1).max(100),
});

export type VideoTagBulkUpdateReq = z.infer<typeof videoTagBulkUpdateReqSchema>;

export const videoResSchema = z.object({
  video_id: z.number().int(),
  video_title: z.string(),
  video_subtitle: z.string().optional(),
  video_language: z.string().optional(),
  video_state: videoStateSchema,
  video_access: videoAccessSchema,
  vimeo_video_id: z.string().optional(),
  video_duration_seconds: z.number().int().optional(),
  video_thumbnail_url: z.string().optional(),
  video_link: z.string().optional(),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
  updated_by_user_id: z.number().int(),
  deleted_at: z.string().datetime().optional(),
});

export type VideoRes = z.infer<typeof videoResSchema>;

export const adminVideoListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  size: z.number().int().min(1).max(100).optional(),
  q: z.string().optional(),
  sort: z.string().optional(),
  order: z.string().optional(),
});

export type AdminVideoListReq = z.infer<typeof adminVideoListReqSchema>;

export const adminVideoResSchema = z.object({
  id: z.number().int(),
  title: z.string(),
  url: z.string().optional(),
  description: z.string().optional(),
  views: z.number().int(),
  is_public: z.boolean(),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
});

export type AdminVideoRes = z.infer<typeof adminVideoResSchema>;

export const paginationSchema = z.object({
  total_count: z.number().int(),
  total_pages: z.number().int(),
  current_page: z.number().int(),
  per_page: z.number().int(),
});

export type Pagination = z.infer<typeof paginationSchema>;

export const adminVideoListResSchema = z.object({
  items: z.array(adminVideoResSchema),
  pagination: paginationSchema,
});

export type AdminVideoListRes = z.infer<typeof adminVideoListResSchema>;

export const videoBulkCreateReqSchema = z.object({
  items: z.array(videoCreateReqSchema).min(1).max(100),
});

export type VideoBulkCreateReq = z.infer<typeof videoBulkCreateReqSchema>;

export const videoBulkItemErrorSchema = z.object({
  code: z.string(),
  message: z.string(),
});

export type VideoBulkItemError = z.infer<typeof videoBulkItemErrorSchema>;

export const videoBulkItemResultSchema = z.object({
  id: z.number().int().optional(),
  status: z.number().int(),
  data: adminVideoResSchema.optional(),
  error: videoBulkItemErrorSchema.optional(),
});

export type VideoBulkItemResult = z.infer<typeof videoBulkItemResultSchema>;

export const videoBulkSummarySchema = z.object({
  total: z.number().int(),
  success: z.number().int(),
  failure: z.number().int(),
});

export type VideoBulkSummary = z.infer<typeof videoBulkSummarySchema>;

export const videoBulkCreateResSchema = z.object({
  summary: videoBulkSummarySchema,
  results: z.array(videoBulkItemResultSchema),
});

export type VideoBulkCreateRes = z.infer<typeof videoBulkCreateResSchema>;

export const videoBulkUpdateItemSchema = z.object({
  id: z.number().int(),
  video_tag_title: z.string().min(1).max(200).optional(),
  video_tag_subtitle: z.string().max(500).optional(),
  video_tag_key: z.string().min(1).max(30).optional(),
  video_url_vimeo: z.string().url().max(1024).optional(),
  video_access: videoAccessInputSchema.optional(),
  video_idx: z.string().min(1).max(100).optional(),
});

export type VideoBulkUpdateItem = z.infer<typeof videoBulkUpdateItemSchema>;

export const videoBulkUpdateReqSchema = z.object({
  items: z.array(videoBulkUpdateItemSchema).min(1).max(100),
});

export type VideoBulkUpdateReq = z.infer<typeof videoBulkUpdateReqSchema>;

export const videoBulkUpdateItemResultSchema = z.object({
  id: z.number().int(),
  status: z.number().int(),
  data: adminVideoResSchema.optional(),
  error: videoBulkItemErrorSchema.optional(),
});

export type VideoBulkUpdateItemResult = z.infer<typeof videoBulkUpdateItemResultSchema>;

export const videoBulkUpdateResSchema = z.object({
  summary: videoBulkSummarySchema,
  results: z.array(videoBulkUpdateItemResultSchema),
});

export type VideoBulkUpdateRes = z.infer<typeof videoBulkUpdateResSchema>;
