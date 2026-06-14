//! admin/guide 편집 통합 테스트 — 실 DB 경로.
//!
//! 핵심: 블록 텍스트 수정 → source_version++ → 번역 stale 판정 흐름.
//! 추가로 공개 flip(meta) + 디프 export. 자체 격리 데이터(guidev2-ag-*), 정리 포함.
//! CI "backend integration" 잡에서 --include-ignored 로 실행.

mod common;

use amazing_korean_api::api::admin::guide::dto::{GuideBlockUpdateReq, GuideMetaUpdateReq};
use amazing_korean_api::api::admin::guide::service as ag;
use amazing_korean_api::state::AppState;
use amazing_korean_api::types::{SupportedLanguage, UserAuth};

/// admin 권한 테스트 유저 생성 → user_id 반환
async fn make_admin(st: &AppState) -> i64 {
    let spec = common::TestUserSpec::random();
    let uid = common::insert_test_user(st, &spec).await;
    sqlx::query("UPDATE users SET user_auth = 'admin' WHERE user_id = $1")
        .bind(uid)
        .execute(&st.db)
        .await
        .expect("promote admin");
    uid
}

/// 격리 단원 + 블록 1(번역 대상) + 문장 + zh_cn 번역(source_version=1) 시드. (guide_id, block_id) 반환.
async fn seed(st: &AppState, idx: &str, seq: i32, sn: i32) -> (i64, i64) {
    cleanup(st, idx).await;
    let gid: i64 = sqlx::query_scalar(
        r#"INSERT INTO guide (guide_idx, guide_seq, guide_state, guide_category, guide_theme,
              sentence_start, sentence_end, title_ko, title_en)
           VALUES ($1, $2, 'ready', 'sentence_structure', 'blue', $3, $3, '테스트', 'Test')
           RETURNING guide_id"#,
    )
    .bind(idx)
    .bind(seq)
    .bind(sn)
    .fetch_one(&st.db)
    .await
    .expect("seed guide");

    let bid: i64 = sqlx::query_scalar(
        r#"INSERT INTO guide_block (guide_id, block_seq, block_type, sentence_no,
              text_ko, text_en, source_version, legacy_key)
           VALUES ($1, 20, 'section', $2, '저는 행복합니다.', 'I am happy.', 1, $3)
           RETURNING guide_block_id"#,
    )
    .bind(gid)
    .bind(sn)
    .bind(format!("{idx}:t_020"))
    .fetch_one(&st.db)
    .await
    .expect("seed block");

    sqlx::query(
        "INSERT INTO guide_sentence (guide_id, sentence_no, guide_block_id) VALUES ($1,$2,$3)",
    )
    .bind(gid)
    .bind(sn)
    .bind(bid)
    .execute(&st.db)
    .await
    .expect("seed sentence");

    // zh_cn 번역 (source_version=1 = 원문과 동일 → stale 아님)
    sqlx::query(
        r#"INSERT INTO content_translations
              (content_type, content_id, field_name, lang, translated_text, status, source_version)
           VALUES ('guide_block', $1, 'text', 'zh_cn', '我很高兴。', 'approved', 1)"#,
    )
    .bind(bid)
    .execute(&st.db)
    .await
    .expect("seed translation");

    (gid, bid)
}

async fn cleanup(st: &AppState, idx: &str) {
    sqlx::query(
        r#"DELETE FROM content_translations WHERE content_type='guide_block'
           AND content_id IN (SELECT guide_block_id FROM guide_block b
             JOIN guide g ON g.guide_id=b.guide_id WHERE g.guide_idx=$1)"#,
    )
    .bind(idx)
    .execute(&st.db)
    .await
    .ok();
    sqlx::query("DELETE FROM guide WHERE guide_idx=$1")
        .bind(idx)
        .execute(&st.db)
        .await
        .ok();
}

fn empty_block_req() -> GuideBlockUpdateReq {
    GuideBlockUpdateReq {
        text_ko: None,
        text_en: None,
    }
}

