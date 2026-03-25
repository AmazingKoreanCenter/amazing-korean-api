import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import {
  Play,
  BookOpen,
  GraduationCap,
  ArrowRight,
  Sparkles,
  Clock,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import { HeroSection } from "@/components/sections/hero_section";
import { SectionContainer } from "@/components/sections/section_container";
import { PageMeta } from "@/components/page_meta";

const FEATURES = [
  {
    icon: Play,
    titleKey: "comingSoon.videoTitle",
    descKey: "comingSoon.videoDescription",
  },
  {
    icon: BookOpen,
    titleKey: "comingSoon.studyTitle",
    descKey: "comingSoon.studyDescription",
  },
  {
    icon: GraduationCap,
    titleKey: "comingSoon.lessonTitle",
    descKey: "comingSoon.lessonDescription",
  },
] as const;

export function ComingSoonPage() {
  const { t } = useTranslation();

  return (
    <div className="flex flex-col">
      <PageMeta titleKey="seo.home.title" descriptionKey="seo.home.description" />

      <HeroSection
        badge={
          <>
            <Clock className="h-4 w-4 text-accent" />
            <span className="text-sm text-muted-foreground">
              {t("comingSoon.badge")}
            </span>
          </>
        }
        title={t("comingSoon.title")}
        subtitle={t("comingSoon.description").split("\n").map((line, i) => (
          <span key={i}>
            {i > 0 && <br className="hidden sm:block" />}
            {line}
          </span>
        ))}
      >
        {/* Progress indicator */}
        <div className="flex justify-center mt-10">
          <div className="flex items-center gap-3">
            <div className="flex gap-1.5">
              <span className="w-2.5 h-2.5 rounded-full bg-accent animate-pulse" />
              <span className="w-2.5 h-2.5 rounded-full bg-accent/60 animate-pulse [animation-delay:150ms]" />
              <span className="w-2.5 h-2.5 rounded-full bg-accent/30 animate-pulse [animation-delay:300ms]" />
            </div>
          </div>
        </div>
      </HeroSection>

      {/* Feature Preview Cards */}
      <SectionContainer size="lg">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          {FEATURES.map(({ icon: Icon, titleKey, descKey }) => (
            <div
              key={titleKey}
              className="group relative bg-card rounded-2xl p-8 shadow-card border text-center"
            >
              <div className="w-14 h-14 rounded-xl gradient-primary flex items-center justify-center mx-auto mb-6 opacity-60">
                <Icon className="h-7 w-7 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">
                {t(titleKey)}
              </h3>
              <p className="text-muted-foreground leading-relaxed break-keep">
                {t(descKey).split("\n").map((line, i) => (
                  <span key={i}>
                    {i > 0 && <br />}
                    {line}
                  </span>
                ))}
              </p>
            </div>
          ))}
        </div>

        {/* Notify hint */}
        <p className="text-center text-sm text-muted-foreground mt-10">
          <Sparkles className="inline h-4 w-4 mr-1 text-accent" />
          {t("comingSoon.notifyHint")}
        </p>
      </SectionContainer>

      {/* CTA: Explore available content */}
      <SectionContainer size="lg" className="bg-surface-inverted">
        <div className="text-center">
          <h2 className="text-2xl md:text-3xl font-bold text-surface-inverted-foreground mb-8">
            {t("comingSoon.title")}
          </h2>
          <div className="flex flex-col sm:flex-row justify-center gap-4">
            <Button
              size="lg"
              asChild
              className="gradient-primary hover:opacity-90 text-white shadow-lg hover:shadow-xl transition-all rounded-full px-8 h-14 text-base"
            >
              <Link to="/book/ebook">
                {t("comingSoon.exploreEbook")}
                <ArrowRight className="ml-2 h-5 w-5" />
              </Link>
            </Button>
            <Button
              size="lg"
              variant="outline"
              asChild
              className="rounded-full px-8 h-14 text-base border-2 border-surface-inverted-foreground/30 text-surface-inverted-foreground hover:bg-surface-inverted-foreground/10 hover:border-surface-inverted-foreground/50"
            >
              <Link to="/book/textbook">{t("comingSoon.exploreTextbook")}</Link>
            </Button>
          </div>
        </div>
      </SectionContainer>
    </div>
  );
}
