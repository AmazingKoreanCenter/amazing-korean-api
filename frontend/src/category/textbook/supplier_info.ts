// 교재 영수증·세금계산서 발행 시 공급자 정보 (법인 고정값).
// 다국어 대상 아님 — 모든 언어 인쇄 시 동일하게 한국 법인 공식 표기를 사용한다.
export const TEXTBOOK_SUPPLIER = {
  companyName: "(주) 힘 HYMN Co., Ltd.",
  bizNumber: "505-88-03252",
  repName: "김경륜 (Kyoung Ryun KIM)",
  address: "세종시 한누리대로 350 6층 SB3호",
} as const;

export type TextbookSupplier = typeof TEXTBOOK_SUPPLIER;
