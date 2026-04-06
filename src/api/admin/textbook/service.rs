use crate::api::textbook::dto::OrderRes;
use crate::api::textbook::repo::TextbookRepo;
use crate::api::textbook::service::{TextbookService, build_order_res_from};
use crate::error::{AppError, AppResult};
use crate::external::email::{send_templated, EmailTemplate};
use crate::state::AppState;
use crate::types::{AdminAction, TextbookOrderStatus};

use super::dto::{AdminTextbookListRes, AdminTextbookMeta};

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
