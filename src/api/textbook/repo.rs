use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::error::AppResult;
use crate::types::{
    AdminAction, TextbookLanguage, TextbookOrderStatus, TextbookPaymentMethod, TextbookType,
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
    // 2026-04-23: 관리자 대리 주문 UX 개선으로 nullable 전환 (오프라인·전화 주문 대응)
    pub orderer_email: Option<String>,
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
    /// 할인 전 총액 (수량 × 단가, VAT 포함). 20260423 마이그레이션 신규.
    pub gross_amount: i32,
    /// 할인 금액 (VAT 포함). 0 이면 할인 미적용.
    pub discount_amount: i32,
    /// 할인 사유 (관리자 메모, 선택).
    pub discount_reason: Option<String>,
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

/// Q6 감사 로그 조회 Row (admin_textbook_log + users + textbook JOIN 결과).
/// admin_email_enc 는 users.user_email 원본 (암호화 문자열) — 서비스 레이어에서
/// crypto.decrypt 후 응답 DTO 로 변환.
#[derive(Debug, sqlx::FromRow)]
pub struct AdminLogRow {
    pub log_id: i64,
    pub admin_user_id: i64,
    pub admin_email_enc: String,
    pub admin_nickname: String,
    pub order_id: i64,
    pub order_code: String,
    pub action: AdminAction,
    pub before_data: Option<serde_json::Value>,
    pub after_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// SELECT 공통 컬럼 리스트
const ORDER_COLUMNS: &str = r#"
    order_id, order_code, status,
    orderer_name, orderer_email, orderer_phone,
    org_name, org_type,
    delivery_postal_code, delivery_address, delivery_detail,
    payment_method, depositor_name,
    tax_invoice, tax_biz_number, tax_company_name, tax_rep_name, tax_address, tax_biz_type, tax_biz_item, tax_email,
    total_quantity, total_amount, gross_amount, discount_amount, discount_reason,
    currency, notes,
    tracking_number, tracking_provider, is_deleted,
    confirmed_at, paid_at, shipped_at, delivered_at, canceled_at, deleted_at,
    created_at, updated_at
"#;

/// 교재 주문 생성 파라미터
///
/// `user_id`: 일반 사용자 주문은 `Some(claims.sub)`, 관리자 대리 주문은
///   `None` 또는 귀속시킬 사용자 id. DB 컬럼은 nullable.
pub struct InsertOrderParams<'a> {
    pub order_code: &'a str,
    pub user_id: Option<i64>,
    pub orderer_name: &'a str,
    pub orderer_email: Option<&'a str>,
    pub orderer_phone: &'a str,
    pub org_name: Option<&'a str>,
    pub org_type: Option<&'a str>,
    pub delivery_postal_code: Option<&'a str>,
    pub delivery_address: &'a str,
    pub delivery_detail: Option<&'a str>,
    pub payment_method: TextbookPaymentMethod,
    pub depositor_name: Option<&'a str>,
    pub tax_invoice: bool,
    pub tax_biz_number: Option<&'a str>,
    pub tax_company_name: Option<&'a str>,
    pub tax_rep_name: Option<&'a str>,
    pub tax_address: Option<&'a str>,
    pub tax_biz_type: Option<&'a str>,
    pub tax_biz_item: Option<&'a str>,
    pub tax_email: Option<&'a str>,
    pub total_quantity: i32,
    pub total_amount: i32,
    /// 할인 전 총액 (수량 × 단가). 할인 없을 때는 total_amount 와 동일.
    pub gross_amount: i32,
    /// 할인 금액. 0 이면 할인 미적용. DB CHECK 로 0 ≤ discount ≤ gross.
    pub discount_amount: i32,
    /// 할인 사유 (선택, 관리자 대리 주문 생성 시).
    pub discount_reason: Option<&'a str>,
    pub notes: Option<&'a str>,
}

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
        params: &InsertOrderParams<'_>,
    ) -> AppResult<i64> {
        let order_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO textbook (
                order_code, user_id, orderer_name, orderer_email, orderer_phone,
                org_name, org_type,
                delivery_postal_code, delivery_address, delivery_detail,
                payment_method, depositor_name,
                tax_invoice, tax_biz_number, tax_company_name, tax_rep_name, tax_address, tax_biz_type, tax_biz_item, tax_email,
                total_quantity, total_amount, gross_amount, discount_amount, discount_reason, notes
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26)
            RETURNING order_id
            "#,
        )
        .bind(params.order_code)
        .bind(params.user_id)
        .bind(params.orderer_name)
        .bind(params.orderer_email)
        .bind(params.orderer_phone)
        .bind(params.org_name)
        .bind(params.org_type)
        .bind(params.delivery_postal_code)
        .bind(params.delivery_address)
        .bind(params.delivery_detail)
        .bind(params.payment_method)
        .bind(params.depositor_name)
        .bind(params.tax_invoice)
        .bind(params.tax_biz_number)
        .bind(params.tax_company_name)
        .bind(params.tax_rep_name)
        .bind(params.tax_address)
        .bind(params.tax_biz_type)
        .bind(params.tax_biz_item)
        .bind(params.tax_email)
        .bind(params.total_quantity)
        .bind(params.total_amount)
        .bind(params.gross_amount)
        .bind(params.discount_amount)
        .bind(params.discount_reason)
        .bind(params.notes)
        .fetch_one(&mut **tx)
        .await?;

        Ok(order_id)
    }

    /// 주문 할인 업데이트 (관리자 편집).
    ///
    /// gross_amount 는 불변이므로 discount_amount + total_amount 만 갱신.
    /// DB CHECK 제약으로 0 ≤ discount ≤ gross 및 total = gross - discount
    /// 이 자동 검증됨 (service 레이어에서도 선검증).
    pub async fn update_discount(
        pool: &PgPool,
        order_id: i64,
        discount_amount: i32,
        discount_reason: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE textbook
            SET discount_amount = $2,
                discount_reason = $3,
                total_amount = gross_amount - $2,
                updated_at = NOW()
            WHERE order_id = $1 AND is_deleted = false
            "#,
        )
        .bind(order_id)
        .bind(discount_amount)
        .bind(discount_reason)
        .execute(pool)
        .await?;

        Ok(())
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

    /// 상태별 타임스탬프 컬럼 결정
    fn status_timestamp_col(status: TextbookOrderStatus) -> Option<&'static str> {
        match status {
            TextbookOrderStatus::Confirmed => Some("confirmed_at"),
            TextbookOrderStatus::Paid => Some("paid_at"),
            TextbookOrderStatus::Shipped => Some("shipped_at"),
            TextbookOrderStatus::Delivered => Some("delivered_at"),
            TextbookOrderStatus::Canceled => Some("canceled_at"),
            _ => None, // Pending, Printing — 별도 타임스탬프 없음
        }
    }

    /// 주문 상태 업데이트 (Pool 기반, 상태 전환 API 용).
    /// 2026-04-23: 상태 전환 자유화 + timestamp set-if-null (`COALESCE`). 역행 전환
    /// 시에도 기존 첫 전환 시점 보존. 최근 변경 시각은 `updated_at` 으로 추적.
    pub async fn update_status(
        pool: &PgPool,
        order_id: i64,
        new_status: TextbookOrderStatus,
    ) -> AppResult<()> {
        let now = Utc::now();

        if let Some(col) = Self::status_timestamp_col(new_status) {
            let query = format!(
                "UPDATE textbook SET status = $1, {col} = COALESCE({col}, $3), updated_at = NOW() WHERE order_id = $2 AND is_deleted = false",
                col = col
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

    /// 주문 상태 업데이트 (트랜잭션 내 사용 — 원자성 필요 시).
    /// 관리자 대리 주문 생성 시 insert + 초기 상태 세팅이 원자적으로
    /// 이루어지도록 동일 트랜잭션 내에서 호출. timestamp 는 COALESCE set-if-null.
    pub async fn update_status_in_tx(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        order_id: i64,
        new_status: TextbookOrderStatus,
    ) -> AppResult<()> {
        let now = Utc::now();

        if let Some(col) = Self::status_timestamp_col(new_status) {
            let query = format!(
                "UPDATE textbook SET status = $1, {col} = COALESCE({col}, $3), updated_at = NOW() WHERE order_id = $2 AND is_deleted = false",
                col = col
            );
            sqlx::query(&query)
                .bind(new_status)
                .bind(order_id)
                .bind(now)
                .execute(&mut **tx)
                .await?;
        } else {
            sqlx::query(
                "UPDATE textbook SET status = $1, updated_at = NOW() WHERE order_id = $2 AND is_deleted = false",
            )
            .bind(new_status)
            .bind(order_id)
            .execute(&mut **tx)
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

    /// 관리자 감사 로그 조회 (Q6, 2026-04-22) — admin/textbook/logs 지원.
    ///
    /// action/order_id/admin_user_id 필터 + 페이지네이션. users/textbook 과 JOIN
    /// 해서 관리자 식별 정보 (암호화된 email 포함) + 주문번호 반환.
    /// email 은 서비스 레이어에서 복호화.
    pub async fn list_admin_logs(
        pool: &PgPool,
        action: Option<crate::types::AdminAction>,
        order_id: Option<i64>,
        admin_user_id: Option<i64>,
        page: i64,
        per_page: i64,
    ) -> AppResult<(i64, Vec<AdminLogRow>)> {
        let offset = (page - 1) * per_page;

        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM admin_textbook_log l
            WHERE ($1::admin_action_enum IS NULL OR l.action = $1)
              AND ($2::bigint IS NULL OR l.order_id = $2)
              AND ($3::bigint IS NULL OR l.admin_user_id = $3)
            "#,
        )
        .bind(action)
        .bind(order_id)
        .bind(admin_user_id)
        .fetch_one(pool)
        .await?;

        let rows = sqlx::query_as::<_, AdminLogRow>(
            r#"
            SELECT
                l.log_id,
                l.admin_user_id,
                u.user_email      AS admin_email_enc,
                u.user_nickname   AS admin_nickname,
                l.order_id,
                t.order_code,
                l.action AS "action!: crate::types::AdminAction",
                l.before_data,
                l.after_data,
                l.created_at
            FROM admin_textbook_log l
            INNER JOIN users u     ON u.user_id  = l.admin_user_id
            INNER JOIN textbook t  ON t.order_id = l.order_id
            WHERE ($1::admin_action_enum IS NULL OR l.action = $1)
              AND ($2::bigint IS NULL OR l.order_id = $2)
              AND ($3::bigint IS NULL OR l.admin_user_id = $3)
            ORDER BY l.created_at DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(action)
        .bind(order_id)
        .bind(admin_user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((total, rows))
    }

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
