use paddle_rust_sdk::entities::{Event, Subscription as PaddleSubscription};
use paddle_rust_sdk::enums::EventData;

use crate::error::{AppError, AppResult};
use crate::external::payment::CancelEffectiveFrom;
use crate::state::AppState;
use crate::types::{BillingInterval, PaymentProvider, SubscriptionStatus, TransactionStatus};

use super::dto::{PlanInfo, PlansRes, SubscriptionInfo, SubscriptionRes};
use super::repo::PaymentRepo;

pub struct PaymentService;

impl PaymentService {
    /// 사용 가능한 구독 플랜 목록 반환
    pub async fn get_plans(st: &AppState) -> AppResult<PlansRes> {
        let payment = st
            .payment
            .as_ref()
            .ok_or_else(|| AppError::ServiceUnavailable("Payment provider not configured".into()))?;

        let intervals = [
            (BillingInterval::Month1, "Monthly"),
            (BillingInterval::Month3, "Quarterly"),
            (BillingInterval::Month6, "Semi-Annual"),
            (BillingInterval::Month12, "Annual"),
        ];

        let mut plans = Vec::new();
        for (interval, label) in intervals {
            let price_id = match interval {
                BillingInterval::Month1 => st.cfg.paddle_price_month_1.clone(),
                BillingInterval::Month3 => st.cfg.paddle_price_month_3.clone(),
                BillingInterval::Month6 => st.cfg.paddle_price_month_6.clone(),
                BillingInterval::Month12 => st.cfg.paddle_price_month_12.clone(),
            };

            if let Some(price_id) = price_id {
                let cents = interval.price_cents();
                plans.push(PlanInfo {
                    interval,
                    months: interval.months(),
                    price_cents: cents,
                    price_display: format!("${}.{:02}", cents / 100, cents % 100),
                    price_id,
                    trial_days: 1,
                    label: label.to_string(),
                });
            }
        }

        Ok(PlansRes {
            client_token: payment.client_token().to_string(),
            sandbox: payment.is_sandbox(),
            plans,
        })
    }

    /// 현재 사용자의 구독 상태 조회
    pub async fn get_subscription(st: &AppState, user_id: i64) -> AppResult<SubscriptionRes> {
        let row = PaymentRepo::get_latest_subscription(&st.db, user_id).await?;

        let subscription = row.map(|r| SubscriptionInfo {
            subscription_id: r.subscription_id,
            status: r.status,
            billing_interval: r.billing_interval,
            current_price_cents: r.current_price_cents,
            trial_ends_at: r.trial_ends_at.map(|t| t.to_rfc3339()),
            current_period_start: r.current_period_start.map(|t| t.to_rfc3339()),
            current_period_end: r.current_period_end.map(|t| t.to_rfc3339()),
            canceled_at: r.canceled_at.map(|t| t.to_rfc3339()),
            paused_at: r.paused_at.map(|t| t.to_rfc3339()),
            created_at: r.created_at.to_rfc3339(),
        });

        Ok(SubscriptionRes { subscription })
    }

    /// 사용자가 활성 구독을 보유하고 있는지 확인
    pub async fn has_active_subscription(st: &AppState, user_id: i64) -> AppResult<bool> {
        let row = PaymentRepo::get_active_subscription(&st.db, user_id).await?;
        Ok(row.is_some())
    }

    // =========================================================================
    // 구독 관리 (cancel)
    // =========================================================================

