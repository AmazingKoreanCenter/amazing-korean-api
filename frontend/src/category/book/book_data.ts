import type { BookInfo } from "./types";

export const BOOK_PRICE = "₩25,000";
export const BOOK_PAGES = 124;

const BOOK_MAP: Record<string, BookInfo> = {
  // English
  "9791199772700": { isbn13: "9791199772700", langKey: "en", i18nCode: "en", edition: "student", nameLocal: "English", nameKorean: "영어", sealColor: "#012169", flagFile: "emoji_u1f1ec_1f1e7.svg" },
  "9791199816206": { isbn13: "9791199816206", langKey: "en", i18nCode: "en", edition: "teacher", nameLocal: "English", nameKorean: "영어", sealColor: "#012169", flagFile: "emoji_u1f1ec_1f1e7.svg" },
  // 简体中文
  "9791199772717": { isbn13: "9791199772717", langKey: "zh_cn", i18nCode: "zh-CN", edition: "student", nameLocal: "简体中文", nameKorean: "중국어(간체)", sealColor: "#DE2910", flagFile: "emoji_u1f1e8_1f1f3.svg" },
  "9791199816213": { isbn13: "9791199816213", langKey: "zh_cn", i18nCode: "zh-CN", edition: "teacher", nameLocal: "简体中文", nameKorean: "중국어(간체)", sealColor: "#DE2910", flagFile: "emoji_u1f1e8_1f1f3.svg" },
  // 日本語
  "9791199772724": { isbn13: "9791199772724", langKey: "ja", i18nCode: "ja", edition: "student", nameLocal: "日本語", nameKorean: "일본어", sealColor: "#BC002D", flagFile: "emoji_u1f1ef_1f1f5.svg" },
  "9791199816220": { isbn13: "9791199816220", langKey: "ja", i18nCode: "ja", edition: "teacher", nameLocal: "日本語", nameKorean: "일본어", sealColor: "#BC002D", flagFile: "emoji_u1f1ef_1f1f5.svg" },
  // ภาษาไทย
  "9791199772731": { isbn13: "9791199772731", langKey: "th", i18nCode: "th", edition: "student", nameLocal: "ภาษาไทย", nameKorean: "태국어", sealColor: "#A51931", flagFile: "emoji_u1f1f9_1f1ed.svg" },
  "9791199816237": { isbn13: "9791199816237", langKey: "th", i18nCode: "th", edition: "teacher", nameLocal: "ภาษาไทย", nameKorean: "태국어", sealColor: "#A51931", flagFile: "emoji_u1f1f9_1f1ed.svg" },
  // नेपाली
  "9791199772748": { isbn13: "9791199772748", langKey: "ne", i18nCode: "ne", edition: "student", nameLocal: "नेपाली", nameKorean: "네팔어", sealColor: "#DC143C", flagFile: "emoji_u1f1f3_1f1f5.svg" },
  "9791199816244": { isbn13: "9791199816244", langKey: "ne", i18nCode: "ne", edition: "teacher", nameLocal: "नेपाली", nameKorean: "네팔어", sealColor: "#DC143C", flagFile: "emoji_u1f1f3_1f1f5.svg" },
  // Tiếng Việt
  "9791199772755": { isbn13: "9791199772755", langKey: "vi", i18nCode: "vi", edition: "student", nameLocal: "Tiếng Việt", nameKorean: "베트남어", sealColor: "#DA251D", flagFile: "emoji_u1f1fb_1f1f3.svg" },
  "9791199816251": { isbn13: "9791199816251", langKey: "vi", i18nCode: "vi", edition: "teacher", nameLocal: "Tiếng Việt", nameKorean: "베트남어", sealColor: "#DA251D", flagFile: "emoji_u1f1fb_1f1f3.svg" },
  // Русский
  "9791199772762": { isbn13: "9791199772762", langKey: "ru", i18nCode: "ru", edition: "student", nameLocal: "Русский", nameKorean: "러시아어", sealColor: "#0039A6", flagFile: "emoji_u1f1f7_1f1fa.svg" },
  "9791199816268": { isbn13: "9791199816268", langKey: "ru", i18nCode: "ru", edition: "teacher", nameLocal: "Русский", nameKorean: "러시아어", sealColor: "#0039A6", flagFile: "emoji_u1f1f7_1f1fa.svg" },
  // ភាសាខ្មែរ
  "9791199772779": { isbn13: "9791199772779", langKey: "km", i18nCode: "km", edition: "student", nameLocal: "ភាសាខ្មែរ", nameKorean: "크메르어", sealColor: "#032EA1", flagFile: "emoji_u1f1f0_1f1ed.svg" },
  "9791199816275": { isbn13: "9791199816275", langKey: "km", i18nCode: "km", edition: "teacher", nameLocal: "ភាសាខ្មែរ", nameKorean: "크메르어", sealColor: "#032EA1", flagFile: "emoji_u1f1f0_1f1ed.svg" },
  // Filipino
  "9791199772786": { isbn13: "9791199772786", langKey: "tl", i18nCode: "en", edition: "student", nameLocal: "Filipino", nameKorean: "필리핀어", sealColor: "#0038A8", flagFile: "emoji_u1f1f5_1f1ed.svg" },
  "9791199816282": { isbn13: "9791199816282", langKey: "tl", i18nCode: "en", edition: "teacher", nameLocal: "Filipino", nameKorean: "필리핀어", sealColor: "#0038A8", flagFile: "emoji_u1f1f5_1f1ed.svg" },
  // Bahasa Indonesia
  "9791199772793": { isbn13: "9791199772793", langKey: "id", i18nCode: "id", edition: "student", nameLocal: "Bahasa Indonesia", nameKorean: "인도네시아어", sealColor: "#FF0000", flagFile: "emoji_u1f1ee_1f1e9.svg" },
  "9791199816299": { isbn13: "9791199816299", langKey: "id", i18nCode: "id", edition: "teacher", nameLocal: "Bahasa Indonesia", nameKorean: "인도네시아어", sealColor: "#FF0000", flagFile: "emoji_u1f1ee_1f1e9.svg" },
};

/** 다른 언어 교재 링크용 (student 에디션 10개) */
export const ALL_STUDENT_BOOKS: BookInfo[] = Object.values(BOOK_MAP).filter(
  (b) => b.edition === "student",
);

/** ISBN → BookInfo 룩업 (하이픈 자동 제거) */
export function findBookByISBN(isbn: string): BookInfo | null {
  const normalized = isbn.replace(/-/g, "");
  return BOOK_MAP[normalized] ?? null;
}

/** ISBN 포맷팅 (979-11-997727-5-5) */
export function formatISBN(isbn13: string): string {
  if (isbn13.length !== 13) return isbn13;
  return `${isbn13.slice(0, 3)}-${isbn13.slice(3, 5)}-${isbn13.slice(5, 11)}-${isbn13.slice(11, 12)}-${isbn13.slice(12)}`;
}
