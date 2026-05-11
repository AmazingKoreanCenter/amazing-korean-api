import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MemoryRouter } from "react-router-dom";
import { AccessDeniedPage } from "./access_denied_page";

const mockNavigate = vi.fn();
vi.mock("react-router-dom", async () => {
  const actual = await vi.importActual<typeof import("react-router-dom")>(
    "react-router-dom",
  );
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  };
});

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => {
      if (key === "error.accessDeniedTitle") return "접근 거부";
      if (key === "error.accessDeniedDescription") return "권한이 없습니다\n관리자에게 문의";
      if (key === "error.accessDeniedContact") return "문의: support@example.com";
      if (key === "common.previousPage") return "이전 페이지";
      if (key === "common.goHome") return "홈으로";
      return key;
    },
    i18n: { language: "ko" },
  }),
}));

const renderPage = () =>
  render(
    <MemoryRouter>
      <AccessDeniedPage />
    </MemoryRouter>,
  );

describe("AccessDeniedPage", () => {
  it("renders the 403 badge + title + contact line", () => {
    renderPage();
    expect(screen.getByText("403")).toBeInTheDocument();
    expect(
      screen.getByRole("heading", { level: 1, name: "접근 거부" }),
    ).toBeInTheDocument();
    expect(screen.getByText("문의: support@example.com")).toBeInTheDocument();
  });

  it("navigates back when '이전 페이지' button is clicked", async () => {
    mockNavigate.mockClear();
    const user = userEvent.setup();
    renderPage();
    await user.click(screen.getByRole("button", { name: "이전 페이지" }));
    expect(mockNavigate).toHaveBeenCalledWith(-1);
  });

  it("renders '홈으로' link to /", () => {
    renderPage();
    const homeLink = screen.getByRole("link", { name: /홈으로/ });
    expect(homeLink).toHaveAttribute("href", "/");
  });
});
