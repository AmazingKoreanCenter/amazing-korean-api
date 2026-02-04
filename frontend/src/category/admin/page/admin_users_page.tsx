import { useState } from "react";
import { Link } from "react-router-dom";
import { Search, Plus, ChevronUp, ChevronDown, Upload, Users, BarChart3, LogIn, UserPlus, Loader2 } from "lucide-react";
import { toast } from "sonner";
import { useMutation } from "@tanstack/react-query";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { Checkbox } from "@/components/ui/checkbox";
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
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";

import { useAdminUsers } from "../hook/use_admin_users";
import { updateAdminUsersBulk, createAdminInvite } from "../admin_api";
import type { AdminListReq, AdminUserSummary, UpgradeInviteReq } from "../types";
import { upgradeInviteReqSchema } from "../types";

// 백엔드가 허용하는 정렬 필드: id, created_at, email, nickname, role
type SortField = "id" | "email" | "nickname" | "role" | "created_at";
type SortOrder = "asc" | "desc";

export function AdminUsersPage() {
  const [params, setParams] = useState<AdminListReq>({
    page: 1,
    size: 20,
  });
  const [searchInput, setSearchInput] = useState("");
  const [sortField, setSortField] = useState<SortField>("created_at");
  const [sortOrder, setSortOrder] = useState<SortOrder>("desc");

  // 체크박스 선택 상태
  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());

  // 벌크 수정 다이얼로그
  const [bulkEditOpen, setBulkEditOpen] = useState(false);
  const [bulkRole, setBulkRole] = useState<string>("");
  const [bulkState, setBulkState] = useState<string>("");
  const [bulkUpdating, setBulkUpdating] = useState(false);

  // 관리자 초대 다이얼로그
  const [inviteOpen, setInviteOpen] = useState(false);

  // 초대 폼
  const inviteForm = useForm<UpgradeInviteReq>({
    resolver: zodResolver(upgradeInviteReqSchema),
    defaultValues: {
      email: "",
      role: "manager",
    },
  });

  // 초대 뮤테이션
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
    ...params,
    sort: sortField,
    order: sortOrder,
  });

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    setParams((prev) => ({ ...prev, page: 1, q: searchInput || undefined }));
    setSelectedIds(new Set());
  };

  const handleSort = (field: SortField) => {
    if (sortField === field) {
      setSortOrder((prev) => (prev === "asc" ? "desc" : "asc"));
    } else {
      setSortField(field);
      setSortOrder("desc");
    }
  };

  const handlePageChange = (page: number) => {
    setParams((prev) => ({ ...prev, page }));
    setSelectedIds(new Set());
  };

  const handleSelectAll = (checked: boolean) => {
    if (checked && data?.items) {
      setSelectedIds(new Set(data.items.map((u) => u.id)));
    } else {
      setSelectedIds(new Set());
    }
  };

  const handleSelectOne = (id: number, checked: boolean) => {
    const newSet = new Set(selectedIds);
    if (checked) {
      newSet.add(id);
    } else {
      newSet.delete(id);
    }
    setSelectedIds(newSet);
  };

  const handleBulkUpdate = async () => {
    if (selectedIds.size === 0) return;

    const items = Array.from(selectedIds).map((id) => ({
      id,
      ...(bulkRole ? { user_auth: bulkRole as "learner" | "manager" | "admin" } : {}),
      ...(bulkState ? { user_state: bulkState === "active" } : {}),
    }));

    // 아무 변경사항이 없으면 return
    if (!bulkRole && !bulkState) {
      toast.error("Please select at least one field to update");
      return;
    }

    setBulkUpdating(true);
    try {
      const result = await updateAdminUsersBulk({ items });
      toast.success(`Updated ${result.summary.success} of ${result.summary.total} users`);
      setBulkEditOpen(false);
      setBulkRole("");
      setBulkState("");
      setSelectedIds(new Set());
      refetch();
    } catch {
      toast.error("Bulk update failed");
    } finally {
      setBulkUpdating(false);
    }
  };

  const SortIcon = ({ field }: { field: SortField }) => {
    if (sortField !== field) return null;
    return sortOrder === "asc" ? (
      <ChevronUp className="ml-1 h-4 w-4 inline" />
    ) : (
      <ChevronDown className="ml-1 h-4 w-4 inline" />
    );
  };

  const getRoleBadgeVariant = (role: string) => {
    switch (role) {
      case "HYMN":
        return "destructive";
      case "admin":
        return "default";
      case "manager":
        return "secondary";
      default:
        return "outline";
    }
  };

  const allSelected = !!(data?.items && data.items.length > 0 && data.items.every((u) => selectedIds.has(u.id)));

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

      {/* Search & Bulk Actions */}
      <div className="flex items-center justify-between gap-4">
        <form onSubmit={handleSearch} className="flex gap-2 max-w-md">
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              placeholder="Search by email or nickname..."
              value={searchInput}
              onChange={(e) => setSearchInput(e.target.value)}
              className="pl-9"
            />
          </div>
          <Button type="submit" variant="secondary">
            Search
          </Button>
        </form>

        {selectedIds.size > 0 && (
          <Button
            variant="outline"
            onClick={() => setBulkEditOpen(true)}
          >
            <Users className="mr-2 h-4 w-4" />
            Edit {selectedIds.size} Selected
          </Button>
        )}
      </div>

      {/* Table */}
      <div className="rounded-md border">
        <table className="w-full text-sm">
          <thead className="border-b bg-muted/50">
            <tr>
              <th className="h-10 px-4 text-left font-medium w-10">
                <Checkbox
                  checked={allSelected}
                  onCheckedChange={handleSelectAll}
                />
              </th>
              <th
                className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                onClick={() => handleSort("id")}
              >
                ID
                <SortIcon field="id" />
              </th>
              <th
                className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                onClick={() => handleSort("email")}
              >
                Email
                <SortIcon field="email" />
              </th>
              <th
                className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                onClick={() => handleSort("nickname")}
              >
                Nickname
                <SortIcon field="nickname" />
              </th>
              <th
                className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                onClick={() => handleSort("role")}
              >
                Role
                <SortIcon field="role" />
              </th>
              <th
                className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                onClick={() => handleSort("created_at")}
              >
                Created At
                <SortIcon field="created_at" />
              </th>
              <th className="h-10 px-4 text-left font-medium">Actions</th>
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <tr key={i} className="border-b">
                  <td className="p-4">
                    <Skeleton className="h-4 w-4" />
                  </td>
                  <td className="p-4">
                    <Skeleton className="h-4 w-8" />
                  </td>
                  <td className="p-4">
                    <Skeleton className="h-4 w-40" />
                  </td>
                  <td className="p-4">
                    <Skeleton className="h-4 w-24" />
                  </td>
                  <td className="p-4">
                    <Skeleton className="h-5 w-16" />
                  </td>
                  <td className="p-4">
                    <Skeleton className="h-4 w-28" />
                  </td>
                  <td className="p-4">
                    <Skeleton className="h-8 w-16" />
                  </td>
                </tr>
              ))
            ) : isError ? (
              <tr>
                <td colSpan={7} className="p-4 text-center text-destructive">
                  Failed to load users
                </td>
              </tr>
            ) : data?.items.length === 0 ? (
              <tr>
                <td colSpan={7} className="p-4 text-center text-muted-foreground">
                  No users found
                </td>
              </tr>
            ) : (
              data?.items.map((user: AdminUserSummary) => (
                <tr key={user.id} className="border-b hover:bg-muted/50">
                  <td className="p-4">
                    <Checkbox
                      checked={selectedIds.has(user.id)}
                      onCheckedChange={(checked) => handleSelectOne(user.id, !!checked)}
                    />
                  </td>
                  <td className="p-4">{user.id}</td>
                  <td className="p-4">{user.email}</td>
                  <td className="p-4">{user.nickname || "-"}</td>
                  <td className="p-4">
                    <Badge variant={getRoleBadgeVariant(user.role)}>
                      {user.role}
                    </Badge>
                  </td>
                  <td className="p-4">
                    {new Date(user.created_at).toLocaleDateString()}
                  </td>
                  <td className="p-4">
                    <Button variant="ghost" size="sm" asChild>
                      <Link to={`/admin/users/${user.id}`}>Edit</Link>
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
                onClick={() => handlePageChange(Math.max(1, params.page! - 1))}
                className={
                  params.page === 1
                    ? "pointer-events-none opacity-50"
                    : "cursor-pointer"
                }
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
                onClick={() =>
                  handlePageChange(Math.min(data.meta.total_pages, params.page! + 1))
                }
                className={
                  params.page === data.meta.total_pages
                    ? "pointer-events-none opacity-50"
                    : "cursor-pointer"
                }
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      )}

      {/* Stats */}
      {data?.meta && (
        <p className="text-sm text-muted-foreground text-center">
          Showing {data.items.length} of {data.meta.total_count} users
        </p>
      )}

      {/* Bulk Edit Dialog */}
      <Dialog open={bulkEditOpen} onOpenChange={setBulkEditOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Edit Users</DialogTitle>
            <DialogDescription>
              Update {selectedIds.size} selected users. Leave fields empty to keep current values.
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
