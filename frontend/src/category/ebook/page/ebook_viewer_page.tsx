import { useCallback, useEffect, useRef, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import {
  ChevronLeft,
  ChevronRight,
  List,
  Maximize2,
  Minimize2,
  X,
  ZoomIn,
  ZoomOut,
  Loader2,
  BookOpen,
  FileText,
  Lock,
  ShieldX,
} from "lucide-react";

import { ApiError } from "@/api/client";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";

import { useViewerMeta } from "../hook/use_viewer_meta";
import { usePageImage } from "../hook/use_page_image";

const ZOOM_LEVELS = [50, 75, 100, 120, 150];
const DEFAULT_ZOOM_INDEX = 2; // 100%

type ViewMode = "single" | "spread";

/** ArrayBuffer → Canvas 렌더링 (blob URL 미노출, 즉시 revoke) */
function PageCanvas({
  data,
  className,
  style,
}: {
  data: ArrayBuffer;
  className?: string;
  style?: React.CSSProperties;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!data || !canvasRef.current) return;

    const blob = new Blob([data], { type: "image/webp" });
    const url = URL.createObjectURL(blob);
    const img = new Image();

    img.onload = () => {
      const canvas = canvasRef.current;
      if (canvas) {
        canvas.width = img.naturalWidth;
        canvas.height = img.naturalHeight;
        const ctx = canvas.getContext("2d");
        ctx?.drawImage(img, 0, 0);
      }
      URL.revokeObjectURL(url);
    };

    img.src = url;
  }, [data]);

  return (
    <canvas
      ref={canvasRef}
      className={className}
      style={{ ...style, userSelect: "none" }}
      onContextMenu={(e) => e.preventDefault()}
      draggable={false}
      onDragStart={(e) => e.preventDefault()}
    />
  );
}

