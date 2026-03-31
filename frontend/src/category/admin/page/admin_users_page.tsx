import { useState } from "react";
import { Link } from "react-router-dom";
import { Plus, Upload, Users, BarChart3, LogIn, UserPlus, Loader2 } from "lucide-react";
import { toast } from "sonner";
import { useMutation } from "@tanstack/react-query";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";

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
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";

import { DataTable, useDataTable, type DataTableColumn } from "@/components/blocks/data_table";
import { useAdminUsers } from "../hook/use_admin_users";
import { updateAdminUsersBulk, createAdminInvite } from "../admin_api";
import type { AdminUserSummary, UpgradeInviteReq } from "../types";
import { upgradeInviteReqSchema } from "../types";

const getRoleBadgeVariant = (role: string) => {
  switch (role) {
    case "HYMN":
      return "purple" as const;
    case "admin":
      return "orange" as const;
    case "manager":
      return "secondary" as const;
    case "learner":
      return "success" as const;
    default:
      return "outline" as const;
  }
};

const columns: DataTableColumn<AdminUserSummary>[] = [
  { key: "id", header: "ID", sortField: "id", skeletonWidth: "w-8", render: (u) => u.id },
  { key: "email", header: "Email", sortField: "email", skeletonWidth: "w-40", render: (u) => u.email },
  { key: "nickname", header: "Nickname", sortField: "nickname", skeletonWidth: "w-24", render: (u) => u.nickname || "-" },
  {
    key: "role",
    header: "Role",
    sortField: "role",
    skeletonWidth: "w-16",
    render: (u) => <Badge variant={getRoleBadgeVariant(u.role)}>{u.role}</Badge>,
  },
  {
    key: "created_at",
    header: "Created At",
    sortField: "created_at",
    skeletonWidth: "w-28",
    render: (u) => new Date(u.created_at).toLocaleDateString(),
  },
  {
    key: "actions",
    header: "Actions",
    skeletonWidth: "w-16",
    render: (u) => (
      <Button variant="ghost" size="sm" asChild>
        <Link to={`/admin/users/${u.id}`}>Edit</Link>
      </Button>
    ),
  },
];

