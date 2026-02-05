import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { BookMarked, Layers, Lock, Crown } from "lucide-react";

import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import type { LessonListReq, LessonAccess } from "@/category/lesson/types";

import { useLessonList } from "../hook/use_lesson_list";

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

export function LessonListPage() {
  const { t } = useTranslation();
  const [page, setPage] = useState(1);
  const sort = undefined;

  const getAccessBadge = (access: LessonAccess) => {
    switch (access) {
      case "public":
        return null; // Don't show badge for public (free) content
      case "paid":
        return (
          <Badge className="absolute top-3 right-3 bg-amber-500 hover:bg-amber-500 text-white border-0 gap-1">
            <Crown className="h-3 w-3" />
            {t("lesson.accessPaid")}
          </Badge>
        );
      case "private":
        return (
          <Badge className="absolute top-3 right-3 bg-gray-500 hover:bg-gray-500 text-white border-0 gap-1">
            <Lock className="h-3 w-3" />
            {t("lesson.accessPrivate")}
          </Badge>
        );
      case "promote":
        return (
          <Badge className="absolute top-3 right-3 bg-green-500 hover:bg-green-500 text-white border-0">
            {t("lesson.accessPromote")}
          </Badge>
        );
      default:
        return null;
    }
  };

  const params = useMemo<LessonListReq>(() => {
    return {
      page,
      per_page: PER_PAGE,
      sort,
    };
  }, [page, sort]);

  const { data, isPending, isFetching } = useLessonList(params);

  const items = data?.items ?? [];
  const meta = data?.meta;

  const currentPage = meta?.current_page ?? page;
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

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="bg-gradient-to-br from-[#F0F3FF] via-white to-[#E8F4FF] border-b">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8 py-12 lg:py-16">
          <div className="space-y-4">
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-white shadow-sm border">
              <Layers className="h-5 w-5 text-secondary" />
              <span className="text-sm font-medium text-muted-foreground">
                {t("lesson.heroBadge")}
              </span>
            </div>
            <h1 className="text-3xl md:text-4xl font-bold tracking-tight">
              {t("lesson.listTitle")}
            </h1>
            <p className="text-muted-foreground max-w-lg">
              {t("lesson.listDescription")}
            </p>
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
                <BookMarked className="h-4 w-4" />
                <span>
                  {t("lesson.totalLessons", { count: meta.total_count ?? 0 })}
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
                  <Skeleton className="aspect-video w-full" />
                  <CardHeader className="space-y-2">
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
                <BookMarked className="h-10 w-10 text-muted-foreground" />
              </div>
              <h3 className="text-lg font-semibold mb-2">{t("lesson.emptyTitle")}</h3>
              <p className="text-sm text-muted-foreground">
                {t("lesson.emptyDescription")}
              </p>
            </div>
          ) : (
            <>
              {/* Lesson Cards Grid */}
              <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                {items.map((lesson) => (
                  <Link key={lesson.id} to={`/lessons/${lesson.id}`}>
                    <Card className="h-full border-0 shadow-card rounded-2xl overflow-hidden transition-all duration-300 hover:-translate-y-1 hover:shadow-card-hover group relative">
                      {getAccessBadge(lesson.lesson_access)}
                      {lesson.thumbnail_url ? (
                        <div className="aspect-video w-full overflow-hidden bg-muted">
                          <img
                            src={lesson.thumbnail_url}
                            alt={lesson.title}
                            className="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
                          />
                        </div>
                      ) : (
                        <div className="aspect-video w-full bg-gradient-to-br from-primary/10 to-secondary/10 flex items-center justify-center">
                          <BookMarked className="h-12 w-12 text-primary/30" />
                        </div>
                      )}
                      <CardHeader className="space-y-2 pb-2">
                        <CardTitle className="text-lg leading-snug group-hover:text-primary transition-colors line-clamp-2">
                          {lesson.title}
                        </CardTitle>
                      </CardHeader>
                      <CardContent className="pt-0 space-y-2">
                        {lesson.description && (
                          <p className="text-sm text-muted-foreground line-clamp-2">
                            {lesson.description}
                          </p>
                        )}
                        <p className="text-xs text-muted-foreground/60">
                          ID: {lesson.lesson_idx}
                        </p>
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
