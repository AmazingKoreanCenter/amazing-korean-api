import { useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { SubmitAnswerReq } from "@/category/study/types";

import { submitAnswer } from "../study_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "제출에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useSubmitAnswer = (taskId: number) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (body: SubmitAnswerReq) => submitAnswer(taskId, body),
    onSuccess: (data) => {
      if (data.is_correct) {
        toast.success("정답입니다!");
      } else {
        toast.error("오답입니다.");
      }
      queryClient.invalidateQueries({ queryKey: ["task-status", taskId] });
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};
