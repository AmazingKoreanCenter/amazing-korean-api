import type { UserDetail } from "@/category/user/types";
import { request } from "@/api/client";

export const getUserMe = () => {
  return request<UserDetail>("/users/me");
};
