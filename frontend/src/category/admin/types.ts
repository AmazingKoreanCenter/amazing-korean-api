import { z } from "zod";
import { userAuthSchema, userGenderSchema } from "@/category/auth/types";

// ==========================================
// 1. 공통 타입 (Admin 전역)
// ==========================================

// 공통 목록 요청 파라미터
export const adminListReqSchema = z.object({
  page: z.number().int().min(1).default(1),
  size: z.number().int().min(1).max(100).default(20),
  q: z.string().optional(),
  sort: z.string().optional(),
  order: z.enum(["asc", "desc"]).optional(),
});
export type AdminListReq = z.infer<typeof adminListReqSchema>;

// 공통 페이지네이션 메타
export const adminListMetaSchema = z.object({
  total_count: z.number().int(),
  total_pages: z.number().int(),
  current_page: z.number().int(),
  per_page: z.number().int(),
});
export type AdminListMeta = z.infer<typeof adminListMetaSchema>;

// 벌크 작업 결과 요약
export const bulkSummarySchema = z.object({
  total: z.number().int(),
  success: z.number().int(),
  failure: z.number().int(),
});
export type BulkSummary = z.infer<typeof bulkSummarySchema>;

// 벌크 항목 에러
export const bulkItemErrorSchema = z.object({
  code: z.string(),
  message: z.string(),
});
export type BulkItemError = z.infer<typeof bulkItemErrorSchema>;

// ==========================================
// 2. Admin User 타입
// ==========================================

// 사용자 목록 아이템 (요약)
export const adminUserSummarySchema = z.object({
  id: z.number().int(),
  email: z.string(),
  nickname: z.string().nullable(),
  role: userAuthSchema,
  created_at: z.string().datetime(),
});
export type AdminUserSummary = z.infer<typeof adminUserSummarySchema>;

// 사용자 목록 응답
export const adminUserListResSchema = z.object({
  items: z.array(adminUserSummarySchema),
  meta: adminListMetaSchema,
});
export type AdminUserListRes = z.infer<typeof adminUserListResSchema>;

// 사용자 상세 응답
export const adminUserResSchema = z.object({
  id: z.number().int(),
  email: z.string(),
  name: z.string(),
  nickname: z.string().nullable(),
  language: z.string().nullable(),
  country: z.string().nullable(),
  birthday: z.string().nullable(), // YYYY-MM-DD
  gender: userGenderSchema,
  user_state: z.boolean(),
  user_auth: userAuthSchema,
  created_at: z.string().datetime(),
  quit_at: z.string().datetime().nullable(),
});
export type AdminUserRes = z.infer<typeof adminUserResSchema>;

// 사용자 생성 요청
export const adminCreateUserReqSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
  name: z.string().min(1).max(100),
  nickname: z.string().min(1).max(100),
  language: z.string().min(1).max(50).optional(),
  country: z.string().min(1).max(50).optional(),
  birthday: z.string().optional(), // YYYY-MM-DD
  gender: userGenderSchema.optional(),
  user_auth: z.string().optional(), // 백엔드에서 변환
});
export type AdminCreateUserReq = z.infer<typeof adminCreateUserReqSchema>;

// 사용자 수정 요청
export const adminUpdateUserReqSchema = z.object({
  email: z.string().email().optional(),
  // password: 빈 문자열 허용, 값이 있으면 8자 이상
  password: z
    .string()
    .optional()
    .refine((val) => !val || val.length >= 8, {
      message: "Password must be at least 8 characters",
    }),
  name: z.string().min(1).max(50).optional(),
  nickname: z.string().min(1).max(100).optional(),
  language: z.string().min(1).max(50).optional(),
  country: z.string().min(1).max(50).optional(),
  birthday: z.string().optional(), // YYYY-MM-DD
  gender: userGenderSchema.optional(),
  user_state: z.boolean().optional(),
  user_auth: userAuthSchema.optional(),
});
export type AdminUpdateUserReq = z.infer<typeof adminUpdateUserReqSchema>;

