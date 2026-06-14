//! guide(온라인 콘텐츠) 서빙 통합 테스트 — 실 DB 경로.
//!
//! 자체 격리 데이터(guide_idx 'guidev2-test-it')를 삽입 → GuideService 로
//! 표 재조립 + i18n 폴백 + state='open' 게이트를 검증 → 정리.
//! CI "backend integration" 잡(postgres+redis)에서 --include-ignored 로 실행.

mod common;

use amazing_korean_api::api::auth::extractor::AuthUser;
use amazing_korean_api::api::auth::jwt::Claims;
use amazing_korean_api::api::guide::dto::GuideLogReq;
use amazing_korean_api::api::guide::service::GuideService;
use amazing_korean_api::state::AppState;
use amazing_korean_api::types::{GuideActivity, GuideLogAction, SupportedLanguage, UserAuth};

/// session_id → login_id 유도용 활성 login 행 삽입 (record_log_tx 가 세션에서 login_id 유도).
async fn insert_login_session(st: &AppState, user_id: i64, session_id: &str) {
    sqlx::query(
        r#"INSERT INTO login
              (user_id, login_country, login_asn, login_org,
               login_session_id, login_state, login_expire_at)
           VALUES ($1, 'LC', 0, 'local',
               CAST($2 AS uuid), 'active', now() + interval '1 hour')"#,
    )
    .bind(user_id)
    .bind(session_id)
    .execute(&st.db)
    .await
    .expect("insert login session");
}

/// 서비스 직접 호출용 AuthUser (extractor 우회 — 세션 활성 검증은 통합 외 범위).
fn auth(user_id: i64, session_id: &str) -> AuthUser {
    AuthUser(Claims {
        sub: user_id,
        session_id: session_id.to_string(),
        role: UserAuth::Learner,
        jti: "test-jti".to_string(),
        exp: 0,
        iat: 0,
        iss: "test".to_string(),
    })
}

fn log_req(action: GuideLogAction) -> GuideLogReq {
    GuideLogReq {
        activity: GuideActivity::SentenceWrite,
        action,
        answer: Some(serde_json::json!({ "text": "저는 행복합니다." })),
    }
}

/// 격리 테스트 단원 삽입 (단원 + 제목 블록 + 표 4셀 + 문장 + zh_cn 번역).
/// 병렬 실행 격리를 위해 idx/seq 는 테스트별 고유값. 반환 = guide_id.
async fn seed_test_guide(st: &AppState, idx: &str, seq: i32, sn: i32, state: &str) -> i64 {
    cleanup(st, idx).await;
    let gid: i64 = sqlx::query_scalar(
        r#"INSERT INTO guide (guide_idx, guide_seq, guide_state, guide_category, guide_theme,
              sentence_start, sentence_end, title_ko, title_en)
           VALUES ($1, $2, $3::guide_state_enum, 'sentence_structure', 'blue',
              $4, $4, '테스트 단원', 'Test Unit')
           RETURNING guide_id"#,
    )
    .bind(idx)
    .bind(seq)
    .bind(state)
    .bind(sn)
    .fetch_one(&st.db)
    .await
    .expect("insert guide");

    // 블록: 제목(첫 블록) / 문장 SECTION / 표 헤더 R0 + 셀 R1(en, ko-only)
    let mk = |seq: i32,
              btype: &str,
              sn: Option<i32>,
              ko: Option<&str>,
              en: Option<&str>,
              tbl: Option<(i32, i32, i32)>,
              key: &str| {
        let (t, r, c) = tbl.map_or((None, None, None), |(t, r, c)| (Some(t), Some(r), Some(c)));
        (
            seq,
            btype.to_string(),
            sn,
            ko.map(String::from),
            en.map(String::from),
            t,
            r,
            c,
            key.to_string(),
        )
    };
    let k = |s: &str| format!("{idx}:{s}");
    let section_en = format!("{sn}) I am happy.");
    let blocks = vec![
        mk(
            10,
            "title",
            None,
            Some("테스트 단원"),
            Some("Test Unit"),
            None,
            &k("t_010"),
        ),
        mk(
            20,
            "section",
            Some(sn),
            Some("저는 행복합니다."),
            Some(&section_en),
            None,
            &k("t_020"),
        ),
        mk(
            30,
            "table_header",
            Some(sn),
            Some("영어"),
            Some("English"),
            Some((1, 0, 0)),
            &k("t_030"),
        ),
        mk(
            40,
            "table_cell",
            Some(sn),
            None,
            Some("happy"),
            Some((1, 1, 0)),
            &k("t_040"),
        ),
        mk(
            50,
            "table_cell",
            Some(sn),
            Some("행복하다"),
            None,
            Some((1, 1, 1)),
            &k("t_050"),
        ),
    ];
    let mut title_block_id = 0i64;
    let mut cell_en_block_id = 0i64;
    let mut section_block_id = 0i64;
    for (seq, btype, sn, ko, en, t, r, c, key) in blocks {
        let bid: i64 = sqlx::query_scalar(
            r#"INSERT INTO guide_block (guide_id, block_seq, block_type, sentence_no,
                  text_ko, text_en, table_no, row_no, col_no, legacy_key)
               VALUES ($1, $2, $3::guide_block_type_enum, $4, $5, $6, $7, $8, $9, $10)
               RETURNING guide_block_id"#,
        )
        .bind(gid)
        .bind(seq)
        .bind(&btype)
        .bind(sn)
        .bind(&ko)
        .bind(&en)
        .bind(t)
        .bind(r)
        .bind(c)
        .bind(&key)
        .fetch_one(&st.db)
        .await
        .expect("insert block");
        match seq {
            10 => title_block_id = bid,
            20 => section_block_id = bid,
            40 => cell_en_block_id = bid,
            _ => {}
        }
    }

    // 문장 학습항목 (section 블록 참조)
    sqlx::query(
        r#"INSERT INTO guide_sentence (guide_id, sentence_no, guide_block_id)
           VALUES ($1, $2, $3)"#,
    )
    .bind(gid)
    .bind(sn)
    .bind(section_block_id)
    .execute(&st.db)
    .await
    .expect("insert sentence");

    // zh_cn 번역: 제목 블록 + 표 en 셀 ("happy" → "高兴")
    for (bid, text) in [(title_block_id, "测试单元"), (cell_en_block_id, "高兴")] {
        sqlx::query(
            r#"INSERT INTO content_translations
                  (content_type, content_id, field_name, lang, translated_text, status, source_version)
               VALUES ('guide_block', $1, 'text', 'zh_cn', $2, 'approved', 1)"#,
        )
        .bind(bid)
        .bind(text)
        .execute(&st.db)
        .await
        .expect("insert translation");
    }
    gid
}

