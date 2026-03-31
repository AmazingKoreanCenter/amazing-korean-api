import { useState, useCallback } from "react";
import { useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { BookOpen, LayoutGrid, Disc3, Search } from "lucide-react";

import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { HeroSection } from "@/components/blocks/hero_section";
import { SectionContainer } from "@/components/blocks/section_container";
import { PageMeta } from "@/components/page_meta";

import { useCatalog } from "../hook/use_catalog";
import { useCatalogView } from "../hook/use_catalog_view";
import type { CatalogItem, TextbookType } from "../types";
import { TextbookCarouselView } from "./textbook_carousel_view";
import { TextbookDetailModal } from "./textbook_detail_modal";

type ViewMode = "grid" | "carousel";

const STORAGE_KEY = "amk_catalog_view_mode";

function getStoredViewMode(): ViewMode {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored === "grid") return "grid";
  } catch {
    // localStorage unavailable
  }
  return "carousel";
}

function CoverCard({
  item,
  type,
  onClick,
}: {
  item: CatalogItem;
  type: "student" | "teacher";
  onClick: () => void;
}) {
  const { t, i18n } = useTranslation();
  const langName = i18n.language === "ko" ? item.language_name_ko : item.language_name_en;

  return (
    <button
      type="button"
      onClick={onClick}
      className="bg-card rounded-2xl overflow-hidden shadow-card hover:shadow-card-hover hover:-translate-y-1 transition-all duration-300 border hover:border-accent/50 text-left cursor-pointer"
    >
      <div className="aspect-[3/4] overflow-hidden bg-muted border-b">
        <img
          src={`/covers/${type}-${item.language}.webp`}
          alt={`${langName} ${type}`}
          className="w-full h-full object-cover"
          loading="lazy"
        />
      </div>
      <div className="p-4 space-y-2">
        <h3 className="font-semibold text-sm">{t("textbook.catalog.bookTitle", { language: langName })}</h3>
        <p className="text-xs text-muted-foreground text-right py-0.5">{t("textbook.catalog.pricePerUnit")}</p>
        <span className="inline-flex items-center justify-center w-full rounded-md bg-primary text-primary-foreground text-sm font-medium h-8 px-3">
          {t("textbook.detail.viewDetail")}
        </span>
      </div>
    </button>
  );
}

interface ModalTarget {
  item: CatalogItem;
  type: TextbookType;
}

