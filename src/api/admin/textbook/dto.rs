use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::api::textbook::dto::CreateOrderItemReq;
use crate::types::{TextbookOrderStatus, TextbookPaymentMethod};

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
// 상태 변경
// =============================================================================

#[derive(Debug, Deserialize, ToSchema)]
pub struct AdminUpdateStatusReq {
    pub status: TextbookOrderStatus,
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
    #[validate(email, length(max = 255))]
    pub orderer_email: String,
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

    /// 비고 (관리자 메모 가능)
    pub notes: Option<String>,
}
