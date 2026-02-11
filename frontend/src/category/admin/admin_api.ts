import { request } from "@/api/client";
import type {
  AdminListReq,
  AdminUserListRes,
  AdminUserRes,
  AdminCreateUserReq,
  AdminUpdateUserReq,
  AdminBulkCreateUserReq,
  AdminBulkCreateUserRes,
  AdminBulkUpdateUserReq,
  AdminBulkUpdateUserRes,
  AdminVideoListRes,
  AdminVideoRes,
  VideoCreateReq,
  VideoUpdateReq,
  VideoTagUpdateReq,
  VideoBulkCreateReq,
  VideoBulkCreateRes,
  VideoBulkUpdateReq,
  VideoBulkUpdateRes,
  VideoTagBulkUpdateReq,
  VimeoPreviewRes,
  VimeoUploadTicketReq,
  VimeoUploadTicketRes,
  StatsQuery,
  TopVideosQuery,
  StatsSummaryRes,
  TopVideosRes,
  AggregateDailyStatsRes,
  AdminStudyListRes,
  AdminStudyDetailRes,
  AdminStudyRes,
  StudyListReq,
  StudyCreateReq,
  StudyUpdateReq,
  StudyBulkCreateReq,
  StudyBulkCreateRes,
  StudyBulkUpdateReq,
  StudyBulkUpdateRes,
  StudyTaskCreateReq,
  StudyTaskBulkCreateReq,
  StudyTaskBulkCreateRes,
  StudyTaskUpdateReq,
  StudyTaskBulkUpdateReq,
  StudyTaskBulkUpdateRes,
  AdminStudyTaskDetailRes,
  TaskExplainListReq,
  TaskExplainCreateReq,
  TaskExplainUpdateReq,
  TaskExplainBulkCreateReq,
  TaskExplainBulkCreateRes,
  TaskExplainBulkUpdateReq,
  TaskExplainBulkUpdateRes,
  AdminTaskExplainListRes,
  AdminTaskExplainRes,
  TaskStatusListReq,
  TaskStatusUpdateReq,
  TaskStatusBulkUpdateReq,
  TaskStatusBulkUpdateRes,
  AdminTaskStatusListRes,
  AdminTaskStatusRes,
  AdminUserLogsReq,
  AdminUserLogsRes,
  UserLogsRes,
  UserStatsSummaryRes,
  UserStatsSignupsRes,
  LoginStatsSummaryRes,
  LoginStatsDailyRes,
  LoginStatsDevicesRes,
  StudyStatsQuery,
  TopStudiesQuery,
  StudyStatsSummaryRes,
  TopStudiesRes,
  StudyDailyStatsRes,
  TestEmailReq,
  TestEmailRes,
  UpgradeInviteReq,
  UpgradeInviteRes,
  UpgradeVerifyRes,
  UpgradeAcceptReq,
  UpgradeAcceptRes,
} from "./types";

import type {
  TranslationListReq,
  TranslationCreateReq,
  TranslationUpdateReq,
  TranslationStatusUpdateReq,
  TranslationRes,
  TranslationListRes,
  AutoTranslateReq,
  AutoTranslateRes,
} from "./translation/types";

import type {
  LessonListReq,
  AdminLessonListRes,
  AdminLessonRes,
  LessonCreateReq,
  LessonBulkCreateReq,
  LessonBulkCreateRes,
  LessonUpdateReq,
  LessonBulkUpdateReq,
  LessonBulkUpdateRes,
  LessonItemListReq,
  AdminLessonItemListRes,
  AdminLessonItemsDetailRes,
  LessonItemCreateReq,
  AdminLessonItemRes,
  LessonItemBulkCreateReq,
  LessonItemBulkCreateRes,
  LessonItemUpdateReq,
  LessonItemBulkUpdateReq,
  LessonItemBulkUpdateRes,
  LessonItemBulkDeleteReq,
  LessonItemBulkDeleteRes,
  LessonProgressListReq,
  AdminLessonProgressListRes,
  AdminLessonProgressListDetailRes,
  LessonProgressUpdateReq,
  AdminLessonProgressRes,
  LessonProgressBulkUpdateReq,
  LessonProgressBulkUpdateRes,
} from "./lesson/types";

