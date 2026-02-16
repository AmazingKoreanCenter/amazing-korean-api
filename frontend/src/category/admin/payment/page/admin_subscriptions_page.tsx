import { useState } from "react";
import { Link } from "react-router-dom";
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
      return "default";
    case "trialing":
      return "secondary";
    case "past_due":
      return "destructive";
    case "paused":
      return "outline";
    case "canceled":
      return "destructive";
    default:
      return "outline";
  }
};

const formatCents = (cents: number) =>
  `$${(cents / 100).toFixed(2)}`;

export function AdminSubscriptionsPage() {
  const [params, setParams] = useState<AdminSubListReq>({ page: 1, size: 20 });
  const [searchInput, setSearchInput] = useState("");
  const [statusFilter, setStatusFilter] = useState<string>("");
  const [sortField, setSortField] = useState<SortField>("created_at");
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
        <h1 className="text-2xl font-bold">Subscriptions</h1>
        <div className="flex gap-2">
          <Button variant="outline" asChild>
            <Link to="/admin/payment/transactions">
              <Receipt className="mr-2 h-4 w-4" />
              Transactions
            </Link>
          </Button>
          <Button variant="outline" asChild>
            <Link to="/admin/payment/grants">
              <Gift className="mr-2 h-4 w-4" />
              Manual Grants
            </Link>
          </Button>
        </div>
      </div>

      {/* Search & Filter */}
      <div className="flex items-center gap-4">
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
          <Button type="submit" variant="secondary">Search</Button>
        </form>

        <Select value={statusFilter} onValueChange={(v) => {
          setStatusFilter(v === "all" ? "" : v);
          setParams((prev) => ({ ...prev, page: 1 }));
        }}>
          <SelectTrigger className="w-[160px]">
            <SelectValue placeholder="All Status" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Status</SelectItem>
            <SelectItem value="trialing">Trialing</SelectItem>
            <SelectItem value="active">Active</SelectItem>
            <SelectItem value="past_due">Past Due</SelectItem>
            <SelectItem value="paused">Paused</SelectItem>
            <SelectItem value="canceled">Canceled</SelectItem>
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
                ID<SortIcon field="id" />
              </th>
              <th className="h-10 px-4 text-left font-medium">Email</th>
              <th className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                  onClick={() => handleSort("status")}>
                Status<SortIcon field="status" />
              </th>
              <th className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                  onClick={() => handleSort("billing_interval")}>
                Interval<SortIcon field="billing_interval" />
              </th>
              <th className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                  onClick={() => handleSort("price")}>
                Price<SortIcon field="price" />
              </th>
              <th className="h-10 px-4 text-left font-medium">Period End</th>
              <th className="h-10 px-4 text-left font-medium cursor-pointer hover:bg-muted"
                  onClick={() => handleSort("created_at")}>
                Created<SortIcon field="created_at" />
              </th>
              <th className="h-10 px-4 text-left font-medium">Actions</th>
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <tr key={i} className="border-b">
                  {Array.from({ length: 8 }).map((__, j) => (
                    <td key={j} className="p-4"><Skeleton className="h-4 w-20" /></td>
                  ))}
                </tr>
              ))
            ) : isError ? (
              <tr>
                <td colSpan={8} className="p-4 text-center text-destructive">
                  Failed to load subscriptions
                </td>
              </tr>
            ) : data?.items.length === 0 ? (
              <tr>
                <td colSpan={8} className="p-4 text-center text-muted-foreground">
                  No subscriptions found
                </td>
              </tr>
            ) : (
              data?.items.map((sub) => (
                <tr key={sub.subscription_id} className="border-b hover:bg-muted/50">
                  <td className="p-4">{sub.subscription_id}</td>
                  <td className="p-4">{sub.user_email}</td>
                  <td className="p-4">
                    <Badge variant={statusBadgeVariant(sub.status)}>{sub.status}</Badge>
                  </td>
                  <td className="p-4">{sub.billing_interval}</td>
                  <td className="p-4">{formatCents(sub.current_price_cents)}</td>
                  <td className="p-4">
                    {sub.current_period_end
                      ? new Date(sub.current_period_end).toLocaleDateString()
                      : "-"}
                  </td>
                  <td className="p-4">
                    {new Date(sub.created_at).toLocaleDateString()}
                  </td>
                  <td className="p-4">
                    <Button variant="ghost" size="sm" asChild>
                      <Link to={`/admin/payment/subscriptions/${sub.subscription_id}`}>
                        <CreditCard className="mr-1 h-3 w-3" />
                        Detail
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
          Showing {data.items.length} of {data.meta.total_count} subscriptions
        </p>
      )}
    </div>
  );
}
