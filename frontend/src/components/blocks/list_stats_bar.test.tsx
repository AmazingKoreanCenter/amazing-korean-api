import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { Users } from "lucide-react";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => {
      if (key === "common.page") return "페이지";
      if (key === "common.loading") return "불러오는 중";
      return key;
    },
  }),
}));

import { ListStatsBar } from "./list_stats_bar";

describe("ListStatsBar", () => {
  it("renders totalLabel and current/total page with translated suffix", () => {
    render(
      <ListStatsBar
        icon={Users}
        totalLabel="총 100명"
        currentPage={2}
        totalPages={10}
      />,
    );
    expect(screen.getByText("총 100명")).toBeInTheDocument();
    expect(screen.getByText("2 / 10 페이지")).toBeInTheDocument();
  });

  it("shows loading indicator when isFetching=true", () => {
    render(
      <ListStatsBar
        icon={Users}
        totalLabel="t"
        currentPage={1}
        totalPages={1}
        isFetching
      />,
    );
    expect(screen.getByText("불러오는 중")).toBeInTheDocument();
  });

  it("hides loading indicator when isFetching is falsy", () => {
    render(
      <ListStatsBar
        icon={Users}
        totalLabel="t"
        currentPage={1}
        totalPages={1}
      />,
    );
    expect(screen.queryByText("불러오는 중")).toBeNull();
  });

  it("merges custom className with the base wrapper", () => {
    const { container } = render(
      <ListStatsBar
        icon={Users}
        totalLabel="t"
        currentPage={1}
        totalPages={1}
        className="my-extra"
      />,
    );
    const root = container.firstChild as HTMLElement;
    expect(root.className).toContain("my-extra");
    expect(root.className).toContain("flex");
  });
});
