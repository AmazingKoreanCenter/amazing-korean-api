import { describe, expect, it } from "vitest";
import { parseErrorMessage } from "./parse_error_message";

describe("parseErrorMessage", () => {
  describe("falsy data", () => {
    it("returns status-aware fallback when data is null and status given", () => {
      expect(parseErrorMessage(null, 500)).toBe("Request failed with status 500");
    });

    it("returns generic fallback when data is null and status missing", () => {
      expect(parseErrorMessage(null)).toBe("Request failed");
    });

    it("treats undefined the same as null (fallback)", () => {
      expect(parseErrorMessage(undefined, 404)).toBe("Request failed with status 404");
    });
  });

  describe("string data", () => {
    it("parses JSON string and extracts error.message (envelope path)", () => {
      const json = JSON.stringify({ error: { message: "이메일 없음" } });
      expect(parseErrorMessage(json, 401)).toBe("이메일 없음");
    });

    it("parses JSON string and falls back to top-level message when error.message missing", () => {
      const json = JSON.stringify({ message: "정합성 오류" });
      expect(parseErrorMessage(json, 422)).toBe("정합성 오류");
    });

    it("returns the raw string when JSON.parse throws", () => {
      expect(parseErrorMessage("not-json", 500)).toBe("not-json");
    });

    it("returns the raw JSON string when parsed envelope has no message fields", () => {
      const json = JSON.stringify({ unrelated: 1 });
      expect(parseErrorMessage(json, 500)).toBe(json);
    });

    it("returns fallback when raw string is empty", () => {
      expect(parseErrorMessage("", 503)).toBe("Request failed with status 503");
    });
  });

  describe("object data", () => {
    it("extracts error.message from object envelope", () => {
      expect(parseErrorMessage({ error: { message: "오류" } }, 400)).toBe("오류");
    });

    it("extracts top-level message when error.message missing", () => {
      expect(parseErrorMessage({ message: "msg only" }, 400)).toBe("msg only");
    });

    it("returns fallback when object has neither field", () => {
      expect(parseErrorMessage({ foo: "bar" }, 418)).toBe("Request failed with status 418");
    });

    it("ignores non-string error.message (e.g. number) and falls back", () => {
      expect(parseErrorMessage({ error: { message: 42 as unknown as string } }, 500)).toBe(
        "Request failed with status 500",
      );
    });

    it("ignores empty error.message and falls back", () => {
      expect(parseErrorMessage({ error: { message: "" } }, 500)).toBe(
        "Request failed with status 500",
      );
    });
  });
});
