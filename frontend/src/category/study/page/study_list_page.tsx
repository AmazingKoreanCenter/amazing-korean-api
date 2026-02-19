import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { BookOpen, Calendar, GraduationCap, Filter } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { HeroSection } from "@/components/sections/hero_section";
import { ListStatsBar } from "@/components/sections/list_stats_bar";
import { EmptyState } from "@/components/sections/empty_state";
import { PaginationBar } from "@/components/sections/pagination_bar";
import { SkeletonGrid } from "@/components/sections/skeleton_grid";
import type { StudyListReq, StudyProgram } from "@/category/study/types";
import { studyProgramSchema } from "@/category/study/types";

import { useStudyList } from "../hook/use_study_list";

const PER_PAGE = 10;

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
      <HeroSection
        variant="list"
        badge={
          <>
            <GraduationCap className="h-5 w-5 text-secondary" />
            <span className="text-sm font-medium text-muted-foreground">
              {t("study.heroBadge")}
            </span>
          </>
        }
        title={t("study.listTitle")}
        subtitle={t("study.listDescription")}
      >
        {/* Filter Section */}
        <div className="w-full lg:w-80">
          <div className="bg-card rounded-2xl shadow-card p-4 space-y-3">
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
      </HeroSection>

      {/* Content Section */}
      <section className="py-10 lg:py-14">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8">
          {meta && (
            <ListStatsBar
              icon={BookOpen}
              totalLabel={t("study.totalProblems", { count: meta.total_count ?? 0 })}
              total={meta.total_count ?? 0}
              currentPage={currentPage}
              totalPages={totalPages}
              isFetching={isFetching}
            />
          )}

          {/* Loading State */}
          {isPending ? (
            <SkeletonGrid count={PER_PAGE} variant="study-card" />
          ) : items.length === 0 ? (
            <EmptyState
              icon={<BookOpen className="h-10 w-10 text-muted-foreground" />}
              title={t("study.emptyTitle")}
              description={t("study.emptyDescription")}
            />
          ) : (
            <>
              {/* Study Cards Grid */}
              <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                {items.map((study) => (
                  <Link key={study.study_id} to={`/studies/${study.study_id}`}>
                    <Card variant="interactive" className="h-full group">
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
