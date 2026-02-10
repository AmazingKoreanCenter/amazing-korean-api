import { create } from "zustand";
import { persist } from "zustand/middleware";

import type { LoginRes } from "@/category/auth/types";

type AuthState = {
  user: Pick<LoginRes, "user_id"> | null;
  accessToken: string | null;
  isLoggedIn: boolean;
  login: (data: LoginRes) => void;
  logout: () => void;
};

const initialState: Pick<AuthState, "user" | "accessToken" | "isLoggedIn"> = {
  user: null,
  accessToken: null,
  isLoggedIn: false,
};

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      ...initialState,
      login: (data) => {
        set({
          user: { user_id: data.user_id },
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
