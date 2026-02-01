import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

import {
  getAdminVideos,
  getAdminVideo,
  createAdminVideo,
  createAdminVideosBulk,
  updateAdminVideo,
  updateAdminVideosBulk,
  updateVideoTag,
  updateVideoTagsBulk,
  getVimeoPreview,
  createVimeoUploadTicket,
  getVideoStatsSummary,
  getVideoStatsTop,
  getVideoStatsDaily,
} from "../admin_api";
import type {
  AdminListReq,
  VideoCreateReq,
  VideoUpdateReq,
  VideoTagUpdateReq,
  VideoBulkCreateReq,
  VideoBulkUpdateReq,
  VideoTagBulkUpdateReq,
  VimeoUploadTicketReq,
  StatsQuery,
  TopVideosQuery,
} from "../types";

// ==========================================
// Query Keys
// ==========================================

export const adminVideosKeys = {
  all: ["admin", "videos"] as const,
  lists: () => [...adminVideosKeys.all, "list"] as const,
  list: (params: AdminListReq) => [...adminVideosKeys.lists(), params] as const,
  details: () => [...adminVideosKeys.all, "detail"] as const,
  detail: (id: number) => [...adminVideosKeys.details(), id] as const,
};

// ==========================================
// Queries
// ==========================================

/**
 * Admin 비디오 목록 조회
 */
export const useAdminVideos = (params: AdminListReq) => {
  return useQuery({
    queryKey: adminVideosKeys.list(params),
    queryFn: () => getAdminVideos(params),
  });
};

/**
 * Admin 비디오 상세 조회
 */
export const useAdminVideoDetail = (id: number) => {
  return useQuery({
    queryKey: adminVideosKeys.detail(id),
    queryFn: () => getAdminVideo(id),
    enabled: id > 0,
  });
};

// ==========================================
// Mutations
// ==========================================

/**
 * Admin 비디오 생성
 */
export const useCreateAdminVideo = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: VideoCreateReq) => createAdminVideo(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminVideosKeys.lists() });
    },
  });
};

/**
 * Admin 비디오 벌크 생성
 */
export const useCreateAdminVideosBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: VideoBulkCreateReq) => createAdminVideosBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminVideosKeys.lists() });
    },
  });
};

/**
 * Admin 비디오 수정
 */
export const useUpdateAdminVideo = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: VideoUpdateReq }) =>
      updateAdminVideo(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: adminVideosKeys.lists() });
      queryClient.invalidateQueries({ queryKey: adminVideosKeys.detail(id) });
    },
  });
};

/**
 * Admin 비디오 벌크 수정
 */
export const useUpdateAdminVideosBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: VideoBulkUpdateReq) => updateAdminVideosBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminVideosKeys.lists() });
      queryClient.invalidateQueries({ queryKey: adminVideosKeys.details() });
    },
  });
};

/**
 * 비디오 태그 수정
 */
export const useUpdateVideoTag = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: VideoTagUpdateReq }) =>
      updateVideoTag(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: adminVideosKeys.lists() });
      queryClient.invalidateQueries({ queryKey: adminVideosKeys.detail(id) });
    },
  });
};

/**
 * 비디오 태그 벌크 수정
 */
export const useUpdateVideoTagsBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: VideoTagBulkUpdateReq) => updateVideoTagsBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminVideosKeys.lists() });
      queryClient.invalidateQueries({ queryKey: adminVideosKeys.details() });
    },
  });
};

/**
 * Vimeo 메타데이터 미리보기
 */
export const useVimeoPreview = () => {
  return useMutation({
    mutationFn: (url: string) => getVimeoPreview(url),
  });
};

/**
 * Vimeo 업로드 티켓 생성 (tus resumable upload용)
 */
export const useCreateVimeoUploadTicket = () => {
  return useMutation({
    mutationFn: (data: VimeoUploadTicketReq) => createVimeoUploadTicket(data),
  });
};

// ==========================================
// Video Stats Query Keys
// ==========================================

export const videoStatsKeys = {
  all: ["admin", "videos", "stats"] as const,
  summary: (params: StatsQuery) => [...videoStatsKeys.all, "summary", params] as const,
  top: (params: TopVideosQuery) => [...videoStatsKeys.all, "top", params] as const,
  daily: (params: StatsQuery) => [...videoStatsKeys.all, "daily", params] as const,
};

// ==========================================
// Video Stats Queries
// ==========================================

/**
 * 통계 요약 조회 (총 조회수, 완료수, 활성 비디오 수)
 */
export const useVideoStatsSummary = (params: StatsQuery) => {
  return useQuery({
    queryKey: videoStatsKeys.summary(params),
    queryFn: () => getVideoStatsSummary(params),
    enabled: !!params.from && !!params.to,
  });
};

/**
 * TOP 비디오 조회
 */
export const useVideoStatsTop = (params: TopVideosQuery) => {
  return useQuery({
    queryKey: videoStatsKeys.top(params),
    queryFn: () => getVideoStatsTop(params),
    enabled: !!params.from && !!params.to,
  });
};

/**
 * 일별 통계 조회 (전체 집계)
 */
export const useVideoStatsDaily = (params: StatsQuery) => {
  return useQuery({
    queryKey: videoStatsKeys.daily(params),
    queryFn: () => getVideoStatsDaily(params),
    enabled: !!params.from && !!params.to,
  });
};
