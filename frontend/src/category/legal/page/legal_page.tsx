import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { ArrowLeft } from "lucide-react";

interface LegalSection {
  titleKey: string;
  contentKey: string;
}

interface LegalPageProps {
  pageKey: string;
  sections: LegalSection[];
}

export function LegalPage({ pageKey, sections }: LegalPageProps) {
  const { t } = useTranslation();

  return (
    <div className="min-h-screen bg-gradient-to-b from-muted to-background">
      <div className="max-w-3xl mx-auto px-6 lg:px-8 py-12 lg:py-20">
        {/* Back link */}
        <Link
          to="/"
          className="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground transition-colors mb-8"
        >
          <ArrowLeft className="h-4 w-4" />
          {t("common.goHome")}
        </Link>

        {/* Title */}
        <h1 className="text-3xl md:text-4xl font-bold mb-3">
          {t(`legal.${pageKey}.title`)}
        </h1>
        <p className="text-sm text-muted-foreground mb-10">
          {t("legal.lastUpdated")}: {t(`legal.${pageKey}.updatedAt`)}
        </p>

        {/* Intro */}
        <p className="text-muted-foreground leading-relaxed mb-10 border-l-4 border-primary/30 pl-4">
          {t(`legal.${pageKey}.intro`)}
        </p>

        {/* Sections */}
        <div className="space-y-8">
          {sections.map((section, idx) => (
            <section key={idx}>
              <h2 className="text-lg font-semibold mb-3">
                {t(section.titleKey)}
              </h2>
              <p className="text-muted-foreground leading-relaxed whitespace-pre-line">
                {t(section.contentKey)}
              </p>
            </section>
          ))}
        </div>

        {/* Contact */}
        <div className="mt-12 pt-8 border-t">
          <p className="text-sm text-muted-foreground">
            {t("legal.contactInfo")}:{" "}
            <a
              href="mailto:amazingkoreancenter@gmail.com"
              className="text-primary hover:underline"
            >
              amazingkoreancenter@gmail.com
            </a>
          </p>
        </div>
      </div>
    </div>
  );
}
