import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { Play, BookOpen, Users, ArrowRight, CheckCircle2 } from "lucide-react";

import { Button } from "@/components/ui/button";

export default function HomePage() {
  const { t } = useTranslation();
  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="relative overflow-hidden bg-gradient-to-br from-[#F0F3FF] via-white to-[#E8F4FF]">
        {/* Background Decoration */}
        <div className="absolute inset-0 overflow-hidden">
          <div className="absolute -top-40 -right-40 w-80 h-80 bg-[#129DD8]/10 rounded-full blur-3xl" />
          <div className="absolute -bottom-40 -left-40 w-80 h-80 bg-[#4F71EB]/10 rounded-full blur-3xl" />
        </div>

        <div className="relative max-w-[1350px] mx-auto px-6 lg:px-8 py-20 lg:py-32">
          <div className="max-w-3xl mx-auto text-center">
            {/* Badge */}
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-white shadow-sm border mb-8">
              <span className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
              <span className="text-sm text-muted-foreground">
                {t("home.heroBadge")}
              </span>
            </div>

            {/* Main Heading */}
            <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight mb-6">
              {t("home.heroTitle").split("\n").map((line, i) => (
                <span key={i}>
                  {i > 0 && <br className="hidden sm:block" />}
                  {line}
                </span>
              ))}
            </h1>

            {/* Description */}
            <p className="text-lg md:text-xl text-muted-foreground max-w-2xl mx-auto mb-10 leading-relaxed">
              {t("home.heroDescription").split("\n").map((line, i) => (
                <span key={i}>
                  {i > 0 && <br className="hidden sm:block" />}
                  {line}
                </span>
              ))}
            </p>

            {/* CTA Buttons */}
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
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 lg:py-28">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8">
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
            <div className="group relative bg-white rounded-2xl p-8 shadow-card hover:shadow-card-hover transition-all duration-300 border">
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
            <div className="group relative bg-white rounded-2xl p-8 shadow-card hover:shadow-card-hover transition-all duration-300 border">
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
            <div className="group relative bg-white rounded-2xl p-8 shadow-card hover:shadow-card-hover transition-all duration-300 border">
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
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 lg:py-28 bg-[#051D55]">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8 text-center">
          <h2 className="text-3xl md:text-4xl font-bold text-white mb-4">
            {t("home.ctaSectionTitle")}
          </h2>
          <p className="text-white/70 text-lg max-w-xl mx-auto mb-10">
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
              className="rounded-full px-8 h-14 text-base border-2 border-white/30 text-black hover:bg-white/10 hover:border-white/50"
            >
              <Link to="/login">{t("home.ctaAlreadyHaveAccount")}</Link>
            </Button>
          </div>
        </div>
      </section>
    </div>
  );
}
