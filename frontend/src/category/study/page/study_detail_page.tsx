import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link, useParams } from "react-router-dom";
import { ArrowLeft, BookOpen, ClipboardList, Keyboard, Mic } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/sections/empty_state";
import { PaginationBar } from "@/components/sections/pagination_bar";
import { SkeletonGrid } from "@/components/sections/skeleton_grid";
import type { StudyDetailReq, StudyProgram, StudyTaskKind } from "@/category/study/types";

import { useStudyDetail } from "../hook/use_study_detail";

const PER_PAGE = 10;

const KIND_ICONS: Record<StudyTaskKind, typeof ClipboardList> = {
  choice: ClipboardList,
  typing: Keyboard,
  voice: Mic,
};

export function StudyDetailPage() {
  const { t } = useTranslation();
  const { studyId: studyIdParam } = useParams<{ studyId: string }>();

  const PROGRAM_LABELS: Record<StudyProgram, string> = {
    basic_pronunciation: t("study.programBasicPronunciation"),
    basic_word: t("study.programBasicWord"),
    basic_900: t("study.programBasic900"),
    topik_read: t("study.programTopikRead"),
    topik_listen: t("study.programTopikListen"),
    topik_write: t("study.programTopikWrite"),
    tbc: t("study.programTbc"),
  };

  const KIND_LABELS: Record<StudyTaskKind, string> = {
    choice: t("study.kindChoice"),
    typing: t("study.kindTyping"),
    voice: t("study.kindVoice"),
  };
  const studyId = studyIdParam ? Number(studyIdParam) : undefined;

  const [page, setPage] = useState(1);

  const params = useMemo<StudyDetailReq>(() => {
    return {
      page,
      per_page: PER_PAGE,
    };
  }, [page]);

  const { data, isPending, isFetching } = useStudyDetail(studyId, params);

  const tasks = data?.tasks ?? [];
  const meta = data?.meta;

  const currentPage = meta?.page ?? page;
  const totalPages = Math.max(meta?.total_pages ?? 1, 1);

  if (!studyId || !Number.isFinite(studyId)) {
    return (
      <div className="min-h-screen flex items-center justify-center p-4">
        <Card className="w-full max-w-md text-center shadow-card border-0 rounded-2xl">
          <CardHeader className="pb-4">
            <div className="w-16 h-16 rounded-2xl bg-muted flex items-center justify-center mx-auto mb-4">
              <span className="text-3xl">😕</span>
            </div>
            <CardTitle className="text-xl">{t("study.invalidAccess")}</CardTitle>
            <p className="text-sm text-muted-foreground mt-2">
              {t("study.invalidStudyId")}
            </p>
          </CardHeader>
          <CardContent>
            <Link
              to="/studies"
              className="inline-flex items-center justify-center gap-2 gradient-primary text-white rounded-full px-6 py-2.5 text-sm font-medium hover:opacity-90 transition"
            >
              {t("common.backToList")}
            </Link>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="bg-hero-gradient border-b">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8 py-10 lg:py-14">
          {isPending ? (
            <div className="space-y-4">
              <Skeleton className="h-6 w-24 rounded-full" />
              <Skeleton className="h-10 w-2/3" />
              <Skeleton className="h-5 w-1/2" />
            </div>
          ) : data ? (
            <div className="space-y-4">
              <Link
                to="/studies"
                className="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-primary transition-colors"
              >
                <ArrowLeft className="h-4 w-4" />
                {t("common.backToListShort")}
              </Link>

              <div className="flex flex-wrap items-center gap-3">
                <Badge className="gradient-primary text-white border-0 px-4 py-1.5 rounded-full">
                  {PROGRAM_LABELS[data.program]}
                </Badge>
              </div>

              <h1 className="text-3xl md:text-4xl font-bold tracking-tight">
                {data.title ?? t("common.noTitle")}
              </h1>

              {data.subtitle && (
                <p className="text-lg text-muted-foreground">{data.subtitle}</p>
              )}

              <p className="text-sm text-muted-foreground/60">
                Study ID: {data.study_idx}
              </p>
            </div>
          ) : null}
        </div>
      </section>

      {/* Content Section */}
      <section className="py-10 lg:py-14">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8">
          {/* Stats Bar */}
          {meta && (
            <div className="mb-8 flex items-center justify-between">
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <BookOpen className="h-4 w-4" />
                <span>
                  {t("study.totalProblems", { count: meta.total_count ?? 0 })}
                </span>
                <span className="text-border">|</span>
                <span>{currentPage} / {totalPages} {t("common.page")}</span>
              </div>
              {isFetching && (
                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                  <span className="h-2 w-2 animate-pulse rounded-full bg-secondary" />
                  {t("common.loading")}
                </div>
              )}
            </div>
          )}

          {/* Loading State */}
          {isPending ? (
            <SkeletonGrid count={PER_PAGE} variant="study-card" />
          ) : tasks.length === 0 ? (
            <EmptyState
              icon={<BookOpen className="h-10 w-10 text-muted-foreground" />}
              title={t("study.emptyTitle")}
              description={t("study.emptyDetailDescription")}
            />
          ) : (
            <>
              {/* Task Cards Grid */}
              <div className="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3">
                {tasks.map((task) => {
                  const KindIcon = KIND_ICONS[task.kind];
                  return (
                    <Link key={task.task_id} to={`/studies/tasks/${task.task_id}`}>
                      <Card variant="interactive" className="h-full group">
                        <CardHeader className="pb-3">
                          <div className="flex items-center justify-between">
                            <Badge variant="outline" className="gap-1.5 px-3 py-1 rounded-full">
                              <KindIcon className="h-3 w-3" />
                              {KIND_LABELS[task.kind]}
                            </Badge>
                            <span className="text-sm font-medium text-muted-foreground bg-muted px-2.5 py-0.5 rounded-full">
                              #{task.seq}
                            </span>
                          </div>
                        </CardHeader>
                        <CardContent className="pt-0">
                          <CardTitle className="text-lg group-hover:text-primary transition-colors">
                            {t("study.problemNumber", { seq: task.seq })}
                          </CardTitle>
                          <p className="text-xs text-muted-foreground/60 mt-2">
                            Task ID: {task.task_id}
                          </p>
                        </CardContent>
                      </Card>
                    </Link>
                  );
                })}
              </div>

              <PaginationBar
                currentPage={currentPage}
                totalPages={totalPages}
                onPageChange={setPage}
              />
            </>
          )}
        </div>
      </section>
    </div>
  );
}
