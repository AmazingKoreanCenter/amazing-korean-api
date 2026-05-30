//! Phase 3 통합 테스트 — 세션 limit 패치 (2026-05-30 MFA 세션 limit 사건 회귀).
//!
//! 동시 세션 카운트의 권위 소스를 Redis SCARD → DB(login_state='active' AND
//! login_expire_at > now())로 전환한 것(축 A)과, 시간 기반 reaper(축 D)를 검증한다.
//! login 흐름 전체(세션 생성/cleanup 부담)는 기존 auth_login_integration 의 방침을 따라
//! 미포함 — 여기선 repo-레벨 권위 로직만 직접 검증한다.
//!
//! 실행 (DATABASE_URL/REDIS_URL/JWT_SECRET/HMAC_KEY/ENCRYPTION_KEY_V1 은 `.env.test` 제공):
//! ```bash
//! EBOOK_IMAGE_ENCRYPTION_KEY=$(openssl rand -hex 32) \
//!   cargo test --test auth_session_limit_integration -- --ignored --test-threads=1
//! ```

mod common;

use amazing_korean_api::api::auth::repo::AuthRepo;
use common::{cleanup_test_user, insert_test_user, TestUserSpec};

/// 지정한 state / 만료 offset(초, 음수=과거)으로 login 행을 직접 삽입 (테스트 전용 최소 insert).
async fn insert_login_row(
    st: &amazing_korean_api::state::AppState,
    user_id: i64,
    state: &str,
    expire_offset_secs: i64,
) {
    sqlx::query(
        r#"
        INSERT INTO public.login
            (user_id, login_country, login_asn, login_org,
             login_session_id, login_state, login_expire_at)
        VALUES
            ($1, 'LC', 0, 'local',
             gen_random_uuid(), $2::login_state_enum,
             now() + make_interval(secs => $3))
        "#,
    )
    .bind(user_id)
    .bind(state)
    .bind(expire_offset_secs as f64)
    .execute(&st.db)
    .await
    .expect("insert test login row");
}

/// 축 A: 카운트가 login_expire_at > now() 인 active 행만 센다.
/// phantom(active 이나 시간만료) 과 non-active 상태는 제외 — 사건의 거짓 403 근절 핵심.
#[tokio::test]
#[ignore]
async fn count_active_sessions_excludes_phantoms_and_nonactive() {
    let st = common::make_test_state().await;
    let spec = TestUserSpec::random();
    let user_id = insert_test_user(&st, &spec).await;

    insert_login_row(&st, user_id, "active", 3600).await; // 활성(미래)
    insert_login_row(&st, user_id, "active", 7200).await; // 활성(미래)
    insert_login_row(&st, user_id, "active", -3600).await; // phantom(시간만료) → 제외
    insert_login_row(&st, user_id, "compromised", 3600).await; // non-active → 제외
    insert_login_row(&st, user_id, "revoked", 3600).await; // non-active → 제외

    let count = AuthRepo::count_active_sessions(&st.db, user_id)
        .await
        .expect("count_active_sessions");
    assert_eq!(
        count, 2,
        "active+미래 2건만 카운트 (phantom·non-active 제외)"
    );

    cleanup_test_user(&st, user_id).await;
}

/// 축 D: reaper 는 시간만료된 active 행만 expired/ttl_reaped 로 정리한다.
#[tokio::test]
#[ignore]
async fn reap_expired_sessions_flips_only_stale_active() {
    let st = common::make_test_state().await;
    let spec = TestUserSpec::random();
    let user_id = insert_test_user(&st, &spec).await;

    insert_login_row(&st, user_id, "active", -3600).await; // stale active → reap 대상
    insert_login_row(&st, user_id, "active", 3600).await; // 미래 → 유지
    insert_login_row(&st, user_id, "compromised", -3600).await; // 이미 종료상태 → 무시

    let reaped = AuthRepo::reap_expired_sessions(&st.db)
        .await
        .expect("reap_expired_sessions");
    assert!(reaped >= 1, "stale active 1건 이상 정리");

    let stale_active_left: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM public.login \
         WHERE user_id = $1 AND login_state = 'active' AND login_expire_at < now()",
    )
    .bind(user_id)
    .fetch_one(&st.db)
    .await
    .expect("query stale active");
    assert_eq!(stale_active_left, 0, "stale active 잔존 0 (reaper 가 정리)");

    let live = AuthRepo::count_active_sessions(&st.db, user_id)
        .await
        .expect("count_active_sessions after reap");
    assert_eq!(live, 1, "미래 active 1건은 그대로 유지");

    cleanup_test_user(&st, user_id).await;
}
