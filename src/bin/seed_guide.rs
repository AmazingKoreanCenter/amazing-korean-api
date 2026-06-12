//! guide(온라인 콘텐츠/해설집) 시드 적재 바이너리
//!
//! 설계 SoT = docs/AMK_GUIDE_CONTENT_DESIGN.md §4. 두 모드:
//!   --input seeds/guide_seed.json              : guide/guide_block/guide_sentence 적재
//!   --translations seeds/2_zh_cn_translated.json : 맥미니 번역 → content_translations
//!
//! D-0 (DB = 편집 가능한 SoT): 시드는 1회성 부트스트랩.
//! 재실행 가드 — 편집 흔적(updated_by_user_id NOT NULL) 존재 시 거부.
//!
//! 사용법:
//!   cargo run --bin seed_guide -- --input seeds/guide_seed.json
//!   cargo run --bin seed_guide -- --translations seeds/2_zh_cn_translated.json

use std::collections::HashMap;
use std::time::Duration;

use amazing_korean_api::config::Config;
use clap::Parser;
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Parser)]
#[command(
    name = "seed_guide",
    about = "guide 도메인 1회성 부트스트랩 시드 + 번역 적재"
)]
struct Args {
    /// guide_seed.json 경로 (구조+ko/en 적재 모드)
    #[arg(long)]
    input: Option<String>,
    /// 번역 납품본 경로 (content_translations 적재 모드, meta.target_lang 사용)
    #[arg(long)]
    translations: Option<String>,
}

// ---- --input 모드 구조 ----

#[derive(Deserialize)]
struct Seed {
    meta: SeedMeta,
    guides: Vec<SeedGuide>,
}

#[derive(Deserialize)]
struct SeedMeta {
    guide_count: i64,
    block_count: i64,
    sentence_count: i64,
}

#[derive(Deserialize)]
struct SeedGuide {
    guide_idx: String,
    guide_seq: i32,
    guide_category: String,
    guide_theme: String,
    sentence_start: i32,
    sentence_end: i32,
    title_ko: Option<String>,
    title_en: Option<String>,
    subtitle_ko: Option<String>,
    subtitle_en: Option<String>,
    blocks: Vec<SeedBlock>,
    sentences: Vec<SeedSentence>,
}

#[derive(Deserialize)]
struct SeedBlock {
    block_seq: i32,
    block_type: String,
    sentence_no: Option<i32>,
    text_ko: Option<String>,
    text_en: Option<String>,
    marker: Option<String>,
    table_no: Option<i32>,
    row_no: Option<i32>,
    col_no: Option<i32>,
    col_span: Option<i32>,
    row_span: Option<i32>,
    legacy_key: String,
}

#[derive(Deserialize)]
struct SeedSentence {
    sentence_no: i32,
    legacy_key: String,
}

// ---- --translations 모드 구조 ----

#[derive(Deserialize)]
struct TransFile {
    meta: TransMeta,
    translations: Vec<TransItem>,
}

#[derive(Deserialize)]
struct TransMeta {
    target_lang: Option<String>,
}

#[derive(Deserialize)]
struct TransItem {
    id: String,
    source_text: String,
    translated_text: String,
    status: String,
    #[serde(default)]
    ko_preserve: HashMap<String, String>,
}

/// 편집 흔적 가드 (D-0): admin 편집 이후 시드 재실행은 덮어쓰기 사고 → 거부
async fn assert_no_edits(pool: &PgPool) -> anyhow::Result<()> {
    let edited: i64 = sqlx::query_scalar(
        r#"SELECT (SELECT count(*) FROM guide WHERE updated_by_user_id IS NOT NULL)
                + (SELECT count(*) FROM guide_block WHERE updated_by_user_id IS NOT NULL)
                + (SELECT count(*) FROM guide_sentence WHERE updated_by_user_id IS NOT NULL)"#,
    )
    .fetch_one(pool)
    .await?;
    if edited > 0 {
        anyhow::bail!(
            "거부: 편집 흔적 {edited}건 존재 — DB가 SoT(D-0)로 전환된 후의 시드 재실행은 \
             온라인 편집을 덮어씁니다. 필요 시 admin 경로로 수정하세요."
        );
    }
    Ok(())
}

