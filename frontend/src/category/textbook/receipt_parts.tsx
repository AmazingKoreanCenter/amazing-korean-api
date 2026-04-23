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
    <div className="p-4 border-2 border-black/80 rounded bg-muted/20 print:bg-transparent h-full">
      <h3 className="text-[10px] tracking-[0.2em] uppercase text-muted-foreground print:text-gray-600 mb-2 font-semibold">
        {t(`${ns}.supplier`)}
      </h3>
      <p className="font-semibold text-base">{TEXTBOOK_SUPPLIER.companyName}</p>
      <div className="mt-2 space-y-0.5 text-xs">
        <p>
          <span className="text-muted-foreground print:text-gray-600">
            {t(`${ns}.bizNumber`)}:
          </span>{" "}
          <span className="font-mono">{TEXTBOOK_SUPPLIER.bizNumber}</span>
        </p>
        <p>
          <span className="text-muted-foreground print:text-gray-600">
            {t(`${ns}.repName`)}:
          </span>{" "}
          {TEXTBOOK_SUPPLIER.repName}
        </p>
        <p className="text-muted-foreground print:text-gray-600 leading-snug mt-1">
          {TEXTBOOK_SUPPLIER.address}
        </p>
      </div>
    </div>
  );
}

// ============================================================================
// 합계 3단 (또는 할인 포함 시 5단) 분리 박스
// ============================================================================
//
// 기본: 공급가액 / VAT / 합계
// 할인 있을 때 (2026-04-23 신규): 품목 합계 / 할인 / 공급가액(할인 후) / VAT / 합계
//
// 세법 정확성: 공급가액(과세표준) 은 할인 후 금액이어야 VAT 10% 계산이
// 올바름. totalAmount 는 할인 후 최종 금액이므로 이걸 기준으로 분리.
// grossAmount / discountAmount 는 할인 전 참고 표시용.

interface TotalBreakdownProps {
  /** 할인 후 최종 VAT 포함 금액. 공급가액/VAT 분리 기준. */
  totalAmount: number;
  /** 할인 전 총액 (수량 × 단가, VAT 포함). 할인 있을 때만 표시. */
  grossAmount?: number;
  /** 할인 금액 (VAT 포함). 0 또는 undefined 면 할인 라인 미표시. */
  discountAmount?: number;
  /** 할인 사유 (선택, 괄호 주석으로 표시). */
  discountReason?: string | null;
  currency: string;
  ns: "textbook.print" | "admin.textbook.print";
  t: (key: string) => string;
}

export function ReceiptTotalBreakdown({
  totalAmount,
  grossAmount,
  discountAmount,
  discountReason,
  currency,
  ns,
  t,
}: TotalBreakdownProps) {
  const hasDiscount = (discountAmount ?? 0) > 0;
  // 공급가액·VAT 는 할인 후 기준 (세법 정확). 할인 반영된 totalAmount 에서 역산.
  const { supplyAmount, vatAmount } = calculateReceiptBreakdown(
    totalAmount,
    currency,
  );
  return (
    <div className="border-2 border-black rounded overflow-hidden mb-6">
      {hasDiscount && grossAmount !== undefined && (
        <>
          <div className="flex justify-between px-4 py-2 text-sm">
            <span className="text-muted-foreground print:text-gray-700">
              {t(`${ns}.subtotal`)}
            </span>
            <span className="font-mono">
              {formatReceiptAmount(grossAmount, currency)} {currency}
            </span>
          </div>
          <div className="flex justify-between px-4 py-2 text-sm text-destructive print:text-red-700">
            <span className="flex-1">
              − {t(`${ns}.discount`)}
              {discountReason && (
                <span className="text-xs text-muted-foreground print:text-gray-600 ml-2 font-normal">
                  ({discountReason})
                </span>
              )}
            </span>
            <span className="font-mono">
              − {formatReceiptAmount(discountAmount ?? 0, currency)} {currency}
            </span>
          </div>
          <div className="border-t" />
        </>
      )}
      <div className="flex justify-between px-4 py-2 text-sm">
        <span className="text-muted-foreground print:text-gray-700">
          {t(`${ns}.supplyAmount`)}
        </span>
        <span className="font-mono">
          {formatReceiptAmount(supplyAmount, currency)} {currency}
        </span>
      </div>
      <div className="flex justify-between px-4 py-2 text-sm border-b">
        <span className="text-muted-foreground print:text-gray-700">
          {t(`${ns}.vatAmount`)}
        </span>
        <span className="font-mono">
          {formatReceiptAmount(vatAmount, currency)} {currency}
        </span>
      </div>
      <div className="flex justify-between items-center px-4 py-3 bg-black text-white print:bg-gray-900">
        <span className="text-sm tracking-wider uppercase">
          {t(`${ns}.receiptTotal`)}
        </span>
        <span className="text-2xl font-bold font-mono">
          {formatReceiptAmount(totalAmount, currency)} {currency}
        </span>
      </div>
      <p className="text-center py-2 text-sm bg-muted/30 print:bg-gray-50 border-t">
        {t(`${ns}.receiptNotice`)}
      </p>
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
    <div className="mt-10 mb-6 flex justify-end">
      <div className="text-right">
        <p className="text-[10px] tracking-[0.2em] uppercase text-muted-foreground print:text-gray-600 mb-2">
          {t(`${ns}.issuedBy`)}
        </p>
        <p className="font-bold text-base">{TEXTBOOK_SUPPLIER.companyName}</p>
        <p className="text-xs mt-1">
          <span className="text-muted-foreground print:text-gray-600">
            {t(`${ns}.repName`)}:
          </span>{" "}
          {TEXTBOOK_SUPPLIER.repName}
        </p>
        {/* 인감 자리 박스 */}
        <div className="mt-4 ml-auto w-24 h-24 border-2 border-dashed border-muted-foreground print:border-gray-400 rounded flex items-center justify-center text-[10px] text-muted-foreground print:text-gray-500">
          {t(`${ns}.sealLine`)}
        </div>
      </div>
    </div>
  );
}
