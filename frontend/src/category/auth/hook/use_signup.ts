import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";

import i18n from "@/i18n";
import { ApiError } from "@/api/client";
import { signup } from "../auth_api";
import type { SignupReq } from "../types";
import { useAuthStore } from "@/hooks/use_auth_store";

export const useSignup = () => {
  return useMutation({
    mutationFn: (data: SignupReq) => signup(data),
    onSuccess: (data) => {
      // 회원가입 응답에 토큰이 포함되어 있다면 자동 로그인 처리
      if (data.access) {
        useAuthStore.getState().login(data as any);
      }
    },
    onError: (error) => {
      if (error instanceof ApiError) {
        toast.error(error.message);
      } else {
        toast.error(i18n.t("auth.toastSignupFailed"));
      }
    },
  });
};
