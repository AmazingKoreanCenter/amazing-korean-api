import { describe, expect, it } from "vitest";
import { cn } from "./utils";

describe("cn", () => {
  it("joins multiple class strings", () => {
    expect(cn("a", "b", "c")).toBe("a b c");
  });

  it("filters out falsy values (undefined, null, false, empty string)", () => {
    expect(cn("a", undefined, null, false, "", "b")).toBe("a b");
  });

  it("supports object syntax for conditional classes", () => {
    expect(cn("base", { active: true, disabled: false })).toBe("base active");
  });

  it("merges conflicting tailwind classes (last one wins)", () => {
    expect(cn("p-2", "p-4")).toBe("p-4");
    expect(cn("text-red-500", "text-blue-500")).toBe("text-blue-500");
  });

  it("preserves non-conflicting tailwind classes alongside merge", () => {
    const out = cn("p-2 text-sm", "p-4 font-bold");
    expect(out).toContain("p-4");
    expect(out).toContain("text-sm");
    expect(out).toContain("font-bold");
    expect(out).not.toContain("p-2");
  });

  it("returns empty string when called with no arguments", () => {
    expect(cn()).toBe("");
  });
});
