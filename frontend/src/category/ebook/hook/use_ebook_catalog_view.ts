import { useState, useMemo, useCallback } from "react";
import type { EbookCatalogItem, EbookEdition } from "../types";

export function useEbookCatalogView(items: EbookCatalogItem[]) {
  const [activeEdition, setActiveEdition] = useState<EbookEdition>("student");
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
    activeEdition,
    setActiveEdition,
    searchQuery,
    setSearchQuery: handleSearchChange,
    filteredItems,
    selectedIndex,
    setSelectedIndex,
    selectedItem,
  };
}
