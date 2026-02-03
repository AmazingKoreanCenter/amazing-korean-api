import { z } from "zod";

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
  password: z.string().min(8).max(72),
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
  user_id: z.number().int(),
  email: z.string(),
  name: z.string(),
  nickname: z.string(),
  // 가입 성공 시 바로 로그인이 된다면 access 토큰이 있을 수 있음 (Optional)
  access: accessTokenResSchema.optional(),
});
export type SignupRes = z.infer<typeof signupResSchema>;

// ==========================================
// 3. 로그인 (Login)
// ==========================================

export const loginReqSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6).max(72),
  // 필요 시 유지, 불필요 시 삭제 가능
  device: z.string().optional(),
  browser: z.string().optional(),
  os: z.string().optional(),
  user_agent: z.string().optional(),
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
  token: z.string().min(1),
  new_password: z.string().min(6),
});
export type ResetPasswordReq = z.infer<typeof resetPasswordReqSchema>;

export const resetPwResSchema = z.object({
  message: z.string(),
});
export type ResetPwRes = z.infer<typeof resetPwResSchema>;

// ==========================================
// 5. 아이디 찾기 (Find ID)
// ==========================================

export const findIdReqSchema = z.object({
  name: z.string().min(1).max(100),
  email: z.string().email(),
});
export type FindIdReq = z.infer<typeof findIdReqSchema>;

export const findIdResSchema = z.object({
  message: z.string(),
});
export type FindIdRes = z.infer<typeof findIdResSchema>;

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