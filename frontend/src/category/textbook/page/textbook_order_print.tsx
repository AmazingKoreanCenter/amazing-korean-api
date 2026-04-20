import { useParams, useSearchParams } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { Printer } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";

import { useOrderByCode } from "../hook/use_order_by_code";
import { TEXTBOOK_SUPPLIER } from "../supplier_info";

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

  // 영수증 모드: 공급가액/VAT/합계 분리 표시 (총액은 VAT 포함 기준)
  const vatRate = 0.1;
  const supplyAmount = isReceipt
    ? Math.round(order.total_amount / (1 + vatRate))
    : 0;
  const vatAmount = isReceipt ? order.total_amount - supplyAmount : 0;

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
      <div className="max-w-[800px] mx-auto p-8 print:p-0 print:max-w-full text-sm">
        {/* 헤더 */}
        <div className="text-center mb-8 border-b-2 border-black pb-4">
          <h1 className="text-2xl font-bold">{docTitle}</h1>
          <p className="text-muted-foreground mt-1 print:text-black">
            {t("textbook.print.companyName")}
          </p>
        </div>

        {/* 문서 정보 */}
        <div className="grid grid-cols-2 gap-4 mb-6">
          <div>
            <p>
              <strong>{t("textbook.print.docNumber")}:</strong>{" "}
              {order.order_code}
            </p>
            <p>
              <strong>
                {isReceipt
                  ? t("textbook.print.paidDate")
                  : t("textbook.print.date")}
                :
              </strong>{" "}
              {docDate}
            </p>
          </div>
          <div className="text-right">
            <p>
              <strong>{t("textbook.print.status")}:</strong>{" "}
              {t(`textbook.status.label.${order.status}`)}
            </p>
          </div>
        </div>

        {/* 공급자 정보 (영수증 전용) */}
        {isReceipt && (
          <div className="mb-6 p-4 border rounded">
            <h3 className="font-bold mb-2">{t("textbook.print.supplier")}</h3>
            <p>{TEXTBOOK_SUPPLIER.companyName}</p>
            <p>
              {t("textbook.print.bizNumber")}: {TEXTBOOK_SUPPLIER.bizNumber}
            </p>
            <p>
              {t("textbook.print.repName")}: {TEXTBOOK_SUPPLIER.repName}
            </p>
            <p>{TEXTBOOK_SUPPLIER.address}</p>
          </div>
        )}

        {/* 수신자 정보 */}
        <div className="mb-6 p-4 border rounded">
          <h3 className="font-bold mb-2">
            {t("textbook.print.recipient")}
          </h3>
          <p>{order.orderer_name}</p>
          {order.org_name && <p>{order.org_name}</p>}
          <p>{order.orderer_email}</p>
          <p>{order.orderer_phone}</p>
          <p>{order.delivery_address}</p>
          {order.delivery_detail && <p>{order.delivery_detail}</p>}
        </div>

        {/* 항목 테이블 */}
        <table className="w-full border-collapse mb-6">
          <thead>
            <tr className="border-b-2 border-black">
              <th className="text-left py-2 px-2">#</th>
              <th className="text-left py-2 px-2">
                {t("textbook.print.colItem")}
              </th>
              <th className="text-left py-2 px-2">
                {t("textbook.print.colType")}
              </th>
              <th className="text-right py-2 px-2">
                {t("textbook.print.colQty")}
              </th>
              <th className="text-right py-2 px-2">
                {t("textbook.print.colPrice")}
              </th>
              <th className="text-right py-2 px-2">
                {t("textbook.print.colSubtotal")}
              </th>
            </tr>
          </thead>
          <tbody>
            {order.items.map((item, i) => (
              <tr key={i} className="border-b">
                <td className="py-2 px-2">{i + 1}</td>
                <td className="py-2 px-2">
                  {t("textbook.print.textbookPrefix")} {item.language_name}
                </td>
                <td className="py-2 px-2">
                  {item.textbook_type === "student"
                    ? t("textbook.order.typeStudent")
                    : t("textbook.order.typeTeacher")}
                </td>
                <td className="py-2 px-2 text-right">{item.quantity}</td>
                <td className="py-2 px-2 text-right">
                  {item.unit_price.toLocaleString()}
                </td>
                <td className="py-2 px-2 text-right">
                  {item.subtotal.toLocaleString()}
                </td>
              </tr>
            ))}
          </tbody>
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
        </table>

        {/* 합계 박스 */}
        {isReceipt ? (
          <div className="p-4 border-2 border-black rounded mb-6">
            <div className="flex justify-between py-1">
              <span>{t("textbook.print.supplyAmount")}</span>
              <span>
                {supplyAmount.toLocaleString()} {order.currency}
              </span>
            </div>
            <div className="flex justify-between py-1 border-b">
              <span>{t("textbook.print.vatAmount")}</span>
              <span>
                {vatAmount.toLocaleString()} {order.currency}
              </span>
            </div>
            <div className="flex justify-between py-2 font-bold text-lg">
              <span>{t("textbook.print.receiptTotal")}</span>
              <span className="text-xl">
                {order.total_amount.toLocaleString()} {order.currency}
              </span>
            </div>
            <p className="text-center mt-2 text-sm">
              {t("textbook.print.receiptNotice")}
            </p>
          </div>
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
        {isReceipt && (
          <div className="mt-8 mb-6 flex justify-end">
            <div className="text-right">
              <p className="text-sm mb-2">{t("textbook.print.issuedBy")}</p>
              <p className="font-bold">{TEXTBOOK_SUPPLIER.companyName}</p>
              <p className="text-sm">
                {t("textbook.print.repName")}: {TEXTBOOK_SUPPLIER.repName}
              </p>
              <div className="mt-6 w-40 border-t pt-1 text-xs text-center text-muted-foreground print:text-gray-600">
                {t("textbook.print.sealLine")}
              </div>
            </div>
          </div>
        )}

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
