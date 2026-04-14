import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { WritingStatsReq } from "@/category/study/types";

import { getWritingStats } from "../study_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "통계를 불러오지 못했습니다. 잠시 후 다시 시도해주세요.";
};

/**
 * 한글 자판 연습 통계 (GET /studies/writing/stats)
 * days 범위 내에서 total/avg_accuracy/avg_cpm + 레벨별 + 일별 추이 + 취약 글자 Top 10
 */
export const useWritingStats = (params: WritingStatsReq = {}) => {
  const query = useQuery({
    queryKey: ["writing-stats", params],
    queryFn: () => getWritingStats(params),
    placeholderData: keepPreviousData,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