/// {{koN}} 자리표시자 복원 + 트레일링 LLM 찌꺼기("},") 제거
fn restore_text(item: &TransItem) -> anyhow::Result<(String, bool)> {
    let mut text = item.translated_text.clone();
    // 결함 규칙: 번역문 끝 "}," 이 원문에 없으면 LLM 산출 찌꺼기 (zh 18_002 / id 24_234)
    let stripped = if text.ends_with("},") && !item.source_text.ends_with('}') {
        text.truncate(text.len() - 2);
        true
    } else {
        false
    };
    for (key, value) in &item.ko_preserve {
        text = text.replace(&format!("{{{{{key}}}}}"), value);
    }
    if text.contains("{{") {
        anyhow::bail!("placeholder 미복원: {} {:?}", item.id, text);
    }
    Ok((text, stripped))
}

async fn load_input(pool: &PgPool, path: &str) -> anyhow::Result<()> {
    let seed: Seed = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    println!(
        "seed 로드: guides={} blocks={} sentences={} (input={path})",
        seed.meta.guide_count, seed.meta.block_count, seed.meta.sentence_count
    );

    assert_no_edits(pool).await?;
    let mut tx = pool.begin().await?;

    let (mut n_guide, mut n_block, mut n_sentence) = (0u32, 0u32, 0u32);
    for g in &seed.guides {
        let gid: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO guide
              (guide_idx, guide_seq, guide_category, guide_theme,
               sentence_start, sentence_end, title_ko, title_en, subtitle_ko, subtitle_en)
            VALUES ($1,$2,$3::guide_category_enum,$4::guide_theme_enum,$5,$6,$7,$8,$9,$10)
            ON CONFLICT (guide_idx) DO UPDATE SET
              guide_seq=EXCLUDED.guide_seq, guide_category=EXCLUDED.guide_category,
              guide_theme=EXCLUDED.guide_theme,
              sentence_start=EXCLUDED.sentence_start, sentence_end=EXCLUDED.sentence_end,
              title_ko=EXCLUDED.title_ko, title_en=EXCLUDED.title_en,
              subtitle_ko=EXCLUDED.subtitle_ko, subtitle_en=EXCLUDED.subtitle_en,
              guide_updated_at=now()
            RETURNING guide_id
            "#,
        )
        .bind(&g.guide_idx)
        .bind(g.guide_seq)
        .bind(&g.guide_category)
        .bind(&g.guide_theme)
        .bind(g.sentence_start)
        .bind(g.sentence_end)
        .bind(&g.title_ko)
        .bind(&g.title_en)
        .bind(&g.subtitle_ko)
        .bind(&g.subtitle_en)
        .fetch_one(&mut *tx)
        .await?;
        n_guide += 1;

        let mut block_id: HashMap<&str, i64> = HashMap::new();
        for b in &g.blocks {
            let bid: i64 = sqlx::query_scalar(
                r#"
                INSERT INTO guide_block
                  (guide_id, block_seq, block_type, sentence_no, text_ko, text_en, marker,
                   table_no, row_no, col_no, col_span, row_span, legacy_key)
                VALUES ($1,$2,$3::guide_block_type_enum,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
                ON CONFLICT (legacy_key) DO UPDATE SET
                  guide_id=EXCLUDED.guide_id, block_seq=EXCLUDED.block_seq,
                  block_type=EXCLUDED.block_type, sentence_no=EXCLUDED.sentence_no,
                  text_ko=EXCLUDED.text_ko, text_en=EXCLUDED.text_en,
                  marker=EXCLUDED.marker, table_no=EXCLUDED.table_no,
                  row_no=EXCLUDED.row_no, col_no=EXCLUDED.col_no,
                  col_span=EXCLUDED.col_span, row_span=EXCLUDED.row_span,
                  guide_block_updated_at=now()
                RETURNING guide_block_id
                "#,
            )
            .bind(gid)
            .bind(b.block_seq)
            .bind(&b.block_type)
            .bind(b.sentence_no)
            .bind(&b.text_ko)
            .bind(&b.text_en)
            .bind(&b.marker)
            .bind(b.table_no)
            .bind(b.row_no)
            .bind(b.col_no)
            .bind(b.col_span)
            .bind(b.row_span)
            .bind(&b.legacy_key)
            .fetch_one(&mut *tx)
            .await?;
            block_id.insert(&b.legacy_key, bid);
            n_block += 1;
        }

        for s in &g.sentences {
            let bid = block_id
                .get(s.legacy_key.as_str())
                .ok_or_else(|| anyhow::anyhow!("문장 블록 해소 불가: {}", s.legacy_key))?;
            sqlx::query(
                r#"
                INSERT INTO guide_sentence (guide_id, sentence_no, guide_block_id)
                VALUES ($1,$2,$3)
                ON CONFLICT (sentence_no) DO UPDATE SET
                  guide_id=EXCLUDED.guide_id, guide_block_id=EXCLUDED.guide_block_id,
                  guide_sentence_updated_at=now()
                "#,
            )
            .bind(gid)
            .bind(s.sentence_no)
            .bind(bid)
            .execute(&mut *tx)
            .await?;
            n_sentence += 1;
        }
    }

    tx.commit().await?;
    println!("적재 완료: guide={n_guide} block={n_block} sentence={n_sentence}");
    anyhow::ensure!(
        i64::from(n_guide) == seed.meta.guide_count
            && i64::from(n_block) == seed.meta.block_count
            && i64::from(n_sentence) == seed.meta.sentence_count,
        "meta 카운트 불일치 — 시드 파일 점검 필요"
    );
    Ok(())
}

