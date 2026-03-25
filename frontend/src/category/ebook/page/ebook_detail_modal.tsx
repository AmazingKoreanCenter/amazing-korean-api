import { useState, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { ChevronLeft, ChevronRight, ImageOff } from "lucide-react";

import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
import type { EbookCatalogItem, EbookEdition } from "../types";

interface EbookDetailModalProps {
  item: EbookCatalogItem | null;
  edition: EbookEdition;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

type ImageVariant = "cover" | "inner" | "toc";

const VARIANTS: ImageVariant[] = ["cover", "inner", "toc"];

function imageSrc(edition: EbookEdition, language: string, variant: ImageVariant) {
  const suffix = variant === "cover" ? "" : `-${variant}`;
  return `/covers/${edition}-${language}${suffix}.webp`;
}

export function EbookDetailModal({
  item,
  edition,
  open,
  onOpenChange,
}: EbookDetailModalProps) {
  const { t, i18n } = useTranslation();
  const [variantIndex, setVariantIndex] = useState(0);

  const goPrev = useCallback(() => {
    setVariantIndex((i) => (i - 1 + VARIANTS.length) % VARIANTS.length);
  }, []);

  const goNext = useCallback(() => {
    setVariantIndex((i) => (i + 1) % VARIANTS.length);
  }, []);

  if (!item) return null;

  const langName = i18n.language === "ko" ? item.language_name_ko : item.language_name_en;
  const editionInfo = item.editions.find((e) => e.edition === edition);
  const activeVariant = VARIANTS[variantIndex];

  const variantLabels: Record<ImageVariant, string> = {
    cover: t("ebook.detail.coverImage"),
    inner: t("ebook.detail.innerImage"),
    toc: t("ebook.detail.tocImage"),
  };

  return (
    <Dialog open={open} onOpenChange={(o) => { if (!o) setVariantIndex(0); onOpenChange(o); }}>
      <DialogContent className="max-w-3xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <div className="flex items-center justify-between gap-3">
            <DialogTitle className="text-left">
              {t("ebook.catalog.bookTitle", { language: langName })}
            </DialogTitle>
            <span className="inline-flex items-center gap-1.5 text-xs font-medium text-primary bg-primary/10 border border-primary/20 rounded-md px-2.5 py-1 flex-shrink-0">
              E-book
            </span>
          </div>
          <DialogDescription>
            {edition === "student"
              ? t("ebook.catalog.studentEdition")
              : t("ebook.catalog.teacherEdition")}
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-6">
          {/* Image gallery with left/right navigation */}
          <div className="relative">
            <GalleryPreview
              edition={edition}
              language={item.language}
              variant={activeVariant}
              langName={langName}
            />

            {/* Navigation arrows */}
            <button
              type="button"
              onClick={goPrev}
              className="absolute left-2 top-1/2 -translate-y-1/2 w-9 h-9 rounded-full bg-background/80 border shadow-sm flex items-center justify-center hover:bg-background transition-colors"
            >
              <ChevronLeft className="h-5 w-5" />
            </button>
            <button
              type="button"
              onClick={goNext}
              className="absolute right-2 top-1/2 -translate-y-1/2 w-9 h-9 rounded-full bg-background/80 border shadow-sm flex items-center justify-center hover:bg-background transition-colors"
            >
              <ChevronRight className="h-5 w-5" />
            </button>

            {/* Indicator dots + label */}
            <div className="absolute bottom-3 left-1/2 -translate-x-1/2 flex items-center gap-2 bg-background/80 rounded-full px-3 py-1.5">
              {VARIANTS.map((v, i) => (
                <button
                  key={v}
                  type="button"
                  onClick={() => setVariantIndex(i)}
                  className={`w-2 h-2 rounded-full transition-colors ${
                    i === variantIndex ? "bg-primary" : "bg-muted-foreground/30"
                  }`}
                />
              ))}
              <span className="text-xs text-muted-foreground ml-1">{variantLabels[activeVariant]}</span>
            </div>
          </div>

          {/* Description + Price */}
          <div className="space-y-3">
            <p className="text-sm text-muted-foreground leading-relaxed">
              {t("ebook.detail.description", { language: langName })}
            </p>
            {editionInfo && (
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">
                  {editionInfo.total_pages}{t("ebook.detail.pages")}
                </span>
                <span className="text-sm font-semibold">
                  {editionInfo.price.toLocaleString()} {editionInfo.currency}
                </span>
              </div>
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}

function GalleryPreview({
  edition,
  language,
  variant,
  langName,
}: {
  edition: EbookEdition;
  language: string;
  variant: ImageVariant;
  langName: string;
}) {
  const [error, setError] = useState(false);
  const { t } = useTranslation();
  const src = imageSrc(edition, language, variant);

  // Reset error when variant changes
  const [prevSrc, setPrevSrc] = useState(src);
  if (src !== prevSrc) {
    setPrevSrc(src);
    setError(false);
  }

  return (
    <div className="aspect-[3/4] max-h-[50vh] mx-auto overflow-hidden rounded-xl bg-muted">
      {error ? (
        <div className="w-full h-full flex flex-col items-center justify-center text-muted-foreground gap-3">
          <ImageOff className="h-12 w-12" />
          <span className="text-sm">{t("ebook.detail.imageNotAvailable")}</span>
        </div>
      ) : (
        <img
          src={src}
          alt={langName}
          className="w-full h-full object-contain"
          onError={() => setError(true)}
        />
      )}
    </div>
  );
}
