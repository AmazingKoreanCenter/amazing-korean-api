import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { ResetPasswordReq } from "@/category/auth/types"; // ✅ 올바른 경로

import { resetPassword } from "../auth_api";

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useResetPassword = () => {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: (data: ResetPasswordReq) => resetPassword(data),
    onSuccess: () => {
      toast.success("비밀번호가 성공적으로 변경되었습니다.");
      navigate("/login", { replace: true });
    },
    onError: (error) => {
      // 401: 토큰 만료 등 -> 홈으로 튕겨내기
      if (error instanceof ApiError && error.status === 401) {
        toast.error("유효하지 않거나 만료된 링크입니다.");
        navigate("/", { replace: true });
        return;
      }

      // 422 등 기타 에러: 침묵하지 말고 메시지 표시 ✅
      toast.error(getErrorMessage(error));
    },
  });
};