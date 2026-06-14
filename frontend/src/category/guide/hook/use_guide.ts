import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { useAuthStore } from "@/hooks/use_auth_store";
import { getContentLang } from "@/utils/content_lang";

import {
  getGuide,
  getGuideProgress,
  getGuides,
  logGuideSentence,
} from "../guide_api";
import type { GuideLogReq } from "../types";

/** 공개 단원 목록 (표시 언어 = 콘텐츠 언어) */
export const useGuides = () => {
  const lang = getContentLang();
  return useQuery({
    queryKey: ["guides", lang],
    queryFn: () => getGuides(lang),
    staleTime: 5 * 60 * 1000,
  });
};

/** 단원 상세 */
export const useGuide = (guideIdx: string | undefined) => {
  const lang = getContentLang();
  return useQuery({
    queryKey: ["guide", guideIdx, lang],
    queryFn: () => getGuide(guideIdx!, lang),
    enabled: typeof guideIdx === "string" && guideIdx.length > 0,
    staleTime: 5 * 60 * 1000,
  });
};

/** 내 단원 진행 (로그인 시에만 조회 — 비로그인은 진행 추적 없음) */
export const useGuideProgress = (guideIdx: string | undefined) => {
  const isLoggedIn = useAuthStore((s) => s.isLoggedIn);
  return useQuery({
    queryKey: ["guide-progress", guideIdx],
    queryFn: () => getGuideProgress(guideIdx!),
    enabled: isLoggedIn && typeof guideIdx === "string" && guideIdx.length > 0,
    staleTime: 60 * 1000,
  });
};

/**
 * 문장 학습 로그 기록 mutation. 성공 시 진행 캐시 무효화.
 * 진행 저장은 best-effort — 실패해도 학습 흐름을 막지 않음(토스트 없음, 의도적).
 * 호출부에서 로그인 여부를 가드(비로그인은 mutate 호출 안 함).
 */
export const useGuideLog = (guideIdx: string | undefined) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      sentenceNo,
      body,
    }: {
      sentenceNo: number;
      body: GuideLogReq;
    }) => logGuideSentence(guideIdx!, sentenceNo, body),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["guide-progress", guideIdx] });
    },
  });
};
