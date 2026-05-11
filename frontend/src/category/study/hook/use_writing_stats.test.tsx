import { describe, expect, it, vi } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { useWritingStats } from "./use_writing_stats";
import * as studyApi from "../study_api";

const toastError = vi.fn();
vi.mock("sonner", () => ({
  toast: { error: (...args: unknown[]) => toastError(...args) },
}));

const renderWithQuery = () => {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={client}>{children}</QueryClientProvider>
  );
  return renderHook(() => useWritingStats({ days: 7 }), { wrapper });
};

describe("useWritingStats", () => {
  it("transitions isLoading → isSuccess with stats data", async () => {
    server.use(
      http.get("/api/studies/writing/stats", () =>
        HttpResponse.json({
          total_sessions: 10,
          avg_accuracy: 95,
          avg_cpm: 250,
        }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data?.total_sessions).toBe(10);
  });

  it("calls toast.error with ApiError message on 5xx (ApiError branch)", async () => {
    toastError.mockClear();
    server.use(
      http.get(
        "/api/studies/writing/stats",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    await waitFor(() => expect(toastError).toHaveBeenCalled());
  });

  it("calls toast.error with Error.message on generic Error throw (Error branch)", async () => {
    toastError.mockClear();
    vi.spyOn(studyApi, "getWritingStats").mockRejectedValueOnce(
      new Error("stats error"),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    await waitFor(() =>
      expect(toastError).toHaveBeenCalledWith("stats error"),
    );
  });

  it("calls toast.error with fallback on non-Error throw (fallback branch)", async () => {
    toastError.mockClear();
    vi.spyOn(studyApi, "getWritingStats").mockRejectedValueOnce(
      "not an Error instance",
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    await waitFor(() =>
      expect(toastError).toHaveBeenCalledWith(
        "통계를 불러오지 못했습니다. 잠시 후 다시 시도해주세요.",
      ),
    );
  });
});
