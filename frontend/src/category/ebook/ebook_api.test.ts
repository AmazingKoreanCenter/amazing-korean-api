import { describe, expect, it } from "vitest";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import {
  cancelEbookPurchase,
  createEbookPurchase,
  deleteAdminEbookPurchase,
  fetchPageImage,
  fetchPageTile,
  getAdminEbookPurchase,
  getAdminEbookPurchases,
  getEbookCatalog,
  getMyPurchases,
  getViewerMeta,
  sendViewerHeartbeat,
  updateAdminEbookStatus,
} from "./ebook_api";
import { ApiError } from "@/api/client";

const samplePurchase = {
  purchase_code: "EB-260511-0001",
  status: "completed",
  language: "en",
  edition: "premium",
  payment_method: "paddle",
  price: 19900,
  currency: "KRW",
  created_at: "2026-05-11T00:00:00Z",
};

describe("ebook_api", () => {
  it("getEbookCatalog returns the parsed EbookCatalogRes", async () => {
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
    const res = await getEbookCatalog();
    expect(res.items).toHaveLength(1);
    expect(res.items[0].language).toBe("en");
  });

  it("createEbookPurchase POSTs the body and returns PurchaseRes", async () => {
    let body: unknown = null;
    server.use(
      http.post("/api/ebook/purchase", async ({ request }) => {
        body = await request.json();
        return HttpResponse.json(samplePurchase);
      }),
    );
    const res = await createEbookPurchase({
      language: "en",
      edition: "premium",
      payment_method: "paddle",
    } as Parameters<typeof createEbookPurchase>[0]);
    expect((body as { language: string }).language).toBe("en");
    expect(res.purchase_code).toBe("EB-260511-0001");
  });

  it("getMyPurchases returns items array", async () => {
    server.use(
      http.get("/api/ebook/my", () =>
        HttpResponse.json({ items: [samplePurchase] }),
      ),
    );
    const res = await getMyPurchases();
    expect(res.items).toHaveLength(1);
  });

  it("cancelEbookPurchase DELETEs path with code", async () => {
    let observedMethod: string | null = null;
    server.use(
      http.delete("/api/ebook/purchase/EB-260511-0001", ({ request }) => {
        observedMethod = request.method;
        return new HttpResponse(null, { status: 204 });
      }),
    );
    await cancelEbookPurchase("EB-260511-0001");
    expect(observedMethod).toBe("DELETE");
  });

  it("getViewerMeta returns ViewerMetaRes for the path code", async () => {
    server.use(
      http.get("/api/ebook/viewer/EB-1/meta", () =>
        HttpResponse.json({
          purchase_code: "EB-1",
          language: "en",
          edition: "premium",
          total_pages: 100,
          toc: [],
          session_id: "sess_1",
          hmac_secret: "deadbeef",
          tile_mode: false,
        }),
      ),
    );
    const res = await getViewerMeta("EB-1");
    expect(res.purchase_code).toBe("EB-1");
    expect(res.total_pages).toBe(100);
  });

  it("sendViewerHeartbeat POSTs session_id and returns {valid}", async () => {
    let body: unknown = null;
    server.use(
      http.post("/api/ebook/viewer/heartbeat", async ({ request }) => {
        body = await request.json();
        return HttpResponse.json({ valid: true });
      }),
    );
    const res = await sendViewerHeartbeat("sess_xyz");
    expect((body as { session_id: string }).session_id).toBe("sess_xyz");
    expect(res.valid).toBe(true);
  });

  it("fetchPageImage returns ArrayBuffer (no HMAC path when secret omitted)", async () => {
    server.use(
      http.get("/api/ebook/viewer/EB-1/pages/1", ({ request }) => {
        // HMAC 헤더 미주입 path 검증 (secret omitted)
        expect(request.headers.get("X-Ebook-Signature")).toBeNull();
        return HttpResponse.arrayBuffer(new Uint8Array([1, 2, 3]).buffer);
      }),
    );
    const buf = await fetchPageImage("EB-1", 1, "sess_xyz");
    expect(buf).toBeInstanceOf(ArrayBuffer);
    expect(new Uint8Array(buf).length).toBe(3);
  });

  it("fetchPageImage includes HMAC headers when secret provided", async () => {
    let sigHeader: string | null = null;
    let tsHeader: string | null = null;
    server.use(
      http.get("/api/ebook/viewer/EB-1/pages/2", ({ request }) => {
        sigHeader = request.headers.get("X-Ebook-Signature");
        tsHeader = request.headers.get("X-Ebook-Timestamp");
        return HttpResponse.arrayBuffer(new Uint8Array([0]).buffer);
      }),
    );
    await fetchPageImage("EB-1", 2, "sess_xyz", "deadbeef");
    expect(sigHeader).toMatch(/^[0-9a-f]{64}$/);
    expect(tsHeader).toMatch(/^\d+$/);
  });

  it("fetchPageTile returns ArrayBuffer (no HMAC path)", async () => {
    server.use(
      http.get("/api/ebook/viewer/EB-1/pages/1/tiles/0/0", () =>
        HttpResponse.arrayBuffer(new Uint8Array([7]).buffer),
      ),
    );
    const buf = await fetchPageTile("EB-1", 1, 0, 0, "sess_xyz");
    expect(buf).toBeInstanceOf(ArrayBuffer);
  });

  it("fetchPageTile includes HMAC headers when secret provided", async () => {
    let sigHeader: string | null = null;
    server.use(
      http.get("/api/ebook/viewer/EB-1/pages/1/tiles/1/2", ({ request }) => {
        sigHeader = request.headers.get("X-Ebook-Signature");
        return HttpResponse.arrayBuffer(new Uint8Array([0]).buffer);
      }),
    );
    await fetchPageTile("EB-1", 1, 1, 2, "sess_xyz", "deadbeef");
    expect(sigHeader).toMatch(/^[0-9a-f]{64}$/);
  });

  it("throws ApiError on 5xx error response", async () => {
    server.use(
      http.get(
        "/api/ebook/catalog",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    await expect(getEbookCatalog()).rejects.toBeInstanceOf(ApiError);
  });

  // Admin functions
  it("getAdminEbookPurchases passes query params and returns AdminEbookListRes", async () => {
    let observedUrl: string | null = null;
    server.use(
      http.get("/api/admin/ebook/purchases", ({ request }) => {
        observedUrl = request.url;
        return HttpResponse.json({ items: [], total: 0 });
      }),
    );
    const res = await getAdminEbookPurchases({ page: 2, per_page: 20, status: "pending" });
    expect(observedUrl).toContain("page=2");
    expect(observedUrl).toContain("per_page=20");
    expect(observedUrl).toContain("status=pending");
    expect(res.total).toBe(0);
  });

  it("getAdminEbookPurchase returns single AdminEbookPurchaseItem", async () => {
    server.use(
      http.get("/api/admin/ebook/purchases/42", () =>
        HttpResponse.json({ purchase_id: 42, purchase_code: "EB-1" }),
      ),
    );
    const res = await getAdminEbookPurchase(42);
    expect(res.purchase_id).toBe(42);
  });

  it("updateAdminEbookStatus PATCHes status body", async () => {
    let body: unknown = null;
    server.use(
      http.patch("/api/admin/ebook/purchases/42/status", async ({ request }) => {
        body = await request.json();
        return HttpResponse.json({ purchase_id: 42, purchase_code: "EB-1" });
      }),
    );
    await updateAdminEbookStatus(42, { status: "completed" });
    expect((body as { status: string }).status).toBe("completed");
  });

  it("deleteAdminEbookPurchase DELETEs and returns {message}", async () => {
    server.use(
      http.delete("/api/admin/ebook/purchases/42", () =>
        HttpResponse.json({ message: "ok" }),
      ),
    );
    const res = await deleteAdminEbookPurchase(42);
    expect(res.message).toBe("ok");
  });
});
