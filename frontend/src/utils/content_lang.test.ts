import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("i18next", () => ({
  default: {
    language: "en",
  },
}));

import i18n from "i18next";
import { getContentLang } from "./content_lang";

describe("getContentLang", () => {
  afterEach(() => {
    (i18n as { language: string }).language = "en";
  });

  it("returns undefined for ko (Korean source = no lang param)", () => {
    (i18n as { language: string }).language = "ko";
    expect(getContentLang()).toBeUndefined();
  });

  it("returns the locale code for non-ko languages", () => {
    (i18n as { language: string }).language = "en";
    expect(getContentLang()).toBe("en");

    (i18n as { language: string }).language = "ja";
    expect(getContentLang()).toBe("ja");

    (i18n as { language: string }).language = "zh-CN";
    expect(getContentLang()).toBe("zh-CN");
  });
});
