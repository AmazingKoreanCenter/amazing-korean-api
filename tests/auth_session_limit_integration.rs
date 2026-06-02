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

use amazing_korean_api::api::auth::dto::LoginReq;
use amazing_korean_api::api::auth::handler::ParsedUa;
use amazing_korean_api::api::auth::repo::AuthRepo;
use amazing_korean_api::api::auth::service::{AuthService, LoginOutcome};
use amazing_korean_api::types::UserAuth;
use common::{cleanup_test_user, insert_test_user, TestUserSpec};

fn parsed_ua_default() -> ParsedUa {
    ParsedUa {
        os: None,
        browser: None,
        device: "other".into(),
    }
}

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

/// refresh_hash 와 login_begin_at(FIFO 순서 결정용)까지 지정하는 확장 insert (evict 테스트 전용).
async fn insert_login_row_full(
    st: &amazing_korean_api::state::AppState,
    user_id: i64,
    state: &str,
    expire_offset_secs: i64,
    begin_offset_secs: i64,
    refresh_hash: &str,
) {
    sqlx::query(
        r#"
        INSERT INTO public.login
            (user_id, login_country, login_asn, login_org,
             login_session_id, login_refresh_hash, login_state,
             login_begin_at, login_expire_at)
        VALUES
            ($1, 'LC', 0, 'local',
             gen_random_uuid(), $2, $3::login_state_enum,
             now() + make_interval(secs => $4),
             now() + make_interval(secs => $5))
        "#,
    )
    .bind(user_id)
    .bind(refresh_hash)
    .bind(state)
    .bind(begin_offset_secs as f64)
    .bind(expire_offset_secs as f64)
    .execute(&st.db)
    .await
    .expect("insert test login row (full)");
}

/// 관리자 세션 v2: `evict_oldest_sessions_tx` 가 가장 오래된 N개 active 세션만 revoked 로
/// 전환하고 `(session_id, refresh_hash)` 를 반환한다 (동시 로그인 정확히 1 의 핵심 연산).
/// ORDER BY login_begin_at ASC(FIFO) 선택성 검증 — oldest/mid 퇴장, newest 유지.
#[tokio::test]
#[ignore]
async fn evict_oldest_sessions_tx_revokes_only_oldest_n() {
    let st = common::make_test_state().await;
    let spec = TestUserSpec::random();
    let user_id = insert_test_user(&st, &spec).await;

    // login_refresh_hash 전역 unique 제약 회피 위해 user_id 접미.
    let h_old = format!("h_old_{user_id}");
    let h_mid = format!("h_mid_{user_id}");
    let h_new = format!("h_new_{user_id}");

    // begin_offset 오름차순 = old < mid < new (전부 미래 만료 active).
    insert_login_row_full(&st, user_id, "active", 3600, -300, &h_old).await;
    insert_login_row_full(&st, user_id, "active", 3600, -200, &h_mid).await;
    insert_login_row_full(&st, user_id, "active", 3600, -100, &h_new).await;

    // 가장 오래된 2개 퇴장 (limit=2). ORDER BY ASC → old, mid.
    let mut tx = st.db.begin().await.expect("begin tx");
    let evicted = AuthRepo::evict_oldest_sessions_tx(&mut tx, user_id, 2, "session_limit_evicted")
        .await
        .expect("evict_oldest_sessions_tx");
    tx.commit().await.expect("commit");

    assert_eq!(evicted.len(), 2, "가장 오래된 2개만 퇴장");
    let evicted_hashes: Vec<&str> = evicted.iter().map(|(_, h)| h.as_str()).collect();
    assert!(evicted_hashes.contains(&h_old.as_str()), "oldest 퇴장 포함");
    assert!(evicted_hashes.contains(&h_mid.as_str()), "mid 퇴장 포함");
    assert!(!evicted_hashes.contains(&h_new.as_str()), "newest 미퇴장");

    // 최신 1개만 active 잔존 (이후 새 세션 insert 로 정확히 1 = last-login-wins).
    let live = AuthRepo::count_active_sessions(&st.db, user_id)
        .await
        .expect("count after evict");
    assert_eq!(live, 1, "최신 active 1건만 유지");

    cleanup_test_user(&st, user_id).await;
}

