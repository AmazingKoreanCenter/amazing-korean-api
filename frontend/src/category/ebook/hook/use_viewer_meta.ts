import { useQuery } from "@tanstack/react-query";

import { getViewerMeta } from "../ebook_api";

export const useViewerMeta = (code: string) => {
  return useQuery({
    queryKey: ["ebook", "viewer", code],
    queryFn: () => getViewerMeta(code),
    enabled: !!code,
  });
};
