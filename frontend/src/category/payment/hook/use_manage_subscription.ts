import { useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import { useTranslation } from "react-i18next";

import { cancelSubscription } from "../payment_api";
import type { CancelSubscriptionReq } from "../types";

const SUBSCRIPTION_KEY = ["payment", "subscription"] as const;

/** Webhook이 DB를 업데이트할 시간을 확보한 뒤 구독 상태를 refetch */
const invalidateWithDelay = (queryClient: ReturnType<typeof useQueryClient>) => {
  setTimeout(() => {
    void queryClient.invalidateQueries({ queryKey: [...SUBSCRIPTION_KEY] });
  }, 3000);
};

export const useCancelSubscription = () => {
  const queryClient = useQueryClient();
  const { t } = useTranslation();

  return useMutation({
    mutationFn: (data: CancelSubscriptionReq) => cancelSubscription(data),
    onSuccess: () => {
      invalidateWithDelay(queryClient);
      toast.success(t("payment.cancelSuccess"));
    },
    onError: () => {
      toast.error(t("payment.cancelFailed"));
    },
  });
};
