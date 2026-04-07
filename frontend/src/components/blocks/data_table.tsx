import { useState, type ReactNode } from "react";
import { Search, ChevronUp, ChevronDown } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";

// ─── Hook ────────────────────────────────────────────────────────

interface UseDataTableOptions {
  defaultSortField: string;
  defaultPage?: number;
  defaultSize?: number;
}

export function useDataTable({ defaultSortField, defaultPage = 1, defaultSize = 20 }: UseDataTableOptions) {
  const [params, setParams] = useState({ page: defaultPage, size: defaultSize, q: undefined as string | undefined });
  const [searchInput, setSearchInput] = useState("");
  const [sortField, setSortField] = useState(defaultSortField);
  const [sortOrder, setSortOrder] = useState<"asc" | "desc">("desc");
  const [selectedIds, setSelectedIds] = useState<Set<number>>(new Set());

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    setParams((prev) => ({ ...prev, page: 1, q: searchInput || undefined }));
    setSelectedIds(new Set());
  };

  const handleSort = (field: string) => {
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

  const handleSelectOne = (id: number, checked: boolean) => {
    const newSet = new Set(selectedIds);
    if (checked) {
      newSet.add(id);
    } else {
      newSet.delete(id);
    }
    setSelectedIds(newSet);
  };

  const clearSelection = () => setSelectedIds(new Set());

  return {
    params,
    searchInput,
    setSearchInput,
    sortField,
    sortOrder,
    selectedIds,
    setSelectedIds,
    handleSearch,
    handleSort,
    handlePageChange,
    handleSelectOne,
    clearSelection,
  };
}

// ─── Column Definition ───────────────────────────────────────────

export interface DataTableColumn<T> {
  key: string;
  header: string;
  sortField?: string;
  skeletonWidth: string;
  render: (item: T) => ReactNode;
}

// ─── Component Props ─────────────────────────────────────────────

interface DataTableProps<T> {
  columns: DataTableColumn<T>[];
  data: T[] | undefined;
  isLoading: boolean;
  isError: boolean;
  entityName: string;
  getId: (item: T) => number;

  // Search
  searchPlaceholder: string;
  searchInput: string;
  onSearchInputChange: (value: string) => void;
  onSearch: (e: React.FormEvent) => void;

  // Sort
  sortField: string;
  sortOrder: "asc" | "desc";
  onSort: (field: string) => void;

  // Selection
  selectedIds: Set<number>;
  onSelectAll: (ids: Set<number>) => void;
  onSelectOne: (id: number, checked: boolean) => void;
  bulkActionSlot?: ReactNode;

  // Pagination
  page: number;
  totalPages?: number;
  totalCount?: number;
  onPageChange: (page: number) => void;
}

// ─── Component ───────────────────────────────────────────────────

