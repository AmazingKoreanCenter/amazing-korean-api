//! guide admin 편집 service (RBAC + audit + source_version 처리)

use std::net::IpAddr;

use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::types::{SupportedLanguage, UserAuth};
use crate::AppState;

use super::dto::{
    AdminGuideDetailRes, AdminGuideListRes, AdminOkRes, DiffExportRes, GuideBlockUpdateReq,
    GuideMetaUpdateReq, GuideSentenceUpdateReq, StaleDashboardRes,
};
use super::repo::AdminGuideRepo;

const VALID_STATES: [&str; 3] = ["ready", "open", "close"];
const VALID_THEMES: [&str; 10] = [
    "blue", "green", "orange", "purple", "pink", "teal", "indigo", "rose", "amber", "slate",
];

async fn check_admin_rbac(pool: &sqlx::PgPool, actor_user_id: i64) -> AppResult<()> {
    let actor = crate::api::user::repo::find_user(pool, actor_user_id)
        .await?
        .ok_or(AppError::Unauthorized("Actor user not found".into()))?;
    match actor.user_auth {
        UserAuth::Hymn | UserAuth::Admin | UserAuth::Manager => Ok(()),
        _ => Err(AppError::Forbidden("Forbidden".to_string())),
    }
}

async fn audit(
    st: &AppState,
    actor: i64,
    action: &str,
    target_id: Option<i64>,
    details: &serde_json::Value,
    ip: Option<IpAddr>,
    ua: Option<&str>,
) -> AppResult<()> {
    crate::api::admin::user::repo::write_audit_log(
        st, actor, action, "guide", target_id, details, ip, ua,
    )
    .await
}

pub async fn list(st: &AppState, actor: i64) -> AppResult<AdminGuideListRes> {
    check_admin_rbac(&st.db, actor).await?;
    let items = AdminGuideRepo::list(&st.db).await?;
    Ok(AdminGuideListRes { items })
}

pub async fn detail(st: &AppState, actor: i64, guide_idx: &str) -> AppResult<AdminGuideDetailRes> {
    check_admin_rbac(&st.db, actor).await?;
    let h = AdminGuideRepo::detail_header(&st.db, guide_idx)
        .await?
        .ok_or(AppError::NotFound)?;
    let blocks = AdminGuideRepo::detail_blocks(&st.db, h.guide_id).await?;
    let sentences = AdminGuideRepo::detail_sentences(&st.db, h.guide_id).await?;
    Ok(AdminGuideDetailRes {
        guide_id: h.guide_id,
        guide_idx: h.guide_idx,
        guide_seq: h.guide_seq,
        guide_state: h.guide_state,
        guide_category: h.guide_category,
        guide_theme: h.guide_theme,
        sentence_start: h.sentence_start,
        sentence_end: h.sentence_end,
        title_ko: h.title_ko,
        title_en: h.title_en,
        subtitle_ko: h.subtitle_ko,
        subtitle_en: h.subtitle_en,
        blocks,
        sentences,
    })
}

