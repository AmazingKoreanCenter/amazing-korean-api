import { request } from "@/api/client";
import type {
  FindIdReq,
  GoogleAuthUrlRes,
  LoginReq,
  LoginRes,
  ResetPasswordReq,
  SignupReq,
  SignupRes,
} from "@/category/auth/types";

// URL Prefix 정책: client.ts baseURL에 '/api'가 있다면 아래에서 '/api'를 빼야 합니다.
// 여기서는 안전하게 기존 코드를 존중하되, Signup 경로만 체크합니다.

export const login = (data: LoginReq) => {
  return request<LoginRes>("/auth/login", {
    method: "POST",
    data,
    // 로그인 요청은 토큰 갱신 인터셉터를 건너뜀
    // (소셜 전용 계정 등 401 에러 메시지를 그대로 전달받기 위함)
    skipAuthRefresh: true,
  });
};

export const signup = (data: SignupReq) => {
  // 백엔드 엔드포인트가 POST /users 인지 POST /auth/signup 인지 확인 필요!
  // RESTful 하다면 /users가 맞습니다.
  return request<SignupRes>("/users", { 
    method: "POST",
    data,
  });
};

export const findId = (data: FindIdReq) => {
  return request<void>("/auth/find-id", {
    method: "POST",
    data,
  });
};

export const resetPassword = (data: ResetPasswordReq) => {
  return request<void>("/auth/reset-pw", {
    method: "POST",
    data,
  });
};

export const logout = () => {
  // 인자 제거: 토큰 헤더는 client interceptor가 알아서 붙입니다.
  return request<void>("/auth/logout", {
    method: "POST",
  });
};

// ==========================================
// Google OAuth
// ==========================================

export const getGoogleAuthUrl = () => {
  return request<GoogleAuthUrlRes>("/auth/google");
};

// OAuth 콜백 후 토큰 갱신 (쿠키의 refresh token 사용)
export const refreshToken = () => {
  return request<LoginRes>("/auth/refresh", {
    method: "POST",
  });
};