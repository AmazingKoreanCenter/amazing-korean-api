import { request } from "@/api/client";
import type {
  StudyListReq,
  StudyListRes,
  StudyDetailReq,
  StudyDetailRes,
  StudyTaskDetailRes,
  SubmitAnswerReq,
  SubmitAnswerRes,
  TaskStatusRes,
  TaskExplainRes,
  StartWritingSessionReq,
  FinishWritingSessionReq,
  WritingSessionRes,
  WritingSessionListReq,
  WritingSessionListRes,
  WritingStatsReq,
  WritingStatsRes,
  WritingPracticeSeedReq,
  WritingPracticeSeedRes,
} from "@/category/study/types";

const sanitizeParams = <T extends Record<string, unknown>>(params: T): Partial<T> => {
  return Object.fromEntries(
    Object.entries(params).filter(([, value]) => value !== undefined)
  ) as Partial<T>;
};

export const getStudyList = (params: StudyListReq = {}, lang?: string) => {
  return request<StudyListRes>("/studies", {
    params: sanitizeParams({ ...params, lang }),
  });
};

export const getStudyDetail = (studyId: number, params: StudyDetailReq = {}, lang?: string) => {
  return request<StudyDetailRes>(`/studies/${studyId}`, {
    params: sanitizeParams({ ...params, lang }),
  });
};

export const getStudyTask = (taskId: number) => {
  return request<StudyTaskDetailRes>(`/studies/tasks/${taskId}`);
};

export const submitAnswer = (taskId: number, body: SubmitAnswerReq) => {
  return request<SubmitAnswerRes>(`/studies/tasks/${taskId}/answer`, {
    method: "POST",
    data: body,
  });
};

export const getTaskStatus = (taskId: number) => {
  return request<TaskStatusRes>(`/studies/tasks/${taskId}/status`);
};

export const getTaskExplain = (taskId: number) => {
  return request<TaskExplainRes>(`/studies/tasks/${taskId}/explain`);
};

// =========================================================================
// Writing Practice Session (한글 자판 연습)
// =========================================================================

export const startWritingSession = (body: StartWritingSessionReq) => {
  return request<WritingSessionRes>("/studies/writing/sessions", {
    method: "POST",
    data: body,
  });
};

export const finishWritingSession = (sessionId: number, body: FinishWritingSessionReq) => {
  return request<WritingSessionRes>(`/studies/writing/sessions/${sessionId}`, {
    method: "PATCH",
    data: body,
  });
};

export const listWritingSessions = (params: WritingSessionListReq = {}) => {
  return request<WritingSessionListRes>("/studies/writing/sessions", {
    params: sanitizeParams(params),
  });
};

export const getWritingStats = (params: WritingStatsReq = {}) => {
  return request<WritingStatsRes>("/studies/writing/stats", {
    params: sanitizeParams(params),
  });
};

// 자유 연습 시드 조회 (비인증 엔드포인트)
export const getWritingPracticeSeed = (params: WritingPracticeSeedReq) => {
  return request<WritingPracticeSeedRes>("/studies/writing/practice", {
    params: sanitizeParams(params),
  });
};
