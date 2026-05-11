import { describe, expect, it } from "vitest";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import {
  finishWritingSession,
  getStudyDetail,
  getStudyList,
  getStudyTask,
  getTaskExplain,
  getTaskStatus,
  getWritingPracticeSeed,
  getWritingStats,
  listWritingSessions,
  startWritingSession,
  submitAnswer,
} from "./study_api";
import { ApiError } from "@/api/client";

describe("study_api", () => {
  it("getStudyList sends params + lang and returns StudyListRes", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/studies", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ items: [], total: 0 });
      }),
    );
    const res = await getStudyList(
      { page: 2, program: "beginner", sort: "latest" } as Parameters<typeof getStudyList>[0],
      "en",
    );
    expect(observedUrl).toContain("page=2");
    expect(observedUrl).toContain("program=beginner");
    expect(observedUrl).toContain("lang=en");
    expect(res.total).toBe(0);
  });

  it("getStudyList sanitizes undefined params", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/studies", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ items: [], total: 0 });
      }),
    );
    await getStudyList(
      { page: 1, program: undefined } as Parameters<typeof getStudyList>[0],
      undefined,
    );
    expect(observedUrl).toContain("page=1");
    expect(observedUrl).not.toContain("program=");
    expect(observedUrl).not.toContain("lang=");
  });

  it("getStudyDetail returns the parsed body", async () => {
    server.use(
      http.get("/api/studies/42", () =>
        HttpResponse.json({ study_id: 42, title: "Test" }),
      ),
    );
    const res = await getStudyDetail(42);
    expect((res as { study_id: number }).study_id).toBe(42);
  });

  it("getStudyTask returns the parsed body", async () => {
    server.use(
      http.get("/api/studies/tasks/7", () =>
        HttpResponse.json({ task_id: 7, kind: "reading" }),
      ),
    );
    const res = await getStudyTask(7);
    expect((res as { task_id: number }).task_id).toBe(7);
  });

  it("submitAnswer POSTs body to /studies/tasks/:id/answer", async () => {
    let body: unknown = null;
    server.use(
      http.post("/api/studies/tasks/7/answer", async ({ request }) => {
        body = await request.json();
        return HttpResponse.json({ correct: true });
      }),
    );
    await submitAnswer(7, { kind: "reading", answer: "abc" } as Parameters<typeof submitAnswer>[1]);
    expect((body as { kind: string }).kind).toBe("reading");
  });

  it("getTaskStatus returns parsed TaskStatusRes", async () => {
    server.use(
      http.get("/api/studies/tasks/7/status", () =>
        HttpResponse.json({ completed: true }),
      ),
    );
    const res = await getTaskStatus(7);
    expect((res as { completed: boolean }).completed).toBe(true);
  });

  it("getTaskExplain returns parsed TaskExplainRes", async () => {
    server.use(
      http.get("/api/studies/tasks/7/explain", () =>
        HttpResponse.json({ explain: "..." }),
      ),
    );
    const res = await getTaskExplain(7);
    expect((res as { explain: string }).explain).toBe("...");
  });

  it("startWritingSession POSTs body", async () => {
    let body: unknown = null;
    server.use(
      http.post("/api/studies/writing/sessions", async ({ request }) => {
        body = await request.json();
        return HttpResponse.json({ session_id: 100 });
      }),
    );
    await startWritingSession({ level: "beginner" } as Parameters<typeof startWritingSession>[0]);
    expect((body as { level: string }).level).toBe("beginner");
  });

  it("finishWritingSession PATCHes body to /sessions/:id", async () => {
    let body: unknown = null;
    server.use(
      http.patch(
        "/api/studies/writing/sessions/100",
        async ({ request }) => {
          body = await request.json();
          return HttpResponse.json({ session_id: 100, finished: true });
        },
      ),
    );
    await finishWritingSession(100, {
      total_chars: 50,
      correct_chars: 48,
    } as Parameters<typeof finishWritingSession>[1]);
    expect((body as { total_chars: number }).total_chars).toBe(50);
  });

  it("listWritingSessions sends params", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/studies/writing/sessions", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ items: [], total: 0 });
      }),
    );
    await listWritingSessions({
      level: "beginner",
    } as Parameters<typeof listWritingSessions>[0]);
    expect(observedUrl).toContain("level=beginner");
  });

  it("getWritingStats sends params and returns body", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/studies/writing/stats", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({
          total_sessions: 5,
          avg_accuracy: 95,
          avg_cpm: 250,
        });
      }),
    );
    const res = await getWritingStats({ days: 7 } as Parameters<typeof getWritingStats>[0]);
    expect(observedUrl).toContain("days=7");
    expect((res as { total_sessions: number }).total_sessions).toBe(5);
  });

  it("getWritingPracticeSeed sends params and returns body", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/studies/writing/practice", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ items: [] });
      }),
    );
    await getWritingPracticeSeed({
      level: "beginner",
      practice_type: "jamo",
    } as Parameters<typeof getWritingPracticeSeed>[0]);
    expect(observedUrl).toContain("level=beginner");
    expect(observedUrl).toContain("practice_type=jamo");
  });

  it("throws ApiError on 5xx error", async () => {
    server.use(
      http.get(
        "/api/studies",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    await expect(getStudyList({})).rejects.toBeInstanceOf(ApiError);
  });
});
