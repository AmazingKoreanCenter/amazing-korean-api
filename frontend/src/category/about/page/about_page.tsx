import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { Lightbulb, Timer, Languages, ArrowRight, Sparkles } from "lucide-react";

import { Button } from "@/components/ui/button";
import { FeatureGrid } from "@/components/blocks/feature_grid";
import { HeroSection } from "@/components/blocks/hero_section";
import { SectionContainer } from "@/components/blocks/section_container";
import { PageMeta } from "@/components/page_meta";

export function AboutPage() {
  const { t } = useTranslation();
  return (
    <div className="flex flex-col">
      <PageMeta titleKey="seo.about.title" descriptionKey="seo.about.description" />

      {/* ─── Hero ─── */}
      <HeroSection
        badge={
          <>
            <Sparkles className="h-4 w-4 text-accent" />
            <span className="text-sm text-muted-foreground">{t("about.badge")}</span>
          </>
        }
        title={<span className="text-gradient">{t("about.heroTitle")}</span>}
        subtitle={
          <span className="text-lg md:text-xl">{t("about.heroDescription")}</span>
        }
      />

      {/* ─── Why — Amazing Korean란? ─── */}
      <SectionContainer size="lg">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-20 items-center">
          <div>
            <h2 className="text-3xl md:text-4xl font-bold mb-8">
              {t("about.missionTitle")}
            </h2>
            <div className="space-y-5">
              <p className="text-muted-foreground text-lg leading-relaxed whitespace-pre-line">
                {t("about.missionDescription1")}
              </p>
              <p className="text-foreground text-lg leading-relaxed font-medium whitespace-pre-line">
                {t("about.missionDescription2")}
              </p>
              <p className="text-muted-foreground text-lg leading-relaxed whitespace-pre-line">
                {t("about.missionDescription3")}
              </p>
              <p className="text-foreground text-lg leading-relaxed font-semibold">
                {t("about.missionDescription4")}
              </p>
            </div>
            <Button
              size="lg"
              asChild
              className="gradient-primary hover:opacity-90 text-white rounded-full px-8 mt-8"
            >
              <Link to="/signup">
                {t("about.startLearning")}
                <ArrowRight className="ml-2 h-5 w-5" />
              </Link>
            </Button>
          </div>

          {/* Stats Card */}
          <div className="relative">
            <div className="bg-gradient-to-br from-secondary to-accent rounded-3xl p-6 md:p-10 text-white space-y-8">
              <div>
                <div className="text-2xl font-bold mb-2">{t("about.stat1Title")}</div>
                <div className="text-white/70 leading-relaxed">{t("about.stat1Desc")}</div>
              </div>
              <div className="border-t border-white/20 pt-8">
                <div className="text-2xl font-bold mb-2">{t("about.stat2Title")}</div>
                <div className="text-white/70 leading-relaxed">{t("about.stat2Desc")}</div>
              </div>
            </div>
          </div>
        </div>
      </SectionContainer>

      {/* ─── Values (Why & How) — 3 Cards Detail ─── */}
      <SectionContainer size="lg" className="bg-muted/30">
        <div className="text-center mb-16">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">{t("about.valueTitle")}</h2>
          <p className="text-muted-foreground text-lg max-w-2xl mx-auto">
            {t("about.valueDescription")}
          </p>
        </div>

        <FeatureGrid
          items={[
            {
              icon: <Lightbulb className="h-8 w-8 text-white" />,
              title: t("about.valueAcquisitionTitle"),
              description: t("about.valueAcquisitionDesc"),
            },
            {
              icon: <Timer className="h-8 w-8 text-white" />,
              title: t("about.valueEfficiencyTitle"),
              description: t("about.valueEfficiencyDesc"),
            },
            {
              icon: <Languages className="h-8 w-8 text-white" />,
              title: t("about.valueUnderstandingTitle"),
              description: t("about.valueUnderstandingDesc"),
            },
          ]}
        />
      </SectionContainer>

      {/* ─── CTA ─── */}
      <SectionContainer size="lg" className="bg-surface-inverted">
        <div className="text-center">
          <h2 className="text-3xl md:text-4xl font-bold text-surface-inverted-foreground mb-10">
            {t("about.ctaTitle")}
          </h2>
          <div className="flex flex-col sm:flex-row justify-center gap-4">
            <Button
              size="lg"
              asChild
              className="gradient-primary hover:opacity-90 text-white shadow-lg rounded-full px-8 h-14 text-base"
            >
              <Link to="/signup">
                {t("about.ctaStart")}
                <ArrowRight className="ml-2 h-5 w-5" />
              </Link>
            </Button>
            <Button
              size="lg"
              variant="outline"
              asChild
              className="rounded-full px-8 h-14 text-base border-2 border-surface-inverted-foreground/30 text-surface-inverted-foreground hover:bg-surface-inverted-foreground/10 hover:border-surface-inverted-foreground/50"
            >
              <Link to="/book">{t("about.ctaBrowseBooks")}</Link>
            </Button>
          </div>
        </div>
      </SectionContainer>
    </div>
  );
}
