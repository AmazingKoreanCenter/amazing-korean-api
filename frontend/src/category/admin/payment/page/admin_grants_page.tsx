import { useState } from "react";
import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { ArrowLeft, Plus, Trash2 } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";

import {
  useAdminGrants,
  useCreateAdminGrant,
  useRevokeAdminGrant,
} from "../hook/use_admin_payment";
import type { AdminGrantListReq } from "../types";

export function AdminGrantsPage() {
  const { t } = useTranslation();
  const [params, setParams] = useState<AdminGrantListReq>({ page: 1, size: 20 });
  const [grantDialogOpen, setGrantDialogOpen] = useState(false);
  const [revokeDialogOpen, setRevokeDialogOpen] = useState(false);
  const [revokeUserId, setRevokeUserId] = useState<number | null>(null);

  // Grant form state
  const [grantUserId, setGrantUserId] = useState("");
  const [grantExpireAt, setGrantExpireAt] = useState("");
  const [grantReason, setGrantReason] = useState("");

  const { data, isLoading, isError } = useAdminGrants(params);
  const createGrant = useCreateAdminGrant();
  const revokeGrant = useRevokeAdminGrant();

  const handlePageChange = (page: number) => {
    setParams((prev) => ({ ...prev, page }));
  };

  const handleCreateGrant = () => {
    const userId = parseInt(grantUserId);
    if (isNaN(userId) || userId <= 0) {
      toast.error(t("admin.payment.invalidUserId"));
      return;
    }
    if (!grantReason.trim()) {
      toast.error(t("admin.payment.reasonRequired"));
      return;
    }

    createGrant.mutate(
      {
        user_id: userId,
        expire_at: grantExpireAt ? `${grantExpireAt}T23:59:59Z` : undefined,
        reason: grantReason.trim(),
      },
      {
        onSuccess: (res) => {
          toast.success(t("admin.payment.grantSuccess", { userId: res.user_id, count: res.courses_granted }));
          setGrantDialogOpen(false);
          setGrantUserId("");
          setGrantExpireAt("");
          setGrantReason("");
        },
        onError: (e) => toast.error(e.message || t("admin.payment.grantFailed")),
      }
    );
  };

  const handleRevoke = () => {
    if (revokeUserId === null) return;

    revokeGrant.mutate(revokeUserId, {
      onSuccess: () => {
        toast.success(t("admin.payment.revokeSuccess", { userId: revokeUserId }));
        setRevokeDialogOpen(false);
        setRevokeUserId(null);
      },
      onError: (e) => toast.error(e.message || t("admin.payment.revokeFailed")),
    });
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Button variant="ghost" size="sm" asChild>
            <Link to="/admin/payment/subscriptions">
              <ArrowLeft className="mr-1 h-4 w-4" />
              {t("admin.payment.subscriptions")}
            </Link>
          </Button>
          <h1 className="text-2xl font-bold">{t("admin.payment.manualGrants")}</h1>
        </div>
        <Button onClick={() => setGrantDialogOpen(true)}>
          <Plus className="mr-2 h-4 w-4" />
          {t("admin.payment.grantCourses")}
        </Button>
      </div>

      <p className="text-sm text-muted-foreground">
        {t("admin.payment.grantPageDesc")}
      </p>

      {/* Table */}
      <div className="bg-card rounded-lg border overflow-hidden shadow-sm">
        <table className="w-full text-sm">
          <thead className="border-b-2 bg-secondary">
            <tr>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">{t("admin.payment.colUserId")}</th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">{t("admin.payment.colEmail")}</th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">{t("admin.payment.colCourses")}</th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">{t("admin.payment.colExpires")}</th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">{t("admin.payment.colActions")}</th>
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              Array.from({ length: 3 }).map((_, i) => (
                <tr key={i} className="border-b">
                  {Array.from({ length: 5 }).map((__, j) => (
                    <td key={j} className="px-4 py-3"><Skeleton className="h-4 w-20" /></td>
                  ))}
                </tr>
              ))
            ) : isError ? (
              <tr>
                <td colSpan={5} className="p-4 text-center text-destructive">
                  {t("admin.payment.failedLoad")}
                </td>
              </tr>
            ) : data?.items.length === 0 ? (
              <tr>
                <td colSpan={5} className="p-4 text-center text-muted-foreground">
                  {t("admin.payment.noGrants")}
                </td>
              </tr>
            ) : (
              data?.items.map((grant) => (
                <tr key={grant.user_id} className="border-b hover:bg-accent/10">
                  <td className="px-4 py-3">
                    <Link
                      to={`/admin/users/${grant.user_id}`}
                      className="text-primary hover:underline"
                    >
                      {grant.user_id}
                    </Link>
                  </td>
                  <td className="px-4 py-3">{grant.user_email}</td>
                  <td className="px-4 py-3">{grant.course_count}</td>
                  <td className="px-4 py-3">
                    {grant.expire_at
                      ? new Date(grant.expire_at).toLocaleDateString()
                      : t("admin.payment.noExpiry")}
                  </td>
                  <td className="px-4 py-3">
                    <Button
                      variant="ghost"
                      size="sm"
                      className="text-destructive hover:text-destructive"
                      onClick={() => {
                        setRevokeUserId(grant.user_id);
                        setRevokeDialogOpen(true);
                      }}
                    >
                      <Trash2 className="mr-1 h-3 w-3" />
                      {t("admin.payment.revoke")}
                    </Button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      {data?.meta && data.meta.total_pages > 1 && (
        <Pagination>
          <PaginationContent>
            <PaginationItem>
              <PaginationPrevious
                onClick={() => handlePageChange(Math.max(1, (params.page ?? 1) - 1))}
                className={params.page === 1 ? "pointer-events-none opacity-50" : "cursor-pointer"}
              />
            </PaginationItem>
            {Array.from({ length: Math.min(5, data.meta.total_pages) }, (_, i) => {
              const page = i + 1;
              return (
                <PaginationItem key={page}>
                  <PaginationLink
                    onClick={() => handlePageChange(page)}
                    isActive={params.page === page}
                    className="cursor-pointer"
                  >
                    {page}
                  </PaginationLink>
                </PaginationItem>
              );
            })}
            <PaginationItem>
              <PaginationNext
                onClick={() => handlePageChange(Math.min(data.meta.total_pages, (params.page ?? 1) + 1))}
                className={params.page === data.meta.total_pages ? "pointer-events-none opacity-50" : "cursor-pointer"}
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      )}

      {data?.meta && (
        <p className="text-sm text-muted-foreground text-center">
          {t("admin.payment.showing", { count: data.items.length, total: data.meta.total_count })}
        </p>
      )}

      {/* Grant Dialog */}
      <Dialog open={grantDialogOpen} onOpenChange={setGrantDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("admin.payment.grantCourses")}</DialogTitle>
            <DialogDescription>
              {t("admin.payment.grantDescription")}
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label>{t("admin.payment.grantUserId")}</Label>
              <Input
                type="number"
                placeholder={t("admin.payment.grantUserIdPlaceholder")}
                value={grantUserId}
                onChange={(e) => setGrantUserId(e.target.value)}
              />
            </div>
            <div className="space-y-2">
              <Label>{t("admin.payment.grantExpireDate")}</Label>
              <Input
                type="date"
                value={grantExpireAt}
                onChange={(e) => setGrantExpireAt(e.target.value)}
              />
            </div>
            <div className="space-y-2">
              <Label>{t("admin.payment.grantReason")}</Label>
              <Textarea
                placeholder={t("admin.payment.grantReasonPlaceholder")}
                value={grantReason}
                onChange={(e) => setGrantReason(e.target.value)}
              />
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setGrantDialogOpen(false)}>
              {t("admin.payment.cancel")}
            </Button>
            <Button onClick={handleCreateGrant} disabled={createGrant.isPending}>
              {createGrant.isPending ? t("admin.payment.granting") : t("admin.payment.grant")}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Revoke Confirmation Dialog */}
      <Dialog open={revokeDialogOpen} onOpenChange={setRevokeDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t("admin.payment.revokeTitle")}</DialogTitle>
            <DialogDescription>
              {t("admin.payment.revokeDescription", { userId: revokeUserId })}
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setRevokeDialogOpen(false)}>
              {t("admin.payment.cancel")}
            </Button>
            <Button
              variant="destructive"
              onClick={handleRevoke}
              disabled={revokeGrant.isPending}
            >
              {revokeGrant.isPending ? t("admin.payment.revoking") : t("admin.payment.revoke")}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
