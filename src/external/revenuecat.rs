use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

use crate::error::{AppError, AppResult};

const REVENUECAT_API_URL: &str = "https://api.revenuecat.com/v1";

/// RevenueCat 클라이언트 trait (테스트 목업 가능)
#[async_trait]
pub trait RevenueCatClient: Send + Sync {
    /// 구독자 정보 조회 (entitlements 포함)
    async fn get_subscriber(&self, app_user_id: &str) -> AppResult<RevenueCatSubscriber>;
}

/// RevenueCat 구독자 정보
#[derive(Debug, Clone)]
pub struct RevenueCatSubscriber {
    pub entitlements: HashMap<String, RevenueCatEntitlement>,
    pub non_subscriptions: HashMap<String, Vec<RevenueCatNonSubscription>>,
}

/// RevenueCat entitlement (구독 기반)
#[derive(Debug, Clone)]
pub struct RevenueCatEntitlement {
    pub is_active: bool,
    pub product_identifier: String,
    pub store: String, // "app_store" | "play_store"
    pub purchase_date: String,
    pub expires_date: Option<String>,
}

/// RevenueCat non-subscription (일회성 구매, e-book 등)
#[derive(Debug, Clone)]
pub struct RevenueCatNonSubscription {
    pub id: String,
    pub store: String,
    pub purchase_date: String,
}

/// RevenueCat REST API 응답 (역직렬화용)
#[derive(Debug, Deserialize)]
struct SubscriberResponse {
    subscriber: SubscriberData,
}

#[derive(Debug, Deserialize)]
struct SubscriberData {
    #[serde(default)]
    entitlements: HashMap<String, EntitlementData>,
    #[serde(default)]
    non_subscriptions: HashMap<String, Vec<NonSubscriptionData>>,
}

#[derive(Debug, Deserialize)]
struct EntitlementData {
    #[serde(default)]
    expires_date: Option<String>,
    product_identifier: String,
    purchase_date: String,
    store: String,
}

#[derive(Debug, Deserialize)]
struct NonSubscriptionData {
    id: String,
    store: String,
    purchase_date: String,
}

/// RevenueCat REST API 클라이언트
pub struct RevenueCatApiClient {
    client: Client,
    api_key: String,
}

impl RevenueCatApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl RevenueCatClient for RevenueCatApiClient {
    async fn get_subscriber(&self, app_user_id: &str) -> AppResult<RevenueCatSubscriber> {
        let url = format!("{}/subscribers/{}", REVENUECAT_API_URL, app_user_id);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| AppError::External(format!("RevenueCat API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::External(format!(
                "RevenueCat API error: {} - {}",
                status, body
            )));
        }

        let data: SubscriberResponse = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("RevenueCat response parse error: {}", e)))?;

        let entitlements = data.subscriber.entitlements.into_iter().map(|(k, v)| {
            // Why: 문자열 사전순 비교는 타임존 포맷(`Z` vs `+00:00`)이나 초 정밀도
            // 차이에서 오판한다. RFC3339 파싱 후 UTC 비교로 정확도 확보.
            let is_active = v.expires_date.as_ref()
                .and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
                .map(|d| d.with_timezone(&chrono::Utc) > chrono::Utc::now())
                .unwrap_or(true); // non-subscription entitlements는 만료 없음

            (k, RevenueCatEntitlement {
                is_active,
                product_identifier: v.product_identifier,
                store: v.store,
                purchase_date: v.purchase_date,
                expires_date: v.expires_date,
            })
        }).collect();

        let non_subscriptions = data.subscriber.non_subscriptions.into_iter().map(|(k, v)| {
            (k, v.into_iter().map(|ns| RevenueCatNonSubscription {
                id: ns.id,
                store: ns.store,
                purchase_date: ns.purchase_date,
            }).collect())
        }).collect();

        Ok(RevenueCatSubscriber {
            entitlements,
            non_subscriptions,
        })
    }
}
