import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";

import i18n from "@/i18n";
import { ApiError } from "@/api/client";
import type { FindIdReq } from "@/category/auth/types";

import { findId } from "../auth_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return i18n.t("common.requestFailed");
};

export const useFindId = () => {
  return useMutation({
    mutationFn: (data: FindIdReq) => findId(data),
    onSuccess: () => {
      toast.success(i18n.t("auth.toastFindIdSuccess"));
    },
    onError: (error) => {
      if (error instanceof ApiError && error.status === 429) {
        toast.warning(i18n.t("auth.toastTooManyAttempts"));
        return;
      }

      toast.error(getErrorMessage(error));
    },
  });
};
