import { request } from "@/api/client";
import type { ExplanationListRes } from "./types";

const sanitizeParams = <T extends Record<string, unknown>>(params: T): Partial<T> => {
  return Object.fromEntries(
    Object.entries(params).filter(([, value]) => value !== undefined)
  ) as Partial<T>;
};

/** study.study_idx 로 연결된 pattern_guide 해설 조회 (공개 읽기) */
export const getExplanationsByStudy = (studyIdx: string, lang?: string) => {
  return request<ExplanationListRes>("/explanations", {
    params: sanitizeParams({ study_idx: studyIdx, lang }),
  });
};

/** study_task.study_task_idx (amk500-sent-NNN) 로 연결된 sentence_explain 조회 (공개 읽기) */
export const getExplanationsByTask = (studyTaskIdx: string, lang?: string) => {
  return request<ExplanationListRes>("/explanations", {
    params: sanitizeParams({ study_task_idx: studyTaskIdx, lang }),
  });
};
