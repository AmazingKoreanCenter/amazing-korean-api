import { useState } from "react";
import { Link } from "react-router-dom";
import { Plus, Upload, Video, Eye, EyeOff, BarChart3 } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
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

import { DataTable, useDataTable, type DataTableColumn } from "@/components/blocks/data_table";
import { useAdminVideos } from "../hook/use_admin_videos";
import { updateAdminVideosBulk } from "../admin_api";
import type { AdminVideoSummary } from "../types";

const getStateBadgeVariant = (state: string) => {
  switch (state) {
    case "open":
      return "success" as const;
    case "ready":
      return "warning" as const;
    case "close":
      return "destructive" as const;
    default:
      return "outline" as const;
  }
};

const getAccessBadgeVariant = (access: string) => {
  switch (access) {
    case "public":
      return "success" as const;
    case "paid":
      return "destructive" as const;
    case "private":
      return "blue" as const;
    case "promote":
      return "warning" as const;
    default:
      return "outline" as const;
  }
};

const columns: DataTableColumn<AdminVideoSummary>[] = [
  { key: "id", header: "ID", sortField: "id", skeletonWidth: "w-8", render: (v) => v.id },
  {
    key: "title",
    header: "Title",
    sortField: "title",
    skeletonWidth: "w-48",
    render: (v) => (
      <div className="max-w-xs truncate" title={v.title}>
        {v.title}
      </div>
    ),
  },
  {
    key: "state",
    header: "State",
    sortField: "video_state",
    skeletonWidth: "w-14",
    render: (v) => <Badge variant={getStateBadgeVariant(v.video_state)}>{v.video_state}</Badge>,
  },
  {
    key: "access",
    header: "Access",
    sortField: "video_access",
    skeletonWidth: "w-14",
    render: (v) => (
      <Badge variant={getAccessBadgeVariant(v.video_access)}>
        {v.video_access === "public" && <Eye className="mr-1 h-3 w-3" />}
        {v.video_access === "private" && <EyeOff className="mr-1 h-3 w-3" />}
        {v.video_access}
      </Badge>
    ),
  },
  {
    key: "views",
    header: "Views",
    sortField: "views",
    skeletonWidth: "w-12",
    render: (v) => v.views.toLocaleString(),
  },
  {
    key: "created_at",
    header: "Created At",
    sortField: "created_at",
    skeletonWidth: "w-28",
    render: (v) => new Date(v.created_at).toLocaleDateString(),
  },
  {
    key: "actions",
    header: "Actions",
    skeletonWidth: "w-16",
    render: (v) => (
      <Button variant="ghost" size="sm" asChild>
        <Link to={`/admin/videos/${v.id}`}>Edit</Link>
      </Button>
    ),
  },
];

export function AdminVideosPage() {
  const table = useDataTable({ defaultSortField: "id" });

  const [bulkEditOpen, setBulkEditOpen] = useState(false);
  const [bulkState, setBulkState] = useState<string>("");
  const [bulkAccess, setBulkAccess] = useState<string>("");
  const [bulkUpdating, setBulkUpdating] = useState(false);

  const { data, isLoading, isError, refetch } = useAdminVideos({
    ...table.params,
    sort: table.sortField,
    order: table.sortOrder,
  });

  const handleBulkUpdate = async () => {
    if (table.selectedIds.size === 0) return;

    if (!bulkState && !bulkAccess) {
      toast.error("Please select at least one field to update");
      return;
    }

    const items = Array.from(table.selectedIds).map((id) => ({
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

      <DataTable
        columns={columns}
        data={data?.items}
        isLoading={isLoading}
        isError={isError}
        entityName="videos"
        getId={(v) => v.id}
        searchPlaceholder="Search by title..."
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
            <Video className="mr-2 h-4 w-4" />
            Edit {table.selectedIds.size} Selected
          </Button>
        }
        page={table.params.page}
        totalPages={data?.pagination?.total_pages}
        totalCount={data?.pagination?.total_count}
        onPageChange={table.handlePageChange}
      />

      {/* Bulk Edit Dialog */}
      <Dialog open={bulkEditOpen} onOpenChange={setBulkEditOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Edit Videos</DialogTitle>
            <DialogDescription>
              Update {table.selectedIds.size} selected videos. Leave empty to skip.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label>State</Label>
              <Select value={bulkState} onValueChange={setBulkState}>
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
              <Select value={bulkAccess} onValueChange={setBulkAccess}>
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