#[ignore = "requires local PostgreSQL + Redis (.env.test) — CI backend integration"]
#[tokio::test]
async fn block_edit_bumps_source_version_and_marks_translation_stale() {
    let idx = "guidev2-ag-stale";
    let st = common::make_test_state().await;
    let admin = make_admin(&st).await;
    let (_gid, bid) = seed(&st, idx, 9101, 9101).await;

    // 편집 전: zh_cn stale 0 (source_version 1 == 1)
    let before = ag::stale_dashboard(&st, admin, Some(SupportedLanguage::ZhCn))
        .await
        .expect("stale before");
    let zh_before = before.rows.iter().find(|r| r.lang == "zh_cn");
    assert_eq!(zh_before.map(|r| r.stale_count), Some(0), "편집 전 stale 0");

    // 영어 원문 수정 → source_version 2
    let mut req = empty_block_req();
    req.text_en = Some(Some("I feel happy.".into()));
    let res = ag::update_block(&st, admin, bid, req, None, None)
        .await
        .expect("update block");
    assert!(
        res.message.contains("source_version=2"),
        "버전 2로 증가: {}",
        res.message
    );

    // 편집 후: zh_cn 번역(ver1) < 원문(ver2) → stale 1
    let after = ag::stale_dashboard(&st, admin, Some(SupportedLanguage::ZhCn))
        .await
        .expect("stale after");
    let zh_after = after
        .rows
        .iter()
        .find(|r| r.lang == "zh_cn")
        .expect("zh row");
    assert_eq!(zh_after.stale_count, 1, "편집 후 stale 1");

    // 디프 export 에 이 블록 포함 (재번역 대상)
    let diff = ag::diff_export(&st, admin, SupportedLanguage::ZhCn)
        .await
        .expect("diff export");
    assert!(
        diff.items
            .iter()
            .any(|i| i.guide_block_id == bid && i.source_text == "I feel happy."),
        "디프 export 에 수정된 원문 포함"
    );

    // 동일 텍스트 재요청 = no change (버전 불변)
    let mut same = empty_block_req();
    same.text_en = Some(Some("I feel happy.".into()));
    let nores = ag::update_block(&st, admin, bid, same, None, None)
        .await
        .unwrap();
    assert!(nores.message.contains("no change"), "동일값 = no change");

    cleanup(&st, idx).await;
}

#[ignore = "requires local PostgreSQL + Redis (.env.test) — CI backend integration"]
#[tokio::test]
async fn meta_update_flips_state_to_open() {
    let idx = "guidev2-ag-flip";
    let st = common::make_test_state().await;
    let admin = make_admin(&st).await;
    seed(&st, idx, 9102, 9102).await;

    let mut req = GuideMetaUpdateReq {
        guide_state: Some("open".into()),
        guide_theme: None,
        title_ko: None,
        title_en: None,
        subtitle_ko: None,
        subtitle_en: None,
    };
    ag::update_meta(&st, admin, idx, req, None, None)
        .await
        .expect("flip open");

    let state: String =
        sqlx::query_scalar("SELECT guide_state::text FROM guide WHERE guide_idx=$1")
            .bind(idx)
            .fetch_one(&st.db)
            .await
            .unwrap();
    assert_eq!(state, "open", "공개 flip 반영");

    // 잘못된 state 거부
    req = GuideMetaUpdateReq {
        guide_state: Some("bogus".into()),
        guide_theme: None,
        title_ko: None,
        title_en: None,
        subtitle_ko: None,
        subtitle_en: None,
    };
    let err = ag::update_meta(&st, admin, idx, req, None, None).await;
    assert!(err.is_err(), "잘못된 state 거부");

    cleanup(&st, idx).await;
}

#[ignore = "requires local PostgreSQL + Redis (.env.test) — CI backend integration"]
#[tokio::test]
async fn non_admin_is_forbidden() {
    let idx = "guidev2-ag-rbac";
    let st = common::make_test_state().await;
    seed(&st, idx, 9103, 9103).await;

    // learner 권한 유저
    let spec = common::TestUserSpec::random();
    let learner = common::insert_test_user(&st, &spec).await;
    // insert_test_user 기본이 learner 가 아니면 강등
    sqlx::query("UPDATE users SET user_auth = 'learner' WHERE user_id = $1")
        .bind(learner)
        .execute(&st.db)
        .await
        .unwrap();

    let res = ag::list(&st, learner).await;
    assert!(
        matches!(res, Err(amazing_korean_api::error::AppError::Forbidden(_))),
        "learner = 403"
    );
    let _ = UserAuth::Learner; // 타입 참조 유지

    cleanup(&st, idx).await;
    common::cleanup_test_user(&st, learner).await;
}
