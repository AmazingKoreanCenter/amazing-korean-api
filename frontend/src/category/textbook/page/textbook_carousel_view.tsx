import { useTranslation } from "react-i18next";

import type { CatalogItem, TextbookType } from "../types";
import { SealList } from "./seal_list";
import { SelectedBookDetail } from "./selected_book_detail";

interface TextbookCarouselViewProps {
  items: CatalogItem[];
  type: TextbookType;
  selectedIndex: number;
  onSelect: (index: number) => void;
  selectedItem: CatalogItem | null;
  onDetailOpen?: (item: CatalogItem, type: TextbookType) => void;
}

export function TextbookCarouselView({
  items,
  type,
  selectedIndex,
  onSelect,
  selectedItem,
  onDetailOpen,
}: TextbookCarouselViewProps) {
  const { t } = useTranslation();

  if (items.length === 0) {
    return (
      <div className="py-16 text-center text-muted-foreground">
        {t("textbook.catalog.noResults")}
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-8 mt-6">
      {/* Mobile: seal list first, Desktop: detail first */}
      <div className="md:order-1 order-2">
        {selectedItem && (
          <SelectedBookDetail item={selectedItem} type={type} onDetailOpen={onDetailOpen} />
        )}
      </div>
      <div className="md:order-2 order-1">
        <SealList items={items} selectedIndex={selectedIndex} onSelect={onSelect} />
      </div>
    </div>
  );
}
