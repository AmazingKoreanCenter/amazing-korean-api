use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::types::{EbookEdition, EbookPaymentMethod, EbookPurchaseStatus, TextbookLanguage};

// ─────────────────────── Catalog ───────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct EbookEditionInfo {
    pub edition: EbookEdition,
    pub price: i32,
    pub currency: String,
    pub paddle_price_usd: Option<i32>,
    pub total_pages: i32,
    pub available: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EbookCatalogItem {
    pub language: TextbookLanguage,
    pub language_name_ko: String,
    pub language_name_en: String,
    pub editions: Vec<EbookEditionInfo>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EbookCatalogRes {
    pub items: Vec<EbookCatalogItem>,
    pub paddle_ebook_price_id: Option<String>,
    pub client_token: Option<String>,
    pub sandbox: bool,
}

// ─────────────────────── Purchase ───────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePurchaseReq {
    pub language: TextbookLanguage,
    pub edition: EbookEdition,
    pub payment_method: EbookPaymentMethod,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PurchaseRes {
    pub purchase_code: String,
    pub status: EbookPurchaseStatus,
    pub language: TextbookLanguage,
    pub edition: EbookEdition,
    pub payment_method: EbookPaymentMethod,
    pub price: i32,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MyPurchasesRes {
    pub items: Vec<PurchaseRes>,
}

// ─────────────────────── IAP Purchase ───────────────────────

/// IAP 플랫폼
#[derive(Debug, Clone, Copy, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum IapPlatform {
    Apple,
    Google,
}

/// 모바일 IAP 구매 확정 요청 (RevenueCat 영수증 검증 후 구매 생성)
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateIapPurchaseReq {
    pub language: TextbookLanguage,
    pub edition: EbookEdition,
    pub platform: IapPlatform,
    #[validate(length(min = 1, message = "product_id is required"))]
    pub product_id: String,
    #[validate(length(min = 1, message = "transaction_id is required"))]
    pub transaction_id: String,
}

// ─────────────────────── Viewer ───────────────────────

#[derive(Debug, Serialize, ToSchema)]
pub struct TocEntry {
    pub title: String,
    pub title_ko: String,
    pub page: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ViewerMetaRes {
    pub purchase_code: String,
    pub language: TextbookLanguage,
    pub edition: EbookEdition,
    pub total_pages: i32,
    pub toc: Vec<TocEntry>,
    pub session_id: String,
    pub hmac_secret: String,
    pub tile_mode: bool,
    pub grid_rows: Option<u32>,
    pub grid_cols: Option<u32>,
}

// ─────────────────────── Heartbeat ───────────────────────

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct HeartbeatReq {
    pub session_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HeartbeatRes {
    pub valid: bool,
}
