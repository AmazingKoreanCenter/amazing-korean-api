import { describe, expect, it } from "vitest";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import {
  cancelSubscription,
  getPaymentPlans,
  getSubscription,
} from "./payment_api";
import { ApiError } from "@/api/client";

const samplePlan = {
  interval: "month_1" as const,
  months: 1,
  price_cents: 9900,
  price_display: "$9.90",
  price_id: "pri_001",
  trial_days: 7,
  label: "1 month",
};

describe("payment_api.getPaymentPlans", () => {
  it("returns the parsed PlansRes body on 200", async () => {
    server.use(
      http.get("/api/payment/plans", () =>
        HttpResponse.json({
          client_token: "ctk_abc",
          sandbox: true,
          plans: [samplePlan],
        }),
      ),
    );
    const res = await getPaymentPlans();
    expect(res.client_token).toBe("ctk_abc");
    expect(res.sandbox).toBe(true);
    expect(res.plans).toHaveLength(1);
    expect(res.plans[0].price_id).toBe("pri_001");
  });

  it("throws ApiError on 5xx error response", async () => {
    server.use(
      http.get(
        "/api/payment/plans",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    await expect(getPaymentPlans()).rejects.toBeInstanceOf(ApiError);
  });

  it("getSubscription returns parsed SubscriptionRes body on 200", async () => {
    server.use(
      http.get("/api/payment/subscription", () =>
        HttpResponse.json({
          subscription: {
            subscription_id: 42,
            status: "active",
            billing_interval: "month_1",
            current_price_cents: 9900,
            trial_ends_at: null,
            current_period_start: "2026-01-01",
            current_period_end: "2026-02-01",
            canceled_at: null,
            paused_at: null,
            created_at: "2026-01-01",
          },
        }),
      ),
    );
    const res = await getSubscription();
    expect(res.subscription?.subscription_id).toBe(42);
    expect(res.subscription?.status).toBe("active");
  });

  it("cancelSubscription POSTs the body and returns SubscriptionRes", async () => {
    let observedBody: unknown = null;
    server.use(
      http.post("/api/payment/subscription/cancel", async ({ request }) => {
        observedBody = await request.json();
        return HttpResponse.json({
          subscription: {
            subscription_id: 42,
            status: "canceled",
            billing_interval: "month_1",
            current_price_cents: 9900,
            trial_ends_at: null,
            current_period_start: "2026-01-01",
            current_period_end: "2026-02-01",
            canceled_at: "2026-01-15",
            paused_at: null,
            created_at: "2026-01-01",
          },
        });
      }),
    );
    const res = await cancelSubscription({ immediately: false });
    expect(observedBody).toEqual({ immediately: false });
    expect(res.subscription?.status).toBe("canceled");
  });

  it("propagates the server error envelope message in ApiError when provided", async () => {
    // 백엔드 envelope = `{ error: { message } }` (parse_error_message 규약)
    server.use(
      http.get("/api/payment/plans", () =>
        HttpResponse.json(
          { error: { message: "plans temporarily unavailable" } },
          { status: 503 },
        ),
      ),
    );
    try {
      await getPaymentPlans();
      throw new Error("expected ApiError");
    } catch (error) {
      expect(error).toBeInstanceOf(ApiError);
      const apiErr = error as ApiError;
      expect(apiErr.status).toBe(503);
      expect(apiErr.message).toContain("plans temporarily unavailable");
    }
  });
});
