import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MemoryRouter } from "react-router-dom";
import { NotFoundPage } from "./not_found_page";

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
      if (key === "error.notFoundTitle") return "페이지를 찾을 수 없습니다";
      if (key === "error.notFoundDescription") return "URL 을 확인해주세요\n또는 홈으로 이동";
      if (key === "error.notFoundCheckUrl") return "주소가 정확한지 확인하세요";
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
      <NotFoundPage />
    </MemoryRouter>,
  );

describe("NotFoundPage", () => {
  it("renders the 404 badge + title + description", () => {
    renderPage();
    expect(screen.getByText("404")).toBeInTheDocument();
    expect(
      screen.getByRole("heading", { level: 1, name: "페이지를 찾을 수 없습니다" }),
    ).toBeInTheDocument();
    expect(screen.getByText("주소가 정확한지 확인하세요")).toBeInTheDocument();
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
