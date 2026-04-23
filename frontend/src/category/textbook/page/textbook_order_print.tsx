import { useParams, useSearchParams } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { Printer } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";

import { useOrderByCode } from "../hook/use_order_by_code";
import {
  ReceiptSignature,
  ReceiptSupplierBox,
  ReceiptTotalBreakdown,
} from "../receipt_parts";

export function TextbookOrderPrint() {
  const { t } = useTranslation();
  const { code } = useParams<{ code: string }>();
  const [searchParams] = useSearchParams();
  const type = searchParams.get("type") ?? "quote";

  const { data: order, isLoading, isError } = useOrderByCode(code ?? "");

  const isQuote = type === "quote";
  const isReceipt = type === "receipt";
  const docTitle = isReceipt
    ? t("textbook.print.receiptTitle")
    : isQuote
      ? t("textbook.print.quoteTitle")
      : t("textbook.print.confirmationTitle");

  if (isLoading) {
    return (
      <div className="p-8 space-y-4">
        <Skeleton className="h-8 w-48 mx-auto" />
        <Skeleton className="h-64" />
      </div>
    );
  }

  if (isError || !order) {
    return (
      <div className="p-8 text-center text-muted-foreground">
        {t("textbook.print.notFound")}
      </div>
    );
  }

  // 영수증은 입금 완료 후에만 발급 가능
  if (isReceipt && !order.paid_at) {
    return (
      <div className="p-8 text-center text-muted-foreground">
        {t("textbook.print.receiptUnpaid")}
      </div>
    );
  }

  const docDate = isReceipt && order.paid_at
    ? new Date(order.paid_at).toLocaleDateString()
    : new Date(order.created_at).toLocaleDateString();

  return (
    <>
      {/* 인쇄 버튼 + 안내 (인쇄 시 숨김) */}
      <div className="print:hidden p-4 text-center border-b space-y-2">
        <Button onClick={() => window.print()}>
          <Printer className="h-4 w-4 mr-2" />
          {t("textbook.print.printButton")}
        </Button>
        <p className="text-xs text-muted-foreground">
          {t("textbook.print.pdfGuide")}
        </p>
      </div>

      {/* 인쇄 콘텐츠 */}
      <div className="max-w-[800px] mx-auto p-8 print:p-8 print:max-w-full text-sm bg-white text-black">
        {/* 헤더 — 영수증은 정식 문서 느낌으로 강화 */}
        {isReceipt ? (
          <div className="mb-8">
            <div className="flex justify-between items-end border-b-[3px] border-black pb-4">
              <div>
                <p className="text-[10px] tracking-[0.3em] uppercase text-muted-foreground print:text-gray-600 mb-1">
                  RECEIPT · 영수증
                </p>
                <h1 className="text-4xl font-bold tracking-tight">
                  {docTitle}
                </h1>
              </div>
              <div className="text-right">
                <p className="text-[10px] tracking-[0.2em] uppercase text-muted-foreground print:text-gray-600">
                  No.
                </p>
                <p className="font-mono text-base font-semibold">
                  {order.order_code}
                </p>
                <p className="text-xs mt-2">
                  <span className="text-muted-foreground print:text-gray-600">
                    {t("textbook.print.paidDate")}:
                  </span>{" "}
                  {docDate}
                </p>
              </div>
            </div>
          </div>
        ) : (
          <>
            <div className="text-center mb-8 border-b-2 border-black pb-4">
              <h1 className="text-2xl font-bold">{docTitle}</h1>
              <p className="text-muted-foreground mt-1 print:text-black">
                {t("textbook.print.companyName")}
              </p>
            </div>
            <div className="grid grid-cols-2 gap-4 mb-6">
              <div>
                <p>
                  <strong>{t("textbook.print.docNumber")}:</strong>{" "}
                  {order.order_code}
                </p>
                <p>
                  <strong>{t("textbook.print.date")}:</strong> {docDate}
                </p>
              </div>
              <div className="text-right">
                <p>
                  <strong>{t("textbook.print.status")}:</strong>{" "}
                  {t(`textbook.status.label.${order.status}`)}
                </p>
              </div>
            </div>
          </>
        )}

        {/* 공급자 / 수신자 — 영수증은 2컬럼, 그 외는 수신자만 */}
        {isReceipt ? (
          <div className="grid grid-cols-2 gap-4 mb-6">
            <ReceiptSupplierBox ns="textbook.print" t={t} />
            <div className="p-4 border-2 border-black/80 rounded h-full">
              <h3 className="text-[10px] tracking-[0.2em] uppercase text-muted-foreground print:text-gray-600 mb-2 font-semibold">
                {t("textbook.print.recipient")}
              </h3>
              <p className="font-semibold text-base">{order.orderer_name}</p>
              {order.org_name && (
                <p className="text-xs text-muted-foreground print:text-gray-600">
                  {order.org_name}
                </p>
              )}
              <div className="mt-2 space-y-0.5 text-xs">
                {order.orderer_email && <p>{order.orderer_email}</p>}
                <p>{order.orderer_phone}</p>
                <p className="text-muted-foreground print:text-gray-600 leading-snug mt-1">
                  {order.delivery_address}
                  {order.delivery_detail ? ` ${order.delivery_detail}` : ""}
                </p>
              </div>
            </div>
          </div>
        ) : (
          <div className="mb-6 p-4 border rounded">
            <h3 className="font-bold mb-2">
              {t("textbook.print.recipient")}
            </h3>
            <p>{order.orderer_name}</p>
            {order.org_name && <p>{order.org_name}</p>}
            {order.orderer_email && <p>{order.orderer_email}</p>}
            <p>{order.orderer_phone}</p>
            <p>{order.delivery_address}</p>
            {order.delivery_detail && <p>{order.delivery_detail}</p>}
          </div>
        )}

        {/* 항목 테이블 */}
        <div className={isReceipt ? "border rounded overflow-hidden mb-6" : "mb-6"}>
          <table className="w-full border-collapse">
            <thead>
              <tr className={isReceipt ? "bg-muted/40 print:bg-gray-100 border-b-2 border-black" : "border-b-2 border-black"}>
                <th className={`text-left ${isReceipt ? "py-3 px-3 text-[11px] uppercase tracking-wider font-semibold" : "py-2 px-2"}`}>#</th>
                <th className={`text-left ${isReceipt ? "py-3 px-3 text-[11px] uppercase tracking-wider font-semibold" : "py-2 px-2"}`}>
                  {t("textbook.print.colItem")}
                </th>
                <th className={`text-left ${isReceipt ? "py-3 px-3 text-[11px] uppercase tracking-wider font-semibold" : "py-2 px-2"}`}>
                  {t("textbook.print.colType")}
                </th>
                <th className={`text-right ${isReceipt ? "py-3 px-3 text-[11px] uppercase tracking-wider font-semibold" : "py-2 px-2"}`}>
                  {t("textbook.print.colQty")}
                </th>
                <th className={`text-right ${isReceipt ? "py-3 px-3 text-[11px] uppercase tracking-wider font-semibold" : "py-2 px-2"}`}>
                  {t("textbook.print.colPrice")}
                </th>
                <th className={`text-right ${isReceipt ? "py-3 px-3 text-[11px] uppercase tracking-wider font-semibold" : "py-2 px-2"}`}>
                  {t("textbook.print.colSubtotal")}
                </th>
              </tr>
            </thead>
            <tbody>
              {order.items.map((item, i) => (
                <tr key={i} className={isReceipt ? "border-b last:border-b-0" : "border-b"}>
                  <td className={`text-muted-foreground print:text-gray-700 ${isReceipt ? "py-3 px-3" : "py-2 px-2"}`}>{i + 1}</td>
                  <td className={isReceipt ? "py-3 px-3" : "py-2 px-2"}>
                    {t("textbook.print.textbookPrefix")} {item.language_name}
                  </td>
                  <td className={isReceipt ? "py-3 px-3" : "py-2 px-2"}>
                    {item.textbook_type === "student"
                      ? t("textbook.order.typeStudent")
                      : t("textbook.order.typeTeacher")}
                  </td>
                  <td className={`text-right font-mono ${isReceipt ? "py-3 px-3" : "py-2 px-2"}`}>{item.quantity}</td>
                  <td className={`text-right font-mono ${isReceipt ? "py-3 px-3" : "py-2 px-2"}`}>
                    {item.unit_price.toLocaleString()}
                  </td>
                  <td className={`text-right font-mono ${isReceipt ? "py-3 px-3 font-medium" : "py-2 px-2"}`}>
                    {item.subtotal.toLocaleString()}
                  </td>
                </tr>
              ))}
            </tbody>
            {!isReceipt && (
              <tfoot>
                <tr className="border-t-2 border-black font-bold">
                  <td colSpan={3} className="py-2 px-2">
                    {t("textbook.print.total")}
                  </td>
                  <td className="py-2 px-2 text-right">{order.total_quantity}</td>
                  <td />
                  <td className="py-2 px-2 text-right">
                    {order.total_amount.toLocaleString()} {order.currency}
                  </td>
                </tr>
              </tfoot>
            )}
          </table>
        </div>

        {/* 합계 박스 */}
        {isReceipt ? (
          <ReceiptTotalBreakdown
            totalAmount={order.total_amount}
            grossAmount={order.gross_amount}
            discountAmount={order.discount_amount}
            discountReason={order.discount_reason}
            currency={order.currency}
            ns="textbook.print"
            t={t}
          />
        ) : (
          <div className="p-4 border-2 border-black rounded mb-6 text-center">
            <p className="text-lg font-bold">
              {isQuote
                ? t("textbook.print.quoteTotal")
                : t("textbook.print.confirmationTotal")}
              :{" "}
              <span className="text-xl">
                {order.total_amount.toLocaleString()} {order.currency}
              </span>
            </p>
            <p className="text-xs text-muted-foreground print:text-gray-600 mt-1">
              ({t("textbook.print.vatIncluded")})
            </p>
          </div>
        )}

        {/* 입금 계좌 안내 (영수증엔 표시 안 함) */}
        {!isReceipt && (
          <div className="mb-6 p-4 border rounded">
            <h3 className="font-bold mb-2">
              {t("textbook.print.bankInfo")}
            </h3>
            <p>{t("textbook.print.bankAccount")}</p>
            <p className="text-xs text-muted-foreground print:text-gray-600 mt-1">
              {t("textbook.print.bankNote")}
            </p>
          </div>
        )}

        {/* 세금계산서 정보 */}
        {order.tax_invoice && (
          <div className="mb-6 p-4 border rounded">
            <h3 className="font-bold mb-2">
              {t("textbook.print.taxInvoice")}
            </h3>
            <p>
              {t("textbook.print.bizNumber")}: {order.tax_biz_number}
            </p>
            <p>
              {t("textbook.print.taxCompanyName")}: {order.tax_company_name ?? "-"}
            </p>
            <p>
              {t("textbook.print.repName")}: {order.tax_rep_name ?? "-"}
            </p>
            <p>
              {t("textbook.print.taxEmail")}: {order.tax_email}
            </p>
            {order.tax_address && (
              <p>{t("textbook.print.taxAddress")}: {order.tax_address}</p>
            )}
            {order.tax_biz_type && (
              <p>{t("textbook.print.taxBizType")}: {order.tax_biz_type}</p>
            )}
            {order.tax_biz_item && (
              <p>{t("textbook.print.taxBizItem")}: {order.tax_biz_item}</p>
            )}
          </div>
        )}

        {/* 비고 */}
        {order.notes && (
          <div className="mb-6">
            <h3 className="font-bold mb-2">
              {t("textbook.print.notes")}
            </h3>
            <p className="whitespace-pre-wrap">{order.notes}</p>
          </div>
        )}

        {/* 서명란 (영수증 전용) */}
        {isReceipt && <ReceiptSignature ns="textbook.print" t={t} />}

        {/* 고지 문구 (영수증엔 표시 안 함 — 이미 서명란으로 대체) */}
        {!isReceipt && (
          <div className="mb-6 p-3 bg-muted/50 print:bg-gray-50 rounded text-xs text-muted-foreground print:text-gray-600">
            {isQuote
              ? t("textbook.print.quoteNotice")
              : t("textbook.print.confirmationNotice")}
          </div>
        )}

        {/* 푸터 */}
        <div className="mt-12 pt-4 border-t text-center text-xs text-muted-foreground print:text-gray-500">
          <p>{t("textbook.print.footer")}</p>
          <p className="mt-1">{t("textbook.print.companyInfo")}</p>
        </div>
      </div>
    </>
  );
}
