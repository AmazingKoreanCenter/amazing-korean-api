import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { Play, BookOpen, Users, ArrowRight, CheckCircle2 } from "lucide-react";

import { Button } from "@/components/ui/button";
import { HeroSection } from "@/components/sections/hero_section";
import { SectionContainer } from "@/components/sections/section_container";
import { PageMeta } from "@/components/page_meta";

export default function HomePage() {
  const { t } = useTranslation();
  return (
    <div className="flex flex-col">
      <PageMeta titleKey="seo.home.title" descriptionKey="seo.home.description" />
      <HeroSection
        badge={
          <>
            <span className="w-2 h-2 rounded-full bg-status-success animate-pulse" />
            <span className="text-sm text-muted-foreground">
              {t("home.heroBadge")}
            </span>
          </>
        }
        title={t("home.heroTitle").split("\n").map((line, i) => (
          <span key={i}>
            {i > 0 && <br className="hidden sm:block" />}
            {line}
          </span>
        ))}
        subtitle={t("home.heroDescription").split("\n").map((line, i) => (
          <span key={i}>
            {i > 0 && <br className="hidden sm:block" />}
            {line}
          </span>
        ))}
      >
        {/* CTA Buttons */}
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
        <div className="flex flex-wrap justify-center gap-8 mt-12 pt-12 border-t">
          <div className="text-center">
            <div className="text-2xl font-bold text-primary">1,000+</div>
            <div className="text-sm text-muted-foreground">{t("home.statVideos")}</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-primary">50+</div>
            <div className="text-sm text-muted-foreground">{t("home.statInstructors")}</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-primary">10,000+</div>
            <div className="text-sm text-muted-foreground">{t("home.statStudents")}</div>
          </div>
        </div>
      </HeroSection>

      {/* Features Section */}
      <SectionContainer size="lg">
          {/* Section Header */}
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold mb-4">
              {t("home.featureTitle")}
            </h2>
            <p className="text-muted-foreground text-lg max-w-2xl mx-auto">
              {t("home.featureDescription")}
            </p>
          </div>

          {/* Feature Cards */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {/* Video Learning */}
            <div className="group relative bg-card rounded-2xl p-8 shadow-card hover:shadow-card-hover transition-all duration-300 border">
              <div className="w-14 h-14 rounded-xl gradient-primary flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                <Play className="h-7 w-7 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">{t("home.videoLearningTitle")}</h3>
              <p className="text-muted-foreground mb-6 leading-relaxed">
                {t("home.videoLearningDescription")}
              </p>
              <ul className="space-y-2 mb-6">
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>{t("home.videoFeature1")}</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>{t("home.videoFeature2")}</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>{t("home.videoFeature3")}</span>
                </li>
              </ul>
              <Button
                variant="ghost"
                asChild
                className="p-0 h-auto text-primary hover:text-primary/80 group-hover:translate-x-1 transition-transform"
              >
                <Link to="/videos" className="flex items-center gap-1">
                  {t("home.videoLink")} <ArrowRight className="h-4 w-4" />
                </Link>
              </Button>
            </div>

            {/* Structured Learning */}
            <div className="group relative bg-card rounded-2xl p-8 shadow-card hover:shadow-card-hover transition-all duration-300 border">
              <div className="w-14 h-14 rounded-xl gradient-primary flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                <BookOpen className="h-7 w-7 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">{t("home.structuredLearningTitle")}</h3>
              <p className="text-muted-foreground mb-6 leading-relaxed">
                {t("home.structuredLearningDescription")}
              </p>
              <ul className="space-y-2 mb-6">
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>{t("home.studyFeature1")}</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>{t("home.studyFeature2")}</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>{t("home.studyFeature3")}</span>
                </li>
              </ul>
              <Button
                variant="ghost"
                asChild
                className="p-0 h-auto text-primary hover:text-primary/80 group-hover:translate-x-1 transition-transform"
              >
                <Link to="/studies" className="flex items-center gap-1">
                  {t("home.studyLink")} <ArrowRight className="h-4 w-4" />
                </Link>
              </Button>
            </div>

            {/* 1:1 Lessons */}
            <div className="group relative bg-card rounded-2xl p-8 shadow-card hover:shadow-card-hover transition-all duration-300 border">
              <div className="w-14 h-14 rounded-xl gradient-primary flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                <Users className="h-7 w-7 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">{t("home.lessonTitle")}</h3>
              <p className="text-muted-foreground mb-6 leading-relaxed">
                {t("home.lessonDescription")}
              </p>
              <ul className="space-y-2 mb-6">
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>{t("home.lessonFeature1")}</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>{t("home.lessonFeature2")}</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>{t("home.lessonFeature3")}</span>
                </li>
              </ul>
              <Button
                variant="ghost"
                asChild
                className="p-0 h-auto text-primary hover:text-primary/80 group-hover:translate-x-1 transition-transform"
              >
                <Link to="/lessons" className="flex items-center gap-1">
                  {t("home.lessonLink")} <ArrowRight className="h-4 w-4" />
                </Link>
              </Button>
            </div>
          </div>
      </SectionContainer>

      {/* CTA Section */}
      <SectionContainer size="lg" className="bg-surface-inverted">
        <div className="text-center">
          <h2 className="text-3xl md:text-4xl font-bold text-surface-inverted-foreground mb-4">
            {t("home.ctaSectionTitle")}
          </h2>
          <p className="text-surface-inverted-foreground/70 text-lg max-w-xl mx-auto mb-10">
            {t("home.ctaSectionDescription").split("\n").map((line, i) => (
              <span key={i}>
                {i > 0 && <br />}
                {line}
              </span>
            ))}
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
