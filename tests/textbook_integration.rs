//! Phase 3 통합 테스트 — `TextbookService` (B5 트랙).
//!
//! ## 범위 — 주문 lookup non-existent path

mod common;

use amazing_korean_api::api::textbook::dto::{CreateOrderItemReq, CreateOrderReq};
use amazing_korean_api::api::textbook::service::TextbookService;
use amazing_korean_api::error::AppError;
use amazing_korean_api::types::{TextbookLanguage, TextbookPaymentMethod, TextbookType};
use common::{cleanup_test_user, insert_test_user, TestUserSpec};

fn minimal_order_req(orderer_email: &str) -> CreateOrderReq {
    CreateOrderReq {
        orderer_name: "Phase3 Tester".to_string(),
        orderer_email: orderer_email.to_string(),
        orderer_phone: "010-1234-5678".to_string(),
        org_name: None,
        org_type: None,
        delivery_postal_code: None,
        delivery_address: "Seoul, KR".to_string(),
        delivery_detail: None,
        payment_method: TextbookPaymentMethod::BankTransfer,
        depositor_name: None,
        tax_invoice: false,
        tax_biz_number: None,
        tax_company_name: None,
        tax_rep_name: None,
        tax_address: None,
        tax_biz_type: None,
        tax_biz_item: None,
        tax_email: None,
        items: vec![CreateOrderItemReq {
            language: TextbookLanguage::Ja,
            textbook_type: TextbookType::Student,
            quantity: 10, // MIN_TOTAL_QUANTITY 충족
        }],
        notes: None,
    }
}

