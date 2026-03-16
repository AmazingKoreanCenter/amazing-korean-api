import { useMutation } from "@tanstack/react-query";

import { createTextbookOrder } from "../textbook_api";
import type { CreateOrderReq } from "../types";

export const useCreateOrder = () => {
  return useMutation({
    mutationFn: (data: CreateOrderReq) => createTextbookOrder(data),
  });
};
