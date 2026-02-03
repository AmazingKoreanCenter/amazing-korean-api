import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { LoginReq } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";

import { login } from "../auth_api";

const statusMessageMap: Record<number, string> = {
  400: "입력 형식을 확인해주세요.",
  401: "이메일 또는 비밀번호가 일치하지 않습니다.",
  403: "접근이 차단된 계정입니다. 관리자에게 문의하세요.",
  423: "접근이 차단된 계정입니다. 관리자에게 문의하세요.",
  429: "너무 많은 시도가 있었습니다. 잠시 후 다시 시도해주세요.",
  500: "서버 오류가 발생했습니다.",
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
    return statusMessageMap[error.status] ?? error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useLogin = () => {
  const navigate = useNavigate();

  const mutation = useMutation({
    mutationFn: (data: LoginReq) => login(data),
    onSuccess: (data) => {
      useAuthStore.getState().login(data);
      toast.success("로그인 성공!");
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
