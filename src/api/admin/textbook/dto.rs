use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::types::TextbookOrderStatus;

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
