import type {
  CreateOrderItemReq,
  TextbookOrderStatus,
  TextbookPaymentMethod,
} from "@/category/textbook/types";

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

// =============================================================================
// 배송 추적 정보 업데이트
// =============================================================================

export interface AdminUpdateTrackingReq {
  tracking_number?: string;
  tracking_provider?: string;
}

// =============================================================================
// 관리자 대리 주문 생성 (POST /admin/textbook/orders)
// =============================================================================

export interface AdminCreateOrderReq {
  /** 주문을 귀속시킬 사용자 id (선택). 없으면 비회원 주문. */
  user_id?: number;
  /** 초기 상태 (기본 pending). paid 로 지정 시 paid_at 자동 세팅. */
  initial_status?: "pending" | "confirmed" | "paid";
  /** 최소 수량(10권) 제약 강제 여부 (기본 false). */
  enforce_min_quantity?: boolean;

  /** 신청자 정보. 2026-04-23: 이메일은 optional (오프라인·전화 주문 대응). */
  orderer_name: string;
  orderer_email?: string;
  orderer_phone: string;

  /** 기관 정보 (선택) */
  org_name?: string;
  org_type?: string;

  /** 배송 정보 */
  delivery_postal_code?: string;
  delivery_address: string;
  delivery_detail?: string;

  /** 결제 정보 */
  payment_method: TextbookPaymentMethod;
  depositor_name?: string;

  /** 세금계산서 */
  tax_invoice: boolean;
  tax_biz_number?: string;
  tax_company_name?: string;
  tax_rep_name?: string;
  tax_address?: string;
  tax_biz_type?: string;
  tax_biz_item?: string;
  tax_email?: string;

  /** 주문 항목 */
  items: CreateOrderItemReq[];

  /** 할인 금액 (VAT 포함, KRW). 기본 0. gross_amount 초과 금지. */
  discount_amount?: number;
  /** 할인 사유 (관리자 메모, 선택). */
  discount_reason?: string;

  /** 비고 (관리자 메모 가능) */
  notes?: string;
}

// =============================================================================
// 할인 편집 (PATCH /admin/textbook/orders/{id}/discount, 2026-04-23 신규)
// =============================================================================

export interface AdminUpdateDiscountReq {
  /** 할인 금액 (VAT 포함, KRW). 0 이면 할인 해제. gross_amount 초과 금지. */
  discount_amount: number;
  /** 할인 사유 (선택). */
  discount_reason?: string | null;
}

// =============================================================================
// Q6 감사 로그 조회 (GET /admin/textbook/logs, 2026-04-22)
// =============================================================================

/** admin_action_enum 값. 관리자 활동 액션 종류. */
export type AdminAction =
  | "create"
  | "update"
  | "banned"
  | "reorder"
  | "publish"
  | "unpublish"
  | "delete";

export interface AdminTextbookLogQuery {
  action?: AdminAction;
  order_id?: number;
  admin_user_id?: number;
  page?: number;
  per_page?: number;
}

export interface AdminTextbookLogItem {
  log_id: number;
  admin_user_id: number;
  admin_email: string;
  admin_nickname: string;
  order_id: number;
  order_code: string;
  action: AdminAction;
  /** before_data/after_data 는 JSONB raw value — diff 렌더는 프론트에서 */
  before_data: unknown | null;
  after_data: unknown | null;
  created_at: string;
}

export interface AdminTextbookLogMeta {
  total_count: number;
  total_pages: number;
  current_page: number;
  per_page: number;
}

export interface AdminTextbookLogListRes {
  items: AdminTextbookLogItem[];
  meta: AdminTextbookLogMeta;
}