export function AdminUsersPage() {
  const table = useDataTable({ defaultSortField: "id" });

  // 벌크 수정 다이얼로그
  const [bulkEditOpen, setBulkEditOpen] = useState(false);
  const [bulkRole, setBulkRole] = useState<string>("");
  const [bulkState, setBulkState] = useState<string>("");
  const [bulkUpdating, setBulkUpdating] = useState(false);

  // 관리자 초대 다이얼로그
  const [inviteOpen, setInviteOpen] = useState(false);

  const inviteForm = useForm<UpgradeInviteReq>({
    resolver: zodResolver(upgradeInviteReqSchema),
    defaultValues: { email: "", role: "manager" },
  });

  const inviteMutation = useMutation({
    mutationFn: createAdminInvite,
    onSuccess: (data) => {
      toast.success(data.message);
      setInviteOpen(false);
      inviteForm.reset();
    },
    onError: (error: Error) => {
      toast.error(error.message || "초대 발송에 실패했습니다.");
    },
  });

  const handleInviteSubmit = (values: UpgradeInviteReq) => {
    inviteMutation.mutate(values);
  };

  const { data, isLoading, isError, refetch } = useAdminUsers({
    ...table.params,
    sort: table.sortField,
    order: table.sortOrder,
  });

  const handleBulkUpdate = async () => {
    if (table.selectedIds.size === 0) return;

    if (!bulkRole && !bulkState) {
      toast.error("Please select at least one field to update");
      return;
    }

    const items = Array.from(table.selectedIds).map((id) => ({
      id,
      ...(bulkRole ? { user_auth: bulkRole as "learner" | "manager" | "admin" } : {}),
      ...(bulkState ? { user_state: bulkState === "active" } : {}),
    }));

    setBulkUpdating(true);
    try {
      const result = await updateAdminUsersBulk({ items });
      toast.success(`Updated ${result.summary.success} of ${result.summary.total} users`);
      setBulkEditOpen(false);
      setBulkRole("");
      setBulkState("");
      table.clearSelection();
      refetch();
    } catch {
      toast.error("Bulk update failed");
    } finally {
      setBulkUpdating(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Users Management</h1>
        <div className="flex gap-2">
          <Button variant="outline" asChild>
            <Link to="/admin/users/stats">
              <BarChart3 className="mr-2 h-4 w-4" />
              User Stats
            </Link>
          </Button>
          <Button variant="outline" asChild>
            <Link to="/admin/logins/stats">
              <LogIn className="mr-2 h-4 w-4" />
              Login Stats
            </Link>
          </Button>
          <Button variant="outline" asChild>
            <Link to="/admin/users/bulk-create">
              <Upload className="mr-2 h-4 w-4" />
              Bulk Create
            </Link>
          </Button>
          <Button variant="outline" onClick={() => setInviteOpen(true)}>
            <UserPlus className="mr-2 h-4 w-4" />
            Invite Admin
          </Button>
          <Button asChild>
            <Link to="/admin/users/new">
              <Plus className="mr-2 h-4 w-4" />
              Add User
            </Link>
          </Button>
        </div>
      </div>

      <DataTable
        columns={columns}
        data={data?.items}
        isLoading={isLoading}
        isError={isError}
        entityName="users"
        getId={(u) => u.id}
        searchPlaceholder="Search by email or nickname..."
        searchInput={table.searchInput}
        onSearchInputChange={table.setSearchInput}
        onSearch={table.handleSearch}
        sortField={table.sortField}
        sortOrder={table.sortOrder}
        onSort={table.handleSort}
        selectedIds={table.selectedIds}
        onSelectAll={table.setSelectedIds}
        onSelectOne={table.handleSelectOne}
        bulkActionSlot={
          <Button variant="outline" onClick={() => setBulkEditOpen(true)}>
            <Users className="mr-2 h-4 w-4" />
            Edit {table.selectedIds.size} Selected
          </Button>
        }
        page={table.params.page}
        totalPages={data?.meta?.total_pages}
        totalCount={data?.meta?.total_count}
        onPageChange={table.handlePageChange}
      />

      {/* Bulk Edit Dialog */}
      <Dialog open={bulkEditOpen} onOpenChange={setBulkEditOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Edit Users</DialogTitle>
            <DialogDescription>
              Update {table.selectedIds.size} selected users. Leave fields empty to keep current values.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label>Role</Label>
              <Select value={bulkRole} onValueChange={setBulkRole}>
                <SelectTrigger>
                  <SelectValue placeholder="No change" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="learner">Learner</SelectItem>
                  <SelectItem value="manager">Manager</SelectItem>
                  <SelectItem value="admin">Admin</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Status</Label>
              <Select value={bulkState} onValueChange={setBulkState}>
                <SelectTrigger>
                  <SelectValue placeholder="No change" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="active">Active</SelectItem>
                  <SelectItem value="inactive">Inactive</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setBulkEditOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleBulkUpdate} disabled={bulkUpdating}>
              {bulkUpdating ? "Updating..." : "Update"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Admin Invite Dialog */}
      <Dialog open={inviteOpen} onOpenChange={setInviteOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>관리자 초대</DialogTitle>
            <DialogDescription>
              새로운 관리자를 초대합니다. 초대 이메일이 발송되며, 10분간 유효합니다.
            </DialogDescription>
          </DialogHeader>

          <Form {...inviteForm}>
            <form onSubmit={inviteForm.handleSubmit(handleInviteSubmit)} className="space-y-4 py-4">
              <FormField
                control={inviteForm.control}
                name="email"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>이메일</FormLabel>
                    <FormControl>
                      <Input
                        type="email"
                        placeholder="admin@example.com"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={inviteForm.control}
                name="role"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>역할</FormLabel>
                    <Select onValueChange={field.onChange} defaultValue={field.value}>
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="역할을 선택하세요" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        <SelectItem value="admin">Admin (관리자)</SelectItem>
                        <SelectItem value="manager">Manager (매니저)</SelectItem>
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <DialogFooter className="pt-4">
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => {
                    setInviteOpen(false);
                    inviteForm.reset();
                  }}
                >
                  취소
                </Button>
                <Button type="submit" disabled={inviteMutation.isPending}>
                  {inviteMutation.isPending ? (
                    <>
                      <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                      발송 중...
                    </>
                  ) : (
                    "초대 발송"
                  )}
                </Button>
              </DialogFooter>
            </form>
          </Form>
        </DialogContent>
      </Dialog>
    </div>
  );
}
