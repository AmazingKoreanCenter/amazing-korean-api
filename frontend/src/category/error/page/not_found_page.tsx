import { FileQuestion, ArrowRight } from "lucide-react";
import { Link, useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { HeroSection } from "@/components/blocks/hero_section";
import { PageMeta } from "@/components/page_meta";

export function NotFoundPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <div className="flex flex-col">
      <PageMeta titleKey="error.notFoundTitle" descriptionKey="error.notFoundDescription" />

      <HeroSection
        badge={
          <>
            <FileQuestion className="h-4 w-4 text-muted-foreground" />
            <span className="text-sm text-muted-foreground">404</span>
          </>
        }
        title={t("error.notFoundTitle")}
        subtitle={t("error.notFoundDescription").split("\n").map((line, i) => (
          <span key={i}>
            {i > 0 && <br className="hidden sm:block" />}
            {line}
          </span>
        ))}
      >
        <p className="text-sm text-muted-foreground mt-6">
          {t("error.notFoundCheckUrl")}
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
