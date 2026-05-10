import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { SectionContainer } from "./section_container";

describe("SectionContainer", () => {
  it("renders children inside a <section> by default", () => {
    const { container } = render(
      <SectionContainer>
        <p>hello</p>
      </SectionContainer>,
    );
    const section = container.querySelector("section");
    expect(section).not.toBeNull();
    expect(screen.getByText("hello")).toBeInTheDocument();
  });

  it("renders as a different tag when as prop is provided", () => {
    const { container } = render(
      <SectionContainer as="article">
        <p>x</p>
      </SectionContainer>,
    );
    expect(container.querySelector("article")).not.toBeNull();
    expect(container.querySelector("section")).toBeNull();
  });

  it("applies py-section-md by default (size=md)", () => {
    const { container } = render(
      <SectionContainer>
        <p>x</p>
      </SectionContainer>,
    );
    expect((container.firstChild as HTMLElement).className).toContain("py-section-md");
  });

  it("applies size=sm and size=lg classes", () => {
    const sm = render(
      <SectionContainer size="sm">
        <p>x</p>
      </SectionContainer>,
    );
    expect((sm.container.firstChild as HTMLElement).className).toContain("py-section-sm");
    sm.unmount();
    const lg = render(
      <SectionContainer size="lg">
        <p>x</p>
      </SectionContainer>,
    );
    expect((lg.container.firstChild as HTMLElement).className).toContain("py-section-lg");
  });

  it("applies max-w-3xl when container=narrow (vs default max-w-container-default)", () => {
    const narrow = render(
      <SectionContainer container="narrow">
        <p>x</p>
      </SectionContainer>,
    );
    const inner = narrow.container.querySelector("div");
    expect(inner?.className).toContain("max-w-3xl");
  });
});
