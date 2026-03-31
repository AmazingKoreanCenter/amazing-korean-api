import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import {
  ArrowRight,
  CheckCircle2,
  Lightbulb,
  Timer,
  Heart,
  BookOpen,
  Smartphone,
  Play,
  PenTool,
  Clock,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import { FeatureGrid } from "@/components/blocks/feature_grid";
import { HeroSection } from "@/components/blocks/hero_section";
import { SectionContainer } from "@/components/blocks/section_container";
import { PageMeta } from "@/components/page_meta";

export default function HomePage() {
  const { t } = useTranslation();
  return (
    <div className="flex flex-col">
      <PageMeta titleKey="seo.home.title" descriptionKey="seo.home.description" />

      {/* ─── Hero ─── */}
      <HeroSection
        badge={
          <>
            <span className="w-2 h-2 rounded-full bg-status-success animate-pulse" />
            <span className="text-sm text-muted-foreground">
              {t("home.heroBadge")}
            </span>
          </>
        }
        title={t("home.heroTitle")}
        subtitle={
          <span className="text-lg md:text-xl">{t("home.heroDescription")}</span>
        }
      >
        <div className="flex flex-col sm:flex-row justify-center gap-4 mt-10">
          <Button
            size="lg"
            asChild
            className="gradient-primary hover:opacity-90 text-white shadow-lg hover:shadow-xl transition-all rounded-full px-8 h-14 text-base"
          >
            <Link to="/signup">
              {t("home.ctaStart")}
              <ArrowRight className="ml-2 h-5 w-5" />
            </Link>
          </Button>
          <Button
            size="lg"
            variant="outline"
            asChild
            className="rounded-full px-8 h-14 text-base border-2 hover:bg-muted/50"
          >
            <Link to="/about">{t("home.ctaAbout")}</Link>
          </Button>
        </div>

        {/* Trust Indicators */}
        <div className="grid grid-cols-3 mt-12 pt-12 border-t max-w-2xl mx-auto w-full">
          <div className="flex flex-col items-center">
            <div className="text-2xl font-bold text-gradient">{t("home.stat1Value")}</div>
            <div className="text-sm text-muted-foreground">{t("home.stat1Label")}</div>
          </div>
          <div className="flex flex-col items-center">
            <div className="text-2xl font-bold text-gradient">{t("home.stat2Value")}</div>
            <div className="text-sm text-muted-foreground">{t("home.stat2Label")}</div>
          </div>
          <div className="flex flex-col items-center">
            <div className="text-2xl font-bold text-gradient">{t("home.stat3Value")}</div>
            <div className="text-sm text-muted-foreground">{t("home.stat3Label")}</div>
          </div>
        </div>
      </HeroSection>

      {/* ─── Core Values (What) — 3 Cards ─── */}
      <SectionContainer size="lg">
        <div className="text-center mb-16">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            {t("home.valueTitle")}
          </h2>
          <p className="text-muted-foreground text-lg max-w-2xl mx-auto">
            {t("home.valueDescription")}
          </p>
        </div>

        <FeatureGrid
          items={[
            {
              icon: <Lightbulb className="h-8 w-8 text-white" />,
              title: t("home.valueAcquisitionTitle"),
              description: t("home.valueAcquisitionDesc"),
            },
            {
              icon: <Timer className="h-8 w-8 text-white" />,
              title: t("home.valueEfficiencyTitle"),
              description: t("home.valueEfficiencyDesc"),
            },
            {
              icon: <Heart className="h-8 w-8 text-white" />,
              title: t("home.valueUnderstandingTitle"),
              description: t("home.valueUnderstandingDesc"),
            },
          ]}
        />
      </SectionContainer>

      {/* ─── Features — 4 Cards ─── */}
      <SectionContainer size="lg" className="bg-muted/30">
        <div className="text-center mb-16">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            {t("home.featureTitle")}
          </h2>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          {/* Textbook */}
          <div className="group relative bg-card rounded-2xl p-6 shadow-card hover:shadow-card-hover hover:border-accent/50 transition-all duration-300 border">
            <div className="w-12 h-12 rounded-xl gradient-primary flex items-center justify-center mb-5 group-hover:scale-110 transition-transform">
              <BookOpen className="h-6 w-6 text-white" />
            </div>
            <h3 className="text-lg font-semibold mb-2">{t("home.textbookTitle")}</h3>
            <p className="text-muted-foreground text-sm mb-4 leading-relaxed">
              {t("home.textbookDesc")}
            </p>
            <ul className="space-y-1.5 mb-5">
              <li className="flex items-center gap-2 text-sm text-muted-foreground">
                <CheckCircle2 className="h-3.5 w-3.5 text-accent shrink-0" />
                <span>{t("home.textbookFeature1")}</span>
              </li>
              <li className="flex items-center gap-2 text-sm text-muted-foreground">
                <CheckCircle2 className="h-3.5 w-3.5 text-accent shrink-0" />
                <span>{t("home.textbookFeature2")}</span>
              </li>
            </ul>
            <Button
              variant="ghost"
              asChild
              className="p-0 h-auto text-accent font-medium hover:text-accent/80 group-hover:translate-x-1 transition-transform"
            >
              <Link to="/book/textbook" className="flex items-center gap-1">
                {t("home.textbookLink")} <ArrowRight className="h-4 w-4" />
              </Link>
            </Button>
          </div>

          {/* E-book */}
          <div className="group relative bg-card rounded-2xl p-6 shadow-card hover:shadow-card-hover hover:border-accent/50 transition-all duration-300 border">
            <div className="w-12 h-12 rounded-xl gradient-primary flex items-center justify-center mb-5 group-hover:scale-110 transition-transform">
              <Smartphone className="h-6 w-6 text-white" />
            </div>
            <h3 className="text-lg font-semibold mb-2">{t("home.ebookTitle")}</h3>
            <p className="text-muted-foreground text-sm mb-4 leading-relaxed">
              {t("home.ebookDesc")}
            </p>
            <ul className="space-y-1.5 mb-5">
              <li className="flex items-center gap-2 text-sm text-muted-foreground">
                <CheckCircle2 className="h-3.5 w-3.5 text-accent shrink-0" />
                <span>{t("home.ebookFeature1")}</span>
              </li>
              <li className="flex items-center gap-2 text-sm text-muted-foreground">
                <CheckCircle2 className="h-3.5 w-3.5 text-accent shrink-0" />
                <span>{t("home.ebookFeature2")}</span>
              </li>
            </ul>
            <Button
              variant="ghost"
              asChild
              className="p-0 h-auto text-accent font-medium hover:text-accent/80 group-hover:translate-x-1 transition-transform"
            >
              <Link to="/book/ebook" className="flex items-center gap-1">
                {t("home.ebookLink")} <ArrowRight className="h-4 w-4" />
              </Link>
            </Button>
          </div>

          {/* Video — Coming Soon */}
          <div className="group relative bg-card rounded-2xl p-6 shadow-card hover:shadow-card-hover hover:border-accent/50 transition-all duration-300 border">
            <div className="flex items-center justify-between mb-5">
              <div className="w-12 h-12 rounded-xl gradient-primary flex items-center justify-center group-hover:scale-110 transition-transform">
                <Play className="h-6 w-6 text-white" />
              </div>
              <span className="inline-flex items-center gap-1 rounded-full bg-amber-100 dark:bg-amber-900/30 px-2.5 py-1 text-xs font-medium text-amber-700 dark:text-amber-400">
                <Clock className="h-3 w-3" />
                {t("home.videoComingSoon")}
              </span>
            </div>
            <h3 className="text-lg font-semibold mb-2">{t("home.videoTitle")}</h3>
            <p className="text-muted-foreground text-sm mb-4 leading-relaxed">
              {t("home.videoDesc")}
            </p>
            <ul className="space-y-1.5 mb-5">
              <li className="flex items-center gap-2 text-sm text-muted-foreground">
                <CheckCircle2 className="h-3.5 w-3.5 text-accent shrink-0" />
                <span>{t("home.videoFeature1")}</span>
              </li>
              <li className="flex items-center gap-2 text-sm text-muted-foreground">
                <CheckCircle2 className="h-3.5 w-3.5 text-accent shrink-0" />
                <span>{t("home.videoFeature2")}</span>
              </li>
            </ul>
            <span className="text-sm text-muted-foreground/50 flex items-center gap-1">
              {t("home.videoLink")} <ArrowRight className="h-4 w-4" />
            </span>
          </div>

          {/* Study — Coming Soon */}
          <div className="group relative bg-card rounded-2xl p-6 shadow-card hover:shadow-card-hover hover:border-accent/50 transition-all duration-300 border">
            <div className="flex items-center justify-between mb-5">
              <div className="w-12 h-12 rounded-xl gradient-primary flex items-center justify-center group-hover:scale-110 transition-transform">
                <PenTool className="h-6 w-6 text-white" />
              </div>
              <span className="inline-flex items-center gap-1 rounded-full bg-amber-100 dark:bg-amber-900/30 px-2.5 py-1 text-xs font-medium text-amber-700 dark:text-amber-400">
                <Clock className="h-3 w-3" />
                {t("home.studyComingSoon")}
              </span>
            </div>
            <h3 className="text-lg font-semibold mb-2">{t("home.studyTitle")}</h3>
            <p className="text-muted-foreground text-sm mb-4 leading-relaxed">
              {t("home.studyDesc")}
            </p>
            <ul className="space-y-1.5 mb-5">
              <li className="flex items-center gap-2 text-sm text-muted-foreground">
                <CheckCircle2 className="h-3.5 w-3.5 text-accent shrink-0" />
                <span>{t("home.studyFeature1")}</span>
              </li>
              <li className="flex items-center gap-2 text-sm text-muted-foreground">
                <CheckCircle2 className="h-3.5 w-3.5 text-accent shrink-0" />
                <span>{t("home.studyFeature2")}</span>
              </li>
            </ul>
            <span className="text-sm text-muted-foreground/50 flex items-center gap-1">
              {t("home.studyLink")} <ArrowRight className="h-4 w-4" />
            </span>
          </div>
        </div>
      </SectionContainer>

      {/* ─── CTA ─── */}
      <SectionContainer size="lg" className="bg-surface-inverted">
        <div className="text-center">
          <h2 className="text-3xl md:text-4xl font-bold text-surface-inverted-foreground mb-4">
            {t("home.ctaSectionTitle")}
          </h2>
          <p className="text-surface-inverted-foreground/70 text-lg max-w-xl mx-auto mb-10">
            {t("home.ctaSectionDescription")}
          </p>
          <div className="flex flex-col sm:flex-row justify-center gap-4">
            <Button
              size="lg"
              asChild
              className="gradient-primary hover:opacity-90 text-white shadow-lg hover:shadow-xl transition-all rounded-full px-8 h-14 text-base"
            >
              <Link to="/signup">
                {t("home.ctaStart")}
                <ArrowRight className="ml-2 h-5 w-5" />
              </Link>
            </Button>
            <Button
              size="lg"
              variant="outline"
              asChild
              className="rounded-full px-8 h-14 text-base border-2 border-surface-inverted-foreground/30 text-surface-inverted-foreground hover:bg-surface-inverted-foreground/10 hover:border-surface-inverted-foreground/50"
            >
              <Link to="/login">{t("home.ctaAlreadyHaveAccount")}</Link>
            </Button>
          </div>
        </div>
      </SectionContainer>
    </div>
  );
}
