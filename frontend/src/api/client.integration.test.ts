import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import { useAuthStore } from "@/hooks/use_auth_store";
import { ApiError, api, request } from "./client";

const BASE = "/api";

const stubLocation = () => {
  const original = window.location;
  Object.defineProperty(window, "location", {
    configurable: true,
    writable: true,
    value: { ...original, href: original.href, assign: vi.fn() },
  });
  return () => {
    Object.defineProperty(window, "location", {
      configurable: true,
      writable: true,
      value: original,
    });
  };
};

describe("api client interceptors (msw integration)", () => {
  beforeEach(() => {
    useAuthStore.setState({
      user: null,
      accessToken: null,
      isLoggedIn: false,
    });
    api.defaults.baseURL = BASE;
    delete api.defaults.headers.common["Authorization"];
  });

  afterEach(() => {
    localStorage.clear();
  });

  it("attaches Bearer Authorization header from auth store on outbound requests", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "tok-1",
      isLoggedIn: true,
    });
    let seenAuth: string | null = null;
    server.use(
      http.get(`${BASE}/health`, ({ request: req }) => {
        seenAuth = req.headers.get("authorization");
        return HttpResponse.json({ ok: true });
      }),
    );
    const data = await request<{ ok: boolean }>("/health");
    expect(data).toEqual({ ok: true });
    expect(seenAuth).toBe("Bearer tok-1");
  });

  it("retries the original request after a 401 + successful refresh", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "stale",
      isLoggedIn: true,
    });
    let attempt = 0;
    server.use(
      http.get(`${BASE}/me`, ({ request: req }) => {
        attempt += 1;
        if (attempt === 1) {
          return new HttpResponse(null, { status: 401 });
        }
        return HttpResponse.json({
          authorization: req.headers.get("authorization"),
          attempt,
        });
      }),
      http.post(`${BASE}/auth/refresh`, () =>
        HttpResponse.json({
          user_id: 1,
          access: { access_token: "fresh", expires_in: 900 },
          session_id: "sess",
        }),
      ),
    );
    const res = await request<{ authorization: string; attempt: number }>("/me");
    expect(res.attempt).toBe(2);
    expect(res.authorization).toBe("Bearer fresh");
    expect(useAuthStore.getState().accessToken).toBe("fresh");
  });

  it("logs out and redirects to /login when refresh itself fails", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "stale",
      isLoggedIn: true,
    });
    server.use(
      http.get(`${BASE}/me`, () => new HttpResponse(null, { status: 401 })),
      http.post(
        `${BASE}/auth/refresh`,
        () => new HttpResponse(null, { status: 401 }),
      ),
    );
    const restore = stubLocation();
    try {
      await expect(request("/me")).rejects.toBeDefined();
    } finally {
      // location.href = "/login" 호출 확인
      expect(window.location.href).toBe("/login");
      expect(useAuthStore.getState().isLoggedIn).toBe(false);
      restore();
    }
  });

  it("wraps non-2xx responses with ApiError carrying status and parsed message", async () => {
    server.use(
      http.get(`${BASE}/protected`, () =>
        HttpResponse.json(
          { error: { message: "권한 없음" } },
          { status: 403 },
        ),
      ),
    );
    await expect(request("/protected", { skipAuthRefresh: true })).rejects.toMatchObject({
      name: "ApiError",
      status: 403,
      message: "권한 없음",
    });
  });

  it("returns undefined for 204 No Content responses", async () => {
    server.use(
      http.delete(`${BASE}/items/42`, () => new HttpResponse(null, { status: 204 })),
    );
    const out = await request("/items/42", { method: "DELETE" });
    expect(out).toBeUndefined();
  });

  it("single-flight: 동시 401 다발이 와도 /auth/refresh 는 정확히 1번만 호출한다", async () => {
    // 관리자 세션 v2 PR#1 핵심 회귀. single-flight 가 없으면 동시 401 마다 따로 refresh →
    // 서버 토큰 회전 후 옛 토큰 재사용 → reuse 감지 → 강제 로그아웃. 게이트로 refresh 1회만.
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "stale",
      isLoggedIn: true,
    });
    let refreshCalls = 0;
    const okIfFresh = (req: Request, path: string) => {
      const auth = req.headers.get("authorization");
      if (auth !== "Bearer fresh") {
        return new HttpResponse(null, { status: 401 });
      }
      return HttpResponse.json({ path, auth });
    };
    server.use(
      http.get(`${BASE}/a`, ({ request: req }) => okIfFresh(req, "a")),
      http.get(`${BASE}/b`, ({ request: req }) => okIfFresh(req, "b")),
      http.get(`${BASE}/c`, ({ request: req }) => okIfFresh(req, "c")),
      http.post(`${BASE}/auth/refresh`, async () => {
        refreshCalls += 1;
        // 동시성 윈도우 확보: 지연으로 세 요청의 인터셉터가 같은 게이트를 공유하게 함.
        await new Promise((r) => setTimeout(r, 20));
        return HttpResponse.json({
          user_id: 1,
          access: { access_token: "fresh", expires_in: 900 },
          session_id: "sess",
        });
      }),
    );

    const [ra, rb, rc] = await Promise.all([
      request<{ path: string; auth: string }>("/a"),
      request<{ path: string; auth: string }>("/b"),
      request<{ path: string; auth: string }>("/c"),
    ]);

    expect(refreshCalls).toBe(1); // ← single-flight 핵심 (게이트 없으면 3)
    expect(ra.auth).toBe("Bearer fresh");
    expect(rb.auth).toBe("Bearer fresh");
    expect(rc.auth).toBe("Bearer fresh");
    expect(useAuthStore.getState().accessToken).toBe("fresh");
  });

  it("does not refresh when the 401 carries skipAuthRefresh", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "stale",
      isLoggedIn: true,
    });
    server.use(
      http.get(`${BASE}/no-retry`, () => new HttpResponse(null, { status: 401 })),
    );
    await expect(
      request("/no-retry", { skipAuthRefresh: true }),
    ).rejects.toBeInstanceOf(ApiError);
    // refresh handler 미등록 = MSW 가 unhandled 로 에러 throw 했을 것이므로
    // skipAuthRefresh path 가 인터셉터의 retry 분기를 우회한 것을 확인.
  });
});
