import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import i18n from "@/i18n";
import { useAuthStore } from "@/hooks/use_auth_store";

import { logout } from "../auth_api";

export const useLogout = () => {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: () => logout(),
    onSuccess: () => {
      useAuthStore.getState().logout();
      toast.success(i18n.t("auth.toastLogoutSuccess"));
      navigate("/login");
    },
    onError: () => {
      // API 실패해도 로컬 상태는 정리 (보안상 로그아웃 유지)
      useAuthStore.getState().logout();
      toast.warning(i18n.t("auth.toastLogoutPartial"));
      navigate("/login");
    },
  });
};
