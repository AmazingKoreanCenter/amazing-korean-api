import { ShieldX, ArrowRight } from "lucide-react";
import { Link, useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { HeroSection } from "@/components/sections/hero_section";
import { PageMeta } from "@/components/page_meta";

export function AccessDeniedPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <div className="flex flex-col">
      <PageMeta titleKey="error.accessDeniedTitle" descriptionKey="error.accessDeniedDescription" />

      <HeroSection
        badge={
          <>
            <ShieldX className="h-4 w-4 text-destructive" />
            <span className="text-sm text-muted-foreground">403</span>
          </>
        }
        title={t("error.accessDeniedTitle")}
        subtitle={t("error.accessDeniedDescription").split("\n").map((line, i) => (
          <span key={i}>
            {i > 0 && <br className="hidden sm:block" />}
            {line}
          </span>
        ))}
      >
        <p className="text-sm text-muted-foreground mt-6">
          {t("error.accessDeniedContact")}
        </p>
        <div className="flex flex-col sm:flex-row justify-center gap-4 mt-8">
          <Button
            size="lg"
            variant="outline"
            onClick={() => navigate(-1)}
            className="rounded-full px-8 h-12"
          >
            {t("common.previousPage")}
          </Button>
          <Button
            size="lg"
            asChild
            className="gradient-primary hover:opacity-90 text-white rounded-full px-8 h-12"
          >
            <Link to="/">
              {t("common.goHome")}
              <ArrowRight className="ml-2 h-5 w-5" />
            </Link>
          </Button>
        </div>
      </HeroSection>
    </div>
  );
}
