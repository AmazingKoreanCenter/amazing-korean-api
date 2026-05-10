import { describe, expect, it } from "vitest";
import {
  isCJK,
  isRTL,
  isTallScript,
  needsRelaxedTracking,
  LANG_CLASSES,
} from "./language_groups";

describe("language_groups", () => {
  describe("isCJK", () => {
    it("returns true for CJK languages (ko/ja/zh-CN/zh-TW)", () => {
      expect(isCJK("ko")).toBe(true);
      expect(isCJK("ja")).toBe(true);
      expect(isCJK("zh-CN")).toBe(true);
      expect(isCJK("zh-TW")).toBe(true);
    });

    it("returns false for non-CJK languages", () => {
      expect(isCJK("en")).toBe(false);
      expect(isCJK("ar")).toBe(false);
      expect(isCJK("th")).toBe(false);
      expect(isCJK("zh")).toBe(false);
    });
  });

  describe("isTallScript", () => {
    it("returns true for tall-script languages (th/my/km)", () => {
      expect(isTallScript("th")).toBe(true);
      expect(isTallScript("my")).toBe(true);
      expect(isTallScript("km")).toBe(true);
    });

    it("returns false for non-tall-script languages", () => {
      expect(isTallScript("ko")).toBe(false);
      expect(isTallScript("hi")).toBe(false);
      expect(isTallScript("en")).toBe(false);
    });
  });

  describe("needsRelaxedTracking", () => {
    it("returns true for tall-script + Indic + Mongolian languages", () => {
      expect(needsRelaxedTracking("th")).toBe(true);
      expect(needsRelaxedTracking("my")).toBe(true);
      expect(needsRelaxedTracking("km")).toBe(true);
      expect(needsRelaxedTracking("si")).toBe(true);
      expect(needsRelaxedTracking("hi")).toBe(true);
      expect(needsRelaxedTracking("ne")).toBe(true);
      expect(needsRelaxedTracking("mn")).toBe(true);
    });

    it("returns false for languages with default tracking", () => {
      expect(needsRelaxedTracking("ko")).toBe(false);
      expect(needsRelaxedTracking("en")).toBe(false);
      expect(needsRelaxedTracking("ar")).toBe(false);
    });
  });

  describe("isRTL", () => {
    it("returns true for ar/fa/ur", () => {
      expect(isRTL("ar")).toBe(true);
      expect(isRTL("fa")).toBe(true);
      expect(isRTL("ur")).toBe(true);
    });

    it("returns false for LTR languages", () => {
      expect(isRTL("en")).toBe(false);
      expect(isRTL("ko")).toBe(false);
      expect(isRTL("he")).toBe(false);
    });
  });

  describe("LANG_CLASSES", () => {
    it("exposes the four CSS class tokens used in the global lang switch", () => {
      expect(LANG_CLASSES).toEqual([
        "lang-cjk",
        "lang-tall-script",
        "lang-relaxed-tracking",
        "lang-rtl",
      ]);
    });
  });
});
