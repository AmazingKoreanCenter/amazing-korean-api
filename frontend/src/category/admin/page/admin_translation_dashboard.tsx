import { useMemo } from "react";
import { Link } from "react-router-dom";
import { BarChart3, ArrowLeft } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";

import { SUPPORTED_LANGUAGES, TIER_BREAK_INDICES } from "@/i18n";
import { useTranslationStats } from "../hook/use_translations";
import { CONTENT_TYPE_LABELS } from "../translation/types";
import type { ContentType, TranslationStatus } from "../translation/types";

// content_type 정렬 순서
const CONTENT_TYPE_ORDER: ContentType[] = [
  "video",
  "video_tag",
  "lesson",
  "study",
  "study_task_choice",
  "study_task_typing",
  "study_task_voice",
  "study_task_explain",
];

// ko 제외 (번역 대상만)
const TARGET_LANGS = SUPPORTED_LANGUAGES.filter((l) => l.code !== "ko");

function getStatusColor(
  draft: number,
  reviewed: number,
  approved: number,
): string {
  const total = draft + reviewed + approved;
  if (total === 0) return "bg-gray-100 text-gray-400";
  if (approved === total) return "bg-green-100 text-green-700";
  if (approved + reviewed === total) return "bg-blue-100 text-blue-700";
  if (approved > 0) return "bg-yellow-100 text-yellow-700";
  return "bg-orange-100 text-orange-700";
}

export function AdminTranslationDashboard() {
  const { data, isLoading, isError } = useTranslationStats();

  // content_type × lang → {draft, reviewed, approved} 매트릭스 빌드
  const matrix = useMemo(() => {
    if (!data) return null;

    const map = new Map<string, Record<TranslationStatus, number>>();

    for (const item of data.items) {
      const key = `${item.content_type}::${item.lang}`;
      if (!map.has(key)) {
        map.set(key, { draft: 0, reviewed: 0, approved: 0 });
      }
      const entry = map.get(key)!;
      entry[item.status] = item.count;
    }

    // 사용 중인 content_type만 필터링 (데이터가 있는 것만)
    const usedTypes = new Set(data.items.map((i) => i.content_type));
    const contentTypes = CONTENT_TYPE_ORDER.filter((ct) => usedTypes.has(ct));

    return { map, contentTypes };
  }, [data]);

  return (
    <div>
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <BarChart3 className="w-6 h-6 text-gray-700" />
          <h2 className="text-2xl font-bold text-gray-900">
            Translation Dashboard
          </h2>
          {data && (
            <span className="text-sm text-gray-500">
              ({data.total_translations} total)
            </span>
          )}
        </div>
        <Button variant="outline" asChild>
          <Link to="/admin/translations">
            <ArrowLeft className="w-4 h-4 mr-2" />
            Translation List
          </Link>
        </Button>
      </div>

      {/* Legend */}
      <div className="flex gap-4 mb-4 text-xs">
        <span className="flex items-center gap-1.5">
          <span className="w-3 h-3 rounded bg-green-100 border border-green-300" />
          All Approved
        </span>
        <span className="flex items-center gap-1.5">
          <span className="w-3 h-3 rounded bg-blue-100 border border-blue-300" />
          Reviewed
        </span>
        <span className="flex items-center gap-1.5">
          <span className="w-3 h-3 rounded bg-yellow-100 border border-yellow-300" />
          Partially Approved
        </span>
        <span className="flex items-center gap-1.5">
          <span className="w-3 h-3 rounded bg-orange-100 border border-orange-300" />
          Draft Only
        </span>
        <span className="flex items-center gap-1.5">
          <span className="w-3 h-3 rounded bg-gray-100 border border-gray-300" />
          No Translations
        </span>
      </div>

      {/* Matrix Table */}
      {isLoading ? (
        <div className="space-y-2">
          {Array.from({ length: 6 }).map((_, i) => (
            <Skeleton key={i} className="h-10 w-full" />
          ))}
        </div>
      ) : isError ? (
        <div className="text-center text-red-500 py-8">
          Failed to load translation statistics.
        </div>
      ) : matrix && matrix.contentTypes.length > 0 ? (
        <div className="bg-white rounded-lg border overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full text-xs">
              <thead>
                <tr className="border-b bg-gray-50">
                  <th className="text-left px-3 py-2 font-medium text-gray-600 sticky left-0 bg-gray-50 z-10 min-w-[140px]">
                    Content Type
                  </th>
                  {TARGET_LANGS.map((lang, i) => {
                    // Tier 구분선 (왼쪽 border)
                    const tierBorder = TIER_BREAK_INDICES.some(
                      (bi) => i + 1 === bi,
                    )
                      ? "border-l-2 border-l-gray-300"
                      : "";
                    return (
                      <th
                        key={lang.code}
                        className={`text-center px-1 py-2 font-medium text-gray-600 min-w-[52px] ${tierBorder}`}
                        title={`${lang.nativeName} (${lang.name})`}
                      >
                        <div>{lang.flag}</div>
                        <div className="text-[10px] text-gray-400">
                          {lang.code}
                        </div>
                      </th>
                    );
                  })}
                </tr>
              </thead>
              <tbody>
                {matrix.contentTypes.map((ct) => (
                  <tr key={ct} className="border-b hover:bg-gray-50/50">
                    <td className="px-3 py-2 font-medium text-gray-700 sticky left-0 bg-white z-10">
                      {CONTENT_TYPE_LABELS[ct] ?? ct}
                    </td>
                    {TARGET_LANGS.map((lang, i) => {
                      const key = `${ct}::${lang.code}`;
                      const entry = matrix.map.get(key) ?? {
                        draft: 0,
                        reviewed: 0,
                        approved: 0,
                      };
                      const total =
                        entry.draft + entry.reviewed + entry.approved;
                      const colorClass = getStatusColor(
                        entry.draft,
                        entry.reviewed,
                        entry.approved,
                      );
                      const tierBorder = TIER_BREAK_INDICES.some(
                        (bi) => i + 1 === bi,
                      )
                        ? "border-l-2 border-l-gray-300"
                        : "";

                      return (
                        <td
                          key={lang.code}
                          className={`text-center px-1 py-2 ${tierBorder}`}
                        >
                          <Link
                            to={`/admin/translations?content_type=${ct}&lang=${lang.code}`}
                            className={`inline-block rounded px-1.5 py-0.5 ${colorClass} hover:opacity-80 transition-opacity min-w-[32px]`}
                            title={`${CONTENT_TYPE_LABELS[ct]} / ${lang.nativeName}\nApproved: ${entry.approved}, Reviewed: ${entry.reviewed}, Draft: ${entry.draft}`}
                          >
                            {total > 0 ? total : "-"}
                          </Link>
                        </td>
                      );
                    })}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      ) : (
        <div className="text-center text-gray-500 py-8">
          No translation data available.
        </div>
      )}
    </div>
  );
}
