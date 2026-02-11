import { useEffect } from "react";
import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { getContentLang } from "@/utils/content_lang";
import type { StudyListReq } from "@/category/study/types";

import { getStudyList } from "../study_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useStudyList = (params: StudyListReq) => {
  const { page, program, sort } = params;
  const lang = getContentLang();

  const query = useQuery({
    queryKey: ["studies", { page, program, sort, lang }],
    queryFn: () => getStudyList(params, lang),
    placeholderData: keepPreviousData,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
