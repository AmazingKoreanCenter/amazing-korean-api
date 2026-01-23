import { useMemo, useState } from "react";
import { Link, useParams } from "react-router-dom";

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

const PROGRAM_LABELS: Record<StudyProgram, string> = {
  basic_pronunciation: "기초 발음",
  basic_word: "기초 단어",
  basic_900: "기초 900",
  topik_read: "TOPIK 읽기",
  topik_listen: "TOPIK 듣기",
  topik_write: "TOPIK 쓰기",
  tbc: "TBC",
};

const KIND_LABELS: Record<StudyTaskKind, string> = {
  choice: "객관식",
  typing: "입력형",
  voice: "음성형",
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
  const { studyId: studyIdParam } = useParams<{ studyId: string }>();
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
      <div className="min-h-screen bg-muted/30 flex items-center justify-center">
        <p className="text-muted-foreground">잘못된 Study ID입니다.</p>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-muted/30">
      <div className="mx-auto w-full max-w-screen-xl px-4 py-10">
        {/* Header */}
        {isPending ? (
          <div className="mb-8 space-y-3">
            <Skeleton className="h-8 w-1/3" />
            <Skeleton className="h-5 w-1/2" />
          </div>
        ) : data ? (
          <div className="mb-8">
            <div className="flex items-center gap-3 mb-2">
              <Link
                to="/studies"
                className="text-sm text-muted-foreground hover:text-foreground transition"
              >
                &larr; 목록으로
              </Link>
              <Badge variant="secondary">{PROGRAM_LABELS[data.program]}</Badge>
            </div>
            <h1 className="text-2xl font-bold tracking-tight md:text-3xl">
              {data.title ?? "제목 없음"}
            </h1>
            {data.subtitle && (
              <p className="text-sm text-muted-foreground mt-1">{data.subtitle}</p>
            )}
            <p className="text-xs text-muted-foreground mt-2">
              Study ID: {data.study_idx}
            </p>
          </div>
        ) : null}

        {/* Meta Info */}
        {meta && (
          <div className="mb-6 text-xs text-muted-foreground">
            총 {(meta.total_count ?? 0).toLocaleString()}개 문제 · {currentPage}/
            {totalPages} 페이지
            {isFetching && (
              <span className="ml-2 inline-flex items-center gap-1">
                <span className="inline-block h-2 w-2 animate-pulse rounded-full bg-primary" />
                불러오는 중
              </span>
            )}
          </div>
        )}

        {/* Task List */}
        {isPending ? (
          <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
            {Array.from({ length: PER_PAGE }, (_, index) => (
              <div key={`skeleton-${index}`} className="space-y-3">
                <Skeleton className="h-24 w-full" />
              </div>
            ))}
          </div>
        ) : tasks.length === 0 ? (
          <div className="rounded-lg border border-dashed bg-background p-12 text-center text-sm text-muted-foreground">
            등록된 문제가 없습니다.
          </div>
        ) : (
          <>
            <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
              {tasks.map((task) => (
                <Link key={task.task_id} to={`/studies/tasks/${task.task_id}`}>
                  <Card className="h-full transition hover:-translate-y-1 hover:shadow-lg">
                    <CardHeader className="pb-2">
                      <div className="flex items-center justify-between">
                        <Badge variant="outline">{KIND_LABELS[task.kind]}</Badge>
                        <span className="text-xs text-muted-foreground">
                          #{task.seq}
                        </span>
                      </div>
                    </CardHeader>
                    <CardContent>
                      <CardTitle className="text-base">
                        문제 {task.seq}
                      </CardTitle>
                      <p className="text-xs text-muted-foreground mt-1">
                        Task ID: {task.task_id}
                      </p>
                    </CardContent>
                  </Card>
                </Link>
              ))}
            </div>

            {/* Pagination */}
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
