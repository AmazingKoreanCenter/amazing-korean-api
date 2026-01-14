import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import { useAuthStore } from "@/hooks/use_auth_store";

import { logout } from "../auth_api";

export const useLogout = () => {
  const navigate = useNavigate();

  return useMutation({
    // 🚨 [수정됨] logout()은 이제 인자를 받지 않습니다.
    // (헤더 처리는 client.ts의 interceptor가 담당합니다)
    mutationFn: () => logout(),
    onSettled: () => {
      useAuthStore.getState().logout();
      toast.success("로그아웃 되었습니다.");
      navigate("/login");
    },
  });
};