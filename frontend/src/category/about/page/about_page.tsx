import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { Target, Heart, Globe, ArrowRight, Sparkles } from "lucide-react";

import { Button } from "@/components/ui/button";
import { HeroSection } from "@/components/sections/hero_section";
import { SectionContainer } from "@/components/sections/section_container";

export function AboutPage() {
  const { t } = useTranslation();
  return (
    <div className="flex flex-col">
      <HeroSection
        badge={
          <>
            <Sparkles className="h-4 w-4 text-accent" />
            <span className="text-sm text-muted-foreground">{t("about.badge")}</span>
          </>
        }
        title={<span className="text-gradient">Amazing Korean</span>}
        subtitle={t("about.heroDescription").split("\n").map((line, i) => (
          <span key={i}>
            {i > 0 && <br className="hidden sm:block" />}
            {line}
          </span>
        ))}
      />

      {/* Mission Section */}
      <SectionContainer size="lg">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-20 items-center">
            <div>
              <h2 className="text-3xl md:text-4xl font-bold mb-6">
                {t("about.missionTitle")}
              </h2>
              <p className="text-muted-foreground text-lg leading-relaxed mb-6">
                {t("about.missionDescription1")}
              </p>
              <p className="text-muted-foreground text-lg leading-relaxed mb-8">
                {t("about.missionDescription2")}
              </p>
              <Button
                size="lg"
                asChild
                className="gradient-primary hover:opacity-90 text-white rounded-full px-8"
              >
                <Link to="/signup">
                  {t("about.startLearning")}
                  <ArrowRight className="ml-2 h-5 w-5" />
                </Link>
              </Button>
            </div>

            <div className="relative">
              <div className="bg-gradient-to-br from-secondary to-accent rounded-3xl p-10 text-white">
                <div className="text-6xl font-bold mb-2">2024</div>
                <div className="text-white/80 text-lg mb-8">{t("about.serviceStartYear")}</div>
                <div className="grid grid-cols-2 gap-6">
                  <div>
                    <div className="text-3xl font-bold">10,000+</div>
                    <div className="text-white/70">{t("about.statStudents")}</div>
                  </div>
                  <div>
                    <div className="text-3xl font-bold">50+</div>
                    <div className="text-white/70">{t("about.statInstructors")}</div>
                  </div>
                  <div>
                    <div className="text-3xl font-bold">1,000+</div>
                    <div className="text-white/70">{t("about.statContents")}</div>
                  </div>
                  <div>
                    <div className="text-3xl font-bold">30+</div>
                    <div className="text-white/70">{t("about.statCountries")}</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
      </SectionContainer>

      {/* Values Section */}
      <SectionContainer size="lg" className="bg-muted/30">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold mb-4">{t("about.coreValuesTitle")}</h2>
            <p className="text-muted-foreground text-lg max-w-2xl mx-auto">
              {t("about.coreValuesDescription")}
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <div className="bg-white rounded-2xl p-8 shadow-card text-center">
              <div className="w-16 h-16 rounded-2xl gradient-primary flex items-center justify-center mx-auto mb-6">
                <Target className="h-8 w-8 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">{t("about.valueEffective")}</h3>
              <p className="text-muted-foreground leading-relaxed">
                {t("about.valueEffectiveDesc")}
              </p>
            </div>

            <div className="bg-white rounded-2xl p-8 shadow-card text-center">
              <div className="w-16 h-16 rounded-2xl gradient-primary flex items-center justify-center mx-auto mb-6">
                <Heart className="h-8 w-8 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">{t("about.valueLearner")}</h3>
              <p className="text-muted-foreground leading-relaxed">
                {t("about.valueLearnerDesc")}
              </p>
            </div>

            <div className="bg-white rounded-2xl p-8 shadow-card text-center">
              <div className="w-16 h-16 rounded-2xl gradient-primary flex items-center justify-center mx-auto mb-6">
                <Globe className="h-8 w-8 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">{t("about.valueGlobal")}</h3>
              <p className="text-muted-foreground leading-relaxed">
                {t("about.valueGlobalDesc")}
              </p>
            </div>
          </div>
      </SectionContainer>

      {/* CTA Section */}
      <SectionContainer size="lg" className="bg-primary">
        <div className="text-center">
          <h2 className="text-3xl md:text-4xl font-bold text-white mb-4">
            {t("about.ctaTitle")}
          </h2>
          <p className="text-white/70 text-lg max-w-xl mx-auto mb-10">
            {t("about.ctaDescription")}
          </p>
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
              className="rounded-full px-8 h-14 text-base border-2 border-white/30 text-black hover:bg-white/10 hover:border-white/50"
            >
              <Link to="/videos">{t("about.ctaBrowseVideos")}</Link>
            </Button>
          </div>
        </div>
      </SectionContainer>
    </div>
  );
}
