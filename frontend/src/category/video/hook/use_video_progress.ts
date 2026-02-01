import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { useAuthStore } from "@/hooks/use_auth_store";
import type { VideoProgressUpdateReq } from "@/category/video/types";

import { getVideoProgress, updateVideoProgress } from "../video_api";

export const useVideoProgress = (videoId?: number) => {
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
  const isEnabled =
    typeof videoId === "number" && Number.isFinite(videoId) && isLoggedIn;

  return useQuery({
    queryKey: ["video-progress", videoId],
    queryFn: () => getVideoProgress(videoId as number),
    enabled: isEnabled,
    retry: 1,
    staleTime: 0,
  });
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
    onError: () => {
      // Error handled by React Query - toast notification can be added if needed
    },
  });
};
