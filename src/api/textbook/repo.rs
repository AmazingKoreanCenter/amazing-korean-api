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
    pub tax_company_name: Option<String>,
    pub tax_rep_name: Option<String>,
    pub tax_address: Option<String>,
    pub tax_biz_type: Option<String>,
    pub tax_biz_item: Option<String>,
    pub tax_email: Option<String>,
    pub total_quantity: i32,
    pub total_amount: i32,
    pub currency: String,
    pub notes: Option<String>,
    pub tracking_number: Option<String>,
    pub tracking_provider: Option<String>,
    pub is_deleted: bool,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
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

/// SELECT 공통 컬럼 리스트
const ORDER_COLUMNS: &str = r#"
    order_id, order_code, status,
    orderer_name, orderer_email, orderer_phone,
    org_name, org_type,
    delivery_postal_code, delivery_address, delivery_detail,
    payment_method, depositor_name,
    tax_invoice, tax_biz_number, tax_company_name, tax_rep_name, tax_address, tax_biz_type, tax_biz_item, tax_email,
    total_quantity, total_amount, currency, notes,
    tracking_number, tracking_provider, is_deleted,
    confirmed_at, paid_at, shipped_at, delivered_at, canceled_at, deleted_at,
    created_at, updated_at
"#;

pub struct TextbookRepo;

impl TextbookRepo {
    // =========================================================================
    // Order Code Generation
    // =========================================================================

