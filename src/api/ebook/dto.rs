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
}
