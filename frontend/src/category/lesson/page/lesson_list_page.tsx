import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { BookMarked, Layers, Lock, Crown } from "lucide-react";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { HeroSection } from "@/components/sections/hero_section";
import { ListStatsBar } from "@/components/sections/list_stats_bar";
import { EmptyState } from "@/components/sections/empty_state";
import { PaginationBar } from "@/components/sections/pagination_bar";
import { SkeletonGrid } from "@/components/sections/skeleton_grid";
import { PageMeta } from "@/components/page_meta";
import type { LessonListReq, LessonAccess } from "@/category/lesson/types";

import { useLessonList } from "../hook/use_lesson_list";

const PER_PAGE = 10;

export function LessonListPage() {
  const { t } = useTranslation();
  const [page, setPage] = useState(1);
  const sort = undefined;

  const getAccessBadge = (access: LessonAccess) => {
    switch (access) {
      case "public":
        return null;
      case "paid":
        return (
          <Badge variant="warning" className="absolute top-3 right-3 border-0 gap-1">
            <Crown className="h-3 w-3" />
            {t("lesson.accessPaid")}
          </Badge>
        );
      case "private":
        return (
          <Badge className="absolute top-3 right-3 bg-muted-foreground hover:bg-muted-foreground text-white border-0 gap-1">
            <Lock className="h-3 w-3" />
            {t("lesson.accessPrivate")}
          </Badge>
        );
      case "promote":
        return (
          <Badge variant="success" className="absolute top-3 right-3 border-0">
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

  return (
    <div className="min-h-screen">
      <PageMeta titleKey="seo.lessons.title" descriptionKey="seo.lessons.description" />
      <HeroSection
        variant="list"
        badge={
          <>
            <Layers className="h-5 w-5 text-secondary" />
            <span className="text-sm font-medium text-muted-foreground">
              {t("lesson.heroBadge")}
            </span>
          </>
        }
        title={t("lesson.listTitle")}
        subtitle={t("lesson.listDescription")}
      />

      {/* Content Section */}
      <section className="py-10 lg:py-14">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8">
          {meta && (
            <ListStatsBar
              icon={BookMarked}
              totalLabel={t("lesson.totalLessons", { count: meta.total_count ?? 0 })}
              currentPage={currentPage}
              totalPages={totalPages}
              isFetching={isFetching}
            />
          )}

          {isPending ? (
            <SkeletonGrid count={PER_PAGE} variant="content-card" />
          ) : items.length === 0 ? (
            <EmptyState
              icon={<BookMarked className="h-10 w-10 text-muted-foreground" />}
              title={t("lesson.emptyTitle")}
              description={t("lesson.emptyDescription")}
            />
          ) : (
            <>
              {/* Lesson Cards Grid */}
              <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                {items.map((lesson) => (
                  <Link key={lesson.id} to={`/lessons/${lesson.id}`}>
                    <Card variant="interactive" className="h-full group relative">
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
