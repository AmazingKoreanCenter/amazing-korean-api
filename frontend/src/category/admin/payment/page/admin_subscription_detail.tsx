import { useState } from "react";
import { useParams, Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { ArrowLeft, XCircle } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

import {
  useAdminSubscriptionDetail,
  useAdminCancelSubscription,
} from "../hook/use_admin_payment";

const formatCents = (cents: number) => `$${(cents / 100).toFixed(2)}`;
const formatDate = (d: string | null) =>
  d ? new Date(d).toLocaleString() : "-";

const statusBadgeVariant = (status: string) => {
  switch (status) {
    case "active": return "default";
    case "trialing": return "secondary";
    case "past_due": case "canceled": return "destructive";
    case "paused": return "outline";
    default: return "outline";
  }
};

export function AdminSubscriptionDetail() {
  const { t } = useTranslation();
  const { id } = useParams<{ id: string }>();
  const subId = Number(id);

  const { data, isLoading, isError } = useAdminSubscriptionDetail(subId);
  const cancelMutation = useAdminCancelSubscription();

  const [cancelDialogOpen, setCancelDialogOpen] = useState(false);

  const isBusy = cancelMutation.isPending;

  const handleCancel = (immediately: boolean) => {
    cancelMutation.mutate(
      { id: subId, data: { immediately } },
      {
        onSuccess: () => {
          toast.success(t("admin.payment.cancelRequested"));
          setCancelDialogOpen(false);
        },
        onError: (e) => toast.error(e.message || t("admin.payment.cancelFailed")),
      }
    );
  };

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (isError || !data) {
    return (
      <div className="text-center py-12 text-destructive">
        {t("admin.payment.failedLoad")}
      </div>
    );
  }

  const { subscription: sub, user, transactions } = data;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="sm" asChild>
          <Link to="/admin/payment/subscriptions">
            <ArrowLeft className="mr-1 h-4 w-4" />
            {t("admin.payment.back")}
          </Link>
        </Button>
        <h1 className="text-2xl font-bold">
          {t("admin.payment.subscriptionId", { id: sub.subscription_id })}
        </h1>
        <Badge variant={statusBadgeVariant(sub.status)} className="text-sm">
          {sub.status}
        </Badge>
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        {/* Subscription Info */}
        <Card>
          <CardHeader>
            <CardTitle>{t("admin.payment.subscription")}</CardTitle>
            <CardDescription>{t("admin.payment.paddleId", { id: sub.provider_subscription_id })}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-muted-foreground">{t("admin.payment.colInterval")}</span>
              <span>{sub.billing_interval}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">{t("admin.payment.colPrice")}</span>
              <span>{formatCents(sub.current_price_cents)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">{t("admin.payment.periodStart")}</span>
              <span>{formatDate(sub.current_period_start)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">{t("admin.payment.periodEnd")}</span>
              <span>{formatDate(sub.current_period_end)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">{t("admin.payment.trialEnds")}</span>
              <span>{formatDate(sub.trial_ends_at)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">{t("admin.payment.canceledAt")}</span>
              <span>{formatDate(sub.canceled_at)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">{t("admin.payment.pausedAt")}</span>
              <span>{formatDate(sub.paused_at)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">{t("admin.payment.created")}</span>
              <span>{formatDate(sub.created_at)}</span>
            </div>
          </CardContent>
        </Card>

        {/* User Info + Actions */}
        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>{t("admin.payment.user")}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-2 text-sm">
              <div className="flex justify-between">
                <span className="text-muted-foreground">{t("admin.payment.colUserId")}</span>
                <Link to={`/admin/users/${user.user_id}`} className="text-primary hover:underline">
                  {user.user_id}
                </Link>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">{t("admin.payment.colEmail")}</span>
                <span>{user.email}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">{t("admin.payment.colNickname")}</span>
                <span>{user.nickname || "-"}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">{t("admin.payment.colRole")}</span>
                <Badge variant="outline">{user.user_auth}</Badge>
              </div>
            </CardContent>
          </Card>

          {/* Management Actions */}
          {sub.status !== "canceled" && (
            <Card>
              <CardHeader>
                <CardTitle>{t("admin.payment.actions")}</CardTitle>
              </CardHeader>
              <CardContent className="flex gap-2 flex-wrap">
                <Button
                  variant="destructive"
                  size="sm"
                  onClick={() => setCancelDialogOpen(true)}
                  disabled={isBusy}
                >
                  <XCircle className="mr-1 h-4 w-4" />
                  {t("admin.payment.cancel")}
                </Button>
              </CardContent>
            </Card>
          )}
        </div>
      </div>

      {/* Transactions */}
      <Card>
        <CardHeader>
          <CardTitle>{t("admin.payment.transactions")}</CardTitle>
          <CardDescription>{t("admin.payment.transactionCount", { count: transactions.length })}</CardDescription>
        </CardHeader>
        <CardContent>
          {transactions.length === 0 ? (
            <p className="text-muted-foreground text-sm">{t("admin.payment.noTransactionsYet")}</p>
          ) : (
            <div className="rounded-md border">
              <table className="w-full text-sm">
                <thead className="border-b bg-muted/50">
                  <tr>
                    <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colId")}</th>
                    <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colStatus")}</th>
                    <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colAmount")}</th>
                    <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colTax")}</th>
                    <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colCurrency")}</th>
                    <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colDate")}</th>
                  </tr>
                </thead>
                <tbody>
                  {transactions.map((txn) => (
                    <tr key={txn.transaction_id} className="border-b">
                      <td className="p-4">{txn.transaction_id}</td>
                      <td className="p-4">
                        <Badge variant="outline">{txn.status}</Badge>
                      </td>
                      <td className="p-4">{formatCents(txn.amount_cents)}</td>
                      <td className="p-4">{formatCents(txn.tax_cents)}</td>
                      <td className="p-4">{txn.currency}</td>
                      <td className="p-4">{new Date(txn.occurred_at).toLocaleString()}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Cancel Dialog */}
      <Dialog open={cancelDialogOpen} onOpenChange={setCancelDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("admin.payment.cancelSubscription")}</DialogTitle>
            <DialogDescription>
              {t("admin.payment.cancelDescription")}
            </DialogDescription>
          </DialogHeader>
          <DialogFooter className="flex-col gap-2 sm:flex-row">
            <Button
              variant="outline"
              onClick={() => handleCancel(false)}
              disabled={cancelMutation.isPending}
            >
              {t("admin.payment.cancelAtPeriodEnd")}
            </Button>
            <Button
              variant="destructive"
              onClick={() => handleCancel(true)}
              disabled={cancelMutation.isPending}
            >
              {t("admin.payment.cancelImmediately")}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
