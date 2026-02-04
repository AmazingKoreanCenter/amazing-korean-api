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
// 3-2. Admin User Stats 타입
// ==========================================

// 역할별 사용자 수
export const usersByRoleSchema = z.object({
  hymn: z.number().int(),
  admin: z.number().int(),
  manager: z.number().int(),
  learner: z.number().int(),
});
export type UsersByRole = z.infer<typeof usersByRoleSchema>;

// 7-53: 사용자 요약 통계 응답
export const userStatsSummaryResSchema = z.object({
  total_users: z.number().int(),
  new_users: z.number().int(),
  active_users: z.number().int(),
  inactive_users: z.number().int(),
  by_role: usersByRoleSchema,
  from_date: z.string(),
  to_date: z.string(),
});
export type UserStatsSummaryRes = z.infer<typeof userStatsSummaryResSchema>;

// 일별 가입 통계 아이템
export const dailySignupItemSchema = z.object({
  date: z.string(),
  signups: z.number().int(),
  by_role: usersByRoleSchema,
});
export type DailySignupItem = z.infer<typeof dailySignupItemSchema>;

// 7-54: 일별 가입 통계 응답
export const userStatsSignupsResSchema = z.object({
  from_date: z.string(),
  to_date: z.string(),
  items: z.array(dailySignupItemSchema),
});
export type UserStatsSignupsRes = z.infer<typeof userStatsSignupsResSchema>;

// ==========================================
// 3-3. Admin Login Stats 타입
// ==========================================

// 7-55: 로그인 요약 통계 응답
export const loginStatsSummaryResSchema = z.object({
  total_logins: z.number().int(),
  success_count: z.number().int(),
  fail_count: z.number().int(),
  unique_users: z.number().int(),
  active_sessions: z.number().int(),
  from_date: z.string(),
  to_date: z.string(),
});
export type LoginStatsSummaryRes = z.infer<typeof loginStatsSummaryResSchema>;

// 일별 로그인 통계 아이템
export const dailyLoginItemSchema = z.object({
  date: z.string(),
  success: z.number().int(),
  fail: z.number().int(),
  unique_users: z.number().int(),
});
export type DailyLoginItem = z.infer<typeof dailyLoginItemSchema>;

// 7-56: 일별 로그인 통계 응답
export const loginStatsDailyResSchema = z.object({
  from_date: z.string(),
  to_date: z.string(),
  items: z.array(dailyLoginItemSchema),
});
export type LoginStatsDailyRes = z.infer<typeof loginStatsDailyResSchema>;

// 디바이스별 통계 아이템
export const deviceStatsItemSchema = z.object({
  device: z.string(),
  count: z.number().int(),
  percentage: z.number(),
});
export type DeviceStatsItem = z.infer<typeof deviceStatsItemSchema>;

// 7-57: 디바이스별 통계 응답
export const loginStatsDevicesResSchema = z.object({
  from_date: z.string(),
  to_date: z.string(),
  items: z.array(deviceStatsItemSchema),
});
export type LoginStatsDevicesRes = z.infer<typeof loginStatsDevicesResSchema>;

// ==========================================
// 4. Admin Study 타입 - study/types.ts에서 re-export
// ==========================================

export {
  studyStateSchema,
  type StudyState,
  studyAccessSchema,
  type StudyAccess,
  studyListReqSchema,
  type StudyListReq,
  studyCreateReqSchema,
  type StudyCreateReq,
  studyUpdateReqSchema,
  type StudyUpdateReq,
  studyBulkCreateReqSchema,
  type StudyBulkCreateReq,
  studyBulkCreateResSchema,
  type StudyBulkCreateRes,
  studyBulkUpdateReqSchema,
  type StudyBulkUpdateReq,
  adminStudyResSchema,
  type AdminStudyRes,
  adminStudyDetailResSchema,
  type AdminStudyDetailRes,
  adminStudyListResSchema,
  type AdminStudyListRes,
  studyBulkUpdateResSchema,
  type StudyBulkUpdateRes,
  adminStudyTaskResSchema,
  type AdminStudyTaskRes,
  studyTaskCreateReqSchema,
  type StudyTaskCreateReq,
  studyTaskBulkCreateReqSchema,
  type StudyTaskBulkCreateReq,
  studyTaskBulkCreateResSchema,
  type StudyTaskBulkCreateRes,
  adminStudyTaskDetailResSchema,
  type AdminStudyTaskDetailRes,
  // Task Update
  studyTaskUpdateReqSchema,
  type StudyTaskUpdateReq,
  studyTaskBulkUpdateReqSchema,
  type StudyTaskBulkUpdateReq,
  studyTaskBulkUpdateResSchema,
  type StudyTaskBulkUpdateRes,
  // Task Explain
  taskExplainListReqSchema,
  type TaskExplainListReq,
  taskExplainCreateReqSchema,
  type TaskExplainCreateReq,
  taskExplainUpdateReqSchema,
  type TaskExplainUpdateReq,
  taskExplainBulkCreateReqSchema,
  type TaskExplainBulkCreateReq,
  taskExplainBulkCreateResSchema,
  type TaskExplainBulkCreateRes,
  taskExplainBulkUpdateReqSchema,
  type TaskExplainBulkUpdateReq,
  taskExplainBulkUpdateResSchema,
  type TaskExplainBulkUpdateRes,
  adminTaskExplainResSchema,
  type AdminTaskExplainRes,
  adminTaskExplainListResSchema,
  type AdminTaskExplainListRes,
  // Task Status
  taskStatusListReqSchema,
  type TaskStatusListReq,
  taskStatusUpdateReqSchema,
  type TaskStatusUpdateReq,
  taskStatusBulkUpdateReqSchema,
  type TaskStatusBulkUpdateReq,
  taskStatusBulkUpdateResSchema,
  type TaskStatusBulkUpdateRes,
  adminTaskStatusResSchema,
  type AdminTaskStatusRes,
  adminTaskStatusListResSchema,
  type AdminTaskStatusListRes,
  // Study Stats
  studyStatsQuerySchema,
  type StudyStatsQuery,
  topStudiesQuerySchema,
  type TopStudiesQuery,
  programStatsSchema,
  type ProgramStats,
  stateStatsSchema,
  type StateStats,
  studyStatsSummaryResSchema,
  type StudyStatsSummaryRes,
  topStudyItemSchema,
  type TopStudyItem,
  topStudiesResSchema,
  type TopStudiesRes,
  studyDailyStatItemSchema,
  type StudyDailyStatItem,
  studyDailyStatsResSchema,
  type StudyDailyStatsRes,
} from "./study/types";

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

