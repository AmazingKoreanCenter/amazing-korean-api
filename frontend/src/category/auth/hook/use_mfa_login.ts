import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import i18n from "@/i18n";
import { ApiError } from "@/api/client";
import type { MfaLoginReq } from "@/category/auth/types";
import { useAuthStore } from "@/hooks/use_auth_store";

import { mfaLogin } from "../auth_api";

export const useMfaLogin = () => {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: (data: MfaLoginReq) => mfaLogin(data),
    onSuccess: (data) => {
      useAuthStore.getState().login(data);
      toast.success(i18n.t("auth.toastLoginSuccess"));
      navigate("/about");
    },
    onError: (error) => {
      if (error instanceof ApiError) {
        if (error.status === 429) {
          toast.error(i18n.t("mfa.errorTooManyAttempts"));
        } else if (error.message === "MFA_TOKEN_EXPIRED") {
          toast.error(i18n.t("mfa.errorExpiredToken"));
        } else {
          toast.error(i18n.t("mfa.errorInvalidCode"));
        }
      } else {
        toast.error(i18n.t("common.requestFailed"));
      }
    },
  });
};
