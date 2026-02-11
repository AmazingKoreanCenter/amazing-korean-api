import { useEffect } from "react";
import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { getContentLang } from "@/utils/content_lang";
import type { StudyDetailReq } from "@/category/study/types";

import { getStudyDetail } from "../study_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useStudyDetail = (studyId: number | undefined, params: StudyDetailReq = {}) => {
  const { page } = params;
  const lang = getContentLang();

  const query = useQuery({
    queryKey: ["study", studyId, { page, lang }],
    queryFn: () => getStudyDetail(studyId!, params, lang),
    enabled: typeof studyId === "number" && Number.isFinite(studyId),
    placeholderData: keepPreviousData,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
