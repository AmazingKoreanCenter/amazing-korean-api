import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { Search, ChevronLeft, ChevronRight } from "lucide-react";

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

import { getAdminEbookPurchases } from "@/category/ebook/ebook_api";
import type { EbookPurchaseStatus } from "@/category/ebook/types";

export function AdminEbookPurchasesPage() {
  const { t } = useTranslation();
  const [page, setPage] = useState(1);
  const [status, setStatus] = useState<string>("all");
  const [search, setSearch] = useState("");
  const [searchInput, setSearchInput] = useState("");

  const { data, isLoading, isError } = useQuery({
    queryKey: ["admin", "ebook", "purchases", page, status, search],
    queryFn: () =>
      getAdminEbookPurchases({
        page,
        per_page: 20,
        status: status === "all" ? undefined : status,
        search: search || undefined,
      }),
  });

  const handleSearch = () => {
    setSearch(searchInput);
    setPage(1);
  };

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">{t("admin.ebook.title")}</h1>

      {/* 필터 */}
      <div className="flex gap-4 flex-wrap">
        <Select
          value={status}
          onValueChange={(v) => {
            setStatus(v);
            setPage(1);
          }}
        >
          <SelectTrigger className="w-[160px]">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{t("admin.ebook.allStatuses")}</SelectItem>
            <SelectItem value="pending">{t("ebook.status.pending")}</SelectItem>
            <SelectItem value="completed">{t("ebook.status.completed")}</SelectItem>
            <SelectItem value="refunded">{t("ebook.status.refunded")}</SelectItem>
          </SelectContent>
        </Select>

        <div className="flex gap-2">
          <Input
            placeholder={t("admin.ebook.searchPlaceholder")}
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSearch()}
            className="w-64"
          />
          <Button variant="outline" size="icon" onClick={handleSearch}>
            <Search className="w-4 h-4" />
          </Button>
        </div>
      </div>

      {/* 테이블 */}
      {isError ? (
        <div className="text-center text-muted-foreground py-8">
          {t("admin.ebook.loadError")}
        </div>
      ) : isLoading ? (
        <div className="space-y-2">
          {Array.from({ length: 5 }).map((_, i) => (
            <Skeleton key={i} className="h-12" />
          ))}
        </div>
      ) : (
        <div className="border rounded-lg overflow-x-auto">
          <table className="w-full text-sm">
            <thead className="bg-muted/50">
              <tr>
                <th className="px-4 py-3 text-left font-medium">
                  {t("admin.ebook.purchaseCode")}
                </th>
                <th className="px-4 py-3 text-left font-medium">{t("admin.ebook.userId")}</th>
                <th className="px-4 py-3 text-left font-medium">
                  {t("admin.ebook.language")}
                </th>
                <th className="px-4 py-3 text-left font-medium">
                  {t("admin.ebook.edition")}
                </th>
                <th className="px-4 py-3 text-left font-medium">
                  {t("admin.ebook.status")}
                </th>
                <th className="px-4 py-3 text-left font-medium">
                  {t("admin.ebook.payment")}
                </th>
                <th className="px-4 py-3 text-left font-medium">
                  {t("admin.ebook.date")}
                </th>
              </tr>
            </thead>
            <tbody>
              {data?.items.map((item) => (
                <tr
                  key={item.purchase_id}
                  className="border-t hover:bg-muted/30"
                >
                  <td className="px-4 py-3">
                    <Link
                      to={`/admin/ebook/purchases/${item.purchase_id}`}
                      className="text-primary underline-offset-4 hover:underline"
                    >
                      {item.purchase_code}
                    </Link>
                  </td>
                  <td className="px-4 py-3">{item.user_id}</td>
                  <td className="px-4 py-3">{item.language}</td>
                  <td className="px-4 py-3 capitalize">{item.edition}</td>
                  <td className="px-4 py-3">
                    <StatusBadge status={item.status} />
                  </td>
                  <td className="px-4 py-3 capitalize">
                    {item.payment_method.replace("_", " ")}
                  </td>
                  <td className="px-4 py-3 text-muted-foreground">
                    {new Date(item.created_at).toLocaleDateString()}
                  </td>
                </tr>
              ))}
              {data?.items.length === 0 && (
                <tr>
                  <td
                    colSpan={7}
                    className="px-4 py-8 text-center text-muted-foreground"
                  >
                    {t("admin.ebook.noResults")}
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      )}

      {/* 페이지네이션 */}
      {data && data.meta.total_pages > 1 && (
        <div className="flex items-center justify-center gap-2">
          <Button
            variant="outline"
            size="icon"
            onClick={() => setPage((p) => Math.max(1, p - 1))}
            disabled={page <= 1}
          >
            <ChevronLeft className="w-4 h-4" />
          </Button>
          <span className="text-sm text-muted-foreground">
            {page} / {data.meta.total_pages}
          </span>
          <Button
            variant="outline"
            size="icon"
            onClick={() => setPage((p) => p + 1)}
            disabled={page >= data.meta.total_pages}
          >
            <ChevronRight className="w-4 h-4" />
          </Button>
        </div>
      )}
    </div>
  );
}

function StatusBadge({ status }: { status: EbookPurchaseStatus }) {
  const { t } = useTranslation();
  const variant =
    status === "completed"
      ? "default"
      : status === "refunded"
        ? "destructive"
        : "secondary";
  return <Badge variant={variant}>{t(`ebook.status.${status}`)}</Badge>;
}
