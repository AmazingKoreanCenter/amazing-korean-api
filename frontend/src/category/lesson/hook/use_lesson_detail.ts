import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { getContentLang } from "@/utils/content_lang";

import { getLessonDetail } from "../lesson_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useLessonDetail = (lessonId: number | undefined) => {
  const lang = getContentLang();

  const query = useQuery({
    queryKey: ["lesson", lessonId, lang],
    queryFn: () => getLessonDetail(lessonId!, lang),
    enabled: typeof lessonId === "number" && Number.isFinite(lessonId),
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
