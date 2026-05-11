import { describe, expect, it } from "vitest";
import { http, HttpResponse } from "msw";
import { server } from "@/test/server";
import {
  createTextbookOrder,
  getMyTextbookOrders,
  getTextbookCatalog,
  getTextbookOrderByCode,
} from "./textbook_api";
import { ApiError } from "@/api/client";

const sampleCatalog = {
  items: [
    {
      language: "en",
      language_name_ko: "영어",
      language_name_en: "English",
      available_types: ["basic"],
      unit_price: 25000,
      available: true,
      isbn_ready: true,
    },
  ],
  currency: "KRW",
  min_total_quantity: 1,
};

const sampleOrder = {
  order_id: 42,
  order_code: "AKB-260511-0042",
  status: "pending",
  orderer_name: "홍길동",
  orderer_email: "test@example.com",
  orderer_phone: "010-1234-5678",
  org_name: null,
  org_type: null,
  delivery_postal_code: null,
  delivery_address: "서울특별시",
  delivery_detail: null,
  payment_method: "bank_transfer",
  depositor_name: null,
  tax_invoice: false,
  tax_biz_number: null,
  tax_company_name: null,
  tax_rep_name: null,
  tax_address: null,
  tax_biz_type: null,
  tax_biz_item: null,
  tax_email: null,
  total_quantity: 1,
  total_price: 25000,
  items: [],
  created_at: "2026-05-11T00:00:00Z",
};

describe("textbook_api", () => {
  it("getTextbookCatalog returns the parsed CatalogRes body on 200", async () => {
    server.use(
      http.get("/api/textbook/catalog", () => HttpResponse.json(sampleCatalog)),
    );
    const res = await getTextbookCatalog();
    expect(res.currency).toBe("KRW");
    expect(res.items).toHaveLength(1);
    expect(res.items[0].language).toBe("en");
  });

  it("createTextbookOrder POSTs the body and returns OrderRes", async () => {
    let observedBody: unknown = null;
    server.use(
      http.post("/api/textbook/orders", async ({ request }) => {
        observedBody = await request.json();
        return HttpResponse.json(sampleOrder);
      }),
    );
    const res = await createTextbookOrder({
      orderer_name: "홍길동",
      orderer_email: "test@example.com",
      orderer_phone: "010-1234-5678",
      delivery_address: "서울특별시",
      payment_method: "bank_transfer",
      tax_invoice: false,
      items: [{ language: "en", textbook_type: "basic", quantity: 1 }],
    } as Parameters<typeof createTextbookOrder>[0]);
    expect((observedBody as { orderer_name: string }).orderer_name).toBe("홍길동");
    expect(res.order_code).toBe("AKB-260511-0042");
  });

  it("getTextbookOrderByCode returns OrderRes for the path param", async () => {
    server.use(
      http.get("/api/textbook/orders/AKB-260511-0042", () =>
        HttpResponse.json(sampleOrder),
      ),
    );
    const res = await getTextbookOrderByCode("AKB-260511-0042");
    expect(res.order_id).toBe(42);
  });

  it("getMyTextbookOrders returns the orders array on 200", async () => {
    server.use(
      http.get("/api/textbook/my", () =>
        HttpResponse.json({ orders: [sampleOrder] }),
      ),
    );
    const res = await getMyTextbookOrders();
    expect(res.orders).toHaveLength(1);
    expect(res.orders[0].order_id).toBe(42);
  });

  it("throws ApiError on 5xx server error", async () => {
    server.use(
      http.get(
        "/api/textbook/catalog",
        () => new HttpResponse(null, { status: 500 }),
      ),
    );
    await expect(getTextbookCatalog()).rejects.toBeInstanceOf(ApiError);
  });
});
