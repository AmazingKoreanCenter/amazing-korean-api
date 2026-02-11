import { useState } from "react";
import { Link } from "react-router-dom";
import { Plus, Languages, Wand2, Loader2 } from "lucide-react";

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
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";

import { SUPPORTED_LANGUAGES } from "@/i18n";
import { useTranslationList } from "../hook/use_translations";
import {
  useUpdateTranslationStatus,
  useDeleteTranslation,
  useAutoTranslate,
} from "../hook/use_translation_mutations";
import type {
  TranslationListReq,
  ContentType,
  TranslationStatus,
  TranslationRes,
  SupportedLanguage,
  AutoTranslateRes,
} from "../translation/types";

const CONTENT_TYPE_OPTIONS: { value: ContentType; label: string }[] = [
  { value: "course", label: "Course" },
  { value: "lesson", label: "Lesson" },
  { value: "video", label: "Video" },
  { value: "video_tag", label: "Video Tag" },
  { value: "study", label: "Study" },
];

const STATUS_OPTIONS: { value: TranslationStatus; label: string; color: string }[] = [
  { value: "draft", label: "Draft", color: "bg-yellow-100 text-yellow-800" },
  { value: "reviewed", label: "Reviewed", color: "bg-blue-100 text-blue-800" },
  { value: "approved", label: "Approved", color: "bg-green-100 text-green-800" },
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

// ── Auto Translate Dialog ──────────────────────────

function AutoTranslateDialog() {
  const [open, setOpen] = useState(false);
  const [contentType, setContentType] = useState<ContentType>("video");
  const [contentId, setContentId] = useState("");
  const [fieldName, setFieldName] = useState("");
  const [sourceText, setSourceText] = useState("");
  const [selectedLangs, setSelectedLangs] = useState<Set<string>>(new Set());
  const [result, setResult] = useState<AutoTranslateRes | null>(null);

  const autoTranslate = useAutoTranslate();

  const toggleLang = (code: string) => {
    setSelectedLangs((prev) => {
      const next = new Set(prev);
      if (next.has(code)) {
        next.delete(code);
      } else {
        next.add(code);
      }
      return next;
    });
  };

  const selectAllLangs = () => {
    if (selectedLangs.size === LANG_OPTIONS.length) {
      setSelectedLangs(new Set());
    } else {
      setSelectedLangs(new Set(LANG_OPTIONS.map((l) => l.code)));
    }
  };

  const handleSubmit = () => {
    const id = parseInt(contentId, 10);
    if (!id || !fieldName.trim() || !sourceText.trim() || selectedLangs.size === 0) return;

    setResult(null);
    autoTranslate.mutate(
      {
        content_type: contentType,
        content_id: id,
        field_name: fieldName.trim(),
        source_text: sourceText.trim(),
        target_langs: Array.from(selectedLangs) as SupportedLanguage[],
      },
      {
        onSuccess: (res) => {
          setResult(res);
        },
      },
    );
  };

  const handleClose = (isOpen: boolean) => {
    setOpen(isOpen);
    if (!isOpen) {
      setResult(null);
    }
  };

  const isValid =
    contentId && parseInt(contentId, 10) > 0 && fieldName.trim() && sourceText.trim() && selectedLangs.size > 0;

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogTrigger asChild>
        <Button variant="outline">
          <Wand2 className="w-4 h-4 mr-2" />
          Auto Translate
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-lg max-h-[85vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Auto Translate</DialogTitle>
        </DialogHeader>

        <div className="space-y-4 mt-2">
          {/* Content Type */}
          <div className="space-y-1.5">
            <Label>Content Type</Label>
            <Select value={contentType} onValueChange={(v) => setContentType(v as ContentType)}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {CONTENT_TYPE_OPTIONS.map((opt) => (
                  <SelectItem key={opt.value} value={opt.value}>
                    {opt.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* Content ID */}
          <div className="space-y-1.5">
            <Label>Content ID</Label>
            <Input
              type="number"
              placeholder="e.g. 1"
              value={contentId}
              onChange={(e) => setContentId(e.target.value)}
            />
          </div>

          {/* Field Name */}
          <div className="space-y-1.5">
            <Label>Field Name</Label>
            <Input
              placeholder="e.g. title, description"
              value={fieldName}
              onChange={(e) => setFieldName(e.target.value)}
            />
          </div>

          {/* Source Text (ko) */}
          <div className="space-y-1.5">
            <Label>Source Text (Korean)</Label>
            <Textarea
              rows={3}
              placeholder="번역할 원본 텍스트를 입력하세요..."
              value={sourceText}
              onChange={(e) => setSourceText(e.target.value)}
            />
          </div>

          {/* Target Languages */}
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label>Target Languages ({selectedLangs.size})</Label>
              <Button type="button" variant="ghost" size="sm" onClick={selectAllLangs}>
                {selectedLangs.size === LANG_OPTIONS.length ? "Deselect All" : "Select All"}
              </Button>
            </div>
            <div className="grid grid-cols-2 gap-2 max-h-48 overflow-y-auto border rounded-md p-3">
              {LANG_OPTIONS.map((lang) => (
                <label key={lang.code} className="flex items-center gap-2 text-sm cursor-pointer">
                  <Checkbox
                    checked={selectedLangs.has(lang.code)}
                    onCheckedChange={() => toggleLang(lang.code)}
                  />
                  <span>
                    {lang.nativeName}{" "}
                    <span className="text-gray-400">({lang.code})</span>
                  </span>
                </label>
              ))}
            </div>
          </div>

          {/* Submit */}
          <Button
            className="w-full"
            disabled={!isValid || autoTranslate.isPending}
            onClick={handleSubmit}
          >
            {autoTranslate.isPending && <Loader2 className="w-4 h-4 mr-2 animate-spin" />}
            {autoTranslate.isPending
              ? "번역 중..."
              : `${selectedLangs.size}개 언어로 번역`}
          </Button>

          {/* Result */}
          {result && (
            <div className="border rounded-md p-3 space-y-2">
              <p className="text-sm font-medium">
                결과: {result.success_count}/{result.total} 성공
              </p>
              <div className="space-y-1 max-h-40 overflow-y-auto">
                {result.results.map((r) => (
                  <div
                    key={r.lang}
                    className={`text-xs px-2 py-1 rounded ${
                      r.success ? "bg-green-50 text-green-700" : "bg-red-50 text-red-700"
                    }`}
                  >
                    <span className="font-mono font-medium">{r.lang}</span>
                    {r.success ? (
                      <span className="ml-2 text-gray-600">
                        {r.translated_text && r.translated_text.length > 50
                          ? r.translated_text.slice(0, 50) + "..."
                          : r.translated_text}
                      </span>
                    ) : (
                      <span className="ml-2">{r.error}</span>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}

// ── Main Page ──────────────────────────────────────

export function AdminTranslationsPage() {
  const [params, setParams] = useState<TranslationListReq>({
    page: 1,
    per_page: 20,
  });

  const { data, isLoading, isError } = useTranslationList(params);
  const statusMutation = useUpdateTranslationStatus();
  const deleteMutation = useDeleteTranslation();

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

  return (
    <div>
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <Languages className="w-6 h-6 text-gray-700" />
          <h2 className="text-2xl font-bold text-gray-900">Translations</h2>
          {data && (
            <span className="text-sm text-gray-500">
              ({data.meta.total_count} total)
            </span>
          )}
        </div>
        <div className="flex gap-2">
          <AutoTranslateDialog />
          <Button asChild>
            <Link to="/admin/translations/new">
              <Plus className="w-4 h-4 mr-2" />
              New Translation
            </Link>
          </Button>
        </div>
      </div>

      {/* Filters */}
      <div className="flex gap-3 mb-4 flex-wrap">
        <Select
          value={params.content_type ?? "all"}
          onValueChange={(v) => handleFilterChange("content_type", v)}
        >
          <SelectTrigger className="w-40">
            <SelectValue placeholder="Content Type" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Types</SelectItem>
            {CONTENT_TYPE_OPTIONS.map((opt) => (
              <SelectItem key={opt.value} value={opt.value}>
                {opt.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Select
          value={params.lang ?? "all"}
          onValueChange={(v) => handleFilterChange("lang", v)}
        >
          <SelectTrigger className="w-44">
            <SelectValue placeholder="Language" />
          </SelectTrigger>
          <SelectContent className="max-h-60">
            <SelectItem value="all">All Languages</SelectItem>
            {LANG_OPTIONS.map((lang) => (
              <SelectItem key={lang.code} value={lang.code}>
                {lang.nativeName} ({lang.code})
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

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

      {/* Table */}
      <div className="bg-white rounded-lg border overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b bg-gray-50">
                <th className="text-left px-4 py-3 font-medium text-gray-600">ID</th>
                <th className="text-left px-4 py-3 font-medium text-gray-600">Type</th>
                <th className="text-left px-4 py-3 font-medium text-gray-600">Content ID</th>
                <th className="text-left px-4 py-3 font-medium text-gray-600">Field</th>
                <th className="text-left px-4 py-3 font-medium text-gray-600">Lang</th>
                <th className="text-left px-4 py-3 font-medium text-gray-600">Text</th>
                <th className="text-left px-4 py-3 font-medium text-gray-600">Status</th>
                <th className="text-left px-4 py-3 font-medium text-gray-600">Actions</th>
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
                  <td colSpan={8} className="px-4 py-8 text-center text-red-500">
                    Failed to load translations.
                  </td>
                </tr>
              ) : data && data.items.length === 0 ? (
                <tr>
                  <td colSpan={8} className="px-4 py-8 text-center text-gray-500">
                    No translations found.
                  </td>
                </tr>
              ) : (
                data?.items.map((item) => (
                  <tr key={item.translation_id} className="border-b hover:bg-gray-50">
                    <td className="px-4 py-3 text-gray-900 font-mono text-xs">
                      {item.translation_id}
                    </td>
                    <td className="px-4 py-3">
                      <Badge variant="outline">{item.content_type}</Badge>
                    </td>
                    <td className="px-4 py-3 font-mono text-xs">{item.content_id}</td>
                    <td className="px-4 py-3 font-mono text-xs">{item.field_name}</td>
                    <td className="px-4 py-3">
                      <Badge variant="secondary">{item.lang}</Badge>
                    </td>
                    <td className="px-4 py-3 max-w-[200px]">
                      <span className="text-gray-700" title={item.translated_text}>
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
                          className="text-red-600 hover:text-red-700"
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
