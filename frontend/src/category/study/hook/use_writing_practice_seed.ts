import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { WritingPracticeSeedReq } from "@/category/study/types";

import { getWritingPracticeSeed } from "../study_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

/**
 * 자유 연습 시드 조회 (GET /studies/writing/practice)
 * 시드는 거의 변하지 않으므로 staleTime 을 길게 둔다.
 */
export const useWritingPracticeSeed = (params: WritingPracticeSeedReq) => {
  const query = useQuery({
    queryKey: ["writing-practice-seed", params.level, params.practice_type, params.limit ?? null],
    queryFn: () => getWritingPracticeSeed(params),
    staleTime: 10 * 60 * 1000,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
