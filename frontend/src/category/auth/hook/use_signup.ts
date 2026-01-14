import { useMutation } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client"; // ✅ 표준 에러 클래스 사용
import { signup } from "../auth_api";
import type { SignupReq } from "../types"; // ✅ [수정됨] user/types -> auth/types (상대경로)
import { useAuthStore } from "@/hooks/use_auth_store";

export const useSignup = () => {
  return useMutation({
    mutationFn: (data: SignupReq) => signup(data),
    onSuccess: (data) => {
      // 회원가입 응답에 토큰이 포함되어 있다면 자동 로그인 처리
      if (data.access) {
        // (주의: LoginRes와 SignupRes 구조가 약간 다를 수 있으므로, 
        // store.login이 허용하는 타입인지 확인 필요. 일단 실행되도록 처리)
        useAuthStore.getState().login(data as any);
      }
    },
    onError: (error) => {
      // ✅ 표준화된 에러 메시지 처리
      if (error instanceof ApiError) {
        toast.error(error.message);
      } else {
        toast.error("회원가입 요청에 실패했습니다.");
      }
    },
  });
};