import { describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MemoryRouter } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string, vars?: Record<string, unknown>) => {
      if (key === "video.totalVideos") return `총 ${vars?.count ?? 0}편`;
      if (key === "video.emptyTitle") return "비디오 없음";
      if (key === "video.heroBadge") return "비디오 학습";
      if (key === "video.listTitle") return "전체 비디오";
      if (key === "video.listDescription") return "한국어 비디오 모음";
      if (key === "common.page") return "페이지";
      if (key === "common.loading") return "불러오는 중";
      return key;
    },
    i18n: { language: "ko" },
  }),
}));

vi.mock("i18next", () => ({
  default: { language: "ko" },
}));

vi.mock("sonner", () => ({
  toast: { error: vi.fn(), success: vi.fn() },
}));

import { VideoListPage } from "./video_list_page";

const renderPage = () => {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>
      <MemoryRouter>
        <VideoListPage />
      </MemoryRouter>
    </QueryClientProvider>,
  );
};

const item = (id: number, overrides?: Partial<Record<string, unknown>>) => ({
  video_id: id,
  video_idx: `v${id}`,
  title: `Video ${id}`,
  subtitle: null,
  duration_seconds: 120,
  language: "ko",
  thumbnail_url: null,
  state: "open",
  access: "public",
  tags: [],
  has_captions: true,
  created_at: "2026-05-10T00:00:00Z",
  ...overrides,
});

const meta = (overrides?: Partial<Record<string, number>>) => ({
  total_count: 0,
  total_pages: 1,
  current_page: 1,
  per_page: 9,
  ...overrides,
});

describe("VideoListPage", () => {
  it("renders the SkeletonGrid while loading", () => {
    server.use(
      http.get("/api/videos", async () => {
        await new Promise((r) => setTimeout(r, 50));
        return HttpResponse.json({ meta: meta(), data: [] });
      }),
    );
    const { container } = renderPage();
    const grids = container.querySelectorAll(".grid");
    expect(grids.length).toBeGreaterThanOrEqual(1);
  });

  it("renders EmptyState when the response data is empty", async () => {
    server.use(
      http.get("/api/videos", () =>
        HttpResponse.json({
          meta: meta({ total_count: 0, total_pages: 1 }),
          data: [],
        }),
      ),
    );
    renderPage();
    expect(await screen.findByText("비디오 없음")).toBeInTheDocument();
  });

  it("renders VideoCard items + ListStatsBar total when data arrives", async () => {
    server.use(
      http.get("/api/videos", () =>
        HttpResponse.json({
          meta: meta({ total_count: 2, total_pages: 1 }),
          data: [item(1), item(2)],
        }),
      ),
    );
    renderPage();
    expect(await screen.findByText("Video 1")).toBeInTheDocument();
    expect(screen.getByText("Video 2")).toBeInTheDocument();
    expect(screen.getByText(/총 2편/)).toBeInTheDocument();
  });

  it("does not render PaginationBar when totalPages <= 1", async () => {
    server.use(
      http.get("/api/videos", () =>
        HttpResponse.json({
          meta: meta({ total_count: 1, total_pages: 1 }),
          data: [item(1)],
        }),
      ),
    );
    renderPage();
    await screen.findByText("Video 1");
    expect(screen.queryByLabelText("Go to next page")).toBeNull();
  });

  it("changes page and re-fetches when PaginationBar Next is clicked", async () => {
    let lastRequestedPage: string | null = null;
    server.use(
      http.get("/api/videos", ({ request }) => {
        const url = new URL(request.url);
        lastRequestedPage = url.searchParams.get("page");
        const page = Number(lastRequestedPage ?? 1);
        return HttpResponse.json({
          meta: meta({ total_count: 18, total_pages: 2, current_page: page }),
          data: [item(page * 10)],
        });
      }),
    );
    const user = userEvent.setup();
    renderPage();
    expect(await screen.findByText("Video 10")).toBeInTheDocument();
    expect(lastRequestedPage).toBe("1");
    await user.click(screen.getByLabelText("Go to next page"));
    await waitFor(() => {
      expect(lastRequestedPage).toBe("2");
    });
  });
});
