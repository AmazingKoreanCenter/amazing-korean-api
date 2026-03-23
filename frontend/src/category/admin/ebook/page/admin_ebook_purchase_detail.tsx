import { useState } from "react";
import { useParams, useNavigate, Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ArrowLeft, Trash2, Loader2 } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
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
import { Skeleton } from "@/components/ui/skeleton";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

import {
  getAdminEbookPurchase,
  updateAdminEbookStatus,
  deleteAdminEbookPurchase,
} from "@/category/ebook/ebook_api";
import type { EbookPurchaseStatus } from "@/category/ebook/types";

const statusBadgeVariant = (status: EbookPurchaseStatus) => {
  switch (status) {
    case "pending":
      return "warning" as const;
    case "completed":
      return "success" as const;
    case "refunded":
      return "destructive" as const;
    default:
      return "outline" as const;
  }
};

function getValidNextStatuses(
  current: EbookPurchaseStatus
): EbookPurchaseStatus[] {
  switch (current) {
    case "pending":
      return ["completed", "refunded"];
    case "completed":
      return ["refunded"];
    case "refunded":
      return [];
    default:
      return [];
  }
}

export function AdminEbookPurchaseDetail() {
  const { id } = useParams<{ id: string }>();
  const { t } = useTranslation();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const purchaseId = Number(id);

  const [deleteOpen, setDeleteOpen] = useState(false);

  const { data: purchase, isLoading } = useQuery({
    queryKey: ["admin", "ebook", "purchase", purchaseId],
    queryFn: () => getAdminEbookPurchase(purchaseId),
    enabled: !isNaN(purchaseId),
  });

  const statusMutation = useMutation({
    mutationFn: (status: EbookPurchaseStatus) =>
      updateAdminEbookStatus(purchaseId, { status }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["admin", "ebook"],
      });
      toast.success(t("admin.ebook.statusUpdated"));
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  const deleteMutation = useMutation({
    mutationFn: () => deleteAdminEbookPurchase(purchaseId),
    onSuccess: () => {
      toast.success(t("admin.ebook.deleted"));
      navigate("/admin/ebook/purchases");
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 rounded-lg" />
        <Skeleton className="h-48 rounded-lg" />
      </div>
    );
  }

  if (!purchase) {
    return (
      <div className="text-center py-12 text-muted-foreground">
        {t("admin.ebook.notFound")}
      </div>
    );
  }

  const nextStatuses = getValidNextStatuses(purchase.status);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Button variant="ghost" size="icon" asChild>
            <Link to="/admin/ebook/purchases">
              <ArrowLeft className="h-5 w-5" />
            </Link>
          </Button>
          <div>
            <h1 className="text-2xl font-bold font-mono">{purchase.purchase_code}</h1>
            <p className="text-sm text-muted-foreground">
              ID: {purchase.purchase_id} &middot;{" "}
              {new Date(purchase.created_at).toLocaleString()}
            </p>
          </div>
        </div>

        <Button
          variant="destructive"
          size="sm"
          onClick={() => setDeleteOpen(true)}
        >
          <Trash2 className="mr-1 h-4 w-4" />
          {t("admin.ebook.delete")}
        </Button>
      </div>

      {/* 상태 변경 */}
      <Card>
        <CardHeader>
          <CardTitle>{t("admin.ebook.statusSection")}</CardTitle>
        </CardHeader>
        <CardContent className="flex items-center gap-4">
          <div className="flex items-center gap-3">
            <Badge variant={statusBadgeVariant(purchase.status)} className="text-base px-3 py-1">
              {t(`ebook.status.${purchase.status}`)}
            </Badge>
            {nextStatuses.length > 0 && (
              <>
                <span className="text-muted-foreground">&rarr;</span>
                <Select
                  onValueChange={(v) =>
                    statusMutation.mutate(v as EbookPurchaseStatus)
                  }
                  disabled={statusMutation.isPending}
                >
                  <SelectTrigger className="w-[200px]">
                    <SelectValue
                      placeholder={t("admin.ebook.changeStatus")}
                    />
                  </SelectTrigger>
                  <SelectContent>
                    {nextStatuses.map((s) => (
                      <SelectItem key={s} value={s}>
                        {t(`ebook.status.${s}`)}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </>
            )}
          </div>
          {statusMutation.isPending && (
            <Loader2 className="h-4 w-4 animate-spin" />
          )}
        </CardContent>
      </Card>

      {/* 구매 정보 */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>{t("admin.ebook.purchaseInfo")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm">
            <Row label={t("admin.ebook.userId")} value={String(purchase.user_id)} />
            <Row label={t("admin.ebook.language")} value={purchase.language} />
            <Row label={t("admin.ebook.edition")} value={purchase.edition} />
            <Row
              label={t("admin.ebook.price")}
              value={`${purchase.price.toLocaleString()} ${purchase.currency}`}
            />
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("admin.ebook.paymentInfo")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm">
            <Row
              label={t("admin.ebook.payment")}
              value={purchase.payment_method.replace("_", " ")}
            />
            <Row
              label={t("admin.ebook.paddleTxnId")}
              value={purchase.paddle_txn_id || "-"}
            />
          </CardContent>
        </Card>
      </div>

      {/* 타임스탬프 */}
      <Card>
        <CardHeader>
          <CardTitle>{t("admin.ebook.timeline")}</CardTitle>
        </CardHeader>
        <CardContent className="grid grid-cols-2 sm:grid-cols-3 gap-4 text-sm">
          <TimeRow label={t("admin.ebook.date")} value={purchase.created_at} />
          <TimeRow label={t("admin.ebook.completedAt")} value={purchase.completed_at} />
          <TimeRow label={t("admin.ebook.refundedAt")} value={purchase.refunded_at} />
        </CardContent>
      </Card>

      {/* 삭제 확인 다이얼로그 */}
      <Dialog open={deleteOpen} onOpenChange={setDeleteOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("admin.ebook.confirmDeleteTitle")}</DialogTitle>
            <DialogDescription>
              {t("admin.ebook.confirmDeleteDesc", { code: purchase.purchase_code })}
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setDeleteOpen(false)}>
              {t("admin.ebook.cancel")}
            </Button>
            <Button
              variant="destructive"
              onClick={() => {
                deleteMutation.mutate();
                setDeleteOpen(false);
              }}
              disabled={deleteMutation.isPending}
            >
              {deleteMutation.isPending && (
                <Loader2 className="h-4 w-4 animate-spin mr-1" />
              )}
              {t("admin.ebook.confirmDelete")}
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

function TimeRow({ label, value }: { label: string; value?: string | null }) {
  return (
    <div>
      <p className="text-muted-foreground">{label}</p>
      <p className="font-medium">
        {value ? new Date(value).toLocaleString() : "-"}
      </p>
    </div>
  );
}
