import { describe, expect, it } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { useSendTestEmail } from "./use_admin_email";

const renderWithQuery = () => {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={client}>{children}</QueryClientProvider>
  );
  return renderHook(() => useSendTestEmail(), { wrapper });
};

describe("useSendTestEmail", () => {
  it("returns success state after mutate", async () => {
    server.use(
      http.post("/api/admin/email/test", () =>
        HttpResponse.json({ sent: true }),
      ),
    );
    const { result } = renderWithQuery();
    result.current.mutate({
      to_email: "test@example.com",
      template: "welcome",
    } as Parameters<typeof result.current.mutate>[0]);
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
  });

  it("returns error state on failure", async () => {
    server.use(
      http.post(
        "/api/admin/email/test",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    const { result } = renderWithQuery();
    result.current.mutate({
      to_email: "test@example.com",
      template: "welcome",
    } as Parameters<typeof result.current.mutate>[0]);
    await waitFor(() => expect(result.current.isError).toBe(true));
  });
});
