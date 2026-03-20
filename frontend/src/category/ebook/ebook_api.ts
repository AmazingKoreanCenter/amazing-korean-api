import { api, request } from "@/api/client";

import type {
  CreatePurchaseReq,
  EbookCatalogRes,
  MyPurchasesRes,
  PurchaseRes,
  ViewerMetaRes,
} from "./types";

export const getEbookCatalog = () =>
  request<EbookCatalogRes>("/ebook/catalog");

export const createEbookPurchase = (data: CreatePurchaseReq) =>
  request<PurchaseRes>("/ebook/purchase", { method: "POST", data });

export const getMyPurchases = () =>
  request<MyPurchasesRes>("/ebook/my");

export const cancelEbookPurchase = (code: string) =>
  request<void>(`/ebook/purchase/${code}`, { method: "DELETE" });

export const getViewerMeta = (code: string) =>
  request<ViewerMetaRes>(`/ebook/viewer/${code}/meta`);

/**
 * 페이지 이미지를 ArrayBuffer로 가져옴 (Canvas 렌더링용, blob URL 미노출)
 */
export const fetchPageImage = async (
  code: string,
  page: number
): Promise<ArrayBuffer> => {
  const res = await api.get(`/ebook/viewer/${code}/pages/${page}`, {
    responseType: "arraybuffer",
    headers: { "X-Ebook-Viewer": "1" },
  });
  return res.data as ArrayBuffer;
};

// ─────────────────────── Admin ───────────────────────

import type { AdminEbookListRes, EbookPurchaseStatus } from "./types";

export const getAdminEbookPurchases = (params: {
  page?: number;
  per_page?: number;
  status?: string;
  search?: string;
}) => request<AdminEbookListRes>("/admin/ebook/purchases", { params });

export const getAdminEbookPurchase = (id: number) =>
  request<import("./types").AdminEbookPurchaseItem>(
    `/admin/ebook/purchases/${id}`
  );

export const updateAdminEbookStatus = (
  id: number,
  data: { status: EbookPurchaseStatus }
) =>
  request<import("./types").AdminEbookPurchaseItem>(
    `/admin/ebook/purchases/${id}/status`,
    { method: "PATCH", data }
  );

export const deleteAdminEbookPurchase = (id: number) =>
  request<{ message: string }>(`/admin/ebook/purchases/${id}`, {
    method: "DELETE",
  });
