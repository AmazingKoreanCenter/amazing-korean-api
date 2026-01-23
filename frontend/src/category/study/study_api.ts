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
} from "@/category/study/types";

const sanitizeParams = (params: StudyListReq) => {
  return Object.fromEntries(
    Object.entries(params).filter(([, value]) => value !== undefined)
  ) as StudyListReq;
};

export const getStudyList = (params: StudyListReq = {}) => {
  return request<StudyListRes>("/studies", {
    params: sanitizeParams(params),
  });
};

export const getStudyDetail = (studyId: number, params: StudyDetailReq = {}) => {
  return request<StudyDetailRes>(`/studies/${studyId}`, {
    params: sanitizeParams(params as StudyListReq),
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
