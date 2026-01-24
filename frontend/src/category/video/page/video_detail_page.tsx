import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";

import { ApiError } from "@/api/client";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import type { VideoTag } from "@/category/video/types";
import { useAuthStore } from "@/hooks/use_auth_store";

import { VideoPlayer } from "../components/video_player";
import { useVideoDetail } from "../hook/use_video_detail";
import { useUpdateVideoProgress, useVideoProgress } from "../hook/use_video_progress";

// 날짜 포맷팅 함수
const formatDate = (value: string) => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;
  
  return date.toLocaleDateString("ko-KR", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  });
};

// 태그 라벨 추출 함수
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
  const navigate = useNavigate();
  const isLoggedIn = useAuthStore((state) => state.isLoggedIn);
  const pauseTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [isVideoEnded, setIsVideoEnded] = useState(false);

  const id = useMemo(() => Number(videoId), [videoId]);
  const isValidId = Number.isFinite(id);

  // 데이터 조회 Hook
  const { data, isPending, isError, error } = useVideoDetail(id);
  const { data: progressData, isSuccess: isProgressSuccess } = useVideoProgress(
    isValidId ? id : undefined
  );
  const { mutate: updateVideoProgress } = useUpdateVideoProgress(id);

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

  // 유효하지 않은 ID 접근 시 리다이렉트
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

  // 1. 로딩 상태 (Skeleton)
  if (isPending) {
    return (
      <div className="min-h-screen bg-muted/30">
        <div className="mx-auto w-full max-w-screen-lg space-y-6 px-4 py-10">
          <Skeleton className="aspect-video w-full rounded-lg" />
          <div className="space-y-3">
            <Skeleton className="h-8 w-2/3" />
            <Skeleton className="h-4 w-1/2" />
            <div className="flex gap-2">
              <Skeleton className="h-6 w-16" />
              <Skeleton className="h-6 w-16" />
            </div>
          </div>
        </div>
      </div>
    );
  }

  // 2. 에러 상태 (404 etc)
  if (isError || !data) {
    const isNotFound = error instanceof ApiError && error.status === 404;
    return (
      <div className="min-h-screen bg-muted/30 flex items-center justify-center p-4">
        <Card className="w-full max-w-md text-center">
          <CardHeader>
            <CardTitle>{isNotFound ? "영상을 찾을 수 없습니다." : "오류 발생"}</CardTitle>
            <p className="text-sm text-muted-foreground">
              {isNotFound ? "존재하지 않거나 삭제된 영상입니다." : "일시적인 오류입니다. 다시 시도해주세요."}
            </p>
          </CardHeader>
          <CardContent>
            <Button asChild>
              <Link to="/videos">목록으로 돌아가기</Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  // 3. 정상 렌더링
  const tagLabels = data.tags
    .map(getTagLabel)
    .filter((label): label is string => Boolean(label));

  return (
    <div className="min-h-screen bg-muted/30">
      <div className="mx-auto w-full max-w-screen-lg space-y-8 px-4 py-10">
        <Link
          to="/videos"
          className="text-sm text-muted-foreground hover:text-foreground transition inline-block"
        >
          &larr; 목록으로
        </Link>

        {/* ✅ 실제 DB 데이터 연결 (더 이상 하드코딩 아님) */}
        <VideoPlayer
          url={data.video_url_vimeo}
          onPause={handlePause}
          onEnded={handleEnded}
        />

        <div className="space-y-4">
          {/* 메타 정보 */}
          <div className="flex flex-wrap items-center gap-2">
            <Badge variant="secondary" className="uppercase">{data.video_state}</Badge>
            <Badge variant="outline">{formatDate(data.created_at)}</Badge>
          </div>

          {/* 제목 및 설명 */}
          <div className="space-y-2">
            <h1 className="text-2xl font-bold tracking-tight md:text-3xl">
              {data.title ?? "제목 없음"}
            </h1>
            {data.subtitle && (
              <p className="text-lg text-muted-foreground">{data.subtitle}</p>
            )}
          </div>

          {/* 태그 목록 */}
          {tagLabels.length > 0 && (
            <div className="flex flex-wrap gap-2 pt-2">
              {tagLabels.map((label, index) => (
                <Badge key={`${label}-${index}`} variant="outline" className="px-3 py-1">
                  #{label}
                </Badge>
              ))}
            </div>
          )}
        </div>

        {/* 시청 완료 메시지 */}
        {isVideoEnded && (
          <Card className="border-green-500 bg-green-50">
            <CardContent className="p-6 text-center space-y-4">
              <div className="text-4xl">🎉</div>
              <h2 className="text-xl font-bold text-green-700">
                영상 시청을 완료했습니다!
              </h2>
              <Button asChild>
                <Link to="/videos">목록으로 돌아가기</Link>
              </Button>
            </CardContent>
          </Card>
        )}
      </div>
    </div>
  );
}
