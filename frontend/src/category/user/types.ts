import { z } from "zod";

import { accessTokenResSchema } from "../auth/types";

export const userAuthSchema = z.enum(["HYMN", "admin", "manager", "learner"]);

export type UserAuth = z.infer<typeof userAuthSchema>;

export const userGenderSchema = z.enum(["none", "male", "female", "other"]);

export type UserGender = z.infer<typeof userGenderSchema>;

export const signupReqSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8).max(72),
  name: z.string().min(1).max(50),
  terms_service: z.boolean(),
  terms_personal: z.boolean(),
  nickname: z.string().min(1).max(100),
  language: z.string().min(2).max(2),
  country: z.string().min(2).max(50),
  birthday: z.string().date(),
  gender: userGenderSchema,
});

export type SignupReq = z.infer<typeof signupReqSchema>;

export const signupResSchema = z.object({
  user_id: z.number().int(),
  email: z.string(),
  name: z.string(),
  nickname: z.string(),
  language: z.string(),
  country: z.string(),
  birthday: z.string().date(),
  gender: userGenderSchema,
  user_state: z.boolean(),
  user_auth: userAuthSchema,
  created_at: z.string().datetime(),
  access: accessTokenResSchema,
  session_id: z.string(),
});

export type SignupRes = z.infer<typeof signupResSchema>;

export const profileResSchema = z.object({
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
});

export type ProfileRes = z.infer<typeof profileResSchema>;

export const userDetailSchema = z.object({
  user_id: z.number().int(),
  email: z.string().email(),
  name: z.string(),
  nickname: z.string(),
  user_auth: z.string(),
  created_at: z.string().datetime(),
  bio: z.string().optional(),
});

export type UserDetail = z.infer<typeof userDetailSchema>;

export const userSettingSchema = z.object({
  user_id: z.number().int(),
  theme: z.enum(["light", "dark", "system"]),
  is_email_marketing_agreed: z.boolean(),
  language: z.enum(["ko", "en"]),
});

export type UserSetting = z.infer<typeof userSettingSchema>;

export const updateUserReqSchema = z.object({
  nickname: z.string().min(1).max(100).optional(),
  name: z.string().min(1).max(100).optional(),
  bio: z.string().max(500).optional(),
});

export type UpdateUserReq = z.infer<typeof updateUserReqSchema>;

export const profileUpdateReqSchema = z.object({
  nickname: z.string().min(1).max(100).optional(),
  language: z.string().min(1).max(50).optional(),
  country: z.string().min(1).max(50).optional(),
  birthday: z.string().date().optional(),
  gender: userGenderSchema.optional(),
});

export type ProfileUpdateReq = z.infer<typeof profileUpdateReqSchema>;

export const studyLangItemSchema = z.object({
  lang_code: z.string().min(2).max(2),
  priority: z.number().int().min(1),
  is_primary: z.boolean(),
});

export type StudyLangItem = z.infer<typeof studyLangItemSchema>;

export const settingsResSchema = z.object({
  user_set_language: z.string(),
  user_set_timezone: z.string(),
  user_set_note_email: z.boolean(),
  user_set_note_push: z.boolean(),
  updated_at: z.string().datetime(),
});

export type SettingsRes = z.infer<typeof settingsResSchema>;

export const settingsUpdateReqSchema = z.object({
  user_set_language: z.string().min(2).max(2).optional(),
  user_set_timezone: z.string().min(1).optional(),
  user_set_note_email: z.boolean().optional(),
  user_set_note_push: z.boolean().optional(),
});

export type SettingsUpdateReq = z.infer<typeof settingsUpdateReqSchema>;
