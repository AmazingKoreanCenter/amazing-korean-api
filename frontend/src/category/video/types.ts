import { z } from "zod";

// =====================================================================
// Request DTOs
// =====================================================================

export const videoListReqSchema = z.object({
  page: z.number().int().min(1).optional(),
  per_page: z.number().int().min(1).max(100).optional(),
  q: z.string().optional(), // 검색어
  tag: z.string().optional(), // 태그 필터 (단일 스트링)
  lang: z.string().optional(), // 언어 필터
  state: z.string().optional(), // 상태 필터
  sort: z.string().optional(), // 정렬
});

export type VideoListReq = z.infer<typeof videoListReqSchema>;

export const idParamSchema = z.object({
  id: z.number().int(),
});

export type IdParam = z.infer<typeof idParamSchema>;

export const videoProgressUpdateReqSchema = z.object({
  progress_rate: z.number().int().min(0).max(100),
});

export type VideoProgressUpdateReq = z.infer<typeof videoProgressUpdateReqSchema>;

// =====================================================================
// Response DTOs
// =====================================================================

// 1. List Response Items

export const videoListItemSchema = z.object({
  video_id: z.number().int(),
  video_idx: z.string(),
  title: z.string().nullable(), // Option<String> -> null 가능
  subtitle: z.string().nullable(),
  duration_seconds: z.number().int().nullable(),
  language: z.string().nullable(),
  thumbnail_url: z.string().nullable(),
  state: z.string(),
  access: z.string(),
  tags: z.array(z.string()), // 목록에서는 단순 문자열 배열
  has_captions: z.boolean(),
  created_at: z.string().datetime(),
});

export type VideoListItem = z.infer<typeof videoListItemSchema>;

export const videoListMetaSchema = z.object({
  total_count: z.number().int(),
  total_pages: z.number().int(),
  current_page: z.number().int(),
  per_page: z.number().int(),
});

export type VideoListMeta = z.infer<typeof videoListMetaSchema>;

export const videoListResSchema = z.object({
  meta: videoListMetaSchema,
  data: z.array(videoListItemSchema),
});

export type VideoListRes = z.infer<typeof videoListResSchema>;

// 2. Detail Response Items

export const videoTagDetailSchema = z.object({
  key: z.string().nullable(),
  title: z.string().nullable(),
  subtitle: z.string().nullable(),
});

export type VideoTagDetail = z.infer<typeof videoTagDetailSchema>;

export const videoDetailResSchema = z.object({
  video_id: z.number().int(),
  video_url_vimeo: z.string(),
  video_state: z.string(),
  tags: z.array(videoTagDetailSchema), // 상세에서는 객체 배열
  created_at: z.string().datetime(),
});

export type VideoDetailRes = z.infer<typeof videoDetailResSchema>;

// 3. Progress Response

export const videoProgressResSchema = z.object({
  video_id: z.number().int(),
  progress_rate: z.number().int(),
  is_completed: z.boolean(),
  last_watched_at: z.string().datetime().nullable(), // Option -> nullable
});

export type VideoProgressRes = z.infer<typeof videoProgressResSchema>;

// [Phase 3-2] 상세 조회용 인터페이스 (파일 하단에 추가)
export interface VideoTag {
  key: string | null;
  title: string | null;
  subtitle: string | null;
}

export interface VideoDetail {
  video_id: number;
  title: string | null;
  subtitle: string | null;
  video_url_vimeo: string;
  video_state: string;
  tags: VideoTag[];
  created_at: string;
}
