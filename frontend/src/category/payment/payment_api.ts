import { request } from "@/api/client";

import type { CancelSubscriptionReq, PlansRes, SubscriptionRes } from "./types";

export const getPaymentPlans = () =>
  request<PlansRes>("/payment/plans");

export const getSubscription = () =>
  request<SubscriptionRes>("/payment/subscription");

export const cancelSubscription = (data: CancelSubscriptionReq) =>
  request<SubscriptionRes>("/payment/subscription/cancel", { method: "POST", data });

export const pauseSubscription = () =>
  request<SubscriptionRes>("/payment/subscription/pause", { method: "POST" });

export const resumeSubscription = () =>
  request<SubscriptionRes>("/payment/subscription/resume", { method: "POST" });