async fn cleanup_textbook_order_by_user_id(st: &amazing_korean_api::state::AppState, user_id: i64) {
    // textbook_order_item → textbook_order → user 순서 삭제 (FK)
    let _ = sqlx::query(
        "DELETE FROM textbook_order_item WHERE order_id IN (SELECT order_id FROM textbook_order WHERE user_id = $1)",
    )
    .bind(user_id)
    .execute(&st.db)
    .await;
    let _ = sqlx::query("DELETE FROM textbook_order WHERE user_id = $1")
        .bind(user_id)
        .execute(&st.db)
        .await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_my_orders_returns_empty_for_user_without_orders() {
    let st = common::make_test_state().await;

    let result = TextbookService::get_my_orders(&st, 999_999_980).await;
    let orders = match result {
        Ok(o) => o,
        Err(e) => panic!("non-existent user → Ok(empty) expected, got Err: {:?}", e),
    };
    assert!(orders.is_empty(), "orders=empty for user without orders");
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_order_by_code_returns_not_found_for_unknown_code() {
    let st = common::make_test_state().await;

    let unknown_code = format!("phase3_textbook_{}", uuid::Uuid::new_v4());
    let result = TextbookService::get_order_by_code(&st, &unknown_code).await;
    match result {
        Err(AppError::NotFound) => {}
        Err(e) => panic!("unknown code → NotFound expected, got Err: {:?}", e),
        Ok(_) => panic!("unknown code → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_order_by_id_returns_not_found_for_unknown_id() {
    let st = common::make_test_state().await;

    let result = TextbookService::get_order_by_id(&st, 999_999_981).await;
    match result {
        Err(AppError::NotFound) => {}
        Err(e) => panic!("unknown id → NotFound expected, got Err: {:?}", e),
        Ok(_) => panic!("unknown id → Err expected, got Ok"),
    }
}

// =============================================================================
// C-textbook — create_order validation + happy path
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_create_order_rejects_quantity_below_one() {
    let st = common::make_test_state().await;

    let mut req = minimal_order_req("phase3-quantity@example.com");
    req.items[0].quantity = 0;

    let result = TextbookService::create_order(&st, 999_999_982, req).await;
    match result {
        Err(AppError::BadRequest(msg)) => {
            assert!(
                msg.contains("quantity"),
                "msg에 'quantity' 포함, got: {}",
                msg
            );
        }
        Err(e) => panic!("quantity=0 → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("quantity=0 → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_create_order_rejects_total_quantity_below_minimum() {
    // MIN_TOTAL_QUANTITY = 10. 단일 아이템 quantity=5 → BadRequest.
    let st = common::make_test_state().await;

    let mut req = minimal_order_req("phase3-min@example.com");
    req.items[0].quantity = 5;

    let result = TextbookService::create_order(&st, 999_999_983, req).await;
    match result {
        Err(AppError::BadRequest(msg)) => {
            assert!(
                msg.contains("Minimum"),
                "msg에 'Minimum' 포함, got: {}",
                msg
            );
        }
        Err(e) => panic!("total<10 → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("total<10 → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_create_order_rejects_duplicate_item_combination() {
    // 같은 (language, textbook_type) 조합 두 번 → BadRequest.
    let st = common::make_test_state().await;

    let mut req = minimal_order_req("phase3-dup@example.com");
    req.items = vec![
        CreateOrderItemReq {
            language: TextbookLanguage::Ja,
            textbook_type: TextbookType::Student,
            quantity: 5,
        },
        CreateOrderItemReq {
            language: TextbookLanguage::Ja, // 중복
            textbook_type: TextbookType::Student,
            quantity: 5,
        },
    ];

    let result = TextbookService::create_order(&st, 999_999_984, req).await;
    match result {
        Err(AppError::BadRequest(msg)) => {
            assert!(
                msg.contains("Duplicate"),
                "msg에 'Duplicate' 포함, got: {}",
                msg
            );
        }
        Err(e) => panic!("duplicate item → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("duplicate item → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_create_order_rejects_tax_invoice_missing_required_fields() {
    // tax_invoice=true 인데 tax_biz_number/company_name/rep_name/email 누락 → BadRequest.
    let st = common::make_test_state().await;

    let mut req = minimal_order_req("phase3-tax@example.com");
    req.tax_invoice = true;
    // tax_biz_number/company_name/rep_name/email 모두 None

    let result = TextbookService::create_order(&st, 999_999_985, req).await;
    match result {
        Err(AppError::BadRequest(msg)) => {
            assert!(
                msg.contains("Business registration") || msg.contains("required"),
                "msg에 'Business registration' 포함, got: {}",
                msg
            );
        }
        Err(e) => panic!("tax_invoice 누락 → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("tax_invoice 누락 → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_create_order_happy_path_persists_order_and_sends_email() {
    // 정상 주문 생성 → DB 저장 + EmailSender mock 캡처 1건 (TextbookOrderConfirmation).
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let spec = TestUserSpec::random();
    let user_id = insert_test_user(&st, &spec).await;

    let orderer_email = format!("phase3-textbook-{}@example.com", uuid::Uuid::new_v4());
    let req = minimal_order_req(&orderer_email);

    let result = TextbookService::create_order(&st, user_id, req).await;
    let order = match result {
        Ok(o) => o,
        Err(e) => panic!("happy path → Ok expected, got Err: {:?}", e),
    };

    assert!(!order.order_code.is_empty(), "order_code 발급");
    assert_eq!(order.total_quantity, 10);
    assert_eq!(order.items.len(), 1);

    // 이메일 1건 캡처 (TextbookOrderConfirmation)
    let captured = sent.lock().await;
    assert_eq!(captured.len(), 1, "이메일 1건, got: {}", captured.len());
    let mail = &captured[0];
    assert_eq!(mail.to, orderer_email);
    assert!(
        mail.subject.contains("교재 주문") && mail.subject.contains(&order.order_code),
        "subject 에 '교재 주문' + order_code 포함, got: {}",
        mail.subject
    );
    drop(captured);

    // Cleanup: textbook_order/items → user
    cleanup_textbook_order_by_user_id(&st, user_id).await;
    cleanup_test_user(&st, user_id).await;
}
