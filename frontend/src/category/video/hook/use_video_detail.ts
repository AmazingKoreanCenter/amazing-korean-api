import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { getContentLang } from "@/utils/content_lang";

import { getVideoDetail } from "../video_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useVideoDetail = (id: number) => {
  const lang = getContentLang();

  const query = useQuery({
    queryKey: ["videos", "detail", id, lang],
    queryFn: () => getVideoDetail(id, lang),
    enabled: Number.isFinite(id),
    retry: (failureCount, error) => {
      if (error instanceof ApiError && error.status === 404) {
        return false;
      }
      return failureCount < 2;
    },
    staleTime: 1000 * 60 * 5,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
