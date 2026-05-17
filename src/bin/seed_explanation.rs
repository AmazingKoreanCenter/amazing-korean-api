//! 해설(설명) 콘텐츠 시드 적재 바이너리
//!
//! books `explanation_seed.json` (산출 A 구조 시드 + 산출 B 번역행) 을
//! 멱등 적재한다. 계약 = amazing-korean-books/docs/guide/explanation_seed_contract_from_api.md
//! / api SSoT = docs/AMK_API_LEARNING.md §5.10.
//!
//! 흐름:
//!   ① explanation_unit upsert (unit_idx 기준) → PK 확정
//!   ② explanation_block upsert ((unit_idx, block_seq) 기준) → PK 확정
//!   ③ 산출 B → content_translations upsert (unit_idx[+block_seq] → PK 해소, lang=en, status=approved)
//!   ④ 연결키(study_idx/study_task_idx) 정합 검증 리포트 (논리 참조 — 경고만)
//!
//! 사용법:
//!   cargo run --bin seed_explanation -- --input ../amazing-korean-books/scripts/guide-v2/data/explanation_seed.json
//!   EXPLANATION_SEED_PATH=... cargo run --bin seed_explanation

use std::collections::HashMap;
use std::time::Duration;

use amazing_korean_api::config::Config;
use clap::Parser;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;

#[derive(Parser)]
#[command(
    name = "seed_explanation",
    about = "Idempotently load books explanation_seed.json (산출 A + B)"
)]
struct Args {
    /// explanation_seed.json 경로 (미지정 시 env EXPLANATION_SEED_PATH)
    #[arg(long)]
    input: Option<String>,
}

#[derive(Deserialize)]
struct Seed {
    units: Vec<SeedUnit>,
    translations: Vec<SeedTranslation>,
}

#[derive(Deserialize)]
struct SeedUnit {
    unit_idx: String,
    unit_seq: i32,
    unit_kind: String,
    unit_source: String,
    study_idx: Option<String>,
    study_task_idx: Option<String>,
    sentence_num: Option<i32>,
    section_id: Option<String>,
    link_meta: Option<serde_json::Value>,
    title_ko: Option<String>,
    title_en: Option<String>,
    #[serde(default)]
    title_lang_invariant: bool,
    subtitle_ko: Option<String>,
    subtitle_en: Option<String>,
    #[serde(default)]
    subtitle_lang_invariant: bool,
    blocks: Vec<SeedBlock>,
}

