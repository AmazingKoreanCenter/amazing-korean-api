import { z } from "zod";

// =============================================================================
// Enums
// =============================================================================

export const textbookLanguageSchema = z.enum([
  "ja", "zh_cn", "zh_tw", "vi", "th", "id", "my", "mn",
  "ru", "es", "pt", "fr", "de", "hi", "ne", "si", "km", "uz", "kk", "tg",
]);
export type TextbookLanguage = z.infer<typeof textbookLanguageSchema>;

export const textbookTypeSchema = z.enum(["student", "teacher"]);
export type TextbookType = z.infer<typeof textbookTypeSchema>;

export const textbookOrderStatusSchema = z.enum([
  "pending", "confirmed", "paid", "printing", "shipped", "delivered", "canceled",
]);
export type TextbookOrderStatus = z.infer<typeof textbookOrderStatusSchema>;

export const textbookPaymentMethodSchema = z.enum(["bank_transfer"]);
export type TextbookPaymentMethod = z.infer<typeof textbookPaymentMethodSchema>;

// =============================================================================
// GET /textbook/catalog
// =============================================================================

export const catalogItemSchema = z.object({
  language: textbookLanguageSchema,
  language_name_ko: z.string(),
  language_name_en: z.string(),
  available_types: z.array(textbookTypeSchema),
  unit_price: z.number().int(),
  available: z.boolean(),
});
export type CatalogItem = z.infer<typeof catalogItemSchema>;

export const catalogResSchema = z.object({
  items: z.array(catalogItemSchema),
  currency: z.string(),
  min_total_quantity: z.number().int(),
});
export type CatalogRes = z.infer<typeof catalogResSchema>;

// =============================================================================
// POST /textbook/orders
// =============================================================================

export interface CreateOrderItemReq {
  language: TextbookLanguage;
  textbook_type: TextbookType;
  quantity: number;
}

export interface CreateOrderReq {
  orderer_name: string;
  orderer_email: string;
  orderer_phone: string;
  org_name?: string;
  org_type?: string;
  delivery_postal_code?: string;
  delivery_address: string;
  delivery_detail?: string;
  payment_method: TextbookPaymentMethod;
  depositor_name?: string;
  tax_invoice: boolean;
  tax_biz_number?: string;
  tax_company_name?: string;
  tax_rep_name?: string;
  tax_address?: string;
  tax_biz_type?: string;
  tax_biz_item?: string;
  tax_email?: string;
  items: CreateOrderItemReq[];
  notes?: string;
}

// =============================================================================
// GET /textbook/orders/:code — 주문 응답
// =============================================================================

export const orderItemResSchema = z.object({
  language: textbookLanguageSchema,
  language_name: z.string(),
  textbook_type: textbookTypeSchema,
  quantity: z.number().int(),
  unit_price: z.number().int(),
  subtotal: z.number().int(),
});
export type OrderItemRes = z.infer<typeof orderItemResSchema>;

export const orderResSchema = z.object({
  order_id: z.number().int(),
  order_code: z.string(),
  status: textbookOrderStatusSchema,
  orderer_name: z.string(),
  orderer_email: z.string(),
  orderer_phone: z.string(),
  org_name: z.string().nullable(),
  org_type: z.string().nullable(),
  delivery_postal_code: z.string().nullable(),
  delivery_address: z.string(),
  delivery_detail: z.string().nullable(),
  payment_method: textbookPaymentMethodSchema,
  depositor_name: z.string().nullable(),
  tax_invoice: z.boolean(),
  tax_biz_number: z.string().nullable(),
  tax_company_name: z.string().nullable(),
  tax_rep_name: z.string().nullable(),
  tax_address: z.string().nullable(),
  tax_biz_type: z.string().nullable(),
  tax_biz_item: z.string().nullable(),
  tax_email: z.string().nullable(),
  total_quantity: z.number().int(),
  total_amount: z.number().int(),
  currency: z.string(),
  notes: z.string().nullable(),
  tracking_number: z.string().nullable(),
  tracking_provider: z.string().nullable(),
  items: z.array(orderItemResSchema),
  confirmed_at: z.string().nullable(),
  paid_at: z.string().nullable(),
  shipped_at: z.string().nullable(),
  delivered_at: z.string().nullable(),
  canceled_at: z.string().nullable(),
  created_at: z.string(),
  updated_at: z.string(),
});
export type OrderRes = z.infer<typeof orderResSchema>;
