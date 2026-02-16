import { useMutation, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import { useTranslation } from "react-i18next";

import { cancelSubscription, pauseSubscription, resumeSubscription } from "../payment_api";
import type { CancelSubscriptionReq } from "../types";

export const useCancelSubscription = () => {
  const queryClient = useQueryClient();
  const { t } = useTranslation();

  return useMutation({
    mutationFn: (data: CancelSubscriptionReq) => cancelSubscription(data),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["payment", "subscription"] });
      toast.success(t("payment.cancelSuccess"));
    },
    onError: () => {
      toast.error(t("payment.cancelFailed"));
    },
  });
};

export const usePauseSubscription = () => {
  const queryClient = useQueryClient();
  const { t } = useTranslation();

  return useMutation({
    mutationFn: pauseSubscription,
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["payment", "subscription"] });
      toast.success(t("payment.pauseSuccess"));
    },
    onError: () => {
      toast.error(t("payment.pauseFailed"));
    },
  });
};

export const useResumeSubscription = () => {
  const queryClient = useQueryClient();
  const { t } = useTranslation();

  return useMutation({
    mutationFn: resumeSubscription,
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: ["payment", "subscription"] });
      toast.success(t("payment.resumeSuccess"));
    },
    onError: () => {
      toast.error(t("payment.resumeFailed"));
    },
  });
};
