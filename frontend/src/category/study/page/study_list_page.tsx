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

const PROGRAM_LABELS: Record<StudyProgram, string> = {
  basic_pronunciation: "기초 발음",
  basic_word: "기초 단어",
  basic_900: "기초 900",
  topik_read: "TOPIK 읽기",
  topik_listen: "TOPIK 듣기",
  topik_write: "TOPIK 쓰기",
  tbc: "TBC",
};

const PROGRAM_OPTIONS: Array<{ value: StudyProgram | "all"; label: string }> = [
  { value: "all", label: "전체 프로그램" },
  { value: "basic_pronunciation", label: PROGRAM_LABELS.basic_pronunciation },
  { value: "basic_word", label: PROGRAM_LABELS.basic_word },
  { value: "basic_900", label: PROGRAM_LABELS.basic_900 },
  { value: "topik_read", label: PROGRAM_LABELS.topik_read },
  { value: "topik_listen", label: PROGRAM_LABELS.topik_listen },
  { value: "topik_write", label: PROGRAM_LABELS.topik_write },
  { value: "tbc", label: PROGRAM_LABELS.tbc },
];

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
  const [page, setPage] = useState(1);
  const [program, setProgram] = useState<StudyProgram | "all">("all");
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

  const items = data?.data ?? [];
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
    <div className="min-h-screen bg-muted/30">
      <div className="mx-auto w-full max-w-screen-xl px-4 py-10">
        <div className="mb-8 flex flex-col gap-4 md:flex-row md:items-end md:justify-between">
          <div>
            <h1 className="text-2xl font-bold tracking-tight md:text-3xl">
              학습 문제 목록
            </h1>
            <p className="text-sm text-muted-foreground">
              학습 프로그램별 문제를 확인하고 도전해보세요.
            </p>
          </div>
          <div className="w-full md:w-72">
            <div className="mb-2 text-xs font-medium text-muted-foreground">
              Program
            </div>
            <Select value={program} onValueChange={handleProgramChange}>
              <SelectTrigger>
                <SelectValue placeholder="전체 프로그램" />
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

        {meta && (
          <div className="mb-6 text-xs text-muted-foreground">
            총 {(meta.total ?? 0).toLocaleString()}개 · {currentPage}/
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
                <Skeleton className="h-28 w-full" />
                <Skeleton className="h-4 w-2/3" />
                <Skeleton className="h-4 w-1/2" />
                <Skeleton className="h-4 w-full" />
              </div>
            ))}
          </div>
        ) : items.length === 0 ? (
          <div className="rounded-lg border border-dashed bg-background p-12 text-center text-sm text-muted-foreground">
            등록된 문제가 없습니다.
          </div>
        ) : (
          <>
            <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
              {items.map((study) => (
                <Link key={study.study_id} to={`/studies/${study.study_id}`}>
                  <Card className="h-full transition hover:-translate-y-1 hover:shadow-lg">
                    <CardHeader className="space-y-3">
                      <div className="flex items-center justify-between gap-2">
                        <Badge variant="secondary">
                          {PROGRAM_LABELS[study.study_program]}
                        </Badge>
                        <span className="text-xs text-muted-foreground">
                          {formatDate(study.created_at)}
                        </span>
                      </div>
                      <CardTitle className="text-lg">
                        {study.title ?? "제목 없음"}
                      </CardTitle>
                    </CardHeader>
                    <CardContent className="space-y-2 text-sm text-muted-foreground">
                      {study.subtitle && <p>{study.subtitle}</p>}
                      <p className="text-xs">ID · {study.study_idx}</p>
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
