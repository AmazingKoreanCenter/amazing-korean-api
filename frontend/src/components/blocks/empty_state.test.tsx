import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { EmptyState } from "./empty_state";

describe("EmptyState", () => {
  it("renders title and icon (icon container marked aria-hidden)", () => {
    render(
      <EmptyState
        icon={<span data-testid="icon">🪺</span>}
        title="아직 항목이 없습니다"
      />,
    );
    expect(screen.getByRole("status")).toBeInTheDocument();
    expect(screen.getByText("아직 항목이 없습니다")).toBeInTheDocument();
    expect(screen.getByTestId("icon")).toBeInTheDocument();
  });

  it("renders description when provided", () => {
    render(
      <EmptyState
        icon={<span>i</span>}
        title="t"
        description="첫 항목을 추가하세요"
      />,
    );
    expect(screen.getByText("첫 항목을 추가하세요")).toBeInTheDocument();
  });

  it("omits description block when not provided", () => {
    render(<EmptyState icon={<span>i</span>} title="t" />);
    const status = screen.getByRole("status");
    expect(status.querySelector("p")).toBeNull();
  });

  it("renders action when provided", () => {
    render(
      <EmptyState
        icon={<span>i</span>}
        title="t"
        action={<button>추가</button>}
      />,
    );
    expect(screen.getByRole("button", { name: "추가" })).toBeInTheDocument();
  });

  it("merges custom className with the base classes", () => {
    render(
      <EmptyState
        icon={<span>i</span>}
        title="t"
        className="custom-extra"
      />,
    );
    const status = screen.getByRole("status");
    expect(status.className).toContain("custom-extra");
    expect(status.className).toContain("text-center");
  });
});
