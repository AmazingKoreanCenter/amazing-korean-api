import { useQuery } from "@tanstack/react-query";

import { getTextbookCatalog } from "../textbook_api";

export const useCatalog = () => {
  return useQuery({
    queryKey: ["textbook", "catalog"],
    queryFn: getTextbookCatalog,
    staleTime: 10 * 60 * 1000,
  });
};
