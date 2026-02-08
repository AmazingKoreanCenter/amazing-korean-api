//! 암호화 키 로테이션 배치 바이너리
//!
//! 구버전 암호화 데이터를 최신 버전으로 재암호화한다.
//! 키 로테이션 완료 흐름:
//!   V2 키 추가 & 배포 → rekey --check → rekey 실행 → rekey --verify → V1 키 제거 & 배포
//!
//! 사용법:
//!   cargo run --bin rekey_encryption -- --check
//!   cargo run --bin rekey_encryption -- --batch-size 500
//!   cargo run --bin rekey_encryption -- --verify

use std::time::Duration;

use amazing_korean_api::config::Config;
use amazing_korean_api::crypto::{CryptoService, KeyRing};
use amazing_korean_api::crypto::cipher;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

#[derive(Parser)]
#[command(name = "rekey_encryption", about = "Re-encrypt data with the latest key version")]
struct Args {
    /// Check mode: report version distribution per column
    #[arg(long)]
    check: bool,

    /// Verify mode: ensure no old-version data remains
    #[arg(long)]
    verify: bool,

    /// Batch size for re-encryption (default: 500)
    #[arg(long, default_value = "500")]
    batch_size: i64,
}

/// 재암호화 대상 컬럼 정의
struct RekeyTarget {
    table: &'static str,
    pk: &'static str,
    columns: Vec<RekeyColumn>,
}

struct RekeyColumn {
    name: &'static str,
    aad: &'static str,
    /// Strict: NOT NULL 필수 암호문 컬럼 (users.user_email 등)
    /// Loose: NULL 허용 또는 레거시 혼재 가능 컬럼
    strict: bool,
}

fn rekey_targets() -> Vec<RekeyTarget> {
    vec![
        RekeyTarget {
            table: "users",
            pk: "user_id",
            columns: vec![
                RekeyColumn { name: "user_email", aad: "users.user_email", strict: true },
                RekeyColumn { name: "user_name", aad: "users.user_name", strict: true },
                RekeyColumn { name: "user_birthday", aad: "users.user_birthday", strict: true },
            ],
        },
        RekeyTarget {
            table: "user_oauth",
            pk: "oauth_id",
            columns: vec![
                RekeyColumn { name: "oauth_email", aad: "user_oauth.oauth_email", strict: false },
                RekeyColumn { name: "oauth_subject", aad: "user_oauth.oauth_subject", strict: true },
            ],
        },
        RekeyTarget {
            table: "login",
            pk: "login_id",
            columns: vec![
                RekeyColumn { name: "login_ip", aad: "login.login_ip", strict: true },
            ],
        },
        RekeyTarget {
            table: "login_log",
            pk: "login_log_id",
            columns: vec![
                RekeyColumn { name: "login_ip_log", aad: "login_log.login_ip_log", strict: false },
            ],
        },
        // admin_action_log는 Phase 3 완료 후 추가 (현재 INET→TEXT 마이그레이션 전)
    ]
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    dotenvy::dotenv().ok();
    let cfg = Config::from_env();
    let ring = &cfg.encryption_ring;
    let hmac_key = &cfg.hmac_key;

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect(&cfg.database_url)
        .await?;

    if args.check {
        run_check(&pool, ring).await?;
    } else if args.verify {
        run_verify(&pool, ring).await?;
    } else {
        run_rekey(&pool, ring, hmac_key, args.batch_size).await?;
    }

    Ok(())
}

/// --check: 버전별 행 카운트 출력
async fn run_check(pool: &PgPool, ring: &KeyRing) -> anyhow::Result<()> {
    println!("=== Encryption Version Report ===");
    println!("Current version: v{}\n", ring.current_version());

    for target in rekey_targets() {
        for col in &target.columns {
            let query = format!(
                "SELECT {col} FROM {table} WHERE {col} IS NOT NULL",
                col = col.name,
                table = target.table,
            );
            let rows: Vec<(String,)> = sqlx::query_as(&query).fetch_all(pool).await?;

            let mut version_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            let mut non_enc_count = 0usize;

            for (val,) in &rows {
                match cipher::extract_version(val) {
                    Ok(v) => {
                        *version_counts.entry(format!("v{}", v)).or_default() += 1;
                    }
                    Err(_) => {
                        non_enc_count += 1;
                    }
                }
            }

            print!("{}.{}: total={}", target.table, col.name, rows.len());
            for (ver, count) in &version_counts {
                print!(", {}={}", ver, count);
            }
            if non_enc_count > 0 {
                print!(", plaintext/corrupted={}", non_enc_count);
            }
            println!();
        }
    }

    Ok(())
}

