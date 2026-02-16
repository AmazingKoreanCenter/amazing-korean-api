import { useQuery } from "@tanstack/react-query";

import { ApiError } from "@/api/client";
import { useAuthStore } from "@/hooks/use_auth_store";

import { getSubscription } from "../payment_api";

export const useSubscription = () => {
  const isLoggedIn = useAuthStore((s) => s.isLoggedIn);

  return useQuery({
    queryKey: ["payment", "subscription"],
    queryFn: getSubscription,
    enabled: isLoggedIn,
    retry: (failureCount, error) => {
      if (error instanceof ApiError && [401, 404].includes(error.status)) {
        return false;
      }
      return failureCount < 2;
    },
  });
};
