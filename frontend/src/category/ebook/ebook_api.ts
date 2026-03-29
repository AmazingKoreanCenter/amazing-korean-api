import { api, request } from "@/api/client";

import type {
  CreatePurchaseReq,
  EbookCatalogRes,
  MyPurchasesRes,
  PurchaseRes,
  ViewerMetaRes,
} from "./types";

// ─────────────────────── HMAC Signature (Web Crypto API) ───────────────────────

/** hex 문자열 → Uint8Array */
function hexToBytes(hex: string): Uint8Array {
  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < hex.length; i += 2) {
    bytes[i / 2] = parseInt(hex.substring(i, i + 2), 16);
  }
  return bytes;
}

/** Uint8Array → hex 문자열 */
function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

/**
 * HMAC-SHA256 서명 계산 (Web Crypto API).
 * payload = "{sessionId}:{path}:{timestamp}"
 * @returns { signature: hex, timestamp: unix-seconds string }
 */
async function computeHmacSignature(
  hmacSecretHex: string,
  sessionId: string,
  path: string
): Promise<{ signature: string; timestamp: string }> {
  const timestamp = Math.floor(Date.now() / 1000).toString();
  const payload = `${sessionId}:${path}:${timestamp}`;
  const keyBytes = hexToBytes(hmacSecretHex);
  const key = await crypto.subtle.importKey(
    "raw",
    keyBytes.buffer as ArrayBuffer,
    { name: "HMAC", hash: "SHA-256" },
    false,
    ["sign"]
  );
  const encoded = new TextEncoder().encode(payload);
  const sig = await crypto.subtle.sign("HMAC", key, encoded as BufferSource);
  return { signature: bytesToHex(new Uint8Array(sig)), timestamp };
}

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
 * HMAC 서명 포함: X-Ebook-Signature + X-Ebook-Timestamp
 */
export const fetchPageImage = async (
  code: string,
  page: number,
  sessionId?: string,
  hmacSecret?: string
): Promise<ArrayBuffer> => {
  let hmacHeaders: Record<string, string> = {};
  if (hmacSecret && sessionId) {
    const path = `${code}/${page}`;
    const { signature, timestamp } = await computeHmacSignature(hmacSecret, sessionId, path);
    hmacHeaders = {
      "X-Ebook-Signature": signature,
      "X-Ebook-Timestamp": timestamp,
    };
  }
  const res = await api.get(`/ebook/viewer/${code}/pages/${page}`, {
    responseType: "arraybuffer",
    headers: {
      "X-Ebook-Viewer": "1",
      ...(sessionId ? { "X-Ebook-Session": sessionId } : {}),
      ...hmacHeaders,
    },
  });
  return res.data as ArrayBuffer;
};

export const sendViewerHeartbeat = (sessionId: string) =>
  request<{ valid: boolean }>("/ebook/viewer/heartbeat", {
    method: "POST",
    data: { session_id: sessionId },
  });

/**
 * 타일 이미지를 ArrayBuffer로 가져옴 (타일 분할 모드)
 * HMAC 서명 포함: X-Ebook-Signature + X-Ebook-Timestamp
 */
export const fetchPageTile = async (
  code: string,
  page: number,
  row: number,
  col: number,
  sessionId?: string,
  hmacSecret?: string
): Promise<ArrayBuffer> => {
  let hmacHeaders: Record<string, string> = {};
  if (hmacSecret && sessionId) {
    const path = `${code}/${page}/${row}/${col}`;
    const { signature, timestamp } = await computeHmacSignature(hmacSecret, sessionId, path);
    hmacHeaders = {
      "X-Ebook-Signature": signature,
      "X-Ebook-Timestamp": timestamp,
    };
  }
  const res = await api.get(
    `/ebook/viewer/${code}/pages/${page}/tiles/${row}/${col}`,
    {
      responseType: "arraybuffer",
      headers: {
        "X-Ebook-Viewer": "1",
        ...(sessionId ? { "X-Ebook-Session": sessionId } : {}),
        ...hmacHeaders,
      },
    }
  );
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
