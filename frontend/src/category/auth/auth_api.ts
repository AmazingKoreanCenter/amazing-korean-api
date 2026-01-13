import { request } from "@/api/client";
import type { LoginReq, LoginRes } from "@/category/auth/types";
import type { SignupReq, SignupRes } from "@/category/user/types";

export const login = (data: LoginReq) => {
  return request<LoginRes>("/api/auth/login", {
    method: "POST",
    data,
  });
};

export const signup = (data: SignupReq) => {
  return request<SignupRes>("/api/users", {
    method: "POST",
    data,
  });
};

export const logout = (accessToken?: string | null) => {
  return request<void>("/api/auth/logout", {
    method: "POST",
    headers: accessToken
      ? {
          Authorization: `Bearer ${accessToken}`,
        }
      : undefined,
  });
};
