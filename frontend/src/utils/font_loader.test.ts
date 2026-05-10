import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { loadFontForLanguage } from "./font_loader";

describe("loadFontForLanguage", () => {
  beforeEach(() => {
    document.head.innerHTML = "";
  });

  afterEach(() => {
    document.head.innerHTML = "";
  });

  it("appends a <link> tag with the configured href + id for known languages", () => {
    loadFontForLanguage("ja");
    const link = document.getElementById("font-ja") as HTMLLinkElement | null;
    expect(link).not.toBeNull();
    expect(link?.tagName).toBe("LINK");
    expect(link?.rel).toBe("stylesheet");
    expect(link?.href).toContain("Noto+Sans+JP");
    expect(link?.crossOrigin).toBe("anonymous");
  });

  it("does nothing for unknown languages (no <link> appended)", () => {
    const before = document.head.children.length;
    loadFontForLanguage("ko");
    loadFontForLanguage("en");
    loadFontForLanguage("xx-not-real");
    expect(document.head.children.length).toBe(before);
  });

  it("does not append a duplicate <link> when called twice for the same language", () => {
    loadFontForLanguage("th");
    loadFontForLanguage("th");
    const links = document.querySelectorAll("link#font-th");
    expect(links.length).toBe(1);
  });

  it("uses Nastaliq Urdu (no weight 500) for ur per Google Fonts limit", () => {
    loadFontForLanguage("ur");
    const link = document.getElementById("font-ur") as HTMLLinkElement | null;
    expect(link?.href).toContain("Noto+Nastaliq+Urdu");
    expect(link?.href).toContain("wght@400;700");
    expect(link?.href).not.toContain("500");
  });

  it("uses shared Devanagari font for hi and ne", () => {
    loadFontForLanguage("hi");
    loadFontForLanguage("ne");
    const hi = document.getElementById("font-hi") as HTMLLinkElement | null;
    const ne = document.getElementById("font-ne") as HTMLLinkElement | null;
    expect(hi?.href).toContain("Noto+Sans+Devanagari");
    expect(ne?.href).toContain("Noto+Sans+Devanagari");
  });
});
