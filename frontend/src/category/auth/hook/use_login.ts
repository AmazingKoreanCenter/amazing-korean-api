import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import i18n from "@/i18n";
import { ApiError } from "@/api/client";
import type { LoginReq } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";

import { login } from "../auth_api";

const statusMessageMap: Record<number, string> = {
  400: "auth.statusBadRequest",
  401: "auth.statusUnauthorized",
  403: "auth.statusForbidden",
  423: "auth.statusLocked",
  429: "auth.statusTooMany",
  500: "auth.statusServerError",
};

export interface SocialOnlyError {
  isSocialOnly: true;
  providers: string[];
}

// 소셜 전용 계정 에러 파싱
const parseSocialOnlyError = (error: unknown): SocialOnlyError | null => {
  if (error instanceof ApiError && error.status === 401) {
    // 에러 메시지 형식: "AUTH_401_SOCIAL_ONLY_ACCOUNT:google,apple"
    if (error.message.startsWith("AUTH_401_SOCIAL_ONLY_ACCOUNT:")) {
      const providers = error.message
        .replace("AUTH_401_SOCIAL_ONLY_ACCOUNT:", "")
        .split(",")
        .filter(Boolean);
      return { isSocialOnly: true, providers };
    }
  }
  return null;
};

const getErrorMessage = (error: unknown) => {
  // 소셜 전용 계정 에러는 별도 처리 (toast 표시 안함)
  if (parseSocialOnlyError(error)) {
    return null;
  }

  if (error instanceof ApiError) {
    const key = statusMessageMap[error.status];
    return key ? i18n.t(key) : error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return i18n.t("common.requestFailed");
};

export const useLogin = () => {
  const navigate = useNavigate();

  const mutation = useMutation({
    mutationFn: (data: LoginReq) => login(data),
    onSuccess: (data) => {
      useAuthStore.getState().login(data);
      toast.success(i18n.t("auth.toastLoginSuccess"));
      navigate("/about");
    },
    onError: (error) => {
      const message = getErrorMessage(error);
      if (message) {
        toast.error(message);
      }
    },
  });

  // 소셜 전용 계정 에러 상태
  const socialOnlyError = mutation.error
    ? parseSocialOnlyError(mutation.error)
    : null;

  return {
    ...mutation,
    socialOnlyError,
  };
};
