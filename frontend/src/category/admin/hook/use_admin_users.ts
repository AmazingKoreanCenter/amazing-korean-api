import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

import {
  getAdminUsers,
  getAdminUser,
  createAdminUser,
  createAdminUsersBulk,
  updateAdminUser,
  updateAdminUsersBulk,
  getAdminUserLogs,
  getUserSelfLogs,
  getUserStatsSummary,
  getUserStatsSignups,
  getLoginStatsSummary,
  getLoginStatsDaily,
  getLoginStatsDevices,
} from "../admin_api";
import type {
  AdminListReq,
  AdminCreateUserReq,
  AdminUpdateUserReq,
  AdminBulkCreateUserReq,
  AdminBulkUpdateUserReq,
  AdminUserLogsReq,
  StatsQuery,
} from "../types";

// ==========================================
// Query Keys
// ==========================================

export const adminUsersKeys = {
  all: ["admin", "users"] as const,
  lists: () => [...adminUsersKeys.all, "list"] as const,
  list: (params: AdminListReq) => [...adminUsersKeys.lists(), params] as const,
  details: () => [...adminUsersKeys.all, "detail"] as const,
  detail: (id: number) => [...adminUsersKeys.details(), id] as const,
};

// ==========================================
// Queries
// ==========================================

/**
 * Admin 사용자 목록 조회
 */
export const useAdminUsers = (params: AdminListReq) => {
  return useQuery({
    queryKey: adminUsersKeys.list(params),
    queryFn: () => getAdminUsers(params),
  });
};

/**
 * Admin 사용자 상세 조회
 */
export const useAdminUserDetail = (id: number) => {
  return useQuery({
    queryKey: adminUsersKeys.detail(id),
    queryFn: () => getAdminUser(id),
    enabled: id > 0,
  });
};

// ==========================================
// Mutations
// ==========================================

/**
 * Admin 사용자 생성
 */
export const useCreateAdminUser = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: AdminCreateUserReq) => createAdminUser(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminUsersKeys.lists() });
    },
  });
};

/**
 * Admin 사용자 벌크 생성
 */
export const useCreateAdminUsersBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: AdminBulkCreateUserReq) => createAdminUsersBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminUsersKeys.lists() });
    },
  });
};

/**
 * Admin 사용자 수정
 */
export const useUpdateAdminUser = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: AdminUpdateUserReq }) =>
      updateAdminUser(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: adminUsersKeys.lists() });
      queryClient.invalidateQueries({ queryKey: adminUsersKeys.detail(id) });
    },
  });
};

/**
 * Admin 사용자 벌크 수정
 */
export const useUpdateAdminUsersBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: AdminBulkUpdateUserReq) => updateAdminUsersBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminUsersKeys.lists() });
      queryClient.invalidateQueries({ queryKey: adminUsersKeys.details() });
    },
  });
};

// ==========================================
// User Logs Queries
// ==========================================

/**
 * 관리자 변경 로그 조회 (admin이 해당 사용자를 수정한 기록)
 */
export const useAdminUserLogs = (userId: number, params: AdminUserLogsReq) => {
  return useQuery({
    queryKey: [...adminUsersKeys.detail(userId), "admin-logs", params] as const,
    queryFn: () => getAdminUserLogs(userId, params),
    enabled: userId > 0,
  });
};

/**
 * 사용자 자체 변경 로그 조회 (사용자가 직접 수정한 기록)
 */
export const useUserSelfLogs = (userId: number, params: AdminUserLogsReq) => {
  return useQuery({
    queryKey: [...adminUsersKeys.detail(userId), "user-logs", params] as const,
    queryFn: () => getUserSelfLogs(userId, params),
    enabled: userId > 0,
  });
};

// ==========================================
// User Stats Query Keys
// ==========================================

export const userStatsKeys = {
  all: ["admin", "users", "stats"] as const,
  summary: (params: StatsQuery) => [...userStatsKeys.all, "summary", params] as const,
  signups: (params: StatsQuery) => [...userStatsKeys.all, "signups", params] as const,
};

export const loginStatsKeys = {
  all: ["admin", "logins", "stats"] as const,
  summary: (params: StatsQuery) => [...loginStatsKeys.all, "summary", params] as const,
  daily: (params: StatsQuery) => [...loginStatsKeys.all, "daily", params] as const,
  devices: (params: StatsQuery) => [...loginStatsKeys.all, "devices", params] as const,
};

// ==========================================
// User Stats Hooks
// ==========================================

/**
 * 7-53: 사용자 요약 통계 조회
 */
export const useUserStatsSummary = (params: StatsQuery) => {
  return useQuery({
    queryKey: userStatsKeys.summary(params),
    queryFn: () => getUserStatsSummary(params),
    enabled: !!params.from && !!params.to,
  });
};

/**
 * 7-54: 일별 가입 통계 조회
 */
export const useUserStatsSignups = (params: StatsQuery) => {
  return useQuery({
    queryKey: userStatsKeys.signups(params),
    queryFn: () => getUserStatsSignups(params),
    enabled: !!params.from && !!params.to,
  });
};

// ==========================================
// Login Stats Hooks
// ==========================================

/**
 * 7-55: 로그인 요약 통계 조회
 */
export const useLoginStatsSummary = (params: StatsQuery) => {
  return useQuery({
    queryKey: loginStatsKeys.summary(params),
    queryFn: () => getLoginStatsSummary(params),
    enabled: !!params.from && !!params.to,
  });
};

/**
 * 7-56: 일별 로그인 통계 조회
 */
export const useLoginStatsDaily = (params: StatsQuery) => {
  return useQuery({
    queryKey: loginStatsKeys.daily(params),
    queryFn: () => getLoginStatsDaily(params),
    enabled: !!params.from && !!params.to,
  });
};

/**
 * 7-57: 디바이스별 통계 조회
 */
export const useLoginStatsDevices = (params: StatsQuery) => {
  return useQuery({
    queryKey: loginStatsKeys.devices(params),
    queryFn: () => getLoginStatsDevices(params),
    enabled: !!params.from && !!params.to,
  });
};