// ==========================================
// Admin Users API
// ==========================================

export const getAdminUsers = (params: AdminListReq) =>
  request<AdminUserListRes>("/admin/users", {
    method: "GET",
    params,
  });

export const getAdminUser = (id: number) =>
  request<AdminUserRes>(`/admin/users/${id}`, {
    method: "GET",
  });

export const createAdminUser = (data: AdminCreateUserReq) =>
  request<AdminUserRes>("/admin/users", {
    method: "POST",
    data,
  });

export const createAdminUsersBulk = (data: AdminBulkCreateUserReq) =>
  request<AdminBulkCreateUserRes>("/admin/users/bulk", {
    method: "POST",
    data,
  });

export const updateAdminUser = (id: number, data: AdminUpdateUserReq) =>
  request<AdminUserRes>(`/admin/users/${id}`, {
    method: "PATCH",
    data,
  });

export const updateAdminUsersBulk = (data: AdminBulkUpdateUserReq) =>
  request<AdminBulkUpdateUserRes>("/admin/users/bulk", {
    method: "PATCH",
    data,
  });

export const getAdminUserLogs = (userId: number, params: AdminUserLogsReq) =>
  request<AdminUserLogsRes>(`/admin/users/${userId}/admin-logs`, {
    method: "GET",
    params,
  });

export const getUserSelfLogs = (userId: number, params: AdminUserLogsReq) =>
  request<UserLogsRes>(`/admin/users/${userId}/user-logs`, {
    method: "GET",
    params,
  });

// ==========================================
// Admin Videos API
// ==========================================

export const getAdminVideos = (params: AdminListReq) =>
  request<AdminVideoListRes>("/admin/videos", {
    method: "GET",
    params,
  });

export const getAdminVideo = (id: number) =>
  request<AdminVideoRes>(`/admin/videos/${id}`, {
    method: "GET",
  });

export const createAdminVideo = (data: VideoCreateReq) =>
  request<AdminVideoRes>("/admin/videos", {
    method: "POST",
    data,
  });

export const createAdminVideosBulk = (data: VideoBulkCreateReq) =>
  request<VideoBulkCreateRes>("/admin/videos/bulk", {
    method: "POST",
    data,
  });

export const updateAdminVideo = (id: number, data: VideoUpdateReq) =>
  request<AdminVideoRes>(`/admin/videos/${id}`, {
    method: "PATCH",
    data,
  });

export const updateAdminVideosBulk = (data: VideoBulkUpdateReq) =>
  request<VideoBulkUpdateRes>("/admin/videos/bulk", {
    method: "PATCH",
    data,
  });

export const updateVideoTag = (id: number, data: VideoTagUpdateReq) =>
  request<AdminVideoRes>(`/admin/videos/${id}/tags`, {
    method: "PATCH",
    data,
  });

export const updateVideoTagsBulk = (data: VideoTagBulkUpdateReq) =>
  request<VideoBulkUpdateRes>("/admin/videos/tags/bulk", {
    method: "PATCH",
    data,
  });

export const getVimeoPreview = (url: string) =>
  request<VimeoPreviewRes>("/admin/videos/vimeo/preview", {
    method: "GET",
    params: { url },
  });

export const createVimeoUploadTicket = (data: VimeoUploadTicketReq) =>
  request<VimeoUploadTicketRes>("/admin/videos/vimeo/upload-ticket", {
    method: "POST",
    data,
  });

// ==========================================
// Admin Video Stats API
// ==========================================

export const getVideoStatsSummary = (params: StatsQuery) =>
  request<StatsSummaryRes>("/admin/videos/stats/summary", {
    method: "GET",
    params,
  });

export const getVideoStatsTop = (params: TopVideosQuery) =>
  request<TopVideosRes>("/admin/videos/stats/top", {
    method: "GET",
    params,
  });

export const getVideoStatsDaily = (params: StatsQuery) =>
  request<AggregateDailyStatsRes>("/admin/videos/stats/daily", {
    method: "GET",
    params,
  });

// ==========================================
// Admin User Stats API
// ==========================================

