import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";

import i18n from "@/i18n";
import { ApiError } from "@/api/client";
import { signup } from "../auth_api";
import type { SignupReq } from "../types";

export const useSignup = () => {
  return useMutation({
    mutationFn: (data: SignupReq) => signup(data),
    onError: (error) => {
      if (error instanceof ApiError) {
        toast.error(error.message);
      } else {
        toast.error(i18n.t("auth.toastSignupFailed"));
      }
    },
  });
};
