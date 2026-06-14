import { useCallback, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Link, useParams } from "react-router-dom";
import { ChevronLeft } from "lucide-react";

import { Skeleton } from "@/components/ui/skeleton";
import { useAuthStore } from "@/hooks/use_auth_store";

import { GuideIntro } from "../component/GuideBlockStream";
import { GuideReview } from "../component/GuideReview";
import { GuideSentenceCard } from "../component/GuideSentenceCard";
import { useGuide, useGuideLog, useGuideProgress } from "../hook/use_guide";
import { guideThemeColors, type GuideLogAction } from "../types";

/** 단원 학습 페이지 — 도입부 → 문장별 사이클 → 복습 (해설집 HTML 동등) */
export function GuideLearnPage() {
  const { t } = useTranslation();
  const { guideIdx } = useParams<{ guideIdx: string }>();
  const { data, isLoading, isError } = useGuide(guideIdx);
  const isLoggedIn = useAuthStore((s) => s.isLoggedIn);
  const { data: progress } = useGuideProgress(guideIdx);
  const logMutation = useGuideLog(guideIdx);

  // 서버 진행 = 해결한 문장 집합
  const solvedSet = useMemo(
    () =>
      new Set(
        (progress?.items ?? [])
          .filter((p) => p.is_solved)
          .map((p) => p.sentence_no)
      ),
    [progress]
  );

  // 학습 로그 (로그인 시에만 기록 — 비로그인은 조용히 무시)
  const handleLog = useCallback(
    (sentenceNo: number, action: GuideLogAction, answer?: unknown) => {
      if (!isLoggedIn) return;
      logMutation.mutate({
        sentenceNo,
        body: { activity: "sentence_write", action, answer },
      });
    },
    [isLoggedIn, logMutation]
  );

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

      {/* 진행 (로그인 시) */}
      {isLoggedIn && data.sentences.length > 0 && (
        <div className="mb-6">
          {(() => {
            const total = data.sentences.length;
            const done = data.sentences.filter((s) => solvedSet.has(s.sentence_no)).length;
            return (
              <>
                <div className="mb-1 flex items-center justify-between text-xs text-muted-foreground">
                  <span>{t("guide.progress")}</span>
                  <span>{t("guide.solvedCount", { done, total })}</span>
                </div>
                <div
                  className="h-2 w-full overflow-hidden rounded-full"
                  style={{ backgroundColor: theme.bg }}
                  role="progressbar"
                  aria-valuenow={done}
                  aria-valuemin={0}
                  aria-valuemax={total}
                >
                  <div
                    className="h-full rounded-full transition-all"
                    style={{
                      width: `${total > 0 ? (done / total) * 100 : 0}%`,
                      backgroundColor: theme.color,
                    }}
                  />
                </div>
              </>
            );
          })()}
        </div>
      )}

      {/* 문법 도입부 */}
      <GuideIntro items={data.items} />

      {/* 문장별 학습 사이클 */}
      {data.sentences.length > 0 && (
        <section className="mb-6">
          <h2 className="mb-3 text-sm font-semibold text-foreground">{t("guide.practice")}</h2>
          <div className="space-y-3">
            {data.sentences.map((s) => (
              <GuideSentenceCard
                key={s.sentence_no}
                sentence={s}
                items={data.items}
                solved={solvedSet.has(s.sentence_no)}
                onLog={handleLog}
              />
            ))}
          </div>
        </section>
      )}

      {/* 복습 */}
      <GuideReview sentences={data.sentences} />
    </div>
  );
}
