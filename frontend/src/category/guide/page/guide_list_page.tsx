import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";

import { Skeleton } from "@/components/ui/skeleton";

import { useGuides } from "../hook/use_guide";
import { guideThemeColors } from "../types";

/** 온라인 콘텐츠(해설집) 단원 목록 — 교재 10색 테마 카드 */
export function GuideListPage() {
  const { t } = useTranslation();
  const { data, isLoading, isError } = useGuides();

  return (
    <div className="mx-auto max-w-3xl px-4 py-8">
      <header className="mb-6">
        <h1 className="text-2xl font-bold text-foreground">{t("guide.listTitle")}</h1>
        <p className="mt-1 text-sm text-muted-foreground">{t("guide.listSubtitle")}</p>
      </header>

      {isLoading && (
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
          {Array.from({ length: 6 }).map((_, i) => (
            <Skeleton key={i} className="h-24 rounded-lg" />
          ))}
        </div>
      )}

      {isError && (
        <p className="text-sm text-muted-foreground">{t("guide.loadError")}</p>
      )}

      {data && data.items.length === 0 && (
        <p className="text-sm text-muted-foreground">{t("guide.empty")}</p>
      )}

      {data && data.items.length > 0 && (
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
          {data.items.map((g) => {
            const theme = guideThemeColors(g.guide_theme);
            return (
              <Link
                key={g.guide_idx}
                to={`/guides/${g.guide_idx}`}
                className="group rounded-lg border p-4 transition-shadow hover:shadow-md"
                style={{ backgroundColor: theme.bg, borderColor: `${theme.color}33` }}
              >
                <div className="flex items-center gap-2">
                  <span
                    className="inline-flex h-6 min-w-6 items-center justify-center rounded-full px-1.5 text-xs font-bold text-white"
                    style={{ backgroundColor: theme.color }}
                  >
                    {g.guide_seq}
                  </span>
                  {g.sentence_start != null && (
                    <span className="text-xs text-muted-foreground">
                      {g.sentence_start}–{g.sentence_end}
                    </span>
                  )}
                </div>
                <h2 className="mt-2 font-semibold text-foreground group-hover:underline">
                  {g.title ?? g.title_ko}
                </h2>
                {(g.subtitle ?? g.subtitle_ko) && (
                  <p className="mt-0.5 text-sm text-muted-foreground">
                    {g.subtitle ?? g.subtitle_ko}
                  </p>
                )}
              </Link>
            );
          })}
        </div>
      )}
    </div>
  );
}
