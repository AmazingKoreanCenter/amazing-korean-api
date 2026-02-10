import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import i18n from "@/i18n";
import { ApiError } from "@/api/client";
import type { ResetPasswordReq } from "@/category/auth/types";

import { useAuthStore } from "@/hooks/use_auth_store";
import { resetPassword } from "../auth_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return i18n.t("common.requestFailed");
};

export const useResetPassword = () => {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: (data: ResetPasswordReq) => resetPassword(data),
    onSuccess: () => {
      useAuthStore.getState().logout();
      toast.success(i18n.t("auth.toastResetPasswordSuccess"));
      navigate("/login", { replace: true });
    },
    onError: (error) => {
      // 401: 토큰 만료 등 -> 홈으로 튕겨내기
      if (error instanceof ApiError && error.status === 401) {
        toast.error(i18n.t("auth.toastInvalidLink"));
        navigate("/", { replace: true });
        return;
      }

      // 422 등 기타 에러: 메시지 표시
      toast.error(getErrorMessage(error));
    },
  });
};
