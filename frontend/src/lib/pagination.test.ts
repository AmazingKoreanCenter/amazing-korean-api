import { describe, expect, it } from "vitest";
import { ELLIPSIS, getPageItems, type PageItem } from "./pagination";

describe("getPageItems", () => {
  it("returns full sequential range when totalPages fits compact threshold", () => {
    expect(getPageItems(1, 5)).toEqual([1, 2, 3, 4, 5]);
    expect(getPageItems(3, 7)).toEqual([1, 2, 3, 4, 5, 6, 7]);
  });

  it("inserts trailing ellipsis when current page is near the start", () => {
    const items = getPageItems(1, 10);
    expect(items[0]).toBe(1);
    expect(items[items.length - 1]).toBe(10);
    expect(items).toContain(ELLIPSIS);
    const ellipsisCount = items.filter((p) => p === ELLIPSIS).length;
    expect(ellipsisCount).toBe(1);
  });

  it("inserts leading ellipsis when current page is near the end", () => {
    const items = getPageItems(10, 10);
    expect(items[0]).toBe(1);
    expect(items[items.length - 1]).toBe(10);
    expect(items).toContain(ELLIPSIS);
    const ellipsisCount = items.filter((p) => p === ELLIPSIS).length;
    expect(ellipsisCount).toBe(1);
  });

  it("inserts both leading and trailing ellipsis when current page is in the middle", () => {
    const items = getPageItems(5, 10);
    const ellipsisCount = items.filter((p) => p === ELLIPSIS).length;
    expect(ellipsisCount).toBe(2);
    expect(items[0]).toBe(1);
    expect(items[items.length - 1]).toBe(10);
    expect(items).toContain(4);
    expect(items).toContain(5);
    expect(items).toContain(6);
  });

  it("respects custom siblingCount by widening the visible window", () => {
    const items = getPageItems(5, 20, 2);
    const numericItems = items.filter(
      (p): p is number => typeof p === "number",
    );
    expect(numericItems).toContain(3);
    expect(numericItems).toContain(7);
    expect(items[0]).toBe(1);
    expect(items[items.length - 1]).toBe(20);
  });

  it("never duplicates 1 or totalPages even when window touches edges", () => {
    const items: PageItem[] = getPageItems(2, 10);
    const ones = items.filter((p) => p === 1).length;
    const tens = items.filter((p) => p === 10).length;
    expect(ones).toBe(1);
    expect(tens).toBe(1);
  });
});