// 벌크 생성 요청
export const adminBulkCreateUserReqSchema = z.object({
  items: z.array(adminCreateUserReqSchema).min(1).max(100),
});
export type AdminBulkCreateUserReq = z.infer<typeof adminBulkCreateUserReqSchema>;

// 벌크 생성 결과 아이템
export const bulkCreateItemResultSchema = z.object({
  email: z.string(),
  status: z.number().int(),
  data: adminUserResSchema.optional(),
  error: bulkItemErrorSchema.optional(),
});
export type BulkCreateItemResult = z.infer<typeof bulkCreateItemResultSchema>;

// 벌크 생성 응답
export const adminBulkCreateUserResSchema = z.object({
  summary: bulkSummarySchema,
  results: z.array(bulkCreateItemResultSchema),
});
export type AdminBulkCreateUserRes = z.infer<typeof adminBulkCreateUserResSchema>;

// 벌크 수정 요청 아이템
export const adminBulkUpdateUserItemSchema = z.object({
  id: z.number().int(),
  email: z.string().email().optional(),
  password: z.string().min(8).optional(),
  name: z.string().min(1).max(50).optional(),
  nickname: z.string().min(1).max(100).optional(),
  language: z.string().min(1).max(50).optional(),
  country: z.string().min(1).max(50).optional(),
  birthday: z.string().optional(),
  gender: userGenderSchema.optional(),
  user_state: z.boolean().optional(),
  user_auth: userAuthSchema.optional(),
});
export type AdminBulkUpdateUserItem = z.infer<typeof adminBulkUpdateUserItemSchema>;

// 벌크 수정 요청
export const adminBulkUpdateUserReqSchema = z.object({
  items: z.array(adminBulkUpdateUserItemSchema).min(1).max(100),
});
export type AdminBulkUpdateUserReq = z.infer<typeof adminBulkUpdateUserReqSchema>;

// 벌크 수정 결과 아이템
export const bulkUpdateItemResultSchema = z.object({
  id: z.number().int(),
  status: z.number().int(),
  data: adminUserResSchema.optional(),
  error: bulkItemErrorSchema.optional(),
});
export type BulkUpdateItemResult = z.infer<typeof bulkUpdateItemResultSchema>;

// 벌크 수정 응답
export const adminBulkUpdateUserResSchema = z.object({
  summary: bulkSummarySchema,
  results: z.array(bulkUpdateItemResultSchema),
});
export type AdminBulkUpdateUserRes = z.infer<typeof adminBulkUpdateUserResSchema>;

// ==========================================
// 3. Admin Video 타입
// ==========================================

// Video 상태
export const videoStateSchema = z.enum(["ready", "open", "close"]);
export type VideoState = z.infer<typeof videoStateSchema>;

// Video 접근 권한
export const videoAccessSchema = z.enum(["public", "paid", "private", "promote"]);
export type VideoAccess = z.infer<typeof videoAccessSchema>;

// 비디오 목록 아이템 (요약)
export const adminVideoSummarySchema = z.object({
  id: z.number().int(),
  title: z.string(),
  url: z.string().nullable(),
  description: z.string().nullable(),
  views: z.number().int(),
  video_state: videoStateSchema,
  video_access: videoAccessSchema,
  video_idx: z.string(),
  video_tag_key: z.string().nullable(),
  updated_by_user_id: z.number().int().nullable(),
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
});
export type AdminVideoSummary = z.infer<typeof adminVideoSummarySchema>;

// 비디오 목록 응답 (pagination 필드명 주의)
export const adminVideoListResSchema = z.object({
  items: z.array(adminVideoSummarySchema),
  pagination: z.object({
    total_count: z.number().int(),
    total_pages: z.number().int(),
    current_page: z.number().int(),
    per_page: z.number().int(),
  }),
});
export type AdminVideoListRes = z.infer<typeof adminVideoListResSchema>;