export const getUserStatsSummary = (params: StatsQuery) =>
  request<UserStatsSummaryRes>("/admin/users/stats/summary", {
    method: "GET",
    params,
  });

export const getUserStatsSignups = (params: StatsQuery) =>
  request<UserStatsSignupsRes>("/admin/users/stats/signups", {
    method: "GET",
    params,
  });

// ==========================================
// Admin Login Stats API
// ==========================================

export const getLoginStatsSummary = (params: StatsQuery) =>
  request<LoginStatsSummaryRes>("/admin/logins/stats/summary", {
    method: "GET",
    params,
  });

export const getLoginStatsDaily = (params: StatsQuery) =>
  request<LoginStatsDailyRes>("/admin/logins/stats/daily", {
    method: "GET",
    params,
  });

export const getLoginStatsDevices = (params: StatsQuery) =>
  request<LoginStatsDevicesRes>("/admin/logins/stats/devices", {
    method: "GET",
    params,
  });

// ==========================================
// Admin Studies API
// ==========================================

export const getAdminStudies = (params: StudyListReq) =>
  request<AdminStudyListRes>("/admin/studies", {
    method: "GET",
    params,
  });

export const getAdminStudy = (id: number) =>
  request<AdminStudyDetailRes>(`/admin/studies/${id}`, {
    method: "GET",
  });

export const createAdminStudy = (data: StudyCreateReq) =>
  request<AdminStudyRes>("/admin/studies", {
    method: "POST",
    data,
  });

export const createAdminStudiesBulk = (data: StudyBulkCreateReq) =>
  request<StudyBulkCreateRes>("/admin/studies/bulk", {
    method: "POST",
    data,
  });

export const updateAdminStudy = (id: number, data: StudyUpdateReq) =>
  request<AdminStudyRes>(`/admin/studies/${id}`, {
    method: "PATCH",
    data,
  });

export const updateAdminStudiesBulk = (data: StudyBulkUpdateReq) =>
  request<StudyBulkUpdateRes>("/admin/studies/bulk", {
    method: "PATCH",
    data,
  });

// ==========================================
// Admin Study Tasks API
// ==========================================

export const createAdminStudyTask = (data: StudyTaskCreateReq) =>
  request<AdminStudyTaskDetailRes>("/admin/studies/tasks", {
    method: "POST",
    data,
  });

export const createAdminStudyTasksBulk = (data: StudyTaskBulkCreateReq) =>
  request<StudyTaskBulkCreateRes>("/admin/studies/tasks/bulk", {
    method: "POST",
    data,
  });

export const getAdminStudyTask = (taskId: number) =>
  request<AdminStudyTaskDetailRes>(`/admin/studies/tasks/${taskId}`, {
    method: "GET",
  });

export const updateAdminStudyTask = (taskId: number, data: StudyTaskUpdateReq) =>
  request<AdminStudyTaskDetailRes>(`/admin/studies/tasks/${taskId}`, {
    method: "PATCH",
    data,
  });

export const updateAdminStudyTasksBulk = (data: StudyTaskBulkUpdateReq) =>
  request<StudyTaskBulkUpdateRes>("/admin/studies/tasks/bulk", {
    method: "PATCH",
    data,
  });

// ==========================================
// Admin Study Task Explains API
// ==========================================

export const getAdminTaskExplains = (params: TaskExplainListReq) =>
  request<AdminTaskExplainListRes>("/admin/studies/tasks/explain", {
    method: "GET",
    params,
  });

export const createAdminTaskExplain = (taskId: number, data: TaskExplainCreateReq) =>
  request<AdminTaskExplainRes>(`/admin/studies/tasks/${taskId}/explain`, {
    method: "POST",
    data,
  });

export const updateAdminTaskExplain = (taskId: number, data: TaskExplainUpdateReq) =>
  request<AdminTaskExplainRes>(`/admin/studies/tasks/${taskId}/explain`, {
    method: "PATCH",
    data,
  });

export const createAdminTaskExplainsBulk = (data: TaskExplainBulkCreateReq) =>
  request<TaskExplainBulkCreateRes>("/admin/studies/tasks/bulk/explain", {
    method: "POST",
    data,
  });

