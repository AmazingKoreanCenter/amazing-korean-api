import { useState } from "react";
import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { BookOpen, ArrowRight, Tablet, ChevronLeft, ChevronRight, ImageOff, FileText, Globe, Tag } from "lucide-react";

import { Button } from "@/components/ui/button";
import { PageMeta } from "@/components/page_meta";
import { HeroSection } from "@/components/blocks/hero_section";
import { SectionContainer } from "@/components/blocks/section_container";
import { ImageLightbox } from "@/components/image_lightbox";
import { getDefaultLangKey, SAMPLE_PAGES, BOOK_PAGES } from "../book_data";

const SLIDE_COUNT = 6; // cover + 5 sample pages

const SLIDE_COLORS: Record<number, { bg: string; text: string; border: string }> = {
  0: { bg: "bg-blue-500/10", text: "text-blue-600", border: "border-blue-500/20" },
  1: { bg: "bg-emerald-500/10", text: "text-emerald-600", border: "border-emerald-500/20" },
  2: { bg: "bg-amber-500/10", text: "text-amber-600", border: "border-amber-500/20" },
  3: { bg: "bg-violet-500/10", text: "text-violet-600", border: "border-violet-500/20" },
  4: { bg: "bg-rose-500/10", text: "text-rose-600", border: "border-rose-500/20" },
  5: { bg: "bg-teal-500/10", text: "text-teal-600", border: "border-teal-500/20" },
};

function getSlideImage(langKey: string, index: number): string {
  if (index === 0) return `/covers/student-${langKey}.webp`;
  const page = SAMPLE_PAGES[index - 1];
  return `/book-samples/student-${langKey}-p${page}.webp`;
}

export function BookHubPage() {
  const { t, i18n } = useTranslation();
  const langKey = getDefaultLangKey(i18n.language);
  const [slideIndex, setSlideIndex] = useState(0);
  const [lightboxOpen, setLightboxOpen] = useState(false);
  const [imgError, setImgError] = useState<Record<number, boolean>>({});

  const goPrev = () => setSlideIndex((i) => (i - 1 + SLIDE_COUNT) % SLIDE_COUNT);
  const goNext = () => setSlideIndex((i) => (i + 1) % SLIDE_COUNT);

  const currentSrc = getSlideImage(langKey, slideIndex);

  // Reset error state when slide changes
  const handleImgError = () => setImgError((prev) => ({ ...prev, [slideIndex]: true }));

  return (
    <div className="flex flex-col">
      <PageMeta titleKey="bookHub.title" descriptionKey="bookHub.subtitle" />

      {/* Hero */}
      <HeroSection
        size="sm"
        badge={
          <>
            <BookOpen className="h-4 w-4 text-primary" />
            <span className="text-sm text-muted-foreground">{t("bookHub.badge")}</span>
          </>
        }
        title={t("bookHub.title")}
        subtitle={t("bookHub.subtitle")}
      />

      {/* Main: Gallery + Description */}
      <SectionContainer>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 items-start">
          {/* Left: Image gallery */}
          <div className="flex flex-col items-center gap-3">
            <div className="relative">
              <div
                className="h-48 md:h-[420px] w-auto aspect-[3/4] overflow-hidden rounded-xl bg-muted shadow-lg cursor-pointer"
                onClick={() => !imgError[slideIndex] && setLightboxOpen(true)}
              >
                {imgError[slideIndex] ? (
                  <div className="w-full h-full flex flex-col items-center justify-center text-muted-foreground gap-3">
                    <ImageOff className="h-12 w-12" />
                  </div>
                ) : (
                  <img
                    src={currentSrc}
                    alt={t(`bookHub.slideTitle${slideIndex}`)}
                    className="w-full h-full object-contain"
                    onError={handleImgError}
                  />
                )}
              </div>

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
            </div>

            {/* Indicator dots */}
            <div className="flex items-center gap-2">
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

          {/* Right: Title + Description + Buttons (fixed positions) */}
          <div className="flex flex-col justify-between md:h-[420px] md:py-1">
            {/* Title (top) */}
            <h3 className="text-lg md:text-xl font-bold text-center md:text-left">
              {t(`bookHub.slideTitle${slideIndex}`)}
            </h3>

            {/* Keyword tags (fixed below title) */}
            <div className="flex flex-wrap gap-2 mt-3">
              {t(`bookHub.slideTags${slideIndex}`).split(",").map((tag) => {
                const color = SLIDE_COLORS[slideIndex];
                return (
                  <span
                    key={tag}
                    className={`inline-flex items-center gap-1 text-xs font-medium rounded-full px-2.5 py-1 border ${color.bg} ${color.text} ${color.border}`}
                  >
                    <Tag className="h-3 w-3" />
                    {tag}
                  </span>
                );
              })}
            </div>

            {/* Description (middle) */}
            <div className="text-muted-foreground leading-relaxed space-y-2 my-4">
              {t(`bookHub.slideDesc${slideIndex}`).split("\n").map((line, i) => (
                <p key={i}>{line}</p>
              ))}
            </div>

            {/* Spec summary card */}
            <div className="grid grid-cols-3 gap-3 my-3">
              <div className="flex items-center justify-center gap-2 rounded-lg border bg-muted/50 py-3">
                <FileText className="h-5 w-5 text-muted-foreground" />
                <span className="text-sm font-semibold">{t("bookHub.specPages", { count: BOOK_PAGES })}</span>
              </div>
              <div className="flex items-center justify-center gap-2 rounded-lg border bg-muted/50 py-3">
                <Globe className="h-5 w-5 text-muted-foreground" />
                <span className="text-sm font-semibold">{t("bookHub.specLanguages", { count: 22 })}</span>
              </div>
              <div className="flex items-center justify-center gap-2 rounded-lg border bg-muted/50 py-3">
                <BookOpen className="h-5 w-5 text-muted-foreground" />
                <span className="text-sm font-semibold">{t("bookHub.specPrice")}</span>
              </div>
            </div>

            {/* CTA buttons (bottom) */}
            <div className="flex gap-3">
              <Button asChild size="default" className="rounded-full flex-1">
                <Link to="/book/textbook">
                  <BookOpen className="mr-2 h-5 w-5" />
                  {t("bookHub.ctaTextbook")}
                  <ArrowRight className="ml-1.5 h-4 w-4" />
                </Link>
              </Button>
              <Button asChild size="default" variant="outline" className="rounded-full flex-1">
                <Link to="/book/ebook">
                  <Tablet className="mr-2 h-5 w-5" />
                  {t("bookHub.ctaEbook")}
                  <ArrowRight className="ml-1.5 h-4 w-4" />
                </Link>
              </Button>
            </div>
          </div>
        </div>
      </SectionContainer>

      {/* Lightbox */}
      <ImageLightbox
        src={currentSrc}
        alt={t(`bookHub.slideTitle${slideIndex}`)}
        open={lightboxOpen}
        onOpenChange={setLightboxOpen}
        onPrev={goPrev}
        onNext={goNext}
      />
    </div>
  );
}
