import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { getContentLang } from "@/utils/content_lang";
import type { VideoListReq } from "@/category/video/types";

import { getVideoList } from "../video_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useVideoList = (params: VideoListReq) => {
  const lang = getContentLang();

  const query = useQuery({
    queryKey: ["videos", params, lang],
    queryFn: () => getVideoList(params, lang),
    staleTime: 1000 * 60 * 5,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
