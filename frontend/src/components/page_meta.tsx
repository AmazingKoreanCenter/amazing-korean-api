import { useLocation } from "react-router-dom";
import { useTranslation } from "react-i18next";

const BASE_URL = "https://amazingkorean.net";

const LOCALE_MAP: Record<string, string> = {
  ko: "ko_KR",
  en: "en_US",
  ja: "ja_JP",
  "zh-CN": "zh_CN",
  "zh-TW": "zh_TW",
  vi: "vi_VN",
  th: "th_TH",
  id: "id_ID",
  my: "my_MM",
  mn: "mn_MN",
  ru: "ru_RU",
  es: "es_ES",
  pt: "pt_BR",
  fr: "fr_FR",
  de: "de_DE",
  hi: "hi_IN",
  ne: "ne_NP",
  si: "si_LK",
  km: "km_KH",
  uz: "uz_UZ",
  kk: "kk_KZ",
  tg: "tg_TJ",
};

interface PageMetaProps {
  titleKey: string;
  descriptionKey: string;
}

export function PageMeta({ titleKey, descriptionKey }: PageMetaProps) {
  const { t, i18n } = useTranslation();
  const { pathname } = useLocation();

  const title = t(titleKey);
  const description = t(descriptionKey);
  const url = `${BASE_URL}${pathname}`;
  const locale = LOCALE_MAP[i18n.language] || "ko_KR";

  return (
    <>
      <title>{title}</title>
      <link rel="canonical" href={url} />
      <meta name="description" content={description} />
      <meta property="og:title" content={title} />
      <meta property="og:description" content={description} />
      <meta property="og:url" content={url} />
      <meta property="og:locale" content={locale} />
      <meta name="twitter:title" content={title} />
      <meta name="twitter:description" content={description} />
    </>
  );
}
