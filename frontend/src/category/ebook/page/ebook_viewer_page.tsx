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
  ShieldCheck,
} from "lucide-react";

import { ApiError } from "@/api/client";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";

import { useViewerMeta } from "../hook/use_viewer_meta";
import { usePageImage, usePageTiles } from "../hook/use_page_image";
import { sendViewerHeartbeat } from "../ebook_api";
import { useDevToolsDetection } from "../utils/devtools_detect";

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

/** 타일 분할 Canvas: gridRows×gridCols 타일을 하나의 Canvas에 조립 */
function TiledPageCanvas({
  tiles,
  gridRows,
  gridCols,
  className,
  style,
}: {
  tiles: Array<ArrayBuffer | undefined>;
  gridRows: number;
  gridCols: number;
  className?: string;
  style?: React.CSSProperties;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || tiles.some((t) => !t)) return;

    // 모든 타일을 Image로 디코딩
    let cancelled = false;
    const images: HTMLImageElement[] = [];
    const urls: string[] = [];

    Promise.allSettled(
      tiles.map((buf) => {
        const blob = new Blob([buf!], { type: "image/webp" });
        const url = URL.createObjectURL(blob);
        urls.push(url);
        return new Promise<HTMLImageElement>((resolve, reject) => {
          const img = new Image();
          img.onload = () => resolve(img);
          img.onerror = reject;
          img.src = url;
        });
      })
    ).then((results) => {
      if (cancelled) return;

      // 실패한 타일은 1x1 빈 이미지로 대체
      const imgs = results.map((r) => {
        if (r.status === "fulfilled") return r.value;
        const fallback = new Image();
        fallback.width = 1;
        fallback.height = 1;
        return fallback;
      });
      images.push(...imgs);

      // 캔버스 크기 계산 (타일 크기 합산)
      let totalW = 0;
      let totalH = 0;
      for (let r = 0; r < gridRows; r++) {
        let rowH = 0;
        let rowW = 0;
        for (let c = 0; c < gridCols; c++) {
          const idx = r * gridCols + c;
          rowW += imgs[idx].naturalWidth;
          rowH = Math.max(rowH, imgs[idx].naturalHeight);
        }
        totalW = Math.max(totalW, rowW);
        totalH += rowH;
      }

      canvas.width = totalW;
      canvas.height = totalH;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;
      ctx.imageSmoothingEnabled = false;

      // 타일 배치
      let yOff = 0;
      for (let r = 0; r < gridRows; r++) {
        let xOff = 0;
        let rowH = 0;
        for (let c = 0; c < gridCols; c++) {
          const idx = r * gridCols + c;
          ctx.drawImage(imgs[idx], xOff, yOff);
          xOff += imgs[idx].naturalWidth;
          rowH = Math.max(rowH, imgs[idx].naturalHeight);
        }
        yOff += rowH;
      }
    }).finally(() => {
      urls.forEach((u) => URL.revokeObjectURL(u));
    });

    return () => {
      cancelled = true;
      urls.forEach((u) => URL.revokeObjectURL(u));
    };
  }, [tiles, gridRows, gridCols]);

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
  const [isObscured, setIsObscured] = useState(false);
  const [isMobile, setIsMobile] = useState(() => window.innerWidth < 768);
  const [showCopyrightNotice, setShowCopyrightNotice] = useState(
    () => !sessionStorage.getItem("ebook_copyright_ack")
  );
  const hideTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const viewerRef = useRef<HTMLDivElement>(null);
  const touchStartRef = useRef<{ x: number; y: number } | null>(null);

  // ─── Step 1: Canvas 추출 API 무력화 ───
  useEffect(() => {
    const orig = {
      toDataURL: HTMLCanvasElement.prototype.toDataURL,
      toBlob: HTMLCanvasElement.prototype.toBlob,
      getImageData: CanvasRenderingContext2D.prototype.getImageData,
      captureStream: HTMLCanvasElement.prototype.captureStream,
      createImageBitmap: window.createImageBitmap,
    };

    const BLANK =
      "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";

    HTMLCanvasElement.prototype.toDataURL = function () {
      return BLANK;
    };
    HTMLCanvasElement.prototype.toBlob = function (cb) {
      const e = document.createElement("canvas");
      e.width = 1;
      e.height = 1;
      orig.toBlob.call(e, cb);
    };
    CanvasRenderingContext2D.prototype.getImageData = function (
      _sx: number,
      _sy: number,
      sw: number,
      sh: number,
    ) {
      return new ImageData(sw, sh);
    };
    HTMLCanvasElement.prototype.captureStream = function () {
      return new MediaStream();
    };

    // OffscreenCanvas 차단 (지원 브라우저만)
    const hasTransfer = "transferControlToOffscreen" in HTMLCanvasElement.prototype;
    const origTransfer = hasTransfer
      ? HTMLCanvasElement.prototype.transferControlToOffscreen
      : null;
    if (hasTransfer) {
      HTMLCanvasElement.prototype.transferControlToOffscreen = function () {
        throw new DOMException("Not allowed", "SecurityError");
      };
    }

    // createImageBitmap — canvas 소스 차단
    const origCreateImageBitmap = window.createImageBitmap;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (window as any).createImageBitmap = function () {
      // eslint-disable-next-line prefer-rest-params
      const args = Array.from(arguments);
      if (args[0] instanceof HTMLCanvasElement) {
        const empty = document.createElement("canvas");
        empty.width = 1;
        empty.height = 1;
        args[0] = empty;
      }
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      return (origCreateImageBitmap as any).apply(window, args);
    };

    return () => {
      HTMLCanvasElement.prototype.toDataURL = orig.toDataURL;
      HTMLCanvasElement.prototype.toBlob = orig.toBlob;
      CanvasRenderingContext2D.prototype.getImageData = orig.getImageData;
      HTMLCanvasElement.prototype.captureStream = orig.captureStream;
      if (hasTransfer && origTransfer)
        HTMLCanvasElement.prototype.transferControlToOffscreen = origTransfer;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (window as any).createImageBitmap = origCreateImageBitmap;
    };
  }, []);

  // ─── Step 2: 포커스/가시성 감지 → 콘텐츠 블러 ───
  useEffect(() => {
    const onVisibility = () => setIsObscured(document.visibilityState === "hidden");
    const onBlur = () => setIsObscured(true);
    const onFocus = () => setIsObscured(false);
    const onBeforePrint = () => setIsObscured(true);
    const onAfterPrint = () => setIsObscured(false);

    document.addEventListener("visibilitychange", onVisibility);
    window.addEventListener("blur", onBlur);
    window.addEventListener("focus", onFocus);
    window.addEventListener("beforeprint", onBeforePrint);
    window.addEventListener("afterprint", onAfterPrint);

    return () => {
      document.removeEventListener("visibilitychange", onVisibility);
      window.removeEventListener("blur", onBlur);
      window.removeEventListener("focus", onFocus);
      window.removeEventListener("beforeprint", onBeforePrint);
      window.removeEventListener("afterprint", onAfterPrint);
    };
  }, []);

  // ─── Step 3: DOM 조작 감지 ───
  const handleTampering = useCallback(() => {
    document.querySelectorAll("canvas").forEach((c) => {
      c.getContext("2d")?.clearRect(0, 0, c.width, c.height);
    });
    navigate("/book/ebook/my");
  }, [navigate]);

  // MutationObserver — DOM 변경 감지
  useEffect(() => {
    const el = viewerRef.current;
    if (!el) return;

    const observer = new MutationObserver((mutations) => {
      for (const m of mutations) {
        for (const removed of m.removedNodes) {
          if (removed instanceof HTMLCanvasElement) {
            handleTampering();
            return;
          }
        }
        if (m.type === "attributes" && m.attributeName === "style") {
          const target = m.target as HTMLElement;
          if (target.style.userSelect !== "" && target.style.userSelect !== "none") {
            handleTampering();
            return;
          }
        }
      }
    });

    observer.observe(el, {
      childList: true,
      subtree: true,
      attributes: true,
      attributeFilter: ["style", "class", "hidden"],
    });

    return () => observer.disconnect();
  }, [handleTampering]);

  // getComputedStyle 주기 검사 — CSS 규칙 추가 감지
  useEffect(() => {
    const el = viewerRef.current;
    if (!el) return;

    const interval = setInterval(() => {
      const cs = getComputedStyle(el);
      if (cs.userSelect !== "none" || cs.pointerEvents === "auto") {
        handleTampering();
      }
    }, 2000);

    return () => clearInterval(interval);
  }, [handleTampering]);

  // ─── Step 5: DevTools 감지 → 콘텐츠 블러 ───
  useDevToolsDetection(
    useCallback(() => setIsObscured(true), []),
    useCallback(() => setIsObscured(false), []),
  );

  const { data: meta, isLoading: metaLoading, error: metaError } = useViewerMeta(
    purchaseCode ?? ""
  );

  // ─── Step 4: 동시 세션 제한 — Heartbeat ───
  useEffect(() => {
    if (!meta?.session_id) return;

    const interval = setInterval(async () => {
      try {
        const res = await sendViewerHeartbeat(meta.session_id);
        if (!res.valid) {
          // 세션 무효 → Canvas 즉시 클리어 후 리다이렉트
          document.querySelectorAll("canvas").forEach((c) => {
            c.getContext("2d")?.clearRect(0, 0, c.width, c.height);
          });
          navigate("/book/ebook/my");
        }
      } catch {
        // 네트워크 오류 → 다음 heartbeat에서 재시도
      }
    }, 30_000);

    return () => clearInterval(interval);
  }, [meta?.session_id, navigate]);

  const totalPages = meta?.total_pages ?? 0;

  // 두 쪽 보기 시 오른쪽 페이지 번호 (1페이지는 단독 표시 — 표지)
  const spreadRightPage =
    viewMode === "spread" && currentPage > 1 && currentPage < totalPages
      ? currentPage + 1
      : null;

  const tileMode = meta?.tile_mode ?? false;
  const gridRows = meta?.grid_rows ?? 3;
  const gridCols = meta?.grid_cols ?? 3;

  const sessionId = meta?.session_id;
  const hmacSecret = meta?.hmac_secret;

  // 단일 이미지 모드 (tile_mode=false)
  const { data: imageData, isLoading: imageLoading } = usePageImage(
    purchaseCode ?? "",
    currentPage,
    totalPages,
    !!meta && !tileMode,
    viewMode,
    sessionId,
    hmacSecret,
  );

  const { data: imageDataRight, isLoading: imageLoadingRight } = usePageImage(
    purchaseCode ?? "",
    spreadRightPage ?? 0,
    totalPages,
    !!meta && !tileMode && spreadRightPage !== null,
    viewMode,
    sessionId,
    hmacSecret,
  );

  // 타일 분할 모드 (tile_mode=true)
  const { tiles: tilesLeft, isLoading: tilesLeftLoading } = usePageTiles(
    purchaseCode ?? "",
    currentPage,
    totalPages,
    gridRows,
    gridCols,
    !!meta && tileMode,
    sessionId,
    hmacSecret,
  );

  const { tiles: tilesRight, isLoading: tilesRightLoading } = usePageTiles(
    purchaseCode ?? "",
    spreadRightPage ?? 0,
    totalPages,
    gridRows,
    gridCols,
    !!meta && tileMode && spreadRightPage !== null,
    sessionId,
    hmacSecret,
  );

  // 통합 로딩 상태
  const pageLoading = tileMode
    ? tilesLeftLoading
    : imageLoading;
  const pageRightLoading = tileMode
    ? tilesRightLoading
    : imageLoadingRight;

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

  // 모바일 감지 + spread 모드 자동 비활성화
  useEffect(() => {
    const onResize = () => {
      const mobile = window.innerWidth < 768;
      setIsMobile(mobile);
      if (mobile) setViewMode("single");
    };
    window.addEventListener("resize", onResize);
    return () => window.removeEventListener("resize", onResize);
  }, []);

  // 터치 스와이프 네비게이션
  useEffect(() => {
    const el = viewerRef.current;
    if (!el) return;

    const onTouchStart = (e: TouchEvent) => {
      if (e.touches.length === 1) {
        touchStartRef.current = { x: e.touches[0].clientX, y: e.touches[0].clientY };
      }
    };

    const onTouchEnd = (e: TouchEvent) => {
      if (!touchStartRef.current || e.changedTouches.length !== 1) return;
      const dx = e.changedTouches[0].clientX - touchStartRef.current.x;
      const dy = e.changedTouches[0].clientY - touchStartRef.current.y;
      touchStartRef.current = null;

      // 수평 스와이프만 처리 (세로 이동이 가로보다 크면 무시)
      if (Math.abs(dx) < 50 || Math.abs(dy) > Math.abs(dx)) return;

      if (dx < 0) goToNextPage();
      else goToPrevPage();
    };

    el.addEventListener("touchstart", onTouchStart, { passive: true });
    el.addEventListener("touchend", onTouchEnd, { passive: true });
    return () => {
      el.removeEventListener("touchstart", onTouchStart);
      el.removeEventListener("touchend", onTouchEnd);
    };
  }, [goToNextPage, goToPrevPage]);

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
          <Lock className="w-12 h-12 text-status-warning" />
          <h2 className="text-lg font-semibold">{t("ebook.viewer.paymentRequired")}</h2>
          <p className="text-muted-foreground text-sm max-w-sm">
            {t("ebook.viewer.paymentRequiredDesc")}
          </p>
          <Button variant="outline" onClick={() => navigate("/book/ebook/my")}>
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
        <Button variant="outline" onClick={() => navigate("/book/ebook/my")}>
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
        <Button variant="outline" onClick={() => navigate("/book/ebook/my")}>
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

  const isLoading = tileMode
    ? pageLoading || (viewMode === "spread" && pageRightLoading)
    : imageLoading || (viewMode === "spread" && imageLoadingRight);
  const hasImage = tileMode
    ? tilesLeft.every((t) => !!t)
    : !!imageData;

  const handleCopyrightAck = () => {
    sessionStorage.setItem("ebook_copyright_ack", "1");
    setShowCopyrightNotice(false);
  };

  return (
    <>
      {/* ─── 저작권 보호 고지 모달 ─── */}
      {showCopyrightNotice && (
        <div className="fixed inset-0 z-[9999] flex items-center justify-center bg-black/60 backdrop-blur-sm">
          <div className="mx-4 max-w-md rounded-2xl bg-background p-6 shadow-2xl border space-y-4">
            <div className="flex items-center gap-3">
              <ShieldCheck className="h-8 w-8 text-primary flex-shrink-0" />
              <h2 className="text-lg font-semibold">{t("ebook.viewer.copyrightTitle")}</h2>
            </div>
            <div className="space-y-2 text-sm text-muted-foreground">
              <p>{t("ebook.viewer.copyrightNotice")}</p>
              <p>{t("ebook.viewer.copyrightLegal")}</p>
              <p>{t("ebook.viewer.copyrightWatermark")}</p>
            </div>
            <Button className="w-full" onClick={handleCopyrightAck}>
              {t("ebook.viewer.copyrightConfirm")}
            </Button>
          </div>
        </div>
      )}

      {/* 풀스크린 시 사이트 전체 숨기고 뷰어만 표시 */}
      <style>{`
        @media print { body * { display: none !important; visibility: hidden !important; } }
        :fullscreen .site-header,
        :fullscreen .site-nav,
        :fullscreen .site-footer { display: none !important; }
      `}</style>

      <div
        ref={viewerRef}
        className={`ebook-viewer flex flex-col select-none ${
          isFullscreen
            ? "fixed inset-0 z-[9999] bg-surface-inverted"
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
              ? `bg-surface-inverted/90 border-surface-inverted-foreground/20 text-surface-inverted-foreground ${
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
              className={isFullscreen ? "text-surface-inverted-foreground hover:bg-surface-inverted-foreground/10" : ""}
            >
              <List className="w-5 h-5" />
            </Button>
            <span className={`text-sm hidden sm:inline ${isFullscreen ? "text-surface-inverted-foreground/70" : "text-muted-foreground"}`}>
              {pageDisplay}
            </span>
          </div>

          <div className="flex items-center gap-1">
            {/* 한 쪽 / 두 쪽 보기 (모바일에서 숨김) */}
            {!isMobile && (
              <Button
                variant="ghost"
                size="icon"
                onClick={() => setViewMode(viewMode === "single" ? "spread" : "single")}
                title={viewMode === "single" ? t("ebook.viewer.spreadView") : t("ebook.viewer.singleView")}
                className={isFullscreen ? "text-surface-inverted-foreground hover:bg-surface-inverted-foreground/10" : ""}
              >
                {viewMode === "single" ? <BookOpen className="w-4 h-4" /> : <FileText className="w-4 h-4" />}
              </Button>
            )}

            {/* 줌 */}
            <Button
              variant="ghost"
              size="icon"
              onClick={() => setZoomIndex((i) => Math.max(0, i - 1))}
              disabled={zoomIndex === 0}
              className={isFullscreen ? "text-surface-inverted-foreground hover:bg-surface-inverted-foreground/10" : ""}
            >
              <ZoomOut className="w-4 h-4" />
            </Button>
            <span className={`text-xs w-10 text-center hidden sm:inline ${isFullscreen ? "text-surface-inverted-foreground/70" : "text-muted-foreground"}`}>
              {zoom}%
            </span>
            <Button
              variant="ghost"
              size="icon"
              onClick={() => setZoomIndex((i) => Math.min(ZOOM_LEVELS.length - 1, i + 1))}
              disabled={zoomIndex === ZOOM_LEVELS.length - 1}
              className={isFullscreen ? "text-surface-inverted-foreground hover:bg-surface-inverted-foreground/10" : ""}
            >
              <ZoomIn className="w-4 h-4" />
            </Button>

            <Button
              variant="ghost"
              size="icon"
              onClick={toggleFullscreen}
              className={isFullscreen ? "text-surface-inverted-foreground hover:bg-surface-inverted-foreground/10" : ""}
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
                      <span className="flex flex-col leading-snug">
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
            isFullscreen ? "bg-surface-inverted" : "bg-muted"
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
                filter: isObscured ? "blur(30px)" : "none",
                transition: "filter 0.15s ease-out",
                willChange: "filter",
              }}
            >
              {tileMode ? (
                <>
                  <TiledPageCanvas
                    tiles={tilesLeft}
                    gridRows={gridRows}
                    gridCols={gridCols}
                    className="shadow-2xl rounded-sm"
                    style={{
                      maxHeight: isFullscreen ? "calc(100vh - 100px)" : "calc(100vh - 140px)",
                      maxWidth: viewMode === "spread" ? "45vw" : "90vw",
                      width: "auto",
                      height: "auto",
                      pointerEvents: "none",
                    }}
                  />
                  {viewMode === "spread" && tilesRight.every((t) => !!t) && (
                    <TiledPageCanvas
                      tiles={tilesRight}
                      gridRows={gridRows}
                      gridCols={gridCols}
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
                </>
              ) : (
                <>
                  <PageCanvas
                    data={imageData!}
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
                </>
              )}
            </div>
          ) : (
            <div className="text-muted-foreground">{t("ebook.viewer.loadFailed")}</div>
          )}
        </div>

        {/* ─── 하단 네비게이션 ─── */}
        <div
          className={`flex items-center justify-center gap-2 sm:gap-4 px-2 sm:px-4 py-2 sm:py-3 border-t backdrop-blur z-20 transition-all duration-300 ${
            isFullscreen
              ? `bg-surface-inverted/90 border-surface-inverted-foreground/20 ${
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
            className={isFullscreen ? "border-surface-inverted-foreground/20 text-surface-inverted-foreground hover:bg-surface-inverted-foreground/10" : ""}
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
            className="flex-1 max-w-64 accent-primary"
          />

          <Button
            variant="outline"
            size="icon"
            onClick={goToNextPage}
            disabled={currentPage >= totalPages}
            className={isFullscreen ? "border-surface-inverted-foreground/20 text-surface-inverted-foreground hover:bg-surface-inverted-foreground/10" : ""}
          >
            <ChevronRight className="w-5 h-5" />
          </Button>
        </div>
      </div>
    </>
  );
}