export const updateAdminTaskExplainsBulk = (data: TaskExplainBulkUpdateReq) =>
  request<TaskExplainBulkUpdateRes>("/admin/studies/tasks/bulk/explain", {
    method: "PATCH",
    data,
  });

// ==========================================
// Admin Study Task Status API
// ==========================================

export const getAdminTaskStatus = (params: TaskStatusListReq) =>
  request<AdminTaskStatusListRes>("/admin/studies/tasks/status", {
    method: "GET",
    params,
  });

export const updateAdminTaskStatus = (taskId: number, data: TaskStatusUpdateReq) =>
  request<AdminTaskStatusRes>(`/admin/studies/tasks/${taskId}/status`, {
    method: "PATCH",
    data,
  });

export const updateAdminTaskStatusBulk = (data: TaskStatusBulkUpdateReq) =>
  request<TaskStatusBulkUpdateRes>("/admin/studies/tasks/bulk/status", {
    method: "PATCH",
    data,
  });

// ==========================================
// Admin Study Stats API
// ==========================================

export const getStudyStatsSummary = (params: StudyStatsQuery) =>
  request<StudyStatsSummaryRes>("/admin/studies/stats/summary", {
    method: "GET",
    params,
  });

export const getStudyStatsTop = (params: TopStudiesQuery) =>
  request<TopStudiesRes>("/admin/studies/stats/top", {
    method: "GET",
    params,
  });

export const getStudyStatsDaily = (params: StudyStatsQuery) =>
  request<StudyDailyStatsRes>("/admin/studies/stats/daily", {
    method: "GET",
    params,
  });

// ==========================================
// Admin Lessons API
// ==========================================

// 7-45: Lesson List
export const getAdminLessons = (params: LessonListReq) =>
  request<AdminLessonListRes>("/admin/lessons", {
    method: "GET",
    params,
  });

// 7-46: Lesson Detail
export const getAdminLesson = (id: number) =>
  request<AdminLessonRes>(`/admin/lessons/${id}`, {
    method: "GET",
  });

// 7-47: Lesson Create
export const createAdminLesson = (data: LessonCreateReq) =>
  request<AdminLessonRes>("/admin/lessons", {
    method: "POST",
    data,
  });

// 7-48: Lesson Bulk Create
export const createAdminLessonsBulk = (data: LessonBulkCreateReq) =>
  request<LessonBulkCreateRes>("/admin/lessons/bulk", {
    method: "POST",
    data,
  });

// 7-49: Lesson Update
export const updateAdminLesson = (id: number, data: LessonUpdateReq) =>
  request<AdminLessonRes>(`/admin/lessons/${id}`, {
    method: "PATCH",
    data,
  });

// 7-50: Lesson Bulk Update
export const updateAdminLessonsBulk = (data: LessonBulkUpdateReq) =>
  request<LessonBulkUpdateRes>("/admin/lessons/bulk", {
    method: "PATCH",
    data,
  });

// 7-51: Lesson Items List
export const getAdminLessonItems = (params: LessonItemListReq) =>
  request<AdminLessonItemListRes>("/admin/lessons/items", {
    method: "GET",
    params,
  });

// 7-52: Lesson Items Detail
export const getAdminLessonItemsDetail = (lessonId: number) =>
  request<AdminLessonItemsDetailRes>(`/admin/lessons/items/${lessonId}`, {
    method: "GET",
  });

// 7-53: Lesson Item Create
export const createAdminLessonItem = (lessonId: number, data: LessonItemCreateReq) =>
  request<AdminLessonItemRes>(`/admin/lessons/${lessonId}/items`, {
    method: "POST",
    data,
  });

// 7-54: Lesson Items Bulk Create
export const createAdminLessonItemsBulk = (data: LessonItemBulkCreateReq) =>
  request<LessonItemBulkCreateRes>("/admin/lessons/bulk/items", {
    method: "POST",
    data,
  });

// 7-55: Lesson Item Update
export const updateAdminLessonItem = (lessonId: number, seq: number, data: LessonItemUpdateReq) =>
  request<AdminLessonItemRes>(`/admin/lessons/${lessonId}/items/${seq}`, {
    method: "PATCH",
    data,
  });

