import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, renderHook } from "@testing-library/react";

const changeLanguageMock = vi.fn<(lang: string) => Promise<void>>();
changeLanguageMock.mockResolvedValue(undefined);
vi.mock("@/i18n", () => ({
  changeLanguage: (lang: string) => changeLanguageMock(lang),
}));

const useUserSettingsMock = vi.fn();
vi.mock("@/category/user/hook/use_user_settings", () => ({
  useUserSettings: (opts?: { enabled?: boolean }) => useUserSettingsMock(opts),
}));

import { useAuthStore } from "./use_auth_store";
import { useLanguageSync } from "./use_language_sync";

describe("useLanguageSync", () => {
  beforeEach(() => {
    useAuthStore.setState({
      user: null,
      accessToken: null,
      isLoggedIn: false,
    });
    changeLanguageMock.mockClear();
    useUserSettingsMock.mockReset();
    useUserSettingsMock.mockReturnValue({ data: undefined });
  });

  afterEach(() => {
    localStorage.clear();
  });

  it("does not call changeLanguage when logged out", () => {
    useUserSettingsMock.mockReturnValue({ data: { user_set_language: "ja" } });
    renderHook(() => useLanguageSync());
    expect(changeLanguageMock).not.toHaveBeenCalled();
  });

  it("applies user_set_language once when logged in with settings", () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "t",
      isLoggedIn: true,
    });
    useUserSettingsMock.mockReturnValue({ data: { user_set_language: "ja" } });
    renderHook(() => useLanguageSync());
    expect(changeLanguageMock).toHaveBeenCalledTimes(1);
    expect(changeLanguageMock).toHaveBeenCalledWith("ja");
  });

  it("does not re-apply on rerender once already applied (appliedRef guard)", () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "t",
      isLoggedIn: true,
    });
    useUserSettingsMock.mockReturnValue({ data: { user_set_language: "ja" } });
    const { rerender } = renderHook(() => useLanguageSync());
    expect(changeLanguageMock).toHaveBeenCalledTimes(1);
    rerender();
    rerender();
    expect(changeLanguageMock).toHaveBeenCalledTimes(1);
  });

  it("resets the guard on logout so a subsequent login re-applies", () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "t",
      isLoggedIn: true,
    });
    useUserSettingsMock.mockReturnValue({ data: { user_set_language: "ja" } });
    const { rerender } = renderHook(() => useLanguageSync());
    expect(changeLanguageMock).toHaveBeenCalledTimes(1);

    act(() => {
      useAuthStore.setState({ isLoggedIn: false, user: null, accessToken: null });
    });
    rerender();

    act(() => {
      useAuthStore.setState({
        user: { user_id: 1 },
        accessToken: "t",
        isLoggedIn: true,
      });
    });
    rerender();
    expect(changeLanguageMock).toHaveBeenCalledTimes(2);
  });
});
