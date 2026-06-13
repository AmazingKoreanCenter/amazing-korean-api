import { useTranslation } from "react-i18next";
import { Link, useParams } from "react-router-dom";
import { ChevronLeft } from "lucide-react";

import { Skeleton } from "@/components/ui/skeleton";

import { GuideIntro } from "../component/GuideBlockStream";
import { GuideReview } from "../component/GuideReview";
import { GuideSentenceCard } from "../component/GuideSentenceCard";
import { useGuide } from "../hook/use_guide";
import { guideThemeColors } from "../types";

/** 단원 학습 페이지 — 도입부 → 문장별 사이클 → 복습 (해설집 HTML 동등) */
export function GuideLearnPage() {
  const { t } = useTranslation();
  const { guideIdx } = useParams<{ guideIdx: string }>();
  const { data, isLoading, isError } = useGuide(guideIdx);

  if (isLoading) {
    return (
      <div className="mx-auto max-w-3xl px-4 py-8 space-y-4">
        <Skeleton className="h-10 w-2/3" />
        <Skeleton className="h-40 rounded-lg" />
        <Skeleton className="h-40 rounded-lg" />
      </div>
    );
  }

  if (isError || !data) {
    return (
      <div className="mx-auto max-w-3xl px-4 py-8">
        <Link to="/guides" className="inline-flex items-center gap-1 text-sm text-primary hover:underline">
          <ChevronLeft className="h-4 w-4" /> {t("guide.backToList")}
        </Link>
        <p className="mt-4 text-sm text-muted-foreground">{t("guide.loadError")}</p>
      </div>
    );
  }

  const theme = guideThemeColors(data.guide_theme);

  return (
    <div className="mx-auto max-w-3xl px-4 py-8">
      <Link
        to="/guides"
        className="inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground"
      >
        <ChevronLeft className="h-4 w-4" /> {t("guide.backToList")}
      </Link>

      {/* 단원 헤더 (테마색 악센트) */}
      <header
        className="mt-3 mb-6 rounded-lg border-l-4 p-4"
        style={{ borderColor: theme.color, backgroundColor: theme.bg }}
      >
        <div className="flex items-center gap-2">
          <span
            className="inline-flex h-6 min-w-6 items-center justify-center rounded-full px-1.5 text-xs font-bold text-white"
            style={{ backgroundColor: theme.color }}
          >
            {data.guide_seq}
          </span>
          {data.sentence_start != null && (
            <span className="text-xs text-muted-foreground">
              {data.sentence_start}–{data.sentence_end}
            </span>
          )}
        </div>
        <h1 className="mt-2 text-xl font-bold text-foreground">{data.title ?? data.title_ko}</h1>
        {(data.subtitle ?? data.subtitle_ko) && (
          <p className="mt-0.5 text-sm text-muted-foreground">{data.subtitle ?? data.subtitle_ko}</p>
        )}
      </header>

      {/* 문법 도입부 */}
      <GuideIntro items={data.items} />

      {/* 문장별 학습 사이클 */}
      {data.sentences.length > 0 && (
        <section className="mb-6">
          <h2 className="mb-3 text-sm font-semibold text-foreground">{t("guide.practice")}</h2>
          <div className="space-y-3">
            {data.sentences.map((s) => (
              <GuideSentenceCard key={s.sentence_no} sentence={s} items={data.items} />
            ))}
          </div>
        </section>
      )}

      {/* 복습 */}
      <GuideReview sentences={data.sentences} />
    </div>
  );
}
