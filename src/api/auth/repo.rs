// FILE: src/api/auth/repo.rs
use crate::error::AppResult;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};
//use crate::types::{LoginDevice, LoginMethod}; // types.rs에 정의된 Enum 사용

pub struct AuthRepo;

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserLoginInfo {
    pub user_id: i64,
    pub user_email: String,
    pub user_password: String,
    pub user_state: bool,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct LoginRecord {
    pub session_id: String,
    pub refresh_hash: String,
    pub login_ip: Option<String>,
    pub login_device: String,
    pub login_browser: Option<String>,
    pub login_os: Option<String>,
    pub user_agent: Option<String>,
    pub login_method: String,
    pub login_time_at: DateTime<Utc>,
    pub logout_time_at: Option<DateTime<Utc>>, // DB 컬럼 없음, NULL 처리
    pub state: String,
}

impl AuthRepo {
    // 이메일로 사용자 정보 조회
    pub async fn find_user_by_email(
        pool: &PgPool,
        email: &str,
    ) -> AppResult<Option<UserLoginInfo>> {
        let row = sqlx::query_as::<_, UserLoginInfo>(
            r#"
            SELECT
                user_id,
                user_email,
                user_password,
                user_state
            FROM users
            WHERE user_email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;
        Ok(row)
    }

    // 로그인 기록 삽입 (트랜잭션 버전)
    // 수정사항: login_success 제거, 컬럼명 매핑(login_session_id 등), NOT NULL 컬럼 기본값 추가
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
                login_country, login_asn, login_org -- NOT NULL 컬럼들
            )
            VALUES (
                $1, 
                CAST($2 AS inet),
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
                'ZZ', 0, 'Unknown' -- GeoIP 미구현 시 기본값 (DB Constraint 충족용)
            )
            "#,
        )
        .bind(user_id)      // $1
        .bind(login_ip)     // $2
        .bind(device)       // $3
        .bind(browser)      // $4
        .bind(os)           // $5
        .bind(session_id)   // $6
        .bind(refresh_hash) // $7
        .bind(user_agent)   // $8
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    // 로그인 로그 기록 삽입 (트랜잭션 버전)
    // 수정사항: 컬럼명 매핑 (event -> login_event_log, login_success -> login_success_log 등)
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_login_log_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        event: &str,
        #[allow(unused_variables)] login_success_log: bool,
        session_id: &str,
        refresh_hash: &str,
        login_ip: &str,
        device: Option<&str>,
        browser: Option<&str>,
        os: Option<&str>,
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
                login_begin_at_log, 
                login_created_at_log
            )
            VALUES (
                $1, 
                CAST($2 AS login_event_enum), 
                TRUE, 
                CAST($3 AS inet),
                CASE lower($4)
                    WHEN 'mobile'  THEN 'mobile'::login_device_enum
                    WHEN 'tablet'  THEN 'tablet'::login_device_enum
                    WHEN 'desktop' THEN 'desktop'::login_device_enum
                    WHEN 'web'     THEN 'desktop'::login_device_enum
                    WHEN 'browser' THEN 'desktop'::login_device_enum
                    ELSE 'other'::login_device_enum
                END,
                $5, 
                $6, 
                'email'::login_method_enum, 
                CAST($7 AS uuid), 
                $8, 
                $9, 
                NOW(), 
                NOW()
            )
            "#,
        )
        .bind(user_id)      // $1
        .bind(event)        // $2
        .bind(login_ip)     // $3
        .bind(device)       // $4
        .bind(browser)      // $5
        .bind(os)           // $6
        .bind(session_id)   // $7
        .bind(refresh_hash) // $8
        .bind(user_agent)   // $9
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    // 로그아웃 로그 기록 삽입 (트랜잭션 버전)
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_logout_log_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        session_id: &str,
        refresh_hash: &str,
        login_ip: &str,
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
                login_begin_at_log, 
                login_created_at_log
            )
            SELECT
                $1, 
                'logout'::login_event_enum, 
                TRUE,
                COALESCE(CAST($4 AS inet), l.login_ip),
                COALESCE(l.login_device, 'other'::login_device_enum),
                COALESCE(l.login_browser, NULL),
                COALESCE(l.login_os, NULL),
                'email'::login_method_enum, 
                CAST($2 AS uuid), 
                $3,
                COALESCE($5, l.login_user_agent),
                NOW(), 
                NOW()
            FROM public.login l
            WHERE l.login_session_id = CAST($2 AS uuid)
            "#,
        )
        .bind(user_id)      // $1
        .bind(session_id)   // $2
        .bind(refresh_hash) // $3
        .bind(login_ip)     // $4
        .bind(user_agent)   // $5
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    // 세션 ID로 로그인 기록 조회 (트랜잭션 버전)
    pub async fn find_login_by_session_id_tx(
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
    ) -> AppResult<Option<LoginRecord>> {
        let row = sqlx::query_as::<_, LoginRecord>(
            r#"
            SELECT
                login_session_id::text as session_id,
                login_refresh_hash as refresh_hash,
                login_ip::text as login_ip,
                login_device::text as login_device,
                login_browser,
                login_os,
                login_user_agent as user_agent,
                login_method::text as login_method,
                login_begin_at as login_time_at,
                NULL as logout_time_at, -- DB에서 해당 컬럼 제거됨
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

    // 세션 ID로 로그인 기록 조회 (풀 버전)
    pub async fn find_login_by_session_id(
        pool: &PgPool,
        session_id: &str,
    ) -> AppResult<Option<LoginRecord>> {
        let row = sqlx::query_as::<_, LoginRecord>(
            r#"
            SELECT
                login_session_id::text as session_id,
                login_refresh_hash as refresh_hash,
                login_ip::text as login_ip,
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

    // 리프레시 토큰 해시 업데이트 (트랜잭션 버전)
    pub async fn update_login_refresh_hash_tx(
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
        new_refresh_hash: &str,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE public.login
            SET login_refresh_hash = $2
            WHERE login_session_id = CAST($1 AS uuid)
            "#,
        )
        .bind(session_id)
        .bind(new_refresh_hash)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    // 로그인 상태 업데이트 (트랜잭션 버전)
    pub async fn update_login_state_by_session_tx(
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
        state: &str,
    ) -> AppResult<()> {
        // logout_time_at 컬럼이 없어졌으므로 state만 업데이트하거나, login_expire_at 등을 조정해야 함
        // 여기서는 state만 업데이트
        sqlx::query(
            r#"
            UPDATE public.login
            SET login_state = CAST($2 AS login_state_enum)
            WHERE login_session_id = CAST($1 AS uuid)
            "#,
        )
        .bind(session_id)
        .bind(state)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    // 특정 사용자의 모든 세션을 로그아웃 처리 (트랜잭션 버전)
    pub async fn update_login_state_by_user_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        state: &str,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE public.login
            SET login_state = CAST($2 AS login_state_enum)
            WHERE user_id = $1 AND login_state = 'active'::login_state_enum
            "#,
        )
        .bind(user_id)
        .bind(state)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    // 사용자 세션 ID 목록 조회 (트랜잭션 버전)
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

        Ok(rows.into_iter().map(|(session_id,)| session_id).collect())
    }
}