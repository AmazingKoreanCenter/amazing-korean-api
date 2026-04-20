use crate::api::textbook::dto::OrderRes;
use crate::api::textbook::repo::{InsertOrderParams, TextbookRepo};
use crate::api::textbook::service::{
    TextbookService, UNIT_PRICE, MIN_TOTAL_QUANTITY, build_order_res_from, catalog_languages,
};
use crate::error::{AppError, AppResult};
use crate::external::email::{send_templated, EmailTemplate};
use crate::state::AppState;
use crate::types::{AdminAction, TextbookLanguage, TextbookOrderStatus, TextbookType};

use super::dto::{AdminCreateOrderReq, AdminTextbookListRes, AdminTextbookMeta};

pub struct AdminTextbookService;

impl AdminTextbookService {
    /// 주문 목록 조회 (관리자)
    pub async fn list_orders(
        st: &AppState,
        status: Option<TextbookOrderStatus>,
        search: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> AppResult<AdminTextbookListRes> {
        let (rows, total) =
            TextbookRepo::list_orders(&st.db, status, search, page, per_page).await?;

        // N+1 방지: 모든 주문 항목을 한 번에 조회 후 order_id별로 그룹핑
        let order_ids: Vec<i64> = rows.iter().map(|r| r.order_id).collect();
        let all_items = TextbookRepo::find_items_by_orders(&st.db, &order_ids).await?;

        let mut items_map: std::collections::HashMap<i64, Vec<_>> = std::collections::HashMap::new();
        for item in all_items {
            items_map.entry(item.order_id).or_default().push(item);
        }

        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            let order_items = items_map.remove(&row.order_id).unwrap_or_default();
            items.push(build_order_res_from(row, order_items));
        }

        let total_pages = (total + per_page - 1) / per_page;

        Ok(AdminTextbookListRes {
            items,
            meta: AdminTextbookMeta {
                total_count: total,
                total_pages,
                current_page: page,
                per_page,
            },
        })
    }

    /// 주문 상세 조회 (관리자)
    pub async fn get_order(st: &AppState, order_id: i64) -> AppResult<OrderRes> {
        TextbookService::get_order_by_id(st, order_id).await
    }

