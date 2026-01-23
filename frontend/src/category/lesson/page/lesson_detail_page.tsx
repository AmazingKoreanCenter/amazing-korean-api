import { useEffect, useMemo } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";

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

function LessonItemCard({ item }: { item: LessonItemRes }) {
  const getItemLink = () => {
    if (item.kind === "video" && item.video_id) {
      return `/videos/${item.video_id}`;
    }
    if (item.kind === "task" && item.task_id) {
      return `/studies/tasks/${item.task_id}`;
    }
    return null;
  };

  const link = getItemLink();

  const content = (
    <Card className="transition hover:bg-muted/50">
      <CardContent className="p-4 flex items-center gap-4">
        <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-primary font-bold">
          {item.seq}
        </div>
        <div className="flex-1">
          <Badge variant="outline">{KIND_LABELS[item.kind] || item.kind}</Badge>
        </div>
        {link && (
          <span className="text-sm text-muted-foreground">
            {item.kind === "video" ? `영상 #${item.video_id}` : `문제 #${item.task_id}`}
          </span>
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
      <div className="min-h-screen bg-muted/30">
        <div className="mx-auto w-full max-w-screen-md space-y-6 px-4 py-10">
          <Skeleton className="h-8 w-1/2" />
          <Skeleton className="h-4 w-1/3" />
          <Skeleton className="h-2 w-full" />
          <div className="space-y-3">
            {Array.from({ length: 5 }, (_, i) => (
              <Skeleton key={i} className="h-16 w-full" />
            ))}
          </div>
        </div>
      </div>
    );
  }

  if (isError || !data) {
    const isNotFound = error instanceof ApiError && error.status === 404;
    return (
      <div className="min-h-screen bg-muted/30 flex items-center justify-center p-4">
        <Card className="w-full max-w-md text-center">
          <CardHeader>
            <CardTitle>
              {isNotFound ? "레슨을 찾을 수 없습니다." : "오류 발생"}
            </CardTitle>
            <p className="text-sm text-muted-foreground">
              {isNotFound
                ? "존재하지 않거나 삭제된 레슨입니다."
                : "일시적인 오류입니다. 다시 시도해주세요."}
            </p>
          </CardHeader>
          <CardContent>
            <Button asChild>
              <Link to="/lessons">목록으로 돌아가기</Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  const progressPercent = progressData?.percent ?? 0;

  return (
    <div className="min-h-screen bg-muted/30">
      <div className="mx-auto w-full max-w-screen-md space-y-6 px-4 py-10">
        <div>
          <h1 className="text-2xl font-bold tracking-tight md:text-3xl">
            {data.title}
          </h1>
          {data.description && (
            <p className="mt-2 text-muted-foreground">{data.description}</p>
          )}
        </div>

        {isLoggedIn && (
          <Card>
            <CardContent className="p-4 space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">학습 진도</span>
                <span className="font-medium">{progressPercent}%</span>
              </div>
              <div className="h-2 w-full rounded-full bg-muted overflow-hidden">
                <div
                  className="h-full bg-primary transition-all"
                  style={{ width: `${progressPercent}%` }}
                />
              </div>
              {progressData?.last_seq && (
                <p className="text-xs text-muted-foreground">
                  마지막 학습: {progressData.last_seq}번 항목
                </p>
              )}
            </CardContent>
          </Card>
        )}

        <div className="space-y-3">
          <h2 className="text-lg font-semibold">
            학습 항목 ({data.items.length}개)
          </h2>
          {data.items.length === 0 ? (
            <div className="rounded-lg border border-dashed bg-background p-8 text-center text-sm text-muted-foreground">
              등록된 학습 항목이 없습니다.
            </div>
          ) : (
            <div className="space-y-2">
              {data.items.map((item) => (
                <LessonItemCard key={item.seq} item={item} />
              ))}
            </div>
          )}
        </div>

        <div className="flex justify-between pt-4">
          <Button variant="outline" asChild>
            <Link to="/lessons">목록으로</Link>
          </Button>
          {isLoggedIn && data.items.length > 0 && progressPercent < 100 && (
            <Button
              onClick={() => {
                const lastItem = data.items[data.items.length - 1];
                handleProgressUpdate(100, lastItem.seq);
              }}
              disabled={updateProgress.isPending}
            >
              {updateProgress.isPending ? "저장 중..." : "학습 완료"}
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}
