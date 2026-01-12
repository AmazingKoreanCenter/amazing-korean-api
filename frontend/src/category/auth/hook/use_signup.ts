import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";

import { signup } from "../auth_api";
import type { SignupReq } from "@/category/user/types";
import { useAuthStore } from "@/hooks/use_auth_store";

type AxiosErrorShape = {
  response?: {
    data?: {
      error?: {
        message?: string;
      };
    };
  };
  message?: string;
};

const getErrorMessage = (error: unknown) => {
  if (typeof error === "object" && error) {
    const maybeAxios = error as AxiosErrorShape;
    const apiMessage = maybeAxios.response?.data?.error?.message;
    if (apiMessage) {
      return apiMessage;
    }
    if (typeof maybeAxios.message === "string" && maybeAxios.message) {
      return maybeAxios.message;
    }
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useSignup = () => {
  return useMutation({
    mutationFn: (data: SignupReq) => signup(data),
    onSuccess: (data) => {
      useAuthStore.getState().login(data);
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};
