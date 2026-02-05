import i18n from "i18next";
import { initReactI18next } from "react-i18next";

import ko from "./locales/ko.json";
import en from "./locales/en.json";

const LANGUAGE_KEY = "language";
const DEFAULT_LANGUAGE = "ko";

i18n.use(initReactI18next).init({
  resources: {
    ko: { translation: ko },
    en: { translation: en },
  },
  lng: localStorage.getItem(LANGUAGE_KEY) || DEFAULT_LANGUAGE,
  fallbackLng: DEFAULT_LANGUAGE,
  interpolation: {
    escapeValue: false,
  },
});

/** 언어 변경 + localStorage 저장 */
export const changeLanguage = (lang: string) => {
  i18n.changeLanguage(lang);
  localStorage.setItem(LANGUAGE_KEY, lang);
};

/** localStorage에 저장된 언어 코드 반환 */
export const getSavedLanguage = () => {
  return localStorage.getItem(LANGUAGE_KEY) || DEFAULT_LANGUAGE;
};

export default i18n;
