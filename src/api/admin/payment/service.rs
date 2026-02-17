use std::net::IpAddr;

use crate::crypto::CryptoService;
use crate::error::{AppError, AppResult};
use crate::external::payment::CancelEffectiveFrom;
use crate::state::AppState;
use crate::types::UserAuth;

use super::dto::{
    AdminCancelSubReq, AdminGrantListReq, AdminGrantListRes, AdminGrantReq, AdminGrantRes,
    AdminPaymentMeta, AdminSubDetailRes, AdminSubListReq, AdminSubListRes, AdminTxnListReq,
    AdminTxnListRes,
};
use super::repo::AdminPaymentRepo;

pub struct AdminPaymentService;

impl AdminPaymentService {
    // =========================================================================
    // RBAC 검증 (기존 admin 패턴 재사용)
    // =========================================================================

    async fn check_admin_rbac(pool: &sqlx::PgPool, actor_user_id: i64) -> AppResult<UserAuth> {
        let actor = crate::api::user::repo::find_user(pool, actor_user_id)
            .await?
            .ok_or(AppError::Unauthorized("Actor user not found".into()))?;

        match actor.user_auth {
            UserAuth::Hymn | UserAuth::Admin | UserAuth::Manager => Ok(actor.user_auth),
            _ => Err(AppError::Forbidden("Forbidden".to_string())),
        }
    }

    // =========================================================================
    // 감사 로그 헬퍼
    // =========================================================================

    async fn audit_log(
        st: &AppState,
        actor_id: i64,
        action_type: &str,
        target_id: Option<i64>,
        details: &serde_json::Value,
        ip: Option<IpAddr>,
        ua: Option<&str>,
    ) -> AppResult<()> {
        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let ip_enc = ip
            .map(|ip| crypto.encrypt(&ip.to_string(), "admin_action_log.ip_address"))
            .transpose()?;

        crate::api::admin::user::repo::create_audit_log(
            &st.db,
            actor_id,
            action_type,
            Some("subscriptions"),
            target_id,
            details,
            ip_enc.as_deref(),
            ua,
        )
        .await
    }

    // =========================================================================
    // 구독 목록
    // =========================================================================

    pub async fn list_subscriptions(
        st: &AppState,
        actor_user_id: i64,
        req: AdminSubListReq,
        ip: Option<IpAddr>,
        ua: Option<String>,
    ) -> AppResult<AdminSubListRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let page = req.page.unwrap_or(1).max(1);
        let size = req.size.unwrap_or(20).clamp(1, 100);

        let sort = req.sort.as_deref().unwrap_or("created_at");
        if !matches!(sort, "id" | "created_at" | "status" | "billing_interval" | "price") {
            return Err(AppError::Unprocessable("invalid sort".into()));
        }

        let order = req.order.as_deref().unwrap_or("desc");
        if !matches!(order, "asc" | "desc") {
            return Err(AppError::Unprocessable("invalid order".into()));
        }

        // 상태 필터 검증
        if let Some(ref status) = req.status {
            if !matches!(
                status.as_str(),
                "trialing" | "active" | "past_due" | "paused" | "canceled"
            ) {
                return Err(AppError::Unprocessable("invalid status filter".into()));
            }
        }

        // 검색어 처리 (이메일 blind index 또는 닉네임)
        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let (email_idx, nickname) = if let Some(ref keyword) = req.q {
            if keyword.contains('@') {
                (Some(crypto.blind_index(keyword)?), None)
            } else {
                (None, Some(keyword.as_str()))
            }
        } else {
            (None, None)
        };

        let (total_count, mut items) = AdminPaymentRepo::list_subscriptions(
            &st.db,
            email_idx.as_deref(),
            nickname,
            req.status.as_deref(),
            page,
            size,
            sort,
            order,
        )
        .await?;

        // 이메일 복호화
        for item in &mut items {
            item.user_email = crypto.decrypt(&item.user_email, "users.user_email")?;
        }

