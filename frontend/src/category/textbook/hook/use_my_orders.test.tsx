import { describe, expect, it } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { useMyTextbookOrders } from "./use_my_orders";

const sampleOrder = {
  order_id: 1,
  order_code: "AKB-260511-0001",
  status: "pending",
  orderer_name: "홍길동",
  orderer_email: null,
  orderer_phone: "010",
  org_name: null,
  org_type: null,
  delivery_postal_code: null,
  delivery_address: "x",
  delivery_detail: null,
  payment_method: "bank_transfer",
  depositor_name: null,
  tax_invoice: false,
  tax_biz_number: null,
  tax_company_name: null,
  tax_rep_name: null,
  tax_address: null,
  tax_biz_type: null,
  tax_biz_item: null,
  tax_email: null,
  total_quantity: 1,
  total_price: 25000,
  items: [],
  created_at: "2026-05-11T00:00:00Z",
};

const renderWithQuery = () => {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={client}>{children}</QueryClientProvider>
  );
  return renderHook(() => useMyTextbookOrders(), { wrapper });
};

describe("useMyTextbookOrders", () => {
  it("transitions isLoading → isSuccess with orders array", async () => {
    server.use(
      http.get("/api/textbook/my", () =>
        HttpResponse.json({ orders: [sampleOrder] }),
      ),
    );
    const { result } = renderWithQuery();
    expect(result.current.isLoading).toBe(true);
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data?.orders).toHaveLength(1);
    expect(result.current.data?.orders[0].order_code).toBe("AKB-260511-0001");
  });

  it("sets isError when the request fails", async () => {
    server.use(
      http.get(
        "/api/textbook/my",
        () => new HttpResponse(null, { status: 401 }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    expect(result.current.data).toBeUndefined();
  });
});
