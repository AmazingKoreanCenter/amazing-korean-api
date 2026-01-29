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
  AdminLessonListRes,
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
} from "./types";

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

export const getAdminLessons = (params: AdminListReq) =>
  request<AdminLessonListRes>("/admin/lessons", {
    method: "GET",
    params,
  });