export function EbookViewerPage() {
  const { purchaseCode } = useParams<{ purchaseCode: string }>();
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [currentPage, setCurrentPage] = useState(1);
  const [sliderPage, setSliderPage] = useState(1);
  const [isDragging, setIsDragging] = useState(false);
  const [zoomIndex, setZoomIndex] = useState(DEFAULT_ZOOM_INDEX);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [tocOpen, setTocOpen] = useState(false);
  const [viewMode, setViewMode] = useState<ViewMode>("single");
  const [controlsVisible, setControlsVisible] = useState(true);
  const hideTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const { data: meta, isLoading: metaLoading, error: metaError } = useViewerMeta(
    purchaseCode ?? ""
  );
  const totalPages = meta?.total_pages ?? 0;

  // 두 쪽 보기 시 오른쪽 페이지 번호 (1페이지는 단독 표시 — 표지)
  const spreadRightPage =
    viewMode === "spread" && currentPage > 1 && currentPage < totalPages
      ? currentPage + 1
      : null;

  const { data: imageData, isLoading: imageLoading } = usePageImage(
    purchaseCode ?? "",
    currentPage,
    totalPages,
    !!meta,
    viewMode
  );

  const { data: imageDataRight, isLoading: imageLoadingRight } = usePageImage(
    purchaseCode ?? "",
    spreadRightPage ?? 0,
    totalPages,
    !!meta && spreadRightPage !== null,
    viewMode
  );

  const zoom = ZOOM_LEVELS[zoomIndex];

  const goToPrevPage = useCallback(() => {
    if (viewMode === "spread") {
      setCurrentPage((p) => {
        // 3페이지 이하에서 뒤로 → 1페이지 (표지 단독)
        if (p <= 3) return 1;
        return Math.max(1, p - 2);
      });
    } else {
      setCurrentPage((p) => Math.max(1, p - 1));
    }
  }, [viewMode]);

  const goToNextPage = useCallback(() => {
    if (viewMode === "spread") {
      setCurrentPage((p) => {
        // 1페이지(표지 단독) → 2페이지로 이동
        if (p === 1) return 2;
        return Math.min(totalPages, p + 2);
      });
    } else {
      setCurrentPage((p) => Math.min(totalPages, p + 1));
    }
  }, [totalPages, viewMode]);

  // 키보드 네비게이션
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === "ArrowLeft") goToPrevPage();
      else if (e.key === "ArrowRight") goToNextPage();
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [goToPrevPage, goToNextPage]);

  // 이미지 영역 클릭 (좌=이전, 우=다음)
  const handleViewerClick = useCallback(
    (e: React.MouseEvent<HTMLDivElement>) => {
      if ((e.target as HTMLElement).closest("button, input, a")) return;
      const rect = e.currentTarget.getBoundingClientRect();
      const clickX = e.clientX - rect.left;
      if (clickX < rect.width / 2) {
        goToPrevPage();
      } else {
        goToNextPage();
      }
    },
    [goToPrevPage, goToNextPage]
  );

  // ─── 풀스크린 ───
  const toggleFullscreen = useCallback(() => {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen().catch(() => {});
    } else {
      document.exitFullscreen().catch(() => {});
    }
  }, []);

  useEffect(() => {
    const handler = () => {
      const fs = !!document.fullscreenElement;
      setIsFullscreen(fs);
      if (fs) {
        // 풀스크린 진입 시 컨트롤 표시 후 3초 뒤 숨김
        setControlsVisible(true);
        resetHideTimer();
      } else {
        setControlsVisible(true);
      }
    };
    document.addEventListener("fullscreenchange", handler);
    return () => document.removeEventListener("fullscreenchange", handler);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // 풀스크린 시 마우스 움직임 → 컨트롤 표시 + 3초 후 숨김
  const resetHideTimer = useCallback(() => {
    if (hideTimerRef.current) clearTimeout(hideTimerRef.current);
    hideTimerRef.current = setTimeout(() => {
      if (document.fullscreenElement) setControlsVisible(false);
    }, 3000);
  }, []);

  const handleMouseMove = useCallback(() => {
    if (!isFullscreen) return;
    setControlsVisible(true);
    resetHideTimer();
  }, [isFullscreen, resetHideTimer]);

  useEffect(() => {
    return () => {
      if (hideTimerRef.current) clearTimeout(hideTimerRef.current);
    };
  }, []);

  // ─── 에러 상태 ───
  if (metaLoading) {
    return (
      <div className="flex items-center justify-center h-screen">
        <Loader2 className="w-8 h-8 animate-spin text-muted-foreground" />
      </div>
    );
  }

  if (metaError) {
    const apiErr = metaError instanceof ApiError ? metaError : null;
    const status = apiErr?.status ?? 0;

    if (status === 403) {
      return (
        <div className="flex flex-col items-center justify-center h-screen gap-4 text-center px-4">
          <Lock className="w-12 h-12 text-amber-500" />
          <h2 className="text-lg font-semibold">{t("ebook.viewer.paymentRequired")}</h2>
          <p className="text-muted-foreground text-sm max-w-sm">
            {t("ebook.viewer.paymentRequiredDesc")}
          </p>
          <Button variant="outline" onClick={() => navigate("/ebook/my")}>
            {t("ebook.viewer.goToMyEbooks")}
          </Button>
        </div>
      );
    }

    return (
      <div className="flex flex-col items-center justify-center h-screen gap-4 text-center px-4">
        <ShieldX className="w-12 h-12 text-destructive" />
        <h2 className="text-lg font-semibold">{t("ebook.viewer.notFound")}</h2>
        <p className="text-muted-foreground text-sm max-w-sm">
          {t("ebook.viewer.notFoundDesc")}
        </p>
        <Button variant="outline" onClick={() => navigate("/ebook/my")}>
          {t("ebook.viewer.goToMyEbooks")}
        </Button>
      </div>
    );
  }

  if (!meta) {
    return (
      <div className="flex flex-col items-center justify-center h-screen gap-4 text-center px-4">
        <ShieldX className="w-12 h-12 text-destructive" />
        <h2 className="text-lg font-semibold">{t("ebook.viewer.notFound")}</h2>
        <Button variant="outline" onClick={() => navigate("/ebook/my")}>
          {t("ebook.viewer.goToMyEbooks")}
        </Button>
      </div>
    );
  }

  // ─── 렌더 ───
  const displayPage = isDragging ? sliderPage : currentPage;
  const pageDisplay =
    viewMode === "spread" && spreadRightPage !== null
      ? `${displayPage}-${spreadRightPage} / ${totalPages}`
      : `${displayPage} / ${totalPages}`;

  const isLoading = imageLoading || (viewMode === "spread" && imageLoadingRight);
  const hasImage = !!imageData;

  return (
    <>
      {/* 풀스크린 시 사이트 전체 숨기고 뷰어만 표시 */}
      <style>{`
        @media print { .ebook-viewer { display: none !important; } }
        :fullscreen .site-header,
        :fullscreen .site-nav,
        :fullscreen .site-footer,
        :fullscreen > body > *:not(.ebook-viewer-fs) { }
      `}</style>

      <div
        className={`ebook-viewer flex flex-col select-none ${
          isFullscreen
            ? "fixed inset-0 z-[9999] bg-neutral-900"
            : "h-screen bg-background"
        }`}
        onContextMenu={(e) => e.preventDefault()}
        onMouseMove={handleMouseMove}
        style={{ WebkitTouchCallout: "none" } as React.CSSProperties}
      >
        {/* ─── 상단 바 ─── */}
        <div
          className={`flex items-center justify-between px-4 py-2 border-b backdrop-blur z-20 transition-all duration-300 ${
            isFullscreen
              ? `bg-neutral-900/90 border-neutral-700 text-white ${
                  controlsVisible ? "opacity-100 translate-y-0" : "opacity-0 -translate-y-full pointer-events-none"
                }`
              : "bg-background/95 border-border"
          }`}
        >
          <div className="flex items-center gap-2">
            <Button
              variant="ghost"
              size="icon"
              onClick={() => setTocOpen(true)}
              className={isFullscreen ? "text-white hover:bg-white/10" : ""}
            >
              <List className="w-5 h-5" />
            </Button>
            <span className={`text-sm ${isFullscreen ? "text-neutral-300" : "text-muted-foreground"}`}>
              {pageDisplay}
            </span>
          </div>

          <div className="flex items-center gap-1">
            {/* 한 쪽 / 두 쪽 보기 */}
            <Button
              variant="ghost"
              size="icon"
              onClick={() => setViewMode(viewMode === "single" ? "spread" : "single")}
              title={viewMode === "single" ? t("ebook.viewer.spreadView") : t("ebook.viewer.singleView")}
              className={isFullscreen ? "text-white hover:bg-white/10" : ""}
            >
              {viewMode === "single" ? <BookOpen className="w-4 h-4" /> : <FileText className="w-4 h-4" />}
            </Button>

            {/* 줌 */}
            <Button
              variant="ghost"
              size="icon"
              onClick={() => setZoomIndex((i) => Math.max(0, i - 1))}
              disabled={zoomIndex === 0}
              className={isFullscreen ? "text-white hover:bg-white/10" : ""}
            >
              <ZoomOut className="w-4 h-4" />
            </Button>
            <span className={`text-xs w-10 text-center ${isFullscreen ? "text-neutral-300" : "text-muted-foreground"}`}>
              {zoom}%
            </span>
            <Button
              variant="ghost"
              size="icon"
              onClick={() => setZoomIndex((i) => Math.min(ZOOM_LEVELS.length - 1, i + 1))}
              disabled={zoomIndex === ZOOM_LEVELS.length - 1}
              className={isFullscreen ? "text-white hover:bg-white/10" : ""}
            >
              <ZoomIn className="w-4 h-4" />
            </Button>

            <Button
              variant="ghost"
              size="icon"
              onClick={toggleFullscreen}
              className={isFullscreen ? "text-white hover:bg-white/10" : ""}
            >
              {isFullscreen ? <Minimize2 className="w-4 h-4" /> : <Maximize2 className="w-4 h-4" />}
            </Button>
          </div>
        </div>

        {/* ─── TOC 사이드바 ─── */}
        {tocOpen && (
          <>
            <div className="fixed inset-0 z-40 bg-black/50" onClick={() => setTocOpen(false)} />
            <div className="fixed left-0 top-0 z-50 h-full w-72 bg-background border-r shadow-lg overflow-y-auto">
              <div className="flex items-center justify-between px-4 py-3 border-b">
                <h2 className="text-lg font-semibold">{t("ebook.viewer.toc")}</h2>
                <Button variant="ghost" size="icon" onClick={() => setTocOpen(false)}>
                  <X className="w-4 h-4" />
                </Button>
              </div>
              <div className="p-2 space-y-1">
                {meta.toc.map((entry, i) => {
                  const nextPage = meta.toc[i + 1]?.page ?? Infinity;
                  const isActive = currentPage >= entry.page && currentPage < nextPage;
                  return (
                    <button
                      key={i}
                      className={`w-full text-left px-3 py-2 rounded-md text-sm transition-colors ${
                        isActive ? "bg-accent text-accent-foreground" : "hover:bg-muted"
                      }`}
                      onClick={() => {
                        setCurrentPage(entry.page);
                        setTocOpen(false);
                      }}
                    >
                      <span className="text-muted-foreground mr-2 tabular-nums">p.{entry.page}</span>
                      <span className="flex flex-col leading-tight">
                        <span>{entry.title_ko}</span>
                        <span className="text-xs text-muted-foreground">{entry.title}</span>
                      </span>
                    </button>
                  );
                })}
              </div>
            </div>
          </>
        )}

        {/* ─── 이미지 영역 ─── */}
        <div
          className={`flex-1 overflow-auto flex items-center justify-center cursor-pointer ${
            isFullscreen ? "bg-neutral-900" : "bg-neutral-100 dark:bg-neutral-800"
          }`}
          onClick={handleViewerClick}
        >
          {isLoading ? (
            <div className="flex gap-2 p-4">
              <Skeleton className="w-[380px] h-[537px] rounded" />
              {viewMode === "spread" && <Skeleton className="w-[380px] h-[537px] rounded" />}
            </div>
          ) : hasImage ? (
            <div
              className="flex gap-1 p-4"
              style={{
                transform: `scale(${zoom / 100})`,
                transformOrigin: "center center",
              }}
            >
              <PageCanvas
                data={imageData}
                className="shadow-2xl rounded-sm"
                style={{
                  maxHeight: isFullscreen ? "calc(100vh - 100px)" : "calc(100vh - 140px)",
                  maxWidth: viewMode === "spread" ? "45vw" : "90vw",
                  width: "auto",
                  height: "auto",
                  pointerEvents: "none",
                }}
              />
              {viewMode === "spread" && imageDataRight && (
                <PageCanvas
                  data={imageDataRight}
                  className="shadow-2xl rounded-sm"
                  style={{
                    maxHeight: isFullscreen ? "calc(100vh - 100px)" : "calc(100vh - 140px)",
                    maxWidth: "45vw",
                    width: "auto",
                    height: "auto",
                    pointerEvents: "none",
                  }}
                />
              )}
            </div>
          ) : (
            <div className="text-muted-foreground">{t("ebook.viewer.loadFailed")}</div>
          )}
        </div>

        {/* ─── 하단 네비게이션 ─── */}
        <div
          className={`flex items-center justify-center gap-4 px-4 py-3 border-t backdrop-blur z-20 transition-all duration-300 ${
            isFullscreen
              ? `bg-neutral-900/90 border-neutral-700 ${
                  controlsVisible ? "opacity-100 translate-y-0" : "opacity-0 translate-y-full pointer-events-none"
                }`
              : "bg-background/95 border-border"
          }`}
        >
          <Button
            variant="outline"
            size="icon"
            onClick={goToPrevPage}
            disabled={currentPage <= 1}
            className={isFullscreen ? "border-neutral-600 text-white hover:bg-white/10" : ""}
          >
            <ChevronLeft className="w-5 h-5" />
          </Button>

          <input
            type="range"
            min={1}
            max={totalPages}
            value={isDragging ? sliderPage : currentPage}
            onChange={(e) => {
              setSliderPage(Number(e.target.value));
              setIsDragging(true);
            }}
            onMouseUp={() => {
              setCurrentPage(sliderPage);
              setIsDragging(false);
            }}
            onTouchEnd={() => {
              setCurrentPage(sliderPage);
              setIsDragging(false);
            }}
            className="w-48 sm:w-64 accent-primary"
          />

          <Button
            variant="outline"
            size="icon"
            onClick={goToNextPage}
            disabled={currentPage >= totalPages}
            className={isFullscreen ? "border-neutral-600 text-white hover:bg-white/10" : ""}
          >
            <ChevronRight className="w-5 h-5" />
          </Button>
        </div>
      </div>
    </>
  );
}
