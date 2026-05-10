import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MemoryRouter } from "react-router-dom";

const i18nState = { language: "ko" };

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => {
      if (key === "nav.about") return "소개";
      if (key === "nav.book") return "교재";
      if (key === "nav.videos") return "영상";
      if (key === "nav.studies") return "학습";
      if (key === "nav.lessons") return "1:1 수업";
      if (key === "nav.myPage") return "마이페이지";
      if (key === "nav.login") return "로그인";
      if (key === "nav.signup") return "회원가입";
      return key;
    },
    i18n: i18nState,
  }),
}));

const changeLanguageMock = vi.fn();
vi.mock("@/i18n", () => ({
  changeLanguage: (lang: string) => changeLanguageMock(lang),
  SUPPORTED_LANGUAGES: [
    { code: "ko", nativeName: "한국어", flag: "🇰🇷" },
    { code: "en", nativeName: "English", flag: "🇺🇸" },
    { code: "ja", nativeName: "日本語", flag: "🇯🇵" },
  ],
  TIER_BREAK_INDICES: [] as readonly number[],
}));

const updateSettingsMutateMock = vi.fn();
vi.mock("@/category/user/hook/use_update_settings", () => ({
  useUpdateSettings: () => ({
    mutate: updateSettingsMutateMock,
    isPending: false,
  }),
}));

vi.mock("@/category/auth/components/logout_button", () => ({
  LogoutButton: () => <button type="button">로그아웃</button>,
}));

vi.mock("@/components/ui/theme_toggle", () => ({
  ThemeToggle: () => <button type="button" aria-label="theme">theme</button>,
}));

import { useAuthStore } from "@/hooks/use_auth_store";
import { Header } from "./header";

const renderHeader = () =>
  render(
    <MemoryRouter>
      <Header />
    </MemoryRouter>,
  );

describe("Header", () => {
  beforeEach(() => {
    useAuthStore.setState({
      user: null,
      accessToken: null,
      isLoggedIn: false,
    });
    changeLanguageMock.mockClear();
    updateSettingsMutateMock.mockClear();
    i18nState.language = "ko";
  });

  afterEach(() => {
    document.body.style.overflow = "";
    localStorage.clear();
  });

  it("renders the brand mark and the five primary nav links", () => {
    renderHeader();
    expect(screen.getByText("Amazing Korean")).toBeInTheDocument();
    const about = screen.getAllByRole("link", { name: "소개" });
    expect(about[0]).toHaveAttribute("href", "/about");
    const book = screen.getAllByRole("link", { name: "교재" });
    expect(book[0]).toHaveAttribute("href", "/book");
    const videos = screen.getAllByRole("link", { name: "영상" });
    expect(videos[0]).toHaveAttribute("href", "/videos");
    const studies = screen.getAllByRole("link", { name: "학습" });
    expect(studies[0]).toHaveAttribute("href", "/studies");
    const lessons = screen.getAllByRole("link", { name: "1:1 수업" });
    expect(lessons[0]).toHaveAttribute("href", "/lessons");
  });

  it("shows Login + Signup buttons when logged out (LogoutButton stub absent)", () => {
    renderHeader();
    expect(screen.getAllByRole("link", { name: "로그인" })[0]).toHaveAttribute(
      "href",
      "/login",
    );
    expect(screen.getAllByRole("link", { name: "회원가입" })[0]).toHaveAttribute(
      "href",
      "/signup",
    );
    expect(screen.queryAllByText("로그아웃").length).toBe(0);
  });

  it("shows MyPage link + LogoutButton stub when logged in", () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "t",
      isLoggedIn: true,
    });
    renderHeader();
    expect(screen.getAllByRole("link", { name: "마이페이지" })[0]).toHaveAttribute(
      "href",
      "/user/me",
    );
    expect(screen.getAllByText("로그아웃").length).toBeGreaterThanOrEqual(1);
    expect(screen.queryAllByRole("link", { name: "로그인" }).length).toBe(0);
  });

  it("toggles the mobile menu open + closed via the menu button", async () => {
    const user = userEvent.setup();
    renderHeader();
    const toggle = screen.getByRole("button", { name: /toggle menu/i });
    const findMaxHeightWrapper = () =>
      document.querySelector(".lg\\:hidden.overflow-hidden") as HTMLElement | null;
    expect(findMaxHeightWrapper()?.className).toContain("max-h-0");
    await user.click(toggle);
    expect(findMaxHeightWrapper()?.className).toContain("max-h-[400px]");
    await user.click(toggle);
    expect(findMaxHeightWrapper()?.className).toContain("max-h-0");
  });

  it("calls changeLanguage when a language item is clicked (logged out → no settings update)", async () => {
    const user = userEvent.setup();
    renderHeader();
    const triggers = screen.getAllByRole("button", { name: /한국어/ });
    await user.click(triggers[0]);
    const items = await screen.findAllByRole("menuitem");
    const en = items.find((el) => el.textContent?.includes("English"));
    expect(en).toBeTruthy();
    await user.click(en!);
    expect(changeLanguageMock).toHaveBeenCalledWith("en");
    expect(updateSettingsMutateMock).not.toHaveBeenCalled();
  });

  it("syncs the language to the user settings backend when logged in", async () => {
    useAuthStore.setState({
      user: { user_id: 1 },
      accessToken: "t",
      isLoggedIn: true,
    });
    const user = userEvent.setup();
    renderHeader();
    const triggers = screen.getAllByRole("button", { name: /한국어/ });
    await user.click(triggers[0]);
    const items = await screen.findAllByRole("menuitem");
    const ja = items.find((el) => el.textContent?.includes("日本語"));
    await user.click(ja!);
    expect(changeLanguageMock).toHaveBeenCalledWith("ja");
    expect(updateSettingsMutateMock).toHaveBeenCalledWith({
      user_set_language: "ja",
    });
  });
});
