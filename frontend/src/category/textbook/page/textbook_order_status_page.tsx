import { useState } from "react";
import { useParams } from "react-router-dom";
import { useTranslation } from "react-i18next";
import {
  Search,
  Package,
  CheckCircle2,
  Clock,
  Truck,
  XCircle,
  Printer,
  CreditCard,
  Loader2,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { PageMeta } from "@/components/page_meta";

import { useOrderByCode } from "../hook/use_order_by_code";
import type { TextbookOrderStatus } from "../types";

// =============================================================================
// Status Helpers
// =============================================================================

const STATUS_CONFIG: Record<
  TextbookOrderStatus,
  { icon: typeof Clock; color: string; bgColor: string }
> = {
  pending: { icon: Clock, color: "text-status-warning", bgColor: "bg-status-warning/10" },
  confirmed: { icon: CheckCircle2, color: "text-blue-600", bgColor: "bg-blue-600/10" },
  paid: { icon: CreditCard, color: "text-emerald-600", bgColor: "bg-emerald-600/10" },
  printing: { icon: Printer, color: "text-purple-600", bgColor: "bg-purple-600/10" },
  shipped: { icon: Truck, color: "text-indigo-600", bgColor: "bg-indigo-600/10" },
  delivered: { icon: CheckCircle2, color: "text-status-success", bgColor: "bg-status-success/10" },
  canceled: { icon: XCircle, color: "text-destructive", bgColor: "bg-destructive/10" },
};

const STATUS_STEPS: TextbookOrderStatus[] = [
  "pending",
  "confirmed",
  "paid",
  "printing",
  "shipped",
  "delivered",
];

export function TextbookOrderStatusPage() {
  const { t } = useTranslation();
  const { code: paramCode } = useParams<{ code: string }>();
  const [inputCode, setInputCode] = useState(paramCode ?? "");
  const [searchCode, setSearchCode] = useState(paramCode ?? "");

  const { data: order, isLoading, isError } = useOrderByCode(searchCode);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = inputCode.trim();
    if (trimmed) {
      setSearchCode(trimmed);
    }
  };

  const statusConfig = order ? STATUS_CONFIG[order.status] : null;
  const currentStepIndex = order
    ? STATUS_STEPS.indexOf(order.status)
    : -1;

  return (
    <div className="flex flex-col">
      <PageMeta
        titleKey="seo.textbookStatus.title"
        descriptionKey="seo.textbookStatus.description"
      />

      <div className="max-w-3xl mx-auto px-6 py-16 w-full">
        {/* 검색 */}
        <div className="text-center mb-10">
          <h1 className="text-3xl font-bold mb-2">
            {t("textbook.status.title")}
          </h1>
          <p className="text-muted-foreground mb-6">
            {t("textbook.status.subtitle")}
          </p>
          <form
            onSubmit={handleSearch}
            className="flex gap-2 max-w-md mx-auto"
          >
            <Input
              placeholder={t("textbook.status.placeholder")}
              value={inputCode}
              onChange={(e) => setInputCode(e.target.value)}
              className="font-mono"
            />
            <Button type="submit" disabled={!inputCode.trim()}>
              <Search className="h-4 w-4 mr-1" />
              {t("textbook.status.search")}
            </Button>
          </form>
        </div>

        {/* 로딩 */}
        {isLoading && searchCode && (
          <div className="flex justify-center py-12">
            <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
          </div>
        )}

        {/* 에러 */}
        {isError && searchCode && (
          <Card className="text-center py-12">
            <CardContent>
              <XCircle className="h-12 w-12 text-destructive mx-auto mb-4" />
              <p className="text-lg font-medium">
                {t("textbook.status.notFound")}
              </p>
              <p className="text-sm text-muted-foreground mt-1">
                {t("textbook.status.notFoundDesc")}
              </p>
            </CardContent>
          </Card>
        )}

        {/* 결과 */}
        {order && statusConfig && (
          <div className="space-y-6">
            {/* 상태 배너 */}
            <Card>
              <CardContent className="py-6">
                <div className="flex items-center gap-4">
                  <div
                    className={`w-14 h-14 rounded-full ${statusConfig.bgColor} flex items-center justify-center`}
                  >
                    <statusConfig.icon
                      className={`h-7 w-7 ${statusConfig.color}`}
                    />
                  </div>
                  <div>
                    <p className="text-sm text-muted-foreground">
                      {order.order_code}
                    </p>
                    <p className="text-xl font-bold">
                      {t(`textbook.status.label.${order.status}`)}
                    </p>
                    <p className="text-sm text-muted-foreground">
                      {t("textbook.status.orderedAt")}{" "}
                      {new Date(order.created_at).toLocaleDateString()}
                    </p>
                  </div>
                </div>

                {/* Progress Steps (취소 제외) */}
                {order.status !== "canceled" && (
                  <div className="mt-6 flex items-center gap-1">
                    {STATUS_STEPS.map((step, i) => {
                      const isCompleted = i <= currentStepIndex;
                      const isCurrent = i === currentStepIndex;
                      return (
                        <div key={step} className="flex-1 flex flex-col items-center">
                          <div
                            className={`w-full h-2 rounded-full ${
                              isCompleted
                                ? "bg-primary"
                                : "bg-secondary"
                            }`}
                          />
                          <span
                            className={`text-xs mt-1 ${
                              isCurrent
                                ? "text-primary font-semibold"
                                : isCompleted
                                  ? "text-muted-foreground"
                                  : "text-muted-foreground/50"
                            }`}
                          >
                            {t(`textbook.status.step.${step}`)}
                          </span>
                        </div>
                      );
                    })}
                  </div>
                )}
              </CardContent>
            </Card>

            {/* 주문 상세 */}
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Package className="h-5 w-5" />
                  {t("textbook.status.orderDetail")}
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                {/* 항목 테이블 */}
                <div className="overflow-x-auto">
                  <table className="w-full text-sm">
                    <thead className="border-b bg-secondary">
                      <tr>
                        <th className="px-3 py-2 text-left">
                          {t("textbook.status.colLanguage")}
                        </th>
                        <th className="px-3 py-2 text-left">
                          {t("textbook.status.colType")}
                        </th>
                        <th className="px-3 py-2 text-right">
                          {t("textbook.status.colQuantity")}
                        </th>
                        <th className="px-3 py-2 text-right">
                          {t("textbook.status.colUnitPrice")}
                        </th>
                        <th className="px-3 py-2 text-right">
                          {t("textbook.status.colSubtotal")}
                        </th>
                      </tr>
                    </thead>
                    <tbody>
                      {order.items.map((item, i) => (
                        <tr key={i} className="border-b">
                          <td className="px-3 py-2">{item.language_name}</td>
                          <td className="px-3 py-2">
                            <Badge variant="outline">
                              {t(`textbook.order.type${item.textbook_type === "student" ? "Student" : "Teacher"}`)}
                            </Badge>
                          </td>
                          <td className="px-3 py-2 text-right">
                            {item.quantity}
                          </td>
                          <td className="px-3 py-2 text-right">
                            {item.unit_price.toLocaleString()}
                          </td>
                          <td className="px-3 py-2 text-right font-medium">
                            {item.subtotal.toLocaleString()}
                          </td>
                        </tr>
                      ))}
                    </tbody>
                    <tfoot>
                      <tr className="border-t-2 font-bold">
                        <td colSpan={2} className="px-3 py-2">
                          {t("textbook.status.total")}
                        </td>
                        <td className="px-3 py-2 text-right">
                          {order.total_quantity}
                        </td>
                        <td />
                        <td className="px-3 py-2 text-right">
                          {order.total_amount.toLocaleString()}
                          {t("textbook.order.currency")}
                        </td>
                      </tr>
                    </tfoot>
                  </table>
                </div>

                {/* 신청자 / 배송 정보 */}
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-6 pt-4">
                  <div className="space-y-2 text-sm">
                    <h4 className="font-semibold">
                      {t("textbook.status.ordererInfo")}
                    </h4>
                    <p>
                      {order.orderer_name} / {order.orderer_phone}
                    </p>
                    <p>{order.orderer_email}</p>
                    {order.org_name && (
                      <p>
                        {order.org_name}
                        {order.org_type && ` (${order.org_type})`}
                      </p>
                    )}
                  </div>
                  <div className="space-y-2 text-sm">
                    <h4 className="font-semibold">
                      {t("textbook.status.deliveryInfo")}
                    </h4>
                    {order.delivery_postal_code && (
                      <p>[{order.delivery_postal_code}]</p>
                    )}
                    <p>{order.delivery_address}</p>
                    {order.delivery_detail && <p>{order.delivery_detail}</p>}
                  </div>
                </div>

                {order.notes && (
                  <div className="pt-4 text-sm">
                    <h4 className="font-semibold mb-1">
                      {t("textbook.status.notes")}
                    </h4>
                    <p className="text-muted-foreground">{order.notes}</p>
                  </div>
                )}
              </CardContent>
            </Card>
          </div>
        )}

        {/* 초기 상태 (검색 전) */}
        {!searchCode && (
          <div className="text-center py-16 text-muted-foreground">
            <Search className="h-16 w-16 mx-auto mb-4 opacity-30" />
            <p>{t("textbook.status.initialHint")}</p>
          </div>
        )}
      </div>
    </div>
  );
}
