//! guide 콘텐츠 조회 service — i18n 해소 + 표 재조립(D-7) + 문장 학습항목

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::api::auth::extractor::AuthUser;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::{GuideLogAction, SupportedLanguage};

use super::dto::{
    GuideCellRes, GuideDetailRes, GuideItemRes, GuideListRes, GuideLogReq, GuideProgressItemRes,
    GuideProgressRes, GuideSentenceRes, GuideSentenceStatusRes, GuideSummaryRes,
};
use super::repo::{BlockRow, GuideRepo};

pub struct GuideService;

impl GuideService {
    pub async fn list(
        state: &AppState,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<GuideListRes> {
        // ko/en 은 도메인 컬럼이 원천 — 번역 LATERAL 불요
        let tr_lang = effective_tr_lang(lang);
        let rows = GuideRepo::list_open(&state.db, tr_lang).await?;
        let items = rows
            .into_iter()
            .map(|r| GuideSummaryRes {
                title: resolve(lang, &r.title_ko, &r.title_en, r.title_tr.as_ref()),
                title_ko: r.title_ko,
                subtitle: resolve(lang, &r.subtitle_ko, &r.subtitle_en, None),
                subtitle_ko: r.subtitle_ko,
                guide_idx: r.guide_idx,
                guide_seq: r.guide_seq,
                guide_category: r.guide_category,
                guide_theme: r.guide_theme,
                sentence_start: r.sentence_start,
                sentence_end: r.sentence_end,
            })
            .collect();
        Ok(GuideListRes {
            items,
            lang: lang_label(lang),
        })
    }

    pub async fn detail(
        state: &AppState,
        guide_idx: &str,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<GuideDetailRes> {
        let g = GuideRepo::find_open_by_idx(&state.db, guide_idx)
            .await?
            .ok_or(AppError::NotFound)?;
        let blocks = GuideRepo::find_blocks(&state.db, g.guide_id).await?;
        let sentences = GuideRepo::find_sentences(&state.db, g.guide_id).await?;

        let tr = match effective_tr_lang(lang) {
            Some(l) => {
                let ids: Vec<i64> = blocks.iter().map(|b| b.guide_block_id).collect();
                GuideRepo::find_block_translations(&state.db, &ids, l).await?
            }
            None => HashMap::new(),
        };

        // 제목 = 첫 블록 (시드 변환기 규칙), 부제 = guide 컬럼과 텍스트 일치하는 첫 paragraph
        let title_tr = blocks.first().and_then(|b| tr.get(&b.guide_block_id));
        let subtitle_tr = blocks
            .iter()
            .find(|b| {
                b.block_type == "paragraph"
                    && b.text_en.is_some()
                    && b.text_en == g.subtitle_en
                    && Some(b.guide_block_id) != blocks.first().map(|f| f.guide_block_id)
            })
            .and_then(|b| tr.get(&b.guide_block_id));

        let items = assemble_items(&blocks, &tr, lang);

        let block_by_id: HashMap<i64, &BlockRow> =
            blocks.iter().map(|b| (b.guide_block_id, b)).collect();
        let sentences = sentences
            .into_iter()
            .map(|s| {
                let b = block_by_id.get(&s.guide_block_id);
                GuideSentenceRes {
                    sentence_no: s.sentence_no,
                    text_ko: b.and_then(|b| b.text_ko.clone()),
                    text: b.and_then(|b| {
                        resolve(lang, &b.text_ko, &b.text_en, tr.get(&b.guide_block_id))
                    }),
                    pron_ko: s.pron_ko,
                    audio_url: s.audio_url,
                }
            })
            .collect();

        Ok(GuideDetailRes {
            title: resolve(lang, &g.title_ko, &g.title_en, title_tr),
            title_ko: g.title_ko,
            subtitle: resolve(lang, &g.subtitle_ko, &g.subtitle_en, subtitle_tr),
            subtitle_ko: g.subtitle_ko,
            guide_idx: g.guide_idx,
            guide_seq: g.guide_seq,
            guide_category: g.guide_category,
            guide_theme: g.guide_theme,
            sentence_start: g.sentence_start,
            sentence_end: g.sentence_end,
            lang: lang_label(lang),
            items,
            sentences,
        })
    }

    /// 문장 학습 로그 기록(시도/정오). 정/오 액션만 status(try_count/is_solved) 갱신.
    /// 반환 = 기록 직후 권위 상태(프론트 낙관적 업데이트 정합용).
    pub async fn log_sentence(
        state: &AppState,
        auth_user: AuthUser,
        guide_idx: &str,
        sentence_no: i32,
        req: GuideLogReq,
    ) -> AppResult<GuideSentenceStatusRes> {
        let AuthUser(claims) = auth_user;

        let sentence_id = GuideRepo::find_open_sentence_id(&state.db, guide_idx, sentence_no)
            .await?
            .ok_or(AppError::NotFound)?;

        // 비즈니스 규칙: 채점 결과(correct/wrong)만 status 반영, correct 만 해결 처리.
        let affects_status = matches!(req.action, GuideLogAction::Correct | GuideLogAction::Wrong);
        let is_solved = matches!(req.action, GuideLogAction::Correct);

        let status = GuideRepo::record_log_tx(
            &state.db,
            claims.sub,
            &claims.session_id,
            sentence_id,
            req.activity,
            req.action,
            req.answer.as_ref(),
            affects_status,
            is_solved,
        )
        .await?;

        Ok(GuideSentenceStatusRes {
            try_count: status.try_count,
            is_solved: status.is_solved,
            last_attempt_at: status.last_attempt_at,
        })
    }

    /// 내 단원 진행 상황(공개 단원 한정). status 행이 있는 문장만 sentence_no 순.
    pub async fn progress(
        state: &AppState,
        auth_user: AuthUser,
        guide_idx: &str,
    ) -> AppResult<GuideProgressRes> {
        let AuthUser(claims) = auth_user;

        let g = GuideRepo::find_open_by_idx(&state.db, guide_idx)
            .await?
            .ok_or(AppError::NotFound)?;

        let rows = GuideRepo::find_progress(&state.db, claims.sub, g.guide_id).await?;
        let items = rows
            .into_iter()
            .map(|r| GuideProgressItemRes {
                sentence_no: r.sentence_no,
                try_count: r.try_count,
                is_solved: r.is_solved,
                last_attempt_at: r.last_attempt_at,
            })
            .collect();
        Ok(GuideProgressRes { items })
    }
}

/// 번역 조회 대상 언어 — ko/en 은 도메인 컬럼이 원천이라 None
fn effective_tr_lang(lang: Option<SupportedLanguage>) -> Option<SupportedLanguage> {
    match lang {
        None | Some(SupportedLanguage::Ko) | Some(SupportedLanguage::En) => None,
        Some(l) => Some(l),
    }
}

/// 폴백 체인 (explanation 선례): ko 요청=ko→tr→en / en=en→tr→ko / 기타=tr→en→ko
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

/// 블록 스트림 조립: 일반 블록은 그대로, 표(table_no)는 첫 셀 위치에서 격자로 재조립
fn assemble_items(
    blocks: &[BlockRow],
    tr: &HashMap<i64, String>,
    lang: Option<SupportedLanguage>,
) -> Vec<GuideItemRes> {
    let mut emitted_tables: HashSet<i32> = HashSet::new();
    let mut items = Vec::new();

    for b in blocks {
        match b.table_no {
            None => items.push(GuideItemRes {
                kind: "block".to_string(),
                block_seq: b.block_seq,
                sentence_no: b.sentence_no,
                block_type: Some(b.block_type.clone()),
                text: resolve(lang, &b.text_ko, &b.text_en, tr.get(&b.guide_block_id)),
                text_ko: b.text_ko.clone(),
                marker: b.marker.clone(),
                table_no: None,
                rows: None,
            }),
            Some(tno) => {
                if !emitted_tables.insert(tno) {
                    continue; // 이미 조립된 표의 후속 셀
                }
                let cells: Vec<&BlockRow> =
                    blocks.iter().filter(|c| c.table_no == Some(tno)).collect();
                // row_no → (col_no → 셀) 격자 (col_no = 행 내 등장 순번)
                let mut grid: BTreeMap<i32, BTreeMap<i32, &BlockRow>> = BTreeMap::new();
                for c in &cells {
                    grid.entry(c.row_no.unwrap_or(0))
                        .or_default()
                        .insert(c.col_no.unwrap_or(0), c);
                }
                let rows: Vec<Vec<GuideCellRes>> = grid
                    .values()
                    .map(|row| {
                        row.values()
                            .map(|c| GuideCellRes {
                                text: resolve(
                                    lang,
                                    &c.text_ko,
                                    &c.text_en,
                                    tr.get(&c.guide_block_id),
                                ),
                                text_ko: c.text_ko.clone(),
                                marker: c.marker.clone(),
                                header: c.block_type == "table_header",
                                col_span: c.col_span,
                                row_span: c.row_span,
                            })
                            .collect()
                    })
                    .collect();
                items.push(GuideItemRes {
                    kind: "table".to_string(),
                    block_seq: b.block_seq,
                    sentence_no: b.sentence_no,
                    block_type: None,
                    text: None,
                    text_ko: None,
                    marker: None,
                    table_no: Some(tno),
                    rows: Some(rows),
                });
            }
        }
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(
        id: i64,
        seq: i32,
        btype: &str,
        table: Option<(i32, i32, i32)>,
        en: Option<&str>,
        ko: Option<&str>,
    ) -> BlockRow {
        BlockRow {
            guide_block_id: id,
            block_seq: seq,
            block_type: btype.to_string(),
            sentence_no: None,
            text_ko: ko.map(String::from),
            text_en: en.map(String::from),
            marker: None,
            table_no: table.map(|t| t.0),
            row_no: table.map(|t| t.1),
            col_no: table.map(|t| t.2),
            col_span: None,
            row_span: None,
        }
    }

    #[test]
    fn assemble_collapses_table_cells_into_grid_at_first_cell_position() {
        let blocks = vec![
            block(1, 10, "paragraph", None, Some("intro"), Some("도입")),
            block(2, 20, "table_header", Some((1, 0, 0)), Some("H"), None),
            block(3, 30, "table_cell", Some((1, 1, 0)), Some("a"), None),
            block(4, 40, "table_cell", Some((1, 1, 1)), None, Some("나")),
            block(5, 50, "note", None, Some("after"), None),
        ];
        let items = assemble_items(&blocks, &HashMap::new(), Some(SupportedLanguage::En));

        assert_eq!(items.len(), 3); // paragraph + table(셀 3 병합) + note
        assert_eq!(items[0].kind, "block");
        assert_eq!(items[1].kind, "table");
        assert_eq!(items[1].block_seq, 20); // 첫 셀 위치
        let rows = items[1].rows.as_ref().unwrap();
        assert_eq!(rows.len(), 2); // R0 헤더 + R1
        assert!(rows[0][0].header);
        assert_eq!(rows[1].len(), 2);
        assert_eq!(rows[1][0].text.as_deref(), Some("a"));
        assert_eq!(rows[1][1].text.as_deref(), Some("나")); // en 없음 → ko 폴백
        assert_eq!(items[2].kind, "block");
    }

    #[test]
    fn resolve_prefers_translation_for_third_languages_and_falls_back() {
        let ko = Some("한국어".to_string());
        let en = Some("english".to_string());
        let tr = "中文".to_string();
        // 제3언어: tr → en → ko
        assert_eq!(
            resolve(Some(SupportedLanguage::ZhCn), &ko, &en, Some(&tr)).as_deref(),
            Some("中文")
        );
        assert_eq!(
            resolve(Some(SupportedLanguage::ZhCn), &ko, &en, None).as_deref(),
            Some("english")
        );
        assert_eq!(
            resolve(Some(SupportedLanguage::ZhCn), &ko, &None, None).as_deref(),
            Some("한국어")
        );
        // ko 요청: ko 우선 / en 요청: en 우선
        assert_eq!(resolve(None, &ko, &en, None).as_deref(), Some("한국어"));
        assert_eq!(
            resolve(Some(SupportedLanguage::En), &ko, &en, None).as_deref(),
            Some("english")
        );
    }
}
