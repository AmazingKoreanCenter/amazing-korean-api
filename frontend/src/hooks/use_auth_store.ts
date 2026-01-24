import { create } from "zustand";
import { persist } from "zustand/middleware";

import type { LoginRes } from "@/category/auth/types";
import type { SignupRes } from "@/category/auth/types";

type StoredUser = Omit<SignupRes, "access"> | Pick<LoginRes, "user_id">;

type AuthState = {
  user: StoredUser | null;
  accessToken: string | null;
  isLoggedIn: boolean;
  login: (data: LoginRes | SignupRes) => void;
  logout: () => void;
};

const initialState: Pick<AuthState, "user" | "accessToken" | "isLoggedIn"> = {
  user: null,
  accessToken: null,
  isLoggedIn: false,
};

const getStoredUser = (data: LoginRes | SignupRes): StoredUser => {
  if ("email" in data) {
    const { access, ...user } = data;
    return user;
  }

  return { user_id: data.user_id };
};

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      ...initialState,
      login: (data) => {
        set({
          user: getStoredUser(data),
          accessToken: data.access?.access_token ?? null,
          isLoggedIn: true,
        });
      },
      logout: () => {
        set({ ...initialState });
        useAuthStore.persist.clearStorage();
      },
    }),
    {
      name: "auth-storage",
    }
  )
);
