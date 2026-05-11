import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { HeroSection } from "./hero_section";

describe("HeroSection", () => {
  it("renders title (default variant = marketing, default size)", () => {
    render(<HeroSection title="Welcome" />);
    expect(screen.getByRole("heading", { level: 1, name: "Welcome" })).toBeInTheDocument();
  });

  it("renders optional badge, subtitle, and children", () => {
    render(
      <HeroSection
        title="T"
        badge={<span data-testid="badge">★</span>}
        subtitle="Sub"
      >
        <button data-testid="cta">Go</button>
      </HeroSection>,
    );
    expect(screen.getByTestId("badge")).toBeInTheDocument();
    expect(screen.getByText("Sub")).toBeInTheDocument();
    expect(screen.getByTestId("cta")).toBeInTheDocument();
  });

  it("omits badge / subtitle nodes when those props are not provided", () => {
    const { container } = render(<HeroSection title="OnlyTitle" />);
    // 마케팅 variant 의 badge 컨테이너 = rounded-full bg-background — 없을 때 미렌더.
    expect(container.querySelector(".rounded-full.bg-background")).toBeNull();
    // subtitle <p> 가 없으면 .max-w-2xl 도 없다.
    expect(container.querySelector("p.text-lg.max-w-2xl")).toBeNull();
  });

  it("size=sm uses py-section-md (vs default size=default 의 py-section-lg)", () => {
    const sm = render(<HeroSection title="T" size="sm" />);
    expect(sm.container.innerHTML).toContain("py-section-md");
    sm.unmount();
    const def = render(<HeroSection title="T" />);
    expect(def.container.innerHTML).toContain("py-section-lg");
  });

  it("variant=list renders the list layout (flex container instead of decorative blobs)", () => {
    const { container } = render(
      <HeroSection variant="list" title="ListTitle" badge={<span>b</span>} />,
    );
    // list variant = bg-hero-gradient border-b
    expect(container.querySelector("section.border-b")).not.toBeNull();
    // list variant = no decorative blob (marketing 만 사용)
    expect(container.querySelector(".blur-3xl")).toBeNull();
  });

  it("applies external className on the section element", () => {
    const { container } = render(
      <HeroSection title="T" className="custom-cls" />,
    );
    expect(container.querySelector("section.custom-cls")).not.toBeNull();
  });
});
