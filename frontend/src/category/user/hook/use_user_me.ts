import { useQuery } from "@tanstack/react-query";

import { ApiError } from "@/api/client";

import { getUserMe } from "../user_api";

export const useUserMe = () => {
  return useQuery({
    queryKey: ["user", "me"],
    queryFn: getUserMe,
    retry: (failureCount, error) => {
      if (error instanceof ApiError && [401, 404].includes(error.status)) {
        return false;
      }
      return failureCount < 2;
    },
  });
};
