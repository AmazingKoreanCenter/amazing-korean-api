use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::error::AppResult;
use crate::types::{
    TextbookLanguage, TextbookOrderStatus, TextbookPaymentMethod, TextbookType,
};

// =============================================================================
// Row Types
// =============================================================================

#[derive(Debug, sqlx::FromRow)]
pub struct TextbookOrderRow {
    pub order_id: i64,
    pub order_code: String,
    pub status: TextbookOrderStatus,
    pub orderer_name: String,
    pub orderer_email: String,
    pub orderer_phone: String,
    pub org_name: Option<String>,
    pub org_type: Option<String>,
    pub delivery_postal_code: Option<String>,
    pub delivery_address: String,
    pub delivery_detail: Option<String>,
    pub payment_method: TextbookPaymentMethod,
    pub depositor_name: Option<String>,
    pub tax_invoice: bool,
    pub tax_biz_number: Option<String>,
    pub tax_email: Option<String>,
    pub total_quantity: i32,
    pub total_amount: i32,
    pub currency: String,
    pub notes: Option<String>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TextbookItemRow {
    pub item_id: i64,
    pub order_id: i64,
    pub textbook_language: TextbookLanguage,
    pub textbook_type: TextbookType,
    pub quantity: i32,
    pub unit_price: i32,
    pub subtotal: i32,
}

pub struct TextbookRepo;

impl TextbookRepo {
    // =========================================================================
    // Order Code Generation
    // =========================================================================

    /// TB-YYMMDD-NNNN 형식 주문번호 생성
    pub async fn generate_order_code(pool: &PgPool) -> AppResult<String> {
        let today = Utc::now().format("%y%m%d").to_string();
        let prefix = format!("TB-{}-", today);

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM textbook WHERE order_code LIKE $1 || '%'",
        )
        .bind(&prefix)
        .fetch_one(pool)
        .await?;

        Ok(format!("TB-{}-{:04}", today, count + 1))
    }

    // =========================================================================
    // Order CRUD
    // =========================================================================

    /// 주문 생성 (트랜잭션 내에서 호출)
    pub async fn insert_order(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        order_code: &str,
        orderer_name: &str,
        orderer_email: &str,
        orderer_phone: &str,
        org_name: Option<&str>,
        org_type: Option<&str>,
        delivery_postal_code: Option<&str>,
        delivery_address: &str,
        delivery_detail: Option<&str>,
        payment_method: TextbookPaymentMethod,
        depositor_name: Option<&str>,
        tax_invoice: bool,
        tax_biz_number: Option<&str>,
        tax_email: Option<&str>,
        total_quantity: i32,
        total_amount: i32,
        notes: Option<&str>,
    ) -> AppResult<i64> {
        let order_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO textbook (
                order_code, orderer_name, orderer_email, orderer_phone,
                org_name, org_type,
                delivery_postal_code, delivery_address, delivery_detail,
                payment_method, depositor_name,
                tax_invoice, tax_biz_number, tax_email,
                total_quantity, total_amount, notes
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING order_id
            "#,
        )
        .bind(order_code)
        .bind(orderer_name)
        .bind(orderer_email)
        .bind(orderer_phone)
        .bind(org_name)
        .bind(org_type)
        .bind(delivery_postal_code)
        .bind(delivery_address)
        .bind(delivery_detail)
        .bind(payment_method)
        .bind(depositor_name)
        .bind(tax_invoice)
        .bind(tax_biz_number)
        .bind(tax_email)
        .bind(total_quantity)
        .bind(total_amount)
        .bind(notes)
        .fetch_one(&mut **tx)
        .await?;

        Ok(order_id)
    }