async fn cleanup(st: &AppState, idx: &str) {
    // content_translations 는 guide_block 참조(FK 아님) → 블록 삭제 전 정리
    sqlx::query(
        r#"DELETE FROM content_translations WHERE content_type='guide_block'
           AND content_id IN (SELECT guide_block_id FROM guide_block b
             JOIN guide g ON g.guide_id=b.guide_id WHERE g.guide_idx=$1)"#,
    )
    .bind(idx)
    .execute(&st.db)
    .await
    .ok();
    // guide 삭제 → block/sentence CASCADE
    sqlx::query("DELETE FROM guide WHERE guide_idx=$1")
        .bind(idx)
        .execute(&st.db)
        .await
        .ok();
}

#[ignore = "requires local PostgreSQL + Redis (.env.test) — CI backend integration"]
#[tokio::test]
async fn guide_detail_reassembles_table_and_resolves_zh_translation() {
    let idx = "guidev2-test-it-detail";
    let st = common::make_test_state().await;
    seed_test_guide(&st, idx, 9001, 901, "open").await;

    let res = GuideService::detail(&st, idx, Some(SupportedLanguage::ZhCn))
        .await
        .expect("detail ok");

    // 제목 zh 해소
    assert_eq!(res.title.as_deref(), Some("测试单元"));
    assert_eq!(res.lang, "zh-CN");

    // 스트림: title(block) + section(block) + table(격자 1) = 3 아이템
    let tables: Vec<_> = res.items.iter().filter(|i| i.kind == "table").collect();
    assert_eq!(tables.len(), 1, "표 1개로 재조립");
    let rows = tables[0].rows.as_ref().expect("표 격자");
    assert_eq!(rows.len(), 2, "R0 헤더행 + R1 셀행");
    assert!(rows[0][0].header, "R0 = table_header");
    // en 셀 "happy" → zh "高兴"
    assert_eq!(rows[1][0].text.as_deref(), Some("高兴"));
    // ko-only 셀 "행복하다" → zh 없음 → 폴백(tr→en→ko) → "행복하다"
    assert_eq!(rows[1][1].text.as_deref(), Some("행복하다"));

    // 문장 학습항목: 한국어 정답 노출
    assert_eq!(res.sentences.len(), 1);
    assert_eq!(res.sentences[0].sentence_no, 901);
    assert_eq!(
        res.sentences[0].text_ko.as_deref(),
        Some("저는 행복합니다.")
    );

    cleanup(&st, idx).await;
}

#[ignore = "requires local PostgreSQL + Redis (.env.test) — CI backend integration"]
#[tokio::test]
async fn guide_detail_404_when_not_open() {
    let idx = "guidev2-test-it-404";
    let st = common::make_test_state().await;
    seed_test_guide(&st, idx, 9002, 902, "ready").await; // 숨김 상태

    let res = GuideService::detail(&st, idx, None).await;
    assert!(
        matches!(res, Err(amazing_korean_api::error::AppError::NotFound)),
        "ready(숨김) 단원은 404"
    );

    cleanup(&st, idx).await;
}

