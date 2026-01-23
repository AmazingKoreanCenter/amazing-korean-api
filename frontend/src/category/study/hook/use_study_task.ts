import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";

import { getStudyTask } from "../study_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useStudyTask = (taskId: number | undefined) => {
  const query = useQuery({
    queryKey: ["study-task", taskId],
    queryFn: () => getStudyTask(taskId!),
    enabled: typeof taskId === "number" && Number.isFinite(taskId),
  });

  useEffect(() => {
    if (query.isError) {
      toast.error(getErrorMessage(query.error));
    }
  }, [query.error, query.isError]);

  return query;
};
