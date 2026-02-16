import { useQuery } from "@tanstack/react-query";

import { getPaymentPlans } from "../payment_api";

export const usePaymentPlans = () => {
  return useQuery({
    queryKey: ["payment", "plans"],
    queryFn: getPaymentPlans,
    staleTime: 5 * 60 * 1000, // 5분 캐시 (플랜은 자주 변경되지 않음)
  });
};
