import { Link } from "react-router-dom";
import { Download, Pencil } from "lucide-react";

import { Skeleton } from "@/components/ui/skeleton";

import { adminGuideDiffExport } from "../admin_guide_api";
import { useAdminGuides, useAdminGuideStale, useUpdateGuideMeta } from "../hook/use_admin_guide";
import type { AdminGuideSummary } from "../types";

const THEME_COLOR: Record<string, string> = {
  blue: "#2184fc", green: "#10b981", orange: "#f59e0b", purple: "#a855f7", pink: "#ec4899",
  teal: "#14b8a6", indigo: "#322acf", rose: "#f43f5e", amber: "#f97316", slate: "#64748b",
};

function StateBadge({ state }: { state: string }) {
  const cls =
    state === "open"
      ? "bg-green-100 text-green-700"
      : state === "close"
        ? "bg-red-100 text-red-700"
        : "bg-gray-100 text-gray-600";
  return <span className={`rounded px-1.5 py-0.5 text-xs font-medium ${cls}`}>{state}</span>;
}

function GuideRow({ g }: { g: AdminGuideSummary }) {
  const meta = useUpdateGuideMeta(g.guide_idx);
  const next = g.guide_state === "open" ? "ready" : "open";
  return (
    <tr className="border-t border-border">
      <td className="px-3 py-2 text-sm text-muted-foreground">{g.guide_seq}</td>
      <td className="px-3 py-2">
        <span
          className="inline-block h-3 w-3 rounded-full align-middle"
          style={{ backgroundColor: THEME_COLOR[g.guide_theme] ?? "#999" }}
          title={g.guide_theme}
        />
      </td>
      <td className="px-3 py-2">
        <div className="text-sm font-medium text-foreground">{g.title_en ?? g.title_ko}</div>
        <div className="text-xs text-muted-foreground">{g.guide_idx}</div>
      </td>
      <td className="px-3 py-2 text-center"><StateBadge state={g.guide_state} /></td>
      <td className="px-3 py-2 text-center text-sm">
        {g.stale_count > 0 ? (
          <span className="rounded bg-amber-100 px-1.5 py-0.5 text-xs font-medium text-amber-700">
            stale {g.stale_count}
          </span>
        ) : (
          <span className="text-xs text-muted-foreground">—</span>
        )}
      </td>
      <td className="px-3 py-2 text-center text-xs text-muted-foreground">{g.block_count}</td>
      <td className="px-3 py-2 text-right">
        <div className="flex items-center justify-end gap-2">
          <button
            type="button"
            disabled={meta.isPending}
            onClick={() => meta.mutate({ guide_state: next })}
            className="rounded border border-border px-2 py-1 text-xs hover:bg-muted disabled:opacity-50"
          >
            {g.guide_state === "open" ? "숨기기" : "공개"}
          </button>
          <Link
            to={`/admin/guides/${g.guide_idx}`}
            className="inline-flex items-center gap-1 rounded border border-border px-2 py-1 text-xs hover:bg-muted"
          >
            <Pencil className="h-3 w-3" /> 편집
          </Link>
        </div>
      </td>
    </tr>
  );
}

export function AdminGuidesPage() {
  const { data, isLoading } = useAdminGuides();
  const { data: stale } = useAdminGuideStale();

  const downloadDiff = async (l: string) => {
    const json = await adminGuideDiffExport(l);
    const blob = new Blob([JSON.stringify(json, null, 1)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `guide_diff_${l}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-xl font-bold text-foreground">온라인 콘텐츠 (guide)</h1>
        <p className="mt-1 text-sm text-muted-foreground">
          단원 공개/숨김, 블록 텍스트 편집, 번역 stale 추적, 재번역 디프 export
        </p>
      </div>

      {/* stale 대시보드 + 디프 export */}
      <section className="rounded-lg border border-border p-4">
        <h2 className="mb-2 text-sm font-semibold text-foreground">번역 stale 현황</h2>
        {stale && stale.rows.length > 0 ? (
          <div className="overflow-x-auto">
            <table className="text-sm">
              <thead>
                <tr className="text-xs text-muted-foreground">
                  <th className="px-3 py-1 text-left">언어</th>
                  <th className="px-3 py-1 text-right">stale</th>
                  <th className="px-3 py-1 text-right">미번역</th>
                  <th className="px-3 py-1"></th>
                </tr>
              </thead>
              <tbody>
                {stale.rows.map((r) => (
                  <tr key={r.lang} className="border-t border-border">
                    <td className="px-3 py-1 font-mono">{r.lang}</td>
                    <td className="px-3 py-1 text-right text-amber-700">{r.stale_count}</td>
                    <td className="px-3 py-1 text-right text-muted-foreground">{r.missing_count}</td>
                    <td className="px-3 py-1 text-right">
                      {(r.stale_count > 0 || r.missing_count > 0) && (
                        <button
                          type="button"
                          onClick={() => downloadDiff(r.lang)}
                          className="inline-flex items-center gap-1 rounded border border-border px-2 py-0.5 text-xs hover:bg-muted"
                        >
                          <Download className="h-3 w-3" /> 디프
                        </button>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <p className="text-xs text-muted-foreground">stale 번역 없음 (전부 최신)</p>
        )}
      </section>

      {/* 단원 목록 */}
      {isLoading ? (
        <Skeleton className="h-64 rounded-lg" />
      ) : (
        <div className="overflow-x-auto rounded-lg border border-border">
          <table className="w-full text-sm">
            <thead>
              <tr className="bg-muted text-xs text-muted-foreground">
                <th className="px-3 py-2 text-left">#</th>
                <th className="px-3 py-2"></th>
                <th className="px-3 py-2 text-left">단원</th>
                <th className="px-3 py-2 text-center">상태</th>
                <th className="px-3 py-2 text-center">번역</th>
                <th className="px-3 py-2 text-center">블록</th>
                <th className="px-3 py-2"></th>
              </tr>
            </thead>
            <tbody>
              {data?.items.map((g) => <GuideRow key={g.guide_idx} g={g} />)}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