    /// 관리자 대리 주문 생성
    ///
    /// 외부 채널(전화·이메일·오프라인) 주문을 관리자가 입력하거나,
    /// 영수증·통계 관리를 위해 관리자가 직접 생성. `CreateOrderReq` 와
    /// 대부분 동일하나 (a) user_id 선택적, (b) initial_status 로 paid 즉시 세팅
    /// 가능, (c) 최소 수량 제약 면제 기본값, (d) rate limit 미적용.
    pub async fn create_order(
        st: &AppState,
        admin_user_id: i64,
        req: AdminCreateOrderReq,
    ) -> AppResult<OrderRes> {
        // -----------------------------------------------------------------
        // 1. 검증 (공용 로직 — 사용자 create_order 와 동일 규칙)
        // -----------------------------------------------------------------
        if req.items.iter().any(|i| i.quantity < 1) {
            return Err(AppError::BadRequest(
                "Each item quantity must be at least 1".into(),
            ));
        }

        let total_quantity: i32 = req.items.iter().map(|i| i.quantity).sum();
        if req.enforce_min_quantity && total_quantity < MIN_TOTAL_QUANTITY {
            return Err(AppError::BadRequest(format!(
                "Minimum total quantity is {} copies",
                MIN_TOTAL_QUANTITY
            )));
        }

        // 중복 항목
        {
            let mut seen = std::collections::HashSet::new();
            for item in &req.items {
                if !seen.insert((item.language, item.textbook_type)) {
                    return Err(AppError::BadRequest(format!(
                        "Duplicate item: {:?} {:?}",
                        item.language, item.textbook_type
                    )));
                }
            }
        }

        // 언어 가용성
        let catalog = catalog_languages();
        for item in &req.items {
            let available = catalog
                .iter()
                .find(|(lang, _, _, _, _)| *lang == item.language)
                .map(|(_, _, _, avail, _)| *avail)
                .unwrap_or(false);
            if !available {
                return Err(AppError::BadRequest(format!(
                    "Language {:?} is currently not available for ordering",
                    item.language
                )));
            }
        }

        // 세금계산서 필수 항목
        if req.tax_invoice {
            if req.tax_biz_number.as_ref().is_none_or(|s| s.is_empty()) {
                return Err(AppError::BadRequest(
                    "Business registration number is required for tax invoice".into(),
                ));
            }
            if req.tax_company_name.as_ref().is_none_or(|s| s.is_empty()) {
                return Err(AppError::BadRequest(
                    "Company name (상호) is required for tax invoice".into(),
                ));
            }
            if req.tax_rep_name.as_ref().is_none_or(|s| s.is_empty()) {
                return Err(AppError::BadRequest(
                    "Representative name (대표자명) is required for tax invoice".into(),
                ));
            }
            if req.tax_email.as_ref().is_none_or(|s| s.is_empty()) {
                return Err(AppError::BadRequest(
                    "Tax invoice email is required for tax invoice".into(),
                ));
            }
        }

        // 초기 상태 제한: pending / confirmed / paid 만 허용 (그 이후 상태는
        // status 전환 API 거쳐야 감사 로그·이메일 알림 일관됨)
        let initial_status = req.initial_status.unwrap_or(TextbookOrderStatus::Pending);
        if !matches!(
            initial_status,
            TextbookOrderStatus::Pending
                | TextbookOrderStatus::Confirmed
                | TextbookOrderStatus::Paid
        ) {
            return Err(AppError::BadRequest(
                "initial_status must be one of: pending, confirmed, paid".into(),
            ));
        }

        // -----------------------------------------------------------------
        // 2. 트랜잭션 — order INSERT + items INSERT
        // -----------------------------------------------------------------
        let total_amount: i32 = req.items.iter().map(|i| i.quantity * UNIT_PRICE).sum();

        let mut tx = st.db.begin().await?;

        let order_code = TextbookRepo::generate_order_code(&mut tx).await?;

        let order_id = TextbookRepo::insert_order(
            &mut tx,
            &InsertOrderParams {
                order_code: &order_code,
                user_id: req.user_id,
                orderer_name: &req.orderer_name,
                orderer_email: &req.orderer_email,
                orderer_phone: &req.orderer_phone,
                org_name: req.org_name.as_deref(),
                org_type: req.org_type.as_deref(),
                delivery_postal_code: req.delivery_postal_code.as_deref(),
                delivery_address: &req.delivery_address,
                delivery_detail: req.delivery_detail.as_deref(),
                payment_method: req.payment_method,
                depositor_name: req.depositor_name.as_deref(),
                tax_invoice: req.tax_invoice,
                tax_biz_number: req.tax_biz_number.as_deref(),
                tax_company_name: req.tax_company_name.as_deref(),
                tax_rep_name: req.tax_rep_name.as_deref(),
                tax_address: req.tax_address.as_deref(),
                tax_biz_type: req.tax_biz_type.as_deref(),
                tax_biz_item: req.tax_biz_item.as_deref(),
                tax_email: req.tax_email.as_deref(),
                total_quantity,
                total_amount,
                notes: req.notes.as_deref(),
            },
        )
        .await?;

        let items: Vec<(TextbookLanguage, TextbookType, i32, i32)> = req
            .items
            .iter()
            .map(|i| (i.language, i.textbook_type, i.quantity, UNIT_PRICE))
            .collect();

        TextbookRepo::insert_items(&mut tx, order_id, &items).await?;

        tx.commit().await?;

        // -----------------------------------------------------------------
        // 3. 초기 상태가 pending 이 아니면 update_status 로 타임스탬프 세팅
        //    (paid_at / confirmed_at 자동) — 트랜잭션 커밋 후 별도 호출
        // -----------------------------------------------------------------
        if initial_status != TextbookOrderStatus::Pending {
            TextbookRepo::update_status(&st.db, order_id, initial_status).await?;
        }

        // -----------------------------------------------------------------
        // 4. 관리자 감사 로그 — AdminAction::Create
        // -----------------------------------------------------------------
        let after = serde_json::json!({
            "order_code": order_code,
            "orderer_name": req.orderer_name,
            "total_quantity": total_quantity,
            "total_amount": total_amount,
            "initial_status": format!("{:?}", initial_status),
            "user_id": req.user_id,
        });
        TextbookRepo::insert_admin_log(
            &st.db,
            admin_user_id,
            order_id,
            AdminAction::Create,
            None,
            Some(after),
        )
        .await?;

        tracing::info!(
            admin_user_id = admin_user_id,
            order_id = order_id,
            order_code = %order_code,
            total_quantity = total_quantity,
            total_amount = total_amount,
            initial_status = ?initial_status,
            "Textbook order created by admin on behalf"
        );

        // -----------------------------------------------------------------
        // 5. 주문 접수 확인 이메일 (fire-and-forget, 실패해도 주문 유지)
        // -----------------------------------------------------------------
        if let Some(ref email_sender) = st.email {
            let template = EmailTemplate::TextbookOrderConfirmation {
                order_code: order_code.clone(),
                orderer_name: req.orderer_name.clone(),
                total_quantity,
                total_amount,
            };
            if let Err(e) =
                send_templated(email_sender.as_ref(), &req.orderer_email, template).await
            {
                tracing::warn!(
                    order_code = %order_code,
                    email = %req.orderer_email,
                    error = %e,
                    "Failed to send order confirmation email (admin-created, order still created)"
                );
            }
        }

        TextbookService::get_order_by_id(st, order_id).await
    }

