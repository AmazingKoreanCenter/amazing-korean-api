import { describe, expect, it } from "vitest";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import {
  getUserMe,
  getUserSettings,
  updateUserMe,
  updateUserSettings,
} from "./user_api";
import { ApiError } from "@/api/client";

describe("user_api", () => {
  it("getUserMe returns UserDetail on 200", async () => {
    server.use(
      http.get("/api/users/me", () =>
        HttpResponse.json({ user_id: 42, email: "u@e.com" }),
      ),
    );
    const res = await getUserMe();
    expect((res as { user_id: number }).user_id).toBe(42);
  });

  it("updateUserMe POSTs body", async () => {
    let body: unknown = null;
    server.use(
      http.post("/api/users/me", async ({ request }) => {
        body = await request.json();
        return new HttpResponse(null, { status: 204 });
      }),
    );
    await updateUserMe({ name: "테스트" } as Parameters<typeof updateUserMe>[0]);
    expect((body as { name: string }).name).toBe("테스트");
  });

  it("getUserSettings returns SettingsRes on 200", async () => {
    server.use(
      http.get("/api/users/me/settings", () =>
        HttpResponse.json({ language: "ko", theme: "light" }),
      ),
    );
    const res = await getUserSettings();
    expect((res as { language: string }).language).toBe("ko");
  });

  it("updateUserSettings POSTs body", async () => {
    let body: unknown = null;
    server.use(
      http.post("/api/users/me/settings", async ({ request }) => {
        body = await request.json();
        return new HttpResponse(null, { status: 204 });
      }),
    );
    await updateUserSettings({
      language: "en",
    } as Parameters<typeof updateUserSettings>[0]);
    expect((body as { language: string }).language).toBe("en");
  });

  it("throws ApiError on 5xx", async () => {
    server.use(
      http.get(
        "/api/users/me",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    await expect(getUserMe()).rejects.toBeInstanceOf(ApiError);
  });
});
