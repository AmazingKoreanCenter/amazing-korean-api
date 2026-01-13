import type { UpdateUserReq, UserDetail } from "@/category/user/types";
import { request } from "@/api/client";

export const getUserMe = () => {
  return request<UserDetail>("/users/me");
};

export const updateUserMe = (data: UpdateUserReq) => {
  return request<void>("/users/me", {
    method: "POST",
    data,
  });
};
