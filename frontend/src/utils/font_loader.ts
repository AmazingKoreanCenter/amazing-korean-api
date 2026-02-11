/**
 * 언어별 폰트 동적 로딩 유틸리티
 *
 * - ko/en/Latin/Cyrillic 계열: Pretendard가 커버 (CDN으로 이미 로드)
 * - CJK (ja, zh-CN, zh-TW): Noto Sans JP/SC/TC
 * - 특수 스크립트: 각 Noto Sans 변형
 */

interface FontConfig {
  href: string;
  id: string;
}

const FONT_MAP: Record<string, FontConfig> = {
  ja: {
    href: "https://fonts.googleapis.com/css2?family=Noto+Sans+JP:wght@400;500;700&display=swap",
    id: "font-ja",
  },
  "zh-CN": {
    href: "https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@400;500;700&display=swap",
    id: "font-zh-cn",
  },
  "zh-TW": {
    href: "https://fonts.googleapis.com/css2?family=Noto+Sans+TC:wght@400;500;700&display=swap",
    id: "font-zh-tw",
  },
  th: {
    href: "https://fonts.googleapis.com/css2?family=Noto+Sans+Thai:wght@400;500;700&display=swap",
    id: "font-th",
  },
  my: {
    href: "https://fonts.googleapis.com/css2?family=Noto+Sans+Myanmar:wght@400;500;700&display=swap",
    id: "font-my",
  },
  km: {
    href: "https://fonts.googleapis.com/css2?family=Noto+Sans+Khmer:wght@400;500;700&display=swap",
    id: "font-km",
  },
  mn: {
    href: "https://fonts.googleapis.com/css2?family=Noto+Sans+Mongolian&display=swap",
    id: "font-mn",
  },
  hi: {
    href: "https://fonts.googleapis.com/css2?family=Noto+Sans+Devanagari:wght@400;500;700&display=swap",
    id: "font-hi",
  },
  ne: {
    href: "https://fonts.googleapis.com/css2?family=Noto+Sans+Devanagari:wght@400;500;700&display=swap",
    id: "font-ne",
  },
  si: {
    href: "https://fonts.googleapis.com/css2?family=Noto+Sans+Sinhala:wght@400;500;700&display=swap",
    id: "font-si",
  },
};

// ko, en, vi, id, ru, uz, kk, tg, es, pt, fr, de → Pretendard 커버 (별도 로딩 불필요)

/** 언어에 필요한 폰트를 동적으로 로드 (이미 로드된 폰트는 skip) */
export function loadFontForLanguage(lang: string): void {
  const config = FONT_MAP[lang];
  if (!config) return;

  if (document.getElementById(config.id)) return;

  const link = document.createElement("link");
  link.id = config.id;
  link.rel = "stylesheet";
  link.href = config.href;
  link.crossOrigin = "anonymous";
  document.head.appendChild(link);
}
