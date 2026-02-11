import { useEffect } from "react";
import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { getContentLang } from "@/utils/content_lang";
import type { LessonListReq } from "@/category/lesson/types";

import { getLessonList } from "../lesson_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useLessonList = (params: LessonListReq) => {
  const { page, sort } = params;
  const lang = getContentLang();

  const query = useQuery({
    queryKey: ["lessons", { page, sort, lang }],
    queryFn: () => getLessonList(params, lang),
    placeholderData: keepPreviousData,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
