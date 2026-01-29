import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

import {
  getAdminStudies,
  getAdminStudy,
  createAdminStudy,
  createAdminStudiesBulk,
  updateAdminStudy,
  updateAdminStudiesBulk,
  createAdminStudyTask,
  createAdminStudyTasksBulk,
  getAdminStudyTask,
  updateAdminStudyTask,
  updateAdminStudyTasksBulk,
  getAdminTaskExplains,
  createAdminTaskExplain,
  updateAdminTaskExplain,
  createAdminTaskExplainsBulk,
  updateAdminTaskExplainsBulk,
  getAdminTaskStatus,
  updateAdminTaskStatus,
  updateAdminTaskStatusBulk,
  getStudyStatsSummary,
  getStudyStatsTop,
  getStudyStatsDaily,
} from "../admin_api";
import type {
  StudyListReq,
  StudyCreateReq,
  StudyBulkCreateReq,
  StudyUpdateReq,
  StudyBulkUpdateReq,
  StudyTaskCreateReq,
  StudyTaskBulkCreateReq,
  StudyTaskUpdateReq,
  StudyTaskBulkUpdateReq,
  TaskExplainListReq,
  TaskExplainCreateReq,
  TaskExplainUpdateReq,
  TaskExplainBulkCreateReq,
  TaskExplainBulkUpdateReq,
  TaskStatusListReq,
  TaskStatusUpdateReq,
  TaskStatusBulkUpdateReq,
  StudyStatsQuery,
  TopStudiesQuery,
} from "../types";

// ==========================================
// Query Keys
// ==========================================

export const adminStudiesKeys = {
  all: ["admin", "studies"] as const,
  lists: () => [...adminStudiesKeys.all, "list"] as const,
  list: (params: StudyListReq) => [...adminStudiesKeys.lists(), params] as const,
  details: () => [...adminStudiesKeys.all, "detail"] as const,
  detail: (id: number) => [...adminStudiesKeys.details(), id] as const,
  tasks: () => [...adminStudiesKeys.all, "task"] as const,
  task: (taskId: number) => [...adminStudiesKeys.tasks(), taskId] as const,
  explains: () => [...adminStudiesKeys.all, "explain"] as const,
  explainList: (params: TaskExplainListReq) => [...adminStudiesKeys.explains(), "list", params] as const,
  statuses: () => [...adminStudiesKeys.all, "status"] as const,
  statusList: (params: TaskStatusListReq) => [...adminStudiesKeys.statuses(), "list", params] as const,
};

// ==========================================
// Queries
// ==========================================

/**
 * Admin 학습 목록 조회
 */
export const useAdminStudies = (params: StudyListReq) => {
  return useQuery({
    queryKey: adminStudiesKeys.list(params),
    queryFn: () => getAdminStudies(params),
  });
};

/**
 * Admin 학습 상세 조회
 */
export const useAdminStudyDetail = (id: number) => {
  return useQuery({
    queryKey: adminStudiesKeys.detail(id),
    queryFn: () => getAdminStudy(id),
    enabled: id > 0,
  });
};

/**
 * Admin 학습 Task 상세 조회
 */
export const useAdminStudyTaskDetail = (taskId: number) => {
  return useQuery({
    queryKey: adminStudiesKeys.task(taskId),
    queryFn: () => getAdminStudyTask(taskId),
    enabled: taskId > 0,
  });
};

// ==========================================
// Mutations
// ==========================================

/**
 * Admin 학습 생성
 */
export const useCreateAdminStudy = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: StudyCreateReq) => createAdminStudy(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.lists() });
    },
  });
};

/**
 * Admin 학습 벌크 생성
 */
export const useCreateAdminStudiesBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: StudyBulkCreateReq) => createAdminStudiesBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.lists() });
    },
  });
};

/**
 * Admin 학습 수정
 */
export const useUpdateAdminStudy = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: number; data: StudyUpdateReq }) =>
      updateAdminStudy(id, data),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.lists() });
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.detail(id) });
    },
  });
};

/**
 * Admin 학습 벌크 수정
 */
export const useUpdateAdminStudiesBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: StudyBulkUpdateReq) => updateAdminStudiesBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.lists() });
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.details() });
    },
  });
};

/**
 * Admin 학습 Task 생성
 */
export const useCreateAdminStudyTask = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: StudyTaskCreateReq) => createAdminStudyTask(data),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.detail(variables.study_id) });
    },
  });
};

/**
 * Admin 학습 Task 벌크 생성
 */
export const useCreateAdminStudyTasksBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: StudyTaskBulkCreateReq) => createAdminStudyTasksBulk(data),
    onSuccess: (_, variables) => {
      // Invalidate all study details that might be affected
      const studyIds = [...new Set(variables.items.map((item) => item.study_id))];
      studyIds.forEach((studyId) => {
        queryClient.invalidateQueries({ queryKey: adminStudiesKeys.detail(studyId) });
      });
    },
  });
};

/**
 * Admin 학습 Task 수정
 */
export const useUpdateAdminStudyTask = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ taskId, data }: { taskId: number; data: StudyTaskUpdateReq }) =>
      updateAdminStudyTask(taskId, data),
    onSuccess: (_, { taskId }) => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.task(taskId) });
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.details() });
    },
  });
};

/**
 * Admin 학습 Task 벌크 수정
 */
export const useUpdateAdminStudyTasksBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: StudyTaskBulkUpdateReq) => updateAdminStudyTasksBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.tasks() });
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.details() });
    },
  });
};

// ==========================================
// Task Explain Queries & Mutations
// ==========================================

/**
 * Admin Task Explain 목록 조회
 */
export const useAdminTaskExplains = (params: TaskExplainListReq) => {
  return useQuery({
    queryKey: adminStudiesKeys.explainList(params),
    queryFn: () => getAdminTaskExplains(params),
    enabled: params.task_id > 0,
  });
};

/**
 * Admin Task Explain 생성
 */
export const useCreateAdminTaskExplain = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ taskId, data }: { taskId: number; data: TaskExplainCreateReq }) =>
      createAdminTaskExplain(taskId, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.explains() });
    },
  });
};

/**
 * Admin Task Explain 수정
 */
export const useUpdateAdminTaskExplain = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ taskId, data }: { taskId: number; data: TaskExplainUpdateReq }) =>
      updateAdminTaskExplain(taskId, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.explains() });
    },
  });
};

/**
 * Admin Task Explain 벌크 생성
 */
export const useCreateAdminTaskExplainsBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: TaskExplainBulkCreateReq) => createAdminTaskExplainsBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.explains() });
    },
  });
};

/**
 * Admin Task Explain 벌크 수정
 */
export const useUpdateAdminTaskExplainsBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: TaskExplainBulkUpdateReq) => updateAdminTaskExplainsBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.explains() });
    },
  });
};

// ==========================================
// Task Status Queries & Mutations
// ==========================================

/**
 * Admin Task Status 목록 조회
 */
export const useAdminTaskStatus = (params: TaskStatusListReq) => {
  return useQuery({
    queryKey: adminStudiesKeys.statusList(params),
    queryFn: () => getAdminTaskStatus(params),
    enabled: Boolean(params.task_id && params.task_id > 0) || Boolean(params.user_id && params.user_id > 0),
  });
};

/**
 * Admin Task Status 수정
 */
export const useUpdateAdminTaskStatus = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ taskId, data }: { taskId: number; data: TaskStatusUpdateReq }) =>
      updateAdminTaskStatus(taskId, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.statuses() });
    },
  });
};

/**
 * Admin Task Status 벌크 수정
 */
export const useUpdateAdminTaskStatusBulk = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: TaskStatusBulkUpdateReq) => updateAdminTaskStatusBulk(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: adminStudiesKeys.statuses() });
    },
  });
};

// ==========================================
// Study Stats Query Keys
// ==========================================

export const studyStatsKeys = {
  all: ["admin", "studies", "stats"] as const,
  summary: (params: StudyStatsQuery) => [...studyStatsKeys.all, "summary", params] as const,
  top: (params: TopStudiesQuery) => [...studyStatsKeys.all, "top", params] as const,
  daily: (params: StudyStatsQuery) => [...studyStatsKeys.all, "daily", params] as const,
};

// ==========================================
// Study Stats Queries
// ==========================================

/**
 * Study 통계 요약 조회
 */
export const useStudyStatsSummary = (params: StudyStatsQuery) => {
  return useQuery({
    queryKey: studyStatsKeys.summary(params),
    queryFn: () => getStudyStatsSummary(params),
    enabled: !!params.from && !!params.to,
  });
};

/**
 * TOP Study 조회
 */
export const useStudyStatsTop = (params: TopStudiesQuery) => {
  return useQuery({
    queryKey: studyStatsKeys.top(params),
    queryFn: () => getStudyStatsTop(params),
    enabled: !!params.from && !!params.to,
  });
};

/**
 * Study 일별 통계 조회
 */
export const useStudyStatsDaily = (params: StudyStatsQuery) => {
  return useQuery({
    queryKey: studyStatsKeys.daily(params),
    queryFn: () => getStudyStatsDaily(params),
    enabled: !!params.from && !!params.to,
  });
};
