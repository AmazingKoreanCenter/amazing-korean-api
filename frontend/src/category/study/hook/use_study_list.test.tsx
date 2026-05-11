import { describe, expect, it, vi } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { useStudyList } from "./use_study_list";
import * as studyApi from "../study_api";

const toastError = vi.fn();
vi.mock("sonner", () => ({
  toast: { error: (...args: unknown[]) => toastError(...args) },
}));

const renderWithQuery = (
  params: Parameters<typeof useStudyList>[0] = { page: 1 } as Parameters<typeof useStudyList>[0],
) => {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={client}>{children}</QueryClientProvider>
  );
  return renderHook(() => useStudyList(params), { wrapper });
};

describe("useStudyList", () => {
  it("transitions isLoading → isSuccess with list data", async () => {
    server.use(
      http.get("/api/studies", () =>
        HttpResponse.json({ items: [{ study_id: 1 }], total: 1 }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data?.total).toBe(1);
  });

  it("calls toast.error with ApiError message on 5xx (ApiError branch)", async () => {
    toastError.mockClear();
    server.use(
      http.get(
        "/api/studies",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    await waitFor(() => expect(toastError).toHaveBeenCalled());
  });

  it("calls toast.error with Error.message on generic Error throw (Error branch)", async () => {
    toastError.mockClear();
    vi.spyOn(studyApi, "getStudyList").mockRejectedValueOnce(
      new Error("custom error message"),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    await waitFor(() =>
      expect(toastError).toHaveBeenCalledWith("custom error message"),
    );
  });

  it("calls toast.error with fallback on non-Error throw (fallback branch)", async () => {
    toastError.mockClear();
    vi.spyOn(studyApi, "getStudyList").mockRejectedValueOnce(
      "not an Error instance",
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    await waitFor(() =>
      expect(toastError).toHaveBeenCalledWith(
        "요청에 실패했습니다. 잠시 후 다시 시도해주세요.",
      ),
    );
  });
});
