import type { BillingInterval, SubscriptionStatus } from "@/category/payment/types";

// =============================================================================
// 공통
// =============================================================================

export interface AdminPaymentMeta {
  total_count: number;
  total_pages: number;
  current_page: number;
  per_page: number;
}

// =============================================================================
// 구독 목록
// =============================================================================

export interface AdminSubListReq {
  page?: number;
  size?: number;
  q?: string;
  status?: string;
  sort?: string;
  order?: string;
}

export interface AdminSubSummary {
  subscription_id: number;
  user_id: number;
  user_email: string;
  status: SubscriptionStatus;
  billing_interval: BillingInterval;
  current_price_cents: number;
  current_period_end: string | null;
  created_at: string;
}

export interface AdminSubListRes {
  items: AdminSubSummary[];
  meta: AdminPaymentMeta;
}

// =============================================================================
// 구독 상세
// =============================================================================

export interface AdminSubDetail {
  subscription_id: number;
  user_id: number;
  provider_subscription_id: string;
  provider_customer_id: string | null;
  status: SubscriptionStatus;
  billing_interval: BillingInterval;
  current_price_cents: number;
  trial_started_at: string | null;
  trial_ends_at: string | null;
  current_period_start: string | null;
  current_period_end: string | null;
  canceled_at: string | null;
  paused_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface AdminSubUser {
  user_id: number;
  email: string;
  nickname: string | null;
  user_auth: string;
}

export interface AdminTxnSummary {
  transaction_id: number;
  subscription_id: number | null;
  user_id: number;
  user_email: string;
  status: string;
  amount_cents: number;
  tax_cents: number;
  currency: string;
  billing_interval: BillingInterval | null;
  occurred_at: string;
}

export interface AdminSubDetailRes {
  subscription: AdminSubDetail;
  user: AdminSubUser;
  transactions: AdminTxnSummary[];
}

// =============================================================================
// 트랜잭션 목록
// =============================================================================

export interface AdminTxnListReq {
  page?: number;
  size?: number;
  q?: string;
  status?: string;
  sort?: string;
  order?: string;
}

export interface AdminTxnListRes {
  items: AdminTxnSummary[];
  meta: AdminPaymentMeta;
}

// =============================================================================
// 수동 수강권
// =============================================================================

export interface AdminGrantReq {
  user_id: number;
  expire_at?: string;
  reason: string;
}

export interface AdminGrantRes {
  user_id: number;
  courses_granted: number;
  expire_at: string | null;
}

export interface AdminGrantListReq {
  page?: number;
  size?: number;
}

export interface AdminGrantSummary {
  user_id: number;
  user_email: string;
  expire_at: string | null;
  course_count: number;
}

export interface AdminGrantListRes {
  items: AdminGrantSummary[];
  meta: AdminPaymentMeta;
}

// =============================================================================
// 관리자 구독 취소 요청
// =============================================================================

export interface AdminCancelSubReq {
  immediately: boolean;
}