    /// 주문 항목 일괄 삽입
    pub async fn insert_items(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        order_id: i64,
        items: &[(TextbookLanguage, TextbookType, i32, i32)], // (lang, type, qty, unit_price)
    ) -> AppResult<()> {
        for (lang, tb_type, qty, unit_price) in items {
            let subtotal = qty * unit_price;
            sqlx::query(
                r#"
                INSERT INTO textbook_item (order_id, textbook_language, textbook_type, quantity, unit_price, subtotal)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(order_id)
            .bind(lang)
            .bind(tb_type)
            .bind(qty)
            .bind(unit_price)
            .bind(subtotal)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    /// 주문번호로 주문 조회
    pub async fn find_by_code(
        pool: &PgPool,
        order_code: &str,
    ) -> AppResult<Option<TextbookOrderRow>> {
        let row = sqlx::query_as::<_, TextbookOrderRow>(
            r#"
            SELECT order_id, order_code, status,
                   orderer_name, orderer_email, orderer_phone,
                   org_name, org_type,
                   delivery_postal_code, delivery_address, delivery_detail,
                   payment_method, depositor_name,
                   tax_invoice, tax_biz_number, tax_email,
                   total_quantity, total_amount, currency, notes,
                   confirmed_at, paid_at, shipped_at, delivered_at, canceled_at,
                   created_at, updated_at
            FROM textbook
            WHERE order_code = $1
            "#,
        )
        .bind(order_code)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 주문 ID로 주문 조회
    pub async fn find_by_id(
        pool: &PgPool,
        order_id: i64,
    ) -> AppResult<Option<TextbookOrderRow>> {
        let row = sqlx::query_as::<_, TextbookOrderRow>(
            r#"
            SELECT order_id, order_code, status,
                   orderer_name, orderer_email, orderer_phone,
                   org_name, org_type,
                   delivery_postal_code, delivery_address, delivery_detail,
                   payment_method, depositor_name,
                   tax_invoice, tax_biz_number, tax_email,
                   total_quantity, total_amount, currency, notes,
                   confirmed_at, paid_at, shipped_at, delivered_at, canceled_at,
                   created_at, updated_at
            FROM textbook
            WHERE order_id = $1
            "#,
        )
        .bind(order_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 주문 항목 조회
    pub async fn find_items_by_order(
        pool: &PgPool,
        order_id: i64,
    ) -> AppResult<Vec<TextbookItemRow>> {
        let rows = sqlx::query_as::<_, TextbookItemRow>(
            r#"
            SELECT item_id, order_id, textbook_language, textbook_type,
                   quantity, unit_price, subtotal
            FROM textbook_item
            WHERE order_id = $1
            ORDER BY item_id
            "#,
        )
        .bind(order_id)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    // =========================================================================
    // Admin: 목록 조회
    // =========================================================================

    /// 주문 목록 조회 (페이지네이션 + 필터)
    pub async fn list_orders(
        pool: &PgPool,
        status: Option<TextbookOrderStatus>,
        search: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> AppResult<(Vec<TextbookOrderRow>, i64)> {
        let offset = (page - 1) * per_page;

        // 총 개수
        let total: i64 = if let Some(s) = status {
            if let Some(q) = search {
                let pattern = format!("%{}%", q);
                sqlx::query_scalar(
                    "SELECT COUNT(*) FROM textbook WHERE status = $1 AND (orderer_name ILIKE $2 OR order_code ILIKE $2 OR org_name ILIKE $2)",
                )
                .bind(s)
                .bind(&pattern)
                .fetch_one(pool)
                .await?
            } else {
                sqlx::query_scalar("SELECT COUNT(*) FROM textbook WHERE status = $1")
                    .bind(s)
                    .fetch_one(pool)
                    .await?
            }
        } else if let Some(q) = search {
            let pattern = format!("%{}%", q);
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM textbook WHERE orderer_name ILIKE $1 OR order_code ILIKE $1 OR org_name ILIKE $1",
            )
            .bind(&pattern)
            .fetch_one(pool)
            .await?
        } else {
            sqlx::query_scalar("SELECT COUNT(*) FROM textbook")
                .fetch_one(pool)
                .await?
        };

        // 데이터 조회
        let rows = if let Some(s) = status {
            if let Some(q) = search {
                let pattern = format!("%{}%", q);
                sqlx::query_as::<_, TextbookOrderRow>(
                    r#"
                    SELECT order_id, order_code, status,
                           orderer_name, orderer_email, orderer_phone,
                           org_name, org_type,
                           delivery_postal_code, delivery_address, delivery_detail,
                           payment_method, depositor_name,
                           tax_invoice, tax_biz_number, tax_email,
                           total_quantity, total_amount, currency, notes,
                           confirmed_at, paid_at, shipped_at, delivered_at, canceled_at,
                           created_at, updated_at
                    FROM textbook
                    WHERE status = $1 AND (orderer_name ILIKE $2 OR order_code ILIKE $2 OR org_name ILIKE $2)
                    ORDER BY created_at DESC
                    LIMIT $3 OFFSET $4
                    "#,
                )
                .bind(s)
                .bind(&pattern)
                .bind(per_page)
                .bind(offset)
                .fetch_all(pool)
                .await?
            } else {
                sqlx::query_as::<_, TextbookOrderRow>(
                    r#"
                    SELECT order_id, order_code, status,
                           orderer_name, orderer_email, orderer_phone,
                           org_name, org_type,
                           delivery_postal_code, delivery_address, delivery_detail,
                           payment_method, depositor_name,
                           tax_invoice, tax_biz_number, tax_email,
                           total_quantity, total_amount, currency, notes,
                           confirmed_at, paid_at, shipped_at, delivered_at, canceled_at,
                           created_at, updated_at
                    FROM textbook
                    WHERE status = $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                )
                .bind(s)
                .bind(per_page)
                .bind(offset)
                .fetch_all(pool)
                .await?
            }
        } else if let Some(q) = search {
            let pattern = format!("%{}%", q);
            sqlx::query_as::<_, TextbookOrderRow>(
                r#"
                SELECT order_id, order_code, status,
                       orderer_name, orderer_email, orderer_phone,
                       org_name, org_type,
                       delivery_postal_code, delivery_address, delivery_detail,
                       payment_method, depositor_name,
                       tax_invoice, tax_biz_number, tax_email,
                       total_quantity, total_amount, currency, notes,
                       confirmed_at, paid_at, shipped_at, delivered_at, canceled_at,
                       created_at, updated_at
                FROM textbook
                WHERE orderer_name ILIKE $1 OR order_code ILIKE $1 OR org_name ILIKE $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(&pattern)
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, TextbookOrderRow>(
                r#"
                SELECT order_id, order_code, status,
                       orderer_name, orderer_email, orderer_phone,
                       org_name, org_type,
                       delivery_postal_code, delivery_address, delivery_detail,
                       payment_method, depositor_name,
                       tax_invoice, tax_biz_number, tax_email,
                       total_quantity, total_amount, currency, notes,
                       confirmed_at, paid_at, shipped_at, delivered_at, canceled_at,
                       created_at, updated_at
                FROM textbook
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };

        Ok((rows, total))
    }

    // =========================================================================
    // Admin: 상태 변경
    // =========================================================================

    /// 주문 상태 업데이트
    pub async fn update_status(
        pool: &PgPool,
        order_id: i64,
        new_status: TextbookOrderStatus,
    ) -> AppResult<()> {
        let now = Utc::now();

        // 상태별 시각 업데이트
        let (col, val): (&str, Option<DateTime<Utc>>) = match new_status {
            TextbookOrderStatus::Confirmed => ("confirmed_at", Some(now)),
            TextbookOrderStatus::Paid => ("paid_at", Some(now)),
            TextbookOrderStatus::Shipped => ("shipped_at", Some(now)),
            TextbookOrderStatus::Delivered => ("delivered_at", Some(now)),
            TextbookOrderStatus::Canceled => ("canceled_at", Some(now)),
            _ => ("updated_at", None),
        };

        // 동적 컬럼 업데이트를 위한 쿼리
        let query = format!(
            "UPDATE textbook SET status = $1, {} = COALESCE($3, {}), updated_at = NOW() WHERE order_id = $2",
            col, col
        );

        sqlx::query(&query)
            .bind(new_status)
            .bind(order_id)
            .bind(val)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// 주문 삭제
    pub async fn delete_order(pool: &PgPool, order_id: i64) -> AppResult<()> {
        sqlx::query("DELETE FROM textbook WHERE order_id = $1")
            .bind(order_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    // =========================================================================
    // Admin: 로그
    // =========================================================================

    /// 관리자 작업 로그 기록
    pub async fn insert_admin_log(
        pool: &PgPool,
        admin_user_id: i64,
        order_id: i64,
        action: crate::types::AdminAction,
        before_data: Option<serde_json::Value>,
        after_data: Option<serde_json::Value>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO admin_textbook_log (admin_user_id, order_id, action, before_data, after_data)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(admin_user_id)
        .bind(order_id)
        .bind(action)
        .bind(before_data)
        .bind(after_data)
        .execute(pool)
        .await?;

        Ok(())
    }
}
