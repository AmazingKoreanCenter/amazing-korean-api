import { z } from "zod";

// ─────────────────────── Enums ───────────────────────

export const ebookEditionSchema = z.enum(["student", "teacher"]);
export type EbookEdition = z.infer<typeof ebookEditionSchema>;

export const ebookPurchaseStatusSchema = z.enum([
  "pending",
  "completed",
  "refunded",
]);
export type EbookPurchaseStatus = z.infer<typeof ebookPurchaseStatusSchema>;

export const ebookPaymentMethodSchema = z.enum(["paddle", "bank_transfer"]);
export type EbookPaymentMethod = z.infer<typeof ebookPaymentMethodSchema>;

// ─────────────────────── Catalog ───────────────────────

export const ebookEditionInfoSchema = z.object({
  edition: ebookEditionSchema,
  price: z.number(),
  currency: z.string(),
  paddle_price_usd: z.number().nullable().optional(),
  total_pages: z.number(),
  available: z.boolean(),
});
export type EbookEditionInfo = z.infer<typeof ebookEditionInfoSchema>;

export const ebookCatalogItemSchema = z.object({
  language: z.string(),
  language_name_ko: z.string(),
  language_name_en: z.string(),
  editions: z.array(ebookEditionInfoSchema),
});
export type EbookCatalogItem = z.infer<typeof ebookCatalogItemSchema>;

export const ebookCatalogResSchema = z.object({
  items: z.array(ebookCatalogItemSchema),
  paddle_ebook_price_id: z.string().nullable().optional(),
  client_token: z.string().nullable().optional(),
  sandbox: z.boolean().optional(),
});
export type EbookCatalogRes = z.infer<typeof ebookCatalogResSchema>;

// ─────────────────────── Purchase ───────────────────────

export interface CreatePurchaseReq {
  language: string;
  edition: EbookEdition;
  payment_method: EbookPaymentMethod;
}

export const purchaseResSchema = z.object({
  purchase_code: z.string(),
  status: ebookPurchaseStatusSchema,
  language: z.string(),
  edition: ebookEditionSchema,
  payment_method: ebookPaymentMethodSchema,
  price: z.number(),
  currency: z.string(),
  created_at: z.string(),
});
export type PurchaseRes = z.infer<typeof purchaseResSchema>;

export const myPurchasesResSchema = z.object({
  items: z.array(purchaseResSchema),
});
export type MyPurchasesRes = z.infer<typeof myPurchasesResSchema>;

// ─────────────────────── Viewer ───────────────────────

export const tocEntrySchema = z.object({
  title: z.string(),
  title_ko: z.string(),
  page: z.number(),
});
export type TocEntry = z.infer<typeof tocEntrySchema>;

export const viewerMetaResSchema = z.object({
  purchase_code: z.string(),
  language: z.string(),
  edition: ebookEditionSchema,
  total_pages: z.number(),
  toc: z.array(tocEntrySchema),
  session_id: z.string(),
  tile_mode: z.boolean(),
  grid_rows: z.number().nullable().optional(),
  grid_cols: z.number().nullable().optional(),
});
export type ViewerMetaRes = z.infer<typeof viewerMetaResSchema>;

// ─────────────────────── Heartbeat ───────────────────────

export const heartbeatResSchema = z.object({
  valid: z.boolean(),
});
export type HeartbeatRes = z.infer<typeof heartbeatResSchema>;

// ─────────────────────── Admin ───────────────────────

export const adminEbookPurchaseItemSchema = z.object({
  purchase_id: z.number(),
  purchase_code: z.string(),
  user_id: z.number(),
  language: z.string(),
  edition: ebookEditionSchema,
  payment_method: ebookPaymentMethodSchema,
  status: ebookPurchaseStatusSchema,
  price: z.number(),
  currency: z.string(),
  paddle_txn_id: z.string().nullable(),
  completed_at: z.string().nullable(),
  refunded_at: z.string().nullable(),
  created_at: z.string(),
});
export type AdminEbookPurchaseItem = z.infer<
  typeof adminEbookPurchaseItemSchema
>;

export const adminEbookListResSchema = z.object({
  items: z.array(adminEbookPurchaseItemSchema),
  meta: z.object({
    total_count: z.number(),
    page: z.number(),
    per_page: z.number(),
    total_pages: z.number(),
  }),
});
export type AdminEbookListRes = z.infer<typeof adminEbookListResSchema>;
