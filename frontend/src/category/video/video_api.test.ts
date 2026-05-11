import { describe, expect, it } from "vitest";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import {
  getVideoDetail,
  getVideoList,
  getVideoProgress,
  updateVideoProgress,
} from "./video_api";
import { ApiError } from "@/api/client";

describe("video_api", () => {
  it("getVideoList sends params + lang and returns VideoListRes", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/videos", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ items: [], total: 0 });
      }),
    );
    await getVideoList(
      { page: 2 } as Parameters<typeof getVideoList>[0],
      "en",
    );
    expect(observedUrl).toContain("page=2");
    expect(observedUrl).toContain("lang=en");
  });

  it("getVideoDetail returns VideoDetail with lang", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/videos/42", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ video_id: 42 });
      }),
    );
    const res = await getVideoDetail(42, "en");
    expect(observedUrl).toContain("lang=en");
    expect((res as { video_id: number }).video_id).toBe(42);
  });

  it("getVideoDetail omits lang param when not provided", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/videos/42", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ video_id: 42 });
      }),
    );
    await getVideoDetail(42);
    expect(observedUrl).not.toContain("lang=");
  });

  it("getVideoProgress returns VideoProgressRes", async () => {
    server.use(
      http.get("/api/videos/7/progress", () =>
        HttpResponse.json({ video_id: 7, watched_seconds: 42 }),
      ),
    );
    const res = await getVideoProgress(7);
    expect((res as { watched_seconds: number }).watched_seconds).toBe(42);
  });

  it("updateVideoProgress POSTs body", async () => {
    let body: unknown = null;
    server.use(
      http.post("/api/videos/7/progress", async ({ request }) => {
        body = await request.json();
        return new HttpResponse(null, { status: 204 });
      }),
    );
    await updateVideoProgress(7, {
      watched_seconds: 100,
    } as Parameters<typeof updateVideoProgress>[1]);
    expect((body as { watched_seconds: number }).watched_seconds).toBe(100);
  });

  it("throws ApiError on 5xx", async () => {
    server.use(
      http.get(
        "/api/videos",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    await expect(getVideoList({})).rejects.toBeInstanceOf(ApiError);
  });
});
