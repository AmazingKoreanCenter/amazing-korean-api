import { z } from "zod";

export const videoListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  per_page: z.number().int().min(1).max(100).optional(),
  sort: z.string().optional(),
});

export type VideoListReq = z.infer<typeof videoListReqSchema>;

export const videoInfoSchema = z.object({
  video_id: z.number().int(),
  video_url_vimeo: z.string(),
  video_state: z.string(),
  video_access: z.string(),
  tags: z.array(z.string()),
  created_at: z.string().datetime(),
});

export type VideoInfo = z.infer<typeof videoInfoSchema>;

export const videoListMetaSchema = z.object({
  total_count: z.number().int(),
  total_pages: z.number().int(),
  current_page: z.number().int(),
  per_page: z.number().int(),
});

export type VideoListMeta = z.infer<typeof videoListMetaSchema>;

export const videoListResSchema = z.object({
  meta: videoListMetaSchema,
  data: z.array(videoInfoSchema),
});

export type VideoListRes = z.infer<typeof videoListResSchema>;

export const videosQuerySchema = z.object({
  q: z.string().optional(),
  tag: z.array(z.string()).optional(),
  lang: z.string().optional(),
  access: z.string().optional(),
  state: z.string().optional(),
  limit: z.number().int().default(20),
  offset: z.number().int().default(0),
  sort: z.string().optional(),
  order: z.string().optional(),
});

export type VideosQuery = z.infer<typeof videosQuerySchema>;

export const videoListItemSchema = z.object({
  video_id: z.number().int(),
  video_idx: z.string(),
  title: z.string().optional(),
  subtitle: z.string().optional(),
  duration_seconds: z.number().int().optional(),
  language: z.string().optional(),
  thumbnail_url: z.string().optional(),
  state: z.string(),
  access: z.string(),
  tags: z.array(z.string()),
  has_captions: z.boolean(),
  created_at: z.string().datetime(),
});

export type VideoListItem = z.infer<typeof videoListItemSchema>;

export const idParamSchema = z.object({
  id: z.number().int(),
});

export type IdParam = z.infer<typeof idParamSchema>;

export const videoTagDetailSchema = z.object({
  key: z.string().optional(),
  title: z.string().optional(),
  subtitle: z.string().optional(),
});

export type VideoTagDetail = z.infer<typeof videoTagDetailSchema>;

export const videoDetailResSchema = z.object({
  video_id: z.number().int(),
  video_url_vimeo: z.string(),
  video_state: z.string(),
  tags: z.array(videoTagDetailSchema),
  created_at: z.string().datetime(),
});

export type VideoDetailRes = z.infer<typeof videoDetailResSchema>;

export const videoProgressResSchema = z.object({
  video_id: z.number().int(),
  progress_rate: z.number().int(),
  is_completed: z.boolean(),
  last_watched_at: z.string().datetime().optional(),
});

export type VideoProgressRes = z.infer<typeof videoProgressResSchema>;

export const videoProgressUpdateReqSchema = z.object({
  progress_rate: z.number().int().min(0).max(100),
});

export type VideoProgressUpdateReq = z.infer<typeof videoProgressUpdateReqSchema>;