    /// 구독 취소
    ///
    /// immediately=true → 즉시 취소, false → 다음 결제일에 취소.
    /// Paddle API 호출 후 DB 상태는 webhook이 업데이트.
    pub async fn cancel_subscription(
        st: &AppState,
        user_id: i64,
        immediately: bool,
    ) -> AppResult<SubscriptionRes> {
        let payment = st
            .payment
            .as_ref()
            .ok_or_else(|| AppError::ServiceUnavailable("Payment provider not configured".into()))?;

        // get_latest_subscription 사용 — paused 상태에서도 취소 가능
        let sub = PaymentRepo::get_latest_subscription(&st.db, user_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("No subscription found".into()))?;

        if sub.status == SubscriptionStatus::Canceled {
            return Err(AppError::BadRequest("Subscription is already canceled".into()));
        }

        let effective_from = if immediately {
            CancelEffectiveFrom::Immediately
        } else {
            CancelEffectiveFrom::NextBillingPeriod
        };

        payment
            .cancel_subscription(&sub.provider_subscription_id, effective_from)
            .await?;

        tracing::info!(
            user_id = user_id,
            sub_id = %sub.provider_subscription_id,
            immediately = immediately,
            "Subscription cancel requested"
        );

        // 최신 상태 반환 (webhook이 곧 DB를 업데이트)
        Self::get_subscription(st, user_id).await
    }

    // =========================================================================
    // Webhook 이벤트 처리
    // =========================================================================

    /// Paddle webhook 이벤트 처리 (handler에서 호출)
    pub async fn process_webhook_event(
        st: &AppState,
        event: Event,
        raw_body: &str,
    ) -> AppResult<()> {
        let event_id = event.event_id.to_string();

        // 멱등성 체크 — 이미 처리된 이벤트면 스킵
        if PaymentRepo::is_webhook_event_processed(&st.db, PaymentProvider::Paddle, &event_id)
            .await?
        {
            tracing::info!(event_id = %event_id, "Webhook event already processed, skipping");
            return Ok(());
        }

        // 이벤트 타입 문자열 추출 + 처리
        let event_type = event_data_type_name(&event.data);
        tracing::info!(event_id = %event_id, event_type = %event_type, "Processing webhook event");

        match &event.data {
            // --- Subscription 이벤트 ---
            EventData::SubscriptionCreated(sub) => {
                Self::handle_subscription_created(st, sub).await?;
            }
            EventData::SubscriptionActivated(sub)
            | EventData::SubscriptionResumed(sub) => {
                Self::handle_subscription_activated(st, sub).await?;
            }
            EventData::SubscriptionUpdated(sub) => {
                Self::handle_subscription_updated(st, sub).await?;
            }
            EventData::SubscriptionCanceled(sub) => {
                Self::handle_subscription_canceled(st, sub).await?;
            }
            EventData::SubscriptionPaused(sub) => {
                Self::handle_subscription_paused(st, sub).await?;
            }
            EventData::SubscriptionPastDue(sub) => {
                Self::handle_subscription_past_due(st, sub).await?;
            }
            EventData::SubscriptionTrialing(sub) => {
                Self::handle_subscription_trialing(st, sub).await?;
            }

            // --- Transaction 이벤트 ---
            EventData::TransactionCompleted(txn) => {
                Self::handle_transaction_completed(st, txn).await?;
            }

            // 기타 이벤트는 무시
            _ => {
                tracing::debug!(event_type = %event_type, "Ignoring unhandled event type");
            }
        }

        // 이벤트 기록 (멱등성 보장)
        let payload: serde_json::Value =
            serde_json::from_str(raw_body).unwrap_or(serde_json::Value::Null);
        PaymentRepo::record_webhook_event(
            &st.db,
            PaymentProvider::Paddle,
            &event_id,
            &event_type,
            payload,
        )
        .await?;

        Ok(())
    }

    // =========================================================================
    // Subscription 핸들러
    // =========================================================================