        Self::audit_log(
            st,
            actor_user_id,
            "LIST_SUBSCRIPTIONS",
            None,
            &serde_json::json!({ "page": page, "size": size }),
            ip,
            ua.as_deref(),
        )
        .await?;

        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + size - 1) / size
        };

        Ok(AdminSubListRes {
            items,
            meta: AdminPaymentMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page: size,
            },
        })
    }

    // =========================================================================
    // 구독 상세
    // =========================================================================

    pub async fn get_subscription(
        st: &AppState,
        actor_user_id: i64,
        subscription_id: i64,
        ip: Option<IpAddr>,
        ua: Option<String>,
    ) -> AppResult<AdminSubDetailRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);

        let sub = AdminPaymentRepo::get_subscription(&st.db, subscription_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut user = AdminPaymentRepo::get_subscription_user(&st.db, sub.user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // 이메일 복호화
        user.email = crypto.decrypt(&user.email, "users.user_email")?;

        let mut transactions =
            AdminPaymentRepo::list_transactions_for_subscription(&st.db, subscription_id).await?;
        for txn in &mut transactions {
            txn.user_email = crypto.decrypt(&txn.user_email, "users.user_email")?;
        }

        Self::audit_log(
            st,
            actor_user_id,
            "GET_SUBSCRIPTION",
            Some(subscription_id),
            &serde_json::json!({ "subscription_id": subscription_id }),
            ip,
            ua.as_deref(),
        )
        .await?;

        Ok(AdminSubDetailRes {
            subscription: sub,
            user,
            transactions,
        })
    }

    // =========================================================================
    // 관리자 구독 관리 (cancel)
    // =========================================================================

    pub async fn cancel_subscription(
        st: &AppState,
        actor_user_id: i64,
        subscription_id: i64,
        req: AdminCancelSubReq,
        ip: Option<IpAddr>,
        ua: Option<String>,
    ) -> AppResult<AdminSubDetailRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let payment = st
            .payment
            .as_ref()
            .ok_or_else(|| {
                AppError::ServiceUnavailable("Payment provider not configured".into())
            })?;

        let sub = AdminPaymentRepo::get_subscription(&st.db, subscription_id)
            .await?
            .ok_or(AppError::NotFound)?;

        if sub.status == crate::types::SubscriptionStatus::Canceled {
            return Err(AppError::BadRequest(
                "Subscription is already canceled".into(),
            ));
        }

        let effective_from = if req.immediately {
            CancelEffectiveFrom::Immediately
        } else {
            CancelEffectiveFrom::NextBillingPeriod
        };

        payment
            .cancel_subscription(&sub.provider_subscription_id, effective_from)
            .await?;

        tracing::info!(
            admin_id = actor_user_id,
            sub_id = subscription_id,
            immediately = req.immediately,
            "Admin canceled subscription"
        );

        Self::audit_log(
            st,
            actor_user_id,
            "CANCEL_SUBSCRIPTION",
            Some(subscription_id),
            &serde_json::json!({
                "subscription_id": subscription_id,
                "immediately": req.immediately,
            }),
            ip,
            ua.as_deref(),
        )
        .await?;

        // 최신 상태 반환
        Self::get_subscription(st, actor_user_id, subscription_id, None, None).await
    }

    // =========================================================================
    // 트랜잭션 목록
    // =========================================================================

    pub async fn list_transactions(
        st: &AppState,
        actor_user_id: i64,
        req: AdminTxnListReq,
        ip: Option<IpAddr>,
        ua: Option<String>,
    ) -> AppResult<AdminTxnListRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let page = req.page.unwrap_or(1).max(1);
        let size = req.size.unwrap_or(20).clamp(1, 100);

        let sort = req.sort.as_deref().unwrap_or("occurred_at");
        if !matches!(sort, "id" | "occurred_at" | "amount" | "status") {
            return Err(AppError::Unprocessable("invalid sort".into()));
        }

        let order = req.order.as_deref().unwrap_or("desc");
        if !matches!(order, "asc" | "desc") {
            return Err(AppError::Unprocessable("invalid order".into()));
        }

        if let Some(ref status) = req.status {
            if !matches!(
                status.as_str(),
                "completed" | "refunded" | "partially_refunded"
            ) {
                return Err(AppError::Unprocessable("invalid status filter".into()));
            }
        }

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let email_idx = if let Some(ref keyword) = req.q {
            if keyword.contains('@') {
                Some(crypto.blind_index(keyword)?)
            } else {
                None
            }
        } else {
            None
        };

        let (total_count, mut items) = AdminPaymentRepo::list_transactions(
            &st.db,
            email_idx.as_deref(),
            req.status.as_deref(),
            page,
            size,
            sort,
            order,
        )
        .await?;

        for item in &mut items {
            item.user_email = crypto.decrypt(&item.user_email, "users.user_email")?;
        }

        Self::audit_log(
            st,
            actor_user_id,
            "LIST_TRANSACTIONS",
            None,
            &serde_json::json!({ "page": page, "size": size }),
            ip,
            ua.as_deref(),
        )
        .await?;

        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + size - 1) / size
        };

        Ok(AdminTxnListRes {
            items,
            meta: AdminPaymentMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page: size,
            },
        })
    }

    // =========================================================================
    // 수동 수강권
    // =========================================================================

    pub async fn create_grant(
        st: &AppState,
        actor_user_id: i64,
        req: AdminGrantReq,
        ip: Option<IpAddr>,
        ua: Option<String>,
    ) -> AppResult<AdminGrantRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        // 대상 사용자 존재 확인
        let _user = crate::api::user::repo::find_user(&st.db, req.user_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("User not found".into()))?;

        let courses_granted =
            crate::api::payment::repo::PaymentRepo::grant_all_courses(
                &st.db,
                req.user_id,
                req.expire_at,
            )
            .await?;

        tracing::info!(
            admin_id = actor_user_id,
            user_id = req.user_id,
            courses_granted = courses_granted,
            reason = %req.reason,
            "Admin granted courses manually"
        );

        Self::audit_log(
            st,
            actor_user_id,
            "GRANT_COURSES",
            Some(req.user_id),
            &serde_json::json!({
                "user_id": req.user_id,
                "expire_at": req.expire_at.map(|t| t.to_rfc3339()),
                "reason": req.reason,
                "courses_granted": courses_granted,
            }),
            ip,
            ua.as_deref(),
        )
        .await?;

        Ok(AdminGrantRes {
            user_id: req.user_id,
            courses_granted,
            expire_at: req.expire_at.map(|t| t.to_rfc3339()),
        })
    }

    pub async fn list_grants(
        st: &AppState,
        actor_user_id: i64,
        req: AdminGrantListReq,
        ip: Option<IpAddr>,
        ua: Option<String>,
    ) -> AppResult<AdminGrantListRes> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let page = req.page.unwrap_or(1).max(1);
        let size = req.size.unwrap_or(20).clamp(1, 100);

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);

        let (total_count, mut items) =
            AdminPaymentRepo::list_manual_grants(&st.db, page, size).await?;

        for item in &mut items {
            item.user_email = crypto.decrypt(&item.user_email, "users.user_email")?;
        }

        Self::audit_log(
            st,
            actor_user_id,
            "LIST_GRANTS",
            None,
            &serde_json::json!({ "page": page, "size": size }),
            ip,
            ua.as_deref(),
        )
        .await?;

        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + size - 1) / size
        };

        Ok(AdminGrantListRes {
            items,
            meta: AdminPaymentMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page: size,
            },
        })
    }

    pub async fn revoke_grant(
        st: &AppState,
        actor_user_id: i64,
        user_id: i64,
        ip: Option<IpAddr>,
        ua: Option<String>,
    ) -> AppResult<()> {
        Self::check_admin_rbac(&st.db, actor_user_id).await?;

        let revoked =
            crate::api::payment::repo::PaymentRepo::revoke_all_courses(&st.db, user_id).await?;

        tracing::info!(
            admin_id = actor_user_id,
            user_id = user_id,
            courses_revoked = revoked,
            "Admin revoked courses manually"
        );

        Self::audit_log(
            st,
            actor_user_id,
            "REVOKE_COURSES",
            Some(user_id),
            &serde_json::json!({
                "user_id": user_id,
                "courses_revoked": revoked,
            }),
            ip,
            ua.as_deref(),
        )
        .await?;

        Ok(())
    }
}
