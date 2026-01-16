import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";

import { ApiError } from "@/api/client";
import { useAuthStore } from "@/hooks/use_auth_store";

import { getVideoProgress } from "../video_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "Request failed";
};

export const useVideoProgress = (videoId?: string) => {
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);

  const query = useQuery({
    queryKey: ["video-progress", videoId],
    queryFn: () => getVideoProgress(videoId ?? ""),
    enabled: !!videoId && isLoggedIn,
    retry: 1,
    staleTime: 0,
  });

  useEffect(() => {
    if (!query.isError) {
      return;
    }

    console.warn("[VideoProgress] Failed to load:", getErrorMessage(query.error));
  }, [query.error, query.isError]);

  return query;
};
