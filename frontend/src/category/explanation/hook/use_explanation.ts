import { useQuery } from "@tanstack/react-query";

import { getContentLang } from "@/utils/content_lang";

import { getExplanationsByStudy, getExplanationsByTask } from "../explanation_api";

/**
 * study.study_idx 로 pattern_guide 해설 조회 (컨텍스트 통합 — study 상세에 표시).
 * 보조 콘텐츠라 에러 toast 없이 조용히 빈 결과 처리(없으면 섹션 미표시).
 */
export const useExplanationByStudy = (studyIdx: string | undefined) => {
  const lang = getContentLang();
  return useQuery({
    queryKey: ["explanation", "study", studyIdx, lang],
    queryFn: () => getExplanationsByStudy(studyIdx!, lang),
    enabled: typeof studyIdx === "string" && studyIdx.length > 0,
    staleTime: 5 * 60 * 1000,
  });
};

/**
 * study_task.study_task_idx 로 sentence_explain 해설 조회 (컨텍스트 통합 — task 화면에 표시).
 */
export const useExplanationByTask = (studyTaskIdx: string | undefined) => {
  const lang = getContentLang();
  return useQuery({
    queryKey: ["explanation", "task", studyTaskIdx, lang],
    queryFn: () => getExplanationsByTask(studyTaskIdx!, lang),
    enabled: typeof studyTaskIdx === "string" && studyTaskIdx.length > 0,
    staleTime: 5 * 60 * 1000,
  });
};
