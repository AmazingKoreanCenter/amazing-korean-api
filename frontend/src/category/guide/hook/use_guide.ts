import { useQuery } from "@tanstack/react-query";

import { getContentLang } from "@/utils/content_lang";

import { getGuide, getGuides } from "../guide_api";

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
