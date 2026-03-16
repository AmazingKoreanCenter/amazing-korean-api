import { useParams, useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ArrowLeft, Trash2 } from "lucide-react";
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
  getAdminEbookPurchase,
  updateAdminEbookStatus,
  deleteAdminEbookPurchase,
} from "@/category/ebook/ebook_api";
import type { EbookPurchaseStatus } from "@/category/ebook/types";

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
        <Skeleton className="h-64" />
      </div>
    );
  }

  if (!purchase) {
    return <div className="text-muted-foreground">{t("admin.ebook.notFound")}</div>;
  }

  const nextStatuses = getValidNextStatuses(purchase.status);

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Button
          variant="ghost"
          size="icon"
          onClick={() => navigate("/admin/ebook/purchases")}
        >
          <ArrowLeft className="w-5 h-5" />
        </Button>
        <h1 className="text-2xl font-bold">{purchase.purchase_code}</h1>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>{t("admin.ebook.purchaseInfo")}</CardTitle>
        </CardHeader>
        <CardContent>
          <dl className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <dt className="text-muted-foreground">{t("admin.ebook.userId")}</dt>
              <dd className="font-medium">{purchase.user_id}</dd>
            </div>
            <div>
              <dt className="text-muted-foreground">
                {t("admin.ebook.language")}
              </dt>
              <dd className="font-medium">{purchase.language}</dd>
            </div>
            <div>
              <dt className="text-muted-foreground">
                {t("admin.ebook.edition")}
              </dt>
              <dd className="font-medium capitalize">{purchase.edition}</dd>
            </div>
            <div>
              <dt className="text-muted-foreground">
                {t("admin.ebook.payment")}
              </dt>
              <dd className="font-medium capitalize">
                {purchase.payment_method.replace("_", " ")}
              </dd>
            </div>
            <div>
              <dt className="text-muted-foreground">
                {t("admin.ebook.price")}
              </dt>
              <dd className="font-medium">
                {purchase.price.toLocaleString()} {purchase.currency}
              </dd>
            </div>
            <div>
              <dt className="text-muted-foreground">{t("admin.ebook.paddleTxnId")}</dt>
              <dd className="font-medium">
                {purchase.paddle_txn_id || "-"}
              </dd>
            </div>
            <div>
              <dt className="text-muted-foreground">
                {t("admin.ebook.date")}
              </dt>
              <dd className="font-medium">
                {new Date(purchase.created_at).toLocaleString()}
              </dd>
            </div>
            {purchase.completed_at && (
              <div>
                <dt className="text-muted-foreground">
                  {t("admin.ebook.completedAt")}
                </dt>
                <dd className="font-medium">
                  {new Date(purchase.completed_at).toLocaleString()}
                </dd>
              </div>
            )}
            {purchase.refunded_at && (
              <div>
                <dt className="text-muted-foreground">
                  {t("admin.ebook.refundedAt")}
                </dt>
                <dd className="font-medium">
                  {new Date(purchase.refunded_at).toLocaleString()}
                </dd>
              </div>
            )}
            <div>
              <dt className="text-muted-foreground">
                {t("admin.ebook.status")}
              </dt>
              <dd className="flex items-center gap-2">
                <Badge
                  variant={
                    purchase.status === "completed"
                      ? "default"
                      : purchase.status === "refunded"
                        ? "destructive"
                        : "secondary"
                  }
                >
                  {t(`ebook.status.${purchase.status}`)}
                </Badge>
                {nextStatuses.length > 0 && (
                  <Select
                    onValueChange={(v) =>
                      statusMutation.mutate(v as EbookPurchaseStatus)
                    }
                    disabled={statusMutation.isPending}
                  >
                    <SelectTrigger className="w-[160px] h-8 text-xs">
                      <SelectValue
                        placeholder={t("admin.ebook.changeStatus")}
                      />
                    </SelectTrigger>
                    <SelectContent>
                      {nextStatuses.map((s) => (
                        <SelectItem key={s} value={s}>
                          → {s}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                )}
              </dd>
            </div>
          </dl>
        </CardContent>
      </Card>

      <div className="flex justify-end">
        <Button
          variant="destructive"
          size="sm"
          onClick={() => {
            if (window.confirm(t("admin.ebook.confirmDelete"))) {
              deleteMutation.mutate();
            }
          }}
          disabled={deleteMutation.isPending}
        >
          <Trash2 className="w-4 h-4 mr-1" />
          {t("admin.ebook.delete")}
        </Button>
      </div>
    </div>
  );
}
