import { useCallback, useEffect, useRef } from "react";
import { initializePaddle, type Paddle, type CheckoutEventNames } from "@paddle/paddle-js";
import { toast } from "sonner";
import { useTranslation } from "react-i18next";

import { useAuthStore } from "@/hooks/use_auth_store";

interface UsePaddleOptions {
  clientToken: string;
  sandbox: boolean;
}

export const usePaddle = ({ clientToken, sandbox }: UsePaddleOptions) => {
  const { t } = useTranslation();
  const paddleRef = useRef<Paddle | null>(null);
  const initializedRef = useRef(false);

  useEffect(() => {
    if (initializedRef.current || !clientToken) return;
    initializedRef.current = true;

    initializePaddle({
      token: clientToken,
      environment: sandbox ? "sandbox" : "production",
      eventCallback: (event) => {
        if (event.name === "checkout.completed" as CheckoutEventNames) {
          toast.success(t("payment.checkoutSuccess"));
        }
      },
    }).then((paddle) => {
      if (paddle) {
        paddleRef.current = paddle;
      }
    });
  }, [clientToken, sandbox, t]);

  const openCheckout = useCallback(
    (priceId: string, discountCode?: string) => {
      const userId = useAuthStore.getState().user?.user_id;
      if (!userId) {
        toast.error(t("payment.loginRequired"));
        return;
      }

      const paddle = paddleRef.current;
      if (!paddle) {
        toast.error(t("payment.serviceUnavailable"));
        return;
      }

      paddle.Checkout.open({
        items: [{ priceId, quantity: 1 }],
        discountCode: discountCode || undefined,
        customData: { user_id: String(userId) },
      });
    },
    [t]
  );

  return { openCheckout, isReady: !!paddleRef.current };
};
