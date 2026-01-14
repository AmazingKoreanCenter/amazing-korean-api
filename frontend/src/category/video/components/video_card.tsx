import { Link } from "react-router-dom";

import { AspectRatio } from "@/components/ui/aspect-ratio";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import type { VideoListItem } from "@/category/video/types";

// 날짜 포맷팅 함수
const formatDate = (value: string) => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }

  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}.${month}.${day}`;
};

// 초(Seconds)를 "MM:SS" 형식으로 변환하는 함수
const formatDuration = (seconds: number | null) => {
  if (seconds === null) return "00:00";
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
};

type VideoCardProps = {
  video: VideoListItem;
};

export function VideoCard({ video }: VideoCardProps) {
  return (
    <Card className="h-full overflow-hidden transition-all hover:shadow-lg">
      <Link
        to={`/videos/${video.video_id}`}
        className="flex h-full flex-col focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      >
        <CardHeader className="p-0">
          <AspectRatio
            ratio={16 / 9}
            className="bg-muted text-muted-foreground"
          >
            <img
              src={
                video.thumbnail_url ??
                "https://placehold.co/600x400?text=No+Thumbnail"
              }
              alt={video.title ?? "Video Thumbnail"}
              className="h-full w-full object-cover transition-transform hover:scale-105"
            />
          </AspectRatio>
        </CardHeader>
        
        <CardContent className="flex flex-1 flex-col gap-3 p-4">
          <div className="space-y-1">
            <h3 className="line-clamp-1 text-base font-semibold text-foreground">
              {video.title ?? "제목 없음"}
            </h3>
            
            {/* description 대신 subtitle 사용 */}
            {video.subtitle && (
              <p className="line-clamp-2 text-sm text-muted-foreground">
                {video.subtitle}
              </p>
            )}
          </div>

          <div className="flex flex-wrap gap-2 text-xs">
            {/* duration(string) 대신 duration_seconds(number) 변환 */}
            <Badge variant="secondary">
              ⏱ {formatDuration(video.duration_seconds)}
            </Badge>
            
            {/* view_count 대신 language 표시 */}
            {video.language && (
              <Badge variant="outline">
                🗣 {video.language.toUpperCase()}
              </Badge>
            )}

            {/* 자막 보유 여부 표시 */}
            {video.has_captions && (
              <Badge variant="outline" className="border-primary text-primary">
                CC
              </Badge>
            )}
          </div>

          <div className="flex items-center justify-between text-xs text-muted-foreground mt-auto pt-2">
            {/* uploader_name이 없으므로 제거 (필요시 state 표시) */}
            <span className="font-medium text-foreground/80">
               {/* 공란 유지 or video.state 표시 가능 */}
            </span>
            <span>{formatDate(video.created_at)}</span>
          </div>
        </CardContent>
      </Link>
    </Card>
  );
}