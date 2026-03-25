import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { BookOpen, ArrowRight, Tablet } from "lucide-react";

import { Button } from "@/components/ui/button";
import { PageMeta } from "@/components/page_meta";
import { HeroSection } from "@/components/sections/hero_section";
import { SectionContainer } from "@/components/sections/section_container";
import { getDefaultLangKey, SAMPLE_PAGES } from "../book_data";

export function BookHubPage() {
  const { t, i18n } = useTranslation();
  const langKey = getDefaultLangKey(i18n.language);
  const langName = t(`bookHub.langName`, { defaultValue: "" });

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

      {/* Main: Cover + Description + Sample Pages */}
      <SectionContainer>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 items-start">
          {/* Left: Cover image */}
          <div className="flex justify-center">
            <img
              src={`/covers/student-${langKey}.webp`}
              alt={langName}
              className="w-64 md:w-80 rounded-xl shadow-lg"
            />
          </div>

          {/* Right: Description + Sample pages */}
          <div className="space-y-6">
            <h2 className="text-xl md:text-2xl font-bold">
              {t("bookHub.descriptionTitle")}
            </h2>
            <p className="text-muted-foreground leading-relaxed">
              {t("bookHub.description")}
            </p>

            {/* Sample pages */}
            <div className="space-y-3">
              <h3 className="text-lg font-semibold">{t("bookHub.samplePages")}</h3>
              <div className="grid grid-cols-5 gap-2">
                {SAMPLE_PAGES.map((page) => (
                  <div key={page} className="aspect-[3/4] rounded-lg bg-muted overflow-hidden">
                    <img
                      src={`/book-samples/student-${langKey}-p${page}.webp`}
                      alt={`Page ${page}`}
                      className="w-full h-full object-cover"
                      loading="lazy"
                      onError={(e) => {
                        (e.target as HTMLImageElement).style.display = "none";
                      }}
                    />
                  </div>
                ))}
              </div>
              <p className="text-xs text-muted-foreground">{t("bookHub.samplePagesNote")}</p>
            </div>
          </div>
        </div>
      </SectionContainer>

      {/* CTA: Textbook / E-book */}
      <SectionContainer className="border-t">
        <div className="max-w-2xl mx-auto text-center space-y-6">
          <h2 className="text-2xl md:text-3xl font-bold">{t("bookHub.ctaTitle")}</h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <Button asChild size="lg" className="rounded-full h-12">
              <Link to="/book/textbook">
                <BookOpen className="mr-2 h-5 w-5" />
                {t("bookHub.ctaTextbook")}
                <ArrowRight className="ml-2 h-4 w-4" />
              </Link>
            </Button>
            <Button asChild size="lg" variant="outline" className="rounded-full h-12">
              <Link to="/book/ebook">
                <Tablet className="mr-2 h-5 w-5" />
                {t("bookHub.ctaEbook")}
                <ArrowRight className="ml-2 h-4 w-4" />
              </Link>
            </Button>
          </div>
        </div>
      </SectionContainer>
    </div>
  );
}
