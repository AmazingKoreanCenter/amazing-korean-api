import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { BookOpen, ArrowRight, Package, CreditCard, Truck } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { HeroSection } from "@/components/sections/hero_section";
import { SectionContainer } from "@/components/sections/section_container";
import { PageMeta } from "@/components/page_meta";

import { useCatalog } from "../hook/use_catalog";
import type { CatalogItem } from "../types";

function CoverCard({ item, type }: { item: CatalogItem; type: "student" | "teacher" }) {
  const { t, i18n } = useTranslation();
  const langName = i18n.language === "ko" ? item.language_name_ko : item.language_name_en;

  return (
    <div className="bg-card rounded-2xl overflow-hidden shadow-card hover:shadow-card-hover hover:-translate-y-1 transition-all duration-300 border hover:border-accent/50">
      <div className="aspect-[3/4] overflow-hidden bg-muted">
        <img
          src={`/covers/${type}-${item.language}.webp`}
          alt={`${langName} ${type}`}
          className="w-full h-full object-cover"
          loading="lazy"
        />
      </div>
      <div className="p-4 space-y-2">
        <h3 className="font-semibold text-sm">{langName}</h3>
        <p className="text-xs text-muted-foreground">{t("textbook.catalog.pricePerUnit")}</p>
        <Button asChild className="w-full" size="sm">
          <Link to={`/textbook/order?lang=${item.language}&type=${type}`}>
            {t("textbook.catalog.orderButton")}
          </Link>
        </Button>
      </div>
    </div>
  );
}

export function TextbookCatalogPage() {
  const { t } = useTranslation();
  const { data: catalog, isLoading } = useCatalog();

  if (isLoading) {
    return (
      <div className="flex flex-col">
        <PageMeta titleKey="textbook.catalog.title" descriptionKey="textbook.catalog.description" />
        <div className="max-w-[1350px] mx-auto px-6 py-20 space-y-6">
          <Skeleton className="h-10 w-64 mx-auto" />
          <Skeleton className="h-6 w-96 mx-auto" />
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
            {Array.from({ length: 8 }).map((_, i) => (
              <Skeleton key={i} className="aspect-[3/4] rounded-2xl" />
            ))}
          </div>
        </div>
      </div>
    );
  }

  const items = catalog?.items?.filter((item) => item.available) ?? [];

  return (
    <div className="flex flex-col">
      <PageMeta titleKey="textbook.catalog.title" descriptionKey="textbook.catalog.description" />

      <HeroSection
        size="sm"
        badge={
          <>
            <BookOpen className="h-4 w-4 text-primary" />
            <span className="text-sm text-muted-foreground">{t("textbook.catalog.badge")}</span>
          </>
        }
        title={t("textbook.catalog.title")}
        subtitle={t("textbook.catalog.description").split("\n").map((line, i) => (
          <span key={i}>
            {i > 0 && <br className="hidden sm:block" />}
            {line}
          </span>
        ))}
      />

      {/* 학생용 교재 */}
      <SectionContainer>
        <div className="space-y-6">
          <div>
            <h2 className="text-2xl md:text-3xl font-bold">{t("textbook.catalog.studentSection")}</h2>
            <p className="text-muted-foreground mt-2">{t("textbook.catalog.studentDescription")}</p>
          </div>
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 md:gap-6">
            {items.map((item) => (
              <CoverCard key={`student-${item.language}`} item={item} type="student" />
            ))}
          </div>
        </div>
      </SectionContainer>

      {/* 교사용 교재 */}
      <SectionContainer>
        <div className="space-y-6">
          <div>
            <h2 className="text-2xl md:text-3xl font-bold">{t("textbook.catalog.teacherSection")}</h2>
            <p className="text-muted-foreground mt-2">{t("textbook.catalog.teacherDescription")}</p>
          </div>
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 md:gap-6">
            {items.map((item) => (
              <CoverCard key={`teacher-${item.language}`} item={item} type="teacher" />
            ))}
          </div>
        </div>
      </SectionContainer>

      {/* 주문 안내 */}
      <SectionContainer className="border-t">
        <div className="max-w-3xl mx-auto text-center space-y-8">
          <h2 className="text-2xl md:text-3xl font-bold">{t("textbook.catalog.orderGuideTitle")}</h2>
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-6">
            <div className="flex flex-col items-center gap-3 p-6 rounded-2xl bg-card shadow-card border">
              <div className="w-12 h-12 rounded-xl bg-primary/10 flex items-center justify-center">
                <Package className="h-6 w-6 text-primary" />
              </div>
              <p className="text-sm text-muted-foreground text-center">{t("textbook.catalog.orderGuideMinQty")}</p>
            </div>
            <div className="flex flex-col items-center gap-3 p-6 rounded-2xl bg-card shadow-card border">
              <div className="w-12 h-12 rounded-xl bg-primary/10 flex items-center justify-center">
                <CreditCard className="h-6 w-6 text-primary" />
              </div>
              <p className="text-sm text-muted-foreground text-center">{t("textbook.catalog.orderGuidePayment")}</p>
            </div>
            <div className="flex flex-col items-center gap-3 p-6 rounded-2xl bg-card shadow-card border">
              <div className="w-12 h-12 rounded-xl bg-primary/10 flex items-center justify-center">
                <Truck className="h-6 w-6 text-primary" />
              </div>
              <p className="text-sm text-muted-foreground text-center">{t("textbook.catalog.orderGuideBulk")}</p>
            </div>
          </div>
          <Button asChild size="lg" className="gradient-primary hover:opacity-90 text-white rounded-full px-8 h-12">
            <Link to="/textbook/order">
              {t("textbook.catalog.orderCta")}
              <ArrowRight className="ml-2 h-5 w-5" />
            </Link>
          </Button>
        </div>
      </SectionContainer>
    </div>
  );
}
