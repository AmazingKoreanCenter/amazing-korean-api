import { describe, expect, it } from "vitest";
import { ApiError } from "./client";

describe("ApiError", () => {
  it("extends Error and exposes status + message", () => {
    const err = new ApiError(404, "not found");
    expect(err).toBeInstanceOf(Error);
    expect(err).toBeInstanceOf(ApiError);
    expect(err.status).toBe(404);
    expect(err.message).toBe("not found");
  });

  it("sets name to 'ApiError' (distinguishable from generic Error)", () => {
    const err = new ApiError(500, "boom");
    expect(err.name).toBe("ApiError");
  });

  it("preserves the stack trace", () => {
    const err = new ApiError(401, "unauthorized");
    expect(typeof err.stack).toBe("string");
    expect(err.stack).toContain("ApiError");
  });

  it("can be caught as Error and narrowed via instanceof", () => {
    let caught: unknown;
    try {
      throw new ApiError(409, "conflict");
    } catch (e) {
      caught = e;
    }
    expect(caught).toBeInstanceOf(ApiError);
    if (caught instanceof ApiError) {
      expect(caught.status).toBe(409);
    }
  });
});
