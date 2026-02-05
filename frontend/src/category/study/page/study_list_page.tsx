import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { BookOpen, Calendar, GraduationCap, Filter } from "lucide-react";

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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { StudyListReq, StudyProgram } from "@/category/study/types";
import { studyProgramSchema } from "@/category/study/types";

import { useStudyList } from "../hook/use_study_list";

const PER_PAGE = 10;
const ELLIPSIS = "ellipsis" as const;

type PageItem = number | typeof ELLIPSIS;


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

const formatDate = (value: string) => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;

  return date.toLocaleDateString("ko-KR", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  });
};

export function StudyListPage() {
  const { t } = useTranslation();
  const [page, setPage] = useState(1);
  const [program, setProgram] = useState<StudyProgram | "all">("all");

  const PROGRAM_LABELS: Record<StudyProgram, string> = {
    basic_pronunciation: t("study.programBasicPronunciation"),
    basic_word: t("study.programBasicWord"),
    basic_900: t("study.programBasic900"),
    topik_read: t("study.programTopikRead"),
    topik_listen: t("study.programTopikListen"),
    topik_write: t("study.programTopikWrite"),
    tbc: t("study.programTbc"),
  };

  const PROGRAM_OPTIONS: Array<{ value: StudyProgram | "all"; label: string }> = [
    { value: "all", label: t("study.filterAll") },
    { value: "basic_pronunciation", label: PROGRAM_LABELS.basic_pronunciation },
    { value: "basic_word", label: PROGRAM_LABELS.basic_word },
    { value: "basic_900", label: PROGRAM_LABELS.basic_900 },
    { value: "topik_read", label: PROGRAM_LABELS.topik_read },
    { value: "topik_listen", label: PROGRAM_LABELS.topik_listen },
    { value: "topik_write", label: PROGRAM_LABELS.topik_write },
    { value: "tbc", label: PROGRAM_LABELS.tbc },
  ];
  const sort = undefined;

  const params = useMemo<StudyListReq>(() => {
    const programParam = program === "all" ? undefined : program;
    return {
      page,
      per_page: PER_PAGE,
      program: programParam,
      sort,
    };
  }, [page, program, sort]);

  const { data, isPending, isFetching } = useStudyList(params);

  const items = data?.list ?? [];
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

  const handleProgramChange = (value: string) => {
    if (value === "all") {
      setProgram("all");
      setPage(1);
      return;
    }

    const parsed = studyProgramSchema.safeParse(value);
    if (parsed.success) {
      setProgram(parsed.data);
      setPage(1);
    }
  };

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="bg-gradient-to-br from-[#F0F3FF] via-white to-[#E8F4FF] border-b">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8 py-12 lg:py-16">
          <div className="flex flex-col lg:flex-row lg:items-end lg:justify-between gap-6">
            <div className="space-y-4">
              <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-white shadow-sm border">
                <GraduationCap className="h-5 w-5 text-secondary" />
                <span className="text-sm font-medium text-muted-foreground">
                  {t("study.heroBadge")}
                </span>
              </div>
              <h1 className="text-3xl md:text-4xl font-bold tracking-tight">
                {t("study.listTitle")}
              </h1>
              <p className="text-muted-foreground max-w-lg">
                {t("study.listDescription")}
              </p>
            </div>

            {/* Filter Section */}
            <div className="w-full lg:w-80">
              <div className="bg-white rounded-2xl shadow-card p-4 space-y-3">
                <div className="flex items-center gap-2 text-sm font-medium text-muted-foreground">
                  <Filter className="h-4 w-4" />
                  {t("study.filterLabel")}
                </div>
                <Select value={program} onValueChange={handleProgramChange}>
                  <SelectTrigger className="bg-muted/30 border-0 rounded-xl">
                    <SelectValue placeholder={t("study.filterAll")} />
                  </SelectTrigger>
                  <SelectContent>
                    {PROGRAM_OPTIONS.map((option) => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>
          </div>
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
            <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
              {Array.from({ length: PER_PAGE }, (_, index) => (
                <Card key={`skeleton-${index}`} className="border-0 shadow-card rounded-2xl overflow-hidden">
                  <CardHeader className="space-y-3">
                    <div className="flex items-center justify-between">
                      <Skeleton className="h-6 w-24 rounded-full" />
                      <Skeleton className="h-4 w-20" />
                    </div>
                    <Skeleton className="h-6 w-3/4" />
                  </CardHeader>
                  <CardContent>
                    <Skeleton className="h-4 w-full" />
                    <Skeleton className="h-4 w-2/3 mt-2" />
                  </CardContent>
                </Card>
              ))}
            </div>
          ) : items.length === 0 ? (
            /* Empty State */
            <div className="text-center py-20">
              <div className="w-20 h-20 rounded-2xl bg-muted flex items-center justify-center mx-auto mb-6">
                <BookOpen className="h-10 w-10 text-muted-foreground" />
              </div>
              <h3 className="text-lg font-semibold mb-2">{t("study.emptyTitle")}</h3>
              <p className="text-sm text-muted-foreground">
                {t("study.emptyDescription")}
              </p>
            </div>
          ) : (
            <>
              {/* Study Cards Grid */}
              <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                {items.map((study) => (
                  <Link key={study.study_id} to={`/studies/${study.study_id}`}>
                    <Card className="h-full border-0 shadow-card rounded-2xl overflow-hidden transition-all duration-300 hover:-translate-y-1 hover:shadow-card-hover group">
                      <CardHeader className="space-y-4 pb-3">
                        <div className="flex items-center justify-between gap-2">
                          <Badge className="gradient-primary text-white border-0 px-3 py-1 rounded-full text-xs">
                            {PROGRAM_LABELS[study.program]}
                          </Badge>
                          <div className="flex items-center gap-1 text-xs text-muted-foreground">
                            <Calendar className="h-3 w-3" />
                            {formatDate(study.created_at)}
                          </div>
                        </div>
                        <CardTitle className="text-lg leading-snug group-hover:text-primary transition-colors">
                          {study.title ?? t("common.noTitle")}
                        </CardTitle>
                      </CardHeader>
                      <CardContent className="pt-0">
                        {study.subtitle && (
                          <p className="text-sm text-muted-foreground line-clamp-2 mb-3">
                            {study.subtitle}
                          </p>
                        )}
                        <div className="text-xs text-muted-foreground/60">
                          ID: {study.study_idx}
                        </div>
                      </CardContent>
                    </Card>
                  </Link>
                ))}
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
