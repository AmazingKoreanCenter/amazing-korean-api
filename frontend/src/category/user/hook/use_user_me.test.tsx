import { describe, expect, it } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { useUserMe } from "./use_user_me";

const renderWithQuery = () => {
  // QueryClient default retry: false 가 hook 의 retry 함수를 override 가능 →
  // 본 hook 의 retry 분기 검증용 = retry default 미설정.
  const client = new QueryClient();
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={client}>{children}</QueryClientProvider>
  );
  return renderHook(() => useUserMe(), { wrapper });
};

describe("useUserMe", () => {
  it("returns UserDetail on success", async () => {
    server.use(
      http.get("/api/users/me", () =>
        HttpResponse.json({ user_id: 42, email: "u@e.com" }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data?.user_id).toBe(42);
  });

  it("sets isError on 404 (retry: false branch)", async () => {
    // useUserMe.retry = ApiError 401/404 → false (즉시 fail). 그 외 → failureCount < 2.
    // 401 path = axios 인터셉터의 /auth/refresh 자동 호출과 충돌 → 404 사용.
    server.use(
      http.get(
        "/api/users/me",
        () => new HttpResponse(null, { status: 404 }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
  });

  it("retries on 500 (failureCount < 2 branch) then eventually errors", async () => {
    // 500 path = retry 2회 → 3번째 fail 시 isError. 본 test 는 retry 분기 cover 용.
    let attempts = 0;
    server.use(
      http.get("/api/users/me", () => {
        attempts += 1;
        return new HttpResponse(null, { status: 500 });
      }),
    );
    const { result } = renderWithQuery();
    await waitFor(
      () => expect(result.current.isError).toBe(true),
      { timeout: 5000 },
    );
    expect(attempts).toBeGreaterThanOrEqual(2);
  });
});
