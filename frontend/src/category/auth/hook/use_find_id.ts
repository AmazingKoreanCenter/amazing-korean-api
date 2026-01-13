import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { FindIdReq } from "@/category/auth/types";

import { findId } from "../auth_api";

const successMessage = "If the account exists, an email has been sent.";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useFindId = () => {
  return useMutation({
    mutationFn: (data: FindIdReq) => findId(data),
    onSuccess: () => {
      toast.success(successMessage);
    },
    onError: (error) => {
      if (error instanceof ApiError && error.status === 429) {
        toast.warning("너무 많은 시도가 감지되었습니다. 잠시 후 다시 시도해주세요.");
        return;
      }

      toast.error(getErrorMessage(error));
    },
  });
};