    /// subscription.created — 구독 생성 (trialing 상태)
    async fn handle_subscription_created(
        st: &AppState,
        sub: &PaddleSubscription,
    ) -> AppResult<()> {
        let user_id = match extract_user_id(sub) {
            Some(id) => id,
            None => {
                tracing::error!(
                    sub_id = %sub.id,
                    "subscription.created: missing user_id in custom_data"
                );
                return Ok(());
            }
        };

        let provider_sub_id = sub.id.to_string();
        let customer_id = sub.customer_id.to_string();
        let billing_interval = extract_billing_interval(st, sub);
        let price_cents = billing_interval.map(|b| b.price_cents()).unwrap_or(0);

        let period_start = sub.current_billing_period.as_ref().map(|p| p.starts_at);
        let period_end = sub.current_billing_period.as_ref().map(|p| p.ends_at);

        // trial_dates는 items에서 추출
        let trial_ends = sub
            .items
            .first()
            .and_then(|item| item.trial_dates.as_ref())
            .map(|td| td.ends_at);
        let trial_starts = sub
            .items
            .first()
            .and_then(|item| item.trial_dates.as_ref())
            .map(|td| td.starts_at);

        PaymentRepo::create_subscription(
            &st.db,
            user_id,
            PaymentProvider::Paddle,
            &provider_sub_id,
            Some(&customer_id),
            SubscriptionStatus::Trialing,
            billing_interval.unwrap_or(BillingInterval::Month1),
            price_cents,
            trial_starts,
            trial_ends,
            period_start,
            period_end,
            sub.custom_data.clone(),
        )
        .await?;

        tracing::info!(
            user_id = user_id,
            sub_id = %provider_sub_id,
            "Subscription created (trialing)"
        );

        Ok(())
    }

    /// subscription.activated / subscription.resumed — 구독 활성화 + 수강권 부여
    async fn handle_subscription_activated(
        st: &AppState,
        sub: &PaddleSubscription,
    ) -> AppResult<()> {
        let provider_sub_id = sub.id.to_string();
        let period_start = sub.current_billing_period.as_ref().map(|p| p.starts_at);
        let period_end = sub.current_billing_period.as_ref().map(|p| p.ends_at);

        // DB 구독 상태 업데이트
        PaymentRepo::update_subscription_status(
            &st.db,
            &provider_sub_id,
            SubscriptionStatus::Active,
            period_start,
            period_end,
            None, // canceled_at
            None, // paused_at (재개 시 null로)
        )
        .await?;

        // user_course 수강권 부여
        let existing = PaymentRepo::get_subscription_by_provider_id(&st.db, &provider_sub_id).await?;
        if let Some(row) = existing {
            let granted = PaymentRepo::grant_all_courses(&st.db, row.user_id, period_end).await?;
            tracing::info!(
                user_id = row.user_id,
                sub_id = %provider_sub_id,
                courses_granted = granted,
                "Subscription activated — courses granted"
            );
        }

        Ok(())
    }

    /// subscription.updated — 구독 정보 업데이트 (billing period 갱신 등)
    async fn handle_subscription_updated(
        st: &AppState,
        sub: &PaddleSubscription,
    ) -> AppResult<()> {
        let provider_sub_id = sub.id.to_string();
        let status = paddle_status_to_internal(&format!("{:?}", sub.status));
        let period_start = sub.current_billing_period.as_ref().map(|p| p.starts_at);
        let period_end = sub.current_billing_period.as_ref().map(|p| p.ends_at);

        PaymentRepo::update_subscription_status(
            &st.db,
            &provider_sub_id,
            status,
            period_start,
            period_end,
            sub.canceled_at,
            sub.paused_at,
        )
        .await?;

        // 활성 상태면 수강권 만료일도 갱신
        if matches!(
            status,
            SubscriptionStatus::Active | SubscriptionStatus::Trialing
        ) {
            let existing =
                PaymentRepo::get_subscription_by_provider_id(&st.db, &provider_sub_id).await?;
            if let Some(row) = existing {
                PaymentRepo::update_course_expiry(&st.db, row.user_id, period_end).await?;
            }
        }

        tracing::info!(sub_id = %provider_sub_id, ?status, "Subscription updated");
        Ok(())
    }

    /// subscription.canceled — 구독 취소 (current_period_end까지 유지)
    async fn handle_subscription_canceled(
        st: &AppState,
        sub: &PaddleSubscription,
    ) -> AppResult<()> {
        let provider_sub_id = sub.id.to_string();
        let period_end = sub.current_billing_period.as_ref().map(|p| p.ends_at);

        PaymentRepo::update_subscription_status(
            &st.db,
            &provider_sub_id,
            SubscriptionStatus::Canceled,
            None,
            None,
            sub.canceled_at,
            None,
        )
        .await?;

        // 수강권 만료일을 현재 기간 종료일로 설정 (즉시 해제하지 않음)
        let existing =
            PaymentRepo::get_subscription_by_provider_id(&st.db, &provider_sub_id).await?;
        if let Some(row) = existing {
            PaymentRepo::update_course_expiry(&st.db, row.user_id, period_end).await?;
            tracing::info!(
                user_id = row.user_id,
                sub_id = %provider_sub_id,
                expire_at = ?period_end,
                "Subscription canceled — courses expire at period end"
            );
        }

        Ok(())
    }