/// 관리자 세션 v2 **e2e**: Admin 이 한도(max)를 채운 뒤 한 번 더 로그인하면 **거부(403)가 아니라
/// evict** 로 처리되어 active 가 max 를 유지한다(last-login-wins). `AuthService::login` 전체
/// 경로(ghost cleanup → tx → `enforce_admission_in_tx` advisory lock + `evict_oldest_sessions_tx`
/// in-tx evict → insert → commit)를 실제로 통과한다.
///
/// **config 무관**: ambient `max_sessions_admin`(로컬 .env 가 2일 수도)을 그대로 읽어 검증하므로
/// 어떤 환경에서도 결정적. 핵심 판별 = 초과 로그인이 **Success**(구 reject 정책이면 여기서 403).
#[tokio::test]
#[ignore]
async fn admin_over_limit_login_evicts_instead_of_rejecting() {
    let st = common::make_test_state().await;
    let max = st.cfg.max_sessions_admin;
    assert!(max >= 1, "max_sessions_admin >= 1 (테스트 전제)");

    let spec = TestUserSpec::random(); // mfa_enabled=false, check_email=true → 평문 로그인 세션 생성
    let user_id = insert_test_user(&st, &spec).await;

    // 기본 Learner → Admin 승격 (관리자 evict 경로: in-tx advisory lock + evict).
    sqlx::query("UPDATE users SET user_auth = $1 WHERE user_id = $2")
        .bind(UserAuth::Admin)
        .bind(user_id)
        .execute(&st.db)
        .await
        .expect("promote to admin");

    let do_login = |ip: String| {
        AuthService::login(
            &st,
            LoginReq {
                email: spec.email.clone(),
                password: spec.password.clone(),
            },
            ip,
            None,
            parsed_ua_default(),
        )
    };

    // 한도(max)까지 세션을 채운다 (각자 다른 IP 로 RL 회피).
    for i in 0..max {
        let out = do_login(format!("10.0.9.{i}"))
            .await
            .unwrap_or_else(|e| panic!("fill login {i} failed: {e:?}"));
        assert!(
            matches!(out, LoginOutcome::Success(_)),
            "fill {i} = 세션 생성"
        );
    }
    assert_eq!(
        AuthRepo::count_active_sessions(&st.db, user_id)
            .await
            .expect("count after fill"),
        max,
        "한도까지 active = max"
    );

    // 한도 초과 로그인 → **evict 전환이면 Success(거부 아님)**.
    // 구 reject 정책(이 변경 전)이라면 여기서 AUTH_403_SESSION_LIMIT → .expect 패닉 = 회귀 적발.
    let over = do_login("10.0.9.250".to_string())
        .await
        .expect("over-limit login must SUCCEED under evict policy (old reject policy would 403)");
    assert!(
        matches!(over, LoginOutcome::Success(_)),
        "초과 로그인 = evict 라 거부 아님"
    );

    // evict 로 active 가 max 유지 (max+1 아님 = 가장 오래된 1개 퇴장).
    let live = AuthRepo::count_active_sessions(&st.db, user_id)
        .await
        .expect("count after over-limit");
    assert_eq!(live, max, "evict 로 active 가 max 유지 (last-login-wins)");

    // 초과분이 session_limit_evicted 로 revoked 기록.
    let evicted_cnt: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM public.login \
         WHERE user_id = $1 AND login_state = 'revoked'::login_state_enum \
         AND login_revoked_reason = 'session_limit_evicted'",
    )
    .bind(user_id)
    .fetch_one(&st.db)
    .await
    .expect("query evicted count");
    assert!(
        evicted_cnt >= 1,
        "초과분이 session_limit_evicted 로 퇴장 기록"
    );

    cleanup_test_user(&st, user_id).await;
}

// NOTE(관리자 세션 v2 동시성): advisory lock(`acquire_user_session_lock_tx`)의 TOCTOU 직렬화는
// **자동 테스트로 신뢰성 있게 검증하지 못한다** — `tokio::spawn` 으로 N 개 동시 로그인을 띄워도
// `AuthService::login` 의 argon2 비번검증(수십 ms)이 각 로그인을 시간차로 벌려 count→insert 임계
// 구역이 거의 겹치지 않아, lock 을 제거해도 테스트가 통과한다(경험적으로 확인). 즉 race 를 결정적
// 으로 재현하려면 prod 코드에 테스트 전용 배리어를 심어야 해서 채택하지 않는다. lock 원자성은
// (1) Phase 1(#317)부터 prod 동작하는 동일 메커니즘이고 (2) 코드 트레이스로 "lock 이 tx 수명 내내
// 유지되어 count→evict→insert 가 직렬화"됨이 확인된다(commit 시 해제). 본 파일은 그 결과
// 불변식(초과 로그인이 거부 아닌 evict, 최종 active==max)을 순차 경로로 검증한다.
