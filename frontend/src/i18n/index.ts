import i18n from "i18next";
import { initReactI18next } from "react-i18next";

import { loadFontForLanguage } from "@/utils/font_loader";
import {
  isCJK,
  isTallScript,
  needsRelaxedTracking,
  isRTL,
  LANG_CLASSES,
} from "@/utils/language_groups";

import ko from "./locales/ko.json";
import en from "./locales/en.json";

// ── 지원 언어 목록 (33개, RTL 3개 ar/fa/ur 제외) ─────────────────────
export const SUPPORTED_LANGUAGES = [
  // Tier 1: 핵심
  { code: "ko", name: "Korean", nativeName: "한국어", flag: "🇰🇷" },
  { code: "en", name: "English", nativeName: "English", flag: "🇺🇸" },
  { code: "ja", name: "Japanese", nativeName: "日本語", flag: "🇯🇵" },
  { code: "zh-CN", name: "Chinese (Simplified)", nativeName: "中文(简体)", flag: "🇨🇳" },
  { code: "zh-TW", name: "Chinese (Traditional)", nativeName: "中文(繁體)", flag: "🇹🇼" },
  // Tier 2: 동남아 + 북아시아
  { code: "vi", name: "Vietnamese", nativeName: "Tiếng Việt", flag: "🇻🇳" },
  { code: "th", name: "Thai", nativeName: "ภาษาไทย", flag: "🇹🇭" },
  { code: "id", name: "Indonesian", nativeName: "Bahasa Indonesia", flag: "🇮🇩" },
  { code: "my", name: "Myanmar", nativeName: "မြန်မာဘာသာ", flag: "🇲🇲" },
  { code: "mn", name: "Mongolian", nativeName: "Монгол хэл", flag: "🇲🇳" },
  { code: "ru", name: "Russian", nativeName: "Русский", flag: "🇷🇺" },
  { code: "lo", name: "Lao", nativeName: "ລາວ", flag: "🇱🇦" },
  { code: "tl", name: "Tagalog", nativeName: "Tagalog", flag: "🇵🇭" },
  // Tier 3: 중앙아시아 + 남아시아 + 유럽 + 아프리카
  { code: "es", name: "Spanish", nativeName: "Español", flag: "🇪🇸" },
  { code: "pt", name: "Portuguese", nativeName: "Português", flag: "🇧🇷" },
  { code: "pt-PT", name: "Portuguese (Portugal)", nativeName: "Português (Portugal)", flag: "🇵🇹" },
  { code: "fr", name: "French", nativeName: "Français", flag: "🇫🇷" },
  { code: "de", name: "German", nativeName: "Deutsch", flag: "🇩🇪" },
  { code: "it", name: "Italian", nativeName: "Italiano", flag: "🇮🇹" },
  { code: "pl", name: "Polish", nativeName: "Polski", flag: "🇵🇱" },
  { code: "uk", name: "Ukrainian", nativeName: "Українська", flag: "🇺🇦" },
  { code: "tr", name: "Turkish", nativeName: "Türkçe", flag: "🇹🇷" },
  { code: "hi", name: "Hindi", nativeName: "हिन्दी", flag: "🇮🇳" },
  { code: "ne", name: "Nepali", nativeName: "नेपाली", flag: "🇳🇵" },
  { code: "si", name: "Sinhala", nativeName: "සිංහල", flag: "🇱🇰" },
  { code: "bn", name: "Bengali", nativeName: "বাংলা", flag: "🇧🇩" },
  { code: "km", name: "Khmer", nativeName: "ភាសាខ្មែរ", flag: "🇰🇭" },
  { code: "uz", name: "Uzbek", nativeName: "Oʻzbekcha", flag: "🇺🇿" },
  { code: "kk", name: "Kazakh", nativeName: "Қазақ тілі", flag: "🇰🇿" },
  { code: "ky", name: "Kyrgyz", nativeName: "Кыргызча", flag: "🇰🇬" },
  { code: "tg", name: "Tajik", nativeName: "Тоҷикӣ", flag: "🇹🇯" },
  { code: "sw", name: "Swahili", nativeName: "Kiswahili", flag: "🇰🇪" },
  { code: "am", name: "Amharic", nativeName: "አማርኛ", flag: "🇪🇹" },
] as const;

export type LanguageCode = (typeof SUPPORTED_LANGUAGES)[number]["code"];

// Tier 구분 인덱스 (UI 구분선용)
export const TIER_BREAK_INDICES = [5, 13] as const; // Tier1 후, Tier2 후

// ── 상수 ─────────────────────────────────────────────────────────────
const LANGUAGE_KEY = "language";
const DEFAULT_LANGUAGE = "ko";

// ── 동적 로더: Vite dynamic import ──────────────────────────────────
const loadLanguageAsync = async (lang: string): Promise<boolean> => {
  try {
    const module = await import(`./locales/${lang}.json`);
    i18n.addResourceBundle(lang, "translation", module.default, true, true);
    return true;
  } catch {
    return false;
  }
};

// ── i18next 초기화 ──────────────────────────────────────────────────
i18n.use(initReactI18next).init({
  resources: {
    ko: { translation: ko },
    en: { translation: en },
  },
  lng: localStorage.getItem(LANGUAGE_KEY) || DEFAULT_LANGUAGE,
  fallbackLng: ["en", "ko"],
  interpolation: { escapeValue: false },
});

// ── 언어 변경 (async — 동적 로딩 + 폰트 로드) ──────────────────────
export const changeLanguage = async (lang: string): Promise<void> => {
  // 1. 동적으로 locale 파일 로드 (ko/en은 이미 번들에 포함)
  if (!i18n.hasResourceBundle(lang, "translation")) {
    await loadLanguageAsync(lang);
  }
  // 2. 폰트 로드 (CJK, 특수 스크립트만 해당)
  loadFontForLanguage(lang);
  // 3. i18next 언어 전환
  await i18n.changeLanguage(lang);
  // 4. localStorage 저장
  localStorage.setItem(LANGUAGE_KEY, lang);
  // 5. html lang 속성 업데이트
  document.documentElement.lang = lang;
  // 6. 언어 그룹 CSS 클래스 업데이트
  applyLangClasses(lang);
};

// ── 언어 그룹 CSS 클래스 적용 ──────────────────────────────────────
function applyLangClasses(lang: string): void {
  const root = document.documentElement.classList;
  root.remove(...LANG_CLASSES);
  if (isCJK(lang)) root.add("lang-cjk");
  if (isTallScript(lang)) root.add("lang-tall-script");
  if (needsRelaxedTracking(lang)) root.add("lang-relaxed-tracking");
  if (isRTL(lang)) root.add("lang-rtl");
  // RTL 활성 시 dir="rtl", 그 외 dir="ltr". SUPPORTED_LANGUAGES 에 RTL 미포함
  // (PR-B / S5 활성) 이라 현재는 항상 ltr 분기, 인프라 dormant 상태.
  document.documentElement.dir = isRTL(lang) ? "rtl" : "ltr";
}

// 초기 로드 시 현재 언어에 맞는 클래스 설정
applyLangClasses(i18n.language);

/** localStorage에 저장된 언어 코드 반환 */
export const getSavedLanguage = (): string => {
  return localStorage.getItem(LANGUAGE_KEY) || DEFAULT_LANGUAGE;
};

export default i18n;