    /// subscription.paused — 구독 일시정지 + 수강권 비활성화
    async fn handle_subscription_paused(
        st: &AppState,
        sub: &PaddleSubscription,
    ) -> AppResult<()> {
        let provider_sub_id = sub.id.to_string();

        PaymentRepo::update_subscription_status(
            &st.db,
            &provider_sub_id,
            SubscriptionStatus::Paused,
            None,
            None,
            None,
            sub.paused_at,
        )
        .await?;

        // 수강권 즉시 비활성화
        let existing =
            PaymentRepo::get_subscription_by_provider_id(&st.db, &provider_sub_id).await?;
        if let Some(row) = existing {
            let revoked = PaymentRepo::revoke_all_courses(&st.db, row.user_id).await?;
            tracing::info!(
                user_id = row.user_id,
                sub_id = %provider_sub_id,
                courses_revoked = revoked,
                "Subscription paused — courses revoked"
            );
        }

        Ok(())
    }

    /// subscription.past_due — 결제 실패 (수강권은 유지, 유예 기간)
    async fn handle_subscription_past_due(
        st: &AppState,
        sub: &PaddleSubscription,
    ) -> AppResult<()> {
        let provider_sub_id = sub.id.to_string();

        PaymentRepo::update_subscription_status(
            &st.db,
            &provider_sub_id,
            SubscriptionStatus::PastDue,
            None,
            None,
            None,
            None,
        )
        .await?;

        tracing::warn!(sub_id = %provider_sub_id, "Subscription past due — courses maintained during grace period");
        Ok(())
    }

    /// subscription.trialing — 체험 시작 + 수강권 부여
    async fn handle_subscription_trialing(
        st: &AppState,
        sub: &PaddleSubscription,
    ) -> AppResult<()> {
        let provider_sub_id = sub.id.to_string();

        let trial_end = sub
            .items
            .first()
            .and_then(|item| item.trial_dates.as_ref())
            .map(|td| td.ends_at);
        let period_end = sub.current_billing_period.as_ref().map(|p| p.ends_at);
        let expire_at = trial_end.or(period_end);

        PaymentRepo::update_subscription_status(
            &st.db,
            &provider_sub_id,
            SubscriptionStatus::Trialing,
            sub.current_billing_period.as_ref().map(|p| p.starts_at),
            period_end,
            None,
            None,
        )
        .await?;

        // 체험 기간에도 수강권 부여
        let existing =
            PaymentRepo::get_subscription_by_provider_id(&st.db, &provider_sub_id).await?;
        if let Some(row) = existing {
            let granted = PaymentRepo::grant_all_courses(&st.db, row.user_id, expire_at).await?;
            tracing::info!(
                user_id = row.user_id,
                sub_id = %provider_sub_id,
                courses_granted = granted,
                "Subscription trialing — courses granted"
            );
        }

        Ok(())
    }

    // =========================================================================
    // Transaction 핸들러
    // =========================================================================

