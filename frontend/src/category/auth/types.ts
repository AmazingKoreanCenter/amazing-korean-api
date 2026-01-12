import { z } from "zod";

export const loginReqSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6).max(72),
  device: z.string().optional(),
  browser: z.string().optional(),
  os: z.string().optional(),
  user_agent: z.string().optional(),
});

export type LoginReq = z.infer<typeof loginReqSchema>;

export const accessTokenResSchema = z.object({
  access_token: z.string(),
  expires_in: z.number().int(),
});

export type AccessTokenRes = z.infer<typeof accessTokenResSchema>;

export const loginResSchema = z.object({
  user_id: z.number().int(),
  access: accessTokenResSchema,
  session_id: z.string(),
});

export type LoginRes = z.infer<typeof loginResSchema>;

export const refreshResSchema = z.object({
  access_token: z.string(),
  expires_in: z.number().int(),
});

export type RefreshRes = z.infer<typeof refreshResSchema>;

export const refreshReqSchema = z.object({
  refresh_token: z.string().min(1),
});

export type RefreshReq = z.infer<typeof refreshReqSchema>;

export const resetPwReqSchema = z.object({
  reset_token: z.string().min(1),
  new_password: z.string().min(1),
});

export type ResetPwReq = z.infer<typeof resetPwReqSchema>;

export const resetPwResSchema = z.object({
  message: z.string(),
});

export type ResetPwRes = z.infer<typeof resetPwResSchema>;

export const findIdReqSchema = z.object({
  name: z.string().min(1).max(100),
  email: z.string().email(),
});

export type FindIdReq = z.infer<typeof findIdReqSchema>;

export const findIdResSchema = z.object({
  message: z.string(),
});

export type FindIdRes = z.infer<typeof findIdResSchema>;

export const logoutResSchema = z.object({
  ok: z.boolean(),
});

export type LogoutRes = z.infer<typeof logoutResSchema>;

export const logoutAllReqSchema = z.object({
  everywhere: z.boolean().default(true),
});

export type LogoutAllReq = z.infer<typeof logoutAllReqSchema>;
