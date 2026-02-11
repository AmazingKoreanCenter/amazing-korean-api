import i18n from "i18next";

/**
 * 콘텐츠 API용 lang 파라미터 반환.
 * ko(한국어 원본)이면 undefined를 반환하여 쿼리 파라미터에 포함하지 않음.
 */
export function getContentLang(): string | undefined {
  const lang = i18n.language;
  return lang === "ko" ? undefined : lang;
}
