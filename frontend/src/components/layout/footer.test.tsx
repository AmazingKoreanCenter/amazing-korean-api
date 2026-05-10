import { afterEach, describe, expect, it, vi } from "vitest";
import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { MemoryRouter } from "react-router-dom";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string, vars?: Record<string, unknown>) => {
      if (key === "footer.copyright") return `© ${vars?.year} Amazing Korean`;
      if (key === "common.brandName") return "Amazing Korean";
      if (key === "footer.brandDescription") return "한국어 학습 서비스";
      if (key === "footer.certificationCheck") return "인증 확인";
      if (key === "footer.certOnlineMarketing") return "온라인 마케팅";
      if (key === "footer.certAmazingKoreanCenter") return "센터 인증서";
      if (key === "footer.quickLinks") return "바로가기";
      if (key === "footer.support") return "지원";
      if (key === "footer.contact") return "문의";
      if (key === "footer.address") return "세종특별자치시";
      if (key === "footer.faq") return "자주 묻는 질문";
      if (key === "footer.terms") return "이용약관";
      if (key === "footer.privacy") return "개인정보처리방침";
      if (key === "footer.refundPolicy") return "환불정책";
      if (key === "footer.businessInfo") return "사업자등록정보";
      if (key === "footer.serviceIntro") return "서비스 소개";
      if (key === "footer.videoLearning") return "영상 학습";
      if (key === "footer.structuredLearning") return "구조화 학습";
      if (key === "footer.oneOnOneLesson") return "1:1 수업";
      return key;
    },
  }),
}));

import { Footer } from "./footer";

const renderFooter = () =>
  render(
    <MemoryRouter>
      <Footer />
    </MemoryRouter>,
  );

describe("Footer", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders the brand name and description", () => {
    renderFooter();
    expect(screen.getAllByText("Amazing Korean").length).toBeGreaterThanOrEqual(1);
    expect(screen.getByText("한국어 학습 서비스")).toBeInTheDocument();
  });

  it("renders the contact email as a mailto link", () => {
    renderFooter();
    const link = screen.getByRole("link", { name: /amazingkoreancenter@gmail\.com/ }) as HTMLAnchorElement;
    expect(link.getAttribute("href")).toBe("mailto:amazingkoreancenter@gmail.com");
  });

  it("renders quick-link entries pointing to internal routes", () => {
    renderFooter();
    expect(screen.getByRole("link", { name: "서비스 소개" })).toHaveAttribute("href", "/about");
    expect(screen.getByRole("link", { name: "영상 학습" })).toHaveAttribute("href", "/videos");
    expect(screen.getByRole("link", { name: "구조화 학습" })).toHaveAttribute("href", "/studies");
    expect(screen.getByRole("link", { name: "1:1 수업" })).toHaveAttribute("href", "/lessons");
  });

  it("renders the legal links (terms / privacy / refund) in the support section", () => {
    renderFooter();
    expect(screen.getByRole("link", { name: "자주 묻는 질문" })).toHaveAttribute("href", "/faq");
    expect(screen.getAllByRole("link", { name: "이용약관" })[0]).toHaveAttribute("href", "/terms");
    expect(screen.getAllByRole("link", { name: "개인정보처리방침" })[0]).toHaveAttribute("href", "/privacy");
    expect(screen.getByRole("link", { name: "환불정책" })).toHaveAttribute("href", "/refund-policy");
  });

  it("includes the current year in the copyright line", () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-12-25T00:00:00Z"));
    renderFooter();
    expect(screen.getByText(/© 2026 Amazing Korean/)).toBeInTheDocument();
  });

  it("opens the certification dialog when a cert button is clicked", async () => {
    const user = userEvent.setup();
    renderFooter();
    expect(screen.queryByRole("dialog")).toBeNull();
    await user.click(screen.getByRole("button", { name: /온라인 마케팅/ }));
    const dialog = await screen.findByRole("dialog");
    expect(dialog).toBeInTheDocument();
    const img = within(dialog).getByRole("img") as HTMLImageElement;
    expect(img.getAttribute("alt")).toBe("온라인 마케팅");
  });
});
