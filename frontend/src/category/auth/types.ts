import { z } from "zod";
import i18n from "@/i18n";

// ==========================================
// 1. 공통 Enum & Types (Signup 등에서 사용)
// ==========================================

// 액세스 토큰 응답 (공통)
export const accessTokenResSchema = z.object({
  access_token: z.string(),
  expires_in: z.number().int(),
});
export type AccessTokenRes = z.infer<typeof accessTokenResSchema>;

// 유저 권한 (회원가입 결과 등에 사용)
export const userAuthSchema = z.enum(["HYMN", "admin", "manager", "learner"]);
export type UserAuth = z.infer<typeof userAuthSchema>;

// 성별 (회원가입 시 사용)
export const userGenderSchema = z.enum(["none", "male", "female", "other"]);
export type UserGender = z.infer<typeof userGenderSchema>;

// ==========================================
// 2. 회원가입 (Signup) - [User에서 이사옴 ✅]
// ==========================================

export const signupReqSchema = z.object({
  email: z.string().email(),
  password: z.string()
    .min(8)
    .max(72)
    .regex(/[a-zA-Z]/, i18n.t("auth.validationPasswordLetter"))
    .regex(/[0-9]/, i18n.t("auth.validationPasswordDigit")),
  name: z.string().min(1).max(50),
  nickname: z.string().min(1).max(100),
  // 약관 동의 (백엔드 필드명 확인 필수: terms_service vs terms_agreed)
  terms_service: z.boolean(),
  terms_personal: z.boolean(),
  language: z.string().min(2).max(2),
  country: z.string().min(2).max(50),
  birthday: z.string().date(), // YYYY-MM-DD 형식
  gender: userGenderSchema,
});
export type SignupReq = z.infer<typeof signupReqSchema>;

export const signupResSchema = z.object({
  message: z.string(),
  requires_verification: z.boolean(),
});
export type SignupRes = z.infer<typeof signupResSchema>;

// ==========================================
// 3. 로그인 (Login)
// ==========================================

export const loginReqSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6).max(72),
});
export type LoginReq = z.infer<typeof loginReqSchema>;

export const loginResSchema = z.object({
  user_id: z.number().int(),
  access: accessTokenResSchema,
  session_id: z.string(),
});
export type LoginRes = z.infer<typeof loginResSchema>;

// ==========================================
// 4. 비밀번호 재설정 (Reset Password) - [통합 완료 ✅]
// ==========================================

export const resetPasswordReqSchema = z.object({
  reset_token: z.string().min(1),
  new_password: z.string()
    .min(8)
    .max(72)
    .regex(/[a-zA-Z]/, i18n.t("auth.validationPasswordLetter"))
    .regex(/[0-9]/, i18n.t("auth.validationPasswordDigit")),
});
export type ResetPasswordReq = z.infer<typeof resetPasswordReqSchema>;

export const resetPwResSchema = z.object({
  message: z.string(),
});
export type ResetPwRes = z.infer<typeof resetPwResSchema>;

// ==========================================
// 4.5. 비밀번호 재설정 요청/검증
// ==========================================

export const requestResetReqSchema = z.object({
  email: z.string().email(),
});
export type RequestResetReq = z.infer<typeof requestResetReqSchema>;

export const requestResetResSchema = z.object({
  message: z.string(),
  remaining_attempts: z.number(),
});
export type RequestResetRes = z.infer<typeof requestResetResSchema>;

export const verifyResetReqSchema = z.object({
  email: z.string().email(),
  code: z.string().min(6).max(6),
});
export type VerifyResetReq = z.infer<typeof verifyResetReqSchema>;

export const verifyResetResSchema = z.object({
  reset_token: z.string(),
  expires_in: z.number(),
});
export type VerifyResetRes = z.infer<typeof verifyResetResSchema>;

// ==========================================
// 5. 아이디 찾기 (Find ID)
// ==========================================

export const findIdReqSchema = z.object({
  name: z.string().min(1).max(100),
  birthday: z.string().date(),
});
export type FindIdReq = z.infer<typeof findIdReqSchema>;

export const findIdResSchema = z.object({
  message: z.string(),
  masked_emails: z.array(z.string()),
});
export type FindIdRes = z.infer<typeof findIdResSchema>;

// ==========================================
// 5.5. 비밀번호 찾기 (본인 확인 + 코드 발송)
// ==========================================

export const findPasswordReqSchema = z.object({
  name: z.string().min(1).max(100),
  birthday: z.string().date(),
  email: z.string().email(),
});
export type FindPasswordReq = z.infer<typeof findPasswordReqSchema>;

export const findPasswordResSchema = z.object({
  message: z.string(),
  remaining_attempts: z.number(),
});
export type FindPasswordRes = z.infer<typeof findPasswordResSchema>;

// ==========================================
// 6. 토큰 갱신 (Refresh)
// ==========================================

export const refreshReqSchema = z.object({
  refresh_token: z.string().min(1),
});
export type RefreshReq = z.infer<typeof refreshReqSchema>;

export const refreshResSchema = z.object({
  access_token: z.string(),
  expires_in: z.number().int(),
});
export type RefreshRes = z.infer<typeof refreshResSchema>;

// ==========================================
// 7. 로그아웃 (Logout)
// ==========================================

export const logoutAllReqSchema = z.object({
  everywhere: z.boolean().default(true),
});
export type LogoutAllReq = z.infer<typeof logoutAllReqSchema>;

export const logoutResSchema = z.object({
  ok: z.boolean(),
});
export type LogoutRes = z.infer<typeof logoutResSchema>;

// ==========================================
// 8. Google OAuth
// ==========================================

export const googleAuthUrlResSchema = z.object({
  auth_url: z.string().url(),
});
export type GoogleAuthUrlRes = z.infer<typeof googleAuthUrlResSchema>;

// ==========================================
// 9. Email Verification (회원가입 이메일 인증)
// ==========================================

export const verifyEmailReqSchema = z.object({
  email: z.string().email(),
  code: z.string().min(6).max(6),
});
export type VerifyEmailReq = z.infer<typeof verifyEmailReqSchema>;

export const verifyEmailResSchema = z.object({
  message: z.string(),
  verified: z.boolean(),
});
export type VerifyEmailRes = z.infer<typeof verifyEmailResSchema>;

export const resendVerificationReqSchema = z.object({
  email: z.string().email(),
});
export type ResendVerificationReq = z.infer<typeof resendVerificationReqSchema>;

export const resendVerificationResSchema = z.object({
  message: z.string(),
  remaining_attempts: z.number(),
});
export type ResendVerificationRes = z.infer<typeof resendVerificationResSchema>;