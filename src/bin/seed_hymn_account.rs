//! HYMN 시스템 계정 시드 바이너리 (귀속 전용 — 로그인 불가)
//!
//! 콘텐츠 시딩(study/study_task 등)의 `updated_by_user_id` 가 가리킬
//! `user_auth='HYMN'` 시스템 계정을 생성한다. books 시드 SQL 은 이 계정이
//! 없으면 RAISE EXCEPTION 으로 abort 하므로 콘텐츠 시딩의 선행 조건.
//!
//! raw SQL 로 못 만드는 이유: `user_email`/`user_name`/`user_birthday` 는
//! 앱 KeyRing(AES-256-GCM) 암호화 + HMAC blind index 컬럼이라, prod 앱이
//! 쓰는 키로 암호화해야만 한다 → 앱 crypto 를 쓰는 코드 경로 필수.
//!
//! 성격: 귀속 전용. `user_state=false`(로그인 차단) + `user_password=NULL`.
//! FK 대상이라 state/비번 무관하게 동작.
//!
//! 멱등: `user_auth='HYMN'` 존재 시 skip. `user_email_idx` UNIQUE 이중 안전.
//!
//! 사용법:
//!   cargo run --bin seed_hymn_account
//!   docker exec amk-api /app/seed_hymn_account   (prod 1회)

use amazing_korean_api::config::Config;
use amazing_korean_api::crypto::CryptoService;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::time::Duration;

// 시스템 계정 신원 (귀속 전용 표식 — 소유 도메인·비개인·실가입 미충돌)
const HYMN_EMAIL: &str = "system@amazingkorean.net";
const HYMN_NAME: &str = "HYMN";
const HYMN_NICKNAME: &str = "HYMN";
const HYMN_COUNTRY: &str = "KR";
const HYMN_BIRTHDAY: &str = "1900-01-01"; // placeholder (암호화 컬럼, 미사용)

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let cfg = Config::from_env();
    let crypto = CryptoService::new(&cfg.encryption_ring, &cfg.hmac_key);

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect(&cfg.database_url)
        .await?;

    // 멱등: HYMN 이미 있으면 skip
    let existing: Option<i64> = sqlx::query("SELECT user_id FROM users WHERE user_auth = 'HYMN'")
        .fetch_optional(&pool)
        .await?
        .map(|r| r.get::<i64, _>("user_id"));
    if let Some(uid) = existing {
        println!("HYMN 계정 이미 존재 (user_id={uid}) — skip (멱등)");
        return Ok(());
    }

    // 앱 crypto 로 PII 암호화 + blind index (AAD = 앱 read 경로와 정확히 일치)
    let email_enc = crypto
        .encrypt(HYMN_EMAIL, "users.user_email")
        .map_err(|e| anyhow::anyhow!("encrypt email: {e}"))?;
    let email_idx = crypto
        .blind_index(HYMN_EMAIL)
        .map_err(|e| anyhow::anyhow!("blind_index email: {e}"))?;
    let name_enc = crypto
        .encrypt(HYMN_NAME, "users.user_name")
        .map_err(|e| anyhow::anyhow!("encrypt name: {e}"))?;
    let name_idx = crypto
        .blind_index(HYMN_NAME)
        .map_err(|e| anyhow::anyhow!("blind_index name: {e}"))?;
    let birthday_enc = crypto
        .encrypt(HYMN_BIRTHDAY, "users.user_birthday")
        .map_err(|e| anyhow::anyhow!("encrypt birthday: {e}"))?;

    // 단일 INSERT. user_password=NULL(로그인 불가), user_state=false.
    // 나머지 NOT NULL 컬럼은 스키마 기본값(language=ko/gender=none/
    // check_email=false/terms=false/created_at=now()).
    let row = sqlx::query(
        r#"
        INSERT INTO users (
            user_auth, user_state,
            user_email, user_email_idx,
            user_name, user_name_idx,
            user_nickname, user_country, user_birthday
        )
        VALUES ('HYMN', false, $1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (user_email_idx) DO NOTHING
        RETURNING user_id
        "#,
    )
    .bind(&email_enc)
    .bind(&email_idx)
    .bind(&name_enc)
    .bind(&name_idx)
    .bind(HYMN_NICKNAME)
    .bind(HYMN_COUNTRY)
    .bind(&birthday_enc)
    .fetch_optional(&pool)
    .await?;

    match row {
        Some(r) => {
            let uid: i64 = r.get("user_id");
            println!("HYMN 계정 생성 완료 (user_id={uid}, state=false, password=NULL)");
        }
        None => {
            // user_email_idx 충돌 = 동일 이메일의 다른 계정 선재 (HYMN 아님).
            println!(
                "경고: user_email_idx 충돌로 미생성 — '{HYMN_EMAIL}' 가 비-HYMN 계정으로 선재. 확인 필요."
            );
        }
    }

    Ok(())
}
