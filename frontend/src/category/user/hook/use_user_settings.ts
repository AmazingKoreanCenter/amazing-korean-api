import { useQuery } from "@tanstack/react-query";

import { ApiError } from "@/api/client";

import { getUserSettings } from "../user_api";

export const useUserSettings = (options?: { enabled?: boolean }) => {
  return useQuery({
    queryKey: ["user", "settings"],
    queryFn: getUserSettings,
    enabled: options?.enabled ?? true,
    staleTime: 5 * 60 * 1000,
    retry: (failureCount, error) => {
      if (error instanceof ApiError && [401, 404].includes(error.status)) {
        return false;
      }
      return failureCount < 2;
    },
  });
};
