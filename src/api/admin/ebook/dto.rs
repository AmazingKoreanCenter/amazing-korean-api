use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::types::{EbookEdition, EbookPaymentMethod, EbookPurchaseStatus, TextbookLanguage};

#[derive(Debug, Deserialize)]
pub struct AdminEbookListReq {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<EbookPurchaseStatus>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminEbookMeta {
    pub total_count: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminEbookPurchaseItem {
    pub purchase_id: i64,
    pub purchase_code: String,
    pub user_id: i64,
    pub language: TextbookLanguage,
    pub edition: EbookEdition,
    pub payment_method: EbookPaymentMethod,
    pub status: EbookPurchaseStatus,
    pub price: i32,
    pub currency: String,
    pub paddle_txn_id: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub refunded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminEbookListRes {
    pub items: Vec<AdminEbookPurchaseItem>,
    pub meta: AdminEbookMeta,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AdminUpdateEbookStatusReq {
    pub status: EbookPurchaseStatus,
}
