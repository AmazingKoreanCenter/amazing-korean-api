import { useEffect, useMemo } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";
import { ArrowLeft, BookMarked, CheckCircle2, ClipboardList, Play, ChevronRight } from "lucide-react";

import { ApiError } from "@/api/client";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useAuthStore } from "@/hooks/use_auth_store";
import type { LessonItemRes } from "@/category/lesson/types";

import { useLessonDetail } from "../hook/use_lesson_detail";
import { useLessonProgress, useUpdateLessonProgress } from "../hook/use_lesson_progress";

const KIND_LABELS: Record<string, string> = {
  video: "영상",
  task: "문제",
};

const KIND_ICONS: Record<string, typeof Play> = {
  video: Play,
  task: ClipboardList,
};

interface LessonItemCardProps {
  item: LessonItemRes;
  lessonId: number;
  totalItems: number;
  lastSeq?: number;
}

function LessonItemCard({ item, lessonId, totalItems, lastSeq }: LessonItemCardProps) {
  const isCompleted = lastSeq !== undefined && item.seq <= lastSeq;
  const KindIcon = KIND_ICONS[item.kind] || ClipboardList;

  const getItemLink = () => {
    const params = new URLSearchParams({
      lessonId: String(lessonId),
      itemSeq: String(item.seq),
      totalItems: String(totalItems),
    });

    if (item.kind === "video" && item.video_id) {
      return `/videos/${item.video_id}?${params.toString()}`;
    }
    if (item.kind === "task" && item.task_id) {
      return `/studies/tasks/${item.task_id}?${params.toString()}`;
    }
    return null;
  };

  const link = getItemLink();

  const content = (
    <Card className={`border-0 shadow-sm rounded-xl transition-all duration-200 hover:shadow-card hover:-translate-y-0.5 group ${isCompleted ? "bg-green-50/50" : ""}`}>
      <CardContent className="p-4 flex items-center gap-4">
        <div className={`flex h-12 w-12 items-center justify-center rounded-xl font-bold text-lg shrink-0 ${
          isCompleted
            ? "bg-green-100 text-green-600"
            : "bg-gradient-to-br from-primary/10 to-secondary/10 text-primary"
        }`}>
          {isCompleted ? (
            <CheckCircle2 className="h-6 w-6" />
          ) : (
            item.seq
          )}
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            <Badge variant="outline" className="gap-1 px-2 py-0.5 rounded-md text-xs">
              <KindIcon className="h-3 w-3" />
              {KIND_LABELS[item.kind] || item.kind}
            </Badge>
            {isCompleted && (
              <span className="text-xs text-green-600 font-medium">완료</span>
            )}
          </div>
          <p className="text-sm text-muted-foreground truncate">
            {item.kind === "video" ? `영상 #${item.video_id}` : `문제 #${item.task_id}`}
          </p>
        </div>
        {link && (
          <ChevronRight className="h-5 w-5 text-muted-foreground group-hover:text-primary transition-colors shrink-0" />
        )}
      </CardContent>
    </Card>
  );

  if (link) {
    return <Link to={link}>{content}</Link>;
  }

  return content;
}

