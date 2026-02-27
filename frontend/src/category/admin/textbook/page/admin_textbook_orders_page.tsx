import { useState } from "react";
import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { Search, BookOpen, Eye } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";

import { useAdminTextbookOrders } from "../hook/use_admin_textbook";
import type { AdminTextbookListReq } from "../types";
import type { TextbookOrderStatus } from "@/category/textbook/types";

const statusBadgeVariant = (status: TextbookOrderStatus) => {
  switch (status) {
    case "pending":
      return "warning" as const;
    case "confirmed":
      return "blue" as const;
    case "paid":
      return "success" as const;
    case "printing":
      return "purple" as const;
    case "shipped":
      return "blue" as const;
    case "delivered":
      return "success" as const;
    case "canceled":
      return "destructive" as const;
    default:
      return "outline" as const;
  }
};

export function AdminTextbookOrdersPage() {
  const { t } = useTranslation();
  const [params, setParams] = useState<AdminTextbookListReq>({ page: 1, size: 20 });
  const [searchInput, setSearchInput] = useState("");
  const [statusFilter, setStatusFilter] = useState<string>("");

  const { data, isLoading, isError } = useAdminTextbookOrders({
    ...params,
    status: statusFilter || undefined,
  });

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    setParams((prev) => ({ ...prev, page: 1, q: searchInput || undefined }));
  };

  const handlePageChange = (page: number) => {
    setParams((prev) => ({ ...prev, page }));
  };

  const statuses: TextbookOrderStatus[] = [
    "pending", "confirmed", "paid", "printing", "shipped", "delivered", "canceled",
  ];

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold flex items-center gap-2">
          <BookOpen className="h-6 w-6" />
          {t("admin.textbook.title")}
        </h1>
      </div>

      {/* Search & Filter */}
      <div className="bg-card rounded-lg border border-foreground/15 p-4 shadow-sm">
        <div className="flex items-center gap-4">
          <form onSubmit={handleSearch} className="flex gap-2 max-w-md">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                placeholder={t("admin.textbook.searchPlaceholder")}
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
            value={statusFilter}
            onValueChange={(v) => {
              setStatusFilter(v === "all" ? "" : v);
              setParams((prev) => ({ ...prev, page: 1 }));
            }}
          >
            <SelectTrigger className="w-[160px]">
              <SelectValue placeholder={t("admin.textbook.allStatus")} />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">{t("admin.textbook.allStatus")}</SelectItem>
              {statuses.map((s) => (
                <SelectItem key={s} value={s}>
                  {t(`admin.textbook.status.${s}`)}
                </SelectItem>
              ))}
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
                {t("admin.textbook.colOrderCode")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.textbook.colOrderer")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.textbook.colOrg")}
              </th>
              <th className="px-4 py-3 text-right font-semibold text-secondary-foreground">
                {t("admin.textbook.colQuantity")}
              </th>
              <th className="px-4 py-3 text-right font-semibold text-secondary-foreground">
                {t("admin.textbook.colAmount")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.textbook.colStatus")}
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">
                {t("admin.textbook.colDate")}
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
                  {t("admin.textbook.failedLoad")}
                </td>
              </tr>
            ) : data?.items.length === 0 ? (
              <tr>
                <td colSpan={8} className="p-4 text-center text-muted-foreground">
                  {t("admin.textbook.noOrders")}
                </td>
              </tr>
            ) : (
              data?.items.map((order) => (
                <tr key={order.order_id} className="border-b hover:bg-accent/10">
                  <td className="px-4 py-3 font-mono text-xs">
                    {order.order_code}
                  </td>
                  <td className="px-4 py-3">{order.orderer_name}</td>
                  <td className="px-4 py-3 text-muted-foreground">
                    {order.org_name || "-"}
                  </td>
                  <td className="px-4 py-3 text-right">
                    {order.total_quantity}
                  </td>
                  <td className="px-4 py-3 text-right">
                    {order.total_amount.toLocaleString()}
                  </td>
                  <td className="px-4 py-3">
                    <Badge variant={statusBadgeVariant(order.status)}>
                      {t(`admin.textbook.status.${order.status}`)}
                    </Badge>
                  </td>
                  <td className="px-4 py-3">
                    {new Date(order.created_at).toLocaleDateString()}
                  </td>
                  <td className="px-4 py-3">
                    <Button variant="ghost" size="sm" asChild>
                      <Link to={`/admin/textbook/orders/${order.order_id}`}>
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
                onClick={() =>
                  handlePageChange(
                    Math.min(data.meta.total_pages, (params.page ?? 1) + 1),
                  )
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