    /// TB-YYMMDD-NNNN 형식 주문번호 생성 (트랜잭션 내에서 호출하여 race condition 방지)
    pub async fn generate_order_code(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> AppResult<String> {
        let today = Utc::now().format("%y%m%d").to_string();
        let prefix = format!("TB-{}-", today);

        // Advisory lock으로 동시 생성 방지 (blocking — 트랜잭션 종료 시 자동 해제)
        sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(0x7465787462_i64) // 'textb' hash
            .execute(&mut **tx)
            .await?;

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM textbook WHERE order_code LIKE $1 || '%'",
        )
        .bind(&prefix)
        .fetch_one(&mut **tx)
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
        user_id: i64,
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
        tax_company_name: Option<&str>,
        tax_rep_name: Option<&str>,
        tax_address: Option<&str>,
        tax_biz_type: Option<&str>,
        tax_biz_item: Option<&str>,
        tax_email: Option<&str>,
        total_quantity: i32,
        total_amount: i32,
        notes: Option<&str>,
    ) -> AppResult<i64> {
        let order_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO textbook (
                order_code, user_id, orderer_name, orderer_email, orderer_phone,
                org_name, org_type,
                delivery_postal_code, delivery_address, delivery_detail,
                payment_method, depositor_name,
                tax_invoice, tax_biz_number, tax_company_name, tax_rep_name, tax_address, tax_biz_type, tax_biz_item, tax_email,
                total_quantity, total_amount, notes
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            RETURNING order_id
            "#,
        )
        .bind(order_code)
        .bind(user_id)
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
        .bind(tax_company_name)
        .bind(tax_rep_name)
        .bind(tax_address)
        .bind(tax_biz_type)
        .bind(tax_biz_item)
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
        let sql = format!(
            "SELECT {} FROM textbook WHERE order_code = $1 AND is_deleted = false",
            ORDER_COLUMNS,
        );
        let row = sqlx::query_as::<_, TextbookOrderRow>(&sql)
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
        let sql = format!(
            "SELECT {} FROM textbook WHERE order_id = $1 AND is_deleted = false",
            ORDER_COLUMNS,
        );
        let row = sqlx::query_as::<_, TextbookOrderRow>(&sql)
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

    /// 여러 주문의 항목을 한 번에 조회 (N+1 방지)
    pub async fn find_items_by_orders(
        pool: &PgPool,
        order_ids: &[i64],
    ) -> AppResult<Vec<TextbookItemRow>> {
        if order_ids.is_empty() {
            return Ok(vec![]);
        }
        let rows = sqlx::query_as::<_, TextbookItemRow>(
            r#"
            SELECT item_id, order_id, textbook_language, textbook_type,
                   quantity, unit_price, subtotal
            FROM textbook_item
            WHERE order_id = ANY($1)
            ORDER BY order_id, item_id
            "#,
        )
        .bind(order_ids)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    /// 사용자의 주문 목록 조회 (내 주문)
    pub async fn find_by_user_id(
        pool: &PgPool,
        user_id: i64,
    ) -> AppResult<Vec<TextbookOrderRow>> {
        let sql = format!(
            "SELECT {} FROM textbook WHERE user_id = $1 AND is_deleted = false ORDER BY created_at DESC",
            ORDER_COLUMNS,
        );
        let rows = sqlx::query_as::<_, TextbookOrderRow>(&sql)
            .bind(user_id)
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }

    // =========================================================================
    // Admin: 목록 조회
    // =========================================================================

    /// 주문 목록 조회 (페이지네이션 + 필터, soft delete 제외)
    pub async fn list_orders(
        pool: &PgPool,
        status: Option<TextbookOrderStatus>,
        search: Option<&str>,
        page: i64,
        per_page: i64,
    ) -> AppResult<(Vec<TextbookOrderRow>, i64)> {
        // 페이지네이션 범위 제한
        let page = page.max(1);
        let per_page = per_page.clamp(1, 100);
        let offset = (page - 1) * per_page;

        // WHERE 절 동적 구성
        let mut conditions = vec!["is_deleted = false".to_string()];
        let mut bind_idx = 1_usize;

        if status.is_some() {
            conditions.push(format!("status = ${}", bind_idx));
            bind_idx += 1;
        }
        if search.is_some() {
            conditions.push(format!(
                "(orderer_name ILIKE ${bi} OR order_code ILIKE ${bi} OR org_name ILIKE ${bi})",
                bi = bind_idx,
            ));
            bind_idx += 1;
        }

        let where_clause = conditions.join(" AND ");

        // 총 개수
        let count_sql = format!("SELECT COUNT(*) FROM textbook WHERE {}", where_clause);
        let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);
        if let Some(s) = status {
            count_query = count_query.bind(s);
        }
        if let Some(q) = search {
            count_query = count_query.bind(format!("%{}%", escape_like(q)));
        }
        let total = count_query.fetch_one(pool).await?;

        // 데이터 조회
        let data_sql = format!(
            "SELECT {} FROM textbook WHERE {} ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
            ORDER_COLUMNS, where_clause, bind_idx, bind_idx + 1,
        );
        let mut data_query = sqlx::query_as::<_, TextbookOrderRow>(&data_sql);
        if let Some(s) = status {
            data_query = data_query.bind(s);
        }
        if let Some(q) = search {
            data_query = data_query.bind(format!("%{}%", escape_like(q)));
        }
        data_query = data_query.bind(per_page).bind(offset);
        let rows = data_query.fetch_all(pool).await?;

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

        // 상태별 타임스탬프 컬럼 결정
        let timestamp_col: Option<&str> = match new_status {
            TextbookOrderStatus::Confirmed => Some("confirmed_at"),
            TextbookOrderStatus::Paid => Some("paid_at"),
            TextbookOrderStatus::Shipped => Some("shipped_at"),
            TextbookOrderStatus::Delivered => Some("delivered_at"),
            TextbookOrderStatus::Canceled => Some("canceled_at"),
            _ => None, // Pending, Printing — 별도 타임스탬프 없음
        };

        if let Some(col) = timestamp_col {
            let query = format!(
                "UPDATE textbook SET status = $1, {} = $3, updated_at = NOW() WHERE order_id = $2 AND is_deleted = false",
                col
            );
            sqlx::query(&query)
                .bind(new_status)
                .bind(order_id)
                .bind(now)
                .execute(pool)
                .await?;
        } else {
            sqlx::query(
                "UPDATE textbook SET status = $1, updated_at = NOW() WHERE order_id = $2 AND is_deleted = false",
            )
            .bind(new_status)
            .bind(order_id)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    /// 배송 추적 정보 업데이트
    pub async fn update_tracking(
        pool: &PgPool,
        order_id: i64,
        tracking_number: Option<&str>,
        tracking_provider: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            "UPDATE textbook SET tracking_number = $2, tracking_provider = $3, updated_at = NOW() WHERE order_id = $1 AND is_deleted = false",
        )
        .bind(order_id)
        .bind(tracking_number)
        .bind(tracking_provider)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// 주문 Soft Delete
    pub async fn soft_delete_order(pool: &PgPool, order_id: i64) -> AppResult<()> {
        sqlx::query(
            "UPDATE textbook SET is_deleted = true, deleted_at = NOW(), updated_at = NOW() WHERE order_id = $1",
        )
        .bind(order_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // Admin: 로그
    // =========================================================================

    /// 관리자 작업 로그 기록 (ILIKE 특수문자 이스케이프)
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

/// ILIKE 패턴용 특수문자 이스케이프 (`%`, `_`, `\`)
fn escape_like(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}
