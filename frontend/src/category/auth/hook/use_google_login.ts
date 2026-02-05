import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";

import i18n from "@/i18n";
import { ApiError } from "@/api/client";

import { getGoogleAuthUrl } from "../auth_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return i18n.t("auth.toastGoogleLoginUnavailable");
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
