import { useState } from "react";
import { useTranslation } from "react-i18next";
import { ImageOff, ShoppingCart } from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
import type { EbookCatalogItem, EbookEdition } from "../types";

interface EbookPreviewModalProps {
  item: EbookCatalogItem | null;
  edition: EbookEdition;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onPurchase: () => void;
}

type ImageVariant = "cover" | "toc" | "sample-1" | "sample-2";

function imageSrc(edition: EbookEdition, language: string, variant: ImageVariant) {
  return `/ebook-previews/${edition}/${language}/${variant}.webp`;
}

function GalleryImage({
  src,
  alt,
  label,
  selected,
  onClick,
}: {
  src: string;
  alt: string;
  label: string;
  selected: boolean;
  onClick: () => void;
}) {
  const [error, setError] = useState(false);
  const { t } = useTranslation();

  return (
    <button
      type="button"
      onClick={onClick}
      className={`flex flex-col items-center gap-1.5 group ${
        selected ? "ring-2 ring-primary rounded-lg" : ""
      }`}
    >
      <div className="aspect-[3/4] w-full overflow-hidden rounded-lg bg-muted">
        {error ? (
          <div className="w-full h-full flex flex-col items-center justify-center text-muted-foreground gap-2 p-2">
            <ImageOff className="h-8 w-8" />
            <span className="text-xs text-center">{t("ebook.preview.imageNotAvailable")}</span>
          </div>
        ) : (
          <img
            src={src}
            alt={alt}
            className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
            loading="lazy"
            onError={() => setError(true)}
          />
        )}
      </div>
      <span className="text-xs text-muted-foreground">{label}</span>
    </button>
  );
}

function MainPreview({
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
          <span className="text-sm">{t("ebook.preview.imageNotAvailable")}</span>
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

export function EbookPreviewModal({
  item,
  edition,
  open,
  onOpenChange,
  onPurchase,
}: EbookPreviewModalProps) {
  const { t, i18n } = useTranslation();
  const [activeVariant, setActiveVariant] = useState<ImageVariant>("cover");

  if (!item) return null;

  const langName = i18n.language === "ko" ? item.language_name_ko : item.language_name_en;
  const editionLabel =
    edition === "teacher"
      ? t("ebook.catalog.teacherEdition")
      : t("ebook.catalog.studentEdition");

  const editionInfo = item.editions.find((e) => e.edition === edition);

  const variants: { key: ImageVariant; label: string }[] = [
    { key: "cover", label: t("ebook.preview.cover") },
    { key: "toc", label: t("ebook.preview.toc") },
    { key: "sample-1", label: t("ebook.preview.sample", { n: 1 }) },
    { key: "sample-2", label: t("ebook.preview.sample", { n: 2 }) },
  ];

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>
            {langName} — {editionLabel}
          </DialogTitle>
          <DialogDescription>{t("ebook.preview.description")}</DialogDescription>
        </DialogHeader>

        <div className="space-y-6">
          {/* Main preview image */}
          <MainPreview
            edition={edition}
            language={item.language}
            variant={activeVariant}
            langName={langName}
          />

          {/* Thumbnail gallery */}
          <div className="grid grid-cols-4 gap-3">
            {variants.map((v) => (
              <GalleryImage
                key={v.key}
                src={imageSrc(edition, item.language, v.key)}
                alt={`${langName} ${v.label}`}
                label={v.label}
                selected={activeVariant === v.key}
                onClick={() => setActiveVariant(v.key)}
              />
            ))}
          </div>

          {/* Info */}
          {editionInfo && (
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">
                {editionInfo.total_pages}{t("ebook.preview.pages")}
              </span>
              <span className="text-lg font-semibold">
                {editionInfo.price.toLocaleString()} {editionInfo.currency}
              </span>
            </div>
          )}

          {/* Purchase button */}
          <Button
            size="lg"
            className="w-full rounded-full"
            onClick={() => {
              onOpenChange(false);
              onPurchase();
            }}
          >
            <ShoppingCart className="h-4 w-4 mr-2" />
            {t("ebook.preview.purchaseButton")}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
