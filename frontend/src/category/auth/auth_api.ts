import { request } from "@/api/client";
import type { SignupReq, SignupRes } from "@/category/user/types";

export const signup = (data: SignupReq) => {
  return request<SignupRes>("/api/users", {
    method: "POST",
    body: JSON.stringify(data),
  });
};
