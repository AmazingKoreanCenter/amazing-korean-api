import { useState, useMemo, useCallback } from "react";
import type { CatalogItem, TextbookType } from "../types";

export function useCatalogView(items: CatalogItem[]) {
  const [activeType, setActiveType] = useState<TextbookType>("student");
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);

  const filteredItems = useMemo(() => {
    if (!searchQuery.trim()) return items;
    const q = searchQuery.trim().toLowerCase();
    return items.filter(
      (item) =>
        item.language_name_ko.toLowerCase().includes(q) ||
        item.language_name_en.toLowerCase().includes(q) ||
        item.language.toLowerCase().includes(q),
    );
  }, [items, searchQuery]);

  const selectedItem = filteredItems[selectedIndex] ?? filteredItems[0] ?? null;

  const handleSearchChange = useCallback((query: string) => {
    setSearchQuery(query);
    setSelectedIndex(0);
  }, []);

  return {
    activeType,
    setActiveType,
    searchQuery,
    setSearchQuery: handleSearchChange,
    filteredItems,
    selectedIndex,
    setSelectedIndex,
    selectedItem,
  };
}
