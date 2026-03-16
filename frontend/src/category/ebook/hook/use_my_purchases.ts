import { useQuery } from "@tanstack/react-query";

import { useAuthStore } from "@/hooks/use_auth_store";

import { getMyPurchases } from "../ebook_api";

export const useMyPurchases = () => {
  const { isLoggedIn } = useAuthStore();

  return useQuery({
    queryKey: ["ebook", "my-purchases"],
    queryFn: getMyPurchases,
    enabled: isLoggedIn,
  });
};