#[derive(Deserialize)]
struct SeedBlock {
    block_seq: i32,
    block_type: String,
    block_level: Option<i16>,
    text_ko: Option<String>,
    text_en: Option<String>,
    #[serde(default)]
    text_lang_invariant: bool,
    raw: Option<String>,
    structured: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct SeedTranslation {
    unit_idx: String,
    block_seq: Option<i32>,
    field_name: String,
    lang: String,
    text: String,
}

/// jsonb 바인딩용: Value → 문자열 (sqlx json feature 미사용, ::jsonb 캐스트)
fn json_str(v: &Option<serde_json::Value>) -> Option<String> {
    v.as_ref().map(|x| x.to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    dotenvy::dotenv().ok();
    let cfg = Config::from_env();

    let input = args
        .input
        .or_else(|| std::env::var("EXPLANATION_SEED_PATH").ok())
        .ok_or_else(|| anyhow::anyhow!("--input 또는 EXPLANATION_SEED_PATH 필요"))?;

    let raw = std::fs::read_to_string(&input)?;
    let seed: Seed = serde_json::from_str(&raw)?;
    println!(
        "seed 로드: units={} translations={} (input={})",
        seed.units.len(),
        seed.translations.len(),
        input
    );

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect(&cfg.database_url)
        .await?;

    let mut tx = pool.begin().await?;

    // ① explanation_unit upsert → unit_idx -> explanation_unit_id
    let mut unit_id: HashMap<String, i32> = HashMap::new();
    let mut block_id: HashMap<(String, i32), i32> = HashMap::new();

    for u in &seed.units {
        let uid: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO explanation_unit
              (unit_idx, unit_seq, unit_kind, unit_source, study_idx, study_task_idx,
               sentence_num, section_id, link_meta, title_ko, title_en, title_lang_invariant,
               subtitle_ko, subtitle_en, subtitle_lang_invariant)
            VALUES ($1,$2,$3::explanation_unit_kind_enum,$4::explanation_source_enum,$5,$6,
                    $7,$8,$9::jsonb,$10,$11,$12,$13,$14,$15)
            ON CONFLICT (unit_idx) DO UPDATE SET
              unit_seq=EXCLUDED.unit_seq, unit_kind=EXCLUDED.unit_kind,
              unit_source=EXCLUDED.unit_source, study_idx=EXCLUDED.study_idx,
              study_task_idx=EXCLUDED.study_task_idx, sentence_num=EXCLUDED.sentence_num,
              section_id=EXCLUDED.section_id, link_meta=EXCLUDED.link_meta,
              title_ko=EXCLUDED.title_ko, title_en=EXCLUDED.title_en,
              title_lang_invariant=EXCLUDED.title_lang_invariant,
              subtitle_ko=EXCLUDED.subtitle_ko, subtitle_en=EXCLUDED.subtitle_en,
              subtitle_lang_invariant=EXCLUDED.subtitle_lang_invariant,
              explanation_unit_updated_at=now()
            RETURNING explanation_unit_id
            "#,
        )
        .bind(&u.unit_idx)
        .bind(u.unit_seq)
        .bind(&u.unit_kind)
        .bind(&u.unit_source)
        .bind(&u.study_idx)
        .bind(&u.study_task_idx)
        .bind(u.sentence_num)
        .bind(&u.section_id)
        .bind(json_str(&u.link_meta))
        .bind(&u.title_ko)
        .bind(&u.title_en)
        .bind(u.title_lang_invariant)
        .bind(&u.subtitle_ko)
        .bind(&u.subtitle_en)
        .bind(u.subtitle_lang_invariant)
        .fetch_one(&mut *tx)
        .await?;
        unit_id.insert(u.unit_idx.clone(), uid);

        // ② explanation_block upsert → (unit_idx, block_seq) -> explanation_block_id
        for b in &u.blocks {
            let bid: i32 = sqlx::query_scalar(
                r#"
                INSERT INTO explanation_block
                  (explanation_unit_id, block_seq, block_type, block_level,
                   text_ko, text_en, text_lang_invariant, raw, structured)
                VALUES ($1,$2,$3::explanation_block_type_enum,$4,$5,$6,$7,$8,$9::jsonb)
                ON CONFLICT (explanation_unit_id, block_seq) DO UPDATE SET
                  block_type=EXCLUDED.block_type, block_level=EXCLUDED.block_level,
                  text_ko=EXCLUDED.text_ko, text_en=EXCLUDED.text_en,
                  text_lang_invariant=EXCLUDED.text_lang_invariant,
                  raw=EXCLUDED.raw, structured=EXCLUDED.structured,
                  explanation_block_updated_at=now()
                RETURNING explanation_block_id
                "#,
            )
            .bind(uid)
            .bind(b.block_seq)
            .bind(&b.block_type)
            .bind(b.block_level)
            .bind(&b.text_ko)
            .bind(&b.text_en)
            .bind(b.text_lang_invariant)
            .bind(&b.raw)
            .bind(json_str(&b.structured))
            .fetch_one(&mut *tx)
            .await?;
            block_id.insert((u.unit_idx.clone(), b.block_seq), bid);
        }
    }

    // ③ 산출 B → content_translations upsert (lang=en, status=approved — en = 권위 텍스트)
    let mut applied = 0u32;
    for t in &seed.translations {
        let (content_type, content_id): (&str, i64) = match t.block_seq {
            None => (
                "explanation_unit",
                i64::from(
                    *unit_id
                        .get(&t.unit_idx)
                        .ok_or_else(|| anyhow::anyhow!("산출 B unit 해소 불가: {}", t.unit_idx))?,
                ),
            ),
            Some(bs) => (
                "explanation_block",
                i64::from(*block_id.get(&(t.unit_idx.clone(), bs)).ok_or_else(|| {
                    anyhow::anyhow!("산출 B block 해소 불가: {} #{}", t.unit_idx, bs)
                })?),
            ),
        };

        sqlx::query(
            r#"
            INSERT INTO content_translations
              (content_type, content_id, field_name, lang, translated_text, status)
            VALUES ($1::content_type_enum, $2, $3, $4::supported_language_enum, $5,
                    'approved'::translation_status_enum)
            ON CONFLICT (content_type, content_id, field_name, lang) DO UPDATE SET
              translated_text=EXCLUDED.translated_text,
              status='approved'::translation_status_enum,
              updated_at=now()
            "#,
        )
        .bind(content_type)
        .bind(content_id)
        .bind(&t.field_name)
        .bind(&t.lang)
        .bind(&t.text)
        .execute(&mut *tx)
        .await?;
        applied += 1;
    }

    tx.commit().await?;
    println!(
        "적재 완료: unit={} block={} translation={}",
        unit_id.len(),
        block_id.len(),
        applied
    );

    // ④ 연결키 정합 검증 (논리 참조 — FK 아님. 경고만, 실패 아님)
    let orphan_study: i64 = sqlx::query_scalar(
        r#"SELECT count(*) FROM explanation_unit eu
           WHERE eu.study_idx IS NOT NULL
             AND NOT EXISTS (SELECT 1 FROM study s WHERE s.study_idx = eu.study_idx)"#,
    )
    .fetch_one(&pool)
    .await?;
    let orphan_task: i64 = sqlx::query_scalar(
        r#"SELECT count(*) FROM explanation_unit eu
           WHERE eu.study_task_idx IS NOT NULL
             AND NOT EXISTS (SELECT 1 FROM study_task st WHERE st.study_task_idx = eu.study_task_idx)"#,
    )
    .fetch_one(&pool)
    .await?;
    println!(
        "연결키 정합: study_idx 미해소={} / study_task_idx 미해소={} (논리 참조 — 시드 순서 독립, 경고)",
        orphan_study, orphan_task
    );
    if orphan_study > 0 || orphan_task > 0 {
        println!("⚠️  미해소 연결키 존재 — study/study_task 시드 적재 후 재확인 권장 (정상 케이스: 선행 미적재)");
    }

    Ok(())
}
