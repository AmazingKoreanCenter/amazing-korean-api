import { request } from "@/api/client";
import type { StudyListReq, StudyListRes } from "@/category/study/types";

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