// 비디오 상세 응답 (AdminVideoRes와 동일)
export const adminVideoResSchema = adminVideoSummarySchema;
export type AdminVideoRes = z.infer<typeof adminVideoResSchema>;

// 비디오 생성 요청
export const videoCreateReqSchema = z.object({
  // 1. video 테이블 컬럼
  video_idx: z.string().min(1).max(100).optional(),
  video_state: videoStateSchema.optional(), // 기본값: ready
  video_access: videoAccessSchema,
  // 2. video_tag 테이블 컬럼
  video_tag_title: z.string().min(1).max(200),
  video_tag_subtitle: z.string().max(500).optional(),
  video_tag_key: z.string().min(1).max(30).optional(),
  // 3. video URL
  video_url_vimeo: z.string().url().max(1024),
});
export type VideoCreateReq = z.infer<typeof videoCreateReqSchema>;

// 비디오 수정 요청
export const videoUpdateReqSchema = z.object({
  video_tag_title: z.string().min(1).max(200).optional(),
  video_tag_subtitle: z.string().max(500).optional(),
  video_tag_key: z.string().min(1).max(30).optional(),
  video_url_vimeo: z.string().url().max(1024).optional(),
  video_access: videoAccessSchema.optional(),
  video_state: videoStateSchema.optional(),
  video_idx: z.string().min(1).max(100).optional(),
});
export type VideoUpdateReq = z.infer<typeof videoUpdateReqSchema>;

// 비디오 태그 수정 요청
export const videoTagUpdateReqSchema = z.object({
  video_tag_title: z.string().min(1).max(200).optional(),
  video_tag_subtitle: z.string().max(500).optional(),
  video_tag_key: z.string().min(1).max(30).optional(),
});
export type VideoTagUpdateReq = z.infer<typeof videoTagUpdateReqSchema>;

// 벌크 생성 요청
export const videoBulkCreateReqSchema = z.object({
  items: z.array(videoCreateReqSchema).min(1).max(100),
});
export type VideoBulkCreateReq = z.infer<typeof videoBulkCreateReqSchema>;

// 벌크 생성 결과 아이템
export const videoBulkItemResultSchema = z.object({
  id: z.number().int().optional(),
  status: z.number().int(),
  data: adminVideoResSchema.optional(),
  error: bulkItemErrorSchema.optional(),
});
export type VideoBulkItemResult = z.infer<typeof videoBulkItemResultSchema>;

// 벌크 생성 응답
export const videoBulkCreateResSchema = z.object({
  summary: bulkSummarySchema,
  results: z.array(videoBulkItemResultSchema),
});
export type VideoBulkCreateRes = z.infer<typeof videoBulkCreateResSchema>;

// 벌크 수정 요청 아이템
export const videoBulkUpdateItemSchema = z.object({
  id: z.number().int(),
  video_idx: z.string().min(1).max(100).optional(),
  video_state: videoStateSchema.optional(),
  video_access: videoAccessSchema.optional(),
  video_tag_title: z.string().min(1).max(200).optional(),
  video_tag_subtitle: z.string().max(500).optional(),
  video_tag_key: z.string().min(1).max(30).optional(),
  video_url_vimeo: z.string().url().max(1024).optional(),
});
export type VideoBulkUpdateItem = z.infer<typeof videoBulkUpdateItemSchema>;

// 벌크 수정 요청
export const videoBulkUpdateReqSchema = z.object({
  items: z.array(videoBulkUpdateItemSchema).min(1).max(100),
});
export type VideoBulkUpdateReq = z.infer<typeof videoBulkUpdateReqSchema>;

// 벌크 수정 결과 아이템
export const videoBulkUpdateItemResultSchema = z.object({
  id: z.number().int(),
  status: z.number().int(),
  data: adminVideoResSchema.optional(),
  error: bulkItemErrorSchema.optional(),
});
export type VideoBulkUpdateItemResult = z.infer<typeof videoBulkUpdateItemResultSchema>;

