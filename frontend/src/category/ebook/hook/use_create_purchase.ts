import { useMutation } from "@tanstack/react-query";

import { createEbookPurchase } from "../ebook_api";
import type { CreatePurchaseReq } from "../types";

export const useCreateEbookPurchase = () => {
  return useMutation({
    mutationFn: (data: CreatePurchaseReq) => createEbookPurchase(data),
  });
};
