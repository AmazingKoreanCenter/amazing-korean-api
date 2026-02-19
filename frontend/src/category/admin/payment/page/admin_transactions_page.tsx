import { useState } from "react";
import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { Search, ChevronUp, ChevronDown, ArrowLeft } from "lucide-react";

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

import { useAdminTransactions } from "../hook/use_admin_payment";
import type { AdminTxnListReq } from "../types";

type SortField = "id" | "occurred_at" | "amount" | "status";
type SortOrder = "asc" | "desc";

const formatCents = (cents: number) => `$${(cents / 100).toFixed(2)}`;

export function AdminTransactionsPage() {
  const { t } = useTranslation();
  const [params, setParams] = useState<AdminTxnListReq>({ page: 1, size: 20 });
  const [searchInput, setSearchInput] = useState("");
  const [statusFilter, setStatusFilter] = useState<string>("");
  const [sortField, setSortField] = useState<SortField>("occurred_at");
  const [sortOrder, setSortOrder] = useState<SortOrder>("desc");

  const { data, isLoading, isError } = useAdminTransactions({
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
        <div className="flex items-center gap-4">
          <Button variant="ghost" size="sm" asChild>
            <Link to="/admin/payment/subscriptions">
              <ArrowLeft className="mr-1 h-4 w-4" />
              {t("admin.payment.subscriptions")}
            </Link>
          </Button>
          <h1 className="text-2xl font-bold">{t("admin.payment.transactions")}</h1>
        </div>
      </div>

      {/* Search & Filter */}
      <div className="flex items-center gap-4">
        <form onSubmit={handleSearch} className="flex gap-2 max-w-md">
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              placeholder={t("admin.payment.searchByEmail")}
              value={searchInput}
              onChange={(e) => setSearchInput(e.target.value)}
              className="pl-9"
            />
          </div>
          <Button type="submit" variant="secondary">{t("admin.payment.search")}</Button>
        </form>

        <Select value={statusFilter} onValueChange={(v) => {
          setStatusFilter(v === "all" ? "" : v);
          setParams((prev) => ({ ...prev, page: 1 }));
        }}>
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder={t("admin.payment.allStatus")} />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{t("admin.payment.allStatus")}</SelectItem>
            <SelectItem value="completed">{t("admin.payment.completed")}</SelectItem>
            <SelectItem value="refunded">{t("admin.payment.refunded")}</SelectItem>
            <SelectItem value="partially_refunded">{t("admin.payment.partiallyRefunded")}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      {/* Table */}
      <div className="rounded-md border">
        <table className="w-full text-sm">
          <thead className="border-b bg-muted/50">
            <tr>
              <th className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                  onClick={() => handleSort("id")}>
                {t("admin.payment.colId")}<SortIcon field="id" />
              </th>
              <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colEmail")}</th>
              <th className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                  onClick={() => handleSort("status")}>
                {t("admin.payment.colStatus")}<SortIcon field="status" />
              </th>
              <th className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                  onClick={() => handleSort("amount")}>
                {t("admin.payment.colAmount")}<SortIcon field="amount" />
              </th>
              <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colTax")}</th>
              <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colCurrency")}</th>
              <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colInterval")}</th>
              <th className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                  onClick={() => handleSort("occurred_at")}>
                {t("admin.payment.colDate")}<SortIcon field="occurred_at" />
              </th>
              <th className="h-10 px-4 text-left font-medium">{t("admin.payment.colSub")}</th>
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <tr key={i} className="border-b">
                  {Array.from({ length: 9 }).map((__, j) => (
                    <td key={j} className="p-4"><Skeleton className="h-4 w-16" /></td>
                  ))}
                </tr>
              ))
            ) : isError ? (
              <tr>
                <td colSpan={9} className="p-4 text-center text-destructive">
                  {t("admin.payment.failedLoad")}
                </td>
              </tr>
            ) : data?.items.length === 0 ? (
              <tr>
                <td colSpan={9} className="p-4 text-center text-muted-foreground">
                  {t("admin.payment.noTransactions")}
                </td>
              </tr>
            ) : (
              data?.items.map((txn) => (
                <tr key={txn.transaction_id} className="border-b hover:bg-muted/50">
                  <td className="p-4">{txn.transaction_id}</td>
                  <td className="p-4">{txn.user_email}</td>
                  <td className="p-4">
                    <Badge variant="outline">{txn.status}</Badge>
                  </td>
                  <td className="p-4">{formatCents(txn.amount_cents)}</td>
                  <td className="p-4">{formatCents(txn.tax_cents)}</td>
                  <td className="p-4">{txn.currency}</td>
                  <td className="p-4">{txn.billing_interval || "-"}</td>
                  <td className="p-4">{new Date(txn.occurred_at).toLocaleDateString()}</td>
                  <td className="p-4">
                    {txn.subscription_id ? (
                      <Link
                        to={`/admin/payment/subscriptions/${txn.subscription_id}`}
                        className="text-primary hover:underline"
                      >
                        #{txn.subscription_id}
                      </Link>
                    ) : (
                      "-"
                    )}
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