export function TextbookCatalogPage() {
  const { t } = useTranslation();
  const { data: catalog, isLoading } = useCatalog();
  const [viewMode, setViewModeState] = useState<ViewMode>(getStoredViewMode);
  const [modalTarget, setModalTarget] = useState<ModalTarget | null>(null);

  const items = catalog?.items?.filter((item) => item.available) ?? [];

  const {
    activeType,
    setActiveType,
    searchQuery,
    setSearchQuery,
    filteredItems,
    selectedIndex,
    setSelectedIndex,
    selectedItem,
  } = useCatalogView(items);

  const setViewMode = useCallback((mode: ViewMode) => {
    setViewModeState(mode);
    try {
      localStorage.setItem(STORAGE_KEY, mode);
    } catch {
      // localStorage unavailable
    }
  }, []);

  if (isLoading) {
    return (
      <div className="flex flex-col">
        <PageMeta titleKey="textbook.catalog.title" descriptionKey="textbook.catalog.description" />
        <div className="max-w-container-default mx-auto px-6 py-20 space-y-6">
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

      {/* Common header: Tabs + Search + View toggle */}
      <SectionContainer>
        <Tabs
          value={activeType}
          onValueChange={(v) => setActiveType(v as TextbookType)}
        >
          <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
            <div className="flex items-center gap-3">
              <CatalogTypeToggle active="textbook" />
              <TabsList>
                <TabsTrigger value="student">{t("textbook.catalog.studentSection")}</TabsTrigger>
                <TabsTrigger value="teacher">{t("textbook.catalog.teacherSection")}</TabsTrigger>
              </TabsList>
            </div>

            <div className="flex items-center gap-2">
              {/* Search bar */}
              <div className="relative w-full sm:w-64">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                <Input
                  placeholder={t("textbook.catalog.searchPlaceholder")}
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="pl-9 h-10"
                />
              </div>

              {/* View toggle */}
              <div className="inline-flex rounded-lg border p-1 gap-1 flex-shrink-0">
                <button
                  type="button"
                  onClick={() => setViewMode("grid")}
                  className={`p-2 rounded-md transition-colors ${
                    viewMode === "grid"
                      ? "bg-primary text-primary-foreground"
                      : "hover:bg-muted"
                  }`}
                  title={t("textbook.catalog.viewGrid")}
                >
                  <LayoutGrid className="h-4 w-4" />
                </button>
                <button
                  type="button"
                  onClick={() => setViewMode("carousel")}
                  className={`p-2 rounded-md transition-colors ${
                    viewMode === "carousel"
                      ? "bg-primary text-primary-foreground"
                      : "hover:bg-muted"
                  }`}
                  title={t("textbook.catalog.viewCarousel")}
                >
                  <Disc3 className="h-4 w-4" />
                </button>
              </div>
            </div>
          </div>

          <TextbookDetailModal
            item={modalTarget?.item ?? null}
            type={modalTarget?.type ?? "student"}
            open={modalTarget !== null}
            onOpenChange={(open) => { if (!open) setModalTarget(null); }}
          />

          {viewMode === "grid" ? (
            <>
              <TabsContent value="student">
                <GridSection
                  items={filteredItems}
                  type="student"
                  noResultsText={t("textbook.catalog.noResults")}
                  onCardClick={(item) => setModalTarget({ item, type: "student" })}
                />
              </TabsContent>
              <TabsContent value="teacher">
                <GridSection
                  items={filteredItems}
                  type="teacher"
                  noResultsText={t("textbook.catalog.noResults")}
                  onCardClick={(item) => setModalTarget({ item, type: "teacher" })}
                />
              </TabsContent>
            </>
          ) : (
            <>
              <TabsContent value="student">
                <TextbookCarouselView
                  items={filteredItems}
                  type="student"
                  selectedIndex={selectedIndex}
                  onSelect={setSelectedIndex}
                  selectedItem={selectedItem}
                  onDetailOpen={(item, type) => setModalTarget({ item, type })}
                />
              </TabsContent>
              <TabsContent value="teacher">
                <TextbookCarouselView
                  items={filteredItems}
                  type="teacher"
                  selectedIndex={selectedIndex}
                  onSelect={setSelectedIndex}
                  selectedItem={selectedItem}
                  onDetailOpen={(item, type) => setModalTarget({ item, type })}
                />
              </TabsContent>
            </>
          )}
        </Tabs>
      </SectionContainer>

    </div>
  );
}

function GridSection({
  items,
  type,
  noResultsText,
  onCardClick,
}: {
  items: CatalogItem[];
  type: TextbookType;
  noResultsText: string;
  onCardClick: (item: CatalogItem) => void;
}) {
  if (items.length === 0) {
    return (
      <div className="py-16 text-center text-muted-foreground">
        {noResultsText}
      </div>
    );
  }

  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 md:gap-6 mt-6">
      {items.map((item) => (
        <CoverCard
          key={`${type}-${item.language}`}
          item={item}
          type={type}
          onClick={() => onCardClick(item)}
        />
      ))}
    </div>
  );
}

function CatalogTypeToggle({ active }: { active: "textbook" | "ebook" }) {
  const { t } = useTranslation();
  const navigate = useNavigate();

  return (
    <div className="inline-flex items-center rounded-lg border bg-muted p-1 gap-1">
      <button
        type="button"
        onClick={() => active !== "textbook" && navigate("/book/textbook")}
        className={`rounded-md px-3 py-1.5 text-sm font-medium transition-colors ${
          active === "textbook"
            ? "bg-background text-foreground shadow-sm"
            : "text-muted-foreground hover:text-foreground"
        }`}
      >
        {t("bookHub.tabTextbook")}
      </button>
      <button
        type="button"
        onClick={() => active !== "ebook" && navigate("/book/ebook")}
        className={`rounded-md px-3 py-1.5 text-sm font-medium transition-colors ${
          active === "ebook"
            ? "bg-background text-foreground shadow-sm"
            : "text-muted-foreground hover:text-foreground"
        }`}
      >
        {t("bookHub.tabEbook")}
      </button>
    </div>
  );
}
