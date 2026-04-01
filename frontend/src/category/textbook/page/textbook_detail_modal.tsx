import { useState, useCallback } from "react";
import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { ArrowRight, AlertTriangle, CircleCheck, ImageOff, ChevronLeft, ChevronRight } from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
import { ImageLightbox } from "@/components/image_lightbox";
import type { CatalogItem, TextbookType } from "../types";

interface TextbookDetailModalProps {
  item: CatalogItem | null;
  type: TextbookType;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const SAMPLE_PAGES = [7, 18, 29, 53, 118] as const;
const SLIDE_COUNT = 6; // cover + 5 sample pages

function getImageSrc(type: TextbookType, language: string, index: number): string {
  if (index === 0) return `/covers/${type}-${language}.webp`;
  const page = SAMPLE_PAGES[index - 1];
  return `/book-samples/${type}-${language}-p${page}.webp`;
}

export function TextbookDetailModal({
  item,
  type,
  open,
  onOpenChange,
}: TextbookDetailModalProps) {
  const { t, i18n } = useTranslation();
  const [slideIndex, setSlideIndex] = useState(0);
  const [lightboxOpen, setLightboxOpen] = useState(false);

  const goPrev = useCallback(() => {
    setSlideIndex((i) => (i - 1 + SLIDE_COUNT) % SLIDE_COUNT);
  }, []);

  const goNext = useCallback(() => {
    setSlideIndex((i) => (i + 1) % SLIDE_COUNT);
  }, []);

  if (!item) return null;

  const langName = i18n.language === "ko" ? item.language_name_ko : item.language_name_en;
  const currentSrc = getImageSrc(type, item.language, slideIndex);

  return (
    <Dialog open={open} onOpenChange={(o) => { if (!o) { setSlideIndex(0); setLightboxOpen(false); } onOpenChange(o); }}>
      <DialogContent
        className="max-w-[calc(100vw-2rem)] sm:max-w-3xl max-h-[90vh] overflow-y-auto"
        onEscapeKeyDown={(e) => { if (lightboxOpen) { e.preventDefault(); setLightboxOpen(false); } }}
      >
        <DialogHeader>
          <div className="flex items-center justify-between gap-3">
            <DialogTitle className="text-left">
              {t("textbook.catalog.bookTitle", { language: langName })}
            </DialogTitle>
            {item.isbn_ready ? (
              <span className="inline-flex items-center gap-1.5 text-sm font-medium text-status-success bg-status-success/5 border border-status-success/20 rounded-md px-3 py-1.5 flex-shrink-0">
                <CircleCheck className="h-3.5 w-3.5" />
                {t("textbook.catalog.editionInfo", { edition: type === "student" ? t("textbook.catalog.studentSection") : t("textbook.catalog.teacherSection") })}
              </span>
            ) : (
              <span className="inline-flex items-center gap-1.5 text-sm font-medium text-status-warning bg-status-warning/5 border border-status-warning/20 rounded-md px-3 py-1.5 flex-shrink-0">
                <AlertTriangle className="h-3.5 w-3.5" />
                {t("textbook.catalog.isbnPending")}
              </span>
            )}
          </div>
          <DialogDescription className="sr-only">
            {t("textbook.catalog.bookTitle", { language: langName })}
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-6 mt-4">
          {/* Image gallery with left/right navigation */}
          <div className="relative">
            <GalleryPreview
              src={currentSrc}
              alt={langName}
              onImageClick={() => setLightboxOpen(true)}
            />

            {/* Navigation arrows */}
            <button
              type="button"
              onClick={goPrev}
              className="absolute left-2 top-1/2 -translate-y-1/2 w-11 h-11 rounded-full bg-background/80 border shadow-sm flex items-center justify-center hover:bg-background transition-colors"
            >
              <ChevronLeft className="h-5 w-5" />
            </button>
            <button
              type="button"
              onClick={goNext}
              className="absolute right-2 top-1/2 -translate-y-1/2 w-11 h-11 rounded-full bg-background/80 border shadow-sm flex items-center justify-center hover:bg-background transition-colors"
            >
              <ChevronRight className="h-5 w-5" />
            </button>

            {/* Indicator dots */}
            <div className="absolute bottom-3 left-1/2 -translate-x-1/2 flex items-center gap-2 bg-background/80 rounded-full px-3 py-1.5">
              {Array.from({ length: SLIDE_COUNT }).map((_, i) => (
                <button
                  key={i}
                  type="button"
                  onClick={() => setSlideIndex(i)}
                  className={`w-2 h-2 rounded-full transition-colors ${
                    i === slideIndex ? "bg-primary" : "bg-muted-foreground/30"
                  }`}
                />
              ))}
            </div>
          </div>

          {/* Slide description */}
          <div className="space-y-3">
            <h3 className="text-base font-semibold">{t(`bookHub.slideTitle${slideIndex}`)}</h3>
            <div className="text-sm text-muted-foreground leading-relaxed space-y-1">
              {t(`bookHub.slideDesc${slideIndex}`).split("\n").map((line, i) => (
                <p key={i}>{line}</p>
              ))}
            </div>
            <div className="flex justify-end">
              <span className="text-sm font-semibold">{t("textbook.catalog.pricePerUnit")}</span>
            </div>
          </div>

          {/* Order button */}
          <Button asChild size="lg" className="w-full rounded-full">
            <Link
              to={`/book/textbook/order?lang=${item.language}&type=${type}`}
              onClick={() => onOpenChange(false)}
            >
              {t("textbook.detail.orderNow")}
              <ArrowRight className="ml-2 h-4 w-4" />
            </Link>
          </Button>
        </div>

        {/* Lightbox - inside DialogContent for focus trap compatibility */}
        <ImageLightbox
          src={currentSrc}
          alt={langName}
          open={lightboxOpen}
          onOpenChange={setLightboxOpen}
          onPrev={goPrev}
          onNext={goNext}
        />
      </DialogContent>
    </Dialog>
  );
}

function GalleryPreview({
  src,
  alt,
  onImageClick,
}: {
  src: string;
  alt: string;
  onImageClick: () => void;
}) {
  const [error, setError] = useState(false);
  const { t } = useTranslation();

  // Reset error when src changes
  const [prevSrc, setPrevSrc] = useState(src);
  if (src !== prevSrc) {
    setPrevSrc(src);
    setError(false);
  }

  return (
    <div
      className="aspect-[3/4] max-h-[50vh] mx-auto overflow-hidden rounded-xl bg-muted cursor-pointer"
      onClick={() => !error && onImageClick()}
    >
      {error ? (
        <div className="w-full h-full flex flex-col items-center justify-center text-muted-foreground gap-3">
          <ImageOff className="h-12 w-12" />
          <span className="text-sm">{t("textbook.detail.imageNotAvailable")}</span>
        </div>
      ) : (
        <img
          src={src}
          alt={alt}
          className="w-full h-full object-contain"
          onError={() => setError(true)}
        />
      )}
    </div>
  );
}
