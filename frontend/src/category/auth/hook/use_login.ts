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

const getErrorMessage = (error: unknown) => {
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

  return useMutation({
    mutationFn: (data: LoginReq) => login(data),
    onSuccess: (data) => {
      useAuthStore.getState().login(data);
      toast.success("로그인 성공!");
      navigate("/");
    },
    onError: (error) => {
      toast.error(getErrorMessage(error));
    },
  });
};
