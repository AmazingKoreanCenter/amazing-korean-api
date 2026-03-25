use crate::error::{AppError, AppResult};
use crate::external::email::{send_templated, EmailTemplate};
use crate::state::AppState;
use crate::types::{TextbookLanguage, TextbookType};

use super::dto::{
    CatalogItem, CatalogRes, CreateOrderReq, OrderItemRes, OrderRes,
};
use super::repo::{TextbookItemRow, TextbookOrderRow, TextbookRepo};

const UNIT_PRICE: i32 = 25_000; // KRW
const MIN_TOTAL_QUANTITY: i32 = 10;

pub struct TextbookService;

impl TextbookService {
    /// 교재 카탈로그 반환
    pub async fn get_catalog() -> AppResult<CatalogRes> {
        let languages = catalog_languages();
        let items: Vec<CatalogItem> = languages
            .into_iter()
            .map(|(lang, name_ko, name_en, available, isbn_ready)| CatalogItem {
                language: lang,
                language_name_ko: name_ko.to_string(),
                language_name_en: name_en.to_string(),
                available_types: vec![TextbookType::Student, TextbookType::Teacher],
                unit_price: UNIT_PRICE,
                currency: "KRW".to_string(),
                available,
                isbn_ready,
            })
            .collect();

        Ok(CatalogRes {
            items,
            min_total_quantity: MIN_TOTAL_QUANTITY,
            currency: "KRW".to_string(),
        })
    }

    /// 주문 생성 (로그인 필수)
    pub async fn create_order(st: &AppState, user_id: i64, req: CreateOrderReq) -> AppResult<OrderRes> {
        // 항목 수량 검증 (각 항목 1권 이상)
        if req.items.iter().any(|i| i.quantity < 1) {
            return Err(AppError::BadRequest(
                "Each item quantity must be at least 1".into(),
            ));
        }

        // 총 수량 검증
        let total_quantity: i32 = req.items.iter().map(|i| i.quantity).sum();
        if total_quantity < MIN_TOTAL_QUANTITY {
            return Err(AppError::BadRequest(format!(
                "Minimum total quantity is {} copies",
                MIN_TOTAL_QUANTITY
            )));
        }

        // 중복 항목 검증 (같은 language + type 조합 불가)
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

        // 언어 가용성 검증
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

        // 세금계산서 요청 시 홈택스 필수 항목 검증
        if req.tax_invoice {
            if req.tax_biz_number.as_ref().map_or(true, |s| s.is_empty()) {
                return Err(AppError::BadRequest(
                    "Business registration number is required for tax invoice".into(),
                ));
            }
            if req.tax_company_name.as_ref().map_or(true, |s| s.is_empty()) {
                return Err(AppError::BadRequest(
                    "Company name (상호) is required for tax invoice".into(),
                ));
            }
            if req.tax_rep_name.as_ref().map_or(true, |s| s.is_empty()) {
                return Err(AppError::BadRequest(
                    "Representative name (대표자명) is required for tax invoice".into(),
                ));
            }
            if req.tax_email.as_ref().map_or(true, |s| s.is_empty()) {
                return Err(AppError::BadRequest(
                    "Tax invoice email is required for tax invoice".into(),
                ));
            }
        }

        // 총액 계산
        let total_amount: i32 = req.items.iter().map(|i| i.quantity * UNIT_PRICE).sum();

        // 트랜잭션으로 주문 + 항목 생성 (주문번호 생성도 트랜잭션 내에서 — race condition 방지)
        let mut tx = st.db.begin().await?;

        let order_code = TextbookRepo::generate_order_code(&mut tx).await?;

        let order_id = TextbookRepo::insert_order(
            &mut tx,
            &order_code,
            user_id,
            &req.orderer_name,
            &req.orderer_email,
            &req.orderer_phone,
            req.org_name.as_deref(),
            req.org_type.as_deref(),
            req.delivery_postal_code.as_deref(),
            &req.delivery_address,
            req.delivery_detail.as_deref(),
            req.payment_method,
            req.depositor_name.as_deref(),
            req.tax_invoice,
            req.tax_biz_number.as_deref(),
            req.tax_company_name.as_deref(),
            req.tax_rep_name.as_deref(),
            req.tax_address.as_deref(),
            req.tax_biz_type.as_deref(),
            req.tax_biz_item.as_deref(),
            req.tax_email.as_deref(),
            total_quantity,
            total_amount,
            req.notes.as_deref(),
        )
        .await?;

        let items: Vec<(TextbookLanguage, TextbookType, i32, i32)> = req
            .items
            .iter()
            .map(|i| (i.language, i.textbook_type, i.quantity, UNIT_PRICE))
            .collect();

        TextbookRepo::insert_items(&mut tx, order_id, &items).await?;

        tx.commit().await?;

        tracing::info!(
            order_id = order_id,
            order_code = %order_code,
            total_quantity = total_quantity,
            total_amount = total_amount,
            "Textbook order created"
        );

        // 주문 접수 확인 이메일 발송 (실패해도 주문은 유지)
        if let Some(ref email_sender) = st.email {
            let template = EmailTemplate::TextbookOrderConfirmation {
                order_code: order_code.clone(),
                orderer_name: req.orderer_name.clone(),
                total_quantity,
                total_amount,
            };
            if let Err(e) = send_templated(email_sender.as_ref(), &req.orderer_email, template).await {
                tracing::warn!(
                    order_code = %order_code,
                    email = %req.orderer_email,
                    error = %e,
                    "Failed to send order confirmation email (order still created)"
                );
            }
        }

        // 생성된 주문 조회해서 반환
        Self::get_order_by_code(st, &order_code).await
    }

