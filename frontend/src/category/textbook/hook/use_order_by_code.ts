import { useQuery } from "@tanstack/react-query";

import { getTextbookOrderByCode } from "../textbook_api";

export const useOrderByCode = (code: string) => {
  return useQuery({
    queryKey: ["textbook", "order", code],
    queryFn: () => getTextbookOrderByCode(code),
    enabled: !!code,
  });
};
