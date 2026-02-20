import { useState } from "react";
import { Link } from "react-router-dom";
import { Plus, Languages, BarChart3 } from "lucide-react";

import { Button } from "@/components/ui/button";
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

import { SUPPORTED_LANGUAGES, TIER_BREAK_INDICES } from "@/i18n";
import { useTranslationList } from "../hook/use_translations";
import {
  useUpdateTranslationStatus,
  useDeleteTranslation,
} from "../hook/use_translation_mutations";
import type {
  TranslationListReq,
  ContentType,
  TranslationStatus,
  TranslationRes,
  TopCategory,
} from "../translation/types";
import {
  TOP_CATEGORIES,
  STUDY_SUB_TYPES,
  CONTENT_TYPE_LABELS,
  CATEGORY_CONTENT_TYPES,
} from "../translation/types";

const STATUS_OPTIONS: { value: TranslationStatus; label: string }[] = [
  { value: "draft", label: "Draft" },
  { value: "reviewed", label: "Reviewed" },
  { value: "approved", label: "Approved" },
];

// ko를 제외한 언어 목록 (번역 대상)
const LANG_OPTIONS = SUPPORTED_LANGUAGES.filter((l) => l.code !== "ko");

function StatusSelect({
  item,
  onStatusChange,
}: {
  item: TranslationRes;
  onStatusChange: (id: number, status: TranslationStatus) => void;
}) {
  return (
    <Select
      value={item.status}
      onValueChange={(val) => onStatusChange(item.translation_id, val as TranslationStatus)}
    >
      <SelectTrigger className="w-28 h-8 text-xs">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {STATUS_OPTIONS.map((opt) => (
          <SelectItem key={opt.value} value={opt.value}>
            {opt.label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

// ── Main Page ──────────────────────────────────────

export function AdminTranslationsPage() {
  const [topCategory, setTopCategory] = useState<TopCategory | "all">("all");
  const [studySubType, setStudySubType] = useState<string>("all");
  const [params, setParams] = useState<TranslationListReq>({
    page: 1,
    per_page: 20,
  });

  const { data, isLoading, isError } = useTranslationList(params);
  const statusMutation = useUpdateTranslationStatus();
  const deleteMutation = useDeleteTranslation();

  // 카테고리 필터 변경 → content_type / content_types 파라미터에 반영
  const handleCategoryChange = (value: string) => {
    const cat = value as TopCategory | "all";
    setTopCategory(cat);
    setStudySubType("all");

    if (cat === "all") {
      setParams((prev) => ({ ...prev, content_type: undefined, content_types: undefined, page: 1 }));
    } else if (cat === "study") {
      // Study 전체 선택 시 — content_types(복수)로 서버 필터링
      setParams((prev) => ({
        ...prev,
        content_type: undefined,
        content_types: CATEGORY_CONTENT_TYPES["study"].join(","),
        page: 1,
      }));
    } else {
      setParams((prev) => ({ ...prev, content_type: cat as ContentType, content_types: undefined, page: 1 }));
    }
  };

  // Study 하위 타입 필터 변경
  const handleStudySubChange = (value: string) => {
    setStudySubType(value);
    if (value === "all") {
      // Study 전체 — content_types(복수)로 서버 필터링
      setParams((prev) => ({
        ...prev,
        content_type: undefined,
        content_types: CATEGORY_CONTENT_TYPES["study"].join(","),
        page: 1,
      }));
    } else {
      setParams((prev) => ({ ...prev, content_type: value as ContentType, content_types: undefined, page: 1 }));
    }
  };

  const handleFilterChange = (key: keyof TranslationListReq, value: string) => {
    setParams((prev) => ({
      ...prev,
      [key]: value === "all" ? undefined : value,
      page: 1,
    }));
  };

  const handlePageChange = (page: number) => {
    setParams((prev) => ({ ...prev, page }));
  };

  const handleStatusChange = (id: number, status: TranslationStatus) => {
    statusMutation.mutate({ id, data: { status } });
  };

  const handleDelete = (id: number) => {
    if (window.confirm("이 번역을 삭제하시겠습니까?")) {
      deleteMutation.mutate(id);
    }
  };

  const truncate = (text: string, max: number) =>
    text.length > max ? text.slice(0, max) + "..." : text;

  // 서버에서 필터링된 결과를 그대로 사용
  const filteredItems = data?.items;

  return (
    <div>
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <Languages className="w-6 h-6 text-foreground" />
          <h2 className="text-2xl font-bold text-foreground">Translations</h2>
          {data && (
            <span className="text-sm text-muted-foreground">
              ({data.meta.total_count} total)
            </span>
          )}
        </div>
        <div className="flex gap-2">
          <Button variant="outline" asChild>
            <Link to="/admin/translations/dashboard">
              <BarChart3 className="w-4 h-4 mr-2" />
              Dashboard
            </Link>
          </Button>
          <Button asChild>
            <Link to="/admin/translations/new">
              <Plus className="w-4 h-4 mr-2" />
              New Translation
            </Link>
          </Button>
        </div>
      </div>

      {/* Filters */}
      <div className="bg-card rounded-lg border border-foreground/15 p-4 shadow-sm mb-4">
        <div className="flex gap-3 flex-wrap">
        {/* Category Filter (Video / Study / Lesson) */}
        <Select
          value={topCategory}
          onValueChange={handleCategoryChange}
        >
          <SelectTrigger className="w-40">
            <SelectValue placeholder="Category" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Categories</SelectItem>
            {TOP_CATEGORIES.map((cat) => (
              <SelectItem key={cat.value} value={cat.value}>
                {cat.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        {/* Study Sub-type Filter (Study 선택 시만 표시) */}
        {topCategory === "study" && (
          <Select
            value={studySubType}
            onValueChange={handleStudySubChange}
          >
            <SelectTrigger className="w-52">
              <SelectValue placeholder="Study Type" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Study Types</SelectItem>
              {STUDY_SUB_TYPES.map((sub) => (
                <SelectItem key={sub.value} value={sub.value}>
                  {sub.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        )}

        {/* Language Filter */}
        <Select
          value={params.lang ?? "all"}
          onValueChange={(v) => handleFilterChange("lang", v)}
        >
          <SelectTrigger className="w-44">
            <SelectValue placeholder="Language" />
          </SelectTrigger>
          <SelectContent className="max-h-60">
            <SelectItem value="all">All Languages</SelectItem>
            {LANG_OPTIONS.map((lang, idx) => {
              // Tier 구분선 (TIER_BREAK_INDICES는 ko 포함 인덱스이므로 -1 보정)
              const showSeparator = TIER_BREAK_INDICES.some((bi) => idx === bi - 1);
              return (
                <div key={lang.code}>
                  {showSeparator && (
                    <div className="my-1 border-t border-border" />
                  )}
                  <SelectItem value={lang.code}>
                    {lang.flag} {lang.nativeName} ({lang.code})
                  </SelectItem>
                </div>
              );
            })}
          </SelectContent>
        </Select>

        {/* Status Filter */}
        <Select
          value={params.status ?? "all"}
          onValueChange={(v) => handleFilterChange("status", v)}
        >
          <SelectTrigger className="w-36">
            <SelectValue placeholder="Status" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Status</SelectItem>
            {STATUS_OPTIONS.map((opt) => (
              <SelectItem key={opt.value} value={opt.value}>
                {opt.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        </div>
      </div>

      {/* Table */}
      <div className="bg-card rounded-lg border overflow-hidden shadow-sm">
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b-2 bg-secondary">
                <th className="text-left px-4 py-3 font-semibold text-secondary-foreground">ID</th>
                <th className="text-left px-4 py-3 font-semibold text-secondary-foreground">Type</th>
                <th className="text-left px-4 py-3 font-semibold text-secondary-foreground">Content ID</th>
                <th className="text-left px-4 py-3 font-semibold text-secondary-foreground">Field</th>
                <th className="text-left px-4 py-3 font-semibold text-secondary-foreground">Lang</th>
                <th className="text-left px-4 py-3 font-semibold text-secondary-foreground">Text</th>
                <th className="text-left px-4 py-3 font-semibold text-secondary-foreground">Status</th>
                <th className="text-left px-4 py-3 font-semibold text-secondary-foreground">Actions</th>
              </tr>
            </thead>
            <tbody>
              {isLoading ? (
                Array.from({ length: 5 }).map((_, i) => (
                  <tr key={i} className="border-b">
                    {Array.from({ length: 8 }).map((_, j) => (
                      <td key={j} className="px-4 py-3">
                        <Skeleton className="h-4 w-full" />
                      </td>
                    ))}
                  </tr>
                ))
              ) : isError ? (
                <tr>
                  <td colSpan={8} className="px-4 py-8 text-center text-destructive">
                    Failed to load translations.
                  </td>
                </tr>
              ) : filteredItems && filteredItems.length === 0 ? (
                <tr>
                  <td colSpan={8} className="px-4 py-8 text-center text-muted-foreground">
                    No translations found.
                  </td>
                </tr>
              ) : (
                filteredItems?.map((item) => (
                  <tr key={item.translation_id} className="border-b hover:bg-accent/10">
                    <td className="px-4 py-3 text-foreground font-mono text-xs">
                      {item.translation_id}
                    </td>
                    <td className="px-4 py-3">
                      <Badge variant="outline">
                        {CONTENT_TYPE_LABELS[item.content_type] ?? item.content_type}
                      </Badge>
                    </td>
                    <td className="px-4 py-3 font-mono text-xs">{item.content_id}</td>
                    <td className="px-4 py-3 font-mono text-xs">{item.field_name}</td>
                    <td className="px-4 py-3">
                      {(() => {
                        const langInfo = SUPPORTED_LANGUAGES.find((l) => l.code === item.lang);
                        return (
                          <span className="inline-flex items-center gap-1.5 text-sm">
                            <span className="emoji-flag">{langInfo?.flag ?? "🏳️"}</span>
                            <span className="font-medium text-foreground">{langInfo?.nativeName ?? item.lang}</span>
                          </span>
                        );
                      })()}
                    </td>
                    <td className="px-4 py-3 max-w-[200px]">
                      <span className="text-foreground" title={item.translated_text}>
                        {truncate(item.translated_text, 40)}
                      </span>
                    </td>
                    <td className="px-4 py-3">
                      <StatusSelect item={item} onStatusChange={handleStatusChange} />
                    </td>
                    <td className="px-4 py-3">
                      <div className="flex gap-2">
                        <Button variant="ghost" size="sm" asChild>
                          <Link to={`/admin/translations/${item.translation_id}/edit`}>
                            Edit
                          </Link>
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          className="text-destructive hover:text-destructive"
                          onClick={() => handleDelete(item.translation_id)}
                        >
                          Delete
                        </Button>
                      </div>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>

        {/* Pagination */}
        {data && data.meta.total_pages > 1 && (
          <div className="border-t px-4 py-3">
            <Pagination>
              <PaginationContent>
                <PaginationItem>
                  <PaginationPrevious
                    onClick={() => handlePageChange(Math.max(1, data.meta.current_page - 1))}
                    className={data.meta.current_page <= 1 ? "pointer-events-none opacity-50" : "cursor-pointer"}
                  />
                </PaginationItem>
                {Array.from({ length: Math.min(5, data.meta.total_pages) }, (_, i) => {
                  const startPage = Math.max(1, Math.min(data.meta.current_page - 2, data.meta.total_pages - 4));
                  const page = startPage + i;
                  if (page > data.meta.total_pages) return null;
                  return (
                    <PaginationItem key={page}>
                      <PaginationLink
                        onClick={() => handlePageChange(page)}
                        isActive={page === data.meta.current_page}
                        className="cursor-pointer"
                      >
                        {page}
                      </PaginationLink>
                    </PaginationItem>
                  );
                })}
                <PaginationItem>
                  <PaginationNext
                    onClick={() => handlePageChange(Math.min(data.meta.total_pages, data.meta.current_page + 1))}
                    className={data.meta.current_page >= data.meta.total_pages ? "pointer-events-none opacity-50" : "cursor-pointer"}
                  />
                </PaginationItem>
              </PaginationContent>
            </Pagination>
          </div>
        )}
      </div>
    </div>
  );
}