// 벌크 수정 응답
export const videoBulkUpdateResSchema = z.object({
  summary: bulkSummarySchema,
  results: z.array(videoBulkUpdateItemResultSchema),
});
export type VideoBulkUpdateRes = z.infer<typeof videoBulkUpdateResSchema>;

// 벌크 태그 수정 요청 아이템
export const videoTagBulkUpdateItemSchema = z.object({
  id: z.number().int(),
  video_tag_title: z.string().min(1).max(200).optional(),
  video_tag_subtitle: z.string().max(500).optional(),
  video_tag_key: z.string().min(1).max(30).optional(),
});
export type VideoTagBulkUpdateItem = z.infer<typeof videoTagBulkUpdateItemSchema>;

// 벌크 태그 수정 요청
export const videoTagBulkUpdateReqSchema = z.object({
  items: z.array(videoTagBulkUpdateItemSchema).min(1).max(100),
});
export type VideoTagBulkUpdateReq = z.infer<typeof videoTagBulkUpdateReqSchema>;

// Vimeo 메타데이터 미리보기 응답
export const vimeoPreviewResSchema = z.object({
  vimeo_video_id: z.string(),
  title: z.string(),
  description: z.string().nullable(),
  duration: z.number().int(),
  thumbnail_url: z.string().nullable(),
});
export type VimeoPreviewRes = z.infer<typeof vimeoPreviewResSchema>;

// Vimeo 업로드 티켓 요청
export const vimeoUploadTicketReqSchema = z.object({
  file_name: z.string().min(1).max(255),
  file_size: z.number().int().min(1),
});
export type VimeoUploadTicketReq = z.infer<typeof vimeoUploadTicketReqSchema>;

// Vimeo 업로드 티켓 응답
export const vimeoUploadTicketResSchema = z.object({
  video_uri: z.string(),
  vimeo_video_id: z.string(),
  upload_link: z.string(),
});
export type VimeoUploadTicketRes = z.infer<typeof vimeoUploadTicketResSchema>;

// ==========================================
// 3-1. Admin Video Stats 타입
// ==========================================

// 통계 쿼리 파라미터 (공통)
export const statsQuerySchema = z.object({
  from: z.string(), // YYYY-MM-DD
  to: z.string(), // YYYY-MM-DD
});
export type StatsQuery = z.infer<typeof statsQuerySchema>;

// TOP 비디오 쿼리 파라미터
export const topVideosQuerySchema = statsQuerySchema.extend({
  limit: z.number().int().min(1).max(50).optional(),
  sort_by: z.enum(["views", "completes"]).optional(),
});
export type TopVideosQuery = z.infer<typeof topVideosQuerySchema>;

// 통계 요약 응답
export const statsSummaryResSchema = z.object({
  total_views: z.number().int(),
  total_completes: z.number().int(),
  active_video_count: z.number().int(),
  from_date: z.string(), // YYYY-MM-DD
  to_date: z.string(), // YYYY-MM-DD
});
export type StatsSummaryRes = z.infer<typeof statsSummaryResSchema>;

// TOP 비디오 아이템
export const topVideoItemSchema = z.object({
  rank: z.number().int(),
  video_id: z.number().int(),
  video_idx: z.string(),
  title: z.string().nullable(),
  views: z.number().int(),
  completes: z.number().int(),
});
export type TopVideoItem = z.infer<typeof topVideoItemSchema>;

// TOP 비디오 응답
export const topVideosResSchema = z.object({
  from_date: z.string(),
  to_date: z.string(),
  sort_by: z.string(),
  items: z.array(topVideoItemSchema),
});
export type TopVideosRes = z.infer<typeof topVideosResSchema>;

// 일별 통계 아이템
export const dailyStatItemSchema = z.object({
  date: z.string(), // YYYY-MM-DD
  views: z.number().int(),
  completes: z.number().int(),
});
export type DailyStatItem = z.infer<typeof dailyStatItemSchema>;

