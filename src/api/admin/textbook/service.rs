use crate::api::textbook::dto::OrderRes;
use crate::api::textbook::repo::{InsertOrderParams, TextbookRepo};
use crate::api::textbook::service::{
    TextbookService, UNIT_PRICE, MIN_TOTAL_QUANTITY, build_order_res_from, catalog_languages,
};
use crate::crypto::CryptoService;
use crate::error::{AppError, AppResult};
use crate::external::email::{send_templated, EmailTemplate};
use crate::state::AppState;
use crate::types::{AdminAction, TextbookLanguage, TextbookOrderStatus, TextbookType};

use super::dto::{
    AdminCreateOrderReq, AdminTextbookListRes, AdminTextbookLogItem, AdminTextbookLogListRes,
    AdminTextbookLogMeta, AdminTextbookLogQuery, AdminTextbookMeta, AdminUpdateDiscountReq,
};

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

    /// Q6 (2026-04-22) — admin_textbook_log 감사 로그 조회
    ///
    /// 필터 (action/order_id/admin_user_id) + 페이지네이션. repo Row 의
    /// admin_email_enc 를 crypto 로 복호화해 응답 DTO 조립.
    pub async fn list_admin_logs(
        st: &AppState,
        query: AdminTextbookLogQuery,
    ) -> AppResult<AdminTextbookLogListRes> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page_raw = query.per_page.unwrap_or(20);
        let per_page = per_page_raw.clamp(1, 100);

        let (total, rows) = TextbookRepo::list_admin_logs(
            &st.db,
            query.action,
            query.order_id,
            query.admin_user_id,
            page,
            per_page,
        )
        .await?;

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            let admin_email = crypto
                .decrypt(&row.admin_email_enc, "users.user_email")?;
            items.push(AdminTextbookLogItem {
                log_id: row.log_id,
                admin_user_id: row.admin_user_id,
                admin_email,
                admin_nickname: row.admin_nickname,
                order_id: row.order_id,
                order_code: row.order_code,
                action: row.action,
                before_data: row.before_data,
                after_data: row.after_data,
                created_at: row.created_at,
            });
        }

        let total_pages = if total == 0 {
            0
        } else {
            (total + per_page - 1) / per_page
        };

        Ok(AdminTextbookLogListRes {
            items,
            meta: AdminTextbookLogMeta {
                total_count: total,
                total_pages,
                current_page: page,
                per_page,
            },
        })
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
        // 2. 금액 계산 + 할인 검증
        // -----------------------------------------------------------------
        // gross_amount: 수량 × 단가 (할인 전). total_amount: 할인 차감 후.
        let gross_amount: i32 = req.items.iter().map(|i| i.quantity * UNIT_PRICE).sum();
        let discount_amount = req.discount_amount;
        if discount_amount < 0 {
            return Err(AppError::BadRequest(
                "discount_amount must be non-negative".into(),
            ));
        }
        if discount_amount > gross_amount {
            return Err(AppError::BadRequest(format!(
                "discount_amount ({}) cannot exceed gross_amount ({})",
                discount_amount, gross_amount
            )));
        }
        let total_amount = gross_amount - discount_amount;

        // 할인 사유: 빈 문자열 trim 후 None 으로 정규화 (UI 빈 입력 허용).
        let discount_reason_normalized = req
            .discount_reason
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());

        // -----------------------------------------------------------------
        // 3. 트랜잭션 — order INSERT + items INSERT
        // -----------------------------------------------------------------

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
                gross_amount,
                discount_amount,
                discount_reason: discount_reason_normalized,
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

        // -----------------------------------------------------------------
        // 4. 초기 상태가 pending 이 아니면 동일 트랜잭션 내에서 상태 + 타임스탬프
        //    업데이트 (paid_at / confirmed_at). insert 와 원자적으로 처리되어
        //    초기 상태 세팅 실패 시 주문 전체가 롤백됨 — 고아 Pending 상태 방지.
        // -----------------------------------------------------------------
        if initial_status != TextbookOrderStatus::Pending {
            TextbookRepo::update_status_in_tx(&mut tx, order_id, initial_status).await?;
        }

        tx.commit().await?;

        // -----------------------------------------------------------------
        // 5. 관리자 감사 로그 — AdminAction::Create
        // -----------------------------------------------------------------
        let after = serde_json::json!({
            "order_code": order_code,
            "orderer_name": req.orderer_name,
            "total_quantity": total_quantity,
            "gross_amount": gross_amount,
            "discount_amount": discount_amount,
            "discount_reason": discount_reason_normalized,
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
            gross_amount = gross_amount,
            discount_amount = discount_amount,
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

    /// 주문 할인 편집 (관리자, 2026-04-23 신규)
    ///
    /// 생성 후에도 할인 금액/사유를 수정 가능. gross_amount 는 불변이며
    /// discount_amount + total_amount(= gross - discount) 만 갱신.
    /// admin_textbook_log 에 Update 액션 기록.
    pub async fn update_discount(
        st: &AppState,
        admin_user_id: i64,
        order_id: i64,
        req: AdminUpdateDiscountReq,
    ) -> AppResult<OrderRes> {
        let order = TextbookRepo::find_by_id(&st.db, order_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // 검증: 0 ≤ discount ≤ gross. DB CHECK 와 중복이나 에러 메시지 명확화 목적.
        if req.discount_amount < 0 {
            return Err(AppError::BadRequest(
                "discount_amount must be non-negative".into(),
            ));
        }
        if req.discount_amount > order.gross_amount {
            return Err(AppError::BadRequest(format!(
                "discount_amount ({}) cannot exceed gross_amount ({})",
                req.discount_amount, order.gross_amount
            )));
        }

        let reason_normalized = req
            .discount_reason
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());

        let before = serde_json::json!({
            "discount_amount": order.discount_amount,
            "discount_reason": order.discount_reason,
            "total_amount": order.total_amount,
        });

        TextbookRepo::update_discount(&st.db, order_id, req.discount_amount, reason_normalized)
            .await?;

        let new_total = order.gross_amount - req.discount_amount;
        let after = serde_json::json!({
            "discount_amount": req.discount_amount,
            "discount_reason": reason_normalized,
            "total_amount": new_total,
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
            order_code = %order.order_code,
            discount_amount = req.discount_amount,
            new_total_amount = new_total,
            "Textbook order discount updated"
        );

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
