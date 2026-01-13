import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import { useAuthStore } from "@/hooks/use_auth_store";

import { logout } from "../auth_api";

export const useLogout = () => {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: () => logout(useAuthStore.getState().accessToken),
    onSettled: () => {
      useAuthStore.getState().logout();
      toast.success("로그아웃 되었습니다.");
      navigate("/login");
    },
  });
};
