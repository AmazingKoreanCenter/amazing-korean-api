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
import { Skeleton } from "@/components/ui/skeleton";
import type { VideoListReq } from "@/category/video/types";

import { VideoCard } from "../components/video_card";
import { useVideoList } from "../hook/use_video_list";

const PER_PAGE = 9;
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

export function VideoListPage() {
  const [page, setPage] = useState(1);

  const params = useMemo<VideoListReq>(
    () => ({
      page,
      per_page: PER_PAGE,
    }),
    [page]
  );

  const { data, isPending, isFetching } = useVideoList(params);

  // 🚨 [수정 1] items -> data (타입 정의에 맞춤)
  const items = data?.data ?? [];
  const meta = data?.meta;

  // 🚨 [수정 2] meta.page -> meta.current_page
  const currentPage = meta?.current_page ?? page;
  
  // 🚨 [수정 3] meta.total_pages가 없을 경우 방어 코드 강화
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
        <div className="mb-8 flex flex-col gap-3 md:flex-row md:items-end md:justify-between">
          <div>
            <Link
              to="/"
              className="text-sm text-muted-foreground hover:text-foreground transition mb-2 inline-block"
            >
              &larr; 홈으로
            </Link>
            <h1 className="text-2xl font-bold tracking-tight md:text-3xl">
              영상 학습
            </h1>
            <p className="text-sm text-muted-foreground">
              Vimeo 기반 학습 영상을 둘러보세요.
            </p>
          </div>
          {meta && (
            <div className="text-xs text-muted-foreground">
              {/* 🚨 [수정 4] meta.total -> meta.total_count */}
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
        </div>

        {isPending ? (
          <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
            {Array.from({ length: PER_PAGE }, (_, index) => (
              <div key={`skeleton-${index}`} className="space-y-3">
                <Skeleton className="h-40 w-full" />
                <Skeleton className="h-4 w-3/4" />
                <Skeleton className="h-4 w-1/2" />
                <Skeleton className="h-4 w-full" />
              </div>
            ))}
          </div>
        ) : items.length === 0 ? (
          <div className="rounded-lg border border-dashed bg-background p-12 text-center text-sm text-muted-foreground">
            등록된 영상이 없습니다.
          </div>
        ) : (
          <>
            <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
              {items.map((video) => (
                <VideoCard key={video.video_id} video={video} />
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