import { request } from "@/api/client";

import type { CatalogRes, CreateOrderReq, OrderRes } from "./types";

export const getTextbookCatalog = () =>
  request<CatalogRes>("/textbook/catalog");

export const createTextbookOrder = (data: CreateOrderReq) =>
  request<OrderRes>("/textbook/orders", { method: "POST", data });

export const getTextbookOrderByCode = (code: string) =>
  request<OrderRes>(`/textbook/orders/${code}`);
