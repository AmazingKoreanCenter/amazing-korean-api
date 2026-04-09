/**
 * Figma 레퍼런스 캡처용 API 모의 응답
 *
 * Why: 로컬 AMK 백엔드가 실행 중이 아니거나 포트 3000이 점유된 상황에서도
 * textbook/ebook catalog 페이지가 실제 콘텐츠로 렌더링되도록 함.
 * 실제 DB 데이터와 완벽히 일치할 필요는 없음 — Figma 레퍼런스 목적상
 * "로드 완료 상태의 UI"가 보이면 충분.
 *
 * 기준: frontend/src/category/book/book_data.ts 의 10개 언어 중
 * textbookLanguageSchema에 포함되는 9개 (en 제외).
 */

export const TEXTBOOK_CATALOG_FIXTURE = {
  items: [
    {
      language: "zh_cn",
      language_name_ko: "중국어(간체)",
      language_name_en: "Chinese (Simplified)",
      available_types: ["student", "teacher"],
      unit_price: 25000,
      available: true,
      isbn_ready: true,
    },
    {
      language: "ja",
      language_name_ko: "일본어",
      language_name_en: "Japanese",
      available_types: ["student", "teacher"],
      unit_price: 25000,
      available: true,
      isbn_ready: true,
    },
    {
      language: "vi",
      language_name_ko: "베트남어",
      language_name_en: "Vietnamese",
      available_types: ["student", "teacher"],
      unit_price: 25000,
      available: true,
      isbn_ready: true,
    },
    {
      language: "th",
      language_name_ko: "태국어",
      language_name_en: "Thai",
      available_types: ["student", "teacher"],
      unit_price: 25000,
      available: true,
      isbn_ready: true,
    },
    {
      language: "id",
      language_name_ko: "인도네시아어",
      language_name_en: "Indonesian",
      available_types: ["student", "teacher"],
      unit_price: 25000,
      available: true,
      isbn_ready: true,
    },
    {
      language: "ne",
      language_name_ko: "네팔어",
      language_name_en: "Nepali",
      available_types: ["student", "teacher"],
      unit_price: 25000,
      available: true,
      isbn_ready: true,
    },
    {
      language: "km",
      language_name_ko: "크메르어",
      language_name_en: "Khmer",
      available_types: ["student", "teacher"],
      unit_price: 25000,
      available: true,
      isbn_ready: true,
    },
    {
      language: "tl",
      language_name_ko: "필리핀어",
      language_name_en: "Filipino",
      available_types: ["student", "teacher"],
      unit_price: 25000,
      available: true,
      isbn_ready: true,
    },
    {
      language: "ru",
      language_name_ko: "러시아어",
      language_name_en: "Russian",
      available_types: ["student", "teacher"],
      unit_price: 25000,
      available: true,
      isbn_ready: true,
    },
  ],
  currency: "KRW",
  min_total_quantity: 10,
} as const;

export const EBOOK_CATALOG_FIXTURE = {
  items: [
    {
      language: "en",
      language_name_ko: "영어",
      language_name_en: "English",
      editions: [
        { edition: "student", price: 19000, currency: "KRW", paddle_price_usd: 14.9, total_pages: 320, available: true },
        { edition: "teacher", price: 29000, currency: "KRW", paddle_price_usd: 22.9, total_pages: 360, available: true },
      ],
    },
    {
      language: "zh_cn",
      language_name_ko: "중국어(간체)",
      language_name_en: "Chinese (Simplified)",
      editions: [
        { edition: "student", price: 19000, currency: "KRW", paddle_price_usd: 14.9, total_pages: 320, available: true },
        { edition: "teacher", price: 29000, currency: "KRW", paddle_price_usd: 22.9, total_pages: 360, available: true },
      ],
    },
    {
      language: "ja",
      language_name_ko: "일본어",
      language_name_en: "Japanese",
      editions: [
        { edition: "student", price: 19000, currency: "KRW", paddle_price_usd: 14.9, total_pages: 320, available: true },
        { edition: "teacher", price: 29000, currency: "KRW", paddle_price_usd: 22.9, total_pages: 360, available: true },
      ],
    },
    {
      language: "vi",
      language_name_ko: "베트남어",
      language_name_en: "Vietnamese",
      editions: [
        { edition: "student", price: 19000, currency: "KRW", paddle_price_usd: 14.9, total_pages: 320, available: true },
        { edition: "teacher", price: 29000, currency: "KRW", paddle_price_usd: 22.9, total_pages: 360, available: true },
      ],
    },
    {
      language: "th",
      language_name_ko: "태국어",
      language_name_en: "Thai",
      editions: [
        { edition: "student", price: 19000, currency: "KRW", paddle_price_usd: 14.9, total_pages: 320, available: true },
        { edition: "teacher", price: 29000, currency: "KRW", paddle_price_usd: 22.9, total_pages: 360, available: true },
      ],
    },
    {
      language: "id",
      language_name_ko: "인도네시아어",
      language_name_en: "Indonesian",
      editions: [
        { edition: "student", price: 19000, currency: "KRW", paddle_price_usd: 14.9, total_pages: 320, available: true },
        { edition: "teacher", price: 29000, currency: "KRW", paddle_price_usd: 22.9, total_pages: 360, available: true },
      ],
    },
  ],
  paddle_ebook_price_id: null,
  client_token: null,
  sandbox: true,
} as const;