/// --verify: 구버전 0건 확인 (2단 검증: strict/loose)
async fn run_verify(pool: &PgPool, ring: &KeyRing) -> anyhow::Result<()> {
    println!("=== Rekey Verification ===");
    println!("Expected version: v{}\n", ring.current_version());

    let current = ring.current_version();
    let mut all_ok = true;

    for target in rekey_targets() {
        for col in &target.columns {
            if col.strict {
                // Strict: 전체 행이 enc prefix + 구버전 0건
                let non_enc_query = format!(
                    "SELECT COUNT(*) as cnt FROM {table} WHERE {col} IS NOT NULL AND {col} NOT LIKE 'enc:v%'",
                    table = target.table, col = col.name,
                );
                let non_enc: (i64,) = sqlx::query_as(&non_enc_query).fetch_one(pool).await?;

                if non_enc.0 > 0 {
                    println!("[FAIL] {}.{} (strict): {} rows without enc prefix (plaintext/corrupted)",
                        target.table, col.name, non_enc.0);
                    all_ok = false;
                }

                let old_ver_query = format!(
                    "SELECT COUNT(*) as cnt FROM {table} WHERE {col} IS NOT NULL AND {col} LIKE 'enc:v%' AND {col} NOT LIKE 'enc:v{current}:%'",
                    table = target.table, col = col.name, current = current,
                );
                let old_ver: (i64,) = sqlx::query_as(&old_ver_query).fetch_one(pool).await?;

                if old_ver.0 > 0 {
                    println!("[FAIL] {}.{} (strict): {} rows with old version (not v{})",
                        target.table, col.name, old_ver.0, current);
                    all_ok = false;
                } else {
                    println!("[OK]   {}.{} (strict): all rows are v{}", target.table, col.name, current);
                }
            } else {
                // Loose: enc prefix인 값만 대상으로 구버전 0건
                let old_ver_query = format!(
                    "SELECT COUNT(*) as cnt FROM {table} WHERE {col} IS NOT NULL AND {col} LIKE 'enc:v%' AND {col} NOT LIKE 'enc:v{current}:%'",
                    table = target.table, col = col.name, current = current,
                );
                let old_ver: (i64,) = sqlx::query_as(&old_ver_query).fetch_one(pool).await?;

                if old_ver.0 > 0 {
                    println!("[FAIL] {}.{} (loose): {} encrypted rows with old version (not v{})",
                        target.table, col.name, old_ver.0, current);
                    all_ok = false;
                } else {
                    println!("[OK]   {}.{} (loose): no old-version encrypted rows", target.table, col.name);
                }
            }
        }
    }

    // 랜덤 5행 샘플 decrypt 테스트
    println!("\n--- Sample Decrypt Test ---");
    let crypto = CryptoService::new(ring, &[0u8; 32]); // hmac_key 불필요 (decrypt만)
    // Note: CryptoService에서 blind_index는 사용하지 않으므로 더미 hmac_key OK
    // 실제로는 Config에서 가져온 hmac_key를 사용하지만, rekey에서는 decrypt만 하므로 무관

    for target in rekey_targets() {
        for col in &target.columns {
            let sample_query = format!(
                "SELECT {pk}, {col} FROM {table} WHERE {col} IS NOT NULL AND {col} LIKE 'enc:v%' ORDER BY RANDOM() LIMIT 5",
                pk = target.pk, col = col.name, table = target.table,
            );
            let rows = sqlx::query(&sample_query).fetch_all(pool).await?;

            for row in &rows {
                let pk: i64 = row.get(0);
                let val: &str = row.get(1);
                match crypto.decrypt(val, col.aad) {
                    Ok(plain) => {
                        let masked = if plain.len() > 4 {
                            format!("{}***", &plain[..4])
                        } else {
                            "***".to_string()
                        };
                        println!("[OK]   {}.{} pk={}: decrypted → {}", target.table, col.name, pk, masked);
                    }
                    Err(e) => {
                        println!("[FAIL] {}.{} pk={}: decrypt error: {}", target.table, col.name, pk, e);
                        all_ok = false;
                    }
                }
            }
        }
    }

    if all_ok {
        println!("\n✅ Verification PASSED — all data is v{}", ring.current_version());
    } else {
        println!("\n❌ Verification FAILED — see errors above");
        std::process::exit(1);
    }

    Ok(())
}

