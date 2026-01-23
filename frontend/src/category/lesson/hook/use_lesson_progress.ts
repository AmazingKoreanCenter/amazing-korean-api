import { useEffect } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { useAuthStore } from "@/hooks/use_auth_store";
import type { LessonProgressUpdateReq } from "@/category/lesson/types";

import { getLessonProgress, updateLessonProgress } from "../lesson_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useLessonProgress = (lessonId: number | undefined) => {
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);

  const query = useQuery({
    queryKey: ["lesson-progress", lessonId],
    queryFn: () => getLessonProgress(lessonId!),
    enabled: typeof lessonId === "number" && Number.isFinite(lessonId) && isLoggedIn,
  });

  useEffect(() => {
    if (query.isError) {
      const err = query.error;
      if (err instanceof ApiError && (err.status === 401 || err.status === 403)) {
        return;
      }
      toast.error(getErrorMessage(err));
    }
  }, [query.error, query.isError]);

  return query;
};

export const useUpdateLessonProgress = (lessonId: number) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: LessonProgressUpdateReq) => updateLessonProgress(lessonId, body),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["lesson-progress", lessonId] });
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};
