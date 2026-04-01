import { useTranslation } from "react-i18next";

import { SealList } from "@/category/textbook/page/seal_list";
import type { EbookCatalogItem, EbookEdition } from "../types";
import { EbookSelectedDetail } from "./ebook_selected_detail";

interface EbookCarouselViewProps {
  items: EbookCatalogItem[];
  edition: EbookEdition;
  selectedIndex: number;
  onSelect: (index: number) => void;
  selectedItem: EbookCatalogItem | null;
  onDetailOpen?: (item: EbookCatalogItem, edition: EbookEdition) => void;
}

export function EbookCarouselView({
  items,
  edition,
  selectedIndex,
  onSelect,
  selectedItem,
  onDetailOpen,
}: EbookCarouselViewProps) {
  const { t } = useTranslation();

  if (items.length === 0) {
    return (
      <div className="py-8 md:py-16 text-center text-muted-foreground">
        {t("ebook.catalog.noResults")}
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-8 mt-6">
      {/* Mobile: seal list first, Desktop: detail first */}
      <div className="md:order-1 order-2">
        {selectedItem && (
          <EbookSelectedDetail item={selectedItem} edition={edition} onDetailOpen={onDetailOpen} />
        )}
      </div>
      <div className="md:order-2 order-1">
        <SealList items={items} selectedIndex={selectedIndex} onSelect={onSelect} />
      </div>
    </div>
  );
}
