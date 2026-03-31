import { useState } from "react";
import { Link } from "react-router-dom";
import { Plus, BookOpen } from "lucide-react";
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
import { useAdminLessons } from "../hook/use_admin_lessons";
import { updateAdminLessonsBulk } from "../admin_api";
import type { AdminLessonRes, LessonState, LessonAccess } from "../lesson/types";

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

const columns: DataTableColumn<AdminLessonRes>[] = [
  { key: "id", header: "ID", sortField: "lesson_id", skeletonWidth: "w-8", render: (l) => l.lesson_id },
  {
    key: "idx",
    header: "IDX",
    sortField: "lesson_idx",
    skeletonWidth: "w-24",
    render: (l) => (
      <code className="text-xs bg-muted px-1 py-0.5 rounded">{l.lesson_idx}</code>
    ),
  },
  {
    key: "title",
    header: "Title",
    sortField: "lesson_title",
    skeletonWidth: "w-32",
    render: (l) => (
      <div className="max-w-xs truncate" title={l.lesson_title ?? ""}>
        {l.lesson_title || "-"}
      </div>
    ),
  },
  {
    key: "subtitle",
    header: "Subtitle",
    sortField: "lesson_subtitle",
    skeletonWidth: "w-32",
    render: (l) => (
      <div className="max-w-xs truncate text-muted-foreground" title={l.lesson_subtitle ?? ""}>
        {l.lesson_subtitle || "-"}
      </div>
    ),
  },
  {
    key: "state",
    header: "State",
    sortField: "lesson_state",
    skeletonWidth: "w-14",
    render: (l) => <Badge variant={getStateBadgeVariant(l.lesson_state)}>{l.lesson_state}</Badge>,
  },
  {
    key: "access",
    header: "Access",
    sortField: "lesson_access",
    skeletonWidth: "w-14",
    render: (l) => <Badge variant={getAccessBadgeVariant(l.lesson_access)}>{l.lesson_access}</Badge>,
  },
  {
    key: "created_at",
    header: "Created At",
    sortField: "created_at",
    skeletonWidth: "w-28",
    render: (l) => new Date(l.lesson_created_at).toLocaleDateString(),
  },
  {
    key: "actions",
    header: "Actions",
    skeletonWidth: "w-16",
    render: (l) => (
      <Button variant="ghost" size="sm" asChild>
        <Link to={`/admin/lessons/${l.lesson_id}`}>Edit</Link>
      </Button>
    ),
  },
];

export function AdminLessonsPage() {
  const table = useDataTable({ defaultSortField: "lesson_id" });

  const [bulkEditOpen, setBulkEditOpen] = useState(false);
  const [bulkState, setBulkState] = useState<string>("");
  const [bulkAccess, setBulkAccess] = useState<string>("");
  const [bulkUpdating, setBulkUpdating] = useState(false);

  const { data, isLoading, isError, refetch } = useAdminLessons({
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
      lesson_id: id,
      ...(bulkState && { lesson_state: bulkState as LessonState }),
      ...(bulkAccess && { lesson_access: bulkAccess as LessonAccess }),
    }));

    setBulkUpdating(true);
    try {
      const result = await updateAdminLessonsBulk({ items });
      toast.success(
        `Updated ${result.success_count} of ${result.success_count + result.failure_count} lessons`,
      );
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
        <h1 className="text-2xl font-bold">Lessons Management</h1>
        <div className="flex gap-2">
          <Button variant="outline" asChild>
            <Link to="/admin/lessons/bulk-create">Bulk Create</Link>
          </Button>
          <Button asChild>
            <Link to="/admin/lessons/new">
              <Plus className="mr-2 h-4 w-4" />
              Add Lesson
            </Link>
          </Button>
        </div>
      </div>

      <DataTable
        columns={columns}
        data={data?.list}
        isLoading={isLoading}
        isError={isError}
        entityName="lessons"
        getId={(l) => l.lesson_id}
        searchPlaceholder="Search by idx, title..."
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
            <BookOpen className="mr-2 h-4 w-4" />
            Edit {table.selectedIds.size} Selected
          </Button>
        }
        page={table.params.page}
        totalPages={data?.total_pages}
        totalCount={data?.total}
        onPageChange={table.handlePageChange}
      />

      {/* Bulk Edit Dialog */}
      <Dialog open={bulkEditOpen} onOpenChange={setBulkEditOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Edit Lessons</DialogTitle>
            <DialogDescription>
              Update {table.selectedIds.size} selected lessons. Leave empty to skip.
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
            <Button
              onClick={handleBulkUpdate}
              disabled={bulkUpdating || (!bulkState && !bulkAccess)}
            >
              {bulkUpdating ? "Updating..." : "Update"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
