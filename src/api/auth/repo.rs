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
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct UserLoginInfo {
    pub user_id: i64,
    pub user_email: String,
    pub user_password: Option<String>,  // NULL for social-only accounts
    pub user_state: bool,
    pub user_auth: UserAuth,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct LoginRecord {
    pub user_id: i64,
    pub session_id: String,   // UUID -> String
    pub refresh_hash: String,
    pub login_ip: Option<String>, // Inet -> String
    pub login_device: String, // Enum -> String
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

    /// 이메일로 로그인에 필요한 최소 정보 조회
    pub async fn find_user_by_email(
        pool: &PgPool,
        email: &str,
    ) -> AppResult<Option<UserLoginInfo>> {
        let row = sqlx::query_as::<_, UserLoginInfo>(r#"
            SELECT
                user_id,
                user_email,
                user_password,
                user_state,
                user_auth
            FROM users
            WHERE user_email = $1
        "#)
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn find_user_by_name_and_email(
        pool: &PgPool,
        name: &str,
        email: &str,
    ) -> AppResult<Option<UserFindIdInfo>> {
        let row = sqlx::query_as::<_, UserFindIdInfo>(r#"
            SELECT
                user_id,
                user_email,
                user_name
            FROM users
            WHERE user_name = $1
              AND user_email = $2
              AND user_state = true
        "#)
        .bind(name)
        .bind(email)
        .fetch_optional(pool)
        .await?;
        
        Ok(row)
    }

    pub async fn update_user_password_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        new_password_hash: &str,
    ) -> AppResult<()> {
        let res = sqlx::query(r#"
            UPDATE users
            SET user_password = $2
            WHERE user_id = $1
        "#)
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
        // Geolocation fields (from ip-api.com)
        country_code: Option<&str>,
        asn: Option<i64>,
        org: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(r#"
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
                login_country, login_asn, login_org
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
                COALESCE($9, 'ZZ'), COALESCE($10, 0), COALESCE($11, 'Unknown')
            )
        "#)
        .bind(user_id)
        .bind(login_ip)
        .bind(device)
        .bind(browser)
        .bind(os)
        .bind(session_id)
        .bind(refresh_hash)
        .bind(user_agent)
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
        login_ip: &str,
        device: Option<&str>,
        browser: Option<&str>,
        os: Option<&str>,
        user_agent: Option<&str>,
        // Geolocation fields (from ip-api.com)
        country_code: Option<&str>,
        asn: Option<i64>,
        org: Option<&str>,
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
                login_begin_at_log,
                login_created_at_log
            )
            VALUES (
                $1,
                CAST($2 AS login_event_enum),
                $3,
                CAST($4 AS inet),
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
                $11,
                $12,
                $13,
                NOW(),
                NOW()
            )
        "#)
        .bind(user_id)
        .bind(event)
        .bind(login_success_log)
        .bind(login_ip)
        .bind(device)
        .bind(browser)
        .bind(os)
        .bind(session_id)
        .bind(refresh_hash)
        .bind(user_agent)
        .bind(country_code)
        .bind(asn)
        .bind(org)
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
        login_ip: &str,
        user_agent: Option<&str>,
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
        "#)
        .bind(user_id)
        .bind(session_id)
        .bind(refresh_hash)
        .bind(login_ip)
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
        let row = sqlx::query_as::<_, LoginRecord>(r#"
            SELECT
                user_id,
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
        "#)
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
        let row = sqlx::query_as::<_, LoginRecord>(r#"
            SELECT
                user_id,
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
        "#)
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
        let row = sqlx::query_as::<_, LoginRecord>(r#"
            SELECT
                user_id,
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
            FOR UPDATE
        "#)
        .bind(session_id)
        .fetch_optional(&mut **tx)
        .await?;
        
        Ok(row)
    }

    /// 리프레시 토큰 갱신
    pub async fn update_login_refresh_hash_tx(
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
        new_refresh_hash: &str,
    ) -> AppResult<()> {
        sqlx::query(r#"
            UPDATE public.login
            SET login_refresh_hash = $2
            WHERE login_session_id = CAST($1 AS uuid)
        "#)
        .bind(session_id)
        .bind(new_refresh_hash)
        .execute(&mut **tx)
        .await?;
        
        Ok(())
    }

    /// 단일 세션 상태 변경 (로그아웃 등)
    pub async fn update_login_state_by_session_tx(
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
        state: &str,
    ) -> AppResult<()> {
        sqlx::query(r#"
            UPDATE public.login
            SET login_state = CAST($2 AS login_state_enum)
            WHERE login_session_id = CAST($1 AS uuid)
        "#)
        .bind(session_id)
        .bind(state)
        .execute(&mut **tx)
        .await?;
        
        Ok(())
    }

    /// 특정 사용자의 모든 활성 세션 상태 변경
    pub async fn update_login_state_by_user_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        state: &str,
    ) -> AppResult<()> {
        sqlx::query(r#"
            UPDATE public.login
            SET login_state = CAST($2 AS login_state_enum)
            WHERE user_id = $1 AND login_state = 'active'::login_state_enum
        "#)
        .bind(user_id)
        .bind(state)
        .execute(&mut **tx)
        .await?;
        
        Ok(())
    }

    /// 사용자의 활성 세션 ID 목록 조회
    pub async fn find_user_session_ids_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
    ) -> AppResult<Vec<String>> {
        let rows = sqlx::query_as::<_, (String,)>(r#"
            SELECT login_session_id::text
            FROM public.login
            WHERE user_id = $1 AND login_state = 'active'::login_state_enum
        "#)
        .bind(user_id)
        .fetch_all(&mut **tx)
        .await?;

        Ok(rows.into_iter().map(|(id,)| id).collect())
    }

    // ---------------------------------------------------------------------
    // OAuth Related
    // ---------------------------------------------------------------------

    /// OAuth subject로 연결된 계정 조회
    pub async fn find_oauth_by_provider_subject(
        pool: &PgPool,
        provider: &str,
        subject: &str,
    ) -> AppResult<Option<UserOAuthInfo>> {
        let row = sqlx::query_as::<_, UserOAuthInfo>(r#"
            SELECT
                user_oauth_id,
                user_id,
                oauth_provider::text as oauth_provider,
                oauth_subject,
                oauth_email
            FROM user_oauth
            WHERE oauth_provider = $1::login_method_enum
              AND oauth_subject = $2
        "#)
        .bind(provider)
        .bind(subject)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 사용자의 연결된 OAuth Provider 목록 조회
    pub async fn find_oauth_providers_by_user_id(
        pool: &PgPool,
        user_id: i64,
    ) -> AppResult<Vec<String>> {
        let rows = sqlx::query_scalar::<_, String>(r#"
            SELECT oauth_provider::text
            FROM user_oauth
            WHERE user_id = $1
        "#)
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
        subject: &str,
        email: Option<&str>,
        name: Option<&str>,
        picture_url: Option<&str>,
    ) -> AppResult<i64> {
        let oauth_id = sqlx::query_scalar::<_, i64>(r#"
            INSERT INTO user_oauth (
                user_id, oauth_provider, oauth_subject,
                oauth_email, oauth_name, oauth_picture_url,
                oauth_last_login_at
            )
            VALUES ($1, $2::login_method_enum, $3, $4, $5, $6, NOW())
            RETURNING user_oauth_id
        "#)
        .bind(user_id)
        .bind(provider)
        .bind(subject)
        .bind(email)
        .bind(name)
        .bind(picture_url)
        .fetch_one(&mut **tx)
        .await?;

        Ok(oauth_id)
    }

    /// OAuth 마지막 로그인 시간 업데이트
    pub async fn update_oauth_last_login(
        pool: &PgPool,
        user_oauth_id: i64,
    ) -> AppResult<()> {
        sqlx::query(r#"
            UPDATE user_oauth
            SET oauth_last_login_at = NOW()
            WHERE user_oauth_id = $1
        "#)
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
        login_method: &str,  // 'google' or 'apple'
        device: Option<&str>,
        browser: Option<&str>,
        os: Option<&str>,
        user_agent: Option<&str>,
        // Geolocation fields (from ip-api.com)
        country_code: Option<&str>,
        asn: Option<i64>,
        org: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(r#"
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
                login_country, login_asn, login_org
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
                $6::login_method_enum,
                CAST($7 AS uuid),
                $8,
                $9,
                NOW(),
                'active'::login_state_enum,
                COALESCE($10, 'ZZ'), COALESCE($11, 0), COALESCE($12, 'Unknown')
            )
        "#)
        .bind(user_id)
        .bind(login_ip)
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
        login_ip: &str,
        login_method: &str,  // 'google' or 'apple'
        device: Option<&str>,
        browser: Option<&str>,
        os: Option<&str>,
        user_agent: Option<&str>,
        // Geolocation fields (from ip-api.com)
        country_code: Option<&str>,
        asn: Option<i64>,
        org: Option<&str>,
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
                login_begin_at_log,
                login_created_at_log
            )
            VALUES (
                $1,
                CAST($2 AS login_event_enum),
                $3,
                CAST($4 AS inet),
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
                $12,
                $13,
                $14,
                NOW(),
                NOW()
            )
        "#)
        .bind(user_id)
        .bind(event)
        .bind(login_success_log)
        .bind(login_ip)
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
        .execute(&mut **tx)
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