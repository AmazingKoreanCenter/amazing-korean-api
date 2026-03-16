import { useQuery } from "@tanstack/react-query";

import { getEbookCatalog } from "../ebook_api";

export const useEbookCatalog = () => {
  return useQuery({
    queryKey: ["ebook", "catalog"],
    queryFn: getEbookCatalog,
    staleTime: 10 * 60 * 1000,
  });
};
