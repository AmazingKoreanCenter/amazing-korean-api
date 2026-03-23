import { useParams, useSearchParams } from "react-router-dom";
import { useTranslation } from "react-i18next";

import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";

import { useAdminTextbookOrderDetail } from "../hook/use_admin_textbook";

export function AdminTextbookOrderPrint() {
  const { t } = useTranslation();
  const { orderId } = useParams<{ orderId: string }>();
  const [searchParams] = useSearchParams();
  const type = searchParams.get("type") ?? "quote";
  const id = Number(orderId);

  const { data: order, isLoading } = useAdminTextbookOrderDetail(id);

  const isQuote = type === "quote";
  const docTitle = isQuote
    ? t("admin.textbook.print.quoteTitle")
    : t("admin.textbook.print.confirmationTitle");

  if (isLoading) {
    return (
      <div className="p-8 space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64" />
      </div>
    );
  }

  if (!order) {
    return (
      <div className="p-8 text-center text-muted-foreground">
        {t("admin.textbook.orderNotFound")}
      </div>
    );
  }

  return (
    <>
      {/* Print button (hidden in print) */}
      <div className="print:hidden p-4 text-center border-b">
        <Button onClick={() => window.print()}>
          {t("admin.textbook.print.printButton")}
        </Button>
      </div>

      {/* Print content */}
      <div className="max-w-[800px] mx-auto p-8 print:p-0 print:max-w-full text-sm">
        {/* Header */}
        <div className="text-center mb-8 border-b-2 border-black pb-4">
          <h1 className="text-2xl font-bold">{docTitle}</h1>
          <p className="text-muted-foreground mt-1 print:text-black">
            {t("admin.textbook.print.companyName")}
          </p>
        </div>

        {/* 문서 정보 */}
        <div className="grid grid-cols-2 gap-4 mb-6">
          <div>
            <p>
              <strong>{t("admin.textbook.print.docNumber")}:</strong>{" "}
              {order.order_code}
            </p>
            <p>
              <strong>{t("admin.textbook.print.date")}:</strong>{" "}
              {new Date(order.created_at).toLocaleDateString()}
            </p>
          </div>
          <div className="text-right">
            <p>
              <strong>{t("admin.textbook.print.status")}:</strong>{" "}
              {t(`admin.textbook.status.${order.status}`)}
            </p>
          </div>
        </div>

        {/* 수신자 정보 */}
        <div className="mb-6 p-4 border rounded">
          <h3 className="font-bold mb-2">
            {t("admin.textbook.print.recipient")}
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
                {t("admin.textbook.print.colItem")}
              </th>
              <th className="text-left py-2 px-2">
                {t("admin.textbook.print.colType")}
              </th>
              <th className="text-right py-2 px-2">
                {t("admin.textbook.print.colQty")}
              </th>
              <th className="text-right py-2 px-2">
                {t("admin.textbook.print.colPrice")}
              </th>
              <th className="text-right py-2 px-2">
                {t("admin.textbook.print.colSubtotal")}
              </th>
            </tr>
          </thead>
          <tbody>
            {order.items.map((item, i) => (
              <tr key={i} className="border-b">
                <td className="py-2 px-2">{i + 1}</td>
                <td className="py-2 px-2">
                  {t("admin.textbook.print.textbookPrefix")}{" "}
                  {item.language_name}
                </td>
                <td className="py-2 px-2">
                  {item.textbook_type === "student"
                    ? t("admin.textbook.typeStudent")
                    : t("admin.textbook.typeTeacher")}
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
                {t("admin.textbook.print.total")}
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
        <div className="p-4 border-2 border-black rounded mb-6 text-center">
          <p className="text-lg font-bold">
            {isQuote
              ? t("admin.textbook.print.quoteTotal")
              : t("admin.textbook.print.confirmationTotal")}
            :{" "}
            <span className="text-xl">
              {order.total_amount.toLocaleString()} {order.currency}
            </span>
          </p>
          <p className="text-xs text-muted-foreground print:text-gray-600 mt-1">
            ({t("admin.textbook.print.vatIncluded")})
          </p>
        </div>

        {/* 입금 계좌 안내 */}
        <div className="mb-6 p-4 border rounded">
          <h3 className="font-bold mb-2">
            {t("admin.textbook.print.bankInfo")}
          </h3>
          <p>{t("admin.textbook.print.bankAccount")}</p>
          <p className="text-xs text-muted-foreground print:text-gray-600 mt-1">
            {t("admin.textbook.print.bankNote")}
          </p>
        </div>

        {/* 세금계산서 정보 */}
        {order.tax_invoice && (
          <div className="mb-6 p-4 border rounded">
            <h3 className="font-bold mb-2">
              {t("admin.textbook.print.taxInvoice")}
            </h3>
            <p>
              {t("admin.textbook.print.bizNumber")}: {order.tax_biz_number}
            </p>
            <p>
              {t("admin.textbook.print.companyName")}: {order.tax_company_name ?? "-"}
            </p>
            <p>
              {t("admin.textbook.print.repName")}: {order.tax_rep_name ?? "-"}
            </p>
            <p>
              {t("admin.textbook.print.taxEmail")}: {order.tax_email}
            </p>
            {order.tax_address && (
              <p>{t("admin.textbook.print.taxAddress")}: {order.tax_address}</p>
            )}
            {order.tax_biz_type && (
              <p>{t("admin.textbook.print.taxBizType")}: {order.tax_biz_type}</p>
            )}
            {order.tax_biz_item && (
              <p>{t("admin.textbook.print.taxBizItem")}: {order.tax_biz_item}</p>
            )}
          </div>
        )}

        {/* 비고 */}
        {order.notes && (
          <div className="mb-6">
            <h3 className="font-bold mb-2">
              {t("admin.textbook.print.notes")}
            </h3>
            <p className="whitespace-pre-wrap">{order.notes}</p>
          </div>
        )}

        {/* 푸터 */}
        <div className="mt-12 pt-4 border-t text-center text-xs text-muted-foreground print:text-gray-500">
          <p>{t("admin.textbook.print.footer")}</p>
          <p className="mt-1">{t("admin.textbook.print.companyInfo")}</p>
        </div>
      </div>
    </>
  );
}
