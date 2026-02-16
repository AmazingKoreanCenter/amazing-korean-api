import { useState } from "react";
import { useParams, Link } from "react-router-dom";
import { ArrowLeft, Pause, Play, XCircle } from "lucide-react";
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
  useAdminPauseSubscription,
  useAdminResumeSubscription,
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
  const { id } = useParams<{ id: string }>();
  const subId = Number(id);

  const { data, isLoading, isError } = useAdminSubscriptionDetail(subId);
  const cancelMutation = useAdminCancelSubscription();
  const pauseMutation = useAdminPauseSubscription();
  const resumeMutation = useAdminResumeSubscription();

  const [cancelDialogOpen, setCancelDialogOpen] = useState(false);

  const isBusy =
    cancelMutation.isPending || pauseMutation.isPending || resumeMutation.isPending;

  const handleCancel = (immediately: boolean) => {
    cancelMutation.mutate(
      { id: subId, data: { immediately } },
      {
        onSuccess: () => {
          toast.success("Subscription cancel requested");
          setCancelDialogOpen(false);
        },
        onError: (e) => toast.error(e.message || "Cancel failed"),
      }
    );
  };

  const handlePause = () => {
    pauseMutation.mutate(subId, {
      onSuccess: () => toast.success("Subscription pause requested"),
      onError: (e) => toast.error(e.message || "Pause failed"),
    });
  };

  const handleResume = () => {
    resumeMutation.mutate(subId, {
      onSuccess: () => toast.success("Subscription resume requested"),
      onError: (e) => toast.error(e.message || "Resume failed"),
    });
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
        Failed to load subscription
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
            Back
          </Link>
        </Button>
        <h1 className="text-2xl font-bold">Subscription #{sub.subscription_id}</h1>
        <Badge variant={statusBadgeVariant(sub.status)} className="text-sm">
          {sub.status}
        </Badge>
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        {/* Subscription Info */}
        <Card>
          <CardHeader>
            <CardTitle>Subscription</CardTitle>
            <CardDescription>Paddle ID: {sub.provider_subscription_id}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Interval</span>
              <span>{sub.billing_interval}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Price</span>
              <span>{formatCents(sub.current_price_cents)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Period Start</span>
              <span>{formatDate(sub.current_period_start)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Period End</span>
              <span>{formatDate(sub.current_period_end)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Trial Ends</span>
              <span>{formatDate(sub.trial_ends_at)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Canceled At</span>
              <span>{formatDate(sub.canceled_at)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Paused At</span>
              <span>{formatDate(sub.paused_at)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Created</span>
              <span>{formatDate(sub.created_at)}</span>
            </div>
          </CardContent>
        </Card>

        {/* User Info + Actions */}
        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>User</CardTitle>
            </CardHeader>
            <CardContent className="space-y-2 text-sm">
              <div className="flex justify-between">
                <span className="text-muted-foreground">User ID</span>
                <Link to={`/admin/users/${user.user_id}`} className="text-blue-600 hover:underline">
                  {user.user_id}
                </Link>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Email</span>
                <span>{user.email}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Nickname</span>
                <span>{user.nickname || "-"}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Role</span>
                <Badge variant="outline">{user.user_auth}</Badge>
              </div>
            </CardContent>
          </Card>

          {/* Management Actions */}
          {sub.status !== "canceled" && (
            <Card>
              <CardHeader>
                <CardTitle>Actions</CardTitle>
              </CardHeader>
              <CardContent className="flex gap-2 flex-wrap">
                {sub.status === "active" && (
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={handlePause}
                    disabled={isBusy}
                  >
                    <Pause className="mr-1 h-4 w-4" />
                    Pause
                  </Button>
                )}
                {sub.status === "paused" && (
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={handleResume}
                    disabled={isBusy}
                  >
                    <Play className="mr-1 h-4 w-4" />
                    Resume
                  </Button>
                )}
                <Button
                  variant="destructive"
                  size="sm"
                  onClick={() => setCancelDialogOpen(true)}
                  disabled={isBusy}
                >
                  <XCircle className="mr-1 h-4 w-4" />
                  Cancel
                </Button>
              </CardContent>
            </Card>
          )}
        </div>
      </div>

      {/* Transactions */}
      <Card>
        <CardHeader>
          <CardTitle>Transactions</CardTitle>
          <CardDescription>{transactions.length} transaction(s)</CardDescription>
        </CardHeader>
        <CardContent>
          {transactions.length === 0 ? (
            <p className="text-muted-foreground text-sm">No transactions yet</p>
          ) : (
            <div className="rounded-md border">
              <table className="w-full text-sm">
                <thead className="border-b bg-muted/50">
                  <tr>
                    <th className="h-10 px-4 text-left font-medium">ID</th>
                    <th className="h-10 px-4 text-left font-medium">Status</th>
                    <th className="h-10 px-4 text-left font-medium">Amount</th>
                    <th className="h-10 px-4 text-left font-medium">Tax</th>
                    <th className="h-10 px-4 text-left font-medium">Currency</th>
                    <th className="h-10 px-4 text-left font-medium">Date</th>
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
            <DialogTitle>Cancel Subscription</DialogTitle>
            <DialogDescription>
              Choose when to cancel this subscription.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter className="flex-col gap-2 sm:flex-row">
            <Button
              variant="outline"
              onClick={() => handleCancel(false)}
              disabled={cancelMutation.isPending}
            >
              Cancel at Period End
            </Button>
            <Button
              variant="destructive"
              onClick={() => handleCancel(true)}
              disabled={cancelMutation.isPending}
            >
              Cancel Immediately
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
