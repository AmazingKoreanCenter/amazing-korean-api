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
  AdminLessonListRes,
  AdminUserLogsReq,
  AdminUserLogsRes,
  UserLogsRes,
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
// Admin Studies API
// ==========================================

export const getAdminStudies = (params: AdminListReq) =>
  request<AdminStudyListRes>("/admin/studies", {
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
