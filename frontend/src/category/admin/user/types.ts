import { z } from "zod";

import { userAuthSchema, userGenderSchema } from "../../user/types";

export const adminUserResSchema = z.object({
  id: z.number().int(),
  email: z.string(),
  name: z.string(),
  nickname: z.string().optional(),
  language: z.string().optional(),
  country: z.string().optional(),
  birthday: z.string().date().optional(),
  gender: userGenderSchema,
  user_state: z.boolean(),
  user_auth: userAuthSchema,
  created_at: z.string().datetime(),
  quit_at: z.string().datetime().optional(),
});

export type AdminUserRes = z.infer<typeof adminUserResSchema>;

export const adminUserListReqSchema = z.object({
  page: z.number().int().optional(),
  size: z.number().int().optional(),
  q: z.string().optional(),
  sort: z.string().optional(),
  order: z.string().optional(),
});

export type AdminUserListReq = z.infer<typeof adminUserListReqSchema>;

export const adminUserSummarySchema = z.object({
  id: z.number().int(),
  email: z.string(),
  nickname: z.string().optional(),
  role: userAuthSchema,
  created_at: z.string().datetime(),
});

export type AdminUserSummary = z.infer<typeof adminUserSummarySchema>;

export const adminUserListMetaSchema = z.object({
  total_count: z.number().int(),
  total_pages: z.number().int(),
  current_page: z.number().int(),
  per_page: z.number().int(),
});

export type AdminUserListMeta = z.infer<typeof adminUserListMetaSchema>;

export const adminUserListResSchema = z.object({
  items: z.array(adminUserSummarySchema),
  meta: adminUserListMetaSchema,
});

export type AdminUserListRes = z.infer<typeof adminUserListResSchema>;

export const adminCreateUserReqSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
  nickname: z.string().min(1).max(100),
  name: z.string().min(1).max(100),
  user_auth: z.string().optional(),
});

export type AdminCreateUserReq = z.infer<typeof adminCreateUserReqSchema>;

export const adminBulkCreateReqSchema = z.object({
  items: z.array(adminCreateUserReqSchema).min(1).max(100),
});

export type AdminBulkCreateReq = z.infer<typeof adminBulkCreateReqSchema>;

export const bulkSummarySchema = z.object({
  total: z.number().int(),
  success: z.number().int(),
  failure: z.number().int(),
});

export type BulkSummary = z.infer<typeof bulkSummarySchema>;

export const bulkItemErrorSchema = z.object({
  code: z.string(),
  message: z.string(),
});

export type BulkItemError = z.infer<typeof bulkItemErrorSchema>;

export const bulkItemResultSchema = z.object({
  email: z.string(),
  status: z.number().int(),
  data: adminUserResSchema.optional(),
  error: bulkItemErrorSchema.optional(),
});

export type BulkItemResult = z.infer<typeof bulkItemResultSchema>;

export const adminBulkCreateResSchema = z.object({
  summary: bulkSummarySchema,
  results: z.array(bulkItemResultSchema),
});

export type AdminBulkCreateRes = z.infer<typeof adminBulkCreateResSchema>;

export const adminBulkUpdateItemReqSchema = z.object({
  id: z.number().int(),
  email: z.string().email().optional(),
  password: z.string().min(8).optional(),
  name: z.string().min(1).max(50).optional(),
  nickname: z.string().min(1).max(100).optional(),
  language: z.string().min(1).max(50).optional(),
  country: z.string().min(1).max(50).optional(),
  birthday: z.string().date().optional(),
  gender: userGenderSchema.optional(),
  user_state: z.boolean().optional(),
  user_auth: userAuthSchema.optional(),
});

export type AdminBulkUpdateItemReq = z.infer<typeof adminBulkUpdateItemReqSchema>;

export const adminBulkUpdateReqSchema = z.object({
  items: z.array(adminBulkUpdateItemReqSchema).min(1).max(100),
});

export type AdminBulkUpdateReq = z.infer<typeof adminBulkUpdateReqSchema>;

export const bulkUpdateItemResultSchema = z.object({
  id: z.number().int(),
  status: z.number().int(),
  data: adminUserResSchema.optional(),
  error: bulkItemErrorSchema.optional(),
});

export type BulkUpdateItemResult = z.infer<typeof bulkUpdateItemResultSchema>;

export const adminBulkUpdateResSchema = z.object({
  summary: bulkSummarySchema,
  results: z.array(bulkUpdateItemResultSchema),
});

export type AdminBulkUpdateRes = z.infer<typeof adminBulkUpdateResSchema>;

export const adminUpdateUserReqSchema = z.object({
  email: z.string().email().optional(),
  password: z.string().min(8).optional(),
  name: z.string().min(1).max(50).optional(),
  nickname: z.string().min(1).max(100).optional(),
  language: z.string().min(1).max(50).optional(),
  country: z.string().min(1).max(50).optional(),
  birthday: z.string().date().optional(),
  gender: userGenderSchema.optional(),
  user_state: z.boolean().optional(),
  user_auth: userAuthSchema.optional(),
});

export type AdminUpdateUserReq = z.infer<typeof adminUpdateUserReqSchema>;
