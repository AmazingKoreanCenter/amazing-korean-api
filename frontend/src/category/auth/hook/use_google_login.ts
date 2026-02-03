import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";

import { getGoogleAuthUrl } from "../auth_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "Google 로그인을 시작할 수 없습니다. 잠시 후 다시 시도해주세요.";
};

export const useGoogleLogin = () => {
  return useMutation({
    mutationFn: () => getGoogleAuthUrl(),
    onSuccess: (data) => {
      // Google OAuth 페이지로 리다이렉트
      window.location.href = data.auth_url;
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};