// ==========================================
// 8. Admin Email 타입
// ==========================================

// 이메일 템플릿 종류
export const emailTemplateTypeSchema = z.enum([
  "password_reset",
  "email_verification",
  "welcome",
]);
export type EmailTemplateType = z.infer<typeof emailTemplateTypeSchema>;

// 테스트 이메일 발송 요청
export const testEmailReqSchema = z.object({
  to: z.string().email("유효한 이메일 주소를 입력하세요"),
  template: emailTemplateTypeSchema,
});
export type TestEmailReq = z.infer<typeof testEmailReqSchema>;

// 테스트 이메일 발송 응답
export const testEmailResSchema = z.object({
  success: z.boolean(),
  message: z.string(),
  to: z.string(),
  template: emailTemplateTypeSchema,
});
export type TestEmailRes = z.infer<typeof testEmailResSchema>;

// ==========================================
// 9. Admin Upgrade (관리자 초대) 타입
// ==========================================

// 초대 가능한 역할
export const upgradeRoleSchema = z.enum(["admin", "manager"]);
export type UpgradeRole = z.infer<typeof upgradeRoleSchema>;

// 9-1: 관리자 초대 요청 (POST /admin/upgrade)
export const upgradeInviteReqSchema = z.object({
  email: z.string().email("유효한 이메일 주소를 입력하세요"),
  role: upgradeRoleSchema,
});
export type UpgradeInviteReq = z.infer<typeof upgradeInviteReqSchema>;

// 9-2: 관리자 초대 응답
export const upgradeInviteResSchema = z.object({
  message: z.string(),
  expires_at: z.string().datetime(),
});
export type UpgradeInviteRes = z.infer<typeof upgradeInviteResSchema>;

// 9-3: 초대 코드 검증 요청 (GET /admin/upgrade/verify?code=xxx)
export const upgradeVerifyReqSchema = z.object({
  code: z.string().min(1, "초대 코드가 필요합니다"),
});
export type UpgradeVerifyReq = z.infer<typeof upgradeVerifyReqSchema>;

// 9-4: 초대 코드 검증 응답
export const upgradeVerifyResSchema = z.object({
  email: z.string(),
  role: z.string(),
  invited_by: z.string(),
  expires_at: z.string().datetime(),
});
export type UpgradeVerifyRes = z.infer<typeof upgradeVerifyResSchema>;

// 9-5: 관리자 계정 생성 요청 (POST /admin/upgrade/accept)
export const upgradeAcceptReqSchema = z.object({
  code: z.string().min(1, "초대 코드가 필요합니다"),
  password: z
    .string()
    .min(8, "비밀번호는 8자 이상이어야 합니다")
    .refine(
      (val) => /[a-zA-Z]/.test(val) && /[0-9]/.test(val),
      "비밀번호는 영문과 숫자를 포함해야 합니다"
    ),
  name: z.string().min(1, "이름을 입력하세요").max(100),
  nickname: z.string().min(1, "닉네임을 입력하세요").max(100),
  country: z.string().min(1, "국가를 선택하세요").max(50),
  birthday: z.string().min(1, "생년월일을 입력하세요"), // YYYY-MM-DD
  gender: userGenderSchema,
  language: userLanguageSchema,
});
export type UpgradeAcceptReq = z.infer<typeof upgradeAcceptReqSchema>;

// 9-6: 관리자 계정 생성 응답
export const upgradeAcceptResSchema = z.object({
  user_id: z.number().int(),
  email: z.string(),
  user_auth: userAuthSchema,
  message: z.string(),
});
export type UpgradeAcceptRes = z.infer<typeof upgradeAcceptResSchema>;