#[ignore = "requires local PostgreSQL + Redis (.env.test) — CI backend integration"]
#[tokio::test]
async fn guide_list_only_returns_open_units() {
    let idx = "guidev2-test-it-list";
    let st = common::make_test_state().await;
    seed_test_guide(&st, idx, 9003, 903, "open").await;

    let res = GuideService::list(&st, None).await.expect("list ok");
    let found = res.items.iter().find(|i| i.guide_idx == idx);
    assert!(found.is_some(), "open 단원은 목록에 노출");
    assert_eq!(found.unwrap().guide_theme, "blue");

    // ready 로 바꾸면 목록에서 사라짐
    sqlx::query("UPDATE guide SET guide_state='ready' WHERE guide_idx=$1")
        .bind(idx)
        .execute(&st.db)
        .await
        .unwrap();
    let res2 = GuideService::list(&st, None).await.expect("list ok 2");
    assert!(
        res2.items.iter().all(|i| i.guide_idx != idx),
        "ready 단원은 목록에서 제외"
    );

    cleanup(&st, idx).await;
}

#[ignore = "requires local PostgreSQL + Redis (.env.test) — CI backend integration"]
#[tokio::test]
async fn guide_log_records_attempts_and_progress_round_trip() {
    let idx = "guidev2-test-it-log";
    let sn = 904;
    let st = common::make_test_state().await;
    seed_test_guide(&st, idx, 9004, sn, "open").await;

    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;
    let session_id = uuid::Uuid::new_v4().to_string();
    insert_login_session(&st, user_id, &session_id).await;

    // 정답: try_count 1, 해결 true, last_attempt 기록
    let s1 = GuideService::log_sentence(
        &st,
        auth(user_id, &session_id),
        idx,
        sn,
        log_req(GuideLogAction::Correct),
    )
    .await
    .expect("log correct");
    assert_eq!(s1.try_count, 1, "정답 1회 → try_count 1");
    assert!(s1.is_solved, "정답 → is_solved true");
    assert!(s1.last_attempt_at.is_some(), "정답 → last_attempt 기록");

    // 오답: try_count 2, 해결 true 유지(OR 누적)
    let s2 = GuideService::log_sentence(
        &st,
        auth(user_id, &session_id),
        idx,
        sn,
        log_req(GuideLogAction::Wrong),
    )
    .await
    .expect("log wrong");
    assert_eq!(s2.try_count, 2, "오답 추가 → try_count 2");
    assert!(s2.is_solved, "오답이어도 기존 해결 유지");

    // 뷰(비채점): status 미변경, 현재값 그대로 반환
    let s3 = GuideService::log_sentence(
        &st,
        auth(user_id, &session_id),
        idx,
        sn,
        log_req(GuideLogAction::View),
    )
    .await
    .expect("log view");
    assert_eq!(s3.try_count, 2, "view 는 try_count 미변경");
    assert!(s3.is_solved, "view 는 is_solved 미변경");

    // 진행 조회: 기록 있는 문장 1건, 최종 상태 반영
    let prog = GuideService::progress(&st, auth(user_id, &session_id), idx)
        .await
        .expect("progress");
    assert_eq!(prog.items.len(), 1, "status 행 있는 문장만(희소) 1건");
    assert_eq!(prog.items[0].sentence_no, sn);
    assert_eq!(prog.items[0].try_count, 2);
    assert!(prog.items[0].is_solved);

    cleanup(&st, idx).await;
    common::cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis (.env.test) — CI backend integration"]
#[tokio::test]
async fn guide_log_404_when_not_open() {
    let idx = "guidev2-test-it-log404";
    let sn = 905;
    let st = common::make_test_state().await;
    seed_test_guide(&st, idx, 9005, sn, "ready").await; // 숨김

    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;
    let session_id = uuid::Uuid::new_v4().to_string();
    insert_login_session(&st, user_id, &session_id).await;

    let res = GuideService::log_sentence(
        &st,
        auth(user_id, &session_id),
        idx,
        sn,
        log_req(GuideLogAction::Correct),
    )
    .await;
    assert!(
        matches!(res, Err(amazing_korean_api::error::AppError::NotFound)),
        "비공개(ready) 단원 문장 로그는 404"
    );

    cleanup(&st, idx).await;
    common::cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis (.env.test) — CI backend integration"]
#[tokio::test]
async fn guide_log_fails_closed_and_rolls_back_without_login_session() {
    let idx = "guidev2-test-it-log-nosession";
    let sn = 906;
    let st = common::make_test_state().await;
    seed_test_guide(&st, idx, 9006, sn, "open").await;

    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await; // login 행은 의도적으로 미삽입
    let bogus_session = uuid::Uuid::new_v4().to_string();

    let res = GuideService::log_sentence(
        &st,
        auth(user_id, &bogus_session),
        idx,
        sn,
        log_req(GuideLogAction::Correct),
    )
    .await;
    assert!(
        matches!(res, Err(amazing_korean_api::error::AppError::Internal(_))),
        "유효 login 세션 없으면 Internal(fail-closed)"
    );

    // tx 롤백: status 행이 남지 않아야 함(원자성)
    let cnt: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM guide_sentence_status WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&st.db)
            .await
            .expect("count status");
    assert_eq!(cnt, 0, "log 실패 시 status upsert 도 롤백");

    cleanup(&st, idx).await;
    common::cleanup_test_user(&st, user_id).await;
}
