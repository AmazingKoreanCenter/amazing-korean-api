use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::types::{
    TextbookLanguage, TextbookOrderStatus, TextbookPaymentMethod, TextbookType,
};

// =============================================================================
// GET /textbook/catalog — 교재 카탈로그
// =============================================================================

/// 개별 교재 정보
#[derive(Debug, Serialize, ToSchema)]
pub struct CatalogItem {
    pub language: TextbookLanguage,
    pub language_name_ko: String,
    pub language_name_en: String,
    pub available_types: Vec<TextbookType>,
    pub unit_price: i32,
    pub currency: String,
    pub available: bool,
}

/// GET /textbook/catalog 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct CatalogRes {
    pub items: Vec<CatalogItem>,
    pub min_total_quantity: i32,
    pub currency: String,
}

// =============================================================================
// POST /textbook/orders — 주문 생성
// =============================================================================

/// 주문 항목 요청
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateOrderItemReq {
    pub language: TextbookLanguage,
    pub textbook_type: TextbookType,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

/// 주문 생성 요청
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrderReq {
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
    #[validate(email, length(max = 255))]
    pub tax_email: Option<String>,

    /// 주문 항목 (최소 1개)
    #[validate(length(min = 1))]
    pub items: Vec<CreateOrderItemReq>,

    /// 비고
    pub notes: Option<String>,
}

// =============================================================================
// 주문 응답 (공용)
// =============================================================================

/// 주문 항목 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct OrderItemRes {
    pub language: TextbookLanguage,
    pub language_name: String,
    pub textbook_type: TextbookType,
    pub quantity: i32,
    pub unit_price: i32,
    pub subtotal: i32,
}

/// 주문 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct OrderRes {
    pub order_id: i64,
    pub order_code: String,
    pub status: TextbookOrderStatus,
    /// 신청자 정보
    pub orderer_name: String,
    pub orderer_email: String,
    pub orderer_phone: String,
    /// 기관 정보
    pub org_name: Option<String>,
    pub org_type: Option<String>,
    /// 배송 정보
    pub delivery_postal_code: Option<String>,
    pub delivery_address: String,
    pub delivery_detail: Option<String>,
    /// 결제 정보
    pub payment_method: TextbookPaymentMethod,
    pub depositor_name: Option<String>,
    /// 세금계산서
    pub tax_invoice: bool,
    pub tax_biz_number: Option<String>,
    pub tax_email: Option<String>,
    /// 금액
    pub total_quantity: i32,
    pub total_amount: i32,
    pub currency: String,
    /// 비고
    pub notes: Option<String>,
    /// 배송 추적
    pub tracking_number: Option<String>,
    pub tracking_provider: Option<String>,
    /// 항목
    pub items: Vec<OrderItemRes>,
    /// 상태 변경 시각
    pub confirmed_at: Option<String>,
    pub paid_at: Option<String>,
    pub shipped_at: Option<String>,
    pub delivered_at: Option<String>,
    pub canceled_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
