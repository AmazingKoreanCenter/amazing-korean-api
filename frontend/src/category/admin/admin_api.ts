import { request } from "@/api/client";
import type {
  AdminListReq,
  AdminUserListRes,
  AdminUserRes,
  AdminCreateUserReq,
  AdminUpdateUserReq,
  AdminBulkCreateUserReq,
  AdminBulkCreateUserRes,
  AdminVideoListRes,
  AdminStudyListRes,
  AdminLessonListRes,
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

// ==========================================
// Admin Videos API
// ==========================================

export const getAdminVideos = (params: AdminListReq) =>
  request<AdminVideoListRes>("/admin/videos", {
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
