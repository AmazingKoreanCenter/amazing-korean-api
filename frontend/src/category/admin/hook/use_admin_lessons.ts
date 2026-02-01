import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

import {
  getAdminLessons,
  getAdminLesson,
  createAdminLesson,
  createAdminLessonsBulk,
  updateAdminLesson,
  updateAdminLessonsBulk,
  getAdminLessonItems,
  getAdminLessonItemsDetail,
  createAdminLessonItem,
  createAdminLessonItemsBulk,
  updateAdminLessonItem,
  updateAdminLessonItemsBulk,
  deleteAdminLessonItem,
  deleteAdminLessonItemsBulk,
  getAdminLessonProgress,
  getAdminLessonProgressDetail,
  updateAdminLessonProgress,
  updateAdminLessonProgressBulk,
} from "../admin_api";
import type {
  LessonListReq,
  LessonCreateReq,
  LessonBulkCreateReq,
  LessonUpdateReq,
  LessonBulkUpdateReq,
  LessonItemListReq,
  LessonItemCreateReq,
  LessonItemBulkCreateReq,
  LessonItemUpdateReq,
  LessonItemBulkUpdateReq,
  LessonItemBulkDeleteReq,
  LessonProgressListReq,
  LessonProgressUpdateReq,
  LessonProgressBulkUpdateReq,
} from "../lesson/types";

// ==========================================
// Query Keys
// ==========================================

export const adminLessonsKeys = {
  all: ["admin", "lessons"] as const,
  lists: () => [...adminLessonsKeys.all, "list"] as const,
  list: (params: LessonListReq) => [...adminLessonsKeys.lists(), params] as const,
  details: () => [...adminLessonsKeys.all, "detail"] as const,
  detail: (id: number) => [...adminLessonsKeys.details(), id] as const,
  items: () => [...adminLessonsKeys.all, "items"] as const,
  itemsList: (params: LessonItemListReq) => [...adminLessonsKeys.items(), "list", params] as const,
  itemsDetail: (id: number) => [...adminLessonsKeys.items(), "detail", id] as const,
  progress: () => [...adminLessonsKeys.all, "progress"] as const,
  progressList: (params: LessonProgressListReq) =>
    [...adminLessonsKeys.progress(), "list", params] as const,
  progressDetail: (id: number) => [...adminLessonsKeys.progress(), "detail", id] as const,
};

// ==========================================
// Lesson Queries
// ==========================================

/**
 * Admin 수업 목록 조회 (7-45)
 */
export const useAdminLessons = (params: LessonListReq) => {
  return useQuery({
    queryKey: adminLessonsKeys.list(params),
    queryFn: () => getAdminLessons(params),
  });
};

/**
 * Admin 수업 상세 조회 (7-46)
 */
export const useAdminLessonDetail = (id: number) => {
  return useQuery({
    queryKey: adminLessonsKeys.detail(id),
    queryFn: () => getAdminLesson(id),
    enabled: id > 0,
  });
};

// ==========================================
// Lesson Mutations
// ==========================================

/**
 * Admin 수업 생성 (7-47)
 */
export const useCreateAdminLesson = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: LessonCreateReq) => createAdminLesson(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.lists() });
    },
  });
};

/**
 * Admin 수업 벌크 생성 (7-48)
 */
export const useCreateAdminLessonsBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: LessonBulkCreateReq) => createAdminLessonsBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.lists() });
    },
  });
};

/**
 * Admin 수업 수정 (7-49)
 */
export const useUpdateAdminLesson = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: LessonUpdateReq }) => updateAdminLesson(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.lists() });
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.detail(id) });
    },
  });
};

/**
 * Admin 수업 벌크 수정 (7-50)
 */
export const useUpdateAdminLessonsBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: LessonBulkUpdateReq) => updateAdminLessonsBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.lists() });
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.details() });
    },
  });
};

// ==========================================
// Lesson Items Queries
// ==========================================

/**
 * Admin 수업 아이템 목록 조회 (7-51)
 */
export const useAdminLessonItems = (params: LessonItemListReq) => {
  return useQuery({
    queryKey: adminLessonsKeys.itemsList(params),
    queryFn: () => getAdminLessonItems(params),
  });
};

/**
 * Admin 수업 아이템 상세 조회 (7-52)
 */
export const useAdminLessonItemsDetail = (lessonId: number) => {
  return useQuery({
    queryKey: adminLessonsKeys.itemsDetail(lessonId),
    queryFn: () => getAdminLessonItemsDetail(lessonId),
    enabled: lessonId > 0,
  });
};

