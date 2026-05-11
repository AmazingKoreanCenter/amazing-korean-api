import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MemoryRouter } from "react-router-dom";
import { ErrorPage } from "./error_page";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => {
      if (key === "error.errorPageTitle") return "오류가 발생했습니다";
      if (key === "error.errorPageDescription") return "잠시 후 다시 시도\n또는 문의";
      if (key === "error.errorPageContact") return "문의: support@example.com";
      if (key === "common.retry") return "다시 시도";
      if (key === "common.goHome") return "홈으로";
      return key;
    },
    i18n: { language: "ko" },
  }),
}));

const renderPage = () =>
  render(
    <MemoryRouter>
      <ErrorPage />
    </MemoryRouter>,
  );

describe("ErrorPage", () => {
  it("renders the Error badge + title + contact line", () => {
    renderPage();
    expect(screen.getByText("Error")).toBeInTheDocument();
    expect(
      screen.getByRole("heading", { level: 1, name: "오류가 발생했습니다" }),
    ).toBeInTheDocument();
    expect(screen.getByText("문의: support@example.com")).toBeInTheDocument();
  });

  it("calls window.location.reload when '다시 시도' button is clicked", async () => {
    const reloadSpy = vi.fn();
    // jsdom 의 window.location.reload 를 spy 로 교체.
    Object.defineProperty(window, "location", {
      value: { ...window.location, reload: reloadSpy },
      writable: true,
    });

    const user = userEvent.setup();
    renderPage();
    await user.click(screen.getByRole("button", { name: "다시 시도" }));
    expect(reloadSpy).toHaveBeenCalledTimes(1);
  });

  it("renders '홈으로' link to /", () => {
    renderPage();
    const homeLink = screen.getByRole("link", { name: /홈으로/ });
    expect(homeLink).toHaveAttribute("href", "/");
  });
});
