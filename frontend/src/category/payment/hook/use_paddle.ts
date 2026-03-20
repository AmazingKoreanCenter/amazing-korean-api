import { useCallback, useEffect, useRef } from "react";
import { initializePaddle, type Paddle, type CheckoutEventNames } from "@paddle/paddle-js";
import { toast } from "sonner";
import { useTranslation } from "react-i18next";

import { useAuthStore } from "@/hooks/use_auth_store";

interface UsePaddleOptions {
  clientToken: string;
  sandbox: boolean;
  email?: string;
  onCheckoutComplete?: () => void;
}

export const usePaddle = ({ clientToken, sandbox, email, onCheckoutComplete }: UsePaddleOptions) => {
  const { t } = useTranslation();
  const paddleRef = useRef<Paddle | null>(null);
  const initializedRef = useRef(false);
  const onCheckoutCompleteRef = useRef(onCheckoutComplete);
  onCheckoutCompleteRef.current = onCheckoutComplete;

  useEffect(() => {
    if (initializedRef.current || !clientToken) return;
    initializedRef.current = true;

    initializePaddle({
      token: clientToken,
      environment: sandbox ? "sandbox" : "production",
      ...(email ? { pwCustomer: { email } } : {}),
      eventCallback: (event) => {
        if (event.name === "checkout.completed" as CheckoutEventNames) {
          toast.success(t("payment.checkoutSuccess"));
          onCheckoutCompleteRef.current?.();
        }
      },
    }).then((paddle) => {
      if (paddle) {
        paddleRef.current = paddle;
      }
    });
  }, [clientToken, sandbox, t]);

  // Retain용: 이메일이 초기화 후 로드되면 pwCustomer 업데이트
  useEffect(() => {
    if (email && paddleRef.current) {
      paddleRef.current.Update({ pwCustomer: { email } });
    }
  }, [email]);

  const openCheckout = useCallback(
    (priceId: string, options?: { discountId?: string; discountCode?: string }) => {
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

      // discountId와 discountCode는 상호 배타적 (Paddle.js 타입 제약)
      const discountOpts = options?.discountId
        ? { discountId: options.discountId }
        : options?.discountCode
          ? { discountCode: options.discountCode }
          : {};

      paddle.Checkout.open({
        items: [{ priceId, quantity: 1 }],
        ...discountOpts,
        customData: { user_id: String(userId) },
      });
    },
    [t]
  );

  const openEbookCheckout = useCallback(
    (priceId: string, purchaseCode: string) => {
      const paddle = paddleRef.current;
      if (!paddle) {
        toast.error(t("payment.serviceUnavailable"));
        return;
      }

      paddle.Checkout.open({
        items: [{ priceId, quantity: 1 }],
        customData: { type: "ebook", purchase_code: purchaseCode },
      });
    },
    [t]
  );

  return { openCheckout, openEbookCheckout, isReady: !!paddleRef.current };
};
