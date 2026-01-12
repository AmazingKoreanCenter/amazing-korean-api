import { request } from "@/api/client";
import type { LoginReq, LoginRes } from "@/category/auth/types";
import type { SignupReq, SignupRes } from "@/category/user/types";

export const login = (data: LoginReq) => {
  return request<LoginRes>("/api/auth/login", {
    method: "POST",
    body: JSON.stringify(data),
  });
};

export const signup = (data: SignupReq) => {
  return request<SignupRes>("/api/users", {
    method: "POST",
    body: JSON.stringify(data),
  });
};
