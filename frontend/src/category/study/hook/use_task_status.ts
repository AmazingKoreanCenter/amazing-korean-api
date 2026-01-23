import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { useAuthStore } from "@/hooks/use_auth_store";

import { getTaskStatus } from "../study_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useTaskStatus = (taskId: number | undefined) => {
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);

  const query = useQuery({
    queryKey: ["task-status", taskId],
    queryFn: () => getTaskStatus(taskId!),
    enabled: typeof taskId === "number" && Number.isFinite(taskId) && isLoggedIn,
  });

  useEffect(() => {
    if (query.isError) {
      const err = query.error;
      // 401/403 에러는 조용히 무시 (로그인 필요 또는 권한 없음)
      if (err instanceof ApiError && (err.status === 401 || err.status === 403)) {
        return;
      }
      toast.error(getErrorMessage(err));
    }
  }, [query.error, query.isError]);

  return query;
};
