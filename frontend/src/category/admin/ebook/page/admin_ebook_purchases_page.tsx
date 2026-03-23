import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { Search, Tablet, Eye } from "lucide-react";

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
import { Skeleton } from "@/components/ui/skeleton";
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";

import { getAdminEbookPurchases } from "@/category/ebook/ebook_api";
import type { EbookPurchaseStatus } from "@/category/ebook/types";

const statusBadgeVariant = (status: EbookPurchaseStatus) => {
  switch (status) {
    case "pending":
      return "warning" as const;
    case "completed":
      return "success" as const;
    case "refunded":
      return "destructive" as const;
    default:
      return "outline" as const;
  }
};

export function AdminEbookPurchasesPage() {
  const { t } = useTranslation();
  const [page, setPage] = useState(1);
  const [status, setStatus] = useState<string>("");
  const [search, setSearch] = useState("");
  const [searchInput, setSearchInput] = useState("");

  const { data, isLoading, isError } = useQuery({
    queryKey: ["admin", "ebook", "purchases", page, status, search],
    queryFn: () =>
      getAdminEbookPurchases({
        page,
        per_page: 20,
        status: status || undefined,
        search: search || undefined,
      }),
  });

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    setSearch(searchInput);
    setPage(1);
  };

  const handlePageChange = (p: number) => {
    setPage(p);
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold flex items-center gap-2">
          <Tablet className="h-6 w-6" />
          {t("admin.ebook.title")}
        </h1>
      </div>

      {/* Search & Filter */}
      <div className="bg-card rounded-lg border border-foreground/15 p-4 shadow-sm">
        <div className="flex items-center gap-4">
          <form onSubmit={handleSearch} className="flex gap-2 max-w-md">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                placeholder={t("admin.ebook.searchPlaceholder")}
                value={searchInput}
                onChange={(e) => setSearchInput(e.target.value)}
                className="pl-9 border-foreground/20"
              />
            </div>
            <Button type="submit" variant="secondary">
              {t("admin.textbook.search")}
            </Button>
          </form>

          <Select
            value={status}
            onValueChange={(v) => {
              setStatus(v === "all" ? "" : v);
              setPage(1);
            }}
          >
            <SelectTrigger className="w-[160px]">
              <SelectValue placeholder={t("admin.ebook.allStatuses")} />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">{t("admin.ebook.allStatuses")}</SelectItem>
              <SelectItem value="pending">{t("ebook.status.pending")}</SelectItem>
              <SelectItem value="completed">{t("ebook.status.completed")}</SelectItem>
              <SelectItem value="refunded">{t("ebook.status.refunded")}</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      {/* Table */}
      <div className="bg-card rounded-lg border overflow-hidden shadow-sm">
        <table className="w-full text-sm">
          <thead className="border-b-2 bg-secondary">
            <tr>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.ebook.purchaseCode")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.ebook.userId")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.ebook.language")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.ebook.edition")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.ebook.status")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.ebook.payment")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.ebook.date")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.textbook.colActions")}
              </th>
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <tr key={i} className="border-b">
                  {Array.from({ length: 8 }).map((__, j) => (
                    <td key={j} className="px-4 py-3">
                      <Skeleton className="h-4 w-20" />
                    </td>
                  ))}
                </tr>
              ))
            ) : isError ? (
              <tr>
                <td colSpan={8} className="p-4 text-center text-destructive">
                  {t("admin.ebook.loadError")}
                </td>
              </tr>
            ) : data?.items.length === 0 ? (
              <tr>
                <td colSpan={8} className="p-4 text-center text-muted-foreground">
                  {t("admin.ebook.noResults")}
                </td>
              </tr>
            ) : (
              data?.items.map((item) => (
                <tr key={item.purchase_id} className="border-b hover:bg-accent/10">
                  <td className="px-4 py-3 font-mono text-xs">
                    {item.purchase_code}
                  </td>
                  <td className="px-4 py-3">{item.user_id}</td>
                  <td className="px-4 py-3">{item.language}</td>
                  <td className="px-4 py-3 capitalize">{item.edition}</td>
                  <td className="px-4 py-3">
                    <Badge variant={statusBadgeVariant(item.status)}>
                      {t(`ebook.status.${item.status}`)}
                    </Badge>
                  </td>
                  <td className="px-4 py-3 capitalize">
                    {item.payment_method.replace("_", " ")}
                  </td>
                  <td className="px-4 py-3">
                    {new Date(item.created_at).toLocaleDateString()}
                  </td>
                  <td className="px-4 py-3">
                    <Button variant="ghost" size="sm" asChild>
                      <Link to={`/admin/ebook/purchases/${item.purchase_id}`}>
                        <Eye className="mr-1 h-3 w-3" />
                        {t("admin.textbook.detail")}
                      </Link>
                    </Button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      {data && data.meta.total_pages > 1 && (
        <Pagination>
          <PaginationContent>
            <PaginationItem>
              <PaginationPrevious
                onClick={() => handlePageChange(Math.max(1, page - 1))}
                className={page === 1 ? "pointer-events-none opacity-50" : "cursor-pointer"}
              />
            </PaginationItem>
            {(() => {
              const current = page;
              const total = data.meta.total_pages;
              const maxVisible = 5;
              let start = Math.max(1, current - Math.floor(maxVisible / 2));
              const end = Math.min(total, start + maxVisible - 1);
              start = Math.max(1, end - maxVisible + 1);
              return Array.from({ length: end - start + 1 }, (_, i) => {
                const p = start + i;
                return (
                  <PaginationItem key={p}>
                    <PaginationLink
                      onClick={() => handlePageChange(p)}
                      isActive={current === p}
                      className="cursor-pointer"
                    >
                      {p}
                    </PaginationLink>
                  </PaginationItem>
                );
              });
            })()}
            <PaginationItem>
              <PaginationNext
                onClick={() => handlePageChange(Math.min(data.meta.total_pages, page + 1))}
                className={
                  page === data.meta.total_pages
                    ? "pointer-events-none opacity-50"
                    : "cursor-pointer"
                }
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      )}

      {data?.meta && (
        <p className="text-sm text-muted-foreground text-center">
          {t("admin.textbook.showing", {
            count: data.items.length,
            total: data.meta.total_count,
          })}
        </p>
      )}
    </div>
  );
}
