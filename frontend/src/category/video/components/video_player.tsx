import { useEffect, useRef } from "react";
import Player from "@vimeo/player";
import { AspectRatio } from "@/components/ui/aspect-ratio";

type VideoPlayerProps = {
  url: string;
  onEnded?: () => void;
};

export function VideoPlayer({ url, onEnded }: VideoPlayerProps) {
  // 1. iframe 요소에 접근하기 위한 Hook
  const iframeRef = useRef<HTMLIFrameElement>(null);

  // 2. URL에서 ID 추출 (이전 로직 유지)
  const getVideoId = (videoUrl: string) => {
    const match = videoUrl.match(/vimeo\.com\/(?:.*\/)?(\d+)/);
    return match ? match[1] : null;
  };

  const videoId = getVideoId(url);

  // 3. Vimeo SDK 연결 (이벤트 감지용)
  useEffect(() => {
    if (!iframeRef.current || !onEnded) return;

    // iframe을 Vimeo Player 객체로 감싸서 제어권을 얻습니다.
    const player = new Player(iframeRef.current);

    // 'ended' 이벤트가 발생하면 우리가 받은 onEnded 함수를 실행합니다.
    player.on("ended", () => {
      onEnded();
    });

    // 뒷정리 (Unmount 시 이벤트 해제)
    return () => {
      player.off("ended");
    };
  }, [onEnded]); // url이 바뀌면 iframe이 새로 그려지므로 deps 제외 가능

  if (!videoId) return null;

  return (
    <div className="w-full overflow-hidden rounded-lg border bg-black shadow-lg">
      <AspectRatio ratio={16 / 9}>
        <iframe
          ref={iframeRef}
          src={`https://player.vimeo.com/video/${videoId}?badge=0&autopause=0&player_id=0&app_id=58479`}
          className="absolute top-0 left-0 w-full h-full"
          allow="autoplay; fullscreen; picture-in-picture; clipboard-write"
          title="vimeo-player"
        />
      </AspectRatio>
    </div>
  );
}