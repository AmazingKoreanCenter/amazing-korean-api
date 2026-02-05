import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link, useParams } from "react-router-dom";
import { ArrowLeft, BookOpen, ClipboardList, Keyboard, Mic } from "lucide-react";

import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import type { StudyDetailReq, StudyProgram, StudyTaskKind } from "@/category/study/types";

import { useStudyDetail } from "../hook/use_study_detail";

const PER_PAGE = 10;
const ELLIPSIS = "ellipsis" as const;

type PageItem = number | typeof ELLIPSIS;


const KIND_ICONS: Record<StudyTaskKind, typeof ClipboardList> = {
  choice: ClipboardList,
  typing: Keyboard,
  voice: Mic,
};

const getPageItems = (currentPage: number, totalPages: number): PageItem[] => {
  if (totalPages <= 7) {
    return Array.from({ length: totalPages }, (_, index) => index + 1);
  }

  const items: PageItem[] = [1];
  const start = Math.max(2, currentPage - 1);
  const end = Math.min(totalPages - 1, currentPage + 1);

  if (start > 2) {
    items.push(ELLIPSIS);
  }

  for (let page = start; page <= end; page += 1) {
    items.push(page);
  }

  if (end < totalPages - 1) {
    items.push(ELLIPSIS);
  }

  items.push(totalPages);
  return items;
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

  const pageItems = useMemo(
    () => getPageItems(currentPage, totalPages),
    [currentPage, totalPages]
  );

  const hasPrev = currentPage > 1;
  const hasNext = currentPage < totalPages;

  const handlePageChange = (nextPage: number) => {
    if (nextPage === page || nextPage < 1 || nextPage > totalPages) {
      return;
    }
    setPage(nextPage);
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

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
      <section className="bg-gradient-to-br from-[#F0F3FF] via-white to-[#E8F4FF] border-b">
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
                  {t("study.totalProblems", { count: (meta.total_count ?? 0).toLocaleString() })}
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
            <div className="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3">
              {Array.from({ length: PER_PAGE }, (_, index) => (
                <Card key={`skeleton-${index}`} className="border-0 shadow-card rounded-2xl overflow-hidden">
                  <CardHeader className="pb-2">
                    <div className="flex items-center justify-between">
                      <Skeleton className="h-6 w-20 rounded-full" />
                      <Skeleton className="h-5 w-12" />
                    </div>
                  </CardHeader>
                  <CardContent>
                    <Skeleton className="h-6 w-2/3" />
                    <Skeleton className="h-4 w-1/2 mt-2" />
                  </CardContent>
                </Card>
              ))}
            </div>
          ) : tasks.length === 0 ? (
            /* Empty State */
            <div className="text-center py-20">
              <div className="w-20 h-20 rounded-2xl bg-muted flex items-center justify-center mx-auto mb-6">
                <BookOpen className="h-10 w-10 text-muted-foreground" />
              </div>
              <h3 className="text-lg font-semibold mb-2">{t("study.emptyTitle")}</h3>
              <p className="text-sm text-muted-foreground">
                {t("study.emptyDetailDescription")}
              </p>
            </div>
          ) : (
            <>
              {/* Task Cards Grid */}
              <div className="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3">
                {tasks.map((task) => {
                  const KindIcon = KIND_ICONS[task.kind];
                  return (
                    <Link key={task.task_id} to={`/studies/tasks/${task.task_id}`}>
                      <Card className="h-full border-0 shadow-card rounded-2xl overflow-hidden transition-all duration-300 hover:-translate-y-1 hover:shadow-card-hover group">
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

              {/* Pagination */}
              {totalPages > 1 && (
                <div className="mt-12 flex justify-center">
                  <Pagination>
                    <PaginationContent className="gap-1">
                      <PaginationItem>
                        <PaginationPrevious
                          href="#"
                          onClick={(event) => {
                            event.preventDefault();
                            if (hasPrev) {
                              handlePageChange(currentPage - 1);
                            }
                          }}
                          aria-disabled={!hasPrev}
                          className={`rounded-xl ${!hasPrev ? "pointer-events-none opacity-50" : ""}`}
                        />
                      </PaginationItem>
                      {pageItems.map((item, index) => (
                        <PaginationItem
                          key={item === ELLIPSIS ? `ellipsis-${index}` : item}
                        >
                          {item === ELLIPSIS ? (
                            <PaginationEllipsis />
                          ) : (
                            <PaginationLink
                              href="#"
                              isActive={item === currentPage}
                              onClick={(event) => {
                                event.preventDefault();
                                handlePageChange(item);
                              }}
                              className={`rounded-xl ${item === currentPage ? "gradient-primary text-white border-0" : ""}`}
                            >
                              {item}
                            </PaginationLink>
                          )}
                        </PaginationItem>
                      ))}
                      <PaginationItem>
                        <PaginationNext
                          href="#"
                          onClick={(event) => {
                            event.preventDefault();
                            if (hasNext) {
                              handlePageChange(currentPage + 1);
                            }
                          }}
                          aria-disabled={!hasNext}
                          className={`rounded-xl ${!hasNext ? "pointer-events-none opacity-50" : ""}`}
                        />
                      </PaginationItem>
                    </PaginationContent>
                  </Pagination>
                </div>
              )}
            </>
          )}
        </div>
      </section>
    </div>
  );
}
