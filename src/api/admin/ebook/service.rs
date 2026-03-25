use crate::crypto::CryptoService;
use crate::error::{AppError, AppResult};
use crate::external::email::{send_templated, EmailTemplate};
use crate::state::AppState;
use crate::types::{AdminAction, EbookPurchaseStatus};

use crate::api::ebook::repo;
use crate::api::ebook::service::{language_name_ko, edition_label_ko};

use super::dto::{
    AdminEbookListReq, AdminEbookListRes, AdminEbookMeta, AdminEbookPurchaseItem,
    AdminUpdateEbookStatusReq,
};

pub struct AdminEbookService;

impl AdminEbookService {
    /// 구매 목록 조회 (관리자)
    pub async fn list_purchases(
        st: &AppState,
        req: AdminEbookListReq,
    ) -> AppResult<AdminEbookListRes> {
        let page = req.page.unwrap_or(1).max(1);
        let per_page = req.per_page.unwrap_or(20).clamp(1, 100);

        let (rows, total_count) = repo::list_purchases(
            &st.db,
            page,
            per_page,
            req.status,
            req.search.as_deref(),
        )
        .await?;

        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + per_page - 1) / per_page
        };

        let items = rows
            .into_iter()
            .map(|r| AdminEbookPurchaseItem {
                purchase_id: r.purchase_id,
                purchase_code: r.purchase_code,
                user_id: r.user_id,
                language: r.language,
                edition: r.edition,
                payment_method: r.payment_method,
                status: r.status,
                price: r.price,
                currency: r.currency,
                paddle_txn_id: r.paddle_txn_id,
                completed_at: r.completed_at,
                refunded_at: r.refunded_at,
                created_at: r.created_at,
            })
            .collect();

        Ok(AdminEbookListRes {
            items,
            meta: AdminEbookMeta {
                total_count,
                page,
                per_page,
                total_pages,
            },
        })
    }

    /// 구매 상세 조회 (관리자)
    pub async fn get_purchase(
        st: &AppState,
        purchase_id: i64,
    ) -> AppResult<AdminEbookPurchaseItem> {
        let r = repo::find_by_id(&st.db, purchase_id)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(AdminEbookPurchaseItem {
            purchase_id: r.purchase_id,
            purchase_code: r.purchase_code,
            user_id: r.user_id,
            language: r.language,
            edition: r.edition,
            payment_method: r.payment_method,
            status: r.status,
            price: r.price,
            currency: r.currency,
            paddle_txn_id: r.paddle_txn_id,
            completed_at: r.completed_at,
            refunded_at: r.refunded_at,
            created_at: r.created_at,
        })
    }

    /// 구매 상태 변경 (관리자)
    pub async fn update_status(
        st: &AppState,
        admin_user_id: i64,
        purchase_id: i64,
        req: AdminUpdateEbookStatusReq,
    ) -> AppResult<AdminEbookPurchaseItem> {
        let existing = repo::find_by_id(&st.db, purchase_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // 상태 전이 검증
        if !is_valid_status_transition(existing.status, req.status) {
            return Err(AppError::BadRequest(format!(
                "Cannot transition from {:?} to {:?}",
                existing.status, req.status
            )));
        }

        let before_data = serde_json::json!({ "status": format!("{:?}", existing.status) });

        repo::update_status(&st.db, purchase_id, req.status).await?;

        let after_data = serde_json::json!({ "status": format!("{:?}", req.status) });

        // 관리자 로그
        repo::insert_admin_log(
            &st.db,
            admin_user_id,
            purchase_id,
            AdminAction::Update,
            Some(before_data),
            Some(after_data),
        )
        .await?;

        // 결제 완료 이메일 발송 (completed 전환 시)
        if req.status == EbookPurchaseStatus::Completed {
            if let Some(ref email_sender) = st.email {
                let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
                if let Ok(Some(encrypted_email)) = repo::find_user_encrypted_email(&st.db, existing.user_id).await {
                    if let Ok(user_email) = crypto.decrypt(&encrypted_email, "users.user_email") {
                        let template = EmailTemplate::EbookPurchaseCompleted {
                            purchase_code: existing.purchase_code.clone(),
                            language_name: language_name_ko(existing.language).to_string(),
                            edition_label: edition_label_ko(existing.edition).to_string(),
                        };
                        if let Err(e) = send_templated(email_sender.as_ref(), &user_email, template).await {
                            tracing::warn!(
                                purchase_code = %existing.purchase_code,
                                error = %e,
                                "Failed to send ebook purchase completed email"
                            );
                        }
                    }
                }
            }
        }

        Self::get_purchase(st, purchase_id).await
    }

    /// 구매 삭제 (소프트 삭제, 관리자)
    pub async fn delete_purchase(
        st: &AppState,
        admin_user_id: i64,
        purchase_id: i64,
    ) -> AppResult<()> {
        let existing = repo::find_by_id(&st.db, purchase_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let before_data = serde_json::json!({
            "purchase_code": existing.purchase_code,
            "status": format!("{:?}", existing.status),
        });

        repo::soft_delete_purchase(&st.db, purchase_id).await?;

        repo::insert_admin_log(
            &st.db,
            admin_user_id,
            purchase_id,
            AdminAction::Delete,
            Some(before_data),
            None,
        )
        .await?;

        Ok(())
    }
}

fn is_valid_status_transition(from: EbookPurchaseStatus, to: EbookPurchaseStatus) -> bool {
    matches!(
        (from, to),
        (EbookPurchaseStatus::Pending, EbookPurchaseStatus::Completed)
            | (EbookPurchaseStatus::Pending, EbookPurchaseStatus::Refunded)
            | (EbookPurchaseStatus::Completed, EbookPurchaseStatus::Refunded)
    )
}
