import type {
  SettingsRes,
  SettingsUpdateReq,
  UpdateUserReq,
  UserDetail,
} from "@/category/user/types";
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

export const getUserSettings = () => {
  return request<SettingsRes>("/users/me/settings");
};

export const updateUserSettings = (data: SettingsUpdateReq) => {
  return request<void>("/users/me/settings", {
    method: "POST",
    data,
  });
};
