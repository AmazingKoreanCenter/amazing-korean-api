import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { CoverCard } from "./cover_card";

const baseProps = {
  imageSrc: "https://example.com/cover.png",
  imageAlt: "cover alt",
  title: "교재 1",
  subtitle: "기초 문법",
  actionLabel: "보기",
  onClick: () => {},
};

describe("CoverCard", () => {
  it("renders title, subtitle, action label and image with alt", () => {
    render(<CoverCard {...baseProps} />);
    expect(screen.getByText("교재 1")).toBeInTheDocument();
    expect(screen.getByText("기초 문법")).toBeInTheDocument();
    expect(screen.getByText("보기")).toBeInTheDocument();
    const img = screen.getByRole("img", { name: "cover alt" }) as HTMLImageElement;
    expect(img.src).toBe("https://example.com/cover.png");
  });

  it("uses lazy loading attribute on the cover image", () => {
    render(<CoverCard {...baseProps} />);
    const img = screen.getByRole("img", { name: "cover alt" });
    expect(img.getAttribute("loading")).toBe("lazy");
  });

  it("renders as a button element (semantic, focusable)", () => {
    render(<CoverCard {...baseProps} />);
    const btn = screen.getByRole("button");
    expect(btn.tagName).toBe("BUTTON");
    expect(btn.getAttribute("type")).toBe("button");
  });

  it("invokes onClick when the card is clicked", async () => {
    const onClick = vi.fn();
    const user = userEvent.setup();
    render(<CoverCard {...baseProps} onClick={onClick} />);
    await user.click(screen.getByRole("button"));
    expect(onClick).toHaveBeenCalledTimes(1);
  });
});
