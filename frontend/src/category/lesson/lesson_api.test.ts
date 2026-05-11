import { describe, expect, it } from "vitest";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import {
  getLessonDetail,
  getLessonItems,
  getLessonList,
  getLessonProgress,
  updateLessonProgress,
} from "./lesson_api";
import { ApiError } from "@/api/client";

describe("lesson_api", () => {
  it("getLessonList sanitizes undefined params and sends lang", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/lessons", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ items: [], total: 0 });
      }),
    );
    await getLessonList(
      { page: 1, program: undefined } as Parameters<typeof getLessonList>[0],
      "en",
    );
    expect(observedUrl).toContain("page=1");
    expect(observedUrl).not.toContain("program=");
    expect(observedUrl).toContain("lang=en");
  });

  it("getLessonDetail with lang", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/lessons/42", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ lesson_id: 42 });
      }),
    );
    await getLessonDetail(42, "ja");
    expect(observedUrl).toContain("lang=ja");
  });

  it("getLessonDetail without lang omits param", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/lessons/42", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ lesson_id: 42 });
      }),
    );
    await getLessonDetail(42);
    expect(observedUrl).not.toContain("lang=");
  });

  it("getLessonItems sends pagination params", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/lessons/42/items", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ items: [], total: 0 });
      }),
    );
    await getLessonItems(42, { page: 3, per_page: 50 });
    expect(observedUrl).toContain("page=3");
    expect(observedUrl).toContain("per_page=50");
  });

  it("getLessonProgress returns LessonProgressRes", async () => {
    server.use(
      http.get("/api/lessons/42/progress", () =>
        HttpResponse.json({ lesson_id: 42, completed: false }),
      ),
    );
    const res = await getLessonProgress(42);
    expect((res as { lesson_id: number }).lesson_id).toBe(42);
  });

  it("updateLessonProgress POSTs body and returns updated progress", async () => {
    let body: unknown = null;
    server.use(
      http.post("/api/lessons/42/progress", async ({ request }) => {
        body = await request.json();
        return HttpResponse.json({ lesson_id: 42, completed: true });
      }),
    );
    const res = await updateLessonProgress(42, {
      completed: true,
    } as Parameters<typeof updateLessonProgress>[1]);
    expect((body as { completed: boolean }).completed).toBe(true);
    expect((res as { completed: boolean }).completed).toBe(true);
  });

  it("throws ApiError on 5xx", async () => {
    server.use(
      http.get(
        "/api/lessons",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    await expect(getLessonList()).rejects.toBeInstanceOf(ApiError);
  });
});
