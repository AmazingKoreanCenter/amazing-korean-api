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
          <div className="flex items-center gap-4 mb-4">
            <div className="w-12 h-12 rounded-xl gradient-primary flex items-center justify-center">
              <Play className="h-6 w-6 text-white" />
            </div>
            <div>
              <h1 className="text-2xl md:text-3xl font-bold tracking-tight">
                영상 학습
              </h1>
              <p className="text-muted-foreground">
                다양한 주제의 한국어 영상으로 자연스럽게 학습하세요
              </p>
            </div>
          </div>

          {meta && (
            <div className="flex items-center gap-4 mt-6">
              <div className="flex items-center gap-2 px-4 py-2 bg-white rounded-full shadow-sm">
                <Film className="h-4 w-4 text-muted-foreground" />
                <span className="text-sm font-medium">
                  총 {(meta.total_count ?? 0).toLocaleString()}개 영상
                </span>
              </div>
              {isFetching && (
                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                  <span className="inline-block h-2 w-2 animate-pulse rounded-full bg-accent" />
                  불러오는 중
                </div>
              )}
            </div>
          )}
        </div>
      </section>

      {/* Content Section */}
      <section className="py-10 lg:py-14">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8">
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
