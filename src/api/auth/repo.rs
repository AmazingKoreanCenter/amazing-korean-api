use crate::error::AppResult;
use crate::types::UserAuth;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};

pub struct AuthRepo;

// =========================================================================
// Internal Data Models (DB Rows)
// =========================================================================

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserFindIdInfo {
    pub user_id: i64,
    pub user_email: String,
    pub user_name: String,
    pub user_birthday: Option<String>,
    pub user_password: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserLoginInfo {
    pub user_id: i64,
    pub user_email: String,
    pub user_password: Option<String>, // NULL for social-only accounts
    pub user_state: bool,
    pub user_auth: UserAuth,
    pub user_check_email: bool,
    pub user_mfa_enabled: bool,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct LoginRecord {
    pub user_id: i64,
    pub session_id: String, // UUID -> String
    pub refresh_hash: String,
    pub login_ip: Option<String>, // Inet -> String
    pub login_device: String,     // Enum -> String
    pub login_browser: Option<String>,
    pub login_os: Option<String>,
    pub user_agent: Option<String>,
    pub login_method: String,
    pub login_time_at: DateTime<Utc>,
    pub logout_time_at: Option<DateTime<Utc>>, // Legacy field (Always NULL)
    pub state: String,
}

// =========================================================================
// Repository Implementation
// =========================================================================

impl AuthRepo {
    // ---------------------------------------------------------------------
    // User Core Lookups
    // ---------------------------------------------------------------------

    pub async fn update_user_password_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        new_password_hash: &str,
    ) -> AppResult<()> {
        let res = sqlx::query(
            r#"
            UPDATE users
            SET user_password = $2
            WHERE user_id = $1
        "#,
        )
        .bind(user_id)
        .bind(new_password_hash)
        .execute(&mut **tx)
        .await?;

        if res.rows_affected() == 0 {
            return Err(crate::error::AppError::NotFound);
        }
        Ok(())
    }

    // ---------------------------------------------------------------------
    // Login Records (Session & Logs)
    // ---------------------------------------------------------------------

    /// 로그인 세션 기록 생성 (Login Table)
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_login_record_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        session_id: &str,
        refresh_hash: &str,
        login_ip: &str,
        device: Option<&str>,
        browser: Option<&str>,
        os: Option<&str>,
        user_agent: Option<&str>,
        refresh_ttl_secs: i64,
        country_code: Option<&str>,
        asn: Option<i64>,
        org: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO public.login (
                user_id,
                login_ip,
                login_device,
                login_browser,
                login_os,
                login_method,
                login_session_id,
                login_refresh_hash,
                login_user_agent,
                login_begin_at,
                login_state,
                login_expire_at,
                login_active_at,
                login_country, login_asn, login_org,
                login_revoked_reason
            )
            VALUES (
                $1,
                $2,
                CASE lower($3)
                    WHEN 'mobile'  THEN 'mobile'::login_device_enum
                    WHEN 'tablet'  THEN 'tablet'::login_device_enum
                    WHEN 'desktop' THEN 'desktop'::login_device_enum
                    WHEN 'web'     THEN 'desktop'::login_device_enum
                    WHEN 'browser' THEN 'desktop'::login_device_enum
                    ELSE 'other'::login_device_enum
                END,
                $4,
                $5,
                'email'::login_method_enum,
                CAST($6 AS uuid),
                $7,
                $8,
                NOW(),
                'active'::login_state_enum,
                NOW() + make_interval(secs => $9),
                NOW(),
                COALESCE($10, 'LC'), COALESCE($11, 0), COALESCE($12, 'local'),
                'none'
            )
        "#,
        )
        .bind(user_id)
        .bind(login_ip)
        .bind(device)
        .bind(browser)
        .bind(os)
        .bind(session_id)
        .bind(refresh_hash)
        .bind(user_agent)
        .bind(refresh_ttl_secs as f64)
        .bind(country_code)
        .bind(asn)
        .bind(org)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// 로그인 이력 로그 생성 (History Log)
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_login_log_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        event: &str,
        login_success_log: bool,
        session_id: &str,
        refresh_hash: &str,
        login_ip_log: &str,
        device: Option<&str>,
        browser: Option<&str>,
        os: Option<&str>,
        user_agent: Option<&str>,
        country_code: Option<&str>,
        asn: Option<i64>,
        org: Option<&str>,
        access_log: Option<&str>,
        token_id_log: Option<&str>,
        fail_reason_log: Option<&str>,
        expire_secs: Option<i64>,
    ) -> AppResult<()> {
        sqlx::query(r#"
            INSERT INTO public.login_log (
                user_id,
                login_event_log,
                login_success_log,
                login_ip_log,
                login_device_log,
                login_browser_log,
                login_os_log,
                login_method_log,
                login_session_id_log,
                login_refresh_hash_log,
                login_user_agent_log,
                login_country_log,
                login_asn_log,
                login_org_log,
                login_access_log,
                login_token_id_log,
                login_fail_reason_log,
                login_expire_at_log,
                login_begin_at_log,
                login_created_at_log
            )
            VALUES (
                $1,
                CAST($2 AS login_event_enum),
                $3,
                $4,
                CASE lower($5)
                    WHEN 'mobile'  THEN 'mobile'::login_device_enum
                    WHEN 'tablet'  THEN 'tablet'::login_device_enum
                    WHEN 'desktop' THEN 'desktop'::login_device_enum
                    WHEN 'web'     THEN 'desktop'::login_device_enum
                    WHEN 'browser' THEN 'desktop'::login_device_enum
                    ELSE 'other'::login_device_enum
                END,
                $6,
                $7,
                'email'::login_method_enum,
                CAST($8 AS uuid),
                $9,
                $10,
                COALESCE($11, 'LC'),
                COALESCE($12, 0),
                COALESCE($13, 'local'),
                $14,
                $15,
                $16,
                CASE WHEN $17::float8 IS NOT NULL THEN NOW() + make_interval(secs => $17) ELSE NULL END,
                NOW(),
                NOW()
            )
        "#)
        .bind(user_id)
        .bind(event)
        .bind(login_success_log)
        .bind(login_ip_log)
        .bind(device)
        .bind(browser)
        .bind(os)
        .bind(session_id)
        .bind(refresh_hash)
        .bind(user_agent)
        .bind(country_code)
        .bind(asn)
        .bind(org)
        .bind(access_log)
        .bind(token_id_log)
        .bind(fail_reason_log)
        .bind(expire_secs.map(|s| s as f64))
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// 로그아웃 이력 로그 생성
    pub async fn insert_logout_log_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        session_id: &str,
        refresh_hash: &str,
        login_ip_log: &str,
        user_agent: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO public.login_log (
                user_id,
                login_event_log,
                login_success_log,
                login_ip_log,
                login_device_log,
                login_browser_log,
                login_os_log,
                login_method_log,
                login_session_id_log,
                login_refresh_hash_log,
                login_user_agent_log,
                login_country_log,
                login_asn_log,
                login_org_log,
                login_fail_reason_log,
                login_expire_at_log,
                login_begin_at_log,
                login_created_at_log
            )
            SELECT
                $1,
                'logout'::login_event_enum,
                TRUE,
                $4,
                COALESCE(l.login_device, 'other'::login_device_enum),
                COALESCE(l.login_browser, NULL),
                COALESCE(l.login_os, NULL),
                'email'::login_method_enum,
                CAST($2 AS uuid),
                $3,
                COALESCE($5, l.login_user_agent),
                COALESCE(l.login_country, 'LC'),
                COALESCE(l.login_asn, 0),
                COALESCE(l.login_org, 'local'),
                'none',
                l.login_expire_at,
                NOW(),
                NOW()
            FROM public.login l
            WHERE l.login_session_id = CAST($2 AS uuid)
        "#,
        )
        .bind(user_id)
        .bind(session_id)
        .bind(refresh_hash)
        .bind(login_ip_log)
        .bind(user_agent)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    // ---------------------------------------------------------------------
    // Session State Management
    // ---------------------------------------------------------------------

    /// 세션 ID로 로그인 정보 조회 (TX)
    pub async fn find_login_by_session_id_tx(
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
    ) -> AppResult<Option<LoginRecord>> {
        let row = sqlx::query_as::<_, LoginRecord>(
            r#"
            SELECT
                user_id,
                login_session_id::text as session_id,
                login_refresh_hash as refresh_hash,
                login_ip,
                login_device::text as login_device,
                login_browser,
                login_os,
                login_user_agent as user_agent,
                login_method::text as login_method,
                login_begin_at as login_time_at,
                NULL as logout_time_at,
                login_state::text as state
            FROM public.login
            WHERE login_session_id = CAST($1 AS uuid)
        "#,
        )
        .bind(session_id)
        .fetch_optional(&mut **tx)
        .await?;

        Ok(row)
    }

    /// 세션 ID로 로그인 정보 조회 (Pool)
    pub async fn find_login_by_session_id(
        pool: &PgPool,
        session_id: &str,
    ) -> AppResult<Option<LoginRecord>> {
        let row = sqlx::query_as::<_, LoginRecord>(
            r#"
            SELECT
                user_id,
                login_session_id::text as session_id,
                login_refresh_hash as refresh_hash,
                login_ip,
                login_device::text as login_device,
                login_browser,
                login_os,
                login_user_agent as user_agent,
                login_method::text as login_method,
                login_begin_at as login_time_at,
                NULL as logout_time_at,
                login_state::text as state
            FROM public.login
            WHERE login_session_id = CAST($1 AS uuid)
        "#,
        )
        .bind(session_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 세션 조회 및 Lock (For Update)
    pub async fn find_login_by_session_id_for_update_tx(
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
    ) -> AppResult<Option<LoginRecord>> {
        let row = sqlx::query_as::<_, LoginRecord>(
            r#"
            SELECT
                user_id,
                login_session_id::text as session_id,
                login_refresh_hash as refresh_hash,
                login_ip,
                login_device::text as login_device,
                login_browser,
                login_os,
                login_user_agent as user_agent,
                login_method::text as login_method,
                login_begin_at as login_time_at,
                NULL as logout_time_at,
                login_state::text as state
            FROM public.login
            WHERE login_session_id = CAST($1 AS uuid)
            FOR UPDATE
        "#,
        )
        .bind(session_id)
        .fetch_optional(&mut **tx)
        .await?;

        Ok(row)
    }

    /// 리프레시 토큰 갱신 (+ expire_at, active_at 업데이트)
    pub async fn update_login_refresh_hash_tx(
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
        new_refresh_hash: &str,
        refresh_ttl_secs: i64,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE public.login
            SET login_refresh_hash = $2,
                login_expire_at = NOW() + make_interval(secs => $3),
                login_active_at = NOW()
            WHERE login_session_id = CAST($1 AS uuid)
        "#,
        )
        .bind(session_id)
        .bind(new_refresh_hash)
        .bind(refresh_ttl_secs as f64)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// 단일 세션 상태 변경 (로그아웃 등)
    pub async fn update_login_state_by_session_tx(
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
        state: &str,
        revoked_reason: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE public.login
            SET login_state = CAST($2 AS login_state_enum),
                login_revoked_reason = $3
            WHERE login_session_id = CAST($1 AS uuid)
        "#,
        )
        .bind(session_id)
        .bind(state)
        .bind(revoked_reason)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// 특정 사용자의 모든 활성 세션 상태 변경
    pub async fn update_login_state_by_user_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        state: &str,
        revoked_reason: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE public.login
            SET login_state = CAST($2 AS login_state_enum),
                login_revoked_reason = $3
            WHERE user_id = $1 AND login_state = 'active'::login_state_enum
        "#,
        )
        .bind(user_id)
        .bind(state)
        .bind(revoked_reason)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// 사용자의 활성 세션 ID 목록 조회
    pub async fn find_user_session_ids_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
    ) -> AppResult<Vec<String>> {
        let rows = sqlx::query_as::<_, (String,)>(
            r#"
            SELECT login_session_id::text
            FROM public.login
            WHERE user_id = $1 AND login_state = 'active'::login_state_enum
        "#,
        )
        .bind(user_id)
        .fetch_all(&mut **tx)
        .await?;

        Ok(rows.into_iter().map(|(id,)| id).collect())
    }

    /// 사용자의 활성 세션 ID + 리프레시 해시 일괄 조회 (N+1 방지)
    pub async fn find_user_sessions_with_refresh_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
    ) -> AppResult<Vec<(String, String)>> {
        let rows = sqlx::query_as::<_, (String, String)>(
            r#"
            SELECT login_session_id::text, login_refresh_hash
            FROM public.login
            WHERE user_id = $1 AND login_state = 'active'::login_state_enum
        "#,
        )
        .bind(user_id)
        .fetch_all(&mut **tx)
        .await?;

        Ok(rows)
    }

    /// 특정 세션 상태 변경 (pool 버전 — 트랜잭션 없이 단독 실행)
    pub async fn update_login_state_by_session(
        pool: &PgPool,
        session_id: &str,
        state: &str,
        revoked_reason: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE public.login
            SET login_state = CAST($2 AS login_state_enum),
                login_revoked_reason = $3
            WHERE login_session_id = CAST($1 AS uuid)
        "#,
        )
        .bind(session_id)
        .bind(state)
        .bind(revoked_reason)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// 사용자의 가장 오래된 활성 세션 N개 조회 (FIFO 퇴장용).
    /// refresh_hash 를 함께 반환하여 eviction 루프에서 추가 DB 조회(N+1) 를 제거한다.
    pub async fn find_active_sessions_oldest(
        pool: &PgPool,
        user_id: i64,
        limit: usize,
    ) -> AppResult<Vec<(String, String)>> {
        let rows = sqlx::query_as::<_, (String, String)>(
            r#"
            SELECT login_session_id::text, login_refresh_hash
            FROM public.login
            WHERE user_id = $1
              AND login_state = 'active'::login_state_enum
              AND login_expire_at > now()
            ORDER BY login_begin_at ASC
            LIMIT $2
        "#,
        )
        .bind(user_id)
        .bind(limit as i64)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    /// 사용자의 현재 활성 세션 수 (동시 세션 제한 판정의 권위 소스).
    /// Redis SCARD(ak:user_sessions)는 TTL이 없어 만료 세션을 over-count 하므로
    /// (2026-05-30 INC) DB의 login_expire_at을 기준으로 시간 기반 진실을 센다.
    /// index_login_active_by_user(user_id, login_state) + index_login_expire_at 사용.
    pub async fn count_active_sessions(pool: &PgPool, user_id: i64) -> AppResult<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT count(*)
            FROM public.login
            WHERE user_id = $1
              AND login_state = 'active'::login_state_enum
              AND login_expire_at > now()
        "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(count)
    }

    /// `count_active_sessions` 의 트랜잭션 버전 — 동시 세션 admission 을 advisory lock
    /// 안에서 원자적으로 카운트하기 위함 (check-then-insert TOCTOU 차단).
    pub async fn count_active_sessions_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
    ) -> AppResult<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT count(*)
            FROM public.login
            WHERE user_id = $1
              AND login_state = 'active'::login_state_enum
              AND login_expire_at > now()
        "#,
        )
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await?;

        Ok(count)
    }

    /// 사용자별 세션 admission 직렬화용 transaction-scoped advisory lock.
    /// 2-인자 형식(namespace=2, user_id)으로 textbook/ebook 의 1-인자 lock 과
    /// 키 공간이 완전히 분리됨 (Postgres 는 1-arg int8 / 2-arg int4 lock 을 별개로 취급).
    /// 트랜잭션 종료(commit/rollback) 시 자동 해제.
    pub async fn acquire_user_session_lock_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
    ) -> AppResult<()> {
        // login PK 는 bigint(IDENTITY). advisory_xact_lock 2-arg 는 int4 이므로 truncate 하지만,
        // 한도 판정 카운트는 full i64(count_active_sessions_tx)를 쓰므로 정확하다. truncation 은
        // user_id 가 2^31 을 넘어야 발생하고, 그때도 두 user 가 같은 lock 슬롯을 공유해 직렬화가
        // 약화될 뿐(드문 perf 히컵) 카운트/보안엔 영향이 없다. 현 규모에선 1:1 정확.
        sqlx::query("SELECT pg_advisory_xact_lock($1, $2)")
            .bind(2_i32) // namespace: session admission
            .bind(user_id as i32)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    /// 시간 기반 세션 reaper — login_expire_at 이 지난 active 행을 expired 로 정리.
    /// 백그라운드 task(jobs::session_reaper)가 주기적으로 호출. 반환 = 정리된 행 수.
    /// Redis 키는 각자의 TTL 로 자연 만료되므로 여기서 건드리지 않는다.
    pub async fn reap_expired_sessions(pool: &PgPool) -> AppResult<u64> {
        let res = sqlx::query(
            r#"
            UPDATE public.login
            SET login_state = 'expired'::login_state_enum,
                login_revoked_reason = 'ttl_reaped',
                login_updated_at = now()
            WHERE login_state = 'active'::login_state_enum
              AND login_expire_at IS NOT NULL
              AND login_expire_at < now()
        "#,
        )
        .execute(pool)
        .await?;

        Ok(res.rows_affected())
    }

    /// 세션 ID 배치로 refresh_hash 조회 (유령 세션 정리 루프의 N+1 제거용).
    /// 반환값: session_id → refresh_hash 맵. 매칭되지 않은 session_id 는 키에 없음.
    pub async fn find_login_refresh_hashes_by_session_ids(
        pool: &PgPool,
        session_ids: &[String],
    ) -> AppResult<std::collections::HashMap<String, String>> {
        if session_ids.is_empty() {
            return Ok(std::collections::HashMap::new());
        }
        let rows = sqlx::query_as::<_, (String, String)>(
            r#"
            SELECT login_session_id::text, login_refresh_hash
            FROM public.login
            WHERE login_session_id = ANY($1::uuid[])
        "#,
        )
        .bind(session_ids)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().collect())
    }

    /// 세션 ID 배치로 login_state 를 일괄 변경 (eviction/ghost cleanup 루프의 N+1 UPDATE 제거용).
    pub async fn update_login_states_by_sessions(
        pool: &PgPool,
        session_ids: &[String],
        state: &str,
        revoked_reason: Option<&str>,
    ) -> AppResult<()> {
        if session_ids.is_empty() {
            return Ok(());
        }
        sqlx::query(
            r#"
            UPDATE public.login
            SET login_state = CAST($2 AS login_state_enum),
                login_revoked_reason = $3
            WHERE login_session_id = ANY($1::uuid[])
        "#,
        )
        .bind(session_ids)
        .bind(state)
        .bind(revoked_reason)
        .execute(pool)
        .await?;

        Ok(())
    }

    // ---------------------------------------------------------------------
    // OAuth Related
    // ---------------------------------------------------------------------

    /// 사용자의 연결된 OAuth Provider 목록 조회
    pub async fn find_oauth_providers_by_user_id(
        pool: &PgPool,
        user_id: i64,
    ) -> AppResult<Vec<String>> {
        let rows = sqlx::query_scalar::<_, String>(
            r#"
            SELECT oauth_provider::text
            FROM users_oauth
            WHERE user_id = $1
        "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    /// OAuth 연결 정보 생성
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_oauth_link_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        provider: &str,
        oauth_subject: &str,
        oauth_email: Option<&str>,
        name: Option<&str>,
        picture_url: Option<&str>,
        oauth_subject_idx: &str,
    ) -> AppResult<i64> {
        let oauth_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO users_oauth (
                user_id, oauth_provider, oauth_subject,
                oauth_email, oauth_name, oauth_picture_url,
                oauth_last_login_at,
                oauth_subject_idx
            )
            VALUES ($1, $2::login_method_enum, $3, $4, $5, $6, NOW(), $7)
            RETURNING user_oauth_id
        "#,
        )
        .bind(user_id)
        .bind(provider)
        .bind(oauth_subject)
        .bind(oauth_email)
        .bind(name)
        .bind(picture_url)
        .bind(oauth_subject_idx)
        .fetch_one(&mut **tx)
        .await?;

        Ok(oauth_id)
    }

    /// OAuth 마지막 로그인 시간 업데이트
    pub async fn update_oauth_last_login(pool: &PgPool, user_oauth_id: i64) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE users_oauth
            SET oauth_last_login_at = NOW()
            WHERE user_oauth_id = $1
        "#,
        )
        .bind(user_oauth_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// 로그인 세션 기록 생성 (OAuth 전용)
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_login_record_oauth_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        session_id: &str,
        refresh_hash: &str,
        login_ip: &str,
        login_method: &str,
        device: Option<&str>,
        browser: Option<&str>,
        os: Option<&str>,
        user_agent: Option<&str>,
        refresh_ttl_secs: i64,
        country_code: Option<&str>,
        asn: Option<i64>,
        org: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO public.login (
                user_id,
                login_ip,
                login_device,
                login_browser,
                login_os,
                login_method,
                login_session_id,
                login_refresh_hash,
                login_user_agent,
                login_begin_at,
                login_state,
                login_expire_at,
                login_active_at,
                login_country, login_asn, login_org,
                login_revoked_reason
            )
            VALUES (
                $1,
                $2,
                CASE lower($3)
                    WHEN 'mobile'  THEN 'mobile'::login_device_enum
                    WHEN 'tablet'  THEN 'tablet'::login_device_enum
                    WHEN 'desktop' THEN 'desktop'::login_device_enum
                    WHEN 'web'     THEN 'desktop'::login_device_enum
                    WHEN 'browser' THEN 'desktop'::login_device_enum
                    ELSE 'other'::login_device_enum
                END,
                $4,
                $5,
                $6::login_method_enum,
                CAST($7 AS uuid),
                $8,
                $9,
                NOW(),
                'active'::login_state_enum,
                NOW() + make_interval(secs => $10),
                NOW(),
                COALESCE($11, 'LC'), COALESCE($12, 0), COALESCE($13, 'local'),
                'none'
            )
        "#,
        )
        .bind(user_id)
        .bind(login_ip)
        .bind(device)
        .bind(browser)
        .bind(os)
        .bind(login_method)
        .bind(session_id)
        .bind(refresh_hash)
        .bind(user_agent)
        .bind(refresh_ttl_secs as f64)
        .bind(country_code)
        .bind(asn)
        .bind(org)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// 로그인 로그 생성 (OAuth 전용)
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_login_log_oauth_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        event: &str,
        login_success_log: bool,
        session_id: &str,
        refresh_hash: &str,
        login_ip_log: &str,
        login_method: &str,
        device: Option<&str>,
        browser: Option<&str>,
        os: Option<&str>,
        user_agent: Option<&str>,
        country_code: Option<&str>,
        asn: Option<i64>,
        org: Option<&str>,
        access_log: Option<&str>,
        token_id_log: Option<&str>,
        fail_reason_log: Option<&str>,
        expire_secs: Option<i64>,
    ) -> AppResult<()> {
        sqlx::query(r#"
            INSERT INTO public.login_log (
                user_id,
                login_event_log,
                login_success_log,
                login_ip_log,
                login_device_log,
                login_browser_log,
                login_os_log,
                login_method_log,
                login_session_id_log,
                login_refresh_hash_log,
                login_user_agent_log,
                login_country_log,
                login_asn_log,
                login_org_log,
                login_access_log,
                login_token_id_log,
                login_fail_reason_log,
                login_expire_at_log,
                login_begin_at_log,
                login_created_at_log
            )
            VALUES (
                $1,
                CAST($2 AS login_event_enum),
                $3,
                $4,
                CASE lower($5)
                    WHEN 'mobile'  THEN 'mobile'::login_device_enum
                    WHEN 'tablet'  THEN 'tablet'::login_device_enum
                    WHEN 'desktop' THEN 'desktop'::login_device_enum
                    WHEN 'web'     THEN 'desktop'::login_device_enum
                    WHEN 'browser' THEN 'desktop'::login_device_enum
                    ELSE 'other'::login_device_enum
                END,
                $6,
                $7,
                $8::login_method_enum,
                CAST($9 AS uuid),
                $10,
                $11,
                COALESCE($12, 'LC'),
                COALESCE($13, 0),
                COALESCE($14, 'local'),
                $15,
                $16,
                $17,
                CASE WHEN $18::float8 IS NOT NULL THEN NOW() + make_interval(secs => $18) ELSE NULL END,
                NOW(),
                NOW()
            )
        "#)
        .bind(user_id)
        .bind(event)
        .bind(login_success_log)
        .bind(login_ip_log)
        .bind(device)
        .bind(browser)
        .bind(os)
        .bind(login_method)
        .bind(session_id)
        .bind(refresh_hash)
        .bind(user_agent)
        .bind(country_code)
        .bind(asn)
        .bind(org)
        .bind(access_log)
        .bind(token_id_log)
        .bind(fail_reason_log)
        .bind(expire_secs.map(|s| s as f64))
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
    // ---------------------------------------------------------------------
    // Blind Index Lookups
    // ---------------------------------------------------------------------

    /// Blind index로 로그인용 사용자 조회
    pub async fn find_user_by_email_idx(
        pool: &PgPool,
        email_idx: &str,
    ) -> AppResult<Option<UserLoginInfo>> {
        let row = sqlx::query_as::<_, UserLoginInfo>(
            r#"
            SELECT
                user_id,
                user_email,
                user_password,
                user_state,
                user_auth,
                user_check_email,
                user_mfa_enabled
            FROM users
            WHERE user_email_idx = $1
        "#,
        )
        .bind(email_idx)
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    /// user_check_email 상태 업데이트
    pub async fn update_user_check_email(
        pool: &PgPool,
        user_id: i64,
        check_email: bool,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET user_check_email = $2
            WHERE user_id = $1
        "#,
        )
        .bind(user_id)
        .bind(check_email)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Blind index로 이름 조회 (아이디 찾기용 — 복수 결과)
    pub async fn find_users_by_name_idx(
        pool: &PgPool,
        name_idx: &str,
    ) -> AppResult<Vec<UserFindIdInfo>> {
        let rows = sqlx::query_as::<_, UserFindIdInfo>(
            r#"
            SELECT
                user_id,
                user_email,
                user_name,
                user_birthday,
                user_password
            FROM users
            WHERE user_name_idx = $1
              AND user_state = true
        "#,
        )
        .bind(name_idx)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    /// Blind index로 OAuth subject 조회
    pub async fn find_oauth_by_provider_subject_idx(
        pool: &PgPool,
        provider: &str,
        subject_idx: &str,
    ) -> AppResult<Option<UserOAuthInfo>> {
        let row = sqlx::query_as::<_, UserOAuthInfo>(
            r#"
            SELECT
                user_oauth_id,
                user_id,
                oauth_provider::text as oauth_provider,
                oauth_subject,
                oauth_email
            FROM users_oauth
            WHERE oauth_provider = $1::login_method_enum
              AND oauth_subject_idx = $2
        "#,
        )
        .bind(provider)
        .bind(subject_idx)
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    // ---------------------------------------------------------------------
    // MFA (Multi-Factor Authentication)
    // ---------------------------------------------------------------------

    /// MFA 활성화 여부 조회
    pub async fn find_user_mfa_enabled(pool: &PgPool, user_id: i64) -> AppResult<bool> {
        let row: Option<(bool,)> =
            sqlx::query_as("SELECT user_mfa_enabled FROM users WHERE user_id = $1")
                .bind(user_id)
                .fetch_optional(pool)
                .await?;
        Ok(row.map(|r| r.0).unwrap_or(false))
    }

    /// MFA secret 조회 (암호화된 상태)
    pub async fn find_mfa_secret(pool: &PgPool, user_id: i64) -> AppResult<Option<String>> {
        let row: Option<(Option<String>,)> =
            sqlx::query_as("SELECT user_mfa_secret FROM users WHERE user_id = $1")
                .bind(user_id)
                .fetch_optional(pool)
                .await?;
        Ok(row.and_then(|r| r.0))
    }

    /// MFA secret 임시 저장 (setup 단계, enabled=false 유지)
    pub async fn update_mfa_secret(
        pool: &PgPool,
        user_id: i64,
        encrypted_secret: &str,
    ) -> AppResult<()> {
        sqlx::query("UPDATE users SET user_mfa_secret = $2 WHERE user_id = $1")
            .bind(user_id)
            .bind(encrypted_secret)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// MFA 활성화 (verify-setup 성공 후)
    pub async fn enable_mfa(
        pool: &PgPool,
        user_id: i64,
        encrypted_backup_codes: &str,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET user_mfa_enabled = true,
                user_mfa_backup_codes = $2,
                user_mfa_enabled_at = NOW()
            WHERE user_id = $1
        "#,
        )
        .bind(user_id)
        .bind(encrypted_backup_codes)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// MFA 백업 코드 조회 (암호화된 상태)
    pub async fn find_mfa_backup_codes(pool: &PgPool, user_id: i64) -> AppResult<Option<String>> {
        let row: Option<(Option<String>,)> =
            sqlx::query_as("SELECT user_mfa_backup_codes FROM users WHERE user_id = $1")
                .bind(user_id)
                .fetch_optional(pool)
                .await?;
        Ok(row.and_then(|r| r.0))
    }

    /// MFA 백업 코드 갱신 (사용된 코드 제거 후)
    pub async fn update_mfa_backup_codes(
        pool: &PgPool,
        user_id: i64,
        encrypted_backup_codes: &str,
    ) -> AppResult<()> {
        sqlx::query("UPDATE users SET user_mfa_backup_codes = $2 WHERE user_id = $1")
            .bind(user_id)
            .bind(encrypted_backup_codes)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// MFA 비활성화 (HYMN 전용 — 모든 MFA 컬럼 초기화)
    pub async fn disable_mfa(pool: &PgPool, user_id: i64) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET user_mfa_secret = NULL,
                user_mfa_enabled = false,
                user_mfa_backup_codes = NULL,
                user_mfa_enabled_at = NULL
            WHERE user_id = $1
        "#,
        )
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}

// =========================================================================
// OAuth Data Model
// =========================================================================

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserOAuthInfo {
    pub user_oauth_id: i64,
    pub user_id: i64,
    pub oauth_provider: String,
    pub oauth_subject: String,
    pub oauth_email: Option<String>,
}
