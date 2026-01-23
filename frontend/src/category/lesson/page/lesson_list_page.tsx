import { useMemo, useState } from "react";
import { Link } from "react-router-dom";

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
import type { LessonListReq } from "@/category/lesson/types";

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
  const [page, setPage] = useState(1);
  const sort = undefined;

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
    <div className="min-h-screen bg-muted/30">
      <div className="mx-auto w-full max-w-screen-xl px-4 py-10">
        <div className="mb-8">
          <Link
            to="/"
            className="text-sm text-muted-foreground hover:text-foreground transition mb-2 inline-block"
          >
            &larr; 홈으로
          </Link>
          <h1 className="text-2xl font-bold tracking-tight md:text-3xl">
            레슨 목록
          </h1>
          <p className="text-sm text-muted-foreground">
            단계별 학습 레슨을 선택하세요.
          </p>
        </div>

        {meta && (
          <div className="mb-6 text-xs text-muted-foreground">
            총 {(meta.total_count ?? 0).toLocaleString()}개 · {currentPage}/
            {totalPages} 페이지
            {isFetching && (
              <span className="ml-2 inline-flex items-center gap-1">
                <span className="inline-block h-2 w-2 animate-pulse rounded-full bg-primary" />
                불러오는 중
              </span>
            )}
          </div>
        )}

        {isPending ? (
          <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
            {Array.from({ length: PER_PAGE }, (_, index) => (
              <div key={`skeleton-${index}`} className="space-y-3">
                <Skeleton className="h-40 w-full" />
                <Skeleton className="h-4 w-2/3" />
                <Skeleton className="h-4 w-1/2" />
              </div>
            ))}
          </div>
        ) : items.length === 0 ? (
          <div className="rounded-lg border border-dashed bg-background p-12 text-center text-sm text-muted-foreground">
            등록된 레슨이 없습니다.
          </div>
        ) : (
          <>
            <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
              {items.map((lesson) => (
                <Link key={lesson.id} to={`/lessons/${lesson.id}`}>
                  <Card className="h-full transition hover:-translate-y-1 hover:shadow-lg overflow-hidden">
                    {lesson.thumbnail_url && (
                      <div className="aspect-video w-full overflow-hidden">
                        <img
                          src={lesson.thumbnail_url}
                          alt={lesson.title}
                          className="h-full w-full object-cover"
                        />
                      </div>
                    )}
                    <CardHeader className="space-y-2">
                      <CardTitle className="text-lg">{lesson.title}</CardTitle>
                    </CardHeader>
                    <CardContent className="space-y-2 text-sm text-muted-foreground">
                      {lesson.description && <p className="line-clamp-2">{lesson.description}</p>}
                      <p className="text-xs">ID: {lesson.lesson_idx}</p>
                    </CardContent>
                  </Card>
                </Link>
              ))}
            </div>
            {totalPages > 1 && (
              <div className="mt-10 flex justify-center">
                <Pagination>
                  <PaginationContent>
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
                        className={!hasPrev ? "pointer-events-none opacity-50" : ""}
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
                        className={!hasNext ? "pointer-events-none opacity-50" : ""}
                      />
                    </PaginationItem>
                  </PaginationContent>
                </Pagination>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}
