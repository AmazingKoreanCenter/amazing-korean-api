import { useQuery } from "@tanstack/react-query";

import { getMyTextbookOrders } from "../textbook_api";

export const useMyTextbookOrders = () => {
  return useQuery({
    queryKey: ["textbook", "my-orders"],
    queryFn: getMyTextbookOrders,
  });
};
