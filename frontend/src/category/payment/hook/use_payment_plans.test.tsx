import { describe, expect, it } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { usePaymentPlans } from "./use_payment_plans";

const samplePlan = {
  interval: "month_1" as const,
  months: 1,
  price_cents: 9900,
  price_display: "$9.90",
  price_id: "pri_001",
  trial_days: 7,
  label: "1 month",
};

const renderWithQuery = () => {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={client}>{children}</QueryClientProvider>
  );
  return renderHook(() => usePaymentPlans(), { wrapper });
};

describe("usePaymentPlans", () => {
  it("returns isLoading initially then success with plans data", async () => {
    server.use(
      http.get("/api/payment/plans", () =>
        HttpResponse.json({
          client_token: "ctk_abc",
          sandbox: true,
          plans: [samplePlan],
        }),
      ),
    );
    const { result } = renderWithQuery();
    expect(result.current.isLoading).toBe(true);
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data?.client_token).toBe("ctk_abc");
    expect(result.current.data?.plans).toHaveLength(1);
  });

  it("sets isError when the request fails", async () => {
    server.use(
      http.get(
        "/api/payment/plans",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    expect(result.current.data).toBeUndefined();
  });
});
