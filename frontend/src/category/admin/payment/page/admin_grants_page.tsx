import { useState } from "react";
import { Link } from "react-router-dom";
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
      toast.error("Please enter a valid User ID");
      return;
    }
    if (!grantReason.trim()) {
      toast.error("Please enter a reason");
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
          toast.success(`Granted ${res.courses_granted} courses to user #${res.user_id}`);
          setGrantDialogOpen(false);
          setGrantUserId("");
          setGrantExpireAt("");
          setGrantReason("");
        },
        onError: (e) => toast.error(e.message || "Grant failed"),
      }
    );
  };

  const handleRevoke = () => {
    if (revokeUserId === null) return;

    revokeGrant.mutate(revokeUserId, {
      onSuccess: () => {
        toast.success(`Courses revoked for user #${revokeUserId}`);
        setRevokeDialogOpen(false);
        setRevokeUserId(null);
      },
      onError: (e) => toast.error(e.message || "Revoke failed"),
    });
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Button variant="ghost" size="sm" asChild>
            <Link to="/admin/payment/subscriptions">
              <ArrowLeft className="mr-1 h-4 w-4" />
              Subscriptions
            </Link>
          </Button>
          <h1 className="text-2xl font-bold">Manual Grants</h1>
        </div>
        <Button onClick={() => setGrantDialogOpen(true)}>
          <Plus className="mr-2 h-4 w-4" />
          Grant Courses
        </Button>
      </div>

      <p className="text-sm text-muted-foreground">
        Users with active courses but no active subscription (manually granted).
      </p>

      {/* Table */}
      <div className="rounded-md border">
        <table className="w-full text-sm">
          <thead className="border-b bg-muted/50">
            <tr>
              <th className="h-10 px-4 text-left font-medium">User ID</th>
              <th className="h-10 px-4 text-left font-medium">Email</th>
              <th className="h-10 px-4 text-left font-medium">Courses</th>
              <th className="h-10 px-4 text-left font-medium">Expires</th>
              <th className="h-10 px-4 text-left font-medium">Actions</th>
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              Array.from({ length: 3 }).map((_, i) => (
                <tr key={i} className="border-b">
                  {Array.from({ length: 5 }).map((__, j) => (
                    <td key={j} className="p-4"><Skeleton className="h-4 w-20" /></td>
                  ))}
                </tr>
              ))
            ) : isError ? (
              <tr>
                <td colSpan={5} className="p-4 text-center text-destructive">
                  Failed to load grants
                </td>
              </tr>
            ) : data?.items.length === 0 ? (
              <tr>
                <td colSpan={5} className="p-4 text-center text-muted-foreground">
                  No manual grants found
                </td>
              </tr>
            ) : (
              data?.items.map((grant) => (
                <tr key={grant.user_id} className="border-b hover:bg-muted/50">
                  <td className="p-4">
                    <Link
                      to={`/admin/users/${grant.user_id}`}
                      className="text-blue-600 hover:underline"
                    >
                      {grant.user_id}
                    </Link>
                  </td>
                  <td className="p-4">{grant.user_email}</td>
                  <td className="p-4">{grant.course_count}</td>
                  <td className="p-4">
                    {grant.expire_at
                      ? new Date(grant.expire_at).toLocaleDateString()
                      : "No expiry"}
                  </td>
                  <td className="p-4">
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
                      Revoke
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
          Showing {data.items.length} of {data.meta.total_count} grants
        </p>
      )}

      {/* Grant Dialog */}
      <Dialog open={grantDialogOpen} onOpenChange={setGrantDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Grant Courses</DialogTitle>
            <DialogDescription>
              Manually grant all active courses to a user without a Paddle subscription.
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label>User ID</Label>
              <Input
                type="number"
                placeholder="e.g. 123"
                value={grantUserId}
                onChange={(e) => setGrantUserId(e.target.value)}
              />
            </div>
            <div className="space-y-2">
              <Label>Expiration Date (optional)</Label>
              <Input
                type="date"
                value={grantExpireAt}
                onChange={(e) => setGrantExpireAt(e.target.value)}
              />
            </div>
            <div className="space-y-2">
              <Label>Reason</Label>
              <Textarea
                placeholder="e.g. VIP customer, CS compensation..."
                value={grantReason}
                onChange={(e) => setGrantReason(e.target.value)}
              />
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setGrantDialogOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleCreateGrant} disabled={createGrant.isPending}>
              {createGrant.isPending ? "Granting..." : "Grant"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Revoke Confirmation Dialog */}
      <Dialog open={revokeDialogOpen} onOpenChange={setRevokeDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Revoke Courses</DialogTitle>
            <DialogDescription>
              Are you sure you want to revoke all courses for user #{revokeUserId}?
              This action will deactivate all their course access.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setRevokeDialogOpen(false)}>
              Cancel
            </Button>
            <Button
              variant="destructive"
              onClick={handleRevoke}
              disabled={revokeGrant.isPending}
            >
              {revokeGrant.isPending ? "Revoking..." : "Revoke"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
