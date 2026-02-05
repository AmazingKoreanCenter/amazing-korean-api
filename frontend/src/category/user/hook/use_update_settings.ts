import { useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import { ApiError } from "@/api/client";
import i18n from "@/i18n";
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

  return i18n.t("common.requestFailed");
};

export const useUpdateSettings = (options: UseUpdateSettingsOptions = {}) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (values: SettingsUpdateReq) => updateUserSettings(values),
    onSuccess: (_data, variables) => {
      void queryClient.invalidateQueries({ queryKey: ["user", "settings"] });
      toast.success(i18n.t("user.toastSettingsSaved"));
      options.onSuccess?.(variables);
    },
    onError: (error) => {
      toast.error(i18n.t("user.toastSettingsSaveFailed", { message: getErrorMessage(error) }));
    },
  });
};
