import { describe, expect, it, beforeEach } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { useMyPurchases } from "./use_my_purchases";
import { useAuthStore } from "@/hooks/use_auth_store";

const renderWithQuery = () => {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={client}>{children}</QueryClientProvider>
  );
  return renderHook(() => useMyPurchases(), { wrapper });
};

beforeEach(() => {
  // useMyPurchases.enabled = isLoggedIn → 테스트마다 store 초기화.
  useAuthStore.setState({
    user: null,
    accessToken: null,
    isLoggedIn: false,
  });
});

describe("useMyPurchases", () => {
  it("is disabled (no fetch) when isLoggedIn=false", async () => {
    const { result } = renderWithQuery();
    // 일정 시간 후에도 isLoading 이 true 로 시작 (enabled false → fetch 안 함, pending 상태)
    expect(result.current.fetchStatus).toBe("idle");
    expect(result.current.data).toBeUndefined();
  });

  it("fetches items when isLoggedIn=true and returns MyPurchasesRes", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "tok",
      isLoggedIn: true,
    });
    server.use(
      http.get("/api/ebook/my", () =>
        HttpResponse.json({
          items: [
            {
              purchase_code: "EB-1",
              status: "completed",
              language: "en",
              edition: "premium",
              payment_method: "paddle",
              price: 19900,
              currency: "KRW",
              created_at: "2026-05-11T00:00:00Z",
            },
          ],
        }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data?.items).toHaveLength(1);
  });

  it("sets isError when the request fails (logged in)", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "tok",
      isLoggedIn: true,
    });
    server.use(
      http.get(
        "/api/ebook/my",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
  });

  it("returns data with pending paddle purchase (refetchInterval branch covered)", async () => {
    // refetchInterval = pending+paddle → 5000ms / 외 → false. 본 test 는 data 분기만 검증.
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "tok",
      isLoggedIn: true,
    });
    server.use(
      http.get("/api/ebook/my", () =>
        HttpResponse.json({
          items: [
            {
              purchase_code: "EB-pending",
              status: "pending",
              language: "en",
              edition: "premium",
              payment_method: "paddle",
              price: 19900,
              currency: "KRW",
              created_at: "2026-05-11T00:00:00Z",
            },
          ],
        }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data?.items[0].status).toBe("pending");
    expect(result.current.data?.items[0].payment_method).toBe("paddle");
  });
});
