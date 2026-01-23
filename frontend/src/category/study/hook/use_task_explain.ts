import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import { useAuthStore } from "@/hooks/use_auth_store";

import { getTaskExplain } from "../study_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    if (error.status === 403) {
      return "해설을 보려면 먼저 문제를 풀어야 합니다.";
    }
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useTaskExplain = (taskId: number | undefined, enabled: boolean = false) => {
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);

  const query = useQuery({
    queryKey: ["task-explain", taskId],
    queryFn: () => getTaskExplain(taskId!),
    enabled: typeof taskId === "number" && Number.isFinite(taskId) && isLoggedIn && enabled,
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
