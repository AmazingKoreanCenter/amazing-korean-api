import { z } from "zod";

// =============================================================================
// GET /payment/plans — 구독 플랜 목록
// =============================================================================

export const billingIntervalSchema = z.enum([
  "month_1",
  "month_3",
  "month_6",
  "month_12",
]);
export type BillingInterval = z.infer<typeof billingIntervalSchema>;

export const planInfoSchema = z.object({
  interval: billingIntervalSchema,
  months: z.number().int(),
  price_cents: z.number().int(),
  price_display: z.string(),
  price_id: z.string(),
  trial_days: z.number().int(),
  label: z.string(),
});
export type PlanInfo = z.infer<typeof planInfoSchema>;

export const plansResSchema = z.object({
  client_token: z.string(),
  sandbox: z.boolean(),
  plans: z.array(planInfoSchema),
});
export type PlansRes = z.infer<typeof plansResSchema>;

// =============================================================================
// GET /payment/subscription — 현재 사용자 구독 상태
// =============================================================================

export const subscriptionStatusSchema = z.enum([
  "trialing",
  "active",
  "past_due",
  "paused",
  "canceled",
]);
export type SubscriptionStatus = z.infer<typeof subscriptionStatusSchema>;

export const subscriptionInfoSchema = z.object({
  subscription_id: z.number().int(),
  status: subscriptionStatusSchema,
  billing_interval: billingIntervalSchema,
  current_price_cents: z.number().int(),
  trial_ends_at: z.string().nullable(),
  current_period_start: z.string().nullable(),
  current_period_end: z.string().nullable(),
  canceled_at: z.string().nullable(),
  paused_at: z.string().nullable(),
  created_at: z.string(),
});
export type SubscriptionInfo = z.infer<typeof subscriptionInfoSchema>;

export const subscriptionResSchema = z.object({
  subscription: subscriptionInfoSchema.nullable(),
});
export type SubscriptionRes = z.infer<typeof subscriptionResSchema>;

// =============================================================================
// POST /payment/subscription/cancel — 구독 취소 요청
// =============================================================================

export interface CancelSubscriptionReq {
  immediately: boolean;
}
