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

import { useEbookCatalog } from "../hook/use_ebook_catalog";
import { useEbookCatalogView } from "../hook/use_ebook_catalog_view";
import type { EbookCatalogItem, EbookEdition } from "../types";
import { EbookCarouselView } from "./ebook_carousel_view";
import { EbookDetailModal } from "./ebook_detail_modal";

type ViewMode = "grid" | "carousel";

const STORAGE_KEY = "amk_ebook_view_mode";

function getStoredViewMode(): ViewMode {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored === "grid") return "grid";
  } catch {
    // localStorage unavailable
  }
  return "carousel";
}

function EbookCoverCard({
  item,
  edition,
  onClick,
}: {
  item: EbookCatalogItem;
  edition: EbookEdition;
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
          src={`/covers/${edition}-${item.language}.webp`}
          alt={`${langName} ${edition}`}
          className="w-full h-full object-cover"
          loading="lazy"
        />
      </div>
      <div className="p-4 space-y-2">
        <h3 className="font-semibold text-sm">{t("ebook.catalog.bookTitle", { language: langName })}</h3>
        <p className="text-xs text-muted-foreground text-right py-0.5">{t("ebook.catalog.pricePerUnit")}</p>
        <span className="inline-flex items-center justify-center w-full rounded-md bg-primary text-primary-foreground text-sm font-medium h-8 px-3">
          {t("ebook.detail.viewDetail")}
        </span>
      </div>
    </button>
  );
}

interface ModalTarget {
  item: EbookCatalogItem;
  edition: EbookEdition;
}

export function EbookCatalogPage() {
  const { t } = useTranslation();
  const { data, isLoading } = useEbookCatalog();
  const [viewMode, setViewModeState] = useState<ViewMode>(getStoredViewMode);
  const [modalTarget, setModalTarget] = useState<ModalTarget | null>(null);

  const items = data?.items ?? [];

  const {
    activeEdition,
    setActiveEdition,
    searchQuery,
    setSearchQuery,
    filteredItems,
    selectedIndex,
    setSelectedIndex,
    selectedItem,
  } = useEbookCatalogView(items);

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
        <PageMeta titleKey="ebook.catalog.title" descriptionKey="ebook.catalog.subtitle" />
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
      <PageMeta titleKey="ebook.catalog.title" descriptionKey="ebook.catalog.subtitle" />

      <HeroSection
        size="sm"
        badge={
          <>
            <BookOpen className="h-4 w-4 text-primary" />
            <span className="text-sm text-muted-foreground">{t("ebook.catalog.badge")}</span>
          </>
        }
        title={t("ebook.catalog.title")}
        subtitle={t("ebook.catalog.subtitle").split("\n").map((line, i) => (
          <span key={i}>
            {i > 0 && <br className="hidden sm:block" />}
            {line}
          </span>
        ))}
      />

      {/* Common header: Tabs + Search + View toggle */}
      <SectionContainer>
        <Tabs
          value={activeEdition}
          onValueChange={(v) => setActiveEdition(v as EbookEdition)}
        >
          <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
            <div className="flex items-center gap-3">
              <CatalogTypeToggle active="ebook" />
              <TabsList>
                <TabsTrigger value="student">{t("ebook.catalog.studentEdition")}</TabsTrigger>
                <TabsTrigger value="teacher">{t("ebook.catalog.teacherEdition")}</TabsTrigger>
              </TabsList>
            </div>

            <div className="flex items-center gap-2">
              {/* Search bar */}
              <div className="relative w-full sm:w-64">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                <Input
                  placeholder={t("ebook.catalog.searchPlaceholder")}
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
                  title={t("ebook.catalog.viewGrid")}
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
                  title={t("ebook.catalog.viewCarousel")}
                >
                  <Disc3 className="h-4 w-4" />
                </button>
              </div>
            </div>
          </div>

          <EbookDetailModal
            item={modalTarget?.item ?? null}
            edition={modalTarget?.edition ?? "student"}
            open={modalTarget !== null}
            onOpenChange={(open) => { if (!open) setModalTarget(null); }}
          />

          {viewMode === "grid" ? (
            <>
              <TabsContent value="student">
                <EbookGridSection
                  items={filteredItems}
                  edition="student"
                  noResultsText={t("ebook.catalog.noResults")}
                  onCardClick={(item) => setModalTarget({ item, edition: "student" })}
                />
              </TabsContent>
              <TabsContent value="teacher">
                <EbookGridSection
                  items={filteredItems}
                  edition="teacher"
                  noResultsText={t("ebook.catalog.noResults")}
                  onCardClick={(item) => setModalTarget({ item, edition: "teacher" })}
                />
              </TabsContent>
            </>
          ) : (
            <>
              <TabsContent value="student">
                <EbookCarouselView
                  items={filteredItems}
                  edition="student"
                  selectedIndex={selectedIndex}
                  onSelect={setSelectedIndex}
                  selectedItem={selectedItem}
                  onDetailOpen={(item, edition) => setModalTarget({ item, edition })}
                />
              </TabsContent>
              <TabsContent value="teacher">
                <EbookCarouselView
                  items={filteredItems}
                  edition="teacher"
                  selectedIndex={selectedIndex}
                  onSelect={setSelectedIndex}
                  selectedItem={selectedItem}
                  onDetailOpen={(item, edition) => setModalTarget({ item, edition })}
                />
              </TabsContent>
            </>
          )}
        </Tabs>
      </SectionContainer>
    </div>
  );
}

function EbookGridSection({
  items,
  edition,
  noResultsText,
  onCardClick,
}: {
  items: EbookCatalogItem[];
  edition: EbookEdition;
  noResultsText: string;
  onCardClick: (item: EbookCatalogItem) => void;
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
        <EbookCoverCard
          key={`${edition}-${item.language}`}
          item={item}
          edition={edition}
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