export function DataTable<T>({
  columns,
  data,
  isLoading,
  isError,
  entityName,
  getId,
  searchPlaceholder,
  searchInput,
  onSearchInputChange,
  onSearch,
  sortField,
  sortOrder,
  onSort,
  selectedIds,
  onSelectAll,
  onSelectOne,
  bulkActionSlot,
  page,
  totalPages,
  totalCount,
  onPageChange,
}: DataTableProps<T>) {
  const colCount = columns.length + 1; // +1 for checkbox column

  const allSelected = !!(
    data &&
    data.length > 0 &&
    data.every((item) => selectedIds.has(getId(item)))
  );

  const handleSelectAllChange = (checked: boolean) => {
    if (checked && data) {
      onSelectAll(new Set(data.map((item) => getId(item))));
    } else {
      onSelectAll(new Set());
    }
  };

  const SortIcon = ({ field }: { field: string }) => {
    if (sortField !== field) return null;
    return sortOrder === "asc" ? (
      <ChevronUp className="ml-1 h-4 w-4 inline" />
    ) : (
      <ChevronDown className="ml-1 h-4 w-4 inline" />
    );
  };

  return (
    <>
      {/* Search & Bulk Actions */}
      <div className="bg-card rounded-lg border border-foreground/15 p-4 shadow-sm">
        <div className="flex items-center justify-between gap-4">
          <form onSubmit={onSearch} className="flex gap-2 max-w-md">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                placeholder={searchPlaceholder}
                value={searchInput}
                onChange={(e) => onSearchInputChange(e.target.value)}
                className="pl-9 border-foreground/20"
              />
            </div>
            <Button type="submit" variant="secondary">
              Search
            </Button>
          </form>

          {selectedIds.size > 0 && bulkActionSlot}
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
                  onCheckedChange={handleSelectAllChange}
                />
              </th>
              {columns.map((col) =>
                col.sortField ? (
                  <th
                    key={col.key}
                    className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                    onClick={() => onSort(col.sortField!)}
                  >
                    {col.header}
                    <SortIcon field={col.sortField} />
                  </th>
                ) : (
                  <th
                    key={col.key}
                    className="px-4 py-3 text-left font-semibold text-secondary-foreground"
                  >
                    {col.header}
                  </th>
                ),
              )}
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <tr key={i} className="border-b">
                  <td className="px-4 py-3">
                    <Skeleton className="h-4 w-4" />
                  </td>
                  {columns.map((col) => (
                    <td key={col.key} className="px-4 py-3">
                      <Skeleton className={`h-4 ${col.skeletonWidth}`} />
                    </td>
                  ))}
                </tr>
              ))
            ) : isError ? (
              <tr>
                <td colSpan={colCount} className="p-4 text-center text-destructive">
                  Failed to load {entityName}
                </td>
              </tr>
            ) : !data || data.length === 0 ? (
              <tr>
                <td colSpan={colCount} className="p-4 text-center text-muted-foreground">
                  No {entityName} found
                </td>
              </tr>
            ) : (
              data.map((item) => {
                const id = getId(item);
                return (
                  <tr key={id} className="border-b hover:bg-accent/10">
                    <td className="px-4 py-3">
                      <Checkbox
                        checked={selectedIds.has(id)}
                        onCheckedChange={(checked) => onSelectOne(id, !!checked)}
                      />
                    </td>
                    {columns.map((col) => (
                      <td key={col.key} className="px-4 py-3">
                        {col.render(item)}
                      </td>
                    ))}
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      {totalPages != null && totalPages > 1 && (
        <Pagination>
          <PaginationContent>
            <PaginationItem>
              <PaginationPrevious
                onClick={() => onPageChange(Math.max(1, page - 1))}
                className={
                  page === 1 ? "pointer-events-none opacity-50" : "cursor-pointer"
                }
              />
            </PaginationItem>

            {(() => {
              const maxVisible = 5;
              let start = Math.max(1, page - Math.floor(maxVisible / 2));
              let end = Math.min(totalPages, start + maxVisible - 1);
              if (end - start + 1 < maxVisible) {
                start = Math.max(1, end - maxVisible + 1);
              }
              const pages = [];
              for (let i = start; i <= end; i++) pages.push(i);
              return pages.map((p) => (
                <PaginationItem key={p}>
                  <PaginationLink
                    onClick={() => onPageChange(p)}
                    isActive={page === p}
                    className="cursor-pointer"
                  >
                    {p}
                  </PaginationLink>
                </PaginationItem>
              ));
            })()}

            <PaginationItem>
              <PaginationNext
                onClick={() => onPageChange(Math.min(totalPages, page + 1))}
                className={
                  page === totalPages
                    ? "pointer-events-none opacity-50"
                    : "cursor-pointer"
                }
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      )}

      {/* Stats */}
      {totalCount != null && data && (
        <p className="text-sm text-muted-foreground text-center">
          Showing {data.length} of {totalCount} {entityName}
        </p>
      )}
    </>
  );
}