pub async fn update_meta(
    st: &AppState,
    actor: i64,
    guide_idx: &str,
    req: GuideMetaUpdateReq,
    ip: Option<IpAddr>,
    ua: Option<String>,
) -> AppResult<AdminOkRes> {
    check_admin_rbac(&st.db, actor).await?;
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    if let Some(s) = req.guide_state.as_deref() {
        if !VALID_STATES.contains(&s) {
            return Err(AppError::BadRequest(format!("invalid guide_state: {s}")));
        }
    }
    if let Some(t) = req.guide_theme.as_deref() {
        if !VALID_THEMES.contains(&t) {
            return Err(AppError::BadRequest(format!("invalid guide_theme: {t}")));
        }
    }
    let has_any = req.guide_state.is_some()
        || req.guide_theme.is_some()
        || req.title_ko.is_some()
        || req.title_en.is_some()
        || req.subtitle_ko.is_some()
        || req.subtitle_en.is_some();
    if !has_any {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    let guide_id = AdminGuideRepo::find_id(&st.db, guide_idx)
        .await?
        .ok_or(AppError::NotFound)?;

    audit(
        st,
        actor,
        "UPDATE_GUIDE_META",
        Some(guide_id),
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip,
        ua.as_deref(),
    )
    .await?;

    let mut tx = st.db.begin().await?;
    AdminGuideRepo::update_meta(
        &mut tx,
        guide_id,
        actor,
        req.guide_state.as_deref(),
        req.guide_theme.as_deref(),
        req.title_ko.as_deref(),
        req.title_en.as_deref(),
        req.subtitle_ko.as_deref(),
        req.subtitle_en.as_deref(),
    )
    .await?;
    tx.commit().await?;

    Ok(AdminOkRes {
        ok: true,
        message: "guide meta updated".into(),
    })
}

pub async fn update_block(
    st: &AppState,
    actor: i64,
    block_id: i64,
    req: GuideBlockUpdateReq,
    ip: Option<IpAddr>,
    ua: Option<String>,
) -> AppResult<AdminOkRes> {
    check_admin_rbac(&st.db, actor).await?;
    if req.text_ko.is_none() && req.text_en.is_none() {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    let before = AdminGuideRepo::find_block(&st.db, block_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Option<Option>: 미포함 = 기존 유지, 포함 = 새 값(null 가능)
    let new_ko = match &req.text_ko {
        Some(v) => v.as_deref(),
        None => before.text_ko.as_deref(),
    };
    let new_en = match &req.text_en {
        Some(v) => v.as_deref(),
        None => before.text_en.as_deref(),
    };

    // 실제 텍스트 변화 없으면 source_version 증가 안 함 (불필요 stale 회피)
    let changed = new_ko != before.text_ko.as_deref() || new_en != before.text_en.as_deref();
    if !changed {
        return Ok(AdminOkRes {
            ok: true,
            message: "no change".into(),
        });
    }

    audit(
        st,
        actor,
        "UPDATE_GUIDE_BLOCK",
        Some(block_id),
        &serde_json::json!({
            "before": {"text_ko": before.text_ko, "text_en": before.text_en},
            "after": {"text_ko": new_ko, "text_en": new_en},
        }),
        ip,
        ua.as_deref(),
    )
    .await?;

    let mut tx = st.db.begin().await?;
    let new_ver =
        AdminGuideRepo::update_block_text(&mut tx, block_id, actor, new_ko, new_en).await?;
    tx.commit().await?;

    Ok(AdminOkRes {
        ok: true,
        message: format!("block updated, source_version={new_ver} (translations now stale)"),
    })
}

pub async fn update_sentence(
    st: &AppState,
    actor: i64,
    sentence_no: i32,
    req: GuideSentenceUpdateReq,
    ip: Option<IpAddr>,
    ua: Option<String>,
) -> AppResult<AdminOkRes> {
    check_admin_rbac(&st.db, actor).await?;
    req.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let has_any = req.pron_ko.is_some()
        || req.speech_level.is_some()
        || req.subject_honorific.is_some()
        || req.audio_url.is_some();
    if !has_any {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    let sid = AdminGuideRepo::sentence_guide_id(&st.db, sentence_no)
        .await?
        .ok_or(AppError::NotFound)?;

    audit(
        st,
        actor,
        "UPDATE_GUIDE_SENTENCE",
        Some(sid),
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip,
        ua.as_deref(),
    )
    .await?;

    let mut tx = st.db.begin().await?;
    AdminGuideRepo::update_sentence_meta(
        &mut tx,
        sentence_no,
        actor,
        req.pron_ko.as_deref(),
        req.speech_level.as_deref(),
        req.subject_honorific,
        req.audio_url.as_deref(),
    )
    .await?;
    tx.commit().await?;

    Ok(AdminOkRes {
        ok: true,
        message: "sentence meta updated".into(),
    })
}

pub async fn stale_dashboard(
    st: &AppState,
    actor: i64,
    lang: Option<SupportedLanguage>,
) -> AppResult<StaleDashboardRes> {
    check_admin_rbac(&st.db, actor).await?;
    let rows = AdminGuideRepo::stale_dashboard(&st.db, lang).await?;
    Ok(StaleDashboardRes { rows })
}

pub async fn diff_export(
    st: &AppState,
    actor: i64,
    lang: SupportedLanguage,
) -> AppResult<DiffExportRes> {
    check_admin_rbac(&st.db, actor).await?;
    let items = AdminGuideRepo::diff_export(&st.db, lang).await?;
    let lang_label = serde_json::to_value(lang)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_default();
    Ok(DiffExportRes {
        lang: lang_label,
        count: items.len() as i64,
        items,
    })
}