async fn load_translations(pool: &PgPool, path: &str) -> anyhow::Result<()> {
    let file: TransFile = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    let lang = file
        .meta
        .target_lang
        .ok_or_else(|| anyhow::anyhow!("meta.target_lang 없음 — 번역 납품본이 아님: {path}"))?;
    if lang == "en" || lang == "ko" {
        anyhow::bail!(
            "거부: lang={lang} — ko/en 은 guide_block 도메인 컬럼이 원천 (참조본은 적재 대상 아님)"
        );
    }
    println!(
        "번역 로드: lang={lang} items={} (input={path})",
        file.translations.len()
    );

    // legacy_key → guide_block_id 사전 해소
    let rows: Vec<(String, i64)> = sqlx::query_as(
        r#"SELECT legacy_key, guide_block_id FROM guide_block WHERE legacy_key IS NOT NULL"#,
    )
    .fetch_all(pool)
    .await?;
    let block_id: HashMap<String, i64> = rows.into_iter().collect();
    anyhow::ensure!(
        !block_id.is_empty(),
        "guide_block 비어있음 — --input 먼저 실행"
    );

    let mut tx = pool.begin().await?;
    let (mut applied, mut stripped_total, mut skipped) = (0u32, 0u32, 0u32);
    for item in &file.translations {
        if item.status == "missing" {
            skipped += 1;
            continue;
        }
        let bid = block_id
            .get(&item.id)
            .ok_or_else(|| anyhow::anyhow!("번역 블록 해소 불가: {}", item.id))?;
        let (text, stripped) = restore_text(item)?;
        if stripped {
            stripped_total += 1;
            println!("  찌꺼기 제거: {}", item.id);
        }
        sqlx::query(
            r#"
            INSERT INTO content_translations
              (content_type, content_id, field_name, lang, translated_text, status, source_version)
            VALUES ('guide_block'::content_type_enum, $1, 'text',
                    $2::supported_language_enum, $3, 'approved'::translation_status_enum, 1)
            ON CONFLICT (content_type, content_id, field_name, lang) DO UPDATE SET
              translated_text=EXCLUDED.translated_text,
              status='approved'::translation_status_enum,
              source_version=EXCLUDED.source_version,
              updated_at=now()
            "#,
        )
        .bind(bid)
        .bind(&lang)
        .bind(&text)
        .execute(&mut *tx)
        .await?;
        applied += 1;
    }
    tx.commit().await?;
    println!("번역 적재 완료: lang={lang} applied={applied} 찌꺼기제거={stripped_total} skipped={skipped}");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    dotenvy::dotenv().ok();
    let cfg = Config::from_env();

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect(&cfg.database_url)
        .await?;

    match (&args.input, &args.translations) {
        (Some(input), None) => load_input(&pool, input).await,
        (None, Some(trans)) => load_translations(&pool, trans).await,
        _ => anyhow::bail!("--input 또는 --translations 중 정확히 하나를 지정하세요"),
    }
}
