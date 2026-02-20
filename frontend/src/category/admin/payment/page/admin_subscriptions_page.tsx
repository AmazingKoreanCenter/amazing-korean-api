import { useState } from "react";
import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { Search, ChevronUp, ChevronDown, CreditCard, Receipt, Gift } from "lucide-react";

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

import { useAdminSubscriptions } from "../hook/use_admin_payment";
import type { AdminSubListReq } from "../types";

type SortField = "id" | "created_at" | "status" | "billing_interval" | "price";
type SortOrder = "asc" | "desc";

const statusBadgeVariant = (status: string) => {
  switch (status) {
    case "active":
      return "success" as const;
    case "trialing":
      return "blue" as const;
    case "past_due":
      return "warning" as const;
    case "paused":
      return "orange" as const;
    case "canceled":
      return "destructive" as const;
    default:
      return "outline" as const;
  }
};

const formatCents = (cents: number) =>
  `$${(cents / 100).toFixed(2)}`;

export function AdminSubscriptionsPage() {
  const { t } = useTranslation();
  const [params, setParams] = useState<AdminSubListReq>({ page: 1, size: 20 });
  const [searchInput, setSearchInput] = useState("");
  const [statusFilter, setStatusFilter] = useState<string>("");
  const [sortField, setSortField] = useState<SortField>("id");
  const [sortOrder, setSortOrder] = useState<SortOrder>("desc");

  const { data, isLoading, isError } = useAdminSubscriptions({
    ...params,
    status: statusFilter || undefined,
    sort: sortField,
    order: sortOrder,
  });

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    setParams((prev) => ({ ...prev, page: 1, q: searchInput || undefined }));
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
  };

  const SortIcon = ({ field }: { field: SortField }) => {
    if (sortField !== field) return null;
    return sortOrder === "asc" ? (
      <ChevronUp className="ml-1 h-4 w-4 inline" />
    ) : (
      <ChevronDown className="ml-1 h-4 w-4 inline" />
    );
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">{t("admin.payment.subscriptions")}</h1>
        <div className="flex gap-2">
          <Button variant="outline" asChild>
            <Link to="/admin/payment/transactions">
              <Receipt className="mr-2 h-4 w-4" />
              {t("admin.payment.transactions")}
            </Link>
          </Button>
          <Button variant="outline" asChild>
            <Link to="/admin/payment/grants">
              <Gift className="mr-2 h-4 w-4" />
              {t("admin.payment.manualGrants")}
            </Link>
          </Button>
        </div>
      </div>

      {/* Search & Filter */}
      <div className="bg-card rounded-lg border border-foreground/15 p-4 shadow-sm">
        <div className="flex items-center gap-4">
          <form onSubmit={handleSearch} className="flex gap-2 max-w-md">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                placeholder={t("admin.payment.searchByEmailOrNickname")}
                value={searchInput}
                onChange={(e) => setSearchInput(e.target.value)}
                className="pl-9 border-foreground/20"
              />
            </div>
            <Button type="submit" variant="secondary">{t("admin.payment.search")}</Button>
          </form>

          <Select value={statusFilter} onValueChange={(v) => {
            setStatusFilter(v === "all" ? "" : v);
            setParams((prev) => ({ ...prev, page: 1 }));
          }}>
            <SelectTrigger className="w-[160px]">
              <SelectValue placeholder={t("admin.payment.allStatus")} />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">{t("admin.payment.allStatus")}</SelectItem>
              <SelectItem value="trialing">{t("admin.payment.trialing")}</SelectItem>
              <SelectItem value="active">{t("admin.payment.active")}</SelectItem>
              <SelectItem value="past_due">{t("admin.payment.pastDue")}</SelectItem>
              <SelectItem value="paused">{t("admin.payment.paused")}</SelectItem>
              <SelectItem value="canceled">{t("admin.payment.canceled")}</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      {/* Table */}
      <div className="bg-card rounded-lg border overflow-hidden shadow-sm">
        <table className="w-full text-sm">
          <thead className="border-b-2 bg-secondary">
            <tr>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                  onClick={() => handleSort("id")}>
                {t("admin.payment.colId")}<SortIcon field="id" />
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">{t("admin.payment.colEmail")}</th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                  onClick={() => handleSort("status")}>
                {t("admin.payment.colStatus")}<SortIcon field="status" />
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                  onClick={() => handleSort("billing_interval")}>
                {t("admin.payment.colInterval")}<SortIcon field="billing_interval" />
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                  onClick={() => handleSort("price")}>
                {t("admin.payment.colPrice")}<SortIcon field="price" />
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">{t("admin.payment.colPeriodEnd")}</th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground cursor-pointer hover:bg-secondary/80"
                  onClick={() => handleSort("created_at")}>
                {t("admin.payment.colCreated")}<SortIcon field="created_at" />
              </th>
              <th className="px-4 py-3 text-left font-semibold text-secondary-foreground">{t("admin.payment.colActions")}</th>
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <tr key={i} className="border-b">
                  {Array.from({ length: 8 }).map((__, j) => (
                    <td key={j} className="px-4 py-3"><Skeleton className="h-4 w-20" /></td>
                  ))}
                </tr>
              ))
            ) : isError ? (
              <tr>
                <td colSpan={8} className="p-4 text-center text-destructive">
                  {t("admin.payment.failedLoad")}
                </td>
              </tr>
            ) : data?.items.length === 0 ? (
              <tr>
                <td colSpan={8} className="p-4 text-center text-muted-foreground">
                  {t("admin.payment.noSubscriptions")}
                </td>
              </tr>
            ) : (
              data?.items.map((sub) => (
                <tr key={sub.subscription_id} className="border-b hover:bg-accent/10">
                  <td className="px-4 py-3">{sub.subscription_id}</td>
                  <td className="px-4 py-3">{sub.user_email}</td>
                  <td className="px-4 py-3">
                    <Badge variant={statusBadgeVariant(sub.status)}>{sub.status}</Badge>
                  </td>
                  <td className="px-4 py-3">{sub.billing_interval}</td>
                  <td className="px-4 py-3">{formatCents(sub.current_price_cents)}</td>
                  <td className="px-4 py-3">
                    {sub.current_period_end
                      ? new Date(sub.current_period_end).toLocaleDateString()
                      : "-"}
                  </td>
                  <td className="px-4 py-3">
                    {new Date(sub.created_at).toLocaleDateString()}
                  </td>
                  <td className="px-4 py-3">
                    <Button variant="ghost" size="sm" asChild>
                      <Link to={`/admin/payment/subscriptions/${sub.subscription_id}`}>
                        <CreditCard className="mr-1 h-3 w-3" />
                        {t("admin.payment.detail")}
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
                onClick={() => handlePageChange(Math.min(data.meta.total_pages, (params.page ?? 1) + 1))}
                className={params.page === data.meta.total_pages ? "pointer-events-none opacity-50" : "cursor-pointer"}
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      )}

      {data?.meta && (
        <p className="text-sm text-muted-foreground text-center">
          {t("admin.payment.showing", { count: data.items.length, total: data.meta.total_count })}
        </p>
      )}
    </div>
  );
}
