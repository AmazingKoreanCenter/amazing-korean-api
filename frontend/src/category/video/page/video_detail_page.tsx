import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Link, useNavigate, useParams, useSearchParams } from "react-router-dom";
import { ArrowLeft, Calendar, CheckCircle2, ArrowRight } from "lucide-react";

import { ApiError } from "@/api/client";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import type { VideoTag } from "@/category/video/types";
import { useAuthStore } from "@/hooks/use_auth_store";
import { useLessonDetail } from "@/category/lesson/hook/use_lesson_detail";
import { useUpdateLessonProgress } from "@/category/lesson/hook/use_lesson_progress";

import { VideoPlayer } from "../components/video_player";
import { useVideoDetail } from "../hook/use_video_detail";
import { useUpdateVideoProgress, useVideoProgress } from "../hook/use_video_progress";

const formatDate = (value: string) => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;

  return date.toLocaleDateString("ko-KR", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  });
};

const getTagLabel = (tag: VideoTag) => {
  return tag.title ?? tag.subtitle ?? tag.key ?? null;
};

const clampProgressRate = (value: number) => {
  if (!Number.isFinite(value)) return 0;
  const normalized = Math.floor(value);
  return Math.min(100, Math.max(0, normalized));
};

export function VideoDetailPage() {
  const { videoId } = useParams();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
  const pauseTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [isVideoEnded, setIsVideoEnded] = useState(false);

  const id = useMemo(() => Number(videoId), [videoId]);
  const isValidId = Number.isFinite(id);

  const lessonId = useMemo(() => {
    const param = searchParams.get("lessonId");
    return param ? Number(param) : undefined;
  }, [searchParams]);

  const currentItemSeq = useMemo(() => {
    const param = searchParams.get("itemSeq");
    return param ? Number(param) : undefined;
  }, [searchParams]);

  const totalItems = useMemo(() => {
    const param = searchParams.get("totalItems");
    return param ? Number(param) : undefined;
  }, [searchParams]);

  const isInLessonContext = lessonId !== undefined && currentItemSeq !== undefined;

  const { data, isPending, isError, error } = useVideoDetail(id);
  const { data: progressData, isSuccess: isProgressSuccess } = useVideoProgress(
    isValidId ? id : undefined
  );
  const { mutate: updateVideoProgress } = useUpdateVideoProgress(id);

  const { data: lessonData } = useLessonDetail(isInLessonContext ? lessonId : undefined);
  const updateLessonProgress = useUpdateLessonProgress(lessonId ?? 0);

  const nextLessonItem = useMemo(() => {
    if (!isInLessonContext || !lessonData?.items || !currentItemSeq) return null;
    const currentIndex = lessonData.items.findIndex((item) => item.seq === currentItemSeq);
    if (currentIndex >= 0 && currentIndex < lessonData.items.length - 1) {
      return lessonData.items[currentIndex + 1];
    }
    return null;
  }, [isInLessonContext, lessonData, currentItemSeq]);

  const isLastLessonItem = useMemo(() => {
    if (!isInLessonContext || !totalItems || !currentItemSeq) return false;
    return currentItemSeq >= totalItems;
  }, [isInLessonContext, totalItems, currentItemSeq]);

  const sendProgressUpdate = useCallback(
    (progressRate: number) => {
      if (!isLoggedIn || !isValidId) {
        return;
      }

      const progress_rate = clampProgressRate(progressRate);
      console.log("[VideoProgress] Updating:", { progress_rate });
      updateVideoProgress({ progress_rate });
    },
    [isLoggedIn, isValidId, updateVideoProgress]
  );

  const handlePause = useCallback(
    ({ seconds, duration }: { seconds: number; duration: number }) => {
      if (!isLoggedIn || !isValidId) {
        return;
      }

      if (!Number.isFinite(duration) || duration <= 0) {
        return;
      }

      const progressRate = (seconds / duration) * 100;

      if (pauseTimeoutRef.current) {
        clearTimeout(pauseTimeoutRef.current);
      }

      pauseTimeoutRef.current = setTimeout(() => {
        sendProgressUpdate(progressRate);
      }, 500);
    },
    [isLoggedIn, isValidId, sendProgressUpdate]
  );

  const handleEnded = useCallback(() => {
    if (pauseTimeoutRef.current) {
      clearTimeout(pauseTimeoutRef.current);
      pauseTimeoutRef.current = null;
    }

    sendProgressUpdate(100);
    setIsVideoEnded(true);
  }, [sendProgressUpdate]);

  useEffect(() => {
    if (!isValidId) {
      navigate("/videos", { replace: true });
    }
  }, [isValidId, navigate]);

  useEffect(() => {
    if (!isProgressSuccess || !progressData) {
      return;
    }

    console.log("[VideoProgress] Loaded:", {
      progress_rate: progressData.progress_rate,
      is_completed: progressData.is_completed,
    });
  }, [isProgressSuccess, progressData]);

  useEffect(() => {
    return () => {
      if (pauseTimeoutRef.current) {
        clearTimeout(pauseTimeoutRef.current);
        pauseTimeoutRef.current = null;
      }
    };
  }, []);

  if (!isValidId) return null;

  if (isPending) {
    return (
      <div className="min-h-screen">
        <div className="max-w-[1000px] mx-auto space-y-6 px-6 lg:px-8 py-10">
          <Skeleton className="aspect-video w-full rounded-2xl" />
          <div className="space-y-3">
            <Skeleton className="h-8 w-2/3" />
            <Skeleton className="h-4 w-1/2" />
            <div className="flex gap-2">
              <Skeleton className="h-6 w-16 rounded-full" />
              <Skeleton className="h-6 w-16 rounded-full" />
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (isError || !data) {
    const isNotFound = error instanceof ApiError && error.status === 404;
    return (
      <div className="min-h-screen flex items-center justify-center p-4">
        <Card className="w-full max-w-md text-center shadow-card border-0">
          <CardHeader className="pb-4">
            <div className="w-16 h-16 rounded-2xl bg-muted flex items-center justify-center mx-auto mb-4">
              <span className="text-3xl">😕</span>
            </div>
            <CardTitle className="text-xl">
              {isNotFound ? "영상을 찾을 수 없습니다" : "오류 발생"}
            </CardTitle>
            <p className="text-sm text-muted-foreground">
              {isNotFound ? "존재하지 않거나 삭제된 영상입니다." : "일시적인 오류입니다. 다시 시도해주세요."}
            </p>
          </CardHeader>
          <CardContent>
            <Button asChild className="gradient-primary text-white rounded-full">
              <Link to={lessonId ? `/lessons/${lessonId}` : "/videos"}>
                {lessonId ? "수업으로 돌아가기" : "목록으로 돌아가기"}
              </Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  const tagLabels = data.tags
    .map(getTagLabel)
    .filter((label): label is string => Boolean(label));

  return (
    <div className="min-h-screen py-8 lg:py-12">
      <div className="max-w-[1000px] mx-auto space-y-8 px-6 lg:px-8">
        {/* Back Link */}
        <Link
          to={isInLessonContext ? `/lessons/${lessonId}` : "/videos"}
          className="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-primary transition-colors"
        >
          <ArrowLeft className="h-4 w-4" />
          {isInLessonContext ? "수업으로" : "목록으로"}
        </Link>

        {/* Video Player */}
        <div className="rounded-2xl overflow-hidden shadow-card">
          <VideoPlayer
            url={data.video_url_vimeo}
            onPause={handlePause}
            onEnded={handleEnded}
          />
        </div>

        {/* Video Info */}
        <div className="space-y-4">
          <div className="flex flex-wrap items-center gap-2">
            <Badge className="gradient-primary text-white border-0 uppercase">
              {data.video_state}
            </Badge>
            <Badge variant="outline" className="gap-1">
              <Calendar className="h-3 w-3" />
              {formatDate(data.created_at)}
            </Badge>
          </div>

          <div className="space-y-2">
            <h1 className="text-2xl md:text-3xl font-bold tracking-tight">
              {data.title ?? "제목 없음"}
            </h1>
            {data.subtitle && (
              <p className="text-lg text-muted-foreground">{data.subtitle}</p>
            )}
          </div>

          {tagLabels.length > 0 && (
            <div className="flex flex-wrap gap-2 pt-2">
              {tagLabels.map((label, index) => (
                <Badge
                  key={`${label}-${index}`}
                  variant="secondary"
                  className="px-3 py-1 rounded-full"
                >
                  #{label}
                </Badge>
              ))}
            </div>
          )}
        </div>

        {/* Completion Card */}
        {isVideoEnded && (
          <Card className="border-0 bg-gradient-to-br from-green-50 to-emerald-50 shadow-card">
            <CardContent className="p-8 text-center space-y-6">
              <div className="w-16 h-16 rounded-full bg-green-100 flex items-center justify-center mx-auto">
                <CheckCircle2 className="h-8 w-8 text-green-600" />
              </div>
              <div>
                <h2 className="text-xl font-bold text-green-700 mb-2">
                  영상 시청을 완료했습니다!
                </h2>
                <p className="text-sm text-green-600/80">
                  수고하셨습니다. 학습을 계속해보세요.
                </p>
              </div>

              {isInLessonContext ? (
                <div className="flex flex-col gap-3">
                  {isLastLessonItem ? (
                    <>
                      <p className="text-sm text-muted-foreground">
                        "{lessonData?.title ?? "수업"}"의 모든 항목을 완료했습니다!
                      </p>
                      <Button
                        className="gradient-primary text-white rounded-full"
                        onClick={() => {
                          if (isLoggedIn && lessonId && currentItemSeq) {
                            updateLessonProgress.mutate({
                              percent: 100,
                              last_seq: currentItemSeq,
                            });
                          }
                          navigate(`/lessons/${lessonId}`);
                        }}
                      >
                        수업 완료하기
                      </Button>
                    </>
                  ) : nextLessonItem ? (
                    <>
                      <Button
                        asChild
                        className="gradient-primary text-white rounded-full"
                        onClick={() => {
                          if (isLoggedIn && lessonId && currentItemSeq && totalItems) {
                            const percent = Math.floor((currentItemSeq / totalItems) * 100);
                            updateLessonProgress.mutate({
                              percent,
                              last_seq: currentItemSeq,
                            });
                          }
                        }}
                      >
                        <Link
                          to={
                            nextLessonItem.kind === "video" && nextLessonItem.video_id
                              ? `/videos/${nextLessonItem.video_id}?lessonId=${lessonId}&itemSeq=${nextLessonItem.seq}&totalItems=${totalItems}`
                              : nextLessonItem.kind === "task" && nextLessonItem.task_id
                                ? `/studies/tasks/${nextLessonItem.task_id}?lessonId=${lessonId}&itemSeq=${nextLessonItem.seq}&totalItems=${totalItems}`
                                : `/lessons/${lessonId}`
                          }
                        >
                          다음 항목으로
                          <ArrowRight className="ml-2 h-4 w-4" />
                        </Link>
                      </Button>
                      <Button variant="outline" asChild className="rounded-full">
                        <Link to={`/lessons/${lessonId}`}>수업으로 돌아가기</Link>
                      </Button>
                    </>
                  ) : (
                    <Button asChild className="gradient-primary text-white rounded-full">
                      <Link to={`/lessons/${lessonId}`}>수업으로 돌아가기</Link>
                    </Button>
                  )}
                </div>
              ) : (
                <Button asChild className="gradient-primary text-white rounded-full">
                  <Link to="/videos">목록으로 돌아가기</Link>
                </Button>
              )}
            </CardContent>
          </Card>
        )}
      </div>
    </div>
  );
}
