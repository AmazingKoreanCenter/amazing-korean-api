import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { UpdateUserReq } from "@/category/user/types";

import { updateUserMe } from "../user_api";

type UseUpdateUserOptions = {
  onConflict?: () => void;
};

const getErrorMessage = (error: unknown) => {
  if (error instanceof ApiError) {
    return error.message;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return "요청에 실패했습니다. 잠시 후 다시 시도해주세요.";
};

export const useUpdateUser = (options: UseUpdateUserOptions = {}) => {
  const queryClient = useQueryClient();
  const navigate = useNavigate();

  return useMutation({
    mutationFn: (data: UpdateUserReq) => updateUserMe(data),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["user", "me"] });
      toast.success("정보가 수정되었습니다");
      navigate("/user/me");
    },
    onError: (error) => {
      if (error instanceof ApiError && error.status === 409) {
        options.onConflict?.();
        return;
      }

      toast.error(getErrorMessage(error));
    },
  });
};
