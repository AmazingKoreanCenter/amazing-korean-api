import { describe, expect, it } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { useEbookCatalog } from "./use_ebook_catalog";

const renderWithQuery = () => {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={client}>{children}</QueryClientProvider>
  );
  return renderHook(() => useEbookCatalog(), { wrapper });
};

describe("useEbookCatalog", () => {
  it("transitions isLoading → isSuccess with catalog data", async () => {
    server.use(
      http.get("/api/ebook/catalog", () =>
        HttpResponse.json({
          items: [
            {
              language: "en",
              language_name_ko: "영어",
              language_name_en: "English",
              editions: [],
            },
          ],
          sandbox: true,
        }),
      ),
    );
    const { result } = renderWithQuery();
    expect(result.current.isLoading).toBe(true);
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data?.items).toHaveLength(1);
  });

  it("sets isError when the request fails", async () => {
    server.use(
      http.get(
        "/api/ebook/catalog",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    const { result } = renderWithQuery();
    await waitFor(() => expect(result.current.isError).toBe(true));
    expect(result.current.data).toBeUndefined();
  });
});
