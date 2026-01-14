import { useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import type { SettingsUpdateReq } from "@/category/user/types";

import { updateUserSettings } from "../user_api";

type UseUpdateSettingsOptions = {
  onSuccess?: (values: SettingsUpdateReq) => void;
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

export const useUpdateSettings = (options: UseUpdateSettingsOptions = {}) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (values: SettingsUpdateReq) => updateUserSettings(values),
    onSuccess: (_data, variables) => {
      void queryClient.invalidateQueries({ queryKey: ["user", "settings"] });
      toast.success("설정이 저장되었습니다.");
      options.onSuccess?.(variables);
    },
    onError: (error) => {
      toast.error(`설정 저장 실패: ${getErrorMessage(error)}`);
    },
  });
};
