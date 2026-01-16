import { useEffect } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { ApiError } from "@/api/client";
import { useAuthStore } from "@/hooks/use_auth_store";
import type { VideoProgressUpdateReq } from "@/category/video/types";

import { getVideoProgress, updateVideoProgress } from "../video_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "Request failed";
};

export const useVideoProgress = (videoId?: number) => {
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
  const isEnabled =
    typeof videoId === "number" && Number.isFinite(videoId) && isLoggedIn;

  const query = useQuery({
    queryKey: ["video-progress", videoId],
    queryFn: () => getVideoProgress(videoId as number),
    enabled: isEnabled,
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

export const useUpdateVideoProgress = (videoId?: number) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (values: VideoProgressUpdateReq) => {
      if (typeof videoId !== "number" || !Number.isFinite(videoId)) {
        return Promise.reject(new Error("Invalid video id"));
      }

      return updateVideoProgress(videoId, values);
    },
    onSuccess: () => {
      if (typeof videoId !== "number" || !Number.isFinite(videoId)) {
        return;
      }

      void queryClient.invalidateQueries({
        queryKey: ["video-progress", videoId],
      });
    },
    onError: (error) => {
      console.error("[VideoProgress] Update failed:", getErrorMessage(error));
    },
  });
};
