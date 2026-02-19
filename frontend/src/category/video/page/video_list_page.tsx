import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Play, Film } from "lucide-react";

import { HeroSection } from "@/components/sections/hero_section";
import { ListStatsBar } from "@/components/sections/list_stats_bar";
import { EmptyState } from "@/components/sections/empty_state";
import { PaginationBar } from "@/components/sections/pagination_bar";
import { SkeletonGrid } from "@/components/sections/skeleton_grid";
import type { VideoListReq } from "@/category/video/types";

import { VideoCard } from "../components/video_card";
import { useVideoList } from "../hook/use_video_list";

const PER_PAGE = 9;

export function VideoListPage() {
  const { t } = useTranslation();
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

  return (
    <div className="min-h-screen">
      <HeroSection
        variant="list"
        badge={
          <>
            <Play className="h-5 w-5 text-secondary" />
            <span className="text-sm font-medium text-muted-foreground">
              {t("video.heroBadge")}
            </span>
          </>
        }
        title={t("video.listTitle")}
        subtitle={t("video.listDescription")}
      />

      {/* Content Section */}
      <section className="py-10 lg:py-14">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8">
          {meta && (
            <ListStatsBar
              icon={Film}
              totalLabel={t("video.totalVideos", { count: meta.total_count ?? 0 })}
              total={meta.total_count ?? 0}
              currentPage={currentPage}
              totalPages={totalPages}
              isFetching={isFetching}
            />
          )}

          {isPending ? (
            <SkeletonGrid count={PER_PAGE} variant="video-card" />
          ) : items.length === 0 ? (
            <EmptyState
              icon={<Film className="h-10 w-10 text-muted-foreground" />}
              title={t("video.emptyTitle")}
            />
          ) : (
            <>
              <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                {items.map((video) => (
                  <VideoCard key={video.video_id} video={video} />
                ))}
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
