import { useState } from "react";
import { Link } from "react-router-dom";
import { Search, Plus, ChevronUp, ChevronDown, Upload, Video, Eye, EyeOff, BarChart3 } from "lucide-react";
import { toast } from "sonner";

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

import { useAdminVideos } from "../hook/use_admin_videos";
import { updateAdminVideosBulk } from "../admin_api";
import type { AdminListReq, AdminVideoSummary } from "../types";

// Backend allows: id, title, views, video_state, video_access, created_at
type SortField = "id" | "title" | "views" | "video_state" | "video_access" | "created_at";
type SortOrder = "asc" | "desc";

export function AdminVideosPage() {
  const [params, setParams] = useState<AdminListReq>({
    page: 1,
    size: 20,
  });
  const [searchInput, setSearchInput] = useState("");
  const [sortField, setSortField] = useState<SortField>("created_at");
  const [sortOrder, setSortOrder] = useState<SortOrder>("desc");

  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());

  const [bulkEditOpen, setBulkEditOpen] = useState(false);
  const [bulkState, setBulkState] = useState<string>("");
  const [bulkAccess, setBulkAccess] = useState<string>("");
  const [bulkUpdating, setBulkUpdating] = useState(false);

  const { data, isLoading, isError, refetch } = useAdminVideos({
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
      setSelectedIds(new Set(data.items.map((v) => v.id)));
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

    if (!bulkState && !bulkAccess) {
      toast.error("Please select at least one field to update");
      return;
    }

    const items = Array.from(selectedIds).map((id) => ({
      id,
      ...(bulkState && { video_state: bulkState as "ready" | "open" | "close" }),
      ...(bulkAccess && { video_access: bulkAccess as "public" | "paid" | "private" | "promote" }),
    }));

    setBulkUpdating(true);
    try {
      const result = await updateAdminVideosBulk({ items });
      toast.success(`Updated ${result.summary.success} of ${result.summary.total} videos`);
      setBulkEditOpen(false);
      setBulkState("");
      setBulkAccess("");
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

  const getStateBadgeVariant = (state: string) => {
    switch (state) {
      case "ready":
        return "secondary" as const;
      case "open":
        return "default" as const;
      case "close":
        return "outline" as const;
      default:
        return "outline" as const;
    }
  };

  const getAccessBadgeVariant = (access: string) => {
    switch (access) {
      case "public":
        return "default" as const;
      case "promote":
        return "default" as const;
      case "paid":
        return "secondary" as const;
      case "private":
        return "outline" as const;
      default:
        return "outline" as const;
    }
  };

  const allSelected = !!(data?.items && data.items.length > 0 && data.items.every((v) => selectedIds.has(v.id)));

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Videos Management</h1>
        <div className="flex gap-2">
          <Button variant="outline" asChild>
            <Link to="/admin/videos/stats">
              <BarChart3 className="mr-2 h-4 w-4" />
              Stats
            </Link>
          </Button>
          <Button variant="outline" asChild>
            <Link to="/admin/videos/bulk-create">
              <Upload className="mr-2 h-4 w-4" />
              Bulk Create
            </Link>
          </Button>
          <Button asChild>
            <Link to="/admin/videos/new">
              <Plus className="mr-2 h-4 w-4" />
              Add Video
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
              placeholder="Search by title..."
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
            <Video className="mr-2 h-4 w-4" />
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
                onClick={() => handleSort("title")}
              >
                Title
                <SortIcon field="title" />
              </th>
              <th
                className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                onClick={() => handleSort("video_state")}
              >
                State
                <SortIcon field="video_state" />
              </th>
              <th
                className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                onClick={() => handleSort("video_access")}
              >
                Access
                <SortIcon field="video_access" />
              </th>
              <th
                className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                onClick={() => handleSort("views")}
              >
                Views
                <SortIcon field="views" />
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
                    <Skeleton className="h-4 w-48" />
                  </td>
                  <td className="p-4">
                    <Skeleton className="h-5 w-14" />
                  </td>
                  <td className="p-4">
                    <Skeleton className="h-5 w-14" />
                  </td>
                  <td className="p-4">
                    <Skeleton className="h-4 w-12" />
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
                <td colSpan={8} className="p-4 text-center text-destructive">
                  Failed to load videos
                </td>
              </tr>
            ) : data?.items.length === 0 ? (
              <tr>
                <td colSpan={8} className="p-4 text-center text-muted-foreground">
                  No videos found
                </td>
              </tr>
            ) : (
              data?.items.map((video: AdminVideoSummary) => (
                <tr key={video.id} className="border-b hover:bg-muted/50">
                  <td className="p-4">
                    <Checkbox
                      checked={selectedIds.has(video.id)}
                      onCheckedChange={(checked) => handleSelectOne(video.id, !!checked)}
                    />
                  </td>
                  <td className="p-4">{video.id}</td>
                  <td className="p-4">
                    <div className="max-w-xs truncate" title={video.title}>
                      {video.title}
                    </div>
                  </td>
                  <td className="p-4">
                    <Badge variant={getStateBadgeVariant(video.video_state)}>
                      {video.video_state}
                    </Badge>
                  </td>
                  <td className="p-4">
                    <Badge variant={getAccessBadgeVariant(video.video_access)}>
                      {video.video_access === "public" && <Eye className="mr-1 h-3 w-3" />}
                      {video.video_access === "private" && <EyeOff className="mr-1 h-3 w-3" />}
                      {video.video_access}
                    </Badge>
                  </td>
                  <td className="p-4">{video.views.toLocaleString()}</td>
                  <td className="p-4">
                    {new Date(video.created_at).toLocaleDateString()}
                  </td>
                  <td className="p-4">
                    <Button variant="ghost" size="sm" asChild>
                      <Link to={`/admin/videos/${video.id}`}>Edit</Link>
                    </Button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      {data?.pagination && data.pagination.total_pages > 1 && (
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

            {Array.from({ length: Math.min(5, data.pagination.total_pages) }, (_, i) => {
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
                  handlePageChange(Math.min(data.pagination.total_pages, params.page! + 1))
                }
                className={
                  params.page === data.pagination.total_pages
                    ? "pointer-events-none opacity-50"
                    : "cursor-pointer"
                }
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      )}

      {/* Stats */}
      {data?.pagination && (
        <p className="text-sm text-muted-foreground text-center">
          Showing {data.items.length} of {data.pagination.total_count} videos
        </p>
      )}

      {/* Bulk Edit Dialog */}
      <Dialog open={bulkEditOpen} onOpenChange={setBulkEditOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Edit Videos</DialogTitle>
            <DialogDescription>
              Update {selectedIds.size} selected videos. Leave empty to skip.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label>State</Label>
              <Select
                value={bulkState}
                onValueChange={setBulkState}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select state (optional)" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="ready">Ready</SelectItem>
                  <SelectItem value="open">Open</SelectItem>
                  <SelectItem value="close">Close</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Access</Label>
              <Select
                value={bulkAccess}
                onValueChange={setBulkAccess}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select access (optional)" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="public">Public</SelectItem>
                  <SelectItem value="paid">Paid</SelectItem>
                  <SelectItem value="private">Private</SelectItem>
                  <SelectItem value="promote">Promote</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setBulkEditOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleBulkUpdate} disabled={bulkUpdating || (!bulkState && !bulkAccess)}>
              {bulkUpdating ? "Updating..." : "Update"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
