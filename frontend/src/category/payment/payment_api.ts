import { request } from "@/api/client";

import type { CancelSubscriptionReq, PlansRes, SubscriptionRes } from "./types";

export const getPaymentPlans = () =>
  request<PlansRes>("/payment/plans");

export const getSubscription = () =>
  request<SubscriptionRes>("/payment/subscription");

export const cancelSubscription = (data: CancelSubscriptionReq) =>
  request<SubscriptionRes>("/payment/subscription/cancel", { method: "POST", data });
