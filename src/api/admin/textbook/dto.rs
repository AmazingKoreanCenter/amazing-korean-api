use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::api::textbook::dto::CreateOrderItemReq;
use crate::types::{AdminAction, TextbookOrderStatus, TextbookPaymentMethod};

// =============================================================================
// 목록 조회
// =============================================================================

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AdminTextbookListReq {
    pub page: Option<i64>,
    pub size: Option<i64>,
    /// 주문번호, 신청자명, 기관명 검색
    pub q: Option<String>,
    /// 상태 필터 (pending, confirmed, paid, printing, shipped, delivered, canceled)
    pub status: Option<TextbookOrderStatus>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminTextbookMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminTextbookListRes {
    pub items: Vec<crate::api::textbook::dto::OrderRes>,
    pub meta: AdminTextbookMeta,
}

// =============================================================================
// 감사 로그 조회 (Q6, 2026-04-22) — GET /admin/textbook/logs
// =============================================================================
//
// admin_textbook_log 테이블 조회. 관리자가 어느 주문에 대해 언제 어떤 액션
// (create/update/banned/…) 을 했는지 확인. before_data / after_data 는 JSONB
// 원본 그대로 전달 (프론트에서 diff 렌더).

/// 감사 로그 조회 필터 (Query String)
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AdminTextbookLogQuery {
    /// 액션 필터 (create / update / banned / reorder / publish / unpublish / delete)
    pub action: Option<AdminAction>,

    /// 특정 주문 로그만 조회
    pub order_id: Option<i64>,

    /// 특정 관리자가 수행한 로그만 조회
    pub admin_user_id: Option<i64>,

    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminTextbookLogItem {
    pub log_id: i64,
    pub admin_user_id: i64,
    /// 관리자 이메일 (복호화된 평문)
    pub admin_email: String,
    pub admin_nickname: String,
    pub order_id: i64,
    /// 주문번호 (textbook.order_code)
    pub order_code: String,
    pub action: AdminAction,
    #[schema(value_type = Object)]
    pub before_data: Option<serde_json::Value>,
    #[schema(value_type = Object)]
    pub after_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminTextbookLogMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminTextbookLogListRes {
    pub items: Vec<AdminTextbookLogItem>,
    pub meta: AdminTextbookLogMeta,
}

// =============================================================================
// 상태 변경
// =============================================================================

#[derive(Debug, Deserialize, ToSchema)]
pub struct AdminUpdateStatusReq {
    pub status: TextbookOrderStatus,
}

// =============================================================================
// 할인 편집 — PATCH /admin/textbook/orders/{id}/discount
// =============================================================================
//
// 관리자가 대리 주문 생성 후에도 할인 금액/사유를 수정할 수 있도록 별도
// 엔드포인트. gross_amount 는 불변이며 discount_amount + total_amount 만 갱신.
// service 에서 0 ≤ discount ≤ gross 검증 + admin_textbook_log 기록.

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AdminUpdateDiscountReq {
    /// 할인 금액 (VAT 포함, KRW). 0 이면 할인 해제. gross_amount 초과 금지.
    pub discount_amount: i32,
    /// 할인 사유 (선택, 관리자 메모). 빈 문자열 또는 null 허용.
    #[validate(length(max = 500))]
    pub discount_reason: Option<String>,
}

// =============================================================================
// 배송 추적 정보 업데이트
// =============================================================================

#[derive(Debug, Deserialize, ToSchema)]
pub struct AdminUpdateTrackingReq {
    pub tracking_number: Option<String>,
    pub tracking_provider: Option<String>,
}

// =============================================================================
// 관리자 대리 주문 생성 — POST /admin/textbook/orders
// =============================================================================
//
// 외부(전화·이메일·오프라인)로 접수된 주문을 관리자가 시스템에 입력하거나,
// 영수증·통계 정합성 관리를 위해 관리자가 직접 생성하는 주문을 위한 DTO.
// `CreateOrderReq` 와 필드 구조는 같지만:
//   - `user_id` 를 명시 지정 가능 (없으면 비회원 주문으로 NULL 저장)
//   - `initial_status` 로 초기 상태 지정 가능 (paid 로 즉시 만들어 영수증 발급 가능)
//   - 최소 수량(10권) 제약 기본 면제 (단일 샘플·증정 등 대응)

/// 관리자 대리 주문 생성 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AdminCreateOrderReq {
    /// 주문을 귀속시킬 사용자 id (선택). 없으면 비회원 주문으로 저장.
    pub user_id: Option<i64>,

    /// 초기 주문 상태 (기본 pending). paid 로 지정 시 paid_at 자동 세팅.
    /// 허용: pending, confirmed, paid (그 이후 상태는 status 전환 API 사용)
    pub initial_status: Option<TextbookOrderStatus>,

    /// 최소 수량(10권) 제약 강제 여부 (기본 false — 대리 주문은 소량 허용)
    #[serde(default)]
    pub enforce_min_quantity: bool,

    /// 신청자 정보
    #[validate(length(min = 1, max = 100))]
    pub orderer_name: String,
    /// 2026-04-23: 오프라인·전화 주문 대응으로 optional 로 완화. 입력 시 email 형식 검증.
    #[validate(email, length(max = 255))]
    pub orderer_email: Option<String>,
    #[validate(length(min = 1, max = 30))]
    pub orderer_phone: String,

    /// 기관 정보 (선택)
    #[validate(length(max = 200))]
    pub org_name: Option<String>,
    #[validate(length(max = 100))]
    pub org_type: Option<String>,

    /// 배송 정보
    #[validate(length(max = 20))]
    pub delivery_postal_code: Option<String>,
    #[validate(length(min = 1))]
    pub delivery_address: String,
    #[validate(length(max = 200))]
    pub delivery_detail: Option<String>,

    /// 결제 정보
    pub payment_method: TextbookPaymentMethod,
    #[validate(length(max = 100))]
    pub depositor_name: Option<String>,

    /// 세금계산서
    #[serde(default)]
    pub tax_invoice: bool,
    #[validate(length(max = 20))]
    pub tax_biz_number: Option<String>,
    #[validate(length(max = 200))]
    pub tax_company_name: Option<String>,
    #[validate(length(max = 100))]
    pub tax_rep_name: Option<String>,
    #[validate(length(max = 500))]
    pub tax_address: Option<String>,
    #[validate(length(max = 100))]
    pub tax_biz_type: Option<String>,
    #[validate(length(max = 100))]
    pub tax_biz_item: Option<String>,
    #[validate(email, length(max = 255))]
    pub tax_email: Option<String>,

    /// 주문 항목 (최소 1개)
    #[validate(length(min = 1))]
    pub items: Vec<CreateOrderItemReq>,

    /// 할인 금액 (VAT 포함, KRW). 관리자 대리 주문 시에만 설정. 기본 0.
    /// gross_amount(수량×단가) 초과 금지. service 에서 검증.
    #[serde(default)]
    pub discount_amount: i32,

    /// 할인 사유 (선택, 관리자 메모). discount_amount > 0 일 때 권장.
    #[validate(length(max = 500))]
    pub discount_reason: Option<String>,

    /// 비고 (관리자 메모 가능)
    pub notes: Option<String>,
}
