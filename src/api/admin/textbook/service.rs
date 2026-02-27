use crate::api::textbook::dto::OrderRes;
use crate::api::textbook::repo::TextbookRepo;
use crate::api::textbook::service::{TextbookService, build_order_res_from};
use crate::error::{AppError, AppResult};
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

        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            let order_items =
                TextbookRepo::find_items_by_order(&st.db, row.order_id).await?;
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

        TextbookService::get_order_by_id(st, order_id).await
    }

    /// 주문 삭제 (관리자)
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

        // 로그 먼저 기록 (CASCADE 삭제 전)
        TextbookRepo::insert_admin_log(
            &st.db,
            admin_user_id,
            order_id,
            AdminAction::Delete,
            Some(before),
            None,
        )
        .await?;

        TextbookRepo::delete_order(&st.db, order_id).await?;

        tracing::info!(
            admin_user_id = admin_user_id,
            order_id = order_id,
            "Textbook order deleted"
        );

        Ok(())
    }
}
