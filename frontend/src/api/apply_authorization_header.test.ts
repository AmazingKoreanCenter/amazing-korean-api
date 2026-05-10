import { describe, expect, it, vi } from "vitest";
import { applyAuthorizationHeader } from "./apply_authorization_header";

describe("applyAuthorizationHeader", () => {
  it("returns a fresh object with Authorization when headers is undefined", () => {
    const out = applyAuthorizationHeader(undefined, "Bearer t");
    expect(out).toEqual({ Authorization: "Bearer t" });
  });

  it("returns a fresh object with Authorization when headers is null", () => {
    const out = applyAuthorizationHeader(null, "Bearer t");
    expect(out).toEqual({ Authorization: "Bearer t" });
  });

  it("calls .set() in-place when headers exposes a set function (axios Headers)", () => {
    const set = vi.fn();
    const headers = { set };
    const out = applyAuthorizationHeader(headers, "Bearer t");
    expect(set).toHaveBeenCalledTimes(1);
    expect(set).toHaveBeenCalledWith("Authorization", "Bearer t");
    expect(out).toBe(headers);
  });

  it("merges Authorization into a plain object (returns a copy)", () => {
    const headers = { "X-Other": "ok" } as Record<string, string>;
    const out = applyAuthorizationHeader(headers, "Bearer t") as Record<string, string>;
    expect(out).toEqual({ "X-Other": "ok", Authorization: "Bearer t" });
    expect(out).not.toBe(headers);
  });

  it("overrides an existing Authorization value on a plain object", () => {
    const headers = { Authorization: "Bearer old" } as Record<string, string>;
    const out = applyAuthorizationHeader(headers, "Bearer new") as Record<string, string>;
    expect(out.Authorization).toBe("Bearer new");
  });
});
