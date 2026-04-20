// 교재 영수증 전용 공통 파트 컴포넌트 모음.
// 사용자 인쇄 페이지(textbook_order_print.tsx)와 관리자 인쇄 페이지
// (admin_textbook_order_print.tsx)에서 공통 사용. VAT 율과 통화별 소수점
// 반올림 로직도 이곳에 집중시켜 중복 하드코딩을 제거.
//
// Gemini 리뷰 PR #174 반영:
//   - HIGH: 통화별 반올림 자리수 차등 (KRW=0, USD/EUR=2)
//   - MEDIUM: VAT_RATE 상수 단일 소스
//   - MEDIUM: 공급자/합계/서명란 UI 중복 제거

import { TEXTBOOK_SUPPLIER } from "./supplier_info";

/** 부가세율 (현행 한국 국세 기준 10%). 정책 변경 시 이 값만 갱신. */
export const TEXTBOOK_VAT_RATE = 0.1;

/** 통화별 소수점 자리수. ISO 4217 표준 기준 (KRW/JPY 정수, 대부분 2자리). */
const CURRENCY_DECIMALS: Record<string, number> = {
  KRW: 0,
  JPY: 0,
  USD: 2,
  EUR: 2,
  GBP: 2,
  CNY: 2,
  HKD: 2,
  TWD: 2,
  SGD: 2,
  AUD: 2,
  CAD: 2,
};

function getCurrencyDecimals(currency: string): number {
  return CURRENCY_DECIMALS[currency.toUpperCase()] ?? 2;
}

/**
 * 부가세 포함 총액에서 공급가액·VAT 분리.
 * 통화별 소수점 자리수에 맞춰 반올림. 반올림 오차는 공급가액 쪽에서 흡수되어
 * supply + vat === total 등식이 통화 자리수 기준으로 정확히 성립함.
 */
export function calculateReceiptBreakdown(
  totalAmount: number,
  currency: string,
): { supplyAmount: number; vatAmount: number } {
  const decimals = getCurrencyDecimals(currency);
  const factor = Math.pow(10, decimals);
  const supplyAmount =
    Math.round((totalAmount / (1 + TEXTBOOK_VAT_RATE)) * factor) / factor;
  const vatAmount =
    Math.round((totalAmount - supplyAmount) * factor) / factor;
  return { supplyAmount, vatAmount };
}

/** 금액을 통화 자리수에 맞춰 로캘 포맷 */
export function formatReceiptAmount(amount: number, currency: string): string {
  const decimals = getCurrencyDecimals(currency);
  return amount.toLocaleString(undefined, {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  });
}

// ============================================================================
// 공급자 정보 박스 — 영수증 상단
// ============================================================================

interface SupplierBoxProps {
  /** i18n 네임스페이스 ("textbook.print" | "admin.textbook.print") */
  ns: "textbook.print" | "admin.textbook.print";
  /** i18next t() 함수 */
  t: (key: string) => string;
}

export function ReceiptSupplierBox({ ns, t }: SupplierBoxProps) {
  return (
    <div className="mb-6 p-4 border rounded">
      <h3 className="font-bold mb-2">{t(`${ns}.supplier`)}</h3>
      <p>{TEXTBOOK_SUPPLIER.companyName}</p>
      <p>
        {t(`${ns}.bizNumber`)}: {TEXTBOOK_SUPPLIER.bizNumber}
      </p>
      <p>
        {t(`${ns}.repName`)}: {TEXTBOOK_SUPPLIER.repName}
      </p>
      <p>{TEXTBOOK_SUPPLIER.address}</p>
    </div>
  );
}

// ============================================================================
// 합계 3단 분리 박스 — 공급가액 / VAT / 합계 + "정히 영수함" 문구
// ============================================================================

interface TotalBreakdownProps {
  totalAmount: number;
  currency: string;
  ns: "textbook.print" | "admin.textbook.print";
  t: (key: string) => string;
}

export function ReceiptTotalBreakdown({
  totalAmount,
  currency,
  ns,
  t,
}: TotalBreakdownProps) {
  const { supplyAmount, vatAmount } = calculateReceiptBreakdown(
    totalAmount,
    currency,
  );
  return (
    <div className="p-4 border-2 border-black rounded mb-6">
      <div className="flex justify-between py-1">
        <span>{t(`${ns}.supplyAmount`)}</span>
        <span>
          {formatReceiptAmount(supplyAmount, currency)} {currency}
        </span>
      </div>
      <div className="flex justify-between py-1 border-b">
        <span>{t(`${ns}.vatAmount`)}</span>
        <span>
          {formatReceiptAmount(vatAmount, currency)} {currency}
        </span>
      </div>
      <div className="flex justify-between py-2 font-bold text-lg">
        <span>{t(`${ns}.receiptTotal`)}</span>
        <span className="text-xl">
          {formatReceiptAmount(totalAmount, currency)} {currency}
        </span>
      </div>
      <p className="text-center mt-2 text-sm">{t(`${ns}.receiptNotice`)}</p>
    </div>
  );
}

// ============================================================================
// 서명란 — 발행인 + 대표자 + (인)
// ============================================================================

interface SignatureProps {
  ns: "textbook.print" | "admin.textbook.print";
  t: (key: string) => string;
}

export function ReceiptSignature({ ns, t }: SignatureProps) {
  return (
    <div className="mt-8 mb-6 flex justify-end">
      <div className="text-right">
        <p className="text-sm mb-2">{t(`${ns}.issuedBy`)}</p>
        <p className="font-bold">{TEXTBOOK_SUPPLIER.companyName}</p>
        <p className="text-sm">
          {t(`${ns}.repName`)}: {TEXTBOOK_SUPPLIER.repName}
        </p>
        <div className="mt-6 w-40 border-t pt-1 text-xs text-center text-muted-foreground print:text-gray-600">
          {t(`${ns}.sealLine`)}
        </div>
      </div>
    </div>
  );
}
