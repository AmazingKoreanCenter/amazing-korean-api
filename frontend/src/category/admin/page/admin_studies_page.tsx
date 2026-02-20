import { useState } from "react";
import { Link } from "react-router-dom";
import { Search, Plus, ChevronUp, ChevronDown, BookOpen, BarChart3 } from "lucide-react";
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

import { useAdminStudies } from "../hook/use_admin_studies";
import { updateAdminStudiesBulk } from "../admin_api";
import type { StudyListReq, AdminStudyRes, StudyState, StudyAccess } from "../types";
import type { StudyProgram } from "../../study/types";

// Backend allows: study_id, study_idx, study_title, study_subtitle, study_program, study_state, study_access, created_at
type SortField = "study_id" | "study_idx" | "study_title" | "study_subtitle" | "study_program" | "study_state" | "study_access" | "created_at";
type SortOrder = "asc" | "desc";

export function AdminStudiesPage() {
  const [params, setParams] = useState<StudyListReq>({
    page: 1,
    size: 20,
  });
  const [searchInput, setSearchInput] = useState("");
  const [sortField, setSortField] = useState<SortField>("study_id");
  const [sortOrder, setSortOrder] = useState<SortOrder>("desc");

  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());

  const [bulkEditOpen, setBulkEditOpen] = useState(false);
  const [bulkState, setBulkState] = useState<string>("");
  const [bulkAccess, setBulkAccess] = useState<string>("");
  const [bulkProgram, setBulkProgram] = useState<string>("");
  const [bulkUpdating, setBulkUpdating] = useState(false);

  const { data, isLoading, isError, refetch } = useAdminStudies({
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
    if (checked && data?.list) {
      setSelectedIds(new Set(data.list.map((s) => s.study_id)));
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

    if (!bulkState && !bulkAccess && !bulkProgram) {
      toast.error("Please select at least one field to update");
      return;
    }

    const items = Array.from(selectedIds).map((id) => ({
      id,
      ...(bulkState && { study_state: bulkState as StudyState }),
      ...(bulkAccess && { study_access: bulkAccess as StudyAccess }),
      ...(bulkProgram && { study_program: bulkProgram as StudyProgram }),
    }));

    setBulkUpdating(true);
    try {
      const result = await updateAdminStudiesBulk({ items });
      toast.success(`Updated ${result.success_count} of ${result.success_count + result.failure_count} studies`);
      setBulkEditOpen(false);
      setBulkState("");
      setBulkAccess("");
      setBulkProgram("");
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

  const getProgramBadgeVariant = (program: string) => {
    switch (program) {
      case "basic_pronunciation":
      case "basic_word":
      case "basic_900":
        return "sky" as const;
      case "topik_read":
      case "topik_listen":
      case "topik_write":
        return "indigo" as const;
      case "tbc":
        return "outline" as const;
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

  const allSelected = !!(data?.list && data.list.length > 0 && data.list.every((s) => selectedIds.has(s.study_id)));

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Studies Management</h1>
        <div className="flex gap-2">
          <Button variant="outline" asChild>
            <Link to="/admin/studies/stats">
              <BarChart3 className="mr-2 h-4 w-4" />
              Stats
            </Link>
          </Button>
          <Button variant="outline" asChild>
            <Link to="/admin/studies/bulk-create">
              Bulk Create
            </Link>
          </Button>
          <Button asChild>
            <Link to="/admin/studies/new">
              <Plus className="mr-2 h-4 w-4" />
              Add Study
            </Link>
          </Button>
        </div>
      </div>

      {/* Search & Bulk Actions */}
      <div className="bg-card rounded-lg border border-foreground/15 p-4 shadow-sm">
        <div className="flex items-center justify-between gap-4">
          <form onSubmit={handleSearch} className="flex gap-2 max-w-md">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                placeholder="Search by idx, title..."
                value={searchInput}
                onChange={(e) => setSearchInput(e.target.value)}
                className="pl-9 border-foreground/20"
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
              <BookOpen className="mr-2 h-4 w-4" />
              Edit {selectedIds.size} Selected
            </Button>
          )}
        </div>
      </div>

      {/* Table */}
      <div className="bg-card rounded-lg border overflow-hidden shadow-sm">
        <table className="w-full text-sm">
          <thead className="border-b-2 bg-secondary">
            <tr>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground w-10">
                <Checkbox
                  checked={allSelected}
                  onCheckedChange={handleSelectAll}
                />
              </th>
              <th
                className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                onClick={() => handleSort("study_id")}
              >
                ID
                <SortIcon field="study_id" />
              </th>
              <th
                className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                onClick={() => handleSort("study_idx")}
              >
                IDX
                <SortIcon field="study_idx" />
              </th>
              <th
                className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                onClick={() => handleSort("study_title")}
              >
                Title
                <SortIcon field="study_title" />
              </th>
              <th
                className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                onClick={() => handleSort("study_subtitle")}
              >
                Subtitle
                <SortIcon field="study_subtitle" />
              </th>
              <th
                className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                onClick={() => handleSort("study_program")}
              >
                Program
                <SortIcon field="study_program" />
              </th>
              <th
                className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                onClick={() => handleSort("study_state")}
              >
                State
                <SortIcon field="study_state" />
              </th>
              <th
                className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                onClick={() => handleSort("study_access")}
              >
                Access
                <SortIcon field="study_access" />
              </th>
              <th
                className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                onClick={() => handleSort("created_at")}
              >
                Created At
                <SortIcon field="created_at" />
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">Actions</th>
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <tr key={i} className="border-b">
                  <td className="px-4 py-3">
                    <Skeleton className="h-4 w-4" />
                  </td>
                  <td className="px-4 py-3">
                    <Skeleton className="h-4 w-8" />
                  </td>
                  <td className="px-4 py-3">
                    <Skeleton className="h-4 w-24" />
                  </td>
                  <td className="px-4 py-3">
                    <Skeleton className="h-4 w-32" />
                  </td>
                  <td className="px-4 py-3">
                    <Skeleton className="h-4 w-32" />
                  </td>
                  <td className="px-4 py-3">
                    <Skeleton className="h-5 w-12" />
                  </td>
                  <td className="px-4 py-3">
                    <Skeleton className="h-5 w-14" />
                  </td>
                  <td className="px-4 py-3">
                    <Skeleton className="h-5 w-14" />
                  </td>
                  <td className="px-4 py-3">
                    <Skeleton className="h-4 w-28" />
                  </td>
                  <td className="px-4 py-3">
                    <Skeleton className="h-8 w-16" />
                  </td>
                </tr>
              ))
            ) : isError ? (
              <tr>
                <td colSpan={10} className="p-4 text-center text-destructive">
                  Failed to load studies
                </td>
              </tr>
            ) : data?.list.length === 0 ? (
              <tr>
                <td colSpan={10} className="p-4 text-center text-muted-foreground">
                  No studies found
                </td>
              </tr>
            ) : (
              data?.list.map((study: AdminStudyRes) => (
                <tr key={study.study_id} className="border-b hover:bg-accent/10">
                  <td className="px-4 py-3">
                    <Checkbox
                      checked={selectedIds.has(study.study_id)}
                      onCheckedChange={(checked) => handleSelectOne(study.study_id, !!checked)}
                    />
                  </td>
                  <td className="px-4 py-3">{study.study_id}</td>
                  <td className="px-4 py-3">
                    <code className="text-xs bg-muted px-1 py-0.5 rounded">
                      {study.study_idx}
                    </code>
                  </td>
                  <td className="px-4 py-3">
                    <div className="max-w-xs truncate" title={study.study_title ?? ""}>
                      {study.study_title || "-"}
                    </div>
                  </td>
                  <td className="px-4 py-3">
                    <div className="max-w-xs truncate text-muted-foreground" title={study.study_subtitle ?? ""}>
                      {study.study_subtitle || "-"}
                    </div>
                  </td>
                  <td className="px-4 py-3">
                    <Badge variant={getProgramBadgeVariant(study.study_program)}>
                      {study.study_program}
                    </Badge>
                  </td>
                  <td className="px-4 py-3">
                    <Badge variant={getStateBadgeVariant(study.study_state)}>
                      {study.study_state}
                    </Badge>
                  </td>
                  <td className="px-4 py-3">
                    <Badge variant={getAccessBadgeVariant(study.study_access)}>
                      {study.study_access}
                    </Badge>
                  </td>
                  <td className="px-4 py-3">
                    {new Date(study.study_created_at).toLocaleDateString()}
                  </td>
                  <td className="px-4 py-3">
                    <Button variant="ghost" size="sm" asChild>
                      <Link to={`/admin/studies/${study.study_id}`}>Edit</Link>
                    </Button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      {data && data.total_pages > 1 && (
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

            {Array.from({ length: Math.min(5, data.total_pages) }, (_, i) => {
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
                  handlePageChange(Math.min(data.total_pages, params.page! + 1))
                }
                className={
                  params.page === data.total_pages
                    ? "pointer-events-none opacity-50"
                    : "cursor-pointer"
                }
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      )}

      {/* Stats */}
      {data && (
        <p className="text-sm text-muted-foreground text-center">
          Showing {data.list.length} of {data.total} studies
        </p>
      )}

      {/* Bulk Edit Dialog */}
      <Dialog open={bulkEditOpen} onOpenChange={setBulkEditOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Bulk Edit Studies</DialogTitle>
            <DialogDescription>
              Update {selectedIds.size} selected studies. Leave empty to skip.
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

            <div className="space-y-2">
              <Label>Program</Label>
              <Select
                value={bulkProgram}
                onValueChange={setBulkProgram}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select program (optional)" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="basic_pronunciation">Basic Pronunciation</SelectItem>
                  <SelectItem value="basic_word">Basic Word</SelectItem>
                  <SelectItem value="basic_900">Basic 900</SelectItem>
                  <SelectItem value="topik_read">TOPIK Read</SelectItem>
                  <SelectItem value="topik_listen">TOPIK Listen</SelectItem>
                  <SelectItem value="topik_write">TOPIK Write</SelectItem>
                  <SelectItem value="tbc">TBC</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setBulkEditOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleBulkUpdate} disabled={bulkUpdating || (!bulkState && !bulkAccess && !bulkProgram)}>
              {bulkUpdating ? "Updating..." : "Update"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
