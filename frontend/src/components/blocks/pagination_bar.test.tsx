import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";

vi.stubGlobal("scrollTo", vi.fn());

import { PaginationBar } from "./pagination_bar";

describe("PaginationBar", () => {
  it("renders nothing when totalPages <= 1", () => {
    const { container } = render(
      <PaginationBar currentPage={1} totalPages={1} onPageChange={() => {}} />,
    );
    expect(container.firstChild).toBeNull();
  });

  it("disables Previous on first page and enables Next", () => {
    render(
      <PaginationBar currentPage={1} totalPages={5} onPageChange={() => {}} />,
    );
    expect(screen.getByLabelText("Go to previous page")).toHaveAttribute(
      "aria-disabled",
      "true",
    );
    expect(screen.getByLabelText("Go to next page")).toHaveAttribute(
      "aria-disabled",
      "false",
    );
  });

  it("disables Next on last page", () => {
    render(
      <PaginationBar currentPage={5} totalPages={5} onPageChange={() => {}} />,
    );
    expect(screen.getByLabelText("Go to next page")).toHaveAttribute(
      "aria-disabled",
      "true",
    );
  });

  it("marks the current page link with aria-current=page", () => {
    render(
      <PaginationBar currentPage={3} totalPages={5} onPageChange={() => {}} />,
    );
    const current = screen.getByText("3");
    expect(current).toHaveAttribute("aria-current", "page");
  });

  it("calls onPageChange with the clicked page number", async () => {
    const onPageChange = vi.fn();
    const user = userEvent.setup();
    render(
      <PaginationBar
        currentPage={1}
        totalPages={5}
        onPageChange={onPageChange}
      />,
    );
    await user.click(screen.getByText("3"));
    expect(onPageChange).toHaveBeenCalledTimes(1);
    expect(onPageChange).toHaveBeenCalledWith(3);
  });

  it("does not call onPageChange when clicking the current page", async () => {
    const onPageChange = vi.fn();
    const user = userEvent.setup();
    render(
      <PaginationBar
        currentPage={3}
        totalPages={5}
        onPageChange={onPageChange}
      />,
    );
    await user.click(screen.getByText("3"));
    expect(onPageChange).not.toHaveBeenCalled();
  });

  it("renders ellipsis markers when totalPages exceeds the compact threshold", () => {
    render(
      <PaginationBar currentPage={5} totalPages={10} onPageChange={() => {}} />,
    );
    const ellipses = screen.getAllByText("More pages");
    expect(ellipses.length).toBeGreaterThanOrEqual(1);
  });
});
