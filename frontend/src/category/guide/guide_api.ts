import { request } from "@/api/client";

import type {
  GuideDetail,
  GuideListRes,
  GuideLogReq,
  GuideProgress,
  GuideSentenceStatus,
} from "./types";

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

/** 문장 학습 로그 기록 (인증 필요) — 응답 = 기록 직후 권위 상태 */
export const logGuideSentence = (
  guideIdx: string,
  sentenceNo: number,
  body: GuideLogReq
) => {
  return request<GuideSentenceStatus>(
    `/guides/${encodeURIComponent(guideIdx)}/sentences/${sentenceNo}/log`,
    { method: "POST", data: body }
  );
};

/** 내 단원 진행 (인증 필요) — status 행 있는 문장만 */
export const getGuideProgress = (guideIdx: string) => {
  return request<GuideProgress>(
    `/guides/${encodeURIComponent(guideIdx)}/progress`
  );
};