    /// 내 주문 목록 조회
    pub async fn get_my_orders(st: &AppState, user_id: i64) -> AppResult<Vec<OrderRes>> {
        let orders = TextbookRepo::find_by_user_id(&st.db, user_id).await?;
        if orders.is_empty() {
            return Ok(vec![]);
        }

        let order_ids: Vec<i64> = orders.iter().map(|o| o.order_id).collect();
        let all_items = TextbookRepo::find_items_by_orders(&st.db, &order_ids).await?;

        let mut items_map: std::collections::HashMap<i64, Vec<TextbookItemRow>> =
            std::collections::HashMap::new();
        for item in all_items {
            items_map.entry(item.order_id).or_default().push(item);
        }

        Ok(orders
            .into_iter()
            .map(|order| {
                let items = items_map.remove(&order.order_id).unwrap_or_default();
                build_order_res_from(order, items)
            })
            .collect())
    }

    /// 주문번호로 주문 조회
    pub async fn get_order_by_code(st: &AppState, code: &str) -> AppResult<OrderRes> {
        let order = TextbookRepo::find_by_code(&st.db, code)
            .await?
            .ok_or(AppError::NotFound)?;

        let items = TextbookRepo::find_items_by_order(&st.db, order.order_id).await?;

        Ok(build_order_res_from(order, items))
    }

