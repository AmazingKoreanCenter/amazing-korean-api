import { z } from "zod";

// ✅ [변경됨] Auth 모듈로 이사 간 공통 Enum을 가져옵니다. (중복 정의 제거)
import { 
  userAuthSchema, 
  userGenderSchema 
} from "../auth/types";

// ----------------------------------------------------------------------
// 🚨 주의: Signup(회원가입) 관련 코드는 src/category/auth/types.ts로 이동했습니다.
// ----------------------------------------------------------------------

// [User Profile Response] - (타인의 프로필 조회 등에 사용)
export const profileResSchema = z.object({
  id: z.number().int(),
  email: z.string(),
  name: z.string(),
  nickname: z.string().optional(),
  language: z.string().optional(),
  country: z.string().optional(),
  birthday: z.string().date().optional(),
  gender: userGenderSchema, // ✅ Auth에서 가져온 타입 사용
  user_state: z.boolean(),
  user_auth: userAuthSchema, // ✅ Auth에서 가져온 타입 사용
  created_at: z.string().datetime(),
});

export type ProfileRes = z.infer<typeof profileResSchema>;

// [User Detail] - 내 정보 조회 (GET /users/me)
// 백엔드 ProfileRes에 맞춤
export const userDetailSchema = z.object({
  id: z.number().int(),
  email: z.string().email(),
  name: z.string(),
  nickname: z.string().nullable(),
  language: z.string().nullable(),
  country: z.string().nullable(),
  birthday: z.string().nullable(),
  gender: userGenderSchema,
  user_state: z.boolean(),
  user_auth: userAuthSchema,
  created_at: z.string().datetime(),
  // OAuth 전용 계정 여부 판단용 (비밀번호가 설정되어 있으면 true)
  has_password: z.boolean().optional(),
});

export type UserDetail = z.infer<typeof userDetailSchema>;

// [Update User] - 내 정보 수정 (POST /users/me)
// 백엔드 ProfileUpdateReq에 맞춤 (name, bio는 백엔드에서 지원 안함)
export const updateUserReqSchema = z.object({
  nickname: z.string().min(1).max(100).optional(),
  language: z.string().min(1).max(50).optional(),
  country: z.string().min(1).max(50).optional(),
  birthday: z.string().optional(),
  gender: userGenderSchema.optional(),
});

export type UpdateUserReq = z.infer<typeof updateUserReqSchema>;

// [Profile Update] - (관리자 또는 상세 프로필 수정용)
export const profileUpdateReqSchema = z.object({
  nickname: z.string().min(1).max(100).optional(),
  language: z.string().min(1).max(50).optional(),
  country: z.string().min(1).max(50).optional(),
  birthday: z.string().date().optional(),
  gender: userGenderSchema.optional(), // ✅ Auth에서 가져온 타입 사용
});

export type ProfileUpdateReq = z.infer<typeof profileUpdateReqSchema>;

// [Study Lang] - 학습 언어 설정
export const studyLangItemSchema = z.object({
  lang_code: z.string().min(2).max(2),
  priority: z.number().int().min(1),
  is_primary: z.boolean(),
});

export type StudyLangItem = z.infer<typeof studyLangItemSchema>;

// [Settings] - 환경 설정 조회 (GET) - 어제 수정 완료된 부분 ✅
export const settingsResSchema = z.object({
  user_set_language: z.string(),
  user_set_timezone: z.string(),
  user_set_note_email: z.boolean(),
  user_set_note_push: z.boolean(),
  updated_at: z.string().datetime(),
});

export type SettingsRes = z.infer<typeof settingsResSchema>;

// [Settings Update] - 환경 설정 수정 (POST) - 어제 수정 완료된 부분 ✅
export const settingsUpdateReqSchema = z.object({
  user_set_language: z.string().min(2).max(5).optional(),
  user_set_timezone: z.string().min(1).optional(),
  user_set_note_email: z.boolean().optional(),
  user_set_note_push: z.boolean().optional(),
});

export type SettingsUpdateReq = z.infer<typeof settingsUpdateReqSchema>;