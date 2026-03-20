import { useQuery } from "@tanstack/react-query";

import { useAuthStore } from "@/hooks/use_auth_store";

import { getMyPurchases } from "../ebook_api";
import type { MyPurchasesRes } from "../types";

export const useMyPurchases = () => {
  const { isLoggedIn } = useAuthStore();

  return useQuery({
    queryKey: ["ebook", "my-purchases"],
    queryFn: getMyPurchases,
    enabled: isLoggedIn,
    refetchInterval: (query) => {
      const data = query.state.data as MyPurchasesRes | undefined;
      const hasPendingPaddle = data?.items.some(
        (p) => p.status === "pending" && p.payment_method === "paddle"
      );
      return hasPendingPaddle ? 5000 : false;
    },
  });
};