/// 재암호화 실행: 구버전 → 현재 버전
async fn run_rekey(pool: &PgPool, ring: &KeyRing, hmac_key: &[u8; 32], batch_size: i64) -> anyhow::Result<()> {
    let current = ring.current_version();
    let crypto = CryptoService::new(ring, hmac_key);

    println!("=== Rekey Encryption ===");
    println!("Target version: v{}", current);
    println!("Batch size: {}\n", batch_size);

    // Graceful shutdown 플래그
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        println!("\n⚠ Shutdown signal received, finishing current batch...");
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    });

    for target in rekey_targets() {
        if !running.load(std::sync::atomic::Ordering::SeqCst) { break; }

        for col in &target.columns {
            if !running.load(std::sync::atomic::Ordering::SeqCst) { break; }

            println!("--- {}.{} ---", target.table, col.name);
            let mut total_rekeyed = 0u64;
            let mut last_pk: i64 = 0;

            loop {
                if !running.load(std::sync::atomic::Ordering::SeqCst) {
                    println!("  Interrupted. Rekeyed {} rows so far.", total_rekeyed);
                    break;
                }

                // 배치 조회: 현재 버전이 아닌 행만
                let select_query = format!(
                    "SELECT {pk}, {col} FROM {table} \
                     WHERE {pk} > $1 AND {col} IS NOT NULL AND {col} LIKE 'enc:v%' AND {col} NOT LIKE 'enc:v{current}:%' \
                     ORDER BY {pk} LIMIT $2 \
                     FOR UPDATE SKIP LOCKED",
                    pk = target.pk, col = col.name, table = target.table, current = current,
                );

                // lock_timeout, statement_timeout 설정
                sqlx::query("SET LOCAL lock_timeout = '5s'").execute(pool).await.ok();
                sqlx::query("SET LOCAL statement_timeout = '30s'").execute(pool).await.ok();

                let rows = sqlx::query(&select_query)
                    .bind(last_pk)
                    .bind(batch_size)
                    .fetch_all(pool)
                    .await?;

                if rows.is_empty() { break; }

                let batch_count = rows.len();

                for row in &rows {
                    let pk: i64 = row.get(0);
                    let old_val: &str = row.get(1);
                    last_pk = pk;

                    // decrypt → re-encrypt with current version
                    let plaintext = match crypto.decrypt(old_val, col.aad) {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("  [SKIP] {}.{} pk={}: decrypt failed: {}", target.table, col.name, pk, e);
                            continue;
                        }
                    };

                    let new_val = crypto.encrypt(&plaintext, col.aad)?;

                    let update_query = format!(
                        "UPDATE {table} SET {col} = $1 WHERE {pk} = $2",
                        table = target.table, col = col.name, pk = target.pk,
                    );
                    sqlx::query(&update_query)
                        .bind(&new_val)
                        .bind(pk)
                        .execute(pool)
                        .await?;
                }

                total_rekeyed += batch_count as u64;
                print!("  Rekeyed {} rows (total: {})\r", batch_count, total_rekeyed);

                // Throttle
                tokio::time::sleep(Duration::from_millis(100)).await;
            }

            println!("  {}.{}: {} rows rekeyed", target.table, col.name, total_rekeyed);
        }
    }

    println!("\n✅ Rekey complete. Run --verify to confirm.");
    Ok(())
}
