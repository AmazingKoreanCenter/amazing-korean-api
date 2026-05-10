import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { useAuthStore } from "./use_auth_store";
import type { LoginRes } from "@/category/auth/types";

const baseLoginRes = (overrides?: Partial<LoginRes>): LoginRes => ({
  user_id: 42,
  access: {
    access_token: "tok-abc",
    expires_in: 900,
  },
  session_id: "sess-xyz",
  ...overrides,
});

describe("useAuthStore", () => {
  beforeEach(() => {
    useAuthStore.setState({
      user: null,
      accessToken: null,
      isLoggedIn: false,
    });
  });

  afterEach(() => {
    localStorage.clear();
  });

  it("starts with initial state (no user, no token, not logged in)", () => {
    const state = useAuthStore.getState();
    expect(state.user).toBeNull();
    expect(state.accessToken).toBeNull();
    expect(state.isLoggedIn).toBe(false);
  });

  it("login() extracts user_id + access_token and flips isLoggedIn", () => {
    useAuthStore.getState().login(baseLoginRes());
    const state = useAuthStore.getState();
    expect(state.user).toEqual({ user_id: 42 });
    expect(state.accessToken).toBe("tok-abc");
    expect(state.isLoggedIn).toBe(true);
  });

  it("login() with missing access falls back to null token but still marks logged in", () => {
    useAuthStore.getState().login(baseLoginRes({ access: undefined as unknown as LoginRes["access"] }));
    const state = useAuthStore.getState();
    expect(state.user).toEqual({ user_id: 42 });
    expect(state.accessToken).toBeNull();
    expect(state.isLoggedIn).toBe(true);
  });

  it("logout() resets state to initial values", () => {
    useAuthStore.getState().login(baseLoginRes());
    useAuthStore.getState().logout();
    const state = useAuthStore.getState();
    expect(state.user).toBeNull();
    expect(state.accessToken).toBeNull();
    expect(state.isLoggedIn).toBe(false);
  });

  it("logout() clears persisted storage (auth-storage key)", () => {
    useAuthStore.getState().login(baseLoginRes());
    expect(localStorage.getItem("auth-storage")).not.toBeNull();
    useAuthStore.getState().logout();
    expect(localStorage.getItem("auth-storage")).toBeNull();
  });
});