    /// 주문 상태 변경 (관리자)
    pub async fn update_status(
        st: &AppState,
        admin_user_id: i64,
        order_id: i64,
        new_status: TextbookOrderStatus,
    ) -> AppResult<OrderRes> {
        // 현재 주문 확인
        let order = TextbookRepo::find_by_id(&st.db, order_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // 상태 전환 검증
        if !is_valid_status_transition(&order.status, &new_status) {
            return Err(AppError::BadRequest(format!(
                "Invalid status transition: {:?} → {:?}",
                order.status, new_status
            )));
        }

        // Shipped 전환 시 추적 정보 필수
        if new_status == TextbookOrderStatus::Shipped
            && order.tracking_number.as_ref().is_none_or(|s| s.is_empty())
        {
            return Err(AppError::BadRequest(
                "Tracking number is required before marking as shipped".into(),
            ));
        }

        let before = serde_json::json!({ "status": format!("{:?}", order.status) });

        TextbookRepo::update_status(&st.db, order_id, new_status).await?;

        let after = serde_json::json!({ "status": format!("{:?}", new_status) });

        // 관리자 로그 기록
        TextbookRepo::insert_admin_log(
            &st.db,
            admin_user_id,
            order_id,
            AdminAction::Update,
            Some(before),
            Some(after),
        )
        .await?;

        tracing::info!(
            admin_user_id = admin_user_id,
            order_id = order_id,
            new_status = ?new_status,
            "Textbook order status updated"
        );

        // 상태 변경 이메일 알림 (fire-and-forget)
        if let Some(ref email_sender) = st.email {
            let status_label = status_display_label(&new_status);
            let template = EmailTemplate::TextbookOrderStatusUpdate {
                order_code: order.order_code.clone(),
                orderer_name: order.orderer_name.clone(),
                new_status: format!("{:?}", new_status),
                status_label: status_label.to_string(),
                tracking_number: order.tracking_number.clone(),
                tracking_provider: order.tracking_provider.clone(),
            };
            if let Err(e) = send_templated(email_sender.as_ref(), &order.orderer_email, template).await {
                tracing::warn!(
                    order_code = %order.order_code,
                    error = %e,
                    "Failed to send status update email"
                );
            }
        }

        TextbookService::get_order_by_id(st, order_id).await
    }

    /// 배송 추적 정보 업데이트 (관리자)
    pub async fn update_tracking(
        st: &AppState,
        admin_user_id: i64,
        order_id: i64,
        tracking_number: Option<&str>,
        tracking_provider: Option<&str>,
    ) -> AppResult<OrderRes> {
        let order = TextbookRepo::find_by_id(&st.db, order_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let before = serde_json::json!({
            "tracking_number": order.tracking_number,
            "tracking_provider": order.tracking_provider,
        });

        TextbookRepo::update_tracking(&st.db, order_id, tracking_number, tracking_provider)
            .await?;

        let after = serde_json::json!({
            "tracking_number": tracking_number,
            "tracking_provider": tracking_provider,
        });

        TextbookRepo::insert_admin_log(
            &st.db,
            admin_user_id,
            order_id,
            AdminAction::Update,
            Some(before),
            Some(after),
        )
        .await?;

        tracing::info!(
            admin_user_id = admin_user_id,
            order_id = order_id,
            "Textbook order tracking updated"
        );

        TextbookService::get_order_by_id(st, order_id).await
    }

    /// 주문 삭제 (Soft Delete — 감사 로그 보존)
    pub async fn delete_order(
        st: &AppState,
        admin_user_id: i64,
        order_id: i64,
    ) -> AppResult<()> {
        let order = TextbookRepo::find_by_id(&st.db, order_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let before = serde_json::json!({
            "order_code": order.order_code,
            "status": format!("{:?}", order.status),
        });

        // Soft delete (is_deleted = true, deleted_at = NOW())
        TextbookRepo::soft_delete_order(&st.db, order_id).await?;

        // 로그 기록
        TextbookRepo::insert_admin_log(
            &st.db,
            admin_user_id,
            order_id,
            AdminAction::Delete,
            Some(before),
            None,
        )
        .await?;

        tracing::info!(
            admin_user_id = admin_user_id,
            order_id = order_id,
            "Textbook order soft-deleted"
        );

        Ok(())
    }
}

// =============================================================================
// 상태 전환 검증 (State Machine)
// =============================================================================

/// 상태 → 한국어 표시 라벨
fn status_display_label(status: &TextbookOrderStatus) -> &'static str {
    match status {
        TextbookOrderStatus::Pending => "주문 접수",
        TextbookOrderStatus::Confirmed => "주문 확인",
        TextbookOrderStatus::Paid => "입금 확인",
        TextbookOrderStatus::Printing => "인쇄 중",
        TextbookOrderStatus::Shipped => "발송 완료",
        TextbookOrderStatus::Delivered => "배송 완료",
        TextbookOrderStatus::Canceled => "주문 취소",
    }
}

/// 유효한 상태 전환인지 검증
/// pending → confirmed → paid → printing → shipped → delivered (정방향)
/// 모든 상태에서 canceled 전환 가능 (delivered, canceled 제외)
fn is_valid_status_transition(
    current: &TextbookOrderStatus,
    next: &TextbookOrderStatus,
) -> bool {
    use TextbookOrderStatus::*;
    matches!(
        (current, next),
        (Pending, Confirmed)
            | (Pending, Canceled)
            | (Confirmed, Paid)
            | (Confirmed, Canceled)
            | (Paid, Printing)
            | (Paid, Canceled)
            | (Printing, Shipped)
            | (Printing, Canceled)
            | (Shipped, Delivered)
            | (Shipped, Canceled)
    )
}