// ==========================================
// Lesson Items Mutations
// ==========================================

/**
 * Admin 수업 아이템 생성 (7-53)
 */
export const useCreateAdminLessonItem = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ lessonId, data }: { lessonId: number; data: LessonItemCreateReq }) =>
      createAdminLessonItem(lessonId, data),
    onSuccess: (_, { lessonId }) => {
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.itemsDetail(lessonId) });
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.items() });
    },
  });
};

/**
 * Admin 수업 아이템 벌크 생성 (7-54)
 */
export const useCreateAdminLessonItemsBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: LessonItemBulkCreateReq) => createAdminLessonItemsBulk(data),
    onSuccess: (_, variables) => {
      const lessonIds = [...new Set(variables.items.map((item) => item.lesson_id))];
      lessonIds.forEach((lessonId) => {
        queryClient.invalidateQueries({ queryKey: adminLessonsKeys.itemsDetail(lessonId) });
      });
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.items() });
    },
  });
};

/**
 * Admin 수업 아이템 수정 (7-55)
 */
export const useUpdateAdminLessonItem = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({
      lessonId,
      seq,
      data,
    }: {
      lessonId: number;
      seq: number;
      data: LessonItemUpdateReq;
    }) => updateAdminLessonItem(lessonId, seq, data),
    onSuccess: (_, { lessonId }) => {
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.itemsDetail(lessonId) });
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.items() });
    },
  });
};

/**
 * Admin 수업 아이템 삭제
 */
export const useDeleteAdminLessonItem = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ lessonId, seq }: { lessonId: number; seq: number }) =>
      deleteAdminLessonItem(lessonId, seq),
    onSuccess: (_, { lessonId }) => {
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.itemsDetail(lessonId) });
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.items() });
    },
  });
};

/**
 * Admin 수업 아이템 벌크 수정 (7-56)
 */
export const useUpdateAdminLessonItemsBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: LessonItemBulkUpdateReq) => updateAdminLessonItemsBulk(data),
    onSuccess: (_, variables) => {
      const lessonIds = [...new Set(variables.items.map((item) => item.lesson_id))];
      lessonIds.forEach((lessonId) => {
        queryClient.invalidateQueries({ queryKey: adminLessonsKeys.itemsDetail(lessonId) });
      });
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.items() });
    },
  });
};

/**
 * Admin 수업 아이템 벌크 삭제
 */
export const useDeleteAdminLessonItemsBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: LessonItemBulkDeleteReq) => deleteAdminLessonItemsBulk(data),
    onSuccess: (_, variables) => {
      const lessonIds = [...new Set(variables.items.map((item) => item.lesson_id))];
      lessonIds.forEach((lessonId) => {
        queryClient.invalidateQueries({ queryKey: adminLessonsKeys.itemsDetail(lessonId) });
      });
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.items() });
    },
  });
};

// ==========================================
// Lesson Progress Queries
// ==========================================

/**
 * Admin 수업 진행 목록 조회 (7-57)
 */
export const useAdminLessonProgress = (params: LessonProgressListReq) => {
  return useQuery({
    queryKey: adminLessonsKeys.progressList(params),
    queryFn: () => getAdminLessonProgress(params),
  });
};

/**
 * Admin 수업 진행 상세 조회 (7-58)
 */
export const useAdminLessonProgressDetail = (lessonId: number) => {
  return useQuery({
    queryKey: adminLessonsKeys.progressDetail(lessonId),
    queryFn: () => getAdminLessonProgressDetail(lessonId),
    enabled: lessonId > 0,
  });
};

// ==========================================
// Lesson Progress Mutations
// ==========================================

/**
 * Admin 수업 진행 수정 (7-59)
 */
export const useUpdateAdminLessonProgress = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ lessonId, data }: { lessonId: number; data: LessonProgressUpdateReq }) =>
      updateAdminLessonProgress(lessonId, data),
    onSuccess: (_, { lessonId }) => {
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.progressDetail(lessonId) });
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.progress() });
    },
  });
};

/**
 * Admin 수업 진행 벌크 수정 (7-60)
 */
export const useUpdateAdminLessonProgressBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: LessonProgressBulkUpdateReq) => updateAdminLessonProgressBulk(data),
    onSuccess: (_, variables) => {
      const lessonIds = [...new Set(variables.items.map((item) => item.lesson_id))];
      lessonIds.forEach((lessonId) => {
        queryClient.invalidateQueries({ queryKey: adminLessonsKeys.progressDetail(lessonId) });
      });
      queryClient.invalidateQueries({ queryKey: adminLessonsKeys.progress() });
    },
  });
};
