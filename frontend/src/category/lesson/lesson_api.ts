import { request } from "@/api/client";
import type {
  LessonListReq,
  LessonListRes,
  LessonDetailRes,
  LessonItemsRes,
  LessonProgressRes,
  LessonProgressUpdateReq,
} from "@/category/lesson/types";

const sanitizeParams = <T extends Record<string, unknown>>(params: T) => {
  return Object.fromEntries(
    Object.entries(params).filter(([, value]) => value !== undefined)
  ) as T;
};

export const getLessonList = (params: LessonListReq = {}, lang?: string) => {
  return request<LessonListRes>("/lessons", {
    params: sanitizeParams({ ...params, lang }),
  });
};

export const getLessonDetail = (lessonId: number, lang?: string) => {
  return request<LessonDetailRes>(`/lessons/${lessonId}`, {
    params: lang ? { lang } : undefined,
  });
};

export const getLessonItems = (lessonId: number, params: { page?: number; per_page?: number } = {}) => {
  return request<LessonItemsRes>(`/lessons/${lessonId}/items`, {
    params: sanitizeParams(params),
  });
};

export const getLessonProgress = (lessonId: number) => {
  return request<LessonProgressRes>(`/lessons/${lessonId}/progress`);
};

export const updateLessonProgress = (lessonId: number, body: LessonProgressUpdateReq) => {
  return request<LessonProgressRes>(`/lessons/${lessonId}/progress`, {
    method: "POST",
    data: body,
  });
};
