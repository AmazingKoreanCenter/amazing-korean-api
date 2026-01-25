import { useMemo, useState } from "react";
import { Play, Film } from "lucide-react";

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

  const items = data?.data ?? [];
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
              <Play className="h-5 w-5 text-secondary" />
              <span className="text-sm font-medium text-muted-foreground">
                영상으로 배우는 한국어
              </span>
            </div>
            <h1 className="text-3xl md:text-4xl font-bold tracking-tight">
              영상 학습
            </h1>
            <p className="text-muted-foreground max-w-lg">
              다양한 주제의 한국어 영상으로 자연스럽게 학습하세요.
              원어민의 발음과 표현을 직접 보고 들으며 실력을 향상시킬 수 있습니다.
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
                <Film className="h-4 w-4" />
                <span>
                  총 <strong className="text-foreground">{(meta.total_count ?? 0).toLocaleString()}</strong>개 영상
                </span>
                <span className="text-border">|</span>
                <span>{currentPage} / {totalPages} 페이지</span>
              </div>
              {isFetching && (
                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                  <span className="h-2 w-2 animate-pulse rounded-full bg-secondary" />
                  불러오는 중
                </div>
              )}
            </div>
          )}

          {isPending ? (
            <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
              {Array.from({ length: PER_PAGE }, (_, index) => (
                <div key={`skeleton-${index}`} className="space-y-3">
                  <Skeleton className="aspect-video w-full rounded-xl" />
                  <Skeleton className="h-5 w-3/4" />
                  <Skeleton className="h-4 w-1/2" />
                </div>
              ))}
            </div>
          ) : items.length === 0 ? (
            <div className="rounded-2xl border-2 border-dashed bg-muted/30 p-16 text-center">
              <div className="w-16 h-16 rounded-2xl bg-muted flex items-center justify-center mx-auto mb-4">
                <Film className="h-8 w-8 text-muted-foreground" />
              </div>
              <p className="text-muted-foreground">등록된 영상이 없습니다.</p>
            </div>
          ) : (
            <>
              <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                {items.map((video) => (
                  <VideoCard key={video.video_id} video={video} />
                ))}
              </div>

              {totalPages > 1 && (
                <div className="mt-12 flex justify-center">
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
      </section>
    </div>
  );
}
