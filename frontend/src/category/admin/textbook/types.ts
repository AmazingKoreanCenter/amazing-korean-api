import type { TextbookOrderStatus } from "@/category/textbook/types";

// =============================================================================
// 공통
// =============================================================================

export interface AdminTextbookMeta {
  total_count: number;
  total_pages: number;
  current_page: number;
  per_page: number;
}

// =============================================================================
// 주문 목록
// =============================================================================

export interface AdminTextbookListReq {
  page?: number;
  size?: number;
  q?: string;
  status?: string;
}

export interface AdminTextbookListRes {
  items: import("@/category/textbook/types").OrderRes[];
  meta: AdminTextbookMeta;
}

// =============================================================================
// 상태 변경
// =============================================================================

export interface AdminUpdateStatusReq {
  status: TextbookOrderStatus;
}
