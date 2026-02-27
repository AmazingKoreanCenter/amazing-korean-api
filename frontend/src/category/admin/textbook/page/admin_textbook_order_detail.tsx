import { useState } from "react";
import { useParams, useNavigate, Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import {
  ArrowLeft,
  Printer,
  Trash2,
  Loader2,
} from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

import {
  useAdminTextbookOrderDetail,
  useAdminUpdateTextbookStatus,
  useAdminDeleteTextbookOrder,
} from "../hook/use_admin_textbook";
import type { TextbookOrderStatus } from "@/category/textbook/types";

const ALL_STATUSES: TextbookOrderStatus[] = [
  "pending", "confirmed", "paid", "printing", "shipped", "delivered", "canceled",
];

export function AdminTextbookOrderDetail() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { orderId } = useParams<{ orderId: string }>();
  const id = Number(orderId);

  const { data: order, isLoading } = useAdminTextbookOrderDetail(id);
  const updateMutation = useAdminUpdateTextbookStatus();
  const deleteMutation = useAdminDeleteTextbookOrder();

  const [deleteOpen, setDeleteOpen] = useState(false);

  const handleStatusChange = (newStatus: string) => {
    if (!order) return;
    updateMutation.mutate(
      { id, data: { status: newStatus as TextbookOrderStatus } },
      {
        onSuccess: () => {
          toast.success(t("admin.textbook.statusUpdated"));
        },
        onError: () => {
          toast.error(t("admin.textbook.statusUpdateFailed"));
        },
      },
    );
  };

  const handleDelete = () => {
    deleteMutation.mutate(id, {
      onSuccess: () => {
        toast.success(t("admin.textbook.orderDeleted"));
        navigate("/admin/textbook/orders");
      },
      onError: () => {
        toast.error(t("admin.textbook.deleteFailed"));
      },
    });
    setDeleteOpen(false);
  };

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 rounded-lg" />
        <Skeleton className="h-48 rounded-lg" />
      </div>
    );
  }

  if (!order) {
    return (
      <div className="text-center py-12 text-muted-foreground">
        {t("admin.textbook.orderNotFound")}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Button variant="ghost" size="icon" asChild>
            <Link to="/admin/textbook/orders">
              <ArrowLeft className="h-5 w-5" />
            </Link>
          </Button>
          <div>
            <h1 className="text-2xl font-bold font-mono">{order.order_code}</h1>
            <p className="text-sm text-muted-foreground">
              ID: {order.order_id} &middot;{" "}
              {new Date(order.created_at).toLocaleString()}
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm" asChild>
            <Link
              to={`/admin/textbook/orders/${id}/print?type=quote`}
              target="_blank"
            >
              <Printer className="mr-1 h-4 w-4" />
              {t("admin.textbook.printQuote")}
            </Link>
          </Button>
          <Button variant="outline" size="sm" asChild>
            <Link
              to={`/admin/textbook/orders/${id}/print?type=confirmation`}
              target="_blank"
            >
              <Printer className="mr-1 h-4 w-4" />
              {t("admin.textbook.printConfirmation")}
            </Link>
          </Button>
          <Button
            variant="destructive"
            size="sm"
            onClick={() => setDeleteOpen(true)}
          >
            <Trash2 className="mr-1 h-4 w-4" />
            {t("admin.textbook.delete")}
          </Button>
        </div>
      </div>

      {/* 상태 변경 */}
      <Card>
        <CardHeader>
          <CardTitle>{t("admin.textbook.statusSection")}</CardTitle>
        </CardHeader>
        <CardContent className="flex items-center gap-4">
          <Select
            value={order.status}
            onValueChange={handleStatusChange}
            disabled={updateMutation.isPending}
          >
            <SelectTrigger className="w-[200px]">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {ALL_STATUSES.map((s) => (
                <SelectItem key={s} value={s}>
                  {t(`admin.textbook.status.${s}`)}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {updateMutation.isPending && (
            <Loader2 className="h-4 w-4 animate-spin" />
          )}
        </CardContent>
      </Card>

      {/* 주문 항목 */}
      <Card>
        <CardHeader>
          <CardTitle>{t("admin.textbook.itemsSection")}</CardTitle>
        </CardHeader>
        <CardContent>
          <table className="w-full text-sm">
            <thead className="border-b bg-secondary">
              <tr>
                <th className="px-3 py-2 text-left">{t("admin.textbook.colLanguage")}</th>
                <th className="px-3 py-2 text-left">{t("admin.textbook.colType")}</th>
                <th className="px-3 py-2 text-right">{t("admin.textbook.colQty")}</th>
                <th className="px-3 py-2 text-right">{t("admin.textbook.colUnitPrice")}</th>
                <th className="px-3 py-2 text-right">{t("admin.textbook.colSubtotal")}</th>
              </tr>
            </thead>
            <tbody>
              {order.items.map((item, i) => (
                <tr key={i} className="border-b">
                  <td className="px-3 py-2">{item.language_name}</td>
                  <td className="px-3 py-2">
                    <Badge variant="outline">
                      {item.textbook_type === "student"
                        ? t("admin.textbook.typeStudent")
                        : t("admin.textbook.typeTeacher")}
                    </Badge>
                  </td>
                  <td className="px-3 py-2 text-right">{item.quantity}</td>
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
                  {t("admin.textbook.total")}
                </td>
                <td className="px-3 py-2 text-right">{order.total_quantity}</td>
                <td />
                <td className="px-3 py-2 text-right">
                  {order.total_amount.toLocaleString()}{order.currency}
                </td>
              </tr>
            </tfoot>
          </table>
        </CardContent>
      </Card>

      {/* 신청자 / 배송 / 결제 정보 */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>{t("admin.textbook.ordererSection")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm">
            <Row label={t("admin.textbook.name")} value={order.orderer_name} />
            <Row label={t("admin.textbook.email")} value={order.orderer_email} />
            <Row label={t("admin.textbook.phone")} value={order.orderer_phone} />
            {order.org_name && (
              <Row label={t("admin.textbook.org")} value={`${order.org_name}${order.org_type ? ` (${order.org_type})` : ""}`} />
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("admin.textbook.deliverySection")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm">
            {order.delivery_postal_code && (
              <Row label={t("admin.textbook.postalCode")} value={order.delivery_postal_code} />
            )}
            <Row label={t("admin.textbook.address")} value={order.delivery_address} />
            {order.delivery_detail && (
              <Row label={t("admin.textbook.addressDetail")} value={order.delivery_detail} />
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("admin.textbook.paymentSection")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm">
            <Row
              label={t("admin.textbook.paymentMethod")}
              value={t("admin.textbook.bankTransfer")}
            />
            {order.depositor_name && (
              <Row label={t("admin.textbook.depositor")} value={order.depositor_name} />
            )}
          </CardContent>
        </Card>

        {order.tax_invoice && (
          <Card>
            <CardHeader>
              <CardTitle>{t("admin.textbook.taxSection")}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-2 text-sm">
              <Row label={t("admin.textbook.bizNumber")} value={order.tax_biz_number ?? "-"} />
              <Row label={t("admin.textbook.taxEmail")} value={order.tax_email ?? "-"} />
            </CardContent>
          </Card>
        )}
      </div>

      {/* 비고 */}
      {order.notes && (
        <Card>
          <CardHeader>
            <CardTitle>{t("admin.textbook.notesSection")}</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground whitespace-pre-wrap">
              {order.notes}
            </p>
          </CardContent>
        </Card>
      )}

      {/* 타임스탬프 */}
      <Card>
        <CardHeader>
          <CardTitle>{t("admin.textbook.timeline")}</CardTitle>
        </CardHeader>
        <CardContent className="grid grid-cols-2 sm:grid-cols-3 gap-4 text-sm">
          <TimeRow label={t("admin.textbook.createdAt")} value={order.created_at} />
          <TimeRow label={t("admin.textbook.confirmedAt")} value={order.confirmed_at} />
          <TimeRow label={t("admin.textbook.paidAt")} value={order.paid_at} />
          <TimeRow label={t("admin.textbook.shippedAt")} value={order.shipped_at} />
          <TimeRow label={t("admin.textbook.deliveredAt")} value={order.delivered_at} />
          <TimeRow label={t("admin.textbook.canceledAt")} value={order.canceled_at} />
        </CardContent>
      </Card>

      {/* 삭제 확인 다이얼로그 */}
      <Dialog open={deleteOpen} onOpenChange={setDeleteOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("admin.textbook.deleteConfirmTitle")}</DialogTitle>
            <DialogDescription>
              {t("admin.textbook.deleteConfirmDesc", { code: order.order_code })}
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setDeleteOpen(false)}>
              {t("admin.textbook.cancel")}
            </Button>
            <Button
              variant="destructive"
              onClick={handleDelete}
              disabled={deleteMutation.isPending}
            >
              {deleteMutation.isPending && (
                <Loader2 className="h-4 w-4 animate-spin mr-1" />
              )}
              {t("admin.textbook.confirmDelete")}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between">
      <span className="text-muted-foreground">{label}</span>
      <span className="font-medium">{value}</span>
    </div>
  );
}

function TimeRow({ label, value }: { label: string; value: string | null }) {
  return (
    <div>
      <p className="text-muted-foreground">{label}</p>
      <p className="font-medium">
        {value ? new Date(value).toLocaleString() : "-"}
      </p>
    </div>
  );
}