// DELETE: Lesson Item Delete
export const deleteAdminLessonItem = (lessonId: number, seq: number) =>
  request<void>(`/admin/lessons/${lessonId}/items/${seq}`, {
    method: "DELETE",
  });

// 7-56: Lesson Items Bulk Update
export const updateAdminLessonItemsBulk = (data: LessonItemBulkUpdateReq) =>
  request<LessonItemBulkUpdateRes>("/admin/lessons/bulk/items", {
    method: "PATCH",
    data,
  });

// Lesson Items Bulk Delete
export const deleteAdminLessonItemsBulk = (data: LessonItemBulkDeleteReq) =>
  request<LessonItemBulkDeleteRes>("/admin/lessons/bulk/items", {
    method: "DELETE",
    data,
  });

// 7-57: Lesson Progress List
export const getAdminLessonProgress = (params: LessonProgressListReq) =>
  request<AdminLessonProgressListRes>("/admin/lessons/progress", {
    method: "GET",
    params,
  });

// 7-58: Lesson Progress Detail
export const getAdminLessonProgressDetail = (lessonId: number) =>
  request<AdminLessonProgressListDetailRes>(`/admin/lessons/progress/${lessonId}`, {
    method: "GET",
  });

// 7-59: Lesson Progress Update
export const updateAdminLessonProgress = (lessonId: number, data: LessonProgressUpdateReq) =>
  request<AdminLessonProgressRes>(`/admin/lessons/${lessonId}/progress`, {
    method: "PATCH",
    data,
  });

// 7-60: Lesson Progress Bulk Update
export const updateAdminLessonProgressBulk = (data: LessonProgressBulkUpdateReq) =>
  request<LessonProgressBulkUpdateRes>("/admin/lessons/bulk/progress", {
    method: "PATCH",
    data,
  });

// ==========================================
// 8. Admin Email
// ==========================================

// 8-1: Test Email
export const sendTestEmail = (data: TestEmailReq) =>
  request<TestEmailRes>("/admin/email/test", {
    method: "POST",
    data,
  });

// ==========================================
// 9. Admin Upgrade (관리자 초대)
// ==========================================

// 9-1: 관리자 초대 (POST /admin/upgrade)
export const createAdminInvite = (data: UpgradeInviteReq) =>
  request<UpgradeInviteRes>("/admin/upgrade", {
    method: "POST",
    data,
  });

// 9-2: 초대 코드 검증 (GET /admin/upgrade/verify?code=xxx)
export const verifyAdminInvite = (code: string) =>
  request<UpgradeVerifyRes>("/admin/upgrade/verify", {
    method: "GET",
    params: { code },
  });

// 9-3: 관리자 계정 생성 (POST /admin/upgrade/accept)
export const acceptAdminInvite = (data: UpgradeAcceptReq) =>
  request<UpgradeAcceptRes>("/admin/upgrade/accept", {
    method: "POST",
    data,
  });

// ==========================================
// 10. Admin Translations API
// ==========================================

export const getAdminTranslations = (params: TranslationListReq) =>
  request<TranslationListRes>("/admin/translations", {
    method: "GET",
    params,
  });

export const getAdminTranslation = (id: number) =>
  request<TranslationRes>(`/admin/translations/${id}`, {
    method: "GET",
  });

export const createAdminTranslation = (data: TranslationCreateReq) =>
  request<TranslationRes>("/admin/translations", {
    method: "POST",
    data,
  });

export const updateAdminTranslation = (id: number, data: TranslationUpdateReq) =>
  request<TranslationRes>(`/admin/translations/${id}`, {
    method: "PUT",
    data,
  });

export const updateAdminTranslationStatus = (id: number, data: TranslationStatusUpdateReq) =>
  request<TranslationRes>(`/admin/translations/${id}/status`, {
    method: "PATCH",
    data,
  });

export const deleteAdminTranslation = (id: number) =>
  request<void>(`/admin/translations/${id}`, {
    method: "DELETE",
  });

export const autoTranslateContent = (data: AutoTranslateReq) =>
  request<AutoTranslateRes>("/admin/translations/auto", {
    method: "POST",
    data,
  });