export function LessonDetailPage() {
  const { lessonId } = useParams();
  const navigate = useNavigate();
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);

  const id = useMemo(() => Number(lessonId), [lessonId]);
  const isValidId = Number.isFinite(id);

  const { data, isPending, isError, error } = useLessonDetail(isValidId ? id : undefined);
  const { data: progressData } = useLessonProgress(isValidId ? id : undefined);
  const updateProgress = useUpdateLessonProgress(id);

  useEffect(() => {
    if (!isValidId) {
      navigate("/lessons", { replace: true });
    }
  }, [isValidId, navigate]);

  const handleProgressUpdate = (percent: number, lastSeq?: number) => {
    if (!isLoggedIn) return;
    updateProgress.mutate({
      percent,
      last_seq: lastSeq,
    });
  };

  if (!isValidId) return null;

  if (isPending) {
    return (
      <div className="min-h-screen">
        <section className="bg-gradient-to-br from-[#F0F3FF] via-white to-[#E8F4FF] border-b">
          <div className="max-w-[900px] mx-auto px-6 lg:px-8 py-10 lg:py-14">
            <Skeleton className="h-6 w-20 mb-4" />
            <Skeleton className="h-10 w-2/3 mb-3" />
            <Skeleton className="h-5 w-1/2" />
          </div>
        </section>
        <section className="py-10">
          <div className="max-w-[900px] mx-auto px-6 lg:px-8 space-y-4">
            {Array.from({ length: 5 }, (_, i) => (
              <Skeleton key={i} className="h-20 w-full rounded-xl" />
            ))}
          </div>
        </section>
      </div>
    );
  }

  if (isError || !data) {
    const isNotFound = error instanceof ApiError && error.status === 404;
    return (
      <div className="min-h-screen flex items-center justify-center p-4">
        <Card className="w-full max-w-md text-center shadow-card border-0 rounded-2xl">
          <CardHeader className="pb-4">
            <div className="w-16 h-16 rounded-2xl bg-muted flex items-center justify-center mx-auto mb-4">
              <span className="text-3xl">😕</span>
            </div>
            <CardTitle className="text-xl">
              {isNotFound ? "수업을 찾을 수 없습니다" : "오류 발생"}
            </CardTitle>
            <p className="text-sm text-muted-foreground mt-2">
              {isNotFound
                ? "존재하지 않거나 삭제된 수업입니다."
                : "일시적인 오류입니다. 다시 시도해주세요."}
            </p>
          </CardHeader>
          <CardContent>
            <Button asChild className="gradient-primary text-white rounded-full">
              <Link to="/lessons">목록으로 돌아가기</Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  const progressPercent = progressData?.percent ?? 0;
  const lastSeq = progressData?.last_seq;

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="bg-gradient-to-br from-[#F0F3FF] via-white to-[#E8F4FF] border-b">
        <div className="max-w-[900px] mx-auto px-6 lg:px-8 py-10 lg:py-14">
          <Link
            to="/lessons"
            className="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-primary transition-colors mb-6"
          >
            <ArrowLeft className="h-4 w-4" />
            목록으로
          </Link>

          <div className="space-y-4">
            <h1 className="text-3xl md:text-4xl font-bold tracking-tight">
              {data.title}
            </h1>
            {data.description && (
              <p className="text-lg text-muted-foreground max-w-2xl">
                {data.description}
              </p>
            )}
          </div>

          {/* Progress Card */}
          {isLoggedIn && (
            <Card className="mt-8 border-0 shadow-card rounded-2xl overflow-hidden">
              <CardContent className="p-6">
                <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                  <div className="space-y-2 flex-1">
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-medium">학습 진도</span>
                      <span className="text-2xl font-bold text-primary">{progressPercent}%</span>
                    </div>
                    <div className="h-3 w-full rounded-full bg-muted overflow-hidden">
                      <div
                        className="h-full gradient-primary transition-all duration-500"
                        style={{ width: `${progressPercent}%` }}
                      />
                    </div>
                    {lastSeq && (
                      <p className="text-xs text-muted-foreground">
                        마지막 학습: {lastSeq}번째 항목
                      </p>
                    )}
                  </div>
                  {data.items.length > 0 && progressPercent < 100 && (
                    <Button
                      onClick={() => {
                        const lastItem = data.items[data.items.length - 1];
                        handleProgressUpdate(100, lastItem.seq);
                      }}
                      disabled={updateProgress.isPending}
                      className="gradient-primary text-white rounded-full shrink-0"
                    >
                      {updateProgress.isPending ? "저장 중..." : "수업 완료하기"}
                    </Button>
                  )}
                  {progressPercent >= 100 && (
                    <Badge className="bg-green-100 text-green-700 border-0 px-4 py-2 text-sm shrink-0">
                      <CheckCircle2 className="h-4 w-4 mr-1.5" />
                      수업 완료
                    </Badge>
                  )}
                </div>
              </CardContent>
            </Card>
          )}
        </div>
      </section>

      {/* Items Section */}
      <section className="py-10 lg:py-14">
        <div className="max-w-[900px] mx-auto px-6 lg:px-8">
          <div className="flex items-center gap-2 mb-6">
            <BookMarked className="h-5 w-5 text-primary" />
            <h2 className="text-xl font-semibold">
              학습 항목
            </h2>
            <Badge variant="secondary" className="ml-2 rounded-full">
              {data.items.length}개
            </Badge>
          </div>

          {data.items.length === 0 ? (
            <div className="text-center py-16">
              <div className="w-16 h-16 rounded-2xl bg-muted flex items-center justify-center mx-auto mb-4">
                <BookMarked className="h-8 w-8 text-muted-foreground" />
              </div>
              <h3 className="text-lg font-semibold mb-2">등록된 학습 항목이 없습니다</h3>
              <p className="text-sm text-muted-foreground">
                이 수업에는 아직 학습 항목이 등록되지 않았습니다.
              </p>
            </div>
          ) : (
            <div className="space-y-3">
              {data.items.map((item) => (
                <LessonItemCard
                  key={item.seq}
                  item={item}
                  lessonId={id}
                  totalItems={data.items.length}
                  lastSeq={lastSeq}
                />
              ))}
            </div>
          )}

          {/* Bottom Navigation */}
          <div className="mt-10 pt-6 border-t">
            <Button variant="outline" asChild className="rounded-full">
              <Link to="/lessons">
                <ArrowLeft className="h-4 w-4 mr-2" />
                목록으로 돌아가기
              </Link>
            </Button>
          </div>
        </div>
      </section>
    </div>
  );
}
