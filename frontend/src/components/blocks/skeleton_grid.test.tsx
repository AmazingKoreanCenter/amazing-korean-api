import { describe, expect, it } from "vitest";
import { render } from "@testing-library/react";
import { SkeletonGrid } from "./skeleton_grid";

describe("SkeletonGrid", () => {
  it("renders the requested number of skeleton cards (video-card)", () => {
    const { container } = render(
      <SkeletonGrid count={6} variant="video-card" />,
    );
    const grid = container.firstChild as HTMLElement;
    expect(grid.children.length).toBe(6);
  });

  it("renders zero cards when count=0 (no children)", () => {
    const { container } = render(
      <SkeletonGrid count={0} variant="video-card" />,
    );
    const grid = container.firstChild as HTMLElement;
    expect(grid.children.length).toBe(0);
  });

  it("uses StudyCardSkeleton shape for variant=study-card (no aspect-video img placeholder)", () => {
    const { container } = render(
      <SkeletonGrid count={1} variant="study-card" />,
    );
    expect(container.querySelector(".aspect-video")).toBeNull();
  });

  it("uses CardWithImageSkeleton (aspect-video) for variant=video-card and content-card", () => {
    const video = render(<SkeletonGrid count={1} variant="video-card" />);
    expect(video.container.querySelector(".aspect-video")).not.toBeNull();
    video.unmount();
    const content = render(<SkeletonGrid count={1} variant="content-card" />);
    expect(content.container.querySelector(".aspect-video")).not.toBeNull();
  });

  it("applies grid column class corresponding to columns prop (default=3)", () => {
    const { container } = render(
      <SkeletonGrid count={1} variant="video-card" />,
    );
    const grid = container.firstChild as HTMLElement;
    expect(grid.className).toContain("md:grid-cols-3");
  });

  it("respects columns=2 and columns=4 overrides", () => {
    const two = render(
      <SkeletonGrid count={1} variant="video-card" columns={2} />,
    );
    expect((two.container.firstChild as HTMLElement).className).toContain("md:grid-cols-2");
    two.unmount();
    const four = render(
      <SkeletonGrid count={1} variant="video-card" columns={4} />,
    );
    expect((four.container.firstChild as HTMLElement).className).toContain("md:grid-cols-4");
  });
});
