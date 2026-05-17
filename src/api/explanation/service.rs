//! 해설 콘텐츠 조회 service (오버레이 + inherit 계승 + 폴백 재조립)

use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::SupportedLanguage;

use super::dto::{ExplanationBlockRes, ExplanationListRes, ExplanationUnitRes};
use super::repo::{ExplanationRepo, UnitRow};

pub struct ExplanationService;

impl ExplanationService {
    pub async fn get_unit(
        state: &AppState,
        unit_idx: &str,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<ExplanationUnitRes> {
        let unit = ExplanationRepo::find_unit_by_idx(&state.db, unit_idx)
            .await?
            .ok_or(AppError::NotFound)?;
        build_unit(state, unit, lang).await
    }

    pub async fn list_by_link(
        state: &AppState,
        study_idx: Option<&str>,
        study_task_idx: Option<&str>,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<ExplanationListRes> {
        if study_idx.is_none() && study_task_idx.is_none() {
            return Err(AppError::BadRequest(
                "study_idx 또는 study_task_idx 필요".into(),
            ));
        }
        let units =
            ExplanationRepo::find_units_by_link(&state.db, study_idx, study_task_idx).await?;
        let mut items = Vec::with_capacity(units.len());
        for u in units {
            items.push(build_unit(state, u, lang).await?);
        }
        Ok(ExplanationListRes { items })
    }
}

/// 폴백 체인: 요청 언어 우선 → tr(user/en) → en → ko 중 첫 비어있지 않은 값
fn resolve(
    lang: Option<SupportedLanguage>,
    ko: &Option<String>,
    en: &Option<String>,
    tr: Option<&String>,
) -> Option<String> {
    let chain: Vec<Option<&String>> = match lang {
        None | Some(SupportedLanguage::Ko) => vec![ko.as_ref(), tr, en.as_ref()],
        Some(SupportedLanguage::En) => vec![en.as_ref(), tr, ko.as_ref()],
        Some(_) => vec![tr, en.as_ref(), ko.as_ref()],
    };
    chain.into_iter().flatten().find(|s| !s.is_empty()).cloned()
}

fn lang_label(lang: Option<SupportedLanguage>) -> String {
    match lang {
        None => "ko".to_string(),
        Some(l) => serde_json::to_value(l)
            .ok()
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_else(|| "ko".to_string()),
    }
}

async fn build_unit(
    state: &AppState,
    u: UnitRow,
    lang: Option<SupportedLanguage>,
) -> AppResult<ExplanationUnitRes> {
    let eff_lang = lang.unwrap_or(SupportedLanguage::Ko);
    let blocks = ExplanationRepo::find_blocks(&state.db, u.explanation_unit_id).await?;

    // unit-level 번역 (title/subtitle)
    let unit_tr = ExplanationRepo::find_translations(
        &state.db,
        "explanation_unit",
        &[i64::from(u.explanation_unit_id)],
        eff_lang,
    )
    .await?;
    let uid = i64::from(u.explanation_unit_id);
    let title = resolve(
        lang,
        &u.title_ko,
        &u.title_en,
        unit_tr.get(&(uid, "explanation_unit_title".to_string())),
    );
    let subtitle = resolve(
        lang,
        &u.subtitle_ko,
        &u.subtitle_en,
        unit_tr.get(&(uid, "explanation_unit_subtitle".to_string())),
    );

    // block-level 번역 일괄 조회
    let block_ids: Vec<i64> = blocks
        .iter()
        .map(|b| i64::from(b.explanation_block_id))
        .collect();
    let block_tr =
        ExplanationRepo::find_translations(&state.db, "explanation_block", &block_ids, eff_lang)
            .await?;

    let mut out_blocks = Vec::with_capacity(blocks.len());
    for b in &blocks {
        let bid = i64::from(b.explanation_block_id);
        let structured: Option<serde_json::Value> = b
            .structured_txt
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok());

        // 이 블록의 i18n 맵 (find_translations 가 이미 user/en 해소)
        let mut i18n: std::collections::BTreeMap<String, String> = block_tr
            .iter()
            .filter(|((cid, _), _)| *cid == bid)
            .map(|((_, f), t)| (f.clone(), t.clone()))
            .collect();

        // inherit 계승: structured_explain rows[i].inherit=true →
        // 직전 비-inherit row 의 explanation 을 i18n 에 채움
        if b.block_type == "structured_explain" {
            if let Some(rows) = structured
                .as_ref()
                .and_then(|s| s.get("rows"))
                .and_then(|r| r.as_array())
            {
                let mut last_expl: Option<String> = None;
                for (i, row) in rows.iter().enumerate() {
                    let is_inherit = row
                        .get("inherit")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false);
                    let key = format!("explanation_block_row_{i}_explanation");
                    if is_inherit {
                        if let Some(prev) = &last_expl {
                            i18n.entry(key).or_insert_with(|| prev.clone());
                        }
                    } else if let Some(v) = i18n.get(&key) {
                        last_expl = Some(v.clone());
                    }
                }
            }
        }

        let text = if matches!(
            b.block_type.as_str(),
            "paragraph" | "heading" | "subtitle" | "step"
        ) {
            resolve(
                lang,
                &b.text_ko,
                &b.text_en,
                i18n.get("explanation_block_text"),
            )
        } else {
            None
        };

        out_blocks.push(ExplanationBlockRes {
            block_seq: b.block_seq,
            block_type: b.block_type.clone(),
            level: b.block_level,
            text,
            raw: b.raw.clone(),
            structured,
            i18n,
        });
    }

    Ok(ExplanationUnitRes {
        unit_idx: u.unit_idx,
        unit_kind: u.unit_kind,
        unit_source: u.unit_source,
        study_idx: u.study_idx,
        study_task_idx: u.study_task_idx,
        sentence_num: u.sentence_num,
        section_id: u.section_id,
        title,
        subtitle,
        lang: lang_label(lang),
        blocks: out_blocks,
    })
}
