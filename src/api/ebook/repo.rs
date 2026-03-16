use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::error::AppResult;
use crate::types::{
    AdminAction, EbookEdition, EbookPaymentMethod, EbookPurchaseStatus, TextbookLanguage,
};

// ─────────────────────── Row Types ───────────────────────

#[derive(Debug)]
pub struct EbookPurchaseRow {
    pub purchase_id: i64,
    pub purchase_code: String,
    pub user_id: i64,
    pub language: TextbookLanguage,
    pub edition: EbookEdition,
    pub payment_method: EbookPaymentMethod,
    pub status: EbookPurchaseStatus,
    pub price: i32,
    pub currency: String,
    pub paddle_txn_id: Option<String>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub refunded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const PURCHASE_COLUMNS: &str = r#"
    purchase_id, purchase_code, user_id, language, edition,
    payment_method, status, price, currency, paddle_txn_id,
    is_deleted, deleted_at, completed_at, refunded_at, created_at, updated_at
"#;

fn map_purchase_row(row: &sqlx::postgres::PgRow) -> EbookPurchaseRow {
    use sqlx::Row;
    EbookPurchaseRow {
        purchase_id: row.get("purchase_id"),
        purchase_code: row.get("purchase_code"),
        user_id: row.get("user_id"),
        language: row.get("language"),
        edition: row.get("edition"),
        payment_method: row.get("payment_method"),
        status: row.get("status"),
        price: row.get("price"),
        currency: row.get("currency"),
        paddle_txn_id: row.get("paddle_txn_id"),
        is_deleted: row.get("is_deleted"),
        deleted_at: row.get("deleted_at"),
        completed_at: row.get("completed_at"),
        refunded_at: row.get("refunded_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

// ─────────────────────── Purchase Code Generation ───────────────────────

/// {LANG}-{ED}-{YYYYMMDD}-{PAY}-{NNNN} 형식의 주문코드 생성 (advisory lock으로 동시성 안전)
/// 예: VN-ST-20260310-CA-0001
pub async fn generate_purchase_code(
    tx: &mut Transaction<'_, Postgres>,
    language: TextbookLanguage,
    edition: EbookEdition,
    payment_method: EbookPaymentMethod,
) -> AppResult<String> {
    // Advisory lock to prevent race conditions (0x65626f6f6b = "ebook" in hex)
    sqlx::query("SELECT pg_advisory_xact_lock(1701011563)")
        .execute(&mut **tx)
        .await?;

    let today = Utc::now().format("%Y%m%d").to_string();
    let lang_code = language.to_purchase_code();
    let ed_code = edition.to_purchase_code();
    let pay_code = payment_method.to_purchase_code();
    let prefix = format!("{lang_code}-{ed_code}-{today}-{pay_code}-");

    // MAX 기반 순번 추출 (하드 삭제 시에도 중복 방지)
    let row = sqlx::query(
        "SELECT COALESCE(MAX(RIGHT(purchase_code, 4)::INTEGER), 0) as max_seq
         FROM ebook_purchase WHERE purchase_code LIKE $1 || '%'",
    )
    .bind(&prefix)
    .fetch_one(&mut **tx)
    .await?;

    let max_seq: i32 = row.get("max_seq");
    let code = format!("{lang_code}-{ed_code}-{today}-{pay_code}-{:04}", max_seq + 1);

    Ok(code)
}

// ─────────────────────── Insert ───────────────────────

pub async fn insert_purchase(
    tx: &mut Transaction<'_, Postgres>,
    purchase_code: &str,
    user_id: i64,
    language: TextbookLanguage,
    edition: EbookEdition,
    payment_method: EbookPaymentMethod,
    price: i32,
    currency: &str,
) -> AppResult<EbookPurchaseRow> {
    let sql = format!(
        "INSERT INTO ebook_purchase (
            purchase_code, user_id, language, edition, payment_method,
            price, currency
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING {PURCHASE_COLUMNS}"
    );

    let row = sqlx::query(&sql)
        .bind(purchase_code)
        .bind(user_id)
        .bind(language)
        .bind(edition)
        .bind(payment_method)
        .bind(price)
        .bind(currency)
        .fetch_one(&mut **tx)
        .await?;

    Ok(map_purchase_row(&row))
}

// ─────────────────────── Find ───────────────────────

pub async fn find_by_code(db: &PgPool, code: &str) -> AppResult<Option<EbookPurchaseRow>> {
    let sql = format!(
        "SELECT {PURCHASE_COLUMNS} FROM ebook_purchase
         WHERE purchase_code = $1 AND is_deleted = false"
    );

    let row = sqlx::query(&sql).bind(code).fetch_optional(db).await?;

    Ok(row.as_ref().map(map_purchase_row))
}

pub async fn find_by_id(db: &PgPool, purchase_id: i64) -> AppResult<Option<EbookPurchaseRow>> {
    let sql = format!(
        "SELECT {PURCHASE_COLUMNS} FROM ebook_purchase
         WHERE purchase_id = $1 AND is_deleted = false"
    );

    let row = sqlx::query(&sql)
        .bind(purchase_id)
        .fetch_optional(db)
        .await?;

    Ok(row.as_ref().map(map_purchase_row))
}

pub async fn find_by_user(db: &PgPool, user_id: i64) -> AppResult<Vec<EbookPurchaseRow>> {
    let sql = format!(
        "SELECT {PURCHASE_COLUMNS} FROM ebook_purchase
         WHERE user_id = $1 AND is_deleted = false
         ORDER BY created_at DESC"
    );

    let rows = sqlx::query(&sql).bind(user_id).fetch_all(db).await?;

    Ok(rows.iter().map(map_purchase_row).collect())
}

/// 동일 사용자가 같은 언어+에디션을 이미 구매했는지 확인 (pending 또는 completed)
pub async fn find_existing_purchase(
    db: &PgPool,
    user_id: i64,
    language: TextbookLanguage,
    edition: EbookEdition,
) -> AppResult<Option<EbookPurchaseRow>> {
    let sql = format!(
        "SELECT {PURCHASE_COLUMNS} FROM ebook_purchase
         WHERE user_id = $1 AND language = $2 AND edition = $3
           AND status IN ('pending', 'completed') AND is_deleted = false"
    );

    let row = sqlx::query(&sql)
        .bind(user_id)
        .bind(language)
        .bind(edition)
        .fetch_optional(db)
        .await?;

    Ok(row.as_ref().map(map_purchase_row))
}

// ─────────────────────── Update ───────────────────────

pub async fn update_status(
    db: &PgPool,
    purchase_id: i64,
    new_status: EbookPurchaseStatus,
) -> AppResult<()> {
    let timestamp_col = match new_status {
        EbookPurchaseStatus::Completed => "completed_at",
        EbookPurchaseStatus::Refunded => "refunded_at",
        EbookPurchaseStatus::Pending => "",
    };

    let sql = if timestamp_col.is_empty() {
        "UPDATE ebook_purchase SET status = $1, updated_at = NOW() WHERE purchase_id = $2".to_string()
    } else {
        format!(
            "UPDATE ebook_purchase SET status = $1, {timestamp_col} = NOW(), updated_at = NOW()
             WHERE purchase_id = $2"
        )
    };

    sqlx::query(&sql)
        .bind(new_status)
        .bind(purchase_id)
        .execute(db)
        .await?;

    Ok(())
}

/// Paddle 결제 완료 시: 상태 → completed + paddle_txn_id 저장
pub async fn complete_with_paddle_txn(
    db: &PgPool,
    purchase_code: &str,
    paddle_txn_id: &str,
) -> AppResult<Option<EbookPurchaseRow>> {
    let sql = format!(
        "UPDATE ebook_purchase
         SET status = 'completed', paddle_txn_id = $1, completed_at = NOW(), updated_at = NOW()
         WHERE purchase_code = $2 AND status = 'pending' AND is_deleted = false
         RETURNING {PURCHASE_COLUMNS}"
    );

    let row = sqlx::query(&sql)
        .bind(paddle_txn_id)
        .bind(purchase_code)
        .fetch_optional(db)
        .await?;

    Ok(row.as_ref().map(map_purchase_row))
}

// ─────────────────────── Access Log ───────────────────────

pub async fn insert_access_log(
    db: &PgPool,
    purchase_id: i64,
    user_id: i64,
    page_number: i32,
    watermark_id: &str,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO ebook_access_log (
            purchase_id, user_id, page_number, watermark_id, ip_address, user_agent
        ) VALUES ($1, $2, $3, $4, $5::inet, $6)",
    )
    .bind(purchase_id)
    .bind(user_id)
    .bind(page_number)
    .bind(watermark_id)
    .bind(ip_address)
    .bind(user_agent)
    .execute(db)
    .await?;

    Ok(())
}

// ─────────────────────── Admin: List ───────────────────────

pub async fn list_purchases(
    db: &PgPool,
    page: i64,
    per_page: i64,
    status: Option<EbookPurchaseStatus>,
    search: Option<&str>,
) -> AppResult<(Vec<EbookPurchaseRow>, i64)> {
    let mut where_clauses = vec!["is_deleted = false".to_string()];
    let mut bind_idx = 1;

    if status.is_some() {
        where_clauses.push(format!("status = ${bind_idx}"));
        bind_idx += 1;
    }

    if search.is_some() {
        where_clauses.push(format!("purchase_code ILIKE ${bind_idx}"));
        #[allow(unused_assignments)]
        { bind_idx += 1; }
    }

    let where_sql = where_clauses.join(" AND ");
    let offset = (page - 1) * per_page;

    // Count query
    let count_sql = format!("SELECT COUNT(*) as cnt FROM ebook_purchase WHERE {where_sql}");
    let mut count_q = sqlx::query(&count_sql);

    if let Some(s) = status {
        count_q = count_q.bind(s);
    }
    if let Some(s) = search {
        count_q = count_q.bind(format!("%{}%", escape_like(s)));
    }

    let total: i64 = count_q.fetch_one(db).await?.get("cnt");

    // Data query
    let data_sql = format!(
        "SELECT {PURCHASE_COLUMNS} FROM ebook_purchase
         WHERE {where_sql}
         ORDER BY created_at DESC
         LIMIT {per_page} OFFSET {offset}"
    );

    let mut data_q = sqlx::query(&data_sql);

    if let Some(s) = status {
        data_q = data_q.bind(s);
    }
    if let Some(s) = search {
        data_q = data_q.bind(format!("%{}%", escape_like(s)));
    }

    let rows = data_q.fetch_all(db).await?;
    let purchases = rows.iter().map(map_purchase_row).collect();

    Ok((purchases, total))
}

// ─────────────────────── Admin: Log ───────────────────────

pub async fn insert_admin_log(
    db: &PgPool,
    admin_user_id: i64,
    purchase_id: i64,
    action: AdminAction,
    before_data: Option<serde_json::Value>,
    after_data: Option<serde_json::Value>,
) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO admin_ebook_log (admin_user_id, purchase_id, action, before_data, after_data)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(admin_user_id)
    .bind(purchase_id)
    .bind(action)
    .bind(before_data)
    .bind(after_data)
    .execute(db)
    .await?;

    Ok(())
}

// ─────────────────────── Soft Delete ───────────────────────

pub async fn soft_delete_purchase(db: &PgPool, purchase_id: i64) -> AppResult<()> {
    sqlx::query(
        "UPDATE ebook_purchase SET is_deleted = true, deleted_at = NOW(), updated_at = NOW()
         WHERE purchase_id = $1",
    )
    .bind(purchase_id)
    .execute(db)
    .await?;

    Ok(())
}

// ─────────────────────── Helpers ───────────────────────

fn escape_like(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}
