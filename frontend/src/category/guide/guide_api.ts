import { request } from "@/api/client";

import type { GuideDetail, GuideListRes } from "./types";

const sanitizeParams = <T extends Record<string, unknown>>(params: T): Partial<T> => {
  return Object.fromEntries(
    Object.entries(params).filter(([, value]) => value !== undefined)
  ) as Partial<T>;
};

/** 공개 단원 목록 (state=open, 공개 읽기) */
export const getGuides = (lang?: string) => {
  return request<GuideListRes>("/guides", {
    params: sanitizeParams({ lang }),
  });
};

/** 단원 상세 (블록 스트림 + 표 격자 + 문장) */
export const getGuide = (guideIdx: string, lang?: string) => {
  return request<GuideDetail>(`/guides/${encodeURIComponent(guideIdx)}`, {
    params: sanitizeParams({ lang }),
  });
};
