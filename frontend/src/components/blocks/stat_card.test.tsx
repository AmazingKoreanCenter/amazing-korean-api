import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { Users } from "lucide-react";
import { StatCard } from "./stat_card";

describe("StatCard", () => {
  it("renders label and numeric value with thousand separator", () => {
    render(<StatCard icon={Users} label="총 사용자" value={12345} />);
    expect(screen.getByText("총 사용자")).toBeInTheDocument();
    expect(screen.getByText("12,345")).toBeInTheDocument();
  });

  it("renders string value as-is (no formatting)", () => {
    render(<StatCard icon={Users} label="상태" value="ACTIVE" />);
    expect(screen.getByText("ACTIVE")).toBeInTheDocument();
  });

  it("falls back to '-' when value is undefined", () => {
    render(<StatCard icon={Users} label="empty" />);
    expect(screen.getByText("-")).toBeInTheDocument();
  });

  it("shows skeleton instead of value when loading", () => {
    const { container } = render(
      <StatCard icon={Users} label="loading" value={42} loading />,
    );
    expect(screen.queryByText("42")).toBeNull();
    expect(container.querySelector('[data-slot="skeleton"], .animate-pulse')).not.toBeNull();
  });

  it("renders zero correctly (not falsy fallback)", () => {
    render(<StatCard icon={Users} label="zero" value={0} />);
    expect(screen.getByText("0")).toBeInTheDocument();
  });
});
