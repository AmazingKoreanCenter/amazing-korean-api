import { useState } from "react";
import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { ArrowLeft, History } from "lucide-react";

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

import { useAdminTextbookLogs } from "../hook/use_admin_textbook";
import type {
  AdminAction,
  AdminTextbookLogItem,
  AdminTextbookLogQuery,
} from "../types";

const ACTIONS: AdminAction[] = [
  "create",
  "update",
  "banned",
  "reorder",
  "publish",
  "unpublish",
  "delete",
];

const actionBadgeVariant = (action: AdminAction) => {
  switch (action) {
    case "create":
      return "success" as const;
    case "update":
      return "blue" as const;
    case "delete":
    case "banned":
      return "destructive" as const;
    default:
      return "outline" as const;
  }
};

/**
 * Q6 (2026-04-22) — admin_textbook_log 감사 로그 조회 페이지.
 *
 * 관리자가 어느 주문에 대해 언제 어떤 액션(create/update/banned/…)을 했는지
 * 추적. 필터: action, order_id, admin_user_id.
 */
export function AdminTextbookLogsPage() {
  const { t } = useTranslation();

  const [params, setParams] = useState<AdminTextbookLogQuery>({
    page: 1,
    per_page: 20,
  });
  const [actionFilter, setActionFilter] = useState<string>("all");
  const [orderIdInput, setOrderIdInput] = useState("");
  const [adminUserIdInput, setAdminUserIdInput] = useState("");

  const { data, isLoading, isError } = useAdminTextbookLogs({
    ...params,
    action:
      actionFilter === "all" || !actionFilter
        ? undefined
        : (actionFilter as AdminAction),
  });

  const handleApplyFilter = (e: React.FormEvent) => {
    e.preventDefault();
    const orderId = orderIdInput.trim();
    const adminUserId = adminUserIdInput.trim();
    setParams((prev) => ({
      ...prev,
      page: 1,
      order_id: /^\d+$/.test(orderId) ? Number(orderId) : undefined,
      admin_user_id: /^\d+$/.test(adminUserId) ? Number(adminUserId) : undefined,
    }));
  };

  const handleResetFilter = () => {
    setActionFilter("all");
    setOrderIdInput("");
    setAdminUserIdInput("");
    setParams({ page: 1, per_page: 20 });
  };

  const handlePageChange = (page: number) => {
    setParams((prev) => ({ ...prev, page }));
  };

  const items = data?.items ?? [];
  const meta = data?.meta;

  return (
    <div className="space-y-4">
      {/* 헤더 */}
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" asChild>
          <Link to="/admin/textbook/orders">
            <ArrowLeft className="h-5 w-5" />
          </Link>
        </Button>
        <div>
          <h1 className="text-2xl font-bold flex items-center gap-2">
            <History className="h-6 w-6" />
            {t("admin.textbook.logs.title")}
          </h1>
          <p className="text-sm text-muted-foreground">
            {t("admin.textbook.logs.subtitle")}
          </p>
        </div>
      </div>

      {/* Filter */}
      <div className="bg-card rounded-lg border border-foreground/15 p-4 shadow-sm">
        <form
          onSubmit={handleApplyFilter}
          className="grid grid-cols-1 md:grid-cols-[1fr_1fr_1fr_auto_auto] gap-3 items-end"
        >
          <div>
            <label className="text-xs text-muted-foreground">
              {t("admin.textbook.logs.filter.action")}
            </label>
            <Select value={actionFilter} onValueChange={setActionFilter}>
              <SelectTrigger className="mt-1">
                <SelectValue
                  placeholder={t("admin.textbook.logs.filter.actionAll")}
                />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">
                  {t("admin.textbook.logs.filter.actionAll")}
                </SelectItem>
                {ACTIONS.map((a) => (
                  <SelectItem key={a} value={a}>
                    {t(`admin.textbook.logs.actions.${a}`)}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div>
            <label className="text-xs text-muted-foreground">
              {t("admin.textbook.logs.filter.orderId")}
            </label>
            <Input
              type="number"
              className="mt-1"
              value={orderIdInput}
              onChange={(e) => setOrderIdInput(e.target.value)}
              placeholder={t("admin.textbook.logs.filter.orderIdPlaceholder")}
            />
          </div>
          <div>
            <label className="text-xs text-muted-foreground">
              {t("admin.textbook.logs.filter.adminUserId")}
            </label>
            <Input
              type="number"
              className="mt-1"
              value={adminUserIdInput}
              onChange={(e) => setAdminUserIdInput(e.target.value)}
              placeholder={t(
                "admin.textbook.logs.filter.adminUserIdPlaceholder",
              )}
            />
          </div>
          <Button type="submit" size="sm">
            {t("admin.textbook.logs.filter.apply")}
          </Button>
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={handleResetFilter}
          >
            {t("admin.textbook.logs.filter.reset")}
          </Button>
        </form>
      </div>

      {/* Table */}
      <div className="bg-card rounded-lg border border-foreground/15 shadow-sm overflow-hidden">
        {isLoading ? (
          <div className="p-4 space-y-2">
            {Array.from({ length: 5 }).map((_, i) => (
              <Skeleton key={i} className="h-12 w-full" />
            ))}
          </div>
        ) : isError ? (
          <div className="p-8 text-center text-destructive">
            {t("admin.textbook.logs.loadError")}
          </div>
        ) : items.length === 0 ? (
          <div className="p-8 text-center text-muted-foreground">
            {t("admin.textbook.logs.empty")}
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead className="bg-muted/50 text-xs text-muted-foreground">
                <tr>
                  <th className="px-4 py-2 text-left">
                    {t("admin.textbook.logs.table.createdAt")}
                  </th>
                  <th className="px-4 py-2 text-left">
                    {t("admin.textbook.logs.table.action")}
                  </th>
                  <th className="px-4 py-2 text-left">
                    {t("admin.textbook.logs.table.admin")}
                  </th>
                  <th className="px-4 py-2 text-left">
                    {t("admin.textbook.logs.table.order")}
                  </th>
                  <th className="px-4 py-2 text-left">
                    {t("admin.textbook.logs.table.diff")}
                  </th>
                </tr>
              </thead>
              <tbody>
                {items.map((log: AdminTextbookLogItem) => (
                  <tr key={log.log_id} className="border-t hover:bg-muted/30">
                    <td className="px-4 py-2 whitespace-nowrap text-xs text-muted-foreground">
                      {new Date(log.created_at).toLocaleString()}
                    </td>
                    <td className="px-4 py-2">
                      <Badge variant={actionBadgeVariant(log.action)}>
                        {t(`admin.textbook.logs.actions.${log.action}`)}
                      </Badge>
                    </td>
                    <td className="px-4 py-2">
                      <div className="font-medium">{log.admin_nickname}</div>
                      <div className="text-xs text-muted-foreground">
                        {log.admin_email} (ID: {log.admin_user_id})
                      </div>
                    </td>
                    <td className="px-4 py-2">
                      <Link
                        to={`/admin/textbook/orders/${log.order_id}`}
                        className="text-primary hover:underline"
                      >
                        {log.order_code}
                      </Link>
                      <div className="text-xs text-muted-foreground">
                        ID: {log.order_id}
                      </div>
                    </td>
                    <td className="px-4 py-2 text-xs">
                      <LogDiff log={log} />
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Pagination */}
      {meta && meta.total_pages > 1 && (
        <Pagination>
          <PaginationContent>
            <PaginationItem>
              <PaginationPrevious
                onClick={() =>
                  handlePageChange(Math.max(1, meta.current_page - 1))
                }
              />
            </PaginationItem>
            {Array.from({ length: Math.min(meta.total_pages, 7) }).map(
              (_, i) => {
                const p = i + 1;
                return (
                  <PaginationItem key={p}>
                    <PaginationLink
                      isActive={p === meta.current_page}
                      onClick={() => handlePageChange(p)}
                    >
                      {p}
                    </PaginationLink>
                  </PaginationItem>
                );
              },
            )}
            <PaginationItem>
              <PaginationNext
                onClick={() =>
                  handlePageChange(
                    Math.min(meta.total_pages, meta.current_page + 1),
                  )
                }
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      )}
    </div>
  );
}

/**
 * before/after JSONB 원본을 요약 렌더. 구조가 다양하므로 단순 요약만.
 * 세부 diff 는 추후 JSON diff 라이브러리 도입 시 확장.
 */
function LogDiff({ log }: { log: AdminTextbookLogItem }) {
  const { t } = useTranslation();
  const before = log.before_data;
  const after = log.after_data;

  if (!before && !after) {
    return <span className="text-muted-foreground">—</span>;
  }

  if (!before && after) {
    return (
      <span className="text-success-foreground">
        {t("admin.textbook.logs.diff.afterOnly")}
      </span>
    );
  }

  if (before && !after) {
    return (
      <span className="text-destructive">
        {t("admin.textbook.logs.diff.beforeOnly")}
      </span>
    );
  }

  return (
    <details>
      <summary className="cursor-pointer text-primary hover:underline">
        {t("admin.textbook.logs.diff.toggle")}
      </summary>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-2 mt-2 font-mono">
        <div>
          <div className="text-xs text-muted-foreground">
            {t("admin.textbook.logs.diff.before")}
          </div>
          <pre className="bg-muted/50 p-2 rounded overflow-auto max-h-48">
            {JSON.stringify(before, null, 2)}
          </pre>
        </div>
        <div>
          <div className="text-xs text-muted-foreground">
            {t("admin.textbook.logs.diff.after")}
          </div>
          <pre className="bg-muted/50 p-2 rounded overflow-auto max-h-48">
            {JSON.stringify(after, null, 2)}
          </pre>
        </div>
      </div>
    </details>
  );
}