    /// 주문 ID로 주문 조회
    pub async fn get_order_by_id(st: &AppState, order_id: i64) -> AppResult<OrderRes> {
        let order = TextbookRepo::find_by_id(&st.db, order_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let items = TextbookRepo::find_items_by_order(&st.db, order.order_id).await?;

        Ok(build_order_res_from(order, items))
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

pub fn build_order_res_from(order: TextbookOrderRow, items: Vec<TextbookItemRow>) -> OrderRes {
    OrderRes {
        order_id: order.order_id,
        order_code: order.order_code,
        status: order.status,
        orderer_name: order.orderer_name,
        orderer_email: order.orderer_email,
        orderer_phone: order.orderer_phone,
        org_name: order.org_name,
        org_type: order.org_type,
        delivery_postal_code: order.delivery_postal_code,
        delivery_address: order.delivery_address,
        delivery_detail: order.delivery_detail,
        payment_method: order.payment_method,
        depositor_name: order.depositor_name,
        tax_invoice: order.tax_invoice,
        tax_biz_number: order.tax_biz_number,
        tax_company_name: order.tax_company_name,
        tax_rep_name: order.tax_rep_name,
        tax_address: order.tax_address,
        tax_biz_type: order.tax_biz_type,
        tax_biz_item: order.tax_biz_item,
        tax_email: order.tax_email,
        total_quantity: order.total_quantity,
        total_amount: order.total_amount,
        currency: order.currency,
        notes: order.notes,
        tracking_number: order.tracking_number,
        tracking_provider: order.tracking_provider,
        items: items
            .into_iter()
            .map(|i| {
                let language_name = language_display_name(&i.textbook_language);
                OrderItemRes {
                    language: i.textbook_language,
                    language_name,
                    textbook_type: i.textbook_type,
                    quantity: i.quantity,
                    unit_price: i.unit_price,
                    subtotal: i.subtotal,
                }
            })
            .collect(),
        confirmed_at: order.confirmed_at.map(|t| t.to_rfc3339()),
        paid_at: order.paid_at.map(|t| t.to_rfc3339()),
        shipped_at: order.shipped_at.map(|t| t.to_rfc3339()),
        delivered_at: order.delivered_at.map(|t| t.to_rfc3339()),
        canceled_at: order.canceled_at.map(|t| t.to_rfc3339()),
        created_at: order.created_at.to_rfc3339(),
        updated_at: order.updated_at.to_rfc3339(),
    }
}

/// 언어 enum → 한국어 표시명
fn language_display_name(lang: &TextbookLanguage) -> String {
    match lang {
        TextbookLanguage::Ja => "일본어",
        TextbookLanguage::ZhCn => "중국어(간체)",
        TextbookLanguage::ZhTw => "중국어(번체)",
        TextbookLanguage::Vi => "베트남어",
        TextbookLanguage::Th => "태국어",
        TextbookLanguage::Id => "인도네시아어",
        TextbookLanguage::My => "미얀마어",
        TextbookLanguage::Mn => "몽골어",
        TextbookLanguage::Ru => "러시아어",
        TextbookLanguage::Es => "스페인어",
        TextbookLanguage::Pt => "포르투갈어",
        TextbookLanguage::Fr => "프랑스어",
        TextbookLanguage::De => "독일어",
        TextbookLanguage::Hi => "힌디어",
        TextbookLanguage::Ne => "네팔어",
        TextbookLanguage::Si => "싱할라어",
        TextbookLanguage::Km => "크메르어",
        TextbookLanguage::Uz => "우즈베크어",
        TextbookLanguage::Kk => "카자흐어",
        TextbookLanguage::Tg => "타지크어",
        TextbookLanguage::Tl => "필리핀어",
    }
    .to_string()
}

/// 교재 카탈로그 언어 목록 (language, 한국어명, 영어명, 사용가능여부, isbn_ready)
/// ISBN 발급 완료 9개: ja, zh_cn, vi, th, ne, ru, km, tl, id
fn catalog_languages() -> Vec<(TextbookLanguage, &'static str, &'static str, bool, bool)> {
    vec![
        (TextbookLanguage::Ja, "일본어", "Japanese", true, true),
        (TextbookLanguage::ZhCn, "중국어(간체)", "Chinese (Simplified)", true, true),
        (TextbookLanguage::ZhTw, "중국어(번체)", "Chinese (Traditional)", true, false),
        (TextbookLanguage::Vi, "베트남어", "Vietnamese", true, true),
        (TextbookLanguage::Th, "태국어", "Thai", true, true),
        (TextbookLanguage::Id, "인도네시아어", "Indonesian", true, true),
        (TextbookLanguage::My, "미얀마어", "Myanmar", true, false),
        (TextbookLanguage::Mn, "몽골어", "Mongolian", true, false),
        (TextbookLanguage::Ru, "러시아어", "Russian", true, true),
        (TextbookLanguage::Es, "스페인어", "Spanish", true, false),
        (TextbookLanguage::Pt, "포르투갈어", "Portuguese", true, false),
        (TextbookLanguage::Fr, "프랑스어", "French", true, false),
        (TextbookLanguage::De, "독일어", "German", true, false),
        (TextbookLanguage::Hi, "힌디어", "Hindi", true, false),
        (TextbookLanguage::Ne, "네팔어", "Nepali", true, true),
        (TextbookLanguage::Si, "싱할라어", "Sinhala", true, false),
        (TextbookLanguage::Km, "크메르어", "Khmer", true, true),
        (TextbookLanguage::Uz, "우즈베크어", "Uzbek", true, false),
        (TextbookLanguage::Kk, "카자흐어", "Kazakh", true, false),
        (TextbookLanguage::Tg, "타지크어", "Tajik", true, false),
        (TextbookLanguage::Tl, "필리핀어", "Filipino", true, true),
    ]
}
