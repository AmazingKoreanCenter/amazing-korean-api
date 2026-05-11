import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { FeatureGrid } from "./feature_grid";

describe("FeatureGrid", () => {
  const items = [
    { icon: <span data-testid="icon-a">A</span>, title: "Title A", description: "Desc A" },
    { icon: <span data-testid="icon-b">B</span>, title: "Title B", description: "Desc B\nLine 2" },
  ];

  it("renders each item's title, description, and icon", () => {
    render(<FeatureGrid items={items} />);
    expect(screen.getByText("Title A")).toBeInTheDocument();
    expect(screen.getByText("Title B")).toBeInTheDocument();
    expect(screen.getByText("Desc A")).toBeInTheDocument();
    expect(screen.getByText(/Desc B/)).toBeInTheDocument();
    expect(screen.getByTestId("icon-a")).toBeInTheDocument();
    expect(screen.getByTestId("icon-b")).toBeInTheDocument();
  });

  it("renders an empty grid container when items is empty", () => {
    const { container } = render(<FeatureGrid items={[]} />);
    const grid = container.firstChild as HTMLElement;
    expect(grid.className).toContain("grid");
    expect(grid.children.length).toBe(0);
  });

  it("applies responsive grid classes (md:grid-cols-3)", () => {
    const { container } = render(<FeatureGrid items={items} />);
    const grid = container.firstChild as HTMLElement;
    expect(grid.className).toContain("md:grid-cols-3");
  });
});
