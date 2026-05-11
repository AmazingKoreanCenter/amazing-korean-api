import { describe, expect, it, vi } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { useVideoList } from "./use_video_list";
import * as videoApi from "../video_api";

const toastError = vi.fn();
vi.mock("sonner", () => ({
  toast: { error: (...args: unknown[]) => toastError(...args) },
}));

const renderWithQuery = (
  params: Parameters<typeof useVideoList>[0] = { page: 1 } as Parameters<typeof useVideoList>[0],
) => {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={client}>{children}</QueryClientProvider>
  );
  return renderHook(() => useVideoList(params), { wrapper });
};

describe("useVideoList", () => {
  it("returns list data on success", async () => {
    server.use(
      http.get("/api/videos", () =>
        HttpResponse.json({ items: [{ video_id: 1 }], total: 1 }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data?.total).toBe(1);
  });

  it("toasts ApiError message and isError (ApiError branch)", async () => {
    toastError.mockClear();
    server.use(
      http.get(
        "/api/videos",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    await waitFor(() => expect(toastError).toHaveBeenCalled());
  });

  it("toasts Error.message on generic Error throw (Error branch)", async () => {
    toastError.mockClear();
    vi.spyOn(videoApi, "getVideoList").mockRejectedValueOnce(
      new Error("video error"),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    await waitFor(() =>
      expect(toastError).toHaveBeenCalledWith("video error"),
    );
  });

  it("toasts fallback on non-Error throw (fallback branch)", async () => {
    toastError.mockClear();
    vi.spyOn(videoApi, "getVideoList").mockRejectedValueOnce("not error");
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    await waitFor(() =>
      expect(toastError).toHaveBeenCalledWith(
        "요청에 실패했습니다. 잠시 후 다시 시도해주세요.",
      ),
    );
  });
});
