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
import type { CatalogItem, TextbookType } from "../types";

interface TextbookDetailModalProps {
  item: CatalogItem | null;
  type: TextbookType;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

type ImageVariant = "cover" | "inner" | "toc";

const VARIANTS: ImageVariant[] = ["cover", "inner", "toc"];

function imageSrc(type: TextbookType, language: string, variant: ImageVariant) {
  const suffix = variant === "cover" ? "" : `-${variant}`;
  return `/covers/${type}-${language}${suffix}.webp`;
}

export function TextbookDetailModal({
  item,
  type,
  open,
  onOpenChange,
}: TextbookDetailModalProps) {
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
  const activeVariant = VARIANTS[variantIndex];

  const variantLabels: Record<ImageVariant, string> = {
    cover: t("textbook.detail.coverImage"),
    inner: t("textbook.detail.innerImage"),
    toc: t("textbook.detail.tocImage"),
  };

  return (
    <Dialog open={open} onOpenChange={(o) => { if (!o) setVariantIndex(0); onOpenChange(o); }}>
      <DialogContent className="max-w-3xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <div className="flex items-center justify-between gap-3">
            <DialogTitle className="text-left">
              {t("textbook.catalog.bookTitle", { language: langName })}
            </DialogTitle>
            {item.isbn_ready ? (
              <span className="inline-flex items-center gap-1.5 text-xs font-medium text-emerald-600 bg-emerald-50 border border-emerald-200 rounded-md px-2.5 py-1 flex-shrink-0">
                <CircleCheck className="h-3.5 w-3.5" />
                {t("textbook.catalog.editionInfo", { edition: type === "student" ? t("textbook.catalog.studentSection") : t("textbook.catalog.teacherSection") })}
              </span>
            ) : (
              <span className="inline-flex items-center gap-1.5 text-xs font-medium text-amber-600 bg-amber-50 border border-amber-200 rounded-md px-2.5 py-1 flex-shrink-0">
                <AlertTriangle className="h-3.5 w-3.5" />
                {t("textbook.catalog.isbnPending")}
              </span>
            )}
          </div>
          <DialogDescription>
            {type === "student"
              ? t("textbook.catalog.studentSection")
              : t("textbook.catalog.teacherSection")}
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-6">
          {/* Image gallery with left/right navigation */}
          <div className="relative">
            <GalleryPreview
              type={type}
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
              {t("textbook.detail.description", { language: langName })}
            </p>
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
      </DialogContent>
    </Dialog>
  );
}

function GalleryPreview({
  type,
  language,
  variant,
  langName,
}: {
  type: TextbookType;
  language: string;
  variant: ImageVariant;
  langName: string;
}) {
  const [error, setError] = useState(false);
  const { t } = useTranslation();
  const src = imageSrc(type, language, variant);

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
          <span className="text-sm">{t("textbook.detail.imageNotAvailable")}</span>
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