// 일별 통계 응답 (집계)
export const aggregateDailyStatsResSchema = z.object({
  from_date: z.string(),
  to_date: z.string(),
  items: z.array(dailyStatItemSchema),
});
export type AggregateDailyStatsRes = z.infer<typeof aggregateDailyStatsResSchema>;

// ==========================================
// 4. Admin Study 타입 (기본)
// ==========================================

export const adminStudySummarySchema = z.object({
  id: z.number().int(),
  study_idx: z.string(),
  title: z.string().nullable(),
  state: z.string(),
  created_at: z.string().datetime(),
});
export type AdminStudySummary = z.infer<typeof adminStudySummarySchema>;

export const adminStudyListResSchema = z.object({
  items: z.array(adminStudySummarySchema),
  meta: adminListMetaSchema,
});
export type AdminStudyListRes = z.infer<typeof adminStudyListResSchema>;

// ==========================================
// 5. Admin Lesson 타입 (기본)
// ==========================================

export const adminLessonSummarySchema = z.object({
  id: z.number().int(),
  lesson_idx: z.string(),
  title: z.string().nullable(),
  state: z.string(),
  created_at: z.string().datetime(),
});
export type AdminLessonSummary = z.infer<typeof adminLessonSummarySchema>;

export const adminLessonListResSchema = z.object({
  items: z.array(adminLessonSummarySchema),
  meta: adminListMetaSchema,
});
export type AdminLessonListRes = z.infer<typeof adminLessonListResSchema>;

// ==========================================
// 6. Admin User Logs 타입
// ==========================================

// 관리자 액션 타입
export const adminActionSchema = z.enum([
  "create",
  "update",
  "banned",
  "reorder",
  "publish",
  "unpublish",
]);
export type AdminAction = z.infer<typeof adminActionSchema>;

// 사용자 자체 액션 타입
export const userActionLogSchema = z.enum([
  "signup",
  "find_id",
  "reset_pw",
  "update",
]);
export type UserActionLog = z.infer<typeof userActionLogSchema>;

// 사용자 언어 설정
export const userLanguageSchema = z.enum(["ko", "en"]);
export type UserLanguage = z.infer<typeof userLanguageSchema>;

// 관리자 로그 요청 파라미터
export const adminUserLogsReqSchema = z.object({
  page: z.number().int().min(1).default(1),
  size: z.number().int().min(1).max(100).default(20),
});
export type AdminUserLogsReq = z.infer<typeof adminUserLogsReqSchema>;

// 관리자 변경 로그 아이템
export const adminUserLogItemSchema = z.object({
  id: z.number().int(),
  admin_id: z.number().int(),
  admin_email: z.string().nullable(),
  action: adminActionSchema,
  before: z.record(z.string(), z.unknown()).nullable(),
  after: z.record(z.string(), z.unknown()).nullable(),
  created_at: z.string().datetime(),
});
export type AdminUserLogItem = z.infer<typeof adminUserLogItemSchema>;

// 관리자 변경 로그 응답
export const adminUserLogsResSchema = z.object({
  items: z.array(adminUserLogItemSchema),
  meta: adminListMetaSchema,
});
export type AdminUserLogsRes = z.infer<typeof adminUserLogsResSchema>;

// 사용자 자체 변경 로그 아이템
export const userLogItemSchema = z.object({
  id: z.number().int(),
  action: userActionLogSchema,
  success: z.boolean(),
  email: z.string().nullable(),
  nickname: z.string().nullable(),
  language: userLanguageSchema.nullable(),
  country: z.string().nullable(),
  birthday: z.string().nullable(),
  gender: userGenderSchema.nullable(),
  password_changed: z.boolean(),
  created_at: z.string().datetime(),
});
export type UserLogItem = z.infer<typeof userLogItemSchema>;

// 사용자 자체 변경 로그 응답
export const userLogsResSchema = z.object({
  items: z.array(userLogItemSchema),
  meta: adminListMetaSchema,
});
export type UserLogsRes = z.infer<typeof userLogsResSchema>;