    /// transaction.completed — 결제 완료 기록
    async fn handle_transaction_completed(
        st: &AppState,
        txn: &paddle_rust_sdk::entities::Transaction,
    ) -> AppResult<()> {
        let provider_txn_id = txn.id.to_string();

        // 금액 파싱 (Paddle은 최소 통화 단위 문자열로 반환)
        let amount_cents = txn
            .details
            .totals
            .total
            .parse::<i32>()
            .unwrap_or(0);
        let tax_cents = txn
            .details
            .totals
            .tax
            .parse::<i32>()
            .unwrap_or(0);
        let currency = format!("{:?}", txn.details.totals.currency_code);

        // 연결된 구독에서 user_id 조회
        let (subscription_id, user_id, billing_interval) =
            if let Some(ref paddle_sub_id) = txn.subscription_id {
                let sub_id_str = paddle_sub_id.to_string();
                let row =
                    PaymentRepo::get_subscription_by_provider_id(&st.db, &sub_id_str).await?;
                match row {
                    Some(r) => (Some(r.subscription_id), r.user_id, Some(r.billing_interval)),
                    None => {
                        // custom_data에서 user_id 시도
                        let uid = txn
                            .custom_data
                            .as_ref()
                            .and_then(|cd| cd["user_id"].as_str())
                            .and_then(|s| s.parse::<i64>().ok())
                            .unwrap_or(0);
                        if uid == 0 {
                            tracing::error!(
                                txn_id = %provider_txn_id,
                                "transaction.completed: cannot determine user_id"
                            );
                            return Ok(());
                        }
                        (None, uid, None)
                    }
                }
            } else {
                // 구독 없는 일회성 결제 (현재 지원 안 함)
                tracing::warn!(txn_id = %provider_txn_id, "Transaction without subscription_id, skipping");
                return Ok(());
            };

        PaymentRepo::create_transaction(
            &st.db,
            subscription_id,
            user_id,
            PaymentProvider::Paddle,
            &provider_txn_id,
            TransactionStatus::Completed,
            amount_cents,
            tax_cents,
            &currency,
            billing_interval,
            txn.custom_data.clone(),
            txn.created_at,
        )
        .await?;

        tracing::info!(
            user_id = user_id,
            txn_id = %provider_txn_id,
            amount_cents = amount_cents,
            "Transaction completed"
        );

        Ok(())
    }
}

// =============================================================================
// 헬퍼 함수
// =============================================================================

/// Subscription의 custom_data에서 user_id 추출
fn extract_user_id(sub: &PaddleSubscription) -> Option<i64> {
    sub.custom_data
        .as_ref()?
        .get("user_id")?
        .as_str()
        .and_then(|s| s.parse::<i64>().ok())
}

/// Subscription의 items에서 price_id를 추출하고 BillingInterval로 매핑
fn extract_billing_interval(st: &AppState, sub: &PaddleSubscription) -> Option<BillingInterval> {
    let price_id = sub.items.first()?.price.id.to_string();
    st.cfg.billing_interval_for_price(&price_id)
}

/// Paddle SDK의 SubscriptionStatus → 내부 SubscriptionStatus 변환
fn paddle_status_to_internal(status_debug: &str) -> SubscriptionStatus {
    let s = status_debug.to_lowercase();
    if s.contains("active") {
        SubscriptionStatus::Active
    } else if s.contains("trial") {
        SubscriptionStatus::Trialing
    } else if s.contains("past") {
        SubscriptionStatus::PastDue
    } else if s.contains("pause") {
        SubscriptionStatus::Paused
    } else if s.contains("cancel") {
        SubscriptionStatus::Canceled
    } else {
        SubscriptionStatus::Active
    }
}

/// EventData → 이벤트 타입 문자열
fn event_data_type_name(data: &EventData) -> String {
    match data {
        EventData::SubscriptionCreated(_) => "subscription.created",
        EventData::SubscriptionActivated(_) => "subscription.activated",
        EventData::SubscriptionUpdated(_) => "subscription.updated",
        EventData::SubscriptionCanceled(_) => "subscription.canceled",
        EventData::SubscriptionPaused(_) => "subscription.paused",
        EventData::SubscriptionResumed(_) => "subscription.resumed",
        EventData::SubscriptionTrialing(_) => "subscription.trialing",
        EventData::SubscriptionPastDue(_) => "subscription.past_due",
        EventData::TransactionCompleted(_) => "transaction.completed",
        EventData::TransactionCreated(_) => "transaction.created",
        EventData::TransactionBilled(_) => "transaction.billed",
        EventData::TransactionPaid(_) => "transaction.paid",
        EventData::TransactionUpdated(_) => "transaction.updated",
        EventData::TransactionCanceled(_) => "transaction.canceled",
        _ => "unknown",
    }
    .to_string()
}
