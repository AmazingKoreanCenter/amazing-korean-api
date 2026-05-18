import { useState } from "react";
import { ImageOff } from "lucide-react";

interface CoverCardProps {
  imageSrc: string;
  imageAlt: string;
  title: string;
  subtitle: string;
  actionLabel: string;
  onClick: () => void;
}

export function CoverCard({
  imageSrc,
  imageAlt,
  title,
  subtitle,
  actionLabel,
  onClick,
}: CoverCardProps) {
  // 표지 webp 미존재(예: books 파이프라인 표지 도착 전 신규 언어) 시
  // 깨진 이미지 대신 플레이스홀더. 향후 언어 추가에도 재발 방어.
  const [imgError, setImgError] = useState(false);
  return (
    <button
      type="button"
      onClick={onClick}
      className="bg-card rounded-2xl overflow-hidden shadow-card hover:shadow-card-hover hover:-translate-y-1 transition-all duration-300 border hover:border-accent/50 text-start cursor-pointer"
    >
      <div className="aspect-[3/4] overflow-hidden bg-muted border-b">
        {imgError ? (
          <div
            className="w-full h-full flex items-center justify-center text-muted-foreground"
            role="img"
            aria-label={imageAlt}
          >
            <ImageOff className="h-10 w-10" />
          </div>
        ) : (
          <img
            src={imageSrc}
            alt={imageAlt}
            className="w-full h-full object-cover"
            loading="lazy"
            onError={() => setImgError(true)}
          />
        )}
      </div>
      <div className="p-4 space-y-2">
        <h3 className="font-semibold text-sm">{title}</h3>
        <p className="text-xs text-muted-foreground text-end py-0.5">{subtitle}</p>
        <span className="inline-flex items-center justify-center w-full rounded-md bg-primary text-primary-foreground text-sm font-medium h-8 px-3">
          {actionLabel}
        </span>
      </div>
    </button>
  );
}
