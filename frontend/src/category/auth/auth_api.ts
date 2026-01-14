import { request } from "@/api/client";
import type {
  FindIdReq,
  LoginReq,
  LoginRes,
  ResetPasswordReq,
  SignupReq, // ✅ 이제 Auth 타입에서 가져옵니다
  SignupRes, // ✅ 이제 Auth 타입에서 가져옵니다
} from "@/category/auth/types";

// URL Prefix 정책: client.ts baseURL에 '/api'가 있다면 아래에서 '/api'를 빼야 합니다.
// 여기서는 안전하게 기존 코드를 존중하되, Signup 경로만 체크합니다.

export const login = (data: LoginReq) => {
  return request<LoginRes>("/auth/login", { // '/api' 제거 권장 (baseURL 확인 필요)
    method: "POST",
    data,
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